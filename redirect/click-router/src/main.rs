//! Click Router - High-Performance URL Routing and Analytics Service
//!
//! This is the main entry point for the Click Router application, a high-performance
//! HTTP service that handles URL routing, redirects, and analytics collection.
//!
//! ## Features
//! - High-performance request routing with caching
//! - Real-time analytics and metrics collection
//! - Geographic and user agent detection
//! - Flexible module system for custom processing logic
//! - Prometheus metrics integration
//! - TLS/SSL support with dynamic certificate resolution
//!
//! ## Architecture
//! The application uses a flow-based processing model where requests pass through
//! multiple stages (Start, UrlExtract, Register, BuildResult, End), with each stage
//! handled by configurable modules.

use clap::Parser;
use http::StatusCode;
use rustls::server::ClientHello;
use std::{
    io::{Error as IoError, Result as IoResult},
    sync::Arc,
};

use axum::{
    body::Body,
    extract::{ConnectInfo, Request, State},
    http::Request as HttpRequest,
    response::{IntoResponse, Response},
    routing::get,
    Json, Router,
};
use axum_server::tls_rustls::RustlsConfig;
use rustls::server::ResolvesServerCert;
use rustls::sign::CertifiedKey;
use tower_cookies::{CookieManagerLayer, Cookies};
use tower_http::trace::TraceLayer;
use tokio::time::{timeout, Duration};

use click_router::{
    adapters::{
        api::conversion_routes,
        axum::{axum_proxy::{self, hyper_client::HyperClient, Proxy}, parse_queries, AxumRequest, AxumResponse},
        CryptoCacheType, RequestType, ResponseType,
    },
    app::AppBuilder,
    core::{
        crypto::CryptoCache,
        flow_router::{FlowRouterResult, RedirectType},
        metrics::{Timer, METRICS},
        metrics_endpoint::create_metrics_router,
    },
    get_flow_router, init_flow_router,
    settings::Settings,
};

/// Command-line arguments for the Click Router application
///
/// This structure defines all the configurable parameters that can be passed
/// via command line arguments or environment variables.
#[derive(Parser, Debug)]
#[command(version)]
pub struct Args {
    /// Application run mode (development, production, test)
    #[arg(short, long, default_value_t = String::from("production"), env("APP_RUN_MODE"))]
    pub run_mode: String,
    /// Path to the configuration directory
    #[arg(short, long, default_value_t = String::from("./config"), env("APP_CONFIG_PATH"))]
    pub config_path: String,
    /// Address and port for the main HTTP server
    #[arg(long, default_value_t = String::from("0.0.0.0:5800"), env("APP_LISTEN_ADDR"))]
    pub listen_addr: String,
    /// Address and port for the metrics HTTP server
    #[arg(long, default_value_t = String::from("0.0.0.0:9090"), env("APP_METRICS_ADDR"))]
    pub metrics_addr: String,
    /// Whether to enable metrics collection and endpoints
    #[arg(long, default_value_t = true, env("APP_ENABLE_METRICS"))]
    pub enable_metrics: bool,
}

/// Main HTTP request handler for the Click Router
///
/// This handler processes all incoming HTTP requests through the flow router,
/// collecting metrics and generating appropriate responses.
async fn redirect_handler(
    ConnectInfo(addr): ConnectInfo<std::net::SocketAddr>,
    req: Request,
) -> impl IntoResponse {
    // Start timing the request
    let request_timer = Timer::new();
    METRICS.requests_total.inc();
    METRICS.active_requests.inc();

    let router = get_flow_router();

    // Extract request components
    let (parts, body) = req.into_parts();
    let uri = parts.uri.clone();
    let headers = parts.headers.clone();
    let method = parts.method.clone();
    let scheme = uri.scheme().cloned().unwrap_or(http::uri::Scheme::HTTP);

    // Build AxumRequest wrapper
    // Note: Cookies are handled by tower-cookies layer, so we create an empty jar here
    let cookie_jar = cookie::CookieJar::new();
    let axum_req = AxumRequest::from_parts(
        uri,
        headers,
        method,
        scheme,
        indexmap::IndexMap::new(), // TODO: Extract path params if needed
        parse_queries(parts.uri.query()),
        Some(addr),
        cookie_jar.clone(),
    );

    // Build AxumResponse wrapper
    let mut axum_res = AxumResponse::new(cookie_jar);

    // Wrap the entire request handling with a 5-second timeout
    let timeout_result = timeout(
        Duration::from_secs(5),
        router.handle(
            &RequestType::Axum(&axum_req),
            &ResponseType::Axum(&mut axum_res),
        ),
    )
    .await;

    let result = match timeout_result {
        Ok(result) => result,
        Err(_) => {
            // Timeout occurred
            tracing::warn!("Request timeout after 5 seconds");
            METRICS.requests_error.inc();
            request_timer.observe_duration_seconds(&METRICS.request_duration);
            METRICS.active_requests.dec();
            return (StatusCode::GATEWAY_TIMEOUT, "").into_response();
        }
    };

    // Handle the result and update metrics
    let response = match result {
        Ok(flow_result) => {
            METRICS.requests_success.inc();

            match flow_result {
                FlowRouterResult::Empty(status_code) => (status_code, "").into_response(),
                FlowRouterResult::Json(content, status_code) => {
                    (status_code, Json(content)).into_response()
                }
                FlowRouterResult::PlainText(content, status_code) => {
                    (status_code, content).into_response()
                }
                FlowRouterResult::Image(data, content_type, status_code) => {
                    (
                        status_code,
                        [(http::header::CONTENT_TYPE, content_type)],
                        data,
                    )
                        .into_response()
                }
                FlowRouterResult::Proxied(_url, _status_code) => {
                    // TODO: Implement proxy support for Axum
                    // For now, return a placeholder response
                    (StatusCode::NOT_IMPLEMENTED, "Proxy not yet implemented").into_response()
                }
                FlowRouterResult::Redirect(url, redirect_type) => {
                    let status = match redirect_type {
                        RedirectType::Permanent => StatusCode::PERMANENT_REDIRECT,
                        RedirectType::Temporary => StatusCode::TEMPORARY_REDIRECT,
                    };
                    (
                        status,
                        [(http::header::LOCATION, url.to_string())],
                        "",
                    )
                        .into_response()
                }
                FlowRouterResult::Retargeting(url, _script_urls) => url.to_string().into_response(),
                FlowRouterResult::Error => {
                    METRICS.requests_error.inc();
                    (StatusCode::INTERNAL_SERVER_ERROR, "").into_response()
                }
            }
        }
        Err(_) => {
            METRICS.requests_error.inc();
            (StatusCode::INTERNAL_SERVER_ERROR, "").into_response()
        }
    };

    // Record request duration and decrement active requests
    request_timer.observe_duration_seconds(&METRICS.request_duration);
    METRICS.active_requests.dec();

    response
}

/// Create a default TLS configuration from embedded certificates
async fn create_default_tls_config() -> RustlsConfig {
    // TODO: Implement dynamic certificate resolution for SNI
    // For now, using static embedded certificates
    RustlsConfig::from_pem_file(
        "certs/cert.pem",
        "certs/key.pem"
    )
    .await
    .expect("Failed to load TLS certificates")
}

/// Main entry point for the Click Router application
///
/// This function initializes the application, sets up the flow router with all
/// necessary components, and starts both the main HTTP server and the metrics
/// server (if enabled).
///
/// The application supports:
/// - TLS/SSL termination with dynamic certificate resolution
/// - Concurrent main and metrics servers
/// - Configurable modules and adapters
/// - Comprehensive logging and monitoring
#[tokio::main]
async fn main() {
    use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

    eprintln!("click-router: starting...");

    rustls::crypto::ring::default_provider()
        .install_default()
        .expect("Failed to install rustls crypto provider");

    eprintln!("click-router: rustls initialized");

    // Initialize tracing with Loki integration
    let loki_url = std::env::var("LOKI_URL").unwrap_or_else(|_| "http://loki:3100".to_string());

    let filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("warn"));

    let loki_result = url::Url::parse(&loki_url).ok().and_then(|url| {
        tracing_loki::builder()
            .label("service", "click-router")
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
            eprintln!("click-router: Failed to initialize Loki logging");
        }
    }

    eprintln!("click-router: tracing initialized");

    dotenv::from_filename("./.env").ok();

    let args = Args::parse();

    eprintln!("click-router: args parsed");

    let settings = Settings::new(
        Some(args.run_mode.as_str()),
        Some(args.config_path.as_str()),
    )
    .unwrap();

    let mut app_builder = AppBuilder::new(settings)
        .with_geo_ip()
        .with_ua_parser()
        .with_fluvio()
        .await
        .with_mongodb()
        .await
        .with_rabbitmq()
        //.with_dynamo()
        // .await
        .with_default_modules();

    // Get crypto cache for dynamic certificate resolution before building
    let crypto_cache = app_builder.get_crypto_cache();

    let flow_router = app_builder.build();
    init_flow_router(flow_router);

    // Create routers for HTTP and HTTPS servers
    let app = Router::new()
        .merge(conversion_routes::conversion_routes())
        .route("/*rest_path", get(redirect_handler))
        .layer(CookieManagerLayer::new())
        .layer(TraceLayer::new_for_http())
        .into_make_service_with_connect_info::<std::net::SocketAddr>();

    tracing::info!("🚀 Starting Click Router");
    tracing::info!("   HTTP server: http://0.0.0.0:5800");
    tracing::info!("   HTTPS server: https://0.0.0.0:4433");

    // Start metrics server if enabled
    if args.enable_metrics {
        let metrics_router = create_metrics_router();

        tracing::info!("📊 Metrics endpoints enabled:");
        tracing::info!("   Metrics server: http://{}", args.metrics_addr);
        tracing::info!(
            "   • GET {}/health        - Health check",
            args.metrics_addr
        );
        tracing::info!(
            "   • GET {}/metrics       - Prometheus metrics",
            args.metrics_addr
        );
        tracing::info!(
            "   • GET {}/metrics/info  - Detailed metrics info",
            args.metrics_addr
        );

        // Start metrics server in background
        let metrics_addr = args.metrics_addr.clone();
        tokio::spawn(async move {
            let listener = tokio::net::TcpListener::bind(&metrics_addr)
                .await
                .expect("Failed to bind metrics server");
            tracing::info!("📊 Metrics server listening on http://{}", metrics_addr);
            axum::serve(listener, metrics_router)
                .await
                .expect("Metrics server failed");
        });
    } else {
        tracing::info!("📊 Metrics endpoints disabled (use --enable-metrics to enable)");
    }

    // Start main application server
    if crypto_cache.is_some() {
        tracing::info!("🔐 Dynamic TLS certificate resolution enabled (will be implemented)");
    } else {
        tracing::warn!("⚠️ Crypto cache not available");
    }

    // Start HTTPS server on port 4433
    let app_clone = app.clone();
    tokio::spawn(async move {
        let tls_config = create_default_tls_config().await;
        tracing::info!("🔐 HTTPS server listening on https://0.0.0.0:4433");
        axum_server::bind_rustls("0.0.0.0:4433".parse().unwrap(), tls_config)
            .serve(app_clone)
            .await
            .expect("HTTPS server failed");
    });

    // Start HTTP server on port 5800
    let listener = tokio::net::TcpListener::bind("0.0.0.0:5800")
        .await
        .expect("Failed to bind HTTP server");

    tracing::info!("✅ Click Router started successfully!");
    tracing::info!("   HTTP:  http://0.0.0.0:5800");
    tracing::info!("   HTTPS: https://0.0.0.0:4433");
    tracing::info!("");

    axum::serve(listener, app)
        .await
        .expect("HTTP server failed");
}
