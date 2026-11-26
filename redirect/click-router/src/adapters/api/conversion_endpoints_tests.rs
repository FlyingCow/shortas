use std::sync::{
    atomic::{AtomicBool, Ordering},
    Mutex,
};

use tower::ServiceExt;

use super::*;
use crate::{
    adapters::moka::settings::ConversionCacheSettings,
    core::{
        conversion_definitions::ConversionStore,
        conversion_events::{tests::conversion, ConversionRecord},
        rate_limit::Rate,
    },
};

const SECRET: &str = "sk_abc_secret";

#[derive(Default)]
struct FakeStream {
    records: Mutex<Vec<ConversionRecord>>,
    down: AtomicBool,
    slow: AtomicBool,
}

#[async_trait::async_trait()]
impl ConversionPublisher for FakeStream {
    async fn publish(&self, record: &ConversionRecord) -> anyhow::Result<()> {
        if self.slow.load(Ordering::SeqCst) {
            tokio::time::sleep(Duration::from_millis(300)).await;
        }
        if self.down.load(Ordering::SeqCst) {
            anyhow::bail!("down");
        }
        self.records.lock().unwrap().push(record.clone());
        Ok(())
    }
}

struct FakeStore {
    conversions: Vec<Conversion>,
    down: bool,
}

#[async_trait::async_trait()]
impl ConversionStore for FakeStore {
    async fn get_conversion(&self, key: &str) -> anyhow::Result<Option<Conversion>> {
        anyhow::ensure!(!self.down, "down");
        Ok(self.conversions.iter().find(|c| c.public_key == key).cloned())
    }
}

struct App {
    router: Router,
    stream: Arc<FakeStream>,
}

impl App {
    fn new() -> Self {
        Self::with(|_| {}, |_| {}, false)
    }

    fn with(
        change: impl FnOnce(&mut Conversion),
        configure: impl FnOnce(&mut Conversions),
        store_down: bool,
    ) -> Self {
        let mut active = conversion();
        active.secret_hash = format!("{:x}", Sha256::digest(SECRET));
        change(&mut active);
        let mut other = conversion();
        other.public_key = "pk_other".into();
        other.secret_hash = format!("{:x}", Sha256::digest("sk_other_secret"));

        let mut settings = Conversions::default();
        configure(&mut settings);
        let stream = Arc::new(FakeStream::default());
        let definitions = MokaConversionCache::new(
            Arc::new(FakeStore { conversions: vec![active, other], down: store_down }),
            &ConversionCacheSettings::default(),
        );
        let router = ConversionEndpoints::new(&settings, definitions, stream.clone())
            .unwrap()
            .router()
            .route("/*rest", get(|| async { "link" }));
        Self { router, stream }
    }

    async fn send(
        &self,
        method: &str,
        path: &str,
        headers: &[(&str, &str)],
        body: impl Into<Body>,
    ) -> (StatusCode, HeaderMap, String) {
        let mut request = Request::builder().method(method).uri(path);
        for (name, value) in headers {
            request = request.header(*name, *value);
        }
        let mut request = request.body(body.into()).unwrap();
        let peer: SocketAddr = "203.0.113.5:999".parse().unwrap();
        request.extensions_mut().insert(ConnectInfo(peer));
        let response = self.router.clone().oneshot(request).await.unwrap();
        let (parts, body) = response.into_parts();
        let body = axum::body::to_bytes(body, usize::MAX).await.unwrap();
        (parts.status, parts.headers, String::from_utf8(body.to_vec()).unwrap())
    }

    async fn browser(&self, body: Value) -> (StatusCode, HeaderMap, String) {
        let headers = [("content-type", "text/plain"), ("origin", "https://shop.example")];
        self.send("POST", "/.well-known/shortas/v1/events", &headers, body.to_string()).await
    }

    async fn server(&self, path: &str, secret: &str, body: Value) -> (StatusCode, HeaderMap, String) {
        let path = format!("/.well-known/shortas/v1/{path}");
        let authorization = format!("Bearer {secret}");
        self.send("POST", &path, &[("authorization", &authorization)], body.to_string()).await
    }

    fn records(&self) -> Vec<ConversionRecord> {
        self.stream.records.lock().unwrap().clone()
    }
}

fn cart() -> Value {
    json!({"key": "pk_abc", "step": "cart"})
}

#[tokio::test]
async fn only_the_reserved_path_is_taken() {
    let app = App::new();
    for path in ["/conversions/track", "/v1/events", "/.well-known/other"] {
        for method in ["GET", "HEAD"] {
            assert_eq!(app.send(method, path, &[], "").await.0, StatusCode::OK, "{path}");
        }
    }
    for path in ["/", "/x", "/c", "/c/a/b", "/v1", "/v2/events"] {
        let path = format!("{PREFIX}{path}");
        assert_eq!(app.send("GET", &path, &[], "").await.0, StatusCode::NOT_FOUND, "{path}");
    }
}

#[tokio::test]
async fn script_is_cacheable_and_configured() {
    let app = App::new();
    let path = "/.well-known/shortas/c/pk_abc.js";
    let (status, headers, body) = app.send("GET", path, &[], "").await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(headers[CONTENT_TYPE], "text/javascript; charset=utf-8");
    assert_eq!(headers[CACHE_CONTROL], SCRIPT_CACHE);
    assert!(body.contains(r#"{"key":"pk_abc","windowDays":30}"#));
    assert!(!body.contains("__CONFIG__"));
    assert!(!body.contains(&format!("{:x}", Sha256::digest(SECRET))));
}

#[tokio::test]
async fn unknown_or_inactive_script_is_not_found() {
    let app = App::with(|c| c.active = false, |_| {}, false);
    for file in ["pk_abc.js", "pk_nope.js", "pk_other", ".js"] {
        let path = format!("{PREFIX}/c/{file}");
        let (status, headers, _) = app.send("GET", &path, &[], "").await;
        assert_eq!(status, StatusCode::NOT_FOUND, "{file}");
        assert_eq!(headers[CACHE_CONTROL], "no-store");
    }
}

#[tokio::test]
async fn browser_event_is_published() {
    let app = App::new();
    let (status, headers, body) =
        app.browser(json!({"key": "pk_abc", "step": "purchase", "eventId": "e1"})).await;
    assert_eq!((status, body.as_str()), (StatusCode::ACCEPTED, r#"{"eventId":"e1"}"#));
    assert_eq!(headers[ACCESS_CONTROL_ALLOW_ORIGIN], "*");
    assert!(!headers.contains_key("access-control-allow-credentials"));
    let records = app.records();
    assert_eq!(records.len(), 1);
    assert_eq!((records[0].source, records[0].rejection), (Source::Browser, None));
}

#[tokio::test]
async fn refused_browser_events_are_logged_with_the_reason() {
    let app = App::with(|c| c.allowed_origins = vec!["https://a.example".into()], |_| {}, false);
    let (status, headers, _) = app.browser(cart()).await;
    assert_eq!(status, StatusCode::FORBIDDEN);
    assert_eq!(headers[ACCESS_CONTROL_ALLOW_ORIGIN], "*");
    assert_eq!(app.records()[0].rejection, Some("origin_not_allowed"));

    let app = App::with(|c| c.active = false, |_| {}, false);
    assert_eq!(app.browser(cart()).await.0, StatusCode::CONFLICT);
    assert_eq!(app.records()[0].rejection, Some("not_active"));

    let app = App::new();
    let (status, _, body) = app.browser(json!({"key": "pk_abc", "step": "nope"})).await;
    assert_eq!((status, body.as_str()), (StatusCode::BAD_REQUEST, r#"{"error":"invalid","field":"step"}"#));
    assert_eq!(app.records()[0].rejected_field, Some("step"));
}

#[tokio::test]
async fn unknown_keys_and_deleted_conversions_store_nothing() {
    let app = App::new();
    for body in [json!({"step": "cart"}), json!({"key": "pk_nope", "step": "cart"}), json!({"key": 1})] {
        let (status, _, body) = app.browser(body).await;
        assert_eq!((status, body.as_str()), (StatusCode::BAD_REQUEST, r#"{"error":"invalid","field":"key"}"#));
    }
    assert!(app.records().is_empty());

    let app = App::with(|c| { c.active = false; c.workspace_id.clear(); c.steps.clear() }, |_| {}, false);
    assert_eq!(app.browser(cart()).await.0, StatusCode::CONFLICT);
    assert!(app.records().is_empty());
}

#[tokio::test]
async fn server_event_and_refund() {
    let app = App::new();
    let (status, headers, body) =
        app.server("conversions", SECRET, json!({"step": "cart", "clickId": "C"})).await;
    assert_eq!(status, StatusCode::ACCEPTED);
    assert!(!headers.contains_key(ACCESS_CONTROL_ALLOW_ORIGIN));
    let records = app.records();
    assert_eq!(body, json!({"eventId": records[0].event_id}).to_string());
    assert_eq!(records[0].source, Source::Server);

    let refund = json!({"refundId": "f", "externalId": "o", "value": 5});
    assert_eq!(app.server("refunds", SECRET, refund.clone()).await.0, StatusCode::ACCEPTED);
    assert_eq!(app.records()[1].kind, Kind::Refund);
    // Not from browsers
    let mut refund = refund;
    refund["key"] = "pk_abc".into();
    assert_eq!(app.browser(refund).await.0, StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn bad_keys_are_unauthorized_and_store_nothing() {
    let app = App::new();
    for secret in ["", "pk_abc", "sk_abc", "sk_abc_", "sk_abc_Secret", "sk_nope_secret", "sk_other_x",
                   "sk_abc_secret_x"] {
        assert_eq!(app.server("conversions", secret, cart()).await.0, StatusCode::UNAUTHORIZED, "{secret}");
        assert_eq!(app.server("refunds", secret, cart()).await.0, StatusCode::UNAUTHORIZED);
    }
    let path = "/.well-known/shortas/v1/conversions";
    for headers in [&[][..], &[("authorization", "Basic sk_abc_secret")], &[("authorization", "sk_abc_secret")]] {
        assert_eq!(app.send("POST", path, headers, cart().to_string()).await.0, StatusCode::UNAUTHORIZED);
    }
    assert!(app.records().is_empty());
}

#[tokio::test]
async fn unavailable_stream_or_store_is_503() {
    let app = App::new();
    app.stream.down.store(true, Ordering::SeqCst);
    let (status, headers, _) = app.browser(cart()).await;
    assert_eq!(status, StatusCode::SERVICE_UNAVAILABLE);
    assert!(headers.contains_key(RETRY_AFTER));
    assert_eq!(app.server("conversions", SECRET, cart()).await.0, StatusCode::SERVICE_UNAVAILABLE);
    // Still refused for what is wrong with it
    assert_eq!(app.browser(json!({"key": "pk_abc"})).await.0, StatusCode::BAD_REQUEST);

    app.stream.down.store(false, Ordering::SeqCst);
    assert_eq!(app.browser(cart()).await.0, StatusCode::ACCEPTED);

    let app = App::with(|_| {}, |_| {}, true);
    assert_eq!(app.browser(cart()).await.0, StatusCode::SERVICE_UNAVAILABLE);
    assert_eq!(app.server("conversions", SECRET, cart()).await.0, StatusCode::SERVICE_UNAVAILABLE);
}

#[tokio::test]
async fn bodies_are_limited() {
    let app = App::new();
    let mut body = cart();
    let padding = MAX_BODY_BYTES - body.to_string().len() - r#","p":"""#.len();
    body["p"] = "x".repeat(padding).into();
    assert_eq!(body.to_string().len(), MAX_BODY_BYTES);
    assert_eq!(app.browser(body.clone()).await.0, StatusCode::ACCEPTED);
    body["p"] = "x".repeat(padding + 1).into();
    assert_eq!(app.browser(body.clone()).await.0, StatusCode::PAYLOAD_TOO_LARGE);
    assert_eq!(app.server("conversions", SECRET, body).await.0, StatusCode::PAYLOAD_TOO_LARGE);
    for body in ["", "{", "[]", "\"x\""] {
        let path = "/.well-known/shortas/v1/events";
        assert_eq!(app.send("POST", path, &[], body).await.0, StatusCode::BAD_REQUEST);
    }
}

#[tokio::test]
async fn keys_are_limited_separately_for_browsers_and_servers() {
    let app = App::with(|_| {}, |s| s.key_rate = Rate { per_second: 0.5, burst: 2.0 }, false);
    for _ in 0..2 {
        assert_eq!(app.browser(cart()).await.0, StatusCode::ACCEPTED);
    }
    let (status, headers, _) = app.browser(cart()).await;
    assert_eq!(status, StatusCode::TOO_MANY_REQUESTS);
    assert_eq!(headers[RETRY_AFTER], "2");
    assert_eq!(headers[ACCESS_CONTROL_ALLOW_ORIGIN], "*");
    assert_eq!(app.records().len(), 2);

    assert_eq!(app.browser(json!({"key": "pk_other", "step": "cart"})).await.0, StatusCode::ACCEPTED);
    // Wrong secrets don't use up the limit
    for _ in 0..3 {
        app.server("conversions", "sk_abc_wrong", cart()).await;
    }
    assert_eq!(app.server("conversions", SECRET, cart()).await.0, StatusCode::ACCEPTED);
    assert_eq!(app.send("GET", "/abc", &[], "").await.2, "link");
}

#[tokio::test]
async fn addresses_are_limited_whatever_they_claim() {
    let limit = |s: &mut Conversions| s.address_rate = Rate { per_second: 0.1, burst: 1.0 };
    let path = "/.well-known/shortas/v1/events";
    let forwarded = |ip: &'static str| [("x-forwarded-for", ip)];
    let body = || cart().to_string();

    let app = App::with(|_| {}, limit, false);
    assert_eq!(app.send("POST", path, &forwarded("9.9.9.1"), body()).await.0, StatusCode::ACCEPTED);
    let status = app.send("POST", path, &forwarded("9.9.9.2"), body()).await.0;
    assert_eq!(status, StatusCode::TOO_MANY_REQUESTS);

    let app = App::with(
        |_| {},
        |s| {
            limit(s);
            s.trusted_proxies = vec!["203.0.113.0/24".into()];
        },
        false,
    );
    for ip in ["9.9.9.1", "9.9.9.2"] {
        assert_eq!(app.send("POST", path, &forwarded(ip), body()).await.0, StatusCode::ACCEPTED);
    }
    let status = app.send("POST", path, &forwarded("6.6.6.6, 9.9.9.2"), body()).await.0;
    assert_eq!(status, StatusCode::TOO_MANY_REQUESTS);
}

#[tokio::test]
async fn verified_servers_are_not_limited_by_address() {
    let app = App::with(|_| {}, |s| s.address_rate = Rate { per_second: 0.1, burst: 2.0 }, false);
    for _ in 0..10 {
        assert_eq!(app.server("conversions", SECRET, cart()).await.0, StatusCode::ACCEPTED);
    }
    // They don't use up the limit of wrong secrets, which still applies
    for _ in 0..2 {
        let status = app.server("conversions", "sk_abc_wrong", cart()).await.0;
        assert_eq!(status, StatusCode::UNAUTHORIZED);
    }
    let (status, headers, _) = app.server("conversions", "sk_abc_wrong", cart()).await;
    assert_eq!(status, StatusCode::TOO_MANY_REQUESTS);
    assert!(headers.contains_key(RETRY_AFTER));
    // Browsers have their own
    assert_eq!(app.browser(cart()).await.0, StatusCode::ACCEPTED);
}

#[tokio::test]
async fn concurrent_requests_are_capped() {
    let app = App::with(|_| {}, |s| s.max_concurrent_requests = 1, false);
    app.stream.slow.store(true, Ordering::SeqCst);
    let (first, second) = tokio::join!(app.browser(cart()), app.browser(cart()));
    assert_eq!(first.0, StatusCode::ACCEPTED);
    assert_eq!(second.0, StatusCode::SERVICE_UNAVAILABLE);
    assert_eq!(app.browser(cart()).await.0, StatusCode::ACCEPTED);
}

#[tokio::test]
async fn requests_are_counted() {
    let count = || REQUESTS.with_label_values(&["refunds", "401"]).get();
    let before = count();
    App::new().server("refunds", "sk_nope_x", cart()).await;
    assert!(count() > before);
}
