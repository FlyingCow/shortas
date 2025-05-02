use std::{
    io::{Error as IoError, Result as IoResult},
    ops::Deref,
    sync::Arc,
};

use click_router::{
    app::AppBuilder,
    core::flow_router::{FlowRouter, RequestData, ResponseData},
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
        _depot: &mut Depot,
        res: &mut Response,
        _ctrl: &mut FlowCtrl,
    ) {
        let router = get_flow_router();

        let result = router
            .handle(
                RequestData {
                    headers: req.headers().clone(),
                    uri: req.uri().clone(),
                    extensions: req.extensions().clone(),
                    method: req.method().clone(),
                    cookies: req.cookies().clone(),
                    params: req.params().deref().clone(),
                    queries: OnceCell::with_value(req.queries().clone()),
                    version: req.version().clone(),
                    scheme: Some(req.scheme().clone()),
                    local_addr: req.local_addr().clone().into_std(),
                    remote_addr: req.remote_addr().clone().into_std(),
                    tls_info: None,
                },
                ResponseData {
                    cookies: res.cookies.clone(),
                    extensions: res.extensions.clone(),
                    headers: res.headers.clone(),
                    status_code: res.status_code,
                    version: res.version,
                },
            )
            .await
            .unwrap();

        res.render(Text::Plain(result.to_string()));
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
