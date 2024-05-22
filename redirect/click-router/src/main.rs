use click_router::{
    app_builder::AppBuilder,
    core::{base_flow_router::PerRequestData, BaseFlowRouter},
    settings::Settings,
};

use clap::Parser;
use http::Request;

#[derive(Parser, Debug)]
#[command(version)]
pub struct Args {
    #[arg(short, long, default_value_t = String::from("production"), env("APP_RUN_MODE"))]
    pub run_mode: String,
    #[arg(short, long, default_value_t = String::from("./config"), env("APP_CONFIG_PATH"))]
    pub config_path: String,
}

#[tokio::main]
async fn main() {

    dotenv::from_filename("./click-router/.env").ok();
    let args = Args::parse();

    let settings = Settings::new(Some(args.run_mode.as_str()), Some(args.config_path.as_str())).unwrap();

    let app = AppBuilder::new(settings)
        .with_aws()
        .await
        .with_moka()
        .with_defaults()
        .with_uaparser()
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
