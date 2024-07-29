use tracing::info;

use crate::app::AppBuilder;

pub struct DefaultsBuilder {}

impl AppBuilder {
    pub fn with_defaults(&mut self) -> &mut Self {
 

        info!("{}", "WITH FLOW DEFAULTS");

        self
    }
}
