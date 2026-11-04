use salvo::oapi::endpoint;
use salvo::prelude::*;
use serde::Deserialize;

use crate::adapters::api::app_state::AppState;
use crate::adapters::rabbitmq::messages::{ChangeAction, RouteChangedMessage};
use crate::model::route::{Route, RouteProperties};

/// Switch used for per-domain custom index/404 page routes. click-router's
/// root/not-found modules (core/modules/{root,not_found}.rs) look these up
/// by this exact value — it's deliberately underscored, unlike
/// routes_controller.rs's "internal" guard constant, which blocks writes
/// through the general routes API precisely so this is the only way in.
const INTERNAL_SWITCH: &str = "_internal";

pub fn api_routes() -> Router {
    Router::with_path("/domains/{domain}/custom-pages")
        .put(set_custom_pages)
        .delete(clear_custom_pages)
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct SetCustomPagesRequest {
    #[serde(default)]
    index_url: Option<String>,
    #[serde(default)]
    not_found_url: Option<String>,
}

/// Set a domain's custom index/404 pages.
///
/// Writes the internal routes click-router's root/not-found modules look
/// up for a domain's custom index and 404 pages. A null (or omitted) field
/// clears that page back to the global default instead of leaving it
/// untouched — matching the dashboard's "leave empty to use the default
/// pages" behavior, which always sends both fields on every save.
#[endpoint(
    operation_id = "set_custom_pages",
    summary = "Set a domain's custom index/404 pages",
    parameters(
        ("domain" = String, Path, description = "Domain name")
    ),
    responses(
        (status_code = 200, description = "Custom pages updated"),
        (status_code = 400, description = "Invalid request body"),
        (status_code = 500, description = "Failed to write the route")
    )
)]
pub async fn set_custom_pages(req: &mut Request, depot: &mut Depot, res: &mut Response) {
    let domain = req.param::<String>("domain").unwrap_or_default();

    let body: SetCustomPagesRequest = match req.parse_json().await {
        Ok(b) => b,
        Err(e) => {
            res.status_code(StatusCode::BAD_REQUEST);
            res.render(Json(serde_json::json!({ "error": format!("Invalid JSON: {}", e) })));
            return;
        }
    };

    let app_state = depot.obtain::<std::sync::Arc<AppState>>().unwrap();

    if let Err(e) = apply_page(app_state, &domain, "index", body.index_url.as_deref()).await {
        res.status_code(StatusCode::INTERNAL_SERVER_ERROR);
        res.render(Json(serde_json::json!({ "error": e.to_string() })));
        return;
    }
    if let Err(e) =
        apply_page(app_state, &domain, "not-found", body.not_found_url.as_deref()).await
    {
        res.status_code(StatusCode::INTERNAL_SERVER_ERROR);
        res.render(Json(serde_json::json!({ "error": e.to_string() })));
        return;
    }

    res.render(Json(serde_json::json!({ "message": "Custom pages updated" })));
}

/// Clear a domain's custom index/404 pages back to the global defaults.
#[endpoint(
    operation_id = "clear_custom_pages",
    summary = "Clear a domain's custom index/404 pages",
    parameters(
        ("domain" = String, Path, description = "Domain name")
    ),
    responses(
        (status_code = 204, description = "Custom pages cleared"),
        (status_code = 500, description = "Failed to clear the route")
    )
)]
pub async fn clear_custom_pages(req: &mut Request, depot: &mut Depot, res: &mut Response) {
    let domain = req.param::<String>("domain").unwrap_or_default();
    let app_state = depot.obtain::<std::sync::Arc<AppState>>().unwrap();

    for path in ["index", "not-found"] {
        if let Err(e) = apply_page(app_state, &domain, path, None).await {
            res.status_code(StatusCode::INTERNAL_SERVER_ERROR);
            res.render(Json(serde_json::json!({ "error": e.to_string() })));
            return;
        }
    }

    res.status_code(StatusCode::NO_CONTENT);
}

/// Set or clear one internal custom-page route (link = "{domain}%2F{path}").
/// `store_route` is insert-only (plain `insert_one`) and `update_route`'s
/// `replace_one` has no upsert flag, so neither alone is safe to call
/// blindly on every save — check which one applies first, the same way the
/// general routes API's own update handler does before calling update_route.
async fn apply_page(
    app_state: &AppState,
    domain: &str,
    path: &str,
    url: Option<&str>,
) -> anyhow::Result<()> {
    let link = format!("{}%2F{}", domain, path);

    match url {
        Some(dest) => {
            let route = Route::new(
                INTERNAL_SWITCH.to_string(),
                link.clone(),
                Some(dest.to_string()),
                RouteProperties::default(),
            );

            let existing = app_state.routes_store.get_route(INTERNAL_SWITCH, &link).await?;
            if existing.is_some() {
                app_state.routes_store.update_route(&route).await?;
            } else {
                app_state.routes_store.store_route(&route).await?;
            }
            app_state.routes_store.invalidate_route(INTERNAL_SWITCH, &link).await?;

            if let Some(ref publisher) = app_state.rabbitmq_publisher {
                publisher
                    .publish_route_family_changed(&[route], ChangeAction::Updated)
                    .await;
            }
        }
        None => {
            app_state
                .routes_store
                .delete_route_by_switch_and_link(INTERNAL_SWITCH, &link)
                .await?;
            app_state.routes_store.invalidate_route(INTERNAL_SWITCH, &link).await?;

            // Without this, click-router's own routes_cache keeps serving the
            // deleted route until its TTL expires (up to an hour) - the exact
            // bug that made "remove the custom URL" not fall back to the
            // default page. publish_route_changed only reads switch/link off
            // the message to decide what to evict, so a synthetic route with
            // no dest is enough; it doesn't need to reflect what was deleted.
            if let Some(ref publisher) = app_state.rabbitmq_publisher {
                let placeholder = Route::new(
                    INTERNAL_SWITCH.to_string(),
                    link.clone(),
                    None,
                    RouteProperties::default(),
                );
                publisher
                    .publish_route_changed(&RouteChangedMessage::from_route(
                        &placeholder,
                        ChangeAction::Deleted,
                    ))
                    .await;
            }
        }
    }

    Ok(())
}
