use actix_web::web;

use crate::adapters::api::routes::routes_controller;

pub fn app_routes(config: &mut web::ServiceConfig) {
    config
        .service(web::scope("api/v1/routes").configure(routes_controller::routes));
}

