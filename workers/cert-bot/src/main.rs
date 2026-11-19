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

use adapters::aws::dynamo::order_store::DynamoOrderStore;
use adapters::click_router_api::ClickRouterApiClient;
use adapters::mongodb::MongodbOrderStore;
use adapters::OrderStoreType;
use settings::Settings;
use worker::{CertificateWorker, DomainConsumer, RenewalWorker};

async fn load_aws_config(settings: &adapters::aws::settings::AwsSettings) -> aws_config::SdkConfig {
    let mut shared_config = aws_config::defaults(aws_config::BehaviorVersion::latest());

    if settings.local {
        let endpoint = settings
            .localstack_endpoint
            .clone()
            .unwrap_or_else(|| "http://localhost:4566".to_string());
        shared_config = shared_config.endpoint_url(endpoint);
    }

    shared_config.load().await
}

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
    use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

    dotenv::from_filename("./.env").ok();

    rustls::crypto::ring::default_provider()
        .install_default()
        .expect("Failed to install rustls crypto provider");

    // Initialize tracing with Loki integration
    let loki_url = std::env::var("LOKI_URL").unwrap_or_else(|_| "http://loki:3100".to_string());

    let filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new("warn"));

    let loki_result = url::Url::parse(&loki_url)
        .ok()
        .and_then(|url| {
            tracing_loki::builder()
                .label("service", "cert-bot")
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
            eprintln!("cert-bot: Failed to initialize Loki logging");
        }
    }

    let args = Args::parse();

    info!("Starting Cert-Bot");
    info!("   Mode: {}", args.run_mode);

    let settings = Settings::new(Some(&args.run_mode), Some(&args.config_path))?;

    // Create shared dependencies. AWS run modes (aws, aws-dev) use DynamoDB;
    // everything else uses MongoDB. OrderStoreType is Clone (cheap - just
    // wraps a DB client handle), so each worker gets its own owned copy
    // instead of sharing one behind an Arc<dyn OrderStore>.
    let order_store: OrderStoreType = if args.run_mode.starts_with("aws") {
        let dynamo_settings = settings
            .aws
            .dynamo
            .clone()
            .expect("aws.dynamo settings are required when APP_RUN_MODE starts with 'aws'");
        let aws_config = load_aws_config(&settings.aws).await;
        OrderStoreType::Dynamo(DynamoOrderStore::new(&aws_config, dynamo_settings.orders_table))
    } else {
        OrderStoreType::Mongodb(MongodbOrderStore::new(&settings.mongodb).await?)
    };
    let api_client = Arc::new(ClickRouterApiClient::new(&settings.click_router_api)?);

    info!("Cert-Bot initialized, starting workers...");

    // Initialize workers
    let certificate_worker = CertificateWorker::new(settings.clone(), order_store.clone()).await?;
    let renewal_worker = RenewalWorker::new(
        settings.clone(),
        order_store.clone(),
        Arc::clone(&api_client),
    );
    let domain_consumer = DomainConsumer::new(
        settings.clone(),
        order_store,
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
