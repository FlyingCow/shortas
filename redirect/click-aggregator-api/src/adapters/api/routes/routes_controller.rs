use actix_web::{get, web, HttpResponse, Responder};

use crate::adapters::api::{app_state::AppState, error_presenter::ErrorReponse};

pub fn api_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(get_route).service(get_main_route);
}

#[get("/{domain}/{path}/{switch}")]
async fn get_route(
    data: web::Data<AppState>,
    path: web::Path<(String, String, String)>,
) -> impl Responder {
    let path = path.into_inner();

    let route_domain = path.0;
    let route_path = path.1;
    let route_switch = path.2;

    let route = data
        .routes_store
        .get_route(
            route_switch.as_str(),
            route_domain.as_str(),
            route_path.as_str(),
        )
        .await;

    let result = route.map_err(ErrorReponse::map_error).map(|route| {
        if let Some(route) = route {
            return HttpResponse::Ok().json(route);
        }

        return HttpResponse::NotFound().json(());
    });

    result
}

#[get("/{domain}/{path}")]
async fn get_main_route(
    data: web::Data<AppState>,
    path: web::Path<(String, String)>,
) -> impl Responder {
    let path = path.into_inner();

    let route_domain = path.0;
    let route_path = path.1;

    let route = data
        .routes_store
        .get_route("main", route_domain.as_str(), route_path.as_str())
        .await;

    let result = route.map_err(ErrorReponse::map_error).map(|route| {
        if let Some(route) = route {
            return HttpResponse::Ok().json(route);
        }

        return HttpResponse::NotFound().json(());
    });

    result
}
