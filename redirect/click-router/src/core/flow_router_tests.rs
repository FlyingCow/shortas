#[cfg(test)]
mod flow_router_tests {
    use std::collections::HashMap;
    use chrono::Utc;
    use http::{Method, StatusCode, Uri, Version};
    use indexmap::IndexMap;
    use multimap::MultiMap;
    use cookie::CookieJar;

    use crate::{
        adapters::{RequestType, ResponseType, HitRegistrarType, LocationDetectorType, UserAgentDetectorType, RoutesCacheType, UserSettingsCacheType},
        core::{
            flow_router::{
                FlowRouter, FlowRouterContext, FlowRouterResult, FlowInRoute, FlowStep, RedirectType,
                RequestData, ResponseData, TlsInfo
            },
            modules::{FlowModules, RootModule, ConditionalModule, NotFoundModule, RedirectOnlyModule},
            expression::ExpressionEvaluator,
        },
        model::{Route, RouteProperties, RouteStatus, RoutingPolicy, RoutingTerminal, DestinationFormat, BlockedReason},
        settings::Redirect,
    };

    // Test helper functions
    fn create_test_request_data() -> RequestData {
        RequestData {
            uri: "https://short.ly/test".parse().unwrap(),
            headers: http::HeaderMap::new(),
            extensions: http::Extensions::new(),
            method: Method::GET,
            cookies: CookieJar::new(),
            params: IndexMap::new(),
            queries: MultiMap::new(),
            version: Version::HTTP_11,
            scheme: Some(http::uri::Scheme::HTTPS),
            local_addr: None,
            remote_addr: None,
            tls_info: None,
        }
    }

    fn create_test_response_data() -> ResponseData {
        ResponseData {
            status_code: None,
            headers: http::HeaderMap::new(),
            version: Version::HTTP_11,
            cookies: CookieJar::new(),
            extensions: http::Extensions::new(),
        }
    }

    fn create_test_route() -> Route {
        Route {
            switch: "main".to_string(),
            link: "test".to_string(),
            dest: Some("https://example.com".to_string()),
            dest_format: DestinationFormat::Http,
            code: Some(302),
            ttl: Some(3600),
            status: RouteStatus::Active,
            terminal: RoutingTerminal::External,
            policy: RoutingPolicy::Basic,
            properties: RouteProperties {
                route_id: Some("route_123".to_string()),
                domain_id: Some("domain_456".to_string()),
                owner_id: Some("user_789".to_string()),
                creator_id: Some("user_789".to_string()),
                workspace_id: Some("workspace_123".to_string()),
                scripts: None,
                tags: None,
                custom: None,
                native: None,
                bundling: None,
                opengraph: false,
                allow_debug: true,
            },
        }
    }

    fn create_test_flow_router() -> FlowRouter {
        FlowRouter::default(
            RoutesCacheType::Moka(crate::adapters::moka::routes_cache::MokaRoutesCache::new(
                crate::adapters::RoutesStoreType::Mongodb(crate::adapters::mongodb::routes_store::MongodbRoutesStore::new(&crate::adapters::mongodb::settings::Mongodb::default()).await),
                crate::adapters::moka::settings::MokaCacheSettings::default(),
            )),
            UserSettingsCacheType::Moka(crate::adapters::moka::user_settings_cache::MokaUserSettingsCache::new(
                crate::adapters::UserSettingsStoreType::Mongodb(crate::adapters::mongodb::user_settings_store::MongodbUserSettingsStore::new(&crate::adapters::mongodb::settings::Mongodb::default()).await),
                crate::adapters::moka::settings::MokaCacheSettings::default(),
            )),
            UserAgentDetectorType::None(),
            LocationDetectorType::None(),
            HitRegistrarType::None(),
            vec![],
        )
    }

    #[tokio::test]
    async fn test_flow_router_context_creation() {
        let request_data = create_test_request_data();
        let response_data = create_test_response_data();
        let request = RequestType::Test(request_data);
        let response = ResponseType::Test(response_data);

        let in_route = FlowInRoute {
            scheme: "https".to_string(),
            host: "short.ly".to_string(),
            port: 443,
            path: "test".to_string(),
            query: "".to_string(),
        };

        let context = FlowRouterContext::new(in_route.clone(), &request, &response);

        assert_eq!(context.in_route.scheme, "https");
        assert_eq!(context.in_route.host, "short.ly");
        assert_eq!(context.in_route.port, 443);
        assert_eq!(context.in_route.path, "test");
        assert_eq!(context.current_step, FlowStep::Initial);
        assert!(context.id.len() > 0);
        assert!(context.utc <= Utc::now());
    }

    #[tokio::test]
    async fn test_flow_router_context_data_operations() {
        let request_data = create_test_request_data();
        let response_data = create_test_response_data();
        let request = RequestType::Test(request_data);
        let response = ResponseType::Test(response_data);

        let in_route = FlowInRoute {
            scheme: "https".to_string(),
            host: "short.ly".to_string(),
            port: 443,
            path: "test".to_string(),
            query: "".to_string(),
        };

        let mut context = FlowRouterContext::new(in_route, &request, &response);

        // Test adding data
        context.add_bool("test_bool", true);
        context.add_string("test_string", "test_value");
        context.add_num("test_num", 42.0);

        // Test data retrieval
        assert!(context.is_data_true("test_bool"));
        assert!(!context.is_data_true("nonexistent"));

        // Test data values
        let bool_value = context.data.get("test_bool");
        assert!(bool_value.is_some());
        assert!(bool_value.unwrap().is_bool(true));

        let string_value = context.data.get("test_string");
        assert!(string_value.is_some());
        assert!(string_value.unwrap().is_string("test_value"));

        let num_value = context.data.get("test_num");
        assert!(num_value.is_some());
        assert!(num_value.unwrap().is_num(42.0));
    }

    #[tokio::test]
    async fn test_flow_router_result_types() {
        // Test Empty result
        let empty_result = FlowRouterResult::Empty(StatusCode::NOT_FOUND);
        assert!(matches!(empty_result, FlowRouterResult::Empty(_)));

        // Test Json result
        let json_result = FlowRouterResult::Json("{\"test\": true}".to_string(), StatusCode::OK);
        assert!(matches!(json_result, FlowRouterResult::Json(_, _)));

        // Test PlainText result
        let text_result = FlowRouterResult::PlainText("Hello World".to_string(), StatusCode::OK);
        assert!(matches!(text_result, FlowRouterResult::PlainText(_, _)));

        // Test Proxied result
        let proxied_result = FlowRouterResult::Proxied(
            "https://example.com".parse().unwrap(),
            StatusCode::OK,
        );
        assert!(matches!(proxied_result, FlowRouterResult::Proxied(_, _)));

        // Test Redirect result
        let redirect_result = FlowRouterResult::Redirect(
            "https://example.com".parse().unwrap(),
            RedirectType::Temporary,
        );
        assert!(matches!(redirect_result, FlowRouterResult::Redirect(_, _)));

        // Test Retargeting result
        let retargeting_result = FlowRouterResult::Retargeting(
            "https://example.com".parse().unwrap(),
            vec!["https://backup.com".parse().unwrap()],
        );
        assert!(matches!(retargeting_result, FlowRouterResult::Retargeting(_, _)));

        // Test Error result
        let error_result = FlowRouterResult::Error;
        assert!(matches!(error_result, FlowRouterResult::Error));
    }

    #[tokio::test]
    async fn test_flow_router_build_route_uri() {
        let router = create_test_flow_router().await;
        let request_data = create_test_request_data();
        let request = RequestType::Test(request_data);

        let in_route = router.build_route_uri(&request);

        assert_eq!(in_route.scheme, "https");
        assert_eq!(in_route.host, "short.ly");
        assert_eq!(in_route.port, 443);
        assert_eq!(in_route.path, "test");
        assert_eq!(in_route.query, "");
    }

    #[tokio::test]
    async fn test_flow_router_allow_debug() {
        let router = create_test_flow_router().await;
        let request_data = create_test_request_data();
        let response_data = create_test_response_data();
        let request = RequestType::Test(request_data);
        let response = ResponseType::Test(response_data);

        let in_route = FlowInRoute {
            scheme: "https".to_string(),
            host: "short.ly".to_string(),
            port: 443,
            path: "test".to_string(),
            query: "".to_string(),
        };

        let mut context = FlowRouterContext::new(in_route, &request, &response);

        // Test without main route
        assert!(!router.allow_debug(&context));

        // Test with main route that allows debug
        let route = create_test_route();
        context.main_route = Some(route);
        assert!(router.allow_debug(&context));

        // Test with main route that doesn't allow debug
        let mut route = create_test_route();
        route.properties.allow_debug = false;
        context.main_route = Some(route);
        assert!(!router.allow_debug(&context));
    }

    #[tokio::test]
    async fn test_flow_router_load_country() {
        let router = create_test_flow_router().await;
        let request_data = create_test_request_data();
        let response_data = create_test_response_data();
        let request = RequestType::Test(request_data);
        let response = ResponseType::Test(response_data);

        let in_route = FlowInRoute {
            scheme: "https".to_string(),
            host: "short.ly".to_string(),
            port: 443,
            path: "test".to_string(),
            query: "".to_string(),
        };

        let mut context = FlowRouterContext::new(in_route, &request, &response);

        // Test without client IP
        router.load_country(&mut context);
        assert!(!context.client_country.has_value());

        // Test with client IP
        context.client_ip = Some(crate::core::ip::IPInfo {
            address: "192.168.1.1".parse().unwrap(),
            is_private: true,
        });
        router.load_country(&mut context);
        // Should still be None since we're using LocationDetectorType::None()
        assert!(!context.client_country.has_value());
    }

    #[tokio::test]
    async fn test_flow_router_load_os() {
        let router = create_test_flow_router().await;
        let request_data = create_test_request_data();
        let response_data = create_test_response_data();
        let request = RequestType::Test(request_data);
        let response = ResponseType::Test(response_data);

        let in_route = FlowInRoute {
            scheme: "https".to_string(),
            host: "short.ly".to_string(),
            port: 443,
            path: "test".to_string(),
            query: "".to_string(),
        };

        let mut context = FlowRouterContext::new(in_route, &request, &response);

        // Test without user agent
        router.load_os(&mut context);
        assert!(!context.client_os.has_value());

        // Test with user agent
        context.user_agent = Some("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36".to_string());
        router.load_os(&mut context);
        // Should still be None since we're using UserAgentDetectorType::None()
        assert!(!context.client_os.has_value());
    }

    #[tokio::test]
    async fn test_flow_router_load_ua() {
        let router = create_test_flow_router().await;
        let request_data = create_test_request_data();
        let response_data = create_test_response_data();
        let request = RequestType::Test(request_data);
        let response = ResponseType::Test(response_data);

        let in_route = FlowInRoute {
            scheme: "https".to_string(),
            host: "short.ly".to_string(),
            port: 443,
            path: "test".to_string(),
            query: "".to_string(),
        };

        let mut context = FlowRouterContext::new(in_route, &request, &response);

        // Test without user agent
        router.load_ua(&mut context);
        assert!(!context.client_ua.has_value());

        // Test with user agent
        context.user_agent = Some("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36".to_string());
        router.load_ua(&mut context);
        // Should still be None since we're using UserAgentDetectorType::None()
        assert!(!context.client_ua.has_value());
    }

    #[tokio::test]
    async fn test_flow_router_load_device() {
        let router = create_test_flow_router().await;
        let request_data = create_test_request_data();
        let response_data = create_test_response_data();
        let request = RequestType::Test(request_data);
        let response = ResponseType::Test(response_data);

        let in_route = FlowInRoute {
            scheme: "https".to_string(),
            host: "short.ly".to_string(),
            port: 443,
            path: "test".to_string(),
            query: "".to_string(),
        };

        let mut context = FlowRouterContext::new(in_route, &request, &response);

        // Test without user agent
        router.load_device(&mut context);
        assert!(!context.client_device.has_value());

        // Test with user agent
        context.user_agent = Some("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36".to_string());
        router.load_device(&mut context);
        // Should still be None since we're using UserAgentDetectorType::None()
        assert!(!context.client_device.has_value());
    }

    #[tokio::test]
    async fn test_flow_router_replace_debug_data() {
        let router = create_test_flow_router().await;
        let request_data = create_test_request_data();
        let response_data = create_test_response_data();
        let request = RequestType::Test(request_data);
        let response = ResponseType::Test(response_data);

        let in_route = FlowInRoute {
            scheme: "https".to_string(),
            host: "short.ly".to_string(),
            port: 443,
            path: "test".to_string(),
            query: "".to_string(),
        };

        let mut context = FlowRouterContext::new(in_route, &request, &response);
        let route = create_test_route();
        context.main_route = Some(route);

        // Test debug data replacement
        router.replace_debug_data(&mut context);

        // Should have populated debug information
        assert!(context.host.is_some());
        assert!(context.protocol.is_some());
        assert!(context.client_ip.is_some());
        assert!(context.user_agent.is_some());
        assert!(context.client_langs.is_some());
    }

    #[tokio::test]
    async fn test_flow_router_build_context() {
        let router = create_test_flow_router().await;
        let request_data = create_test_request_data();
        let response_data = create_test_response_data();
        let request = RequestType::Test(request_data);
        let response = ResponseType::Test(response_data);

        let context = router.build_context(&request, &response);

        assert_eq!(context.current_step, FlowStep::Initial);
        assert_eq!(context.in_route.scheme, "https");
        assert_eq!(context.in_route.host, "short.ly");
        assert_eq!(context.in_route.path, "test");
        assert!(context.host.is_some());
        assert!(context.protocol.is_some());
        assert!(context.client_ip.is_some());
        assert!(context.user_agent.is_some());
        assert!(context.client_langs.is_some());
    }

    #[tokio::test]
    async fn test_flow_router_get_route() {
        let router = create_test_flow_router().await;
        let request_data = create_test_request_data();
        let response_data = create_test_response_data();
        let request = RequestType::Test(request_data);
        let response = ResponseType::Test(response_data);

        let in_route = FlowInRoute {
            scheme: "https".to_string(),
            host: "short.ly".to_string(),
            port: 443,
            path: "test".to_string(),
            query: "".to_string(),
        };

        let context = FlowRouterContext::new(in_route, &request, &response);

        // Test getting route
        let result = router.get_route("main", &context).await;
        // Should return None since we're using a mock cache
        assert!(result.is_ok());
        assert!(result.unwrap().is_none());
    }

    #[tokio::test]
    async fn test_flow_router_get_user_settings() {
        let router = create_test_flow_router().await;

        // Test getting user settings
        let result = router.get_user_settings("user_123").await;
        // Should return None since we're using a mock cache
        assert!(result.is_ok());
        assert!(result.unwrap().is_none());
    }

    #[tokio::test]
    async fn test_flow_router_handle() {
        let router = create_test_flow_router().await;
        let request_data = create_test_request_data();
        let response_data = create_test_response_data();
        let request = RequestType::Test(request_data);
        let response = ResponseType::Test(response_data);

        // Test handling request
        let result = router.handle(&request, &response).await;
        assert!(result.is_ok());
        
        let flow_result = result.unwrap();
        // Should return Empty with NOT_FOUND since no route is configured
        assert!(matches!(flow_result, FlowRouterResult::Empty(StatusCode::NOT_FOUND)));
    }

    #[tokio::test]
    async fn test_flow_router_router_to() {
        let router = create_test_flow_router().await;
        let request_data = create_test_request_data();
        let response_data = create_test_response_data();
        let request = RequestType::Test(request_data);
        let response = ResponseType::Test(response_data);

        let in_route = FlowInRoute {
            scheme: "https".to_string(),
            host: "short.ly".to_string(),
            port: 443,
            path: "test".to_string(),
            query: "".to_string(),
        };

        let mut context = FlowRouterContext::new(in_route, &request, &response);

        // Test routing to different steps
        let result = router.router_to(&mut context, FlowStep::Start).await;
        assert!(result.is_ok());

        let result = router.router_to(&mut context, FlowStep::UrlExtract).await;
        assert!(result.is_ok());

        let result = router.router_to(&mut context, FlowStep::Register).await;
        assert!(result.is_ok());

        let result = router.router_to(&mut context, FlowStep::BuildResult).await;
        assert!(result.is_ok());

        let result = router.router_to(&mut context, FlowStep::End).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_flow_router_handle_start() {
        let router = create_test_flow_router().await;
        let request_data = create_test_request_data();
        let response_data = create_test_response_data();
        let request = RequestType::Test(request_data);
        let response = ResponseType::Test(response_data);

        let in_route = FlowInRoute {
            scheme: "https".to_string(),
            host: "short.ly".to_string(),
            port: 443,
            path: "test".to_string(),
            query: "".to_string(),
        };

        let mut context = FlowRouterContext::new(in_route, &request, &response);

        // Test handle_start
        let result = router.handle_start(&mut context).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_flow_router_handle_url_extract() {
        let router = create_test_flow_router().await;
        let request_data = create_test_request_data();
        let response_data = create_test_response_data();
        let request = RequestType::Test(request_data);
        let response = ResponseType::Test(response_data);

        let in_route = FlowInRoute {
            scheme: "https".to_string(),
            host: "short.ly".to_string(),
            port: 443,
            path: "test".to_string(),
            query: "".to_string(),
        };

        let mut context = FlowRouterContext::new(in_route, &request, &response);

        // Test handle_url_extract
        let result = router.handle_url_extract(&mut context).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_flow_router_handle_register() {
        let router = create_test_flow_router().await;
        let request_data = create_test_request_data();
        let response_data = create_test_response_data();
        let request = RequestType::Test(request_data);
        let response = ResponseType::Test(response_data);

        let in_route = FlowInRoute {
            scheme: "https".to_string(),
            host: "short.ly".to_string(),
            port: 443,
            path: "test".to_string(),
            query: "".to_string(),
        };

        let mut context = FlowRouterContext::new(in_route, &request, &response);

        // Test handle_register
        let result = router.handle_register(&mut context).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_flow_router_handle_build_result() {
        let router = create_test_flow_router().await;
        let request_data = create_test_request_data();
        let response_data = create_test_response_data();
        let request = RequestType::Test(request_data);
        let response = ResponseType::Test(response_data);

        let in_route = FlowInRoute {
            scheme: "https".to_string(),
            host: "short.ly".to_string(),
            port: 443,
            path: "test".to_string(),
            query: "".to_string(),
        };

        let mut context = FlowRouterContext::new(in_route, &request, &response);

        // Test handle_build_result
        let result = router.handle_build_result(&mut context).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_flow_router_handle_end() {
        let router = create_test_flow_router().await;
        let request_data = create_test_request_data();
        let response_data = create_test_response_data();
        let request = RequestType::Test(request_data);
        let response = ResponseType::Test(response_data);

        let in_route = FlowInRoute {
            scheme: "https".to_string(),
            host: "short.ly".to_string(),
            port: 443,
            path: "test".to_string(),
            query: "".to_string(),
        };

        let mut context = FlowRouterContext::new(in_route, &request, &response);

        // Test handle_end
        let result = router.handle_end(&mut context).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_flow_router_handle_build_result_with_route() {
        let router = create_test_flow_router().await;
        let request_data = create_test_request_data();
        let response_data = create_test_response_data();
        let request = RequestType::Test(request_data);
        let response = ResponseType::Test(response_data);

        let in_route = FlowInRoute {
            scheme: "https".to_string(),
            host: "short.ly".to_string(),
            port: 443,
            path: "test".to_string(),
            query: "".to_string(),
        };

        let mut context = FlowRouterContext::new(in_route, &request, &response);

        // Set up a route
        let route = create_test_route();
        context.out_route = Some(route);

        // Test handle_build_result with route
        let result = router.handle_build_result(&mut context).await;
        assert!(result.is_ok());

        // Should have set a result
        assert!(context.result.is_some());
        let result = context.result.unwrap();
        assert!(matches!(result, FlowRouterResult::Redirect(_, RedirectType::Temporary)));
    }

    #[tokio::test]
    async fn test_flow_router_handle_build_result_without_route() {
        let router = create_test_flow_router().await;
        let request_data = create_test_request_data();
        let response_data = create_test_response_data();
        let request = RequestType::Test(request_data);
        let response = ResponseType::Test(response_data);

        let in_route = FlowInRoute {
            scheme: "https".to_string(),
            host: "short.ly".to_string(),
            port: 443,
            path: "test".to_string(),
            query: "".to_string(),
        };

        let mut context = FlowRouterContext::new(in_route, &request, &response);

        // Don't set a route
        context.out_route = None;

        // Test handle_build_result without route
        let result = router.handle_build_result(&mut context).await;
        assert!(result.is_ok());

        // Should have set a result
        assert!(context.result.is_some());
        let result = context.result.unwrap();
        assert!(matches!(result, FlowRouterResult::Empty(StatusCode::NOT_FOUND)));
    }
}

