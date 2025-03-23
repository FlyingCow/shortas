use tracing::info;

use crate::{
    adapters::fluvio::{
        click_aggs_registrar::FluvioClickAggsRegistrar, hit_stream::FluvioHitStream,
    },
    AppBuilder,
};

impl AppBuilder {
    pub async fn with_fluvio(&mut self) -> &mut Self {
        info!("{}", "WITH FLUVIO");

        let hit_stream = Some(Box::new(
            FluvioHitStream::new(self.settings.fluvio.hit_stream.clone()).await,
        ) as Box<_>);

        let click_aggs_registrar = Some(Box::new(
            FluvioClickAggsRegistrar::new(self.settings.fluvio.click_aggs.clone()).await,
        ) as Box<_>);

        self.hit_stream = hit_stream;
        self.click_aggs_registrar = click_aggs_registrar;

        self
    }
}
