//! Error handling middleware and response utilities.

use salvo::prelude::*;
use serde::Serialize;
use tracing::error;

use crate::domain::entities::ApiError;

/// Standard error response.
#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub code: String,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<String>,
}

impl From<ApiError> for ErrorResponse {
    fn from(error: ApiError) -> Self {
        Self {
            code: error.code.as_str().to_string(),
            message: error.message,
            details: error.details,
        }
    }
}

/// Render an API error as JSON response.
pub fn render_error(res: &mut Response, error: ApiError) {
    let status_code = StatusCode::from_u16(error.status_code())
        .unwrap_or(StatusCode::INTERNAL_SERVER_ERROR);

    // Log all errors server-side
    match error.code {
        crate::domain::entities::ErrorCode::InternalError => {
            error!(
                code = %error.code.as_str(),
                message = %error.message,
                details = ?error.details,
                "Internal server error"
            );
        }
        _ => {
            tracing::warn!(
                code = %error.code.as_str(),
                message = %error.message,
                "API error"
            );
        }
    }

    res.status_code(status_code);
    res.render(Json(ErrorResponse::from(error)));
}

/// Render a success response.
pub fn render_success<T: Serialize + Send>(res: &mut Response, data: T) {
    res.status_code(StatusCode::OK);
    res.render(Json(data));
}

/// Render a created response (201).
pub fn render_created<T: Serialize + Send>(res: &mut Response, data: T) {
    res.status_code(StatusCode::CREATED);
    res.render(Json(data));
}

/// Render a no content response (204).
pub fn render_no_content(res: &mut Response) {
    res.status_code(StatusCode::NO_CONTENT);
}

/// Security headers middleware.
pub struct SecurityHeaders;

#[async_trait]
impl Handler for SecurityHeaders {
    async fn handle(
        &self,
        req: &mut Request,
        depot: &mut Depot,
        res: &mut Response,
        ctrl: &mut FlowCtrl,
    ) {
        ctrl.call_next(req, depot, res).await;

        // Add CORS headers
        res.headers_mut().insert(
            "Access-Control-Allow-Origin",
            "*".parse().unwrap(),
        );
        res.headers_mut().insert(
            "Access-Control-Allow-Methods",
            "GET, POST, PUT, DELETE, PATCH, OPTIONS, HEAD".parse().unwrap(),
        );
        res.headers_mut().insert(
            "Access-Control-Allow-Headers",
            "Authorization, Content-Type, Accept, Origin, X-Requested-With".parse().unwrap(),
        );

        // Add security headers
        res.headers_mut().insert(
            "X-Content-Type-Options",
            "nosniff".parse().unwrap(),
        );
        res.headers_mut().insert(
            "X-Frame-Options",
            "DENY".parse().unwrap(),
        );
        res.headers_mut().insert(
            "X-XSS-Protection",
            "1; mode=block".parse().unwrap(),
        );
        res.headers_mut().insert(
            "Referrer-Policy",
            "strict-origin-when-cross-origin".parse().unwrap(),
        );
    }
}
