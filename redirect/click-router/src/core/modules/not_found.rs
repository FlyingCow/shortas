use std::str::FromStr;

use anyhow::{Ok, Result};
use http::{StatusCode, Uri};
use string_format::*;

use crate::{
    core::{
        flow_module::{FlowModule, FlowStepContinuation},
        flow_router::{
            FlowRouter, FlowRouterContext, FlowRouterResult, FlowStep, RedirectType, Request,
        },
    },
    settings::Redirect,
};

const IS_404: &str = "is_404";
const NOT_FOUND_SWITCH: &str = "404";

#[derive(Debug, Clone)]
pub struct NotFoundModule {
    redirect: Redirect,
}

impl NotFoundModule {
    pub fn new(redirect: Redirect) -> Self {
        Self { redirect }
    }
}

#[async_trait::async_trait()]
impl FlowModule for NotFoundModule {
    async fn handle_start(
        &self,
        context: &mut FlowRouterContext,
        flow_router: &FlowRouter,
    ) -> Result<FlowStepContinuation> {
        if context.main_route.is_none() {
            context.add_bool(IS_404, true);

            let domain = context.in_route.host.as_str();
            let not_found_path = format!("{}%2F", domain);

            // Check for custom 404 route: switch="404", link="{domain}%2F"
            if let std::result::Result::Ok(Some(route)) =
                flow_router.get_route_by_switch(NOT_FOUND_SWITCH, &not_found_path).await
            {
                if let Some(ref dest) = route.dest {
                    context.result = Some(FlowRouterResult::Redirect(
                        Uri::from_str(dest).unwrap(),
                        RedirectType::Temporary,
                    ));
                    context.current_step = FlowStep::End;
                    return Ok(FlowStepContinuation::Break);
                }
            }

            // Fall back to global not found URL
            let request_domain = context.request.uri().host().unwrap_or_default();
            let path = context.request.uri().path().trim_start_matches('/');

            // URL-encode the parameters for safe transmission
            let encoded_domain = urlencoding::encode(request_domain);
            let encoded_path = urlencoding::encode(path);

            // Build the not found URL with domain and path parameters
            let not_found_uri = format!(
                "{}&path={}",
                string_format!(self.redirect.not_found_url.clone(), encoded_domain.to_string()),
                encoded_path
            );

            context.result = Some(FlowRouterResult::Proxied(
                Uri::from_str(&not_found_uri).unwrap(),
                StatusCode::NOT_FOUND,
            ));

            // Set the step to End so the iterative flow will handle the end step
            context.current_step = FlowStep::End;

            return Ok(FlowStepContinuation::Break);
        }

        Ok(FlowStepContinuation::Continue)
    }

    async fn handle_end(
        &self,
        context: &mut FlowRouterContext,
        _flow_router: &FlowRouter,
    ) -> Result<FlowStepContinuation> {
        if context.is_data_true(IS_404) {
            return Ok(FlowStepContinuation::Break);
        } else {
            //println!("{}", "Is NOT 404");
        }

        Ok(FlowStepContinuation::Continue)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_create_not_found_module() {
        let redirect = Redirect {
            index_url: "https://example.com".to_string(),
            not_found_url: "https://example.com/404".to_string(),
            blocked_url: "https://example.com/blocked".to_string(),
        };
        let module = NotFoundModule::new(redirect.clone());

        assert_eq!(module.redirect.not_found_url, "https://example.com/404");
    }

    #[test]
    fn should_clone_not_found_module() {
        let redirect = Redirect {
            index_url: "https://example.com".to_string(),
            not_found_url: "https://example.com/404".to_string(),
            blocked_url: "https://example.com/blocked".to_string(),
        };
        let module = NotFoundModule::new(redirect);
        let cloned = module.clone();

        assert_eq!(cloned.redirect.not_found_url, "https://example.com/404");
    }
}
