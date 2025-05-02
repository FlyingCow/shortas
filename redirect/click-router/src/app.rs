use aws_config::SdkConfig;
use tracing::info;
use typed_builder::TypedBuilder;

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

#[derive(TypedBuilder)]
#[builder(field_defaults(setter(prefix = "with_")))]
pub struct App {
    modules: Vec<FlowModules>,
    user_settings_cache: UserSettingsCacheType,
    routes_cache: RoutesCacheType,
    crypto_cache: CryptoCacheType,
    user_agent_detector: UserAgentDetectorType,
    location_detector: LocationDetectorType,
    hit_registrar: HitRegistrarType,
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

    pub async fn dynamo(&self, settings: Settings) -> Self {
        let (routes_cache, crypto_cache, user_settings_cache) = &self
            .init_moka_cache_with_dynamo_stores(&settings.moka, &settings.aws)
            .await;

        let hit_registrar =
            HitRegistrarType::Fluvio(FluvioHitRegistrar::new(&settings.fluvio.hit_stream).await);

        let location_detector =
            LocationDetectorType::GeoIP(GeoIPLocationDetector::new(&settings.geo_ip));

        let user_agent_detector =
            UserAgentDetectorType::UAParser(UAParserUserAgentDetector::new(&settings.uaparser));

        self.with_crypto_cache(crypto_cache)
            .with_routes_cache(routes_cache)
            .with_user_settings_cache(user_settings_cache)
            .with_hit_registrar(hit_registrar)
            .with_location_detector(location_detector)
            .with_user_agent_detector(user_agent_detector)
            .with_modules(vec![
                FlowModules::Root(RootModule::new()),
                FlowModules::Conditional(ConditionalModule::new()),
                FlowModules::NotFound(NotFoundModule::new()),
                FlowModules::RedirectOnly(RedirectOnlyModule::new()),
            ])
            .clone()
    }
}

impl App {
    pub fn get_router(&self) -> FlowRouter {
        FlowRouter::default(
            self.routes_cache.clone(),
            self.user_settings_cache.clone(),
            self.user_agent_detector.clone(),
            self.location_detector.clone(),
            self.hit_registrar.clone(),
            self.modules.clone(),
        )
    }
}
