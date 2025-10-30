pub mod adapters;
pub mod app;
pub mod core;
pub mod model;
pub mod settings;
pub mod utils;

pub use app::AppBuilder;

use std::sync::OnceLock;
use crate::core::flow_router::FlowRouter;

static FLOW_ROUTER: OnceLock<FlowRouter> = OnceLock::new();

/// Get a reference to the global flow router instance
pub fn get_flow_router() -> &'static FlowRouter {
    FLOW_ROUTER.get().unwrap()
}

/// Initialize the global flow router instance
pub fn init_flow_router(router: FlowRouter) {
    let _ = FLOW_ROUTER.set(router);
}
