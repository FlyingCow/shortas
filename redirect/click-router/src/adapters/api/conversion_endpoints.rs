//! Conversion endpoints, kept apart from short links under a reserved path.

use std::{
    net::{IpAddr, SocketAddr},
    sync::Arc,
    time::{Duration, Instant},
};

use axum::{
    body::Body,
    extract::{ConnectInfo, Path, Request, State},
    response::{IntoResponse, Response},
    routing::{get, post},
    Json, Router,
};
use chrono::Utc;
use http::{
    header::{
        ACCESS_CONTROL_ALLOW_ORIGIN, ACCESS_CONTROL_EXPOSE_HEADERS, AUTHORIZATION, CACHE_CONTROL,
        CONTENT_TYPE, ORIGIN, RETRY_AFTER, X_CONTENT_TYPE_OPTIONS,
    },
    HeaderMap, HeaderValue, StatusCode,
};
use lazy_static::lazy_static;
use prometheus::{
    register_histogram, register_int_counter_vec, register_int_gauge, Histogram, IntCounterVec,
    IntGauge,
};
use serde_json::{json, Map, Value};
use sha2::{Digest, Sha256};
use subtle::ConstantTimeEq;
use tokio::{sync::Semaphore, time::timeout};

use crate::{
    adapters::moka::conversion_cache::MokaConversionCache,
    core::{
        client_address::{limit_key, TrustedProxies},
        conversion_events::{read, Kind, Rejection, Source},
        conversion_publisher::ConversionPublisher,
        rate_limit::RateLimiter,
    },
    model::Conversion,
    settings::Conversions,
};

pub const PREFIX: &str = "/.well-known/shortas";

const MAX_BODY_BYTES: usize = 16 * 1024;
const BODY_TIMEOUT: Duration = Duration::from_secs(5);
const SCRIPT: &str = include_str!("conversion_script.js");
const SCRIPT_CACHE: &str = "public, max-age=300";
/// Suggested wait after a temporary failure.
const RETRY_SECONDS: u64 = 5;

lazy_static! {
    static ref REQUESTS: IntCounterVec = register_int_counter_vec!(
        "conversion_requests_total",
        "Conversion requests by endpoint and response status",
        &["endpoint", "status"]
    )
    .unwrap();
    static ref DURATION: Histogram = register_histogram!(
        "conversion_request_duration_seconds",
        "Time to answer conversion requests"
    )
    .unwrap();
    static ref IN_FLIGHT: IntGauge = register_int_gauge!(
        "conversion_requests_in_flight",
        "Conversion requests being handled"
    )
    .unwrap();
    static ref PUBLISH_FAILURES: IntCounterVec = register_int_counter_vec!(
        "conversion_publish_failures_total",
        "Records the conversion stream did not accept",
        &["outcome"]
    )
    .unwrap();
}

pub struct ConversionEndpoints {
    definitions: MokaConversionCache,
    publisher: Arc<dyn ConversionPublisher>,
    permits: Arc<Semaphore>,
    proxies: TrustedProxies,
    addresses: RateLimiter<IpAddr>,
    /// Failed server authentications. Verified requests have only the key limit.
    failed_secrets: RateLimiter<IpAddr>,
    /// Apart per source, so forged browser events can't block a server.
    browser_keys: RateLimiter<String>,
    server_keys: RateLimiter<String>,
}

impl ConversionEndpoints {
    pub fn new(
        settings: &Conversions,
        definitions: MokaConversionCache,
        publisher: Arc<dyn ConversionPublisher>,
    ) -> anyhow::Result<Self> {
        Ok(Self {
            definitions,
            publisher,
            permits: Arc::new(Semaphore::new(settings.max_concurrent_requests)),
            proxies: TrustedProxies::parse(&settings.trusted_proxies)?,
            addresses: RateLimiter::new(settings.address_rate),
            failed_secrets: RateLimiter::new(settings.address_rate),
            browser_keys: RateLimiter::new(settings.key_rate),
            server_keys: RateLimiter::new(settings.key_rate),
        })
    }

    fn address(&self, peer: SocketAddr, headers: &HeaderMap) -> IpAddr {
        limit_key(self.proxies.client_address(peer.ip(), headers))
    }

    pub fn router(self) -> Router {
        let routes = Router::new()
            .route("/c/:file", get(script))
            .route("/v1/events", post(browser_event))
            .route("/v1/conversions", post(server_event))
            .route("/v1/refunds", post(refund))
            .with_state(Arc::new(self));
        not_found_prefix().merge(Router::new().nest(PREFIX, routes))
    }
}

/// Nothing else under the prefix may reach short links.
pub fn not_found_prefix() -> Router {
    Router::new()
        .route(&format!("{PREFIX}/"), axum::routing::any(not_found))
        .route(&format!("{PREFIX}/*rest"), axum::routing::any(not_found))
}

async fn not_found() -> StatusCode {
    StatusCode::NOT_FOUND
}

type Shared = State<Arc<ConversionEndpoints>>;
type Peer = ConnectInfo<SocketAddr>;

struct Failure {
    status: StatusCode,
    code: &'static str,
    field: Option<&'static str>,
    retry_after: Option<u64>,
}

impl Failure {
    fn new(status: StatusCode, code: &'static str) -> Self {
        Self {
            status,
            code,
            field: None,
            retry_after: None,
        }
    }

    fn invalid(field: &'static str) -> Self {
        Self {
            field: Some(field),
            ..Self::new(StatusCode::BAD_REQUEST, "invalid")
        }
    }

    fn unavailable() -> Self {
        Self {
            retry_after: Some(RETRY_SECONDS),
            ..Self::new(StatusCode::SERVICE_UNAVAILABLE, "unavailable")
        }
    }

    fn unauthorized() -> Self {
        Self::new(StatusCode::UNAUTHORIZED, "unauthorized")
    }

    fn limited(wait: Duration) -> Self {
        Self {
            retry_after: Some(wait.as_secs_f64().ceil().max(1.0) as u64),
            ..Self::new(StatusCode::TOO_MANY_REQUESTS, "rate_limited")
        }
    }
}

impl From<&Rejection> for Failure {
    fn from(rejection: &Rejection) -> Self {
        let status = match rejection {
            Rejection::NotActive => StatusCode::CONFLICT,
            Rejection::OriginNotAllowed => StatusCode::FORBIDDEN,
            Rejection::Invalid(_) => StatusCode::BAD_REQUEST,
        };
        Self {
            field: rejection.field(),
            ..Self::new(status, rejection.reason())
        }
    }
}

impl IntoResponse for Failure {
    fn into_response(self) -> Response {
        let mut body = json!({ "error": self.code });
        if let Some(field) = self.field {
            body["field"] = field.into();
        }
        let mut response = (self.status, Json(body)).into_response();
        let headers = response.headers_mut();
        headers.insert(CACHE_CONTROL, HeaderValue::from_static("no-store"));
        if let Some(seconds) = self.retry_after {
            headers.insert(RETRY_AFTER, seconds.into());
        }
        response
    }
}

/// Counts a request, also when its client goes away.
struct Measured {
    endpoint: &'static str,
    started: Instant,
    status: Option<StatusCode>,
}

impl Drop for Measured {
    fn drop(&mut self) {
        let status = self.status.map_or("cancelled".into(), |s| s.as_u16().to_string());
        REQUESTS.with_label_values(&[self.endpoint, status.as_str()]).inc();
        DURATION.observe(self.started.elapsed().as_secs_f64());
        IN_FLIGHT.dec();
    }
}

/// Caps concurrent requests and records metrics.
async fn guarded<F>(
    endpoints: &ConversionEndpoints,
    endpoint: &'static str,
    handle: F,
) -> Response
where
    F: std::future::Future<Output = Result<Response, Failure>>,
{
    IN_FLIGHT.inc();
    let mut measured = Measured {
        endpoint,
        started: Instant::now(),
        status: None,
    };
    let result = async {
        let _permit = endpoints.permits.try_acquire().map_err(|_| Failure::unavailable())?;
        handle.await
    }
    .await;

    let response = result.unwrap_or_else(IntoResponse::into_response);
    measured.status = Some(response.status());
    response
}

async fn script(
    State(endpoints): Shared,
    ConnectInfo(peer): Peer,
    Path(file): Path<String>,
    headers: HeaderMap,
) -> Response {
    guarded(&endpoints, "script", async {
        let address = endpoints.address(peer, &headers);
        endpoints.addresses.take(address).map_err(Failure::limited)?;
        let not_found = || Failure::new(StatusCode::NOT_FOUND, "not_found");
        let key = file.strip_suffix(".js").ok_or_else(not_found)?;
        let conversion = find(&endpoints, key)
            .await?
            .filter(|c| c.active)
            .ok_or_else(not_found)?;
        // Keys and numbers only, so this can't end a script element.
        let config = json!({
            "key": conversion.public_key,
            "windowDays": conversion.window_days,
        });
        Ok((
            [
                (CONTENT_TYPE, "text/javascript; charset=utf-8"),
                (CACHE_CONTROL, SCRIPT_CACHE),
                (X_CONTENT_TYPE_OPTIONS, "nosniff"),
                (ACCESS_CONTROL_ALLOW_ORIGIN, "*"),
            ],
            SCRIPT.replacen("__CONFIG__", &config.to_string(), 1),
        )
            .into_response())
    })
    .await
}

async fn browser_event(
    State(endpoints): Shared,
    ConnectInfo(peer): Peer,
    request: Request,
) -> Response {
    let (parts, body) = request.into_parts();
    let mut response = guarded(&endpoints, "events", async {
        let address = endpoints.address(peer, &parts.headers);
        endpoints.addresses.take(address).map_err(Failure::limited)?;
        let body = read_body(body).await?;
        let key = match body.get("key") {
            Some(Value::String(key)) => key,
            _ => return Err(Failure::invalid("key")),
        };
        let conversion = find(&endpoints, key).await?.ok_or(Failure::invalid("key"))?;
        endpoints
            .browser_keys
            .take(conversion.public_key.clone())
            .map_err(Failure::limited)?;
        let origin = parts.headers.get(ORIGIN).and_then(|o| o.to_str().ok());
        accept(&endpoints, &conversion, Kind::Event, Source::Browser, origin, &body).await
    })
    .await;

    // Any site may read the answer. Which sites may send is up to the conversion.
    let headers = response.headers_mut();
    headers.insert(ACCESS_CONTROL_ALLOW_ORIGIN, HeaderValue::from_static("*"));
    headers.insert(ACCESS_CONTROL_EXPOSE_HEADERS, HeaderValue::from_static("retry-after"));
    response
}

async fn server_event(
    State(endpoints): Shared,
    ConnectInfo(peer): Peer,
    request: Request,
) -> Response {
    server_request(&endpoints, "conversions", Kind::Event, peer, request).await
}

async fn refund(State(endpoints): Shared, ConnectInfo(peer): Peer, request: Request) -> Response {
    server_request(&endpoints, "refunds", Kind::Refund, peer, request).await
}

async fn server_request(
    endpoints: &ConversionEndpoints,
    endpoint: &'static str,
    kind: Kind,
    peer: SocketAddr,
    request: Request,
) -> Response {
    let (parts, body) = request.into_parts();
    guarded(endpoints, endpoint, async {
        let address = endpoints.address(peer, &parts.headers);
        endpoints.failed_secrets.check(&address).map_err(Failure::limited)?;
        let conversion = match authenticate(endpoints, &parts.headers).await {
            Err(e) if e.status == StatusCode::UNAUTHORIZED => {
                let _ = endpoints.failed_secrets.take(address);
                return Err(e);
            }
            result => result?,
        };
        endpoints
            .server_keys
            .take(conversion.public_key.clone())
            .map_err(Failure::limited)?;
        let body = read_body(body).await?;
        accept(endpoints, &conversion, kind, Source::Server, None, &body).await
    })
    .await
}

async fn find(
    endpoints: &ConversionEndpoints,
    public_key: &str,
) -> Result<Option<Arc<Conversion>>, Failure> {
    endpoints.definitions.get(public_key).await.map_err(|e| {
        tracing::error!(error = %e, "Conversion lookup failed");
        Failure::unavailable()
    })
}

/// Secret keys are `sk_{id}_{random}`, where `pk_{id}` is the public key.
async fn authenticate(
    endpoints: &ConversionEndpoints,
    headers: &HeaderMap,
) -> Result<Arc<Conversion>, Failure> {
    let secret = headers
        .get(AUTHORIZATION)
        .and_then(|v| v.to_str().ok())
        .and_then(|v| v.split_once(' '))
        .filter(|(scheme, _)| scheme.eq_ignore_ascii_case("bearer"))
        .map(|(_, secret)| secret.trim())
        .ok_or_else(Failure::unauthorized)?;
    let id = secret
        .strip_prefix("sk_")
        .and_then(|s| s.split_once('_'))
        .map(|(id, _)| id)
        .ok_or_else(Failure::unauthorized)?;
    let conversion = find(endpoints, &format!("pk_{id}"))
        .await?
        .ok_or_else(Failure::unauthorized)?;

    let hash = format!("{:x}", Sha256::digest(secret.as_bytes()));
    if bool::from(hash.as_bytes().ct_eq(conversion.secret_hash.as_bytes())) {
        Ok(conversion)
    } else {
        Err(Failure::unauthorized())
    }
}

async fn read_body(body: Body) -> Result<Map<String, Value>, Failure> {
    let bytes = timeout(BODY_TIMEOUT, axum::body::to_bytes(body, MAX_BODY_BYTES))
        .await
        .map_err(|_| Failure::new(StatusCode::REQUEST_TIMEOUT, "timeout"))?
        .map_err(|_| Failure::new(StatusCode::PAYLOAD_TOO_LARGE, "too_large"))?;
    match serde_json::from_slice(&bytes) {
        Ok(Value::Object(body)) => Ok(body),
        _ => Err(Failure::invalid("body")),
    }
}

async fn accept(
    endpoints: &ConversionEndpoints,
    conversion: &Conversion,
    kind: Kind,
    source: Source,
    origin: Option<&str>,
    body: &Map<String, Value>,
) -> Result<Response, Failure> {
    let (record, rejection) = read(conversion, kind, source, origin, body, Utc::now());
    // Deleted conversions have no workspace whose log could show it.
    if let Some(rejection) = rejection.as_ref().filter(|_| conversion.workspace_id.is_empty()) {
        return Err(rejection.into());
    }
    let published = endpoints.publisher.publish(&record).await;
    if let Err(e) = &published {
        let outcome = if rejection.is_some() { "rejected" } else { "accepted" };
        PUBLISH_FAILURES.with_label_values(&[outcome]).inc();
        tracing::error!(error = %e, conversion = %record.conversion_id, "Event not published");
    }

    // A rejection stands even if it could not be logged.
    if let Some(rejection) = &rejection {
        return Err(rejection.into());
    }
    published.map_err(|_| Failure::unavailable())?;
    Ok((StatusCode::ACCEPTED, Json(json!({ "eventId": record.event_id }))).into_response())
}

#[cfg(test)]
#[path = "conversion_endpoints_tests.rs"]
mod tests;
