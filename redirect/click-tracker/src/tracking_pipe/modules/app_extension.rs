use tracing::info;

use crate::{
    app::AppBuilder,
};

impl AppBuilder {
    pub fn with_default_modules(&mut self) -> &mut Self {

        info!("{}", "WITH DEFAULT FLOW MODULES");

        self
    }
}
