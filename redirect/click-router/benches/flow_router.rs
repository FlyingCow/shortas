use std::sync::Arc;

use click_router::core::flow_router::PerRequestData;
use click_router::core::BaseFlowRouter;
use click_router::flow_router::default_flow_router::DefaultFlowRouter;
use click_router::{settings::Settings, AppBuilder};

use criterion::*;
use criterion::async_executor::FuturesExecutor;
use http::Request;

const APP_CONFIG_PATH: &'static str = "./config";
const APP_RUN_MODE: &'static str = "test";

async fn init_flow_router() -> DefaultFlowRouter {
    let settings = Settings::new(Some(APP_RUN_MODE), Some(APP_CONFIG_PATH)).unwrap();

    let app: DefaultFlowRouter = AppBuilder::new(settings)
        .with_aws()
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

fn fibonacci(n: u64) -> u64 {
    match n {
        0 => 1,
        1 => 1,
        n => fibonacci(n-1) + fibonacci(n-2),
    }
}

#[tokio::main]
async fn benchmark_flow_router(c: &mut Criterion) {

        dotenv::from_filename("./click-router/.env").ok();

        let app = Arc::new(init_flow_router().await);

        let request = Request::builder()
            .uri("/test")
            .header("Host", "localhost")
            .header("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:126.0) Gecko/20100101 Firefox/126.0");

            let request = Arc::new(request.body(()).unwrap());

        c.bench_function("iter", move |b| {

            b.to_async(FuturesExecutor).iter(|| async { 

                let app_binding = app.as_ref();


                    app_binding.handle(PerRequestData {
                                local_addr: "192.168.0.100:80".parse().unwrap(),
                                remote_addr: "188.138.135.18:80".parse().unwrap(),
                                request: request.as_ref().clone(),
                                tls_info: None,
                            })
                            .await
                            .unwrap();
            } )
        });

     c.bench_function("fib 20", |b| b.iter(|| fibonacci(black_box(20))));
}

criterion_group!(benches, benchmark_flow_router);
criterion_main!(benches);