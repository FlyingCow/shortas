use tracing::info;

use crate::{
    app::AppBuilder,
    tracking_pipe::default::{
//
    },
};

pub struct DefaultsBuilder {}

impl AppBuilder {
    pub fn with_defaults(&mut self) -> &mut Self {
 

        info!("{}", "WITH FLOW DEFAULTS");

        self
    }
}
