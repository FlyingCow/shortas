use tracing::info;

use crate::{adapters::uaparser::user_agent_detector::UAParserUserAgentDetector, AppBuilder};


impl AppBuilder {
    pub fn with_uaparser(&mut self) -> &mut Self {
        info!("{}", "WITH UAPARSER");

        let user_agent_detector = Some(Box::new(UAParserUserAgentDetector::new(
            &self.settings.uaparser.yaml
        ))as Box<_>);

        self.user_agent_detector = user_agent_detector;
        
        self
    }
}
