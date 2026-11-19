use anyhow::Result;
use clap::Parser;
use std::sync::Arc;
use tokio::sync::mpsc;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

use route_icon_worker::adapters::{ImageStore, RouteEventConsumer};
use route_icon_worker::settings::Settings;
use route_icon_worker::worker::IconWorker;

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
                .label("service", "route-icon-worker")
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
            eprintln!("route-icon-worker: Failed to initialize Loki logging");
        }
    }

    tracing::info!("Starting route-icon-worker");

    let args = Args::parse();
    tracing::info!(
        "Run mode: {}, Config path: {}",
        args.run_mode,
        args.config_path
    );

    let settings = Settings::new(Some(args.run_mode.as_str()), Some(args.config_path.as_str()))
        .expect("Failed to load settings");

    // Create message channel
    let (message_tx, message_rx) = mpsc::channel(100);

    // Initialize S3 image store
    let image_store = Arc::new(ImageStore::new(&settings.s3).await?);

    // Start RabbitMQ consumer
    let consumer = Arc::new(RouteEventConsumer::new(settings.rabbitmq.clone(), message_tx));
    consumer.start();

    // Start icon worker
    let worker = IconWorker::new(message_rx, image_store, &settings.worker)?;
    worker.run().await;

    Ok(())
}
