#[cfg(test)]
mod modules_tests {
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
                RequestData, ResponseData
            },
            modules::{FlowModules, RootModule, ConditionalModule, NotFoundModule, RedirectOnlyModule},
            expression::ExpressionEvaluator,
        },
        model::{
            Route, RouteProperties, RouteStatus, RoutingPolicy, RoutingTerminal, DestinationFormat, BlockedReason,
            expression::{Expression, UA, OS, DayOfMonth, Device},
            route::ConditionalRouting,
        },
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

    fn create_test_context() -> FlowRouterContext {
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

        FlowRouterContext::new(in_route, &request, &response)
    }

    // Root Module Tests
    #[tokio::test]
    async fn test_root_module_creation() {
        let redirect = Redirect {
            not_found_url: "http://localhost:5801/404/{}".to_string(),
            index_url: "http://localhost:5801/index/{}".to_string(),
        };

        let module = RootModule::new(redirect);
        assert_eq!(module.redirect.not_found_url, "http://localhost:5801/404/{}");
        assert_eq!(module.redirect.index_url, "http://localhost:5801/index/{}");
    }

    #[tokio::test]
    async fn test_root_module_handle_start_root_path() {
        let redirect = Redirect {
            not_found_url: "http://localhost:5801/404/{}".to_string(),
            index_url: "http://localhost:5801/index/{}".to_string(),
        };

        let module = RootModule::new(redirect);
        let router = create_test_flow_router().await;

        let mut request_data = create_test_request_data();
        request_data.uri = "https://short.ly/".parse().unwrap();
        let response_data = create_test_response_data();
        let request = RequestType::Test(request_data);
        let response = ResponseType::Test(response_data);

        let in_route = FlowInRoute {
            scheme: "https".to_string(),
            host: "short.ly".to_string(),
            port: 443,
            path: "".to_string(),
            query: "".to_string(),
        };

        let mut context = FlowRouterContext::new(in_route, &request, &response);

        let result = module.handle_start(&mut context, &router).await;
        assert!(result.is_ok());
        assert!(matches!(result.unwrap(), crate::core::flow_module::FlowStepContinuation::Break));

        // Should have set a result
        assert!(context.result.is_some());
        let result = context.result.unwrap();
        assert!(matches!(result, FlowRouterResult::Proxied(_, StatusCode::OK)));

        // Should have set root flag
        assert!(context.is_data_true("is_root"));
    }

    #[tokio::test]
    async fn test_root_module_handle_start_non_root_path() {
        let redirect = Redirect {
            not_found_url: "http://localhost:5801/404/{}".to_string(),
            index_url: "http://localhost:5801/index/{}".to_string(),
        };

        let module = RootModule::new(redirect);
        let router = create_test_flow_router().await;

        let mut context = create_test_context();

        let result = module.handle_start(&mut context, &router).await;
        assert!(result.is_ok());
        assert!(matches!(result.unwrap(), crate::core::flow_module::FlowStepContinuation::Continue));

        // Should not have set a result
        assert!(context.result.is_none());

        // Should not have set root flag
        assert!(!context.is_data_true("is_root"));
    }

    // Conditional Module Tests
    #[tokio::test]
    async fn test_conditional_module_creation() {
        let module = ConditionalModule::new();
        assert!(module.evaluator.is_some());
    }

    #[tokio::test]
    async fn test_conditional_module_handle_start_no_main_route() {
        let module = ConditionalModule::new();
        let router = create_test_flow_router().await;
        let mut context = create_test_context();

        let result = module.handle_start(&mut context, &router).await;
        assert!(result.is_ok());
        assert!(matches!(result.unwrap(), crate::core::flow_module::FlowStepContinuation::Continue));
    }

    #[tokio::test]
    async fn test_conditional_module_handle_start_with_basic_route() {
        let module = ConditionalModule::new();
        let router = create_test_flow_router().await;
        let mut context = create_test_context();

        let route = Route {
            switch: "main".to_string(),
            link: "test".to_string(),
            dest: Some("https://example.com".to_string()),
            dest_format: DestinationFormat::Http,
            code: Some(302),
            ttl: Some(3600),
            status: RouteStatus::Active,
            terminal: RoutingTerminal::External,
            policy: RoutingPolicy::Basic,
            properties: RouteProperties::default(),
        };

        context.main_route = Some(route);

        let result = module.handle_start(&mut context, &router).await;
        assert!(result.is_ok());
        assert!(matches!(result.unwrap(), crate::core::flow_module::FlowStepContinuation::Continue));
    }

    #[tokio::test]
    async fn test_conditional_module_handle_start_with_conditional_route() {
        let module = ConditionalModule::new();
        let router = create_test_flow_router().await;
        let mut context = create_test_context();

        let route = Route {
            switch: "main".to_string(),
            link: "test".to_string(),
            dest: Some("https://example.com".to_string()),
            dest_format: DestinationFormat::Http,
            code: Some(302),
            ttl: Some(3600),
            status: RouteStatus::Active,
            terminal: RoutingTerminal::External,
            policy: RoutingPolicy::Conditional(vec![ConditionalRouting {
                key: "mobile".to_string(),
                condition: Expression {
                    device: Some(Device::Mobile),
                    ..Default::default()
                },
            }]),
            properties: RouteProperties::default(),
        };

        context.main_route = Some(route);

        let result = module.handle_start(&mut context, &router).await;
        assert!(result.is_ok());
        assert!(matches!(result.unwrap(), crate::core::flow_module::FlowStepContinuation::Continue));

        // Should have set conditional flag
        assert!(context.is_data_true("is_conditional"));
    }

    #[tokio::test]
    async fn test_conditional_module_handle_url_extract_no_conditional_route() {
        let module = ConditionalModule::new();
        let router = create_test_flow_router().await;
        let mut context = create_test_context();

        let route = Route {
            switch: "main".to_string(),
            link: "test".to_string(),
            dest: Some("https://example.com".to_string()),
            dest_format: DestinationFormat::Http,
            code: Some(302),
            ttl: Some(3600),
            status: RouteStatus::Active,
            terminal: RoutingTerminal::External,
            policy: RoutingPolicy::Basic,
            properties: RouteProperties::default(),
        };

        context.main_route = Some(route);

        let result = module.handle_url_extract(&mut context, &router).await;
        assert!(result.is_ok());
        assert!(matches!(result.unwrap(), crate::core::flow_module::FlowStepContinuation::Continue));
    }

    #[tokio::test]
    async fn test_conditional_module_handle_url_extract_with_conditional_route() {
        let module = ConditionalModule::new();
        let router = create_test_flow_router().await;
        let mut context = create_test_context();

        let route = Route {
            switch: "main".to_string(),
            link: "test".to_string(),
            dest: Some("https://example.com".to_string()),
            dest_format: DestinationFormat::Http,
            code: Some(302),
            ttl: Some(3600),
            status: RouteStatus::Active,
            terminal: RoutingTerminal::External,
            policy: RoutingPolicy::Conditional(vec![ConditionalRouting {
                key: "mobile".to_string(),
                condition: Expression {
                    device: Some(Device::Mobile),
                    ..Default::default()
                },
            }]),
            properties: RouteProperties::default(),
        };

        context.main_route = Some(route);

        let result = module.handle_url_extract(&mut context, &router).await;
        assert!(result.is_ok());
        assert!(matches!(result.unwrap(), crate::core::flow_module::FlowStepContinuation::Continue));
    }

    // NotFound Module Tests
    #[tokio::test]
    async fn test_not_found_module_creation() {
        let redirect = Redirect {
            not_found_url: "http://localhost:5801/404/{}".to_string(),
            index_url: "http://localhost:5801/index/{}".to_string(),
        };

        let module = NotFoundModule::new(redirect);
        assert_eq!(module.redirect.not_found_url, "http://localhost:5801/404/{}");
        assert_eq!(module.redirect.index_url, "http://localhost:5801/index/{}");
    }

    #[tokio::test]
    async fn test_not_found_module_handle_start() {
        let redirect = Redirect {
            not_found_url: "http://localhost:5801/404/{}".to_string(),
            index_url: "http://localhost:5801/index/{}".to_string(),
        };

        let module = NotFoundModule::new(redirect);
        let router = create_test_flow_router().await;
        let mut context = create_test_context();

        let result = module.handle_start(&mut context, &router).await;
        assert!(result.is_ok());
        assert!(matches!(result.unwrap(), crate::core::flow_module::FlowStepContinuation::Continue));
    }

    // RedirectOnly Module Tests
    #[tokio::test]
    async fn test_redirect_only_module_creation() {
        let module = RedirectOnlyModule::new();
        // Module should be created successfully
        assert!(true);
    }

    #[tokio::test]
    async fn test_redirect_only_module_handle_start() {
        let module = RedirectOnlyModule::new();
        let router = create_test_flow_router().await;
        let mut context = create_test_context();

        let result = module.handle_start(&mut context, &router).await;
        assert!(result.is_ok());
        assert!(matches!(result.unwrap(), crate::core::flow_module::FlowStepContinuation::Continue));
    }

    // FlowModules Tests
    #[tokio::test]
    async fn test_flow_modules_root() {
        let redirect = Redirect {
            not_found_url: "http://localhost:5801/404/{}".to_string(),
            index_url: "http://localhost:5801/index/{}".to_string(),
        };

        let module = FlowModules::Root(RootModule::new(redirect));
        let router = create_test_flow_router().await;
        let mut context = create_test_context();

        let result = module.handle_start(&mut context, &router).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_flow_modules_conditional() {
        let module = FlowModules::Conditional(ConditionalModule::new());
        let router = create_test_flow_router().await;
        let mut context = create_test_context();

        let result = module.handle_start(&mut context, &router).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_flow_modules_not_found() {
        let redirect = Redirect {
            not_found_url: "http://localhost:5801/404/{}".to_string(),
            index_url: "http://localhost:5801/index/{}".to_string(),
        };

        let module = FlowModules::NotFound(NotFoundModule::new(redirect));
        let router = create_test_flow_router().await;
        let mut context = create_test_context();

        let result = module.handle_start(&mut context, &router).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_flow_modules_redirect_only() {
        let module = FlowModules::RedirectOnly(RedirectOnlyModule::new());
        let router = create_test_flow_router().await;
        let mut context = create_test_context();

        let result = module.handle_start(&mut context, &router).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_flow_modules_init() {
        let redirect = Redirect {
            not_found_url: "http://localhost:5801/404/{}".to_string(),
            index_url: "http://localhost:5801/index/{}".to_string(),
        };

        let module = FlowModules::Root(RootModule::new(redirect));
        let router = create_test_flow_router().await;
        let mut context = create_test_context();

        let result = module.init(&mut context, &router).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_flow_modules_handle_url_extract() {
        let module = FlowModules::Conditional(ConditionalModule::new());
        let router = create_test_flow_router().await;
        let mut context = create_test_context();

        let result = module.handle_url_extract(&mut context, &router).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_flow_modules_handle_register() {
        let module = FlowModules::RedirectOnly(RedirectOnlyModule::new());
        let router = create_test_flow_router().await;
        let mut context = create_test_context();

        let result = module.handle_register(&mut context, &router).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_flow_modules_handle_build_result() {
        let module = FlowModules::NotFound(NotFoundModule::new(Redirect::default()));
        let router = create_test_flow_router().await;
        let mut context = create_test_context();

        let result = module.handle_build_result(&mut context, &router).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_flow_modules_handle_end() {
        let module = FlowModules::RedirectOnly(RedirectOnlyModule::new());
        let router = create_test_flow_router().await;
        let mut context = create_test_context();

        let result = module.handle_end(&mut context, &router).await;
        assert!(result.is_ok());
    }

    // Integration Tests
    #[tokio::test]
    async fn test_root_module_integration() {
        let redirect = Redirect {
            not_found_url: "http://localhost:5801/404/{}".to_string(),
            index_url: "http://localhost:5801/index/{}".to_string(),
        };

        let module = RootModule::new(redirect);
        let router = create_test_flow_router().await;

        // Test root path
        let mut request_data = create_test_request_data();
        request_data.uri = "https://short.ly/".parse().unwrap();
        let response_data = create_test_response_data();
        let request = RequestType::Test(request_data);
        let response = ResponseType::Test(response_data);

        let in_route = FlowInRoute {
            scheme: "https".to_string(),
            host: "short.ly".to_string(),
            port: 443,
            path: "".to_string(),
            query: "".to_string(),
        };

        let mut context = FlowRouterContext::new(in_route, &request, &response);

        let result = module.handle_start(&mut context, &router).await;
        assert!(result.is_ok());
        assert!(matches!(result.unwrap(), crate::core::flow_module::FlowStepContinuation::Break));

        // Should have set a result
        assert!(context.result.is_some());
        let result = context.result.unwrap();
        assert!(matches!(result, FlowRouterResult::Proxied(_, StatusCode::OK)));

        // Should have set root flag
        assert!(context.is_data_true("is_root"));
    }

    #[tokio::test]
    async fn test_conditional_module_integration() {
        let module = ConditionalModule::new();
        let router = create_test_flow_router().await;
        let mut context = create_test_context();

        let route = Route {
            switch: "main".to_string(),
            link: "test".to_string(),
            dest: Some("https://example.com".to_string()),
            dest_format: DestinationFormat::Http,
            code: Some(302),
            ttl: Some(3600),
            status: RouteStatus::Active,
            terminal: RoutingTerminal::External,
            policy: RoutingPolicy::Conditional(vec![ConditionalRouting {
                key: "mobile".to_string(),
                condition: Expression {
                    device: Some(Device::Mobile),
                    ..Default::default()
                },
            }]),
            properties: RouteProperties::default(),
        };

        context.main_route = Some(route);

        let result = module.handle_start(&mut context, &router).await;
        assert!(result.is_ok());
        assert!(matches!(result.unwrap(), crate::core::flow_module::FlowStepContinuation::Continue));

        // Should have set conditional flag
        assert!(context.is_data_true("is_conditional"));
    }

    #[tokio::test]
    async fn test_not_found_module_integration() {
        let redirect = Redirect {
            not_found_url: "http://localhost:5801/404/{}".to_string(),
            index_url: "http://localhost:5801/index/{}".to_string(),
        };

        let module = NotFoundModule::new(redirect);
        let router = create_test_flow_router().await;
        let mut context = create_test_context();

        let result = module.handle_start(&mut context, &router).await;
        assert!(result.is_ok());
        assert!(matches!(result.unwrap(), crate::core::flow_module::FlowStepContinuation::Continue));
    }

    #[tokio::test]
    async fn test_redirect_only_module_integration() {
        let module = RedirectOnlyModule::new();
        let router = create_test_flow_router().await;
        let mut context = create_test_context();

        let result = module.handle_start(&mut context, &router).await;
        assert!(result.is_ok());
        assert!(matches!(result.unwrap(), crate::core::flow_module::FlowStepContinuation::Continue));
    }
}

