use anyhow::Result;

use clap::Parser;
use domain_verifier::{app_builder::AppBuilder, settings::Settings};
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

    // Initialize tracing with Loki integration
    let loki_url = std::env::var("LOKI_URL").unwrap_or_else(|_| "http://loki:3100".to_string());

    let env_filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("warn"));

    let fmt_layer = tracing_subscriber::fmt::layer()
        .with_target(true)
        .with_thread_ids(true)
        .with_file(true)
        .with_line_number(true);

    let loki_result = url::Url::parse(&loki_url)
        .ok()
        .and_then(|url| {
            tracing_loki::builder()
                .label("service", "domain-verifier")
                .ok()
                .and_then(|b| b.build_url(url).ok())
        });

    match loki_result {
        Some((loki_layer, loki_task)) => {
            tracing_subscriber::registry()
                .with(env_filter)
                .with(fmt_layer)
                .with(loki_layer)
                .init();
            tokio::spawn(loki_task);
        }
        None => {
            tracing_subscriber::registry()
                .with(env_filter)
                .with(fmt_layer)
                .init();
            eprintln!("domain-verifier: Failed to initialize Loki logging");
        }
    }

    tracing::info!("Starting domain-verifier");

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
        .with_dns_verifier()
        .build()?
        .run()
        .await?;

    Ok(())
}
