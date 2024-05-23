use actix_web::web;

use crate::adapters::api::routes::routes_controller;

pub fn routes(config: &mut web::ServiceConfig) {
    config
        .service(web::scope("/v1/routes").configure(routes_controller::api_routes))
        //.service(web::scope("/v1/cats").configure(cat_facts_controllers::routes))
        ;
}