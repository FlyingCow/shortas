use anyhow::Result;

use clap::Parser;
use route_verifier::{app_builder::AppBuilder, settings::Settings};
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
    dotenv::from_filename("./.env").ok();

    let env_filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));

    tracing_subscriber::registry()
        .with(env_filter)
        .with(
            tracing_subscriber::fmt::layer()
                .with_target(true)
                .with_thread_ids(true)
                .with_file(true)
                .with_line_number(true),
        )
        .init();

    tracing::info!("Starting route-verifier");

    let args = Args::parse();
    tracing::info!(
        "Run mode: {}, Config path: {}",
        args.run_mode,
        args.config_path
    );

    let settings = Settings::new(Some(args.run_mode.as_str()), Some(args.config_path.as_str()))
        .expect("Failed to load settings");

    AppBuilder::new(settings)
        .with_mongodb()
        .await
        .with_rabbitmq()
        .await
        .with_safe_browsing_client()
        .with_click_router_api_client()
        .build()?
        .run()
        .await?;

    Ok(())
}
