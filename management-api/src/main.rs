//! Management API - Rust implementation of the URL shortening management service.
//!
//! This service provides REST APIs for managing routes, domains, workspaces,
//! certificates, and analytics.

use anyhow::Result;
use salvo::cors::Cors;
use salvo::http::Method;
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

    // Load settings
    let settings = Settings::load()?;
    info!("Loaded configuration");

    // Create database pool
    let pool = PgPoolOptions::new()
        .max_connections(settings.database.max_connections)
        .connect(&settings.database.connection_string())
        .await?;
    info!("Connected to database");

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
    let router = api_routes(settings.jwt.clone());

    // Add OpenAPI documentation
    let doc = OpenApi::new("Management API", env!("CARGO_PKG_VERSION")).merge_router(&router);

    // Configure CORS
    let cors = Cors::new()
        .allow_origin("*")
        .allow_methods(vec![
            Method::GET,
            Method::POST,
            Method::PUT,
            Method::DELETE,
            Method::OPTIONS,
        ])
        .allow_headers(vec!["Authorization", "Content-Type"])
        .max_age(3600)
        .into_handler();

    // Build final router with middleware
    let router = router
        .hoop(cors)
        .hoop(AppStateMiddleware::new(app_state))
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
