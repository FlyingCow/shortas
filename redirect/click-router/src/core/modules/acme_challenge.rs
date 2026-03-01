use anyhow::Result;
use http::StatusCode;

use crate::{
    adapters::RoutesCacheType,
    core::{
        flow_module::{FlowModule, FlowStepContinuation},
        flow_router::{FlowRouter, FlowRouterContext, FlowRouterResult, Request},
        routes::RoutesCache,
    },
    model::route::RoutingPolicy,
};

const ACME_CHALLENGE_PREFIX: &str = "/.well-known/acme-challenge/";

/// ACME challenge module that serves HTTP-01 challenges from routes with ChallengeRouting policy.
/// This module is readonly - challenges are created via click-router-api.
#[derive(Clone)]
pub struct AcmeChallengeModule {
    routes_cache: RoutesCacheType,
}

impl AcmeChallengeModule {
    pub fn new(routes_cache: RoutesCacheType) -> Self {
        Self { routes_cache }
    }
}

#[async_trait::async_trait()]
impl FlowModule for AcmeChallengeModule {
    async fn handle_start(
        &self,
        context: &mut FlowRouterContext,
        _flow_router: &FlowRouter,
    ) -> Result<FlowStepContinuation> {
        let path = context.request.uri().path();

        // Check if this is an ACME challenge request
        if !path.starts_with(ACME_CHALLENGE_PREFIX) {
            return Ok(FlowStepContinuation::Continue);
        }

        // Get domain from host
        let domain = match context.request.uri().host() {
            Some(host) => host.to_string(),
            None => {
                // Try to get from Host header
                match context.request.headers().get("host") {
                    Some(host) => host.to_str().unwrap_or("").split(':').next().unwrap_or("").to_string(),
                    None => {
                        context.result = Some(FlowRouterResult::Empty(StatusCode::BAD_REQUEST));
                        return Ok(FlowStepContinuation::Break);
                    }
                }
            }
        };

        tracing::debug!("ACME challenge request for domain: {}, path: {}", domain, path);

        // Look up challenge route in cache/store
        // The route is stored with switch=domain and link=path (/.well-known/acme-challenge/{token})
        match self.routes_cache.get_route(&domain, path).await {
            Ok(Some(route)) => {
                // Check if it's a challenge route
                if let RoutingPolicy::Challenge(challenge_routing) = &route.policy {
                    tracing::info!("ACME challenge found for domain: {}", domain);
                    // Return the key authorization as plain text
                    context.result = Some(FlowRouterResult::PlainText(
                        challenge_routing.key.clone(),
                        StatusCode::OK,
                    ));
                    return Ok(FlowStepContinuation::Break);
                }
                // Route exists but is not a challenge - let other modules handle it
                tracing::debug!("Route exists but is not a challenge route for: {}", domain);
                Ok(FlowStepContinuation::Continue)
            }
            Ok(None) => {
                tracing::warn!("ACME challenge not found for domain: {}, path: {}", domain, path);
                context.result = Some(FlowRouterResult::Empty(StatusCode::NOT_FOUND));
                Ok(FlowStepContinuation::Break)
            }
            Err(e) => {
                tracing::error!("Failed to lookup ACME challenge: {}", e);
                context.result = Some(FlowRouterResult::Empty(StatusCode::INTERNAL_SERVER_ERROR));
                Ok(FlowStepContinuation::Break)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_acme_challenge_prefix() {
        assert_eq!(ACME_CHALLENGE_PREFIX, "/.well-known/acme-challenge/");
    }
}
