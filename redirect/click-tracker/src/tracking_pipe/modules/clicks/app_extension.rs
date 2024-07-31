use tracing::info;

use crate::{
    app::AppBuilder,
    tracking_pipe::modules::clicks::{
        enrich_location_module::EnrichLocationModule,
        enrich_user_agent_module::EnrichUserAgentModule, init_module::InitModule,
        register_aggregate_module::RegisterAggregateModule,
    },
};

impl AppBuilder {
    pub fn with_default_click_modules(&mut self) -> &mut Self {
        info!("{}", "WITH DEFAULT CLICK MODULES");
        self.modules.push(Box::new(InitModule {}));
        self.modules.push(Box::new(EnrichLocationModule::new(
            self.location_detector.clone().unwrap(),
        )));
        self.modules.push(Box::new(EnrichUserAgentModule::new(
            self.user_agent_detector.clone().unwrap(),
        )));

        //
        self.modules.push(Box::new(RegisterAggregateModule::new(
            self.click_aggs_registrar.clone().unwrap(),
        )));
        self
    }
}
