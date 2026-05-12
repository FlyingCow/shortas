//! API controllers for all endpoints.

pub mod certificates;
pub mod clickstream;
pub mod domains;
pub mod health;
pub mod routes;
pub mod user;
pub mod workspaces;

pub use certificates::*;
pub use clickstream::*;
pub use domains::*;
pub use health::*;
pub use routes::*;
pub use user::*;
pub use workspaces::*;

use salvo::http::header::{self, HeaderValue};
use salvo::http::StatusCode;
use salvo::prelude::*;

use super::middleware::{JwtAuth, RateLimiter, SecurityHeaders};
use crate::settings::JwtSettings;

/// CORS preflight handler for OPTIONS requests.
#[handler]
pub async fn cors_preflight(res: &mut Response) {
    res.headers_mut().insert(header::ACCESS_CONTROL_ALLOW_ORIGIN, HeaderValue::from_static("*"));
    res.headers_mut().insert(header::ACCESS_CONTROL_ALLOW_METHODS, HeaderValue::from_static("GET, POST, PUT, DELETE, PATCH, OPTIONS, HEAD"));
    res.headers_mut().insert(header::ACCESS_CONTROL_ALLOW_HEADERS, HeaderValue::from_static("Authorization, Content-Type, Accept, Origin, X-Requested-With"));
    res.headers_mut().insert(header::ACCESS_CONTROL_MAX_AGE, HeaderValue::from_static("86400"));
    res.status_code(StatusCode::NO_CONTENT);
}

/// Add OPTIONS handler to a router for all paths.
fn with_options(router: Router) -> Router {
    router.options(cors_preflight)
}

/// Build all API routes.
pub fn api_routes(jwt_settings: JwtSettings) -> Router {
    let jwt_auth = JwtAuth::new(jwt_settings);
    let rate_limiter = RateLimiter::new();

    // Public routes (no auth required)
    let public_routes = Router::new()
        .push(with_options(Router::with_path("health").get(health::health_check)))
        .push(with_options(Router::with_path("health/ready").get(health::readiness_check)))
        .push(with_options(Router::with_path("health/live").get(health::liveness_check)))
        // Public API routes (no auth required) - matches C# [AllowAnonymous]
        .push(with_options(Router::with_path("api/v1/domains/shared").get(domains::list_shared_domains)));

    // Protected routes (auth required) - mounted at /api/v1
    let protected_routes = Router::with_path("api/v1")
        .hoop(jwt_auth)
        .hoop(rate_limiter)
        .push(routes::routes_controller())
        .push(domains::domains_controller())
        .push(workspaces::workspaces_controller())
        .push(certificates::certificates_controller())
        .push(clickstream::clickstream_controller())
        .push(user::user_controller());

    Router::new()
        .hoop(SecurityHeaders)
        .push(public_routes)
        .push(protected_routes)
}
