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
    use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

    dotenv::from_filename("./.env").ok();

    // Initialize tracing with Loki integration
    let loki_url = std::env::var("LOKI_URL").unwrap_or_else(|_| "http://loki:3100".to_string());

    let filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new("warn"));

    let loki_result = url::Url::parse(&loki_url)
        .ok()
        .and_then(|url| {
            tracing_loki::builder()
                .label("service", "click-aggregator-api")
                .ok()
                .and_then(|b| b.build_url(url).ok())
        });

    match loki_result {
        Some((loki_layer, loki_task)) => {
            tracing_subscriber::registry()
                .with(filter)
                .with(tracing_subscriber::fmt::layer())
                .with(loki_layer)
                .init();
            tokio::spawn(loki_task);
        }
        None => {
            tracing_subscriber::registry()
                .with(filter)
                .with(tracing_subscriber::fmt::layer())
                .init();
            eprintln!("click-aggregator-api: Failed to initialize Loki logging");
        }
    }
    let args = Args::parse();

    let settings = Settings::new(
        Some(args.run_mode.as_str()),
        Some(args.config_path.as_str()),
    )
    .unwrap();

    AppBuilder::new(settings).build()?.run().await?;

    Ok(())
}
