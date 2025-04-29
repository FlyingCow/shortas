use typed_builder::TypedBuilder;

use crate::{
    adapters::{
        CryptoCacheType, HitRegistrarType, LocationDetectorType, RoutesCacheType,
        UserAgentDetectorType, UserSettingsCacheType,
    },
    core::{flow_router::FlowRouter, modules::FlowModules},
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
