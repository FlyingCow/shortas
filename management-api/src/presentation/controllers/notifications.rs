//! Notification center controller.

use salvo::prelude::*;
use uuid::Uuid;

use crate::application::dto::NotificationDto;
use crate::domain::entities::ApiError;
use crate::presentation::middleware::{render_error, render_no_content, render_success, DepotExt, UserExt};

use super::cors_preflight;

/// Build notifications controller router.
pub fn notifications_controller() -> Router {
    Router::with_path("notifications")
        .get(list_notifications)
        .options(cors_preflight)
        .push(
            Router::with_path("{id}/read")
                .post(mark_notification_read)
                .options(cors_preflight),
        )
}

/// List the current user's notifications, newest first.
#[endpoint(
    operation_id = "list_notifications",
    summary = "List notifications",
    tags("Notifications"),
    responses((status_code = 200, description = "Notifications", body = Vec<NotificationDto>))
)]
pub async fn list_notifications(depot: &mut Depot, res: &mut Response) {
    let app_state = match depot.app_state() {
        Ok(state) => state,
        Err(e) => {
            render_error(res, ApiError::internal(e.to_string()));
            return;
        }
    };

    let user_id = match depot.user_id() {
        Ok(id) => id,
        Err(e) => {
            render_error(res, ApiError::unauthorized(e.to_string()));
            return;
        }
    };

    match app_state.notification_repo.list_by_user(&user_id, 50).await {
        Ok(notifications) => {
            render_success(res, notifications.into_iter().map(NotificationDto::from_entity).collect::<Vec<_>>());
        }
        Err(e) => render_error(res, e),
    }
}

/// Mark a notification as read.
#[endpoint(
    operation_id = "mark_notification_read",
    summary = "Mark notification read",
    tags("Notifications"),
    parameters(("id" = String, Path, description = "Notification ID")),
    responses((status_code = 204, description = "Marked read"))
)]
pub async fn mark_notification_read(req: &mut Request, depot: &mut Depot, res: &mut Response) {
    let app_state = match depot.app_state() {
        Ok(state) => state,
        Err(e) => {
            render_error(res, ApiError::internal(e.to_string()));
            return;
        }
    };

    let user_id = match depot.user_id() {
        Ok(id) => id,
        Err(e) => {
            render_error(res, ApiError::unauthorized(e.to_string()));
            return;
        }
    };

    let id_str: String = req.param("id").unwrap_or_default();
    let id = match Uuid::parse_str(&id_str) {
        Ok(id) => id,
        Err(_) => {
            render_error(res, ApiError::validation("Invalid notification ID"));
            return;
        }
    };

    let notification = match app_state.notification_repo.get_by_id(id).await {
        Ok(Some(n)) => n,
        Ok(None) => {
            render_error(res, ApiError::not_found("Notification", &id_str));
            return;
        }
        Err(e) => {
            render_error(res, e);
            return;
        }
    };

    if notification.user_id != user_id {
        render_error(res, ApiError::forbidden());
        return;
    }

    match app_state.notification_repo.mark_read(id).await {
        Ok(()) => render_no_content(res),
        Err(e) => render_error(res, e),
    }
}
