use crate::{
    app_builder::AppBuilder,
    flow_router::default::{
        default_expression_evaluator::DefaultExpressionEvaluator,
        default_host_extractor::DefaultHostExtractor, default_ip_extractor::DefaultIPExtractor,
        default_language_extractor::DefaultLanguageExtractor,
        default_protocol_extractor::DefaultProtocolExtractor,
        default_user_agent_string_extractor::DefaultUserAgentStringExtractor,
    },
};

pub struct DefaultsBuilder {}

impl AppBuilder {
    pub fn with_flow_defaults(&mut self) -> &mut Self {
        let protocol_extractor = Some(Box::new(DefaultProtocolExtractor::new()) as Box<_>);

        let host_extractor = Some(Box::new(DefaultHostExtractor::new()) as Box<_>);

        let ip_extractor = Some(Box::new(DefaultIPExtractor::new()) as Box<_>);

        let user_agent_string_extractor =
            Some(Box::new(DefaultUserAgentStringExtractor::new()) as Box<_>);

        let language_extractor = Some(Box::new(DefaultLanguageExtractor::new()) as Box<_>);

        let expression_evaluator = Some(Box::new(DefaultExpressionEvaluator::new()) as Box<_>);

        self.protocol_extractor = protocol_extractor;
        self.host_extractor = host_extractor;
        self.ip_extractor = ip_extractor;
        self.user_agent_string_extractor = user_agent_string_extractor;
        self.language_extractor = language_extractor;
        self.expression_evaluator = expression_evaluator;

        println!("{}", "WITH FLOW DEFAULTS");

        self
    }
}
