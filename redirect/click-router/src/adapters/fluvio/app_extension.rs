use tracing::info;

use crate::{adapters::fluvio::hit_registrar::FluvioHitRegistrar, AppBuilder};

impl AppBuilder {
    pub async fn with_fluvio(&mut self) -> &mut Self {
        info!("{}", "WITH FLUVIO");

        let hit_registrar = Some(Box::new(
            FluvioHitRegistrar::new(self.settings.fluvio.hit_stream.clone()).await,
        ) as Box<_>);

        self.hit_registrar = hit_registrar;

        self
    }
}
