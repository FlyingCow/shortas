use crate::{
    app_builder::AppBuilder,
    flow_router::default::{
        default_host_detector::DefaultHostDetector, default_ip_detector::DefaultIPDetector,
        default_protocol_detector::DefaultProtocolDetector,
    },
};

pub struct DefaultsBuilder {}

impl AppBuilder {
    pub fn with_flow_defaults(&mut self) -> &mut Self {
        let protocol_detector = Some(Box::new(DefaultProtocolDetector::new()) as Box<_>);

        let host_detector = Some(Box::new(DefaultHostDetector::new()) as Box<_>);

        let ip_detector = Some(Box::new(DefaultIPDetector::new()) as Box<_>);

        self.protocol_detector = protocol_detector;
        self.host_detector = host_detector;
        self.ip_detector = ip_detector;

        println!("{}", "WITH FLOW DEFAULTS");

        self
    }
}
