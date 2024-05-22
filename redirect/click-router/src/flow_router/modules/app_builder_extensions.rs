use crate::{
    app_builder::AppBuilder,
    flow_router::modules::{
        not_found_module::NotFoundModule, redirect_only_module::RedirectOnlyModule,
        root_module::RootModule,
    },
};

impl AppBuilder {
    pub fn with_default_modules(&mut self) -> &mut Self {
        self.modules.push(Box::new(RootModule {}));
        self.modules.push(Box::new(NotFoundModule {}));
        self.modules.push(Box::new(RedirectOnlyModule::new(
            self.user_settings_manager.clone().unwrap(),
        )));

        println!("{}", "WITH DEFAULT FLOW MODULES");

        self
    }
}
