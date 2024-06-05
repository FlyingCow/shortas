use std::{
    io::{Error as IoError, Result as IoResult},
    sync::Arc,
};

use click_router::{
    app::AppBuilder,
    core::{flow_router::PerRequestData, BaseFlowRouter},
    flow_router::default_flow_router::DefaultFlowRouter,
    settings::Settings,
};

use clap::Parser;
use once_cell::sync::OnceCell;
use rustls::server::ClientHello;
use salvo::{
    async_trait,
    conn::{
        rustls_async::{Keycert, ResolvesServerConfig, RustlsConfig},
        TcpListener,
    },
    writing::Text,
    Depot, FlowCtrl, Handler, Listener, Request, Response, Router, Server,
};
use tracing::info;

#[derive(Parser, Debug)]
#[command(version)]
pub struct Args {
    #[arg(short, long, default_value_t = String::from("production"), env("APP_RUN_MODE"))]
    pub run_mode: String,
    #[arg(short, long, default_value_t = String::from("./config"), env("APP_CONFIG_PATH"))]
    pub config_path: String,
}

static FLOW_ROUTER: OnceCell<DefaultFlowRouter> = OnceCell::new();

struct Redirect;

#[async_trait]
impl Handler for Redirect {
    async fn handle(
        &self,
        req: &mut Request,
        _depot: &mut Depot,
        res: &mut Response,
        _ctrl: &mut FlowCtrl,
    ) {
        let request = http::Request::builder()
            .uri(req.uri())
            .header("Host", "localhost")
            .header(
                "User-Agent",
                "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:126.0) Gecko/20100101 Firefox/126.0",
            );

        let router = get_flow_router();

        let result = router
            .handle(PerRequestData {
                local_addr: "192.168.0.100:80".parse().unwrap(),
                remote_addr: "188.138.135.18:80".parse().unwrap(),
                request: request.body(()).unwrap(),
                tls_info: None,
            })
            .await
            .unwrap();

        res.render(Text::Plain(result.to_string()));
    }
}

#[inline]
pub fn get_flow_router() -> &'static DefaultFlowRouter {
    FLOW_ROUTER.get().unwrap()
}

struct ServerConfigResolverMock;

#[async_trait]
impl ResolvesServerConfig<IoError> for ServerConfigResolverMock {
    async fn resolve(&self, client_hello: ClientHello<'_>) -> IoResult<Arc<RustlsConfig>> {
        info!("host:{}", client_hello.server_name().unwrap());

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

    dotenv::from_filename("./click-router/.env").ok();

    let args = Args::parse();

    let settings = Settings::new(
        Some(args.run_mode.as_str()),
        Some(args.config_path.as_str()),
    )
    .unwrap();

    let app = AppBuilder::new(settings)
        .with_aws()
        .await
        .with_moka()
        .with_defaults()
        .with_uaparser()
        .with_geo_ip()
        .with_flow_defaults()
        .with_default_modules()
        .build()
        .unwrap();

    let _ = FLOW_ROUTER.set(app);

    let router = Router::with_path("<**rest_path>").get(Redirect);

    println!("{:?}", router);

    let acceptor = TcpListener::new("0.0.0.0:5800")
        .rustls_async(ServerConfigResolverMock)
        .bind()
        .await;

    Server::new(acceptor).serve(router).await;
}
