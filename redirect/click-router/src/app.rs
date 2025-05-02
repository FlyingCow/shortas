use aws_config::SdkConfig;
use tracing::info;

use crate::{
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
            crypto_cache::MokaCryptoCache, routes_cache::MokaRoutesCache, settings::Moka,
            user_settings_cache::MokaUserSettingsCache,
        },
        uaparser::user_agent_detector::UAParserUserAgentDetector,
        CryptoCacheType, CryptoStoreType, HitRegistrarType, LocationDetectorType, RoutesCacheType,
        RoutesStoreType, UserAgentDetectorType, UserSettingsCacheType, UserSettingsStoreType,
    },
    core::{
        flow_router::FlowRouter,
        modules::{
            conditional::ConditionalModule, not_found::NotFoundModule,
            redirect_only::RedirectOnlyModule, root::RootModule, FlowModules,
        },
    },
    settings::Settings,
};

// #[derive(TypedBuilder)]
// #[builder(field_defaults(setter(prefix = "with_")))]
#[derive(Default)]
pub struct AppBuilder {
    settings: Settings,
    modules: Vec<FlowModules>,
    user_settings_cache: Option<UserSettingsCacheType>,
    routes_cache: Option<RoutesCacheType>,
    crypto_cache: Option<CryptoCacheType>,
    user_agent_detector: Option<UserAgentDetectorType>,
    location_detector: Option<LocationDetectorType>,
    hit_registrar: Option<HitRegistrarType>,
}

impl AppBuilder {
    async fn load_aws_config(&self, settings: AWS) -> SdkConfig {
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

    async fn init_dynamo_stores(
        &self,
        settings: &AWS,
    ) -> (
        DynamoRoutesStore,
        DynamoCryptoStore,
        DynamoUserSettingsStore,
    ) {
        let aws_config = &self.load_aws_config(settings.clone()).await;

        let routes_store =
            DynamoRoutesStore::new(&aws_config, settings.dynamo.routes_table.clone());

        let crypto_store =
            DynamoCryptoStore::new(&aws_config, settings.dynamo.encryption_table.clone());

        let user_settings_store =
            DynamoUserSettingsStore::new(&aws_config, settings.dynamo.user_settings_table.clone());

        (routes_store, crypto_store, user_settings_store)
    }

    async fn init_moka_cache_with_dynamo_stores(
        &self,
        moka_settings: &Moka,
        aws_settings: &AWS,
    ) -> (RoutesCacheType, CryptoCacheType, UserSettingsCacheType) {
        let (routes_store, crypto_store, user_settings_store) =
            self.init_dynamo_stores(&aws_settings).await;

        let routes_cache = RoutesCacheType::Moka(MokaRoutesCache::new(
            RoutesStoreType::Dynamo(routes_store),
            moka_settings.routes_cache.clone(),
        ));

        let crypto_cache = CryptoCacheType::Moka(MokaCryptoCache::new(
            CryptoStoreType::Dynamo(crypto_store),
            moka_settings.crypto_cache.clone(),
        ));

        let user_settings_cache = UserSettingsCacheType::Moka(MokaUserSettingsCache::new(
            UserSettingsStoreType::Dynamo(user_settings_store),
            moka_settings.user_settings_cache.clone(),
        ));

        (routes_cache, crypto_cache, user_settings_cache)
    }

    pub async fn with_dynamo(mut self) -> Self {
        let (routes_cache, crypto_cache, user_settings_cache) = self
            .init_moka_cache_with_dynamo_stores(&self.settings.moka, &self.settings.aws)
            .await;

        self.crypto_cache = Some(crypto_cache);
        self.routes_cache = Some(routes_cache);
        self.user_settings_cache = Some(user_settings_cache);

        self
    }

    pub async fn with_fluvio(mut self) -> Self {
        let hit_registrar = HitRegistrarType::Fluvio(
            FluvioHitRegistrar::new(&self.settings.fluvio.hit_stream).await,
        );

        self.hit_registrar = Some(hit_registrar);

        self
    }

    pub fn with_geo_ip(mut self) -> Self {
        let location_detector =
            LocationDetectorType::GeoIP(GeoIPLocationDetector::new(&self.settings.geo_ip));

        self.location_detector = Some(location_detector);

        self
    }

    pub fn with_ua_parser(mut self) -> Self {
        let user_agent_detector = UserAgentDetectorType::UAParser(UAParserUserAgentDetector::new(
            &self.settings.uaparser,
        ));

        self.user_agent_detector = Some(user_agent_detector);

        self
    }

    pub fn with_default_modules(mut self) -> Self {
        self.modules.push(FlowModules::Root(RootModule::new()));

        self.modules
            .push(FlowModules::Conditional(ConditionalModule::new()));

        self.modules
            .push(FlowModules::NotFound(NotFoundModule::new()));

        self.modules
            .push(FlowModules::RedirectOnly(RedirectOnlyModule::new()));

        self
    }

    pub fn new(settings: Settings) -> Self {
        Self {
            settings,
            ..Default::default()
        }
    }

    pub fn build(self) -> FlowRouter {
        FlowRouter::default(
            self.routes_cache.clone().unwrap(),
            self.user_settings_cache.clone().unwrap(),
            self.user_agent_detector.clone().unwrap(),
            self.location_detector.clone().unwrap(),
            self.hit_registrar.clone().unwrap(),
            self.modules.clone(),
        )
    }
}
