use actix_web::{get, web, HttpResponse};

use crate::adapters::api::{
    routes::routes_mappers::RoutePresenterMapper,
    shared::{ 
        app_state::AppState, 
        error_presenter::ErrorResponse 
    }, 
};

use crate::application::{
    mappers::api_mapper::ApiMapper,
    usecases::{ 
        interfaces::AbstractUseCase, 
        routes::get_route_by_switch_and_link::GetRouteBySwitchAndLink 
    }
};

pub fn routes(cfg: &mut web::ServiceConfig){
    cfg.service(get_route);
}

#[get("/{switch}/{path}")]
async fn get_route(data: web::Data<AppState>, param: web::Path<(String, String)>) -> Result<HttpResponse, ErrorResponse>
{
    let link = param.into_inner();
    let get_route_by_switch_and_link = GetRouteBySwitchAndLink::new(
        link.0, link.1, &data.routes_repository);

    
    let route = get_route_by_switch_and_link.execute().await;

    route
        .map_err(ErrorResponse::map_io_error)
        .map(|route| HttpResponse::Ok().json(RoutePresenterMapper::to_api(route)))
}