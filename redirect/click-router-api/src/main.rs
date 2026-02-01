use anyhow::Result;

use click_router_api::{app_builder::AppBuilder, settings::Settings};

use clap::Parser;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

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
    // Load .env file before initializing logging so RUST_LOG is available
    dotenv::from_filename("./.env").ok();

    // Initialize tracing with environment-based log level
    // Set RUST_LOG environment variable to control log level
    // Examples: RUST_LOG=debug, RUST_LOG=info, RUST_LOG=click_router_api=debug
    let env_filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new("info"));

    tracing_subscriber::registry()
        .with(env_filter)
        .with(tracing_subscriber::fmt::layer()
            .with_target(true)
            .with_thread_ids(true)
            .with_file(true)
            .with_line_number(true))
        .init();

    tracing::info!("Starting click-router-api");

    let args = Args::parse();
    tracing::info!("Run mode: {}, Config path: {}", args.run_mode, args.config_path);

    let settings = Settings::new(
        Some(args.run_mode.as_str()),
        Some(args.config_path.as_str()),
    )
    .unwrap();

    AppBuilder::new(settings)
        .with_mongodb()
        .await
        .with_rabbitmq()
        .await
        .build()?
        .run()
        .await?;

    Ok(())
}
