use tracing::info;

use crate::{
    app::AppBuilder,
    flow_router::modules::{
        conditional::ConditionalModule, not_found::NotFoundModule,
        redirect_only::RedirectOnlyModule, root::RootModule,
    },
};

impl AppBuilder {
    pub fn with_default_modules(&mut self) -> &mut Self {
        self.modules.push(Box::new(RootModule {}));
        self.modules.push(Box::new(NotFoundModule {}));
        self.modules.push(Box::new(RedirectOnlyModule::new(
            self.user_settings_manager.clone().unwrap(),
        )));
        self.modules.push(Box::new(ConditionalModule::new(
            self.expression_evaluator.clone().unwrap(),
        )));

        info!("{}", "WITH DEFAULT FLOW MODULES");

        self
    }
}
