use click_router::{
    app_builder::AppBuilder,
    core::{base_flow_router::PerRequestData, BaseFlowRouter},
    settings::Settings,
};
use http::Request;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    let settings = Settings::new().unwrap();

    let app = AppBuilder::new(settings)
        .with_aws()
        .await
        .with_moka()
        .with_defaults()
        .with_flow_defaults()
        .build()
        .unwrap();

    let request = Request::builder()
        .uri("/attr")
        .header("Host", "localhost")
        .header("User-Agent", "my-awesome-agent/1.0");

    let result = app
        .handle(PerRequestData {
            local_addr: "192.168.0.100:80".parse().unwrap(),
            remote_addr: "188.138.135.18:80".parse().unwrap(),
            request: request.body(()).unwrap(),
            tls_info: None,
        })
        .await
        .unwrap();

    println!("{}", result)
}
