use std::{
    io::{Error as IoError, Result as IoResult},
    sync::Arc,
};

use click_router::{
    adapters::{
        salvo::{salvo_proxy, SalvoRequest, SalvoResponse},
        RequestType, ResponseType,
    },
    app::AppBuilder,
    core::flow_router::{FlowRouter, FlowRouterResult, RedirectType},
    settings::Settings,
};

use clap::Parser;
use http::StatusCode;
use once_cell::sync::OnceCell;
use rustls::server::ClientHello;
use salvo::{
    async_trait,
    conn::{
        rustls_async::{Keycert, ResolvesServerConfig, RustlsConfig},
        TcpListener,
    },
    writing::Json,
    Depot, FlowCtrl, Handler, Listener, Request, Response, Router, Server,
};
use salvo_proxy::{hyper_client::HyperClient, Proxy};

#[derive(Parser, Debug)]
#[command(version)]
pub struct Args {
    #[arg(short, long, default_value_t = String::from("production"), env("APP_RUN_MODE"))]
    pub run_mode: String,
    #[arg(short, long, default_value_t = String::from("./config"), env("APP_CONFIG_PATH"))]
    pub config_path: String,
}

static FLOW_ROUTER: OnceCell<FlowRouter> = OnceCell::new();

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
        let router = get_flow_router();

        let result = router
            .handle(
                &RequestType::Salvo(&SalvoRequest::new(&req)),
                &ResponseType::Salvo(&mut SalvoResponse::new(res)),
            )
            .await
            .unwrap();

        match result {
            FlowRouterResult::Empty(statu_code) => res.status_code(statu_code).render(""),
            FlowRouterResult::Json(content, statu_code) => {
                res.status_code(statu_code).render(Json(content))
            }
            FlowRouterResult::PlainText(content, statu_code) => {
                res.status_code(statu_code).render(content)
            }
            FlowRouterResult::Proxied(url, statu_code) => {
                res.status_code(statu_code);

                Proxy::new(url.to_string(), HyperClient::default())
                    .handle(req, depot, res, ctrl)
                    .await;
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
            FlowRouterResult::Error => res
                .status_code(StatusCode::INTERNAL_SERVER_ERROR)
                .render(""),
        }
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

    dotenv::from_filename("./click-router/.env").ok();

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
        .with_dynamo()
        .await
        .build();

    let _ = FLOW_ROUTER.set(flow_router);

    let router = Router::with_path("{**rest_path}").get(Redirect);

    println!("{:?}", router);

    let acceptor = TcpListener::new("0.0.0.0:5800")
        .rustls_async(ServerConfigResolverMock)
        .bind()
        .await;

    // let acceptor = TcpListener::new("127.0.0.1:5800").bind().await;
    Server::new(acceptor).serve(router).await;
}
