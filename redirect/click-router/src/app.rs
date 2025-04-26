use anyhow::Result;
use tracing::info;
use typed_builder::TypedBuilder;

use crate::{
    adapters::{
        CryptoCacheType, HitRegistrarType, LocationDetectorType, RoutesCacheType,
        UserAgentDetectorType, UserSettingsCacheType,
    },
    core::{
        expression::ExpressionEvaluator,
        flow_router::FlowRouter,
        host::HostExtractor,
        ip::IPExtractor,
        language::LanguageExtractor,
        modules::{
            conditional::ConditionalModule, not_found::NotFoundModule,
            redirect_only::RedirectOnlyModule, root::RootModule, FlowModules,
        },
        protocol::ProtocolExtractor,
        routes::RoutesManager,
        user_agent_string::UserAgentStringExtractor,
        user_settings::UserSettingsManager,
    },
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

impl App {
    pub fn get_router(&self) -> FlowRouter {
        let routes_manager = RoutesManager::new(self.routes_cache.clone());
        let settings_manager = UserSettingsManager::new(self.user_settings_cache.clone());
        let host_extractor = HostExtractor::new();
        let protocol_extractor = ProtocolExtractor::new();
        let ip_extractor = IPExtractor::new();
        let user_agent_string_extractor = UserAgentStringExtractor::new();
        let language_extractor = LanguageExtractor::new();

        let root_module = FlowModules::Root(RootModule {});
        let conditional_module =
            FlowModules::Conditional(ConditionalModule::new(ExpressionEvaluator::new()));
        let not_found_module = FlowModules::NotFound(NotFoundModule {});
        let redirect_only =
            FlowModules::RedirectOnly(RedirectOnlyModule::new(settings_manager.clone()));

        let hit_registrar = self.hit_registrar.clone();
        let user_agent_detector = self.user_agent_detector.clone();
        let location_detector = self.location_detector.clone();

        FlowRouter::new(
            routes_manager,
            settings_manager,
            hit_registrar,
            host_extractor,
            protocol_extractor,
            ip_extractor,
            user_agent_string_extractor,
            language_extractor,
            user_agent_detector,
            location_detector,
            vec![
                root_module,
                not_found_module,
                conditional_module,
                redirect_only,
            ],
        )
    }
}
