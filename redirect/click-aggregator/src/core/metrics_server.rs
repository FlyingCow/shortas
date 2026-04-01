//! HTTP server for exposing Prometheus metrics
//!
//! Provides a simple HTTP server that serves metrics on a configurable port.

use anyhow::Result;
use http_body_util::Full;
use hyper::body::Bytes;
use hyper::server::conn::http1;
use hyper::service::service_fn;
use hyper::{Request, Response, StatusCode};
use hyper_util::rt::TokioIo;
use prometheus::{Encoder, TextEncoder};
use std::convert::Infallible;
use std::net::SocketAddr;
use tokio::net::TcpListener;
use tokio_util::sync::CancellationToken;

use super::metrics::METRICS;

async fn handle_request(
    req: Request<hyper::body::Incoming>,
) -> Result<Response<Full<Bytes>>, Infallible> {
    let path = req.uri().path();

    match path {
        "/metrics" => {
            let encoder = TextEncoder::new();
            let metric_families = prometheus::gather();

            match encoder.encode_to_string(&metric_families) {
                Ok(metrics_text) => Ok(Response::builder()
                    .status(StatusCode::OK)
                    .header("Content-Type", "text/plain; version=0.0.4; charset=utf-8")
                    .body(Full::new(Bytes::from(metrics_text)))
                    .unwrap()),
                Err(e) => {
                    tracing::error!("Failed to encode metrics: {}", e);
                    Ok(Response::builder()
                        .status(StatusCode::INTERNAL_SERVER_ERROR)
                        .body(Full::new(Bytes::from("Failed to encode metrics")))
                        .unwrap())
                }
            }
        }
        "/health" => {
            let health_info = serde_json::json!({
                "status": "healthy",
                "timestamp": std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_secs(),
                "metrics": {
                    "clicks_processed_total": METRICS.clicks_processed_total.get(),
                    "debug_clicks_total": METRICS.debug_clicks_total.get(),
                }
            });

            Ok(Response::builder()
                .status(StatusCode::OK)
                .header("Content-Type", "application/json")
                .body(Full::new(Bytes::from(health_info.to_string())))
                .unwrap())
        }
        _ => Ok(Response::builder()
            .status(StatusCode::NOT_FOUND)
            .body(Full::new(Bytes::from("Not Found")))
            .unwrap()),
    }
}

/// Start the metrics HTTP server
pub async fn start_metrics_server(port: u16, token: CancellationToken) -> Result<()> {
    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    let listener = TcpListener::bind(addr).await?;

    tracing::info!("Metrics server listening on http://{}", addr);

    loop {
        tokio::select! {
            _ = token.cancelled() => {
                tracing::info!("Metrics server shutting down");
                break;
            }
            result = listener.accept() => {
                match result {
                    Ok((stream, _)) => {
                        let io = TokioIo::new(stream);
                        tokio::spawn(async move {
                            if let Err(err) = http1::Builder::new()
                                .serve_connection(io, service_fn(handle_request))
                                .await
                            {
                                tracing::error!("Error serving connection: {:?}", err);
                            }
                        });
                    }
                    Err(e) => {
                        tracing::error!("Failed to accept connection: {}", e);
                    }
                }
            }
        }
    }

    Ok(())
}
