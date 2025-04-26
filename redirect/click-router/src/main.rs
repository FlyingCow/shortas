use std::{
    io::{Error as IoError, Result as IoResult},
    ops::Deref,
    sync::Arc,
};

use aws_config::SdkConfig;
use click_router::{
    adapters::{
        aws::{
            dynamo::{
                crypto_store::DynamoCryptoStore, routes_store::DynamoRoutesStore,
                user_settings_store::DynamoUserSettingsStore,
            },
            settings::AWS,
        },
        fluvio::hit_registrar::FluvioHitRegistrar,
        geo_ip::geo_ip_location_detector::GeoIPLocationDetector,
        moka::{
            crypto_cache::MokaCryptoCache, routes_cache::MokaRoutesCache,
            user_settings_cache::MokaUserSettingsCache,
        },
        uaparser::user_agent_detector::UAParserUserAgentDetector,
        CryptoCacheType, CryptoStoreType, HitRegistrarType, LocationDetectorType, RoutesCacheType,
        RoutesStoreType, UserAgentDetectorType, UserSettingsCacheType, UserSettingsStoreType,
    },
    app::App,
    core::{
        expression::ExpressionEvaluator,
        flow_router::{FlowRouter, RequestData, ResponseData},
        modules::{
            conditional::ConditionalModule, not_found::NotFoundModule,
            redirect_only::RedirectOnlyModule, root::RootModule, FlowModules,
        },
        user_settings::UserSettingsManager,
    },
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

async fn load_aws_config(settings: AWS) -> SdkConfig {
    let mut shared_config = aws_config::defaults(aws_config::BehaviorVersion::latest());

    if settings.local {
        let endpoint = settings
            .localstack_endpoint
            .unwrap_or("http://localhost:4566".to_string());

        info!("  {} -> {}", "localstack", endpoint);
        shared_config = shared_config.endpoint_url(endpoint);
    }

    shared_config.load().await
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

    let aws_config = load_aws_config(settings.aws.clone()).await;

    let routes_store =
        DynamoRoutesStore::new(&aws_config, settings.aws.dynamo.routes_table.clone());

    let crypto_store =
        DynamoCryptoStore::new(&aws_config, settings.aws.dynamo.encryption_table.clone());

    let user_settings_store =
        DynamoUserSettingsStore::new(&aws_config, settings.aws.dynamo.user_settings_table.clone());

    let crypto_cache = CryptoCacheType::Moka(MokaCryptoCache::new(
        CryptoStoreType::Dynamo(crypto_store),
        settings.moka.crypto_cache,
    ));

    let routes_cache = RoutesCacheType::Moka(MokaRoutesCache::new(
        RoutesStoreType::Dynamo(routes_store),
        settings.moka.routes_cache,
    ));

    let user_settings_cache = UserSettingsCacheType::Moka(MokaUserSettingsCache::new(
        UserSettingsStoreType::Dynamo(user_settings_store),
        settings.moka.user_settings_cache,
    ));

    let hit_registrar =
        HitRegistrarType::Fluvio(FluvioHitRegistrar::new(&settings.fluvio.hit_stream).await);

    let location_detector =
        LocationDetectorType::GeoIP(GeoIPLocationDetector::new(&settings.geo_ip));

    let user_agent_detector =
        UserAgentDetectorType::UAParser(UAParserUserAgentDetector::new(&settings.uaparser));

    let root_module = FlowModules::Root(RootModule {});
    let conditional_module =
        FlowModules::Conditional(ConditionalModule::new(ExpressionEvaluator {}));
    let not_found_module = FlowModules::NotFound(NotFoundModule {});
    let redirect_only_module = FlowModules::RedirectOnly(RedirectOnlyModule::new(
        UserSettingsManager::new(user_settings_cache.clone()),
    ));

    let app = App::builder()
        .with_crypto_cache(crypto_cache)
        .with_routes_cache(routes_cache)
        .with_user_settings_cache(user_settings_cache)
        .with_hit_registrar(hit_registrar)
        .with_location_detector(location_detector)
        .with_user_agent_detector(user_agent_detector)
        .with_modules(vec![
            root_module,
            conditional_module,
            not_found_module,
            redirect_only_module,
        ])
        .build()
        .get_router();

    let _ = FLOW_ROUTER.set(app);

    let router = Router::with_path("{**rest_path}").get(Redirect);
    //let router = Router::with_path("conds").get(Redirect);

    println!("{:?}", router);

    let acceptor = TcpListener::new("0.0.0.0:5800")
        .rustls_async(ServerConfigResolverMock)
        .bind()
        .await;

    // let acceptor = TcpListener::new("127.0.0.1:5800").bind().await;
    Server::new(acceptor).serve(router).await;
}
