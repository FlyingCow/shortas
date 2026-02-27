use anyhow::Result;
use http::StatusCode;

use crate::{
    adapters::ChallengeCacheType,
    core::{
        challenge::ChallengeCache,
        flow_module::{FlowModule, FlowStepContinuation},
        flow_router::{FlowRouter, FlowRouterContext, FlowRouterResult, Request},
    },
};

const ACME_CHALLENGE_PREFIX: &str = "/.well-known/acme-challenge/";

#[derive(Clone)]
pub struct AcmeChallengeModule {
    challenge_cache: ChallengeCacheType,
}

impl AcmeChallengeModule {
    pub fn new(challenge_cache: ChallengeCacheType) -> Self {
        Self { challenge_cache }
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

        // Extract token from path
        let token = &path[ACME_CHALLENGE_PREFIX.len()..];
        if token.is_empty() {
            context.result = Some(FlowRouterResult::Empty(StatusCode::NOT_FOUND));
            return Ok(FlowStepContinuation::Break);
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

        tracing::debug!("ACME challenge request for domain: {}, token: {}", domain, token);

        // Look up challenge in cache/store
        match self.challenge_cache.get_challenge(&domain, token).await {
            Ok(Some(challenge)) => {
                tracing::info!("ACME challenge found for domain: {}", domain);
                // Return the key authorization as plain text
                context.result = Some(FlowRouterResult::PlainText(
                    challenge.key_authorization,
                    StatusCode::OK,
                ));
                Ok(FlowStepContinuation::Break)
            }
            Ok(None) => {
                tracing::warn!("ACME challenge not found for domain: {}, token: {}", domain, token);
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
