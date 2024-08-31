use tracing::info;

use crate::{adapters::redis::session_detector::RedisSessionDetector, AppBuilder};

impl AppBuilder {
    pub fn with_redis(&mut self) -> &mut Self {
        info!("{}", "WITH REDIS");

        let session_detector = Some(Box::new(RedisSessionDetector::new(
            self.settings
                .redis
                .initial_nodes
                .iter()
                .map(|node| node.as_str())
                .collect(),
        )) as Box<_>);

        self.session_detector = session_detector;

        self
    }
}
