use crate::core::BaseRoutesStore;

#[derive(Clone)]
pub struct AppState {
    pub routes_store: Box<dyn BaseRoutesStore + Send + Sync>,
}

impl AppState {
    pub fn new(routes_store: Box<dyn BaseRoutesStore + Send + Sync>) -> Self {
        AppState {
            routes_store: routes_store,
        }
    }
}
