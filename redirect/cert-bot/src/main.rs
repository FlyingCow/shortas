//! Cert-Bot - Let's Encrypt Certificate Automation Service
//!
//! This service automatically creates and renews SSL certificates for active domains
//! using Let's Encrypt's HTTP-01 challenge.
//!
//! ## Features
//! - Automatic certificate creation for verified domains
//! - Certificate renewal before expiration
//! - Integration with click-router for challenge serving
//! - RabbitMQ event-driven domain verification listening

use clap::Parser;
use std::sync::Arc;
use tracing::{error, info};

mod adapters;
mod core;
mod model;
mod settings;
mod worker;

use adapters::click_router_api::ClickRouterApiClient;
use adapters::mongodb::MongodbOrderStore;
use settings::Settings;
use worker::{CertificateWorker, DomainConsumer, RenewalWorker};

/// Command-line arguments for the Cert-Bot application
#[derive(Parser, Debug)]
#[command(version)]
pub struct Args {
    /// Application run mode (development, production, test)
    #[arg(short, long, default_value_t = String::from("production"), env("APP_RUN_MODE"))]
    pub run_mode: String,
    /// Path to the configuration directory
    #[arg(short, long, default_value_t = String::from("./config"), env("APP_CONFIG_PATH"))]
    pub config_path: String,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt().init();

    dotenv::from_filename("./.env").ok();

    let args = Args::parse();

    info!("Starting Cert-Bot");
    info!("   Mode: {}", args.run_mode);

    let settings = Settings::new(Some(&args.run_mode), Some(&args.config_path))?;

    // Create shared dependencies
    let order_store = Arc::new(MongodbOrderStore::new(&settings.mongodb).await?);
    let api_client = Arc::new(ClickRouterApiClient::new(&settings.click_router_api)?);

    info!("Cert-Bot initialized, starting workers...");

    // Initialize workers
    let certificate_worker = CertificateWorker::new(settings.clone()).await?;
    let renewal_worker = RenewalWorker::new(
        settings.clone(),
        Arc::clone(&order_store),
        Arc::clone(&api_client),
    );
    let domain_consumer = DomainConsumer::new(
        settings.clone(),
        Arc::clone(&order_store),
        Arc::clone(&api_client),
    );

    // Run all workers concurrently
    tokio::select! {
        result = certificate_worker.run() => {
            error!("Certificate worker stopped: {:?}", result);
        }
        result = renewal_worker.run() => {
            error!("Renewal worker stopped: {:?}", result);
        }
        result = domain_consumer.run() => {
            error!("Domain consumer stopped: {:?}", result);
        }
    }

    Ok(())
}
