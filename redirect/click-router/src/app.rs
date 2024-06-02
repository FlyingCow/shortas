use anyhow::Result;

use crate::{
    core::{
        location_detect::BaseLocationDetector,
        user_agent_detect::BaseUserAgentDetector, BaseCryptoManager, BaseCryptoStore,
        BaseRoutesManager, BaseRoutesStore, BaseUserSettingsManager, BaseUserSettingsStore,
    },
    flow_router::{
        expression_evaluate::BaseExpressionEvaluator, flow_module::BaseFlowModule, host_extract::BaseHostExtractor, ip_extract::BaseIPExtractor, language_extract::BaseLanguageExtractor, protocol_extract::BaseProtocolExtractor, user_agent_string_extract::BaseUserAgentStringExtractor, default_flow_router::DefaultFlowRouter
    },
    settings::Settings,
};
#[derive(Clone)]
pub struct AppBuilder {
    pub(super) settings: Settings,
    pub(super) routes_store: Option<Box<dyn BaseRoutesStore + Send + Sync + 'static>>,
    pub(super) routes_manager: Option<Box<dyn BaseRoutesManager + Send + Sync + 'static>>,
    pub(super) host_extractor: Option<Box<dyn BaseHostExtractor + Send + Sync + 'static>>,
    pub(super) protocol_extractor: Option<Box<dyn BaseProtocolExtractor + Send + Sync + 'static>>,
    pub(super) ip_extractor: Option<Box<dyn BaseIPExtractor + Send + Sync + 'static>>,
    pub(super) user_agent_string_extractor: Option<Box<dyn BaseUserAgentStringExtractor + Send + Sync + 'static>>,
    pub(super) language_extractor: Option<Box<dyn BaseLanguageExtractor + Send + Sync + 'static>>,

    pub(super) crypto_store: Option<Box<dyn BaseCryptoStore + Send + Sync + 'static>>,
    pub(super) crypto_manager: Option<Box<dyn BaseCryptoManager + Send + Sync + 'static>>,
    pub(super) user_settings_store: Option<Box<dyn BaseUserSettingsStore + Send + Sync + 'static>>,
    pub(super) user_settings_manager: Option<Box<dyn BaseUserSettingsManager + Send + Sync + 'static>>,
    pub(super) user_agent_detector: Option<Box<dyn BaseUserAgentDetector + Send + Sync + 'static>>,
    pub(super) expression_evaluator: Option<Box<dyn BaseExpressionEvaluator + Send + Sync + 'static>>,
    
    
    pub(super) location_detector: Option<Box<dyn BaseLocationDetector + Send + Sync + 'static>>,
    pub(super) modules: Vec<Box<dyn BaseFlowModule + Send + Sync + 'static>>,
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
            host_extractor: None,
            ip_extractor: None,
            user_agent_string_extractor: None,
            language_extractor: None,
            protocol_extractor: None,
            user_agent_detector: None,
            location_detector: None,
            expression_evaluator: None,
            modules: vec![],
        }
    }

    pub fn build(&self) -> Result<DefaultFlowRouter> {
        println!("{}", "BUILDING");

        let router = DefaultFlowRouter::new(
            self.routes_manager.clone().unwrap(),
            self.host_extractor.clone().unwrap(),
            self.protocol_extractor.clone().unwrap(),
            self.ip_extractor.clone().unwrap(),
            self.user_agent_string_extractor.clone().unwrap(),
            self.language_extractor.clone().unwrap(),
            self.user_agent_detector.clone().unwrap(),
            self.location_detector.clone().unwrap(),
            self.modules.clone(),
        );

        Ok(router)
    }
}
