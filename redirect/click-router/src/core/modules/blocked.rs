use anyhow::{Ok, Result};
use http::StatusCode;

use crate::{
    core::{
        flow_module::{FlowModule, FlowStepContinuation},
        flow_router::{FlowRouter, FlowRouterContext, FlowRouterResult, FlowStep},
    },
    model::route::{BlockedReason, RouteStatus},
};

const IS_BLOCKED: &str = "is_blocked";

#[derive(Debug, Clone, Default)]
pub struct BlockedModule;

impl BlockedModule {
    pub fn new() -> Self {
        Self
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

            let message = match reason {
                BlockedReason::Resoned(msg) => format!("This link has been blocked: {}", msg),
                BlockedReason::Unknown => "This link has been blocked.".to_string(),
            };

            context.result = Some(FlowRouterResult::PlainText(message, StatusCode::GONE));

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
        let module = BlockedModule::new();
        assert!(matches!(module, BlockedModule));
    }

    #[test]
    fn should_clone_blocked_module() {
        let module = BlockedModule::new();
        let cloned = module.clone();
        assert!(matches!(cloned, BlockedModule));
    }
}
