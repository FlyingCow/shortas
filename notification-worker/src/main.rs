//! Notification worker: consumes `NotificationEvent`s published by
//! management-api's OutboxProcessor, renders them from their S3 template
//! set (Handlebars), and delivers to the declared channels (in-app and/or
//! email). Reuses management-api's domain entities, repositories, email
//! sender and template store as a library crate instead of duplicating them.

use std::sync::Arc;

use anyhow::Result;
use clap::Parser;
use sqlx::postgres::PgPoolOptions;
use tokio::sync::broadcast;
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;

use management_api::infrastructure::email::build_email_sender;
use management_api::infrastructure::repositories::{PgNotificationRepository, PgUserRepository};
use management_api::infrastructure::storage::{MinioService, TemplateStore};
use management_api::settings::Settings;

mod worker;

/// Command-line arguments for the notification worker.
#[derive(Parser, Debug)]
#[command(version)]
pub struct Args {
    #[arg(short, long, default_value_t = String::from("development"), env("APP_RUN_MODE"))]
    pub run_mode: String,
    #[arg(short, long, default_value_t = String::from("./config"), env("APP_CONFIG_PATH"))]
    pub config_path: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .with_target(true)
        .with_thread_ids(true)
        .finish();
    tracing::subscriber::set_global_default(subscriber)?;

    info!(
        "Starting notification-worker v{}",
        env!("CARGO_PKG_VERSION")
    );

    dotenv::from_filename("./.env").ok();

    let args = Args::parse();
    let settings = Settings::new(Some(&args.run_mode), Some(&args.config_path))?;
    info!("Loaded configuration");

    let pool = PgPoolOptions::new()
        .max_connections(settings.database.max_connections)
        .connect(&settings.database.connection_string())
        .await?;
    info!("Connected to database");

    let notification_repo = Arc::new(PgNotificationRepository::new(pool.clone()));
    let user_repo = Arc::new(PgUserRepository::new(pool.clone()));
    let email_sender = build_email_sender(&settings.email).await?;
    let storage = Arc::new(MinioService::new(&settings.minio).await?);
    let template_store = Arc::new(TemplateStore::new(storage));

    let (shutdown_tx, shutdown_rx) = broadcast::channel::<()>(1);

    let worker = Arc::new(worker::NotificationWorker::new(
        settings.rabbitmq.clone(),
        notification_repo,
        user_repo,
        email_sender,
        template_store,
    ));

    let worker_handle = {
        let worker = worker.clone();
        tokio::spawn(async move {
            if let Err(e) = worker.run(shutdown_rx).await {
                tracing::error!("Notification worker stopped with error: {}", e);
            }
        })
    };

    tokio::select! {
        _ = worker_handle => {}
        _ = tokio::signal::ctrl_c() => {
            info!("Received shutdown signal");
            let _ = shutdown_tx.send(());
        }
    }

    info!("notification-worker shutdown complete");
    Ok(())
}
