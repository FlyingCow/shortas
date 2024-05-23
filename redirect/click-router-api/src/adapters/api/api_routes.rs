use actix_web::web;

use crate::adapters::api::routes::routes_controller;

use super::routes::{crypto_controller, user_settings_controller};

pub fn routes(config: &mut web::ServiceConfig) {
    config
        .service(web::scope("/v1/routes").configure(routes_controller::api_routes))
        .service(web::scope("/v1/certificates").configure(crypto_controller::api_routes))
        .service(web::scope("/v1/user-settings").configure(user_settings_controller::api_routes));
}