use clap::Parser;
use http::StatusCode;
use rustls::server::ClientHello;
use std::{
    io::{Error as IoError, Result as IoResult},
    sync::{Arc, OnceLock},
};

use salvo::{
    async_trait,
    conn::{
        rustls_async::{Keycert, ResolvesServerConfig, RustlsConfig},
        TcpListener,
    },
    prelude::Logger,
    writing::Json,
    Depot, FlowCtrl, Handler, Listener, Request, Response, Router, Server, Service,
};
use salvo_proxy::{hyper_client::HyperClient, Proxy};

use click_router::{
    adapters::{
        salvo::{salvo_proxy, SalvoRequest, SalvoResponse},
        RequestType, ResponseType,
    },
    app::AppBuilder,
    core::{
        flow_router::{FlowRouter, FlowRouterResult, RedirectType},
        metrics::{Timer, METRICS},
        metrics_endpoint::create_metrics_router,
    },
    settings::Settings,
};

#[derive(Parser, Debug)]
#[command(version)]
pub struct Args {
    #[arg(short, long, default_value_t = String::from("production"), env("APP_RUN_MODE"))]
    pub run_mode: String,
    #[arg(short, long, default_value_t = String::from("./config"), env("APP_CONFIG_PATH"))]
    pub config_path: String,
    #[arg(long, default_value_t = String::from("0.0.0.0:5800"), env("APP_LISTEN_ADDR"))]
    pub listen_addr: String,
    #[arg(long, default_value_t = String::from("0.0.0.0:9090"), env("APP_METRICS_ADDR"))]
    pub metrics_addr: String,
    #[arg(long, default_value_t = true, env("APP_ENABLE_METRICS"))]
    pub enable_metrics: bool,
}

static FLOW_ROUTER: OnceLock<FlowRouter> = OnceLock::new();

struct Redirect;

// fn to_socket_addr()

#[async_trait]
impl Handler for Redirect {
    async fn handle(
        &self,
        req: &mut Request,
        depot: &mut Depot,
        res: &mut Response,
        ctrl: &mut FlowCtrl,
    ) {
        // Start timing the request
        let request_timer = Timer::new();
        METRICS.requests_total.inc();
        METRICS.active_requests.inc();

        let router = get_flow_router();

        let result = router
            .handle(
                &RequestType::Salvo(&SalvoRequest::new(&req)),
                &ResponseType::Salvo(&mut SalvoResponse::new(res)),
            )
            .await;

        // Handle the result and update metrics
        match result {
            Ok(flow_result) => {
                METRICS.requests_success.inc();
                
                match flow_result {
                    FlowRouterResult::Empty(status_code) => res.status_code(status_code).render(""),
                    FlowRouterResult::Json(content, status_code) => {
                        res.status_code(status_code).render(Json(content))
                    }
                    FlowRouterResult::PlainText(content, status_code) => {
                        res.status_code(status_code).render(content)
                    }
                    FlowRouterResult::Proxied(url, _status_code) => {
                        let url = url.to_string();
                        let proxy = Proxy::new(url, HyperClient::default());
                        proxy.handle(req, depot, res, ctrl).await;
                    }
                    FlowRouterResult::Redirect(url, redirect_type) => {
                        match redirect_type {
                            RedirectType::Permanent => res.status_code(StatusCode::PERMANENT_REDIRECT),
                            RedirectType::Temporary => res.status_code(StatusCode::TEMPORARY_REDIRECT),
                        };
                        res.add_header("Location", url.to_string(), true)
                            .unwrap()
                            .render("");
                    }
                    FlowRouterResult::Retargeting(url, _script_urls) => res.render(url.to_string()),
                    FlowRouterResult::Error => {
                        METRICS.requests_error.inc();
                        res.status_code(StatusCode::INTERNAL_SERVER_ERROR).render("")
                    }
                }
            }
            Err(_) => {
                METRICS.requests_error.inc();
                res.status_code(StatusCode::INTERNAL_SERVER_ERROR).render("");
            }
        }

        // Record request duration and decrement active requests
        request_timer.observe_duration_seconds(&METRICS.request_duration);
        METRICS.active_requests.dec();
    }
}

#[inline]
pub fn get_flow_router() -> &'static FlowRouter {
    FLOW_ROUTER.get().unwrap()
}

struct ServerConfigResolverMock;

#[async_trait]
impl ResolvesServerConfig<IoError> for ServerConfigResolverMock {
    async fn resolve(&self, _client_hello: ClientHello<'_>) -> IoResult<Arc<RustlsConfig>> {
        let config = RustlsConfig::new(
            Keycert::new()
                .cert(include_bytes!("../certs/cert.pem").as_ref())
                .key(include_bytes!("../certs/key.pem").as_ref()),
        );

        Ok(Arc::new(config))
    }
}

#[tokio::main]
async fn main() {
    rustls::crypto::ring::default_provider()
        .install_default()
        .expect("Failed to install rustls crypto provider");

    tracing_subscriber::fmt().init();

    dotenv::from_filename("./.env").ok();

    let args = Args::parse();

    let settings = Settings::new(
        Some(args.run_mode.as_str()),
        Some(args.config_path.as_str()),
    )
    .unwrap();

    let flow_router = AppBuilder::new(settings)
        .with_default_modules()
        .with_geo_ip()
        .with_ua_parser()
        .with_fluvio()
        .await
        .with_mongodb()
        .await
        //.with_dynamo()
        // .await
        .build();

    let _ = FLOW_ROUTER.get_or_init(|| flow_router);

    // Create main application router
    let app_router = Router::with_path("{**rest_path}").get(Redirect);

    println!("🚀 Starting Click Router");
    println!("   Main server: https://{}", args.listen_addr);

    // Start metrics server if enabled
    if args.enable_metrics {
        let metrics_router = create_metrics_router();
        let metrics_service = Service::new(metrics_router).hoop(Logger::new());

        println!("📊 Metrics endpoints enabled:");
        println!("   Metrics server: http://{}", args.metrics_addr);
        println!(
            "   • GET {}/health        - Health check",
            args.metrics_addr
        );
        println!(
            "   • GET {}/metrics       - Prometheus metrics",
            args.metrics_addr
        );
        println!(
            "   • GET {}/metrics/info  - Detailed metrics info",
            args.metrics_addr
        );

        // Start metrics server in background with default address
        tokio::spawn(async move {
            let metrics_acceptor = TcpListener::new("0.0.0.0:9090").bind().await;
            Server::new(metrics_acceptor).serve(metrics_service).await;
        });
    } else {
        println!("📊 Metrics endpoints disabled (use --enable-metrics to enable)");
    }

    // Start main application server with default address
    let acceptor = TcpListener::new("0.0.0.0:5800")
        .rustls_async(ServerConfigResolverMock)
        .bind()
        .await;

    let service = Service::new(app_router).hoop(Logger::new());

    println!("✅ Click Router started successfully!");
    println!();

    Server::new(acceptor).serve(service).await;
}
