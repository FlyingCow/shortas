use anyhow::Result;

use crate::{
    core::{
        BaseCryptoManager, BaseCryptoStore, BaseRoutesManager, BaseRoutesStore, BaseUserSettingsManager, BaseUserSettingsStore
    },
    flow_router::{base_flow_module::BaseFlowModule, base_host_detector::BaseHostDetector, base_ip_detector::BaseIPDetector, base_protocol_detector::BaseProtocolDetector, default_flow_router::DefaultFlowRouter},
    settings::Settings,
};
#[derive(Clone)]
pub struct AppBuilder {
    pub(super) settings: Settings,
    pub(super) routes_store: Option<Box<dyn BaseRoutesStore + 'static>>,
    pub(super) routes_manager: Option<Box<dyn BaseRoutesManager + 'static>>,
    pub(super) host_detector: Option<Box<dyn BaseHostDetector + 'static>>,
    pub(super) protocol_detector: Option<Box<dyn BaseProtocolDetector + 'static>>,
    pub(super) ip_detector: Option<Box<dyn BaseIPDetector + 'static>>,
    pub(super) crypto_store: Option<Box<dyn BaseCryptoStore + 'static>>,
    pub(super) crypto_manager: Option<Box<dyn BaseCryptoManager + 'static>>,
    pub(super) user_settings_store: Option<Box<dyn BaseUserSettingsStore + 'static>>,
    pub(super) user_settings_manager: Option<Box<dyn BaseUserSettingsManager + 'static>>,
    pub(super) modules: Vec<Box<dyn BaseFlowModule + 'static>>,
}

impl AppBuilder {
    pub fn new(settings: Settings) -> Self {
        Self {
            settings,
            routes_store: None,
            routes_manager: None,
            crypto_store: None,
            crypto_manager: None,
            user_settings_store: None,
            user_settings_manager: None,
            host_detector:None,
            ip_detector: None,
            protocol_detector: None,
            modules: vec![],
        }
    }

    pub fn build(&self) -> Result<DefaultFlowRouter> {
        println!("{}", "BUILDING");

        let router = DefaultFlowRouter::new(
            self.routes_manager.clone().unwrap(),
            self.host_detector.clone().unwrap(),
            self.protocol_detector.clone().unwrap(),
            self.ip_detector.clone().unwrap(),
            self.modules.clone(),
        );

        Ok(router)
    }
}
