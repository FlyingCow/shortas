//! Management API - Rust implementation of the URL shortening management service.
//! Build timestamp: 2026-05-12T16:40:00Z
//!
//! This service provides REST APIs for managing routes, domains, workspaces,
//! certificates, and analytics.

use anyhow::Result;
use clap::Parser;
use salvo::prelude::*;
use sqlx::postgres::PgPoolOptions;
use std::sync::Arc;
use tokio::sync::broadcast;
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;

use management_api::infrastructure::messaging::{OutboxProcessor, RabbitMqConsumer};
use management_api::presentation::controllers::api_routes;
use management_api::presentation::middleware::{AppState, AppStateMiddleware};
use management_api::settings::Settings;

/// Command-line arguments for the Management API.
#[derive(Parser, Debug)]
#[command(version)]
pub struct Args {
    /// Application run mode (development, production, test)
    #[arg(short, long, default_value_t = String::from("development"), env("APP_RUN_MODE"))]
    pub run_mode: String,
    /// Path to the configuration directory
    #[arg(short, long, default_value_t = String::from("./config"), env("APP_CONFIG_PATH"))]
    pub config_path: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .with_target(true)
        .with_thread_ids(true)
        .finish();
    tracing::subscriber::set_global_default(subscriber)?;

    info!("Starting Management API v{}", env!("CARGO_PKG_VERSION"));

    // Load .env file if present
    dotenv::from_filename("./.env").ok();

    // Parse command line arguments
    let args = Args::parse();
    info!("Run mode: {}, Config path: {}", args.run_mode, args.config_path);

    // Load settings
    let settings = Settings::new(Some(&args.run_mode), Some(&args.config_path))?;
    info!("Loaded configuration");

    // Create database pool
    let pool = PgPoolOptions::new()
        .max_connections(settings.database.max_connections)
        .connect(&settings.database.connection_string())
        .await?;
    info!("Connected to database");

    // Run migrations
    info!("Running database migrations...");
    sqlx::migrate!("./migrations")
        .run(&pool)
        .await?;
    info!("Database migrations completed");

    // Seed shared domains
    seed_shared_domains(&pool, &settings.shared_domains.names).await?;

    // Create application state
    let app_state = AppState::new(settings.clone(), pool.clone()).await?;
    info!("Initialized application state");

    // Create shutdown channel
    let (shutdown_tx, _) = broadcast::channel::<()>(1);

    // Start background services
    let outbox_processor = Arc::new(OutboxProcessor::new(
        app_state.outbox_repo.clone(),
        app_state.search_service.clone(),
        app_state.route_repo.clone(),
    ));

    let outbox_shutdown = shutdown_tx.subscribe();
    let outbox = outbox_processor.clone();
    tokio::spawn(async move {
        outbox.start(outbox_shutdown).await;
    });
    info!("Started outbox processor");

    // Start RabbitMQ consumers
    if let Ok(consumer) = RabbitMqConsumer::new(&settings.rabbitmq).await {
        let consumer = Arc::new(consumer);

        // Domain verification consumer
        let domain_consumer = consumer.clone();
        let domain_repo = app_state.domain_repo.clone();
        let domain_shutdown = shutdown_tx.subscribe();
        tokio::spawn(async move {
            if let Err(e) = domain_consumer
                .start_domain_verification_consumer(domain_repo, domain_shutdown)
                .await
            {
                tracing::error!("Domain verification consumer error: {}", e);
            }
        });

        // Route status consumer
        let status_consumer = consumer.clone();
        let route_repo = app_state.route_repo.clone();
        let status_shutdown = shutdown_tx.subscribe();
        tokio::spawn(async move {
            if let Err(e) = status_consumer
                .start_route_status_consumer(route_repo, status_shutdown)
                .await
            {
                tracing::error!("Route status consumer error: {}", e);
            }
        });

        info!("Started RabbitMQ consumers");
    } else {
        tracing::warn!("RabbitMQ not available, consumers not started");
    }

    // Build router
    let api_router = api_routes(settings.jwt.clone());

    // Add OpenAPI documentation
    let doc = OpenApi::new("Management API", env!("CARGO_PKG_VERSION")).merge_router(&api_router);

    // Build final router with middleware
    let router = Router::new()
        .hoop(AppStateMiddleware::new(app_state))
        .push(api_router)
        .unshift(doc.into_router("/api-doc/openapi.json"))
        .unshift(SwaggerUi::new("/api-doc/openapi.json").into_router("/swagger-ui"));

    // Start server
    let bind_address = format!("{}:{}", settings.server.host, settings.server.port);
    info!("Starting server on {}", bind_address);

    let bind_address: &'static str = Box::leak(bind_address.into_boxed_str());
    let acceptor = TcpListener::new(bind_address).bind().await;

    // Handle graceful shutdown
    let server = Server::new(acceptor);

    tokio::select! {
        _ = server.serve(router) => {}
        _ = tokio::signal::ctrl_c() => {
            info!("Received shutdown signal");
            let _ = shutdown_tx.send(());
        }
    }

    info!("Server shutdown complete");
    Ok(())
}

/// Seed shared domains on startup.
/// Creates any configured shared domains that don't already exist.
async fn seed_shared_domains(pool: &sqlx::PgPool, domain_names: &[String]) -> Result<()> {
    use sqlx::Row;
    use uuid::Uuid;

    if domain_names.is_empty() {
        info!("No shared domains configured");
        return Ok(());
    }

    info!("Seeding shared domains: {:?}", domain_names);

    const SYSTEM_OWNER_ID: &str = "__system__";

    for name in domain_names {
        let normalized_name = name.to_lowercase();

        // Check if domain exists
        let existing = sqlx::query("SELECT id, is_shared FROM route_domains WHERE LOWER(name) = LOWER($1)")
            .bind(&normalized_name)
            .fetch_optional(pool)
            .await?;

        match existing {
            Some(row) => {
                let is_shared: bool = row.try_get("is_shared").unwrap_or(false);
                if !is_shared {
                    tracing::warn!(
                        "Domain '{}' exists but is not marked as shared. Skipping.",
                        normalized_name
                    );
                } else {
                    tracing::debug!("Shared domain '{}' already exists", normalized_name);
                }
            }
            None => {
                let id = Uuid::new_v4();
                sqlx::query(
                    r#"
                    INSERT INTO route_domains (id, name, owner_id, is_shared, verification_status, verification_reason, created_at, updated_at)
                    VALUES ($1, $2, $3, true, 'Verified', 'shared_domain', NOW(), NOW())
                    "#,
                )
                .bind(id)
                .bind(&normalized_name)
                .bind(SYSTEM_OWNER_ID)
                .execute(pool)
                .await?;

                info!("Created shared domain: {} (ID: {})", normalized_name, id);
            }
        }
    }

    info!("Shared domain seeding completed");
    Ok(())
}
