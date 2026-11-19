//! User directory controller: email search for the invite-by-email typeahead.

use salvo::prelude::*;

use crate::application::dto::UserSearchResultDto;
use crate::domain::entities::ApiError;
use crate::presentation::middleware::{render_error, render_success, DepotExt};

use super::cors_preflight;

/// Build users controller router.
pub fn users_controller() -> Router {
    Router::with_path("users")
        .push(Router::with_path("search").get(search_users).options(cors_preflight))
}

/// Search known users by email, for the workspace invite typeahead.
#[endpoint(
    operation_id = "search_users",
    summary = "Search users by email",
    description = "Search the local user directory by email prefix/substring (users who have signed in at least once).",
    tags("Users"),
    parameters(("q" = String, Query, description = "Email search query")),
    responses((status_code = 200, description = "Matching users", body = Vec<UserSearchResultDto>))
)]
pub async fn search_users(req: &mut Request, depot: &mut Depot, res: &mut Response) {
    let app_state = match depot.app_state() {
        Ok(state) => state,
        Err(e) => {
            render_error(res, ApiError::internal(e.to_string()));
            return;
        }
    };

    let query: String = req.query("q").unwrap_or_default();
    if query.trim().len() < 2 {
        render_success(res, Vec::<UserSearchResultDto>::new());
        return;
    }

    match app_state.user_repo.search_by_email(query.trim(), 10).await {
        Ok(users) => {
            render_success(res, users.into_iter().map(UserSearchResultDto::from_entity).collect::<Vec<_>>());
        }
        Err(e) => render_error(res, e),
    }
}
