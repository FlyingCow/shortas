use anyhow::Result;

use crate::core::{BaseRoutesManager, BaseRoutesStore};

use crate::model::Route;

#[derive(Clone)]
pub struct DefaultRoutesManager {
    routes_store: Box<dyn BaseRoutesStore>,
}

fn get_key(domain: &str, link: &str) -> String {
    let domain_str = domain.to_ascii_lowercase();
    let link_str = link.to_ascii_lowercase();

    domain_str + "%2F" + &link_str
}

#[async_trait::async_trait(?Send)]
impl BaseRoutesManager for DefaultRoutesManager {
    async fn get_route(
        &self,
        switch: &'static str,
        domain: &str,
        path: &str,
    ) -> Result<Option<Route>> {
        let key = get_key(domain, path);

        let route_result = self.routes_store.get_route(switch, key.as_str()).await;

        Ok(route_result.unwrap())
    }
}

impl DefaultRoutesManager {
    pub fn new(routes_store: Box<dyn BaseRoutesStore>) -> Self {
        Self { routes_store }
    }
}
