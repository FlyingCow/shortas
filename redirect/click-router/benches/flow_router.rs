use std::env;
use std::sync::Arc;

use click_router::adapters::{RequestType, ResponseType};
use click_router::core::flow_router::{FlowRouter, RequestData, ResponseData};
use click_router::{settings::Settings, AppBuilder};

use criterion::async_executor::FuturesExecutor;
use criterion::*;

const APP_CONFIG_PATH: &'static str = "./config";
const APP_RUN_MODE: &'static str = "test";

async fn init_flow_router() -> FlowRouter {
    let settings = Settings::new(Some(APP_RUN_MODE), Some(APP_CONFIG_PATH)).unwrap();
    let path = env::current_dir().ok().unwrap();
    println!("The current directory is {}", path.display());

    let flow_router = AppBuilder::new(settings)
        .with_default_modules()
        //.with_geo_ip()
        .with_none_location_detector()
        // .with_ua_parser()
        .with_none_user_agent_detector()
        .with_none_hit_registrar()
        // .with_fluvio()
        // .await
        .with_dynamo()
        .await
        .build();

    flow_router
}

#[tokio::main]
async fn benchmark_flow_router(c: &mut Criterion) {
    dotenv::from_filename("./click-router/.env").ok();

    let app = Arc::new(init_flow_router().await);

    c.bench_function("iter", move |b| {
        let mut request_data = RequestData {
            uri: "/test".parse().unwrap(),
            local_addr: Some("192.168.0.100:80".parse().unwrap()),
            remote_addr: Some("188.138.135.18:80".parse().unwrap()),
            tls_info: None,
            ..Default::default()
        };

        request_data
            .headers
            .append("Host", "localhost".parse().unwrap());
        request_data.headers.append(
            "User-Agent",
            "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:126.0) Gecko/20100101 Firefox/126.0"
                .parse()
                .unwrap(),
        );

        let request = RequestType::Test(request_data);

        let response_data = ResponseData {
            ..Default::default()
        };

        let response = ResponseType::Test(response_data);

        b.to_async(FuturesExecutor).iter(|| async {
            let app_binding = app.as_ref();

            app_binding.handle(&request, &response).await.unwrap();
        })
    });

    // c.bench_function("fib 20", |b| b.iter(|| fibonacci(black_box(20))));
}

criterion_group!(benches, benchmark_flow_router);
criterion_main!(benches);
