use criterion::*;
use std::sync::Arc;
use std::hint::black_box;
use click_router::model::route::{Route, RouteProperties, RouteStatus, RoutingPolicy, RoutingTerminal, DestinationFormat};
use click_router::core::expression::ExpressionEvaluator;
use click_router::core::flow_router::{FlowRouterContext, RequestData, FlowInRoute, ResponseData};
use click_router::model::expression::{Expression, Country as CountryExpr};
use click_router::core::location::Country;
use click_router::core::InitOnce;
use click_router::adapters::{RequestType, ResponseType};

/// Benchmark the optimized request ID generation
fn benchmark_request_id_generation(c: &mut Criterion) {
    let mut group = c.benchmark_group("request_id");

    group.bench_function("optimized_generation", |b| {
        b.iter(|| {
            // This is the optimized version using as_secs() and String::with_capacity
            let timestamp = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs();
            let random = rand::random::<u32>();

            let mut id = String::with_capacity(24);
            use std::fmt::Write;
            let _ = write!(id, "{}_{}", timestamp, random);

            black_box(id);
        })
    });

    group.bench_function("old_generation", |b| {
        b.iter(|| {
            // This is the old version using as_nanos() and format!
            let id = format!(
                "{}_{}",
                std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap_or_default()
                    .as_nanos(),
                rand::random::<u32>()
            );

            black_box(id);
        })
    });

    group.finish();
}

/// Benchmark Arc<Route> vs Route cloning
fn benchmark_route_handling(c: &mut Criterion) {
    let route = Route {
        switch: "main".to_string(),
        link: "test-link".to_string(),
        dest: Some("https://example.com/destination".to_string()),
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
            ..Default::default()
        },
    };

    let mut group = c.benchmark_group("route_handling");

    group.bench_function("arc_sharing", |b| {
        b.iter(|| {
            // Optimized: wrap in Arc and share
            let route_arc = Arc::new(route.clone());
            let main_route = route_arc.clone();
            let out_route = route_arc;

            black_box((main_route, out_route));
        })
    });

    group.bench_function("deep_cloning", |b| {
        b.iter(|| {
            // Old: deep clone the route
            let main_route = route.clone();
            let out_route = main_route.clone();

            black_box((main_route, out_route));
        })
    });

    group.finish();
}

/// Benchmark expression evaluation without cloning InitOnce
fn benchmark_expression_evaluation(c: &mut Criterion) {
    let evaluator = ExpressionEvaluator::new();

    let mut client_country = InitOnce::default(None);
    client_country.init_with(Some(Country {
        iso_code: "US".to_string(),
    }));

    let _expression = Expression {
        country: Some(CountryExpr::EQ("us".to_string())),
        ..Default::default()
    };

    // Create a minimal context for testing
    let request_data = RequestData::default();
    let request = RequestType::Test(request_data);
    let response = ResponseType::Test(ResponseData::default());

    let in_route = FlowInRoute {
        scheme: "https".to_string(),
        host: "example.com".to_string(),
        port: 443,
        path: "/test".to_string(),
        query: "".to_string(),
    };

    let mut context = FlowRouterContext::new(in_route, &request, &response);
    context.client_country = client_country;

    let conditions = vec![];

    let mut group = c.benchmark_group("expression_evaluation");

    group.bench_function("optimized_evaluation", |b| {
        b.iter(|| {
            // This uses the optimized as_ref() method
            let result = evaluator.find(&context, &conditions);
            black_box(result);
        })
    });

    group.finish();
}

criterion_group!(
    benches,
    benchmark_request_id_generation,
    benchmark_route_handling,
    benchmark_expression_evaluation
);

criterion_main!(benches);
