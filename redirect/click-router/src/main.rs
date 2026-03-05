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

use salvo::{
    async_trait,
    conn::{
        rustls_async::{Keycert, ResolvesServerConfig, RustlsConfig},
        TcpListener,
    },
    prelude::Logger,
    writing::Json,
    Depot, FlowCtrl, Handler, Listener, Request, Response, Router, Server, Service,
};
use salvo_proxy::{hyper_client::HyperClient, Proxy};
use tokio::time::{timeout, Duration};

use click_router::{
    adapters::{
        api::conversion_routes,
        salvo::{salvo_proxy, SalvoRequest, SalvoResponse},
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
struct Redirect;

// fn to_socket_addr()

#[async_trait]
impl Handler for Redirect {
    /// Handles incoming HTTP requests through the flow router
    ///
    /// This method processes each request through the complete flow pipeline,
    /// collecting metrics and generating the appropriate response based on
    /// the flow router's decision.
    ///
    /// # Arguments
    /// * `req` - The incoming HTTP request
    /// * `depot` - Salvo's request-scoped data storage
    /// * `res` - The HTTP response being constructed
    /// * `ctrl` - Flow control for the request pipeline
    async fn handle(
        &self,
        req: &mut Request,
        depot: &mut Depot,
        res: &mut Response,
        ctrl: &mut FlowCtrl,
    ) {
        // Start timing the request
        let request_timer = Timer::new();
        METRICS.requests_total.inc();
        METRICS.active_requests.inc();

        let router = get_flow_router();

        // Wrap the entire request handling with a 5-second timeout
        let timeout_result = timeout(
            Duration::from_secs(5),
            router.handle(
                &RequestType::Salvo(&SalvoRequest::new(&req)),
                &ResponseType::Salvo(&mut SalvoResponse::new(res)),
            ),
        )
        .await;

        let result = match timeout_result {
            Ok(result) => result,
            Err(_) => {
                // Timeout occurred
                tracing::warn!("Request timeout after 5 seconds");
                METRICS.requests_error.inc();
                res.status_code(StatusCode::GATEWAY_TIMEOUT).render("");
                request_timer.observe_duration_seconds(&METRICS.request_duration);
                METRICS.active_requests.dec();
                return;
            }
        };

        // Handle the result and update metrics
        match result {
            Ok(flow_result) => {
                METRICS.requests_success.inc();

                match flow_result {
                    FlowRouterResult::Empty(status_code) => res.status_code(status_code).render(""),
                    FlowRouterResult::Json(content, status_code) => {
                        res.status_code(status_code).render(Json(content))
                    }
                    FlowRouterResult::PlainText(content, status_code) => {
                        res.status_code(status_code).render(content)
                    }
                    FlowRouterResult::Image(data, content_type, status_code) => {
                        res.status_code(status_code)
                            .add_header("Content-Type", content_type, true)
                            .unwrap();
                        let _ = res.write_body(data);
                    }
                    FlowRouterResult::Proxied(url, _status_code) => {
                        let url = url.to_string();
                        let proxy = Proxy::new(url, HyperClient::default());
                        proxy.handle(req, depot, res, ctrl).await;
                    }
                    FlowRouterResult::Redirect(url, redirect_type) => {
                        match redirect_type {
                            RedirectType::Permanent => {
                                res.status_code(StatusCode::PERMANENT_REDIRECT)
                            }
                            RedirectType::Temporary => {
                                res.status_code(StatusCode::TEMPORARY_REDIRECT)
                            }
                        };
                        res.add_header("Location", url.to_string(), true)
                            .unwrap()
                            .render("");
                    }
                    FlowRouterResult::Retargeting(url, _script_urls) => res.render(url.to_string()),
                    FlowRouterResult::Error => {
                        METRICS.requests_error.inc();
                        res.status_code(StatusCode::INTERNAL_SERVER_ERROR)
                            .render("")
                    }
                }
            }
            Err(_) => {
                METRICS.requests_error.inc();
                res.status_code(StatusCode::INTERNAL_SERVER_ERROR)
                    .render("");
            }
        }

        // Record request duration and decrement active requests
        request_timer.observe_duration_seconds(&METRICS.request_duration);
        METRICS.active_requests.dec();
    }
}

/// Dynamic TLS certificate resolver that loads certificates from the crypto cache
///
/// This resolver implements SNI-based certificate selection, allowing different
/// certificates to be served for different domain names. If no crypto cache is
/// configured, it falls back to the default embedded certificate.
struct DynamicServerConfigResolver {
    crypto_cache: Option<CryptoCacheType>,
    default_config: Arc<RustlsConfig>,
}

impl DynamicServerConfigResolver {
    /// Create a new dynamic certificate resolver with optional crypto cache
    fn new(crypto_cache: Option<CryptoCacheType>) -> Self {
        // Create default certificate config from embedded files
        let default_config = Arc::new(RustlsConfig::new(
            Keycert::new()
                .cert(include_bytes!("../certs/cert.pem").as_ref())
                .key(include_bytes!("../certs/key.pem").as_ref()),
        ));

        Self {
            crypto_cache,
            default_config,
        }
    }
}

#[async_trait]
impl ResolvesServerConfig<IoError> for DynamicServerConfigResolver {
    async fn resolve(&self, client_hello: ClientHello<'_>) -> IoResult<Arc<RustlsConfig>> {
        // If no crypto cache, use default certificate
        let crypto_cache = match &self.crypto_cache {
            Some(cache) => cache,
            None => return Ok(self.default_config.clone()),
        };

        // Extract SNI hostname from client hello
        let server_name = match client_hello.server_name() {
            Some(name) => name,
            None => {
                tracing::debug!("No SNI hostname in client hello, using default certificate");
                return Ok(self.default_config.clone());
            }
        };

        tracing::debug!("TLS connection for domain: {}", server_name);

        // Try to get certificate for this domain from cache
        match crypto_cache.get_certificate(server_name).await {
            Ok(Some(keycert)) => {
                tracing::debug!("Found certificate for domain: {}", server_name);
                let config = RustlsConfig::new(
                    Keycert::new()
                        .cert(keycert.cert.as_slice())
                        .key(keycert.key.as_slice()),
                );
                Ok(Arc::new(config))
            }
            Ok(None) => {
                tracing::debug!(
                    "No certificate found for domain: {}, using default",
                    server_name
                );
                Ok(self.default_config.clone())
            }
            Err(e) => {
                tracing::error!(
                    "Failed to load certificate for domain {}: {}, using default",
                    server_name,
                    e
                );
                Ok(self.default_config.clone())
            }
        }
    }
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
    eprintln!("click-router: starting...");

    rustls::crypto::ring::default_provider()
        .install_default()
        .expect("Failed to install rustls crypto provider");

    eprintln!("click-router: rustls initialized");

    tracing_subscriber::fmt().init();

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

    // Create main application router with both redirect and API functionality
    let app_router = Router::new()
        .push(conversion_routes::conversion_routes()) // Add conversion API routes
        .push(Router::with_path("{**rest_path}").get(Redirect)); // Keep redirect functionality

    tracing::info!("🚀 Starting Click Router");
    tracing::info!("   Main server: https://{}", args.listen_addr);

    // Start metrics server if enabled
    if args.enable_metrics {
        let metrics_router = create_metrics_router();
        let metrics_service = Service::new(metrics_router).hoop(Logger::new());

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

        // Start metrics server in background with default address
        tokio::spawn(async move {
            let metrics_acceptor = TcpListener::new("0.0.0.0:9090").bind().await;
            Server::new(metrics_acceptor).serve(metrics_service).await;
        });
    } else {
        tracing::info!("📊 Metrics endpoints disabled (use --enable-metrics to enable)");
    }

    // Start main application server with dynamic certificate resolution
    if crypto_cache.is_some() {
        tracing::info!("🔐 Dynamic TLS certificate resolution enabled");
    } else {
        tracing::warn!("⚠️ Crypto cache not available, using fallback certificate only");
    }

    let acceptor = TcpListener::new("0.0.0.0:5800")
        .rustls_async(DynamicServerConfigResolver::new(crypto_cache))
        .bind()
        .await;

    let service = Service::new(app_router).hoop(Logger::new());

    tracing::info!("✅ Click Router started successfully!");
    tracing::info!("");

    Server::new(acceptor).serve(service).await;
}
