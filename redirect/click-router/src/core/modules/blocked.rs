use std::str::FromStr;

use anyhow::{Ok, Result};
use http::{StatusCode, Uri};
use string_format::*;

use crate::{
    core::{
        flow_module::{FlowModule, FlowStepContinuation},
        flow_router::{FlowRouter, FlowRouterContext, FlowRouterResult, FlowStep},
    },
    model::route::{BlockedReason, RouteStatus},
    settings::Redirect,
};

const IS_BLOCKED: &str = "is_blocked";

#[derive(Debug, Clone)]
pub struct BlockedModule {
    redirect: Redirect,
}

impl BlockedModule {
    pub fn new(redirect: Redirect) -> Self {
        Self { redirect }
    }
}

#[async_trait::async_trait()]
impl FlowModule for BlockedModule {
    async fn handle_start(
        &self,
        context: &mut FlowRouterContext,
        _flow_router: &FlowRouter,
    ) -> Result<FlowStepContinuation> {
        // Check if the route exists and is blocked
        // Clone the status to avoid borrow checker issues
        let blocked_reason = context.main_route.as_ref().and_then(|route| {
            if let RouteStatus::Blocked(reason) = &route.status {
                Some(reason.clone())
            } else {
                None
            }
        });

        if let Some(reason) = blocked_reason {
            context.add_bool(IS_BLOCKED, true);

            // URL-encode the reason for safe transmission
            let reason_text = match &reason {
                BlockedReason::Resoned(msg) => msg.clone(),
                BlockedReason::Unknown => String::new(),
            };

            let encoded_reason = urlencoding::encode(&reason_text);

            let blocked_uri = string_format!(
                self.redirect.blocked_url.clone(),
                encoded_reason.to_string()
            );

            context.result = Some(FlowRouterResult::Proxied(
                Uri::from_str(&blocked_uri).unwrap(),
                StatusCode::GONE,
            ));

            // Set the step to End so the flow will terminate
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
        if context.is_data_true(IS_BLOCKED) {
            return Ok(FlowStepContinuation::Break);
        }

        Ok(FlowStepContinuation::Continue)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_create_blocked_module() {
        let redirect = Redirect {
            index_url: "https://example.com".to_string(),
            not_found_url: "https://example.com/404".to_string(),
            blocked_url: "https://example.com/blocked".to_string(),
        };
        let module = BlockedModule::new(redirect.clone());

        assert_eq!(module.redirect.blocked_url, "https://example.com/blocked");
    }

    #[test]
    fn should_clone_blocked_module() {
        let redirect = Redirect {
            index_url: "https://example.com".to_string(),
            not_found_url: "https://example.com/404".to_string(),
            blocked_url: "https://example.com/blocked".to_string(),
        };
        let module = BlockedModule::new(redirect);
        let cloned = module.clone();

        assert_eq!(cloned.redirect.blocked_url, "https://example.com/blocked");
    }
}
