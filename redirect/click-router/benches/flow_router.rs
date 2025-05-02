use std::sync::Arc;

use click_router::core::flow_router::{RequestData, ResponseData};
use click_router::{settings::Settings, AppBuilder};

use criterion::async_executor::FuturesExecutor;
use criterion::*;

const APP_CONFIG_PATH: &'static str = "./config";
const APP_RUN_MODE: &'static str = "test";

async fn init_flow_router() -> DefaultFlowRouter {
    let settings = Settings::new(Some(APP_RUN_MODE), Some(APP_CONFIG_PATH)).unwrap();

    let app: DefaultFlowRouter = AppBuilder::new(settings)
        .with_dynamo_stores()
        .await
        .with_rdkafka()
        .await
        .with_moka()
        .with_defaults()
        .with_uaparser()
        .with_geo_ip()
        .with_flow_defaults()
        .with_default_modules()
        .build()
        .unwrap();

    app
}

#[tokio::main]
async fn benchmark_flow_router(c: &mut Criterion) {
    dotenv::from_filename("./click-router/.env").ok();

    let app = Arc::new(init_flow_router().await);

    let mut request = RequestData {
        uri: "/test".parse().unwrap(),
        local_addr: Some("192.168.0.100:80".parse().unwrap()),
        remote_addr: Some("188.138.135.18:80".parse().unwrap()),
        tls_info: None,
        ..Default::default()
    };

    request.headers.append("Host", "localhost".parse().unwrap());
    request.headers.append(
        "User-Agent",
        "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:126.0) Gecko/20100101 Firefox/126.0"
            .parse()
            .unwrap(),
    );

    let response = ResponseData {
        ..Default::default()
    };

    let request = Arc::new(request);
    let response = Arc::new(response);

    c.bench_function("iter", move |b| {
        b.to_async(FuturesExecutor).iter(|| async {
            let app_binding = app.as_ref();

            app_binding
                .handle(request.as_ref().clone(), response.as_ref().clone())
                .await
                .unwrap();
        })
    });

    // c.bench_function("fib 20", |b| b.iter(|| fibonacci(black_box(20))));
}

criterion_group!(benches, benchmark_flow_router);
criterion_main!(benches);
