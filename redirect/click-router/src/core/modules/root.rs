use std::str::FromStr;

use anyhow::Result;
use http::{StatusCode, Uri};
use string_format::*;

use crate::{
    core::{
        flow_module::{FlowModule, FlowStepContinuation},
        flow_router::{FlowRouter, FlowRouterContext, FlowRouterResult, RedirectType, Request},
    },
    settings::Redirect,
};

static IS_ROOT: &str = "is_root";
const INDEX_SWITCH: &str = "index";

#[derive(Debug, Clone)]
pub struct RootModule {
    redirect: Redirect,
}

impl RootModule {
    pub fn new(redirect: Redirect) -> Self {
        Self { redirect }
    }
}

#[async_trait::async_trait()]
impl FlowModule for RootModule {
    async fn handle_start(
        &self,
        context: &mut FlowRouterContext,
        flow_router: &FlowRouter,
    ) -> Result<FlowStepContinuation> {
        if context.request.uri().path() == "/" {
            let domain = context.in_route.host.as_str();
            let index_path = format!("{}%2F", domain);

            // Check for custom index route: switch="index", link="{domain}%2F"
            if let Ok(Some(route)) = flow_router.get_route_by_switch(INDEX_SWITCH, &index_path).await {
                if let Some(ref dest) = route.dest {
                    context.result = Some(FlowRouterResult::Redirect(
                        Uri::from_str(dest).unwrap(),
                        RedirectType::Temporary,
                    ));
                    context.add_bool(IS_ROOT, true);
                    return Ok(FlowStepContinuation::Break);
                }
            }

            // Fall back to global index URL
            let root_uri = string_format!(
                self.redirect.index_url.clone(),
                context.request.uri().host().unwrap().to_string()
            );

            context.result = Some(FlowRouterResult::Proxied(
                Uri::from_str(&root_uri).unwrap(),
                StatusCode::OK,
            ));

            context.add_bool(IS_ROOT, true);

            return Ok(FlowStepContinuation::Break);
        }

        Ok(FlowStepContinuation::Continue)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_create_root_module() {
        let redirect = Redirect {
            index_url: "https://example.com".to_string(),
            not_found_url: "https://example.com/404".to_string(),
            blocked_url: "https://example.com/blocked".to_string(),
        };
        let module = RootModule::new(redirect.clone());

        assert_eq!(module.redirect.index_url, "https://example.com");
    }

    #[test]
    fn should_clone_root_module() {
        let redirect = Redirect {
            index_url: "https://example.com".to_string(),
            not_found_url: "https://example.com/404".to_string(),
            blocked_url: "https://example.com/blocked".to_string(),
        };
        let module = RootModule::new(redirect);
        let cloned = module.clone();

        assert_eq!(cloned.redirect.index_url, "https://example.com");
    }
}
