use anyhow::Result;

use crate::{core::BaseRoutesStore, settings::Settings};

#[derive(Clone)]
pub struct AppBuilder {
    pub(super) settings: Settings,
    pub(super) routes_store: Option<Box<dyn BaseRoutesStore + 'static>>,
}

pub struct Api {}
impl Api {
    fn new() -> Self {
        Api {}
    }
}

impl AppBuilder {
    pub fn new(settings: Settings) -> Self {
        Self {
            settings,
            routes_store: None,
        }
    }

    pub fn build(&self) -> Result<Api> {
        println!("{}", "BUILDING");

        let router = Api::new();

        Ok(router)
    }
}
