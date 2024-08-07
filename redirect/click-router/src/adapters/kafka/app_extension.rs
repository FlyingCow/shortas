
use tracing::info;

use crate::{adapters::kafka::hit_registrar::KafkaHitRegistrar, AppBuilder};

impl AppBuilder {
    pub async fn with_kafka(&mut self) -> &mut Self {
        info!("{}", "WITH KAFKA");

        let hit_registrar = Some(Box::new(KafkaHitRegistrar::new(
            self.settings.kafka.hit_stream.clone(),
        )) as Box<_>);

        self.hit_registrar = hit_registrar;

        self
    }
}
