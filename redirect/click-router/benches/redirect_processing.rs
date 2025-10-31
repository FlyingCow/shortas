use criterion::*;
use std::sync::Arc;

use click_router::{
    adapters::{
        RequestType, ResponseType, HitRegistrarType, LocationDetectorType,
        UserAgentDetectorType, RoutesCacheType, UserSettingsCacheType,
        RoutesStoreType, UserSettingsStoreType,
        memory::{
            routes_store::InMemoryRoutesStore,
            user_settings_store::InMemoryUserSettingsStore,
        },
        moka::{
            routes_cache::MokaRoutesCache,
            user_settings_cache::MokaUserSettingsCache,
            settings::{RoutesCacheSettings, UserSettingsCacheSettings},
        },
    },
    core::{
        flow_router::{FlowRouter, RequestData, ResponseData},
    },
    model::{
        Route,
        route::{RouteProperties, RouteStatus, RoutingPolicy, RoutingTerminal, DestinationFormat},
    },
};

/// Create a test route for benchmarking
fn create_test_route(link: &str, dest: &str) -> Route {
    Route {
        switch: "main".to_string(),
        link: link.to_string(),
        dest: Some(dest.to_string()),
        dest_format: DestinationFormat::Http,
        code: Some(302),
        ttl: Some(3600),
        status: RouteStatus::Active,
        terminal: RoutingTerminal::External,
        policy: RoutingPolicy::Basic,
        properties: RouteProperties {
            route_id: Some("route-123".to_string()),
            owner_id: Some("owner-456".to_string()),
            creator_id: Some("creator-789".to_string()),
            workspace_id: Some("workspace-abc".to_string()),
            allow_debug: false,
            ..Default::default()
        },
    }
}

/// Initialize a FlowRouter with in-memory stores and no external dependencies
async fn init_in_memory_flow_router() -> FlowRouter {
    // Create in-memory stores
    let routes_store = InMemoryRoutesStore::new();
    let user_settings_store = InMemoryUserSettingsStore::new();

    // Pre-populate with test routes
    // Note: Routes are stored with keys in the format "domain%2Fpath" (URL-encoded /)
    routes_store.insert(
        "main",
        "localhost%2Ftest",  // URL-encoded format for "localhost/test"
        create_test_route("localhost%2Ftest", "https://example.com/destination"),
    ).await;

    routes_store.insert(
        "main",
        "localhost%2Fbenchmark",  // URL-encoded format for "localhost/benchmark"
        create_test_route("localhost%2Fbenchmark", "https://example.com/benchmark-dest"),
    ).await;

    routes_store.insert(
        "main",
        "localhost%2Fproduct/123",  // URL-encoded format for "localhost/product/123"
        create_test_route("localhost%2Fproduct/123", "https://shop.example.com/products/123"),
    ).await;

    // Wrap stores in enum variants
    let routes_store_type = RoutesStoreType::InMemory(routes_store);
    let user_settings_store_type = UserSettingsStoreType::InMemory(user_settings_store);

    // Create Moka caches with optimized settings for benchmarking
    let routes_cache_settings = RoutesCacheSettings {
        max_capacity: 10000,
        time_to_live_minutes: 60,
        time_to_idle_minutes: 30,
    };

    let user_settings_cache_settings = UserSettingsCacheSettings {
        max_capacity: 1000,
        time_to_live_minutes: 60,
        time_to_idle_minutes: 30,
    };

    let routes_cache = MokaRoutesCache::new(routes_store_type, routes_cache_settings);
    let user_settings_cache = MokaUserSettingsCache::new(user_settings_store_type, user_settings_cache_settings);

    // Create the router without any complex modules
    // No modules means the default flow will be used which fetches routes and handles redirects
    let modules = vec![];

    FlowRouter::default(
        RoutesCacheType::Moka(routes_cache),
        UserSettingsCacheType::Moka(user_settings_cache),
        UserAgentDetectorType::None(),
        LocationDetectorType::None(),
        HitRegistrarType::None(),
        modules,
    )
}

/// Comprehensive benchmark suite for redirect processing
fn benchmark_redirect_processing(c: &mut Criterion) {
    let rt = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap();

    // Initialize router once to avoid metrics re-registration issues
    let app = rt.block_on(async { Arc::new(init_in_memory_flow_router().await) });

    // Pre-warm the cache with all test routes
    rt.block_on(async {
        for uri in &["/test", "/benchmark", "/product/123"] {
            let request_data = RequestData {
                uri: uri.parse().unwrap(),
                headers: {
                    let mut headers = http::HeaderMap::new();
                    headers.append("Host", "localhost".parse().unwrap());
                    headers
                },
                ..Default::default()
            };
            let request = RequestType::Test(request_data);
            let response = ResponseType::Test(ResponseData::default());
            let _ = app.handle(&request, &response).await;
        }
    });

    let mut group = c.benchmark_group("redirect_processing");

    // Benchmark 1: Simple redirect with basic request
    group.bench_function("simple_redirect", |b| {
        let request_data = RequestData {
            uri: "/test".parse().unwrap(),
            local_addr: Some("192.168.0.100:80".parse().unwrap()),
            remote_addr: Some("188.138.135.18:80".parse().unwrap()),
            headers: {
                let mut headers = http::HeaderMap::new();
                headers.append("Host", "localhost".parse().unwrap());
                headers.append(
                    "User-Agent",
                    "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:126.0) Gecko/20100101 Firefox/126.0"
                        .parse()
                        .unwrap(),
                );
                headers
            },
            ..Default::default()
        };

        let app_clone = app.clone();

        b.iter(|| {
            rt.block_on(async {
                let request = RequestType::Test(request_data.clone());
                let response = ResponseType::Test(ResponseData::default());
                app_clone.handle(&request, &response).await.unwrap()
            })
        })
    });

    // Benchmark 2: Desktop Chrome user agent
    group.bench_function("chrome_desktop", |b| {
        let request_data = RequestData {
            uri: "/benchmark".parse().unwrap(),
            headers: {
                let mut headers = http::HeaderMap::new();
                headers.append("Host", "localhost".parse().unwrap());
                headers.append(
                    "User-Agent",
                    "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36"
                        .parse()
                        .unwrap(),
                );
                headers
            },
            remote_addr: Some("8.8.8.8:443".parse().unwrap()),
            ..Default::default()
        };

        let app_clone = app.clone();

        b.iter(|| {
            rt.block_on(async {
                let request = RequestType::Test(request_data.clone());
                let response = ResponseType::Test(ResponseData::default());
                app_clone.handle(&request, &response).await.unwrap()
            })
        })
    });

    // Benchmark 3: Mobile Safari user agent
    group.bench_function("mobile_safari", |b| {
        let request_data = RequestData {
            uri: "/benchmark".parse().unwrap(),
            headers: {
                let mut headers = http::HeaderMap::new();
                headers.append("Host", "localhost".parse().unwrap());
                headers.append(
                    "User-Agent",
                    "Mozilla/5.0 (iPhone; CPU iPhone OS 17_0 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/17.0 Mobile/15E148 Safari/604.1"
                        .parse()
                        .unwrap(),
                );
                headers
            },
            remote_addr: Some("1.1.1.1:443".parse().unwrap()),
            ..Default::default()
        };

        let app_clone = app.clone();

        b.iter(|| {
            rt.block_on(async {
                let request = RequestType::Test(request_data.clone());
                let response = ResponseType::Test(ResponseData::default());
                app_clone.handle(&request, &response).await.unwrap()
            })
        })
    });

    // Benchmark 4: Request with query parameters
    group.bench_function("with_query_params", |b| {
        let request_data = RequestData {
            uri: "/benchmark?utm_source=test&utm_campaign=bench&ref=homepage".parse().unwrap(),
            headers: {
                let mut headers = http::HeaderMap::new();
                headers.append("Host", "localhost".parse().unwrap());
                headers.append(
                    "User-Agent",
                    "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:126.0) Gecko/20100101 Firefox/126.0"
                        .parse()
                        .unwrap(),
                );
                headers
            },
            remote_addr: Some("192.168.1.1:443".parse().unwrap()),
            ..Default::default()
        };

        let app_clone = app.clone();

        b.iter(|| {
            rt.block_on(async {
                let request = RequestType::Test(request_data.clone());
                let response = ResponseType::Test(ResponseData::default());
                app_clone.handle(&request, &response).await.unwrap()
            })
        })
    });

    // Benchmark 5: Longer path
    group.bench_function("long_path", |b| {
        let request_data = RequestData {
            uri: "/product/123".parse().unwrap(),
            headers: {
                let mut headers = http::HeaderMap::new();
                headers.append("Host", "localhost".parse().unwrap());
                headers.append(
                    "User-Agent",
                    "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:126.0) Gecko/20100101 Firefox/126.0"
                        .parse()
                        .unwrap(),
                );
                headers
            },
            remote_addr: Some("203.0.113.42:443".parse().unwrap()),
            ..Default::default()
        };

        let app_clone = app.clone();

        b.iter(|| {
            rt.block_on(async {
                let request = RequestType::Test(request_data.clone());
                let response = ResponseType::Test(ResponseData::default());
                app_clone.handle(&request, &response).await.unwrap()
            })
        })
    });

    // Benchmark 6: Concurrent requests
    group.bench_function("parallel_10_requests", |b| {
        let app_clone = app.clone();

        b.iter(|| {
            rt.block_on(async {
                let handles: Vec<_> = (0..10)
                    .map(|_| {
                        let app_ref = app_clone.clone();
                        tokio::spawn(async move {
                            let request_data = RequestData {
                                uri: "/test".parse().unwrap(),
                                headers: {
                                    let mut headers = http::HeaderMap::new();
                                    headers.append("Host", "localhost".parse().unwrap());
                                    headers
                                },
                                ..Default::default()
                            };
                            let request = RequestType::Test(request_data);
                            let response = ResponseType::Test(ResponseData::default());
                            app_ref.handle(&request, &response).await.unwrap()
                        })
                    })
                    .collect();

                for handle in handles {
                    handle.await.unwrap();
                }
            })
        })
    });

    group.finish();
}

criterion_group!(benches, benchmark_redirect_processing);
criterion_main!(benches);
