use actix_web::{get, web, HttpResponse, Responder};

use crate::adapters::api::{app_state::AppState, error_presenter::ErrorReponse};

pub fn api_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(get_user_settings);
}

#[get("/{user_id}")]
async fn get_user_settings(
    data: web::Data<AppState>,
    path: web::Path<(String,)>,
) -> impl Responder {
    let path = path.into_inner();

    let user_id = path.0;

    let user_settings = data
        .user_settings_store
        .get_user_settings(
            user_id.as_str(),
        )
        .await;

    let result = user_settings.map_err(ErrorReponse::map_error).map(|user_settings| {
        if let Some(user_settings) = user_settings {
            return HttpResponse::Ok().json(user_settings);
        }

        return HttpResponse::NotFound().json(());
    });

    result
}

