use anyhow::Result;

use click_aggregator_api::{app_builder::AppBuilder, settings::Settings};

use clap::Parser;

#[derive(Parser, Debug)]
#[command(version)]
pub struct Args {
    #[arg(short, long, default_value_t = String::from("production"), env("APP_RUN_MODE"))]
    pub run_mode: String,
    #[arg(short, long, default_value_t = String::from("./config"), env("APP_CONFIG_PATH"))]
    pub config_path: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    dotenv::from_filename("./click-router-api/.env").ok();
    let args = Args::parse();

    let settings = Settings::new(
        Some(args.run_mode.as_str()),
        Some(args.config_path.as_str()),
    )
    .unwrap();

    let _app = AppBuilder::new(settings)
        .with_aws()
        .await
        .build()?
        .run()
        .await?;

    Ok(())
}
