
use tracing::info;

use crate::{adapters::kafka::hit_stream::KafkaHitStream, AppBuilder};

impl AppBuilder {
    pub async fn with_kafka(&mut self) -> &mut Self {
        info!("{}", "WITH KAFKA");


        let hit_stream = Some(Box::new(KafkaHitStream::new(
            self.settings.kafka.hit_stream.clone(),
        )) as Box<_>);

        self.hit_stream = hit_stream;

        self
    }
}
