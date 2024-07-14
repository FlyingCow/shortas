
use tracing::info;

use crate::AppBuilder;

impl AppBuilder {
    pub async fn with_kafka(&mut self) -> &mut Self {
        info!("{}", "WITH KAFKA");

        // let hit_registrar = Some(Box::new(KinesisHitRegistrar::new(
        //     &config,
        //     self.settings..kinesis.hit_stream.clone(),
        // )) as Box<_>);

        // self.hit_registrar = hit_registrar;

        self
    }
}
