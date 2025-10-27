use std::env;
use std::sync::Arc;

use click_router::adapters::{RequestType, ResponseType};
use click_router::core::flow_router::{FlowRouter, RequestData, ResponseData};
use click_router::{settings::Settings, AppBuilder};

use criterion::*;

const APP_CONFIG_PATH: &'static str = "./config";
const APP_RUN_MODE: &'static str = "test";

async fn init_flow_router() -> FlowRouter {
    let settings = Settings::new(Some(APP_RUN_MODE), Some(APP_CONFIG_PATH)).unwrap();
    let path = env::current_dir().ok().unwrap();
    println!("The current directory is {}", path.display());

    let flow_router = AppBuilder::new(settings)
        .with_default_modules()
        .with_none_location_detector()  // Skip GeoIP for benchmark
        .with_none_user_agent_detector()  // Skip UA parser for benchmark
        .with_none_hit_registrar()  // Skip hit registration for benchmark
        .with_dynamo()  // Use DynamoDB (with localstack in test mode)
        .await
        .build();

    flow_router
}

fn benchmark_flow_router(c: &mut Criterion) {
    dotenv::from_filename("./click-router/.env").ok();

    // Create a multi-threaded Tokio runtime for benchmarking
    let rt = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap();

    // Initialize the flow router in the runtime
    let app = rt.block_on(async {
        Arc::new(init_flow_router().await)
    });

    let mut group = c.benchmark_group("flow_router");

    group.bench_function("handle_request", |b| {
        let request_data = RequestData {
            uri: "/test".parse().unwrap(),
            local_addr: Some("192.168.0.100:80".parse().unwrap()),
            remote_addr: Some("188.138.135.18:80".parse().unwrap()),
            tls_info: None,
            ..Default::default()
        };

        let mut headers = http::HeaderMap::new();
        headers.append("Host", "localhost".parse().unwrap());
        headers.append(
            "User-Agent",
            "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:126.0) Gecko/20100101 Firefox/126.0"
                .parse()
                .unwrap(),
        );

        let request_data = RequestData {
            headers,
            ..request_data
        };

        let request = RequestType::Test(request_data);
        let response = ResponseType::Test(ResponseData::default());

        // Clone Arc for the benchmark closure
        let app_clone = app.clone();

        b.iter(|| {
            rt.block_on(async {
                app_clone.handle(&request, &response).await.unwrap();
            })
        })
    });

    group.finish();
}

criterion_group!(benches, benchmark_flow_router);
