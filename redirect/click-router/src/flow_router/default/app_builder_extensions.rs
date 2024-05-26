use crate::{
    app_builder::AppBuilder,
    flow_router::default::{
        default_host_extractor::DefaultHostExtractor, default_ip_extractor::DefaultIPExtractor,
        default_protocol_extractor::DefaultProtocolExtractor,
    },
};

pub struct DefaultsBuilder {}

impl AppBuilder {
    pub fn with_flow_defaults(&mut self) -> &mut Self {
        let protocol_extractor = Some(Box::new(DefaultProtocolExtractor::new()) as Box<_>);

        let host_extractor = Some(Box::new(DefaultHostExtractor::new()) as Box<_>);

        let ip_extractor = Some(Box::new(DefaultIPExtractor::new()) as Box<_>);

        self.protocol_extractor = protocol_extractor;
        self.host_extractor = host_extractor;
        self.ip_extractor = ip_extractor;

        println!("{}", "WITH FLOW DEFAULTS");

        self
    }
}
