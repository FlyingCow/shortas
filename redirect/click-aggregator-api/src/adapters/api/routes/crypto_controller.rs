use actix_web::{get, web, HttpResponse, Responder};

use crate::adapters::api::{app_state::AppState, error_presenter::ErrorReponse};

pub fn api_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(get_certificate);
}

#[get("/{domain}")]
async fn get_certificate(data: web::Data<AppState>, path: web::Path<(String,)>) -> impl Responder {
    let path = path.into_inner();

    let domain = path.0;

    let certificate = data.crypto_store.get_certificate(domain.as_str()).await;

    let result = certificate
        .map_err(ErrorReponse::map_error)
        .map(|certificate| {
            if let Some(certificate) = certificate {
                return HttpResponse::Ok().json(certificate);
            }

            return HttpResponse::NotFound().json(());
        });

    result
}
