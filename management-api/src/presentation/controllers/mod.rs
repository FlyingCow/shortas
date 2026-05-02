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

use salvo::prelude::*;

use super::middleware::{JwtAuth, RateLimiter, SecurityHeaders};
use crate::settings::JwtSettings;

/// Build all API routes.
pub fn api_routes(jwt_settings: JwtSettings) -> Router {
    let jwt_auth = JwtAuth::new(jwt_settings);
    let rate_limiter = RateLimiter::new();

    // Public routes (no auth required)
    let public_routes = Router::new()
        .push(Router::with_path("health").get(health::health_check))
        .push(Router::with_path("health/ready").get(health::readiness_check))
        .push(Router::with_path("health/live").get(health::liveness_check));

    // Protected routes (auth required)
    let protected_routes = Router::with_path("v1")
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
