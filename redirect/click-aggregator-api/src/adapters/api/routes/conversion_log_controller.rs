use salvo::prelude::*;
use tracing::error;

use crate::adapters::api::app_state::AppState;
use crate::adapters::clickhouse::conversion_log_store::ConversionLog;

const DEFAULT_LIMIT: u32 = 50;
const MAX_LIMIT: u32 = 100;
const MAX_OFFSET: u32 = 10_000;

pub fn routes() -> Router {
    Router::with_path("workspaces/{workspace_id}/conversions/{conversion_id}/events")
        .get(get_conversion_events)
}

/// A conversion's events, newest first. Callers must have checked access to
/// the workspace.
#[endpoint(
    operation_id = "get_conversion_events",
    parameters(
        ("workspace_id" = String, Path, description = "Workspace ID"),
        ("conversion_id" = String, Path, description = "Conversion ID"),
        ("limit" = Option<u32>, Query, description = "Up to 100 (default: 50)"),
        ("offset" = Option<u32>, Query, description = "Up to 10000 (default: 0)")
    ),
    responses((status_code = 200, description = "Events", body = ConversionLog))
)]
pub async fn get_conversion_events(req: &mut Request, res: &mut Response, depot: &mut Depot) {
    let workspace_id: String = req.param("workspace_id").unwrap_or_default();
    let conversion_id: String = req.param("conversion_id").unwrap_or_default();
    let limit = req.query("limit").unwrap_or(DEFAULT_LIMIT).clamp(1, MAX_LIMIT);
    let offset = req.query("offset").unwrap_or(0).min(MAX_OFFSET);

    let result = match depot.obtain::<AppState>() {
        Ok(state) => {
            state
                .conversion_log_store
                .read(&workspace_id, &conversion_id, limit, offset)
                .await
        }
        Err(_) => Err(anyhow::anyhow!("no application state")),
    };
    match result {
        Ok(log) => res.render(Json(log)),
        Err(e) => {
            error!("Conversion log query failed: {e:#}");
            res.status_code(StatusCode::INTERNAL_SERVER_ERROR);
            res.render(Json(serde_json::json!({ "error": "Failed to read conversion events" })));
        }
    }
}
