use std::str::FromStr;

use anyhow::Result;
use http::Uri;

use crate::{
    core::base_flow_router::{FlowRouterContext, FlowRouterResult, RedirectType},
    flow_router::{
        base_flow_module::{BaseFlowModule, FlowStepContinuation},
        default_flow_router::DefaultFlowRouter,
    },
};

static IS_ROOT: &str = "is_root";

#[derive(Debug, Clone)]
pub struct RootModule {}

#[async_trait::async_trait()]
impl BaseFlowModule for RootModule {
    async fn handle_start(
        &self,
        context: &mut FlowRouterContext,
        _flow_router: &DefaultFlowRouter,
    ) -> Result<FlowStepContinuation> {
        
        if context.request.request.uri().path() == "/" {

            let root_uri = format!("https://{}", context.request.request.uri().host().unwrap());

            context.result = Some(FlowRouterResult::Redirect(
                Uri::from_str(&root_uri).unwrap(),
                RedirectType::Temporary,
            ));

            context.add_bool(IS_ROOT, true);

            //println!("IS_ROOT");
            return Ok(FlowStepContinuation::Break);
        }

        Ok(FlowStepContinuation::Continue)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        core::base_flow_router::{RequestData, ResponseData, FlowInRoute},
        flow_router::default_flow_router::DefaultFlowRouter,
    };
    use http::Method;

    fn create_test_context(path: &str, host: &str) -> FlowRouterContext {
        let uri_str = format!("https://{}{}", host, path);

        let request_data = RequestData {
            uri: uri_str.parse().unwrap(),
            method: Method::GET,
            ..Default::default()
        };

        let response_data = ResponseData::default();

        let in_route = FlowInRoute {
            scheme: "https".to_string(),
            host: host.to_string(),
            port: 443,
            path: path.trim_start_matches('/').to_string(),
            query: "".to_string(),
        };

        FlowRouterContext::new(in_route, request_data, response_data)
    }

    #[tokio::test]
    async fn should_handle_root_path_with_redirect() {
        let module = RootModule {};
        let flow_router = DefaultFlowRouter::default();

        let mut context = create_test_context("/", "example.com");

        let result = module.handle_start(&mut context, &flow_router).await;

        assert!(result.is_ok());
        assert!(matches!(result.unwrap(), FlowStepContinuation::Break));
        assert!(context.result.is_some());
        assert!(context.is_data_true(IS_ROOT));

        if let Some(FlowRouterResult::Redirect(uri, redirect_type)) = context.result {
            assert_eq!(uri.to_string(), "https://example.com");
            assert!(matches!(redirect_type, RedirectType::Temporary));
        } else {
            panic!("Expected Redirect result");
        }
    }

    #[tokio::test]
    async fn should_continue_for_non_root_path() {
        let module = RootModule {};
        let flow_router = DefaultFlowRouter::default();

        let mut context = create_test_context("/some-path", "example.com");

        let result = module.handle_start(&mut context, &flow_router).await;

        assert!(result.is_ok());
        assert!(matches!(result.unwrap(), FlowStepContinuation::Continue));
        assert!(context.result.is_none());
        assert!(!context.is_data_true(IS_ROOT));
    }

    #[tokio::test]
    async fn should_use_request_host_in_redirect() {
        let module = RootModule {};
        let flow_router = DefaultFlowRouter::default();

        let mut context = create_test_context("/", "custom-domain.org");

        let result = module.handle_start(&mut context, &flow_router).await;

        assert!(result.is_ok());

        if let Some(FlowRouterResult::Redirect(uri, _)) = context.result {
            assert!(uri.to_string().contains("custom-domain.org"));
        } else {
            panic!("Expected Redirect result");
        }
    }
}
