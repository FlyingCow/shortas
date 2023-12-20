use anyhow::Result;
use aws_config::SdkConfig;
use aws_sdk_dynamodb::Client;
use std::{env, net::TcpListener};

use crate::adapters::{
    api::shared::{app_state::AppState, server_routes::app_routes},
    spi::dynamo::dynamo_routes_repository::{ RoutesRepository, RoutesRepositoryConfig }
};

use actix_web::{dev::Server, middleware::Logger};
use actix_web::{web, App, HttpServer};

pub async fn server(listener: TcpListener) -> Result<Server> {

    env::set_var("RUST_BACKTRACE", "1");
    env::set_var("RUST_LOG", "actix_web=debug");

    env_logger::try_init()?;


    let shared_config = load_aws_config().await;
    
    let app_state = AppState{
        app_name: "Router API".to_string(),
        routes_repository: RoutesRepository::new(
            RoutesRepositoryConfig::new(
                Client::new(
                    &shared_config),
                    std::env::var("ROUTES_TABLE").unwrap_or_default()
            ))
    };

    let data = web::Data::new(app_state);

    let port = listener.local_addr().unwrap().port();

    let server = HttpServer::new(move || 
        App::new().app_data(data.clone())
            .wrap(Logger::default())
            .configure(app_routes))
        .listen(listener)?
        .run();

    print!("Server running on port {}", port);

    Ok(server)
}

//https://docs.aws.amazon.com/sdk-for-rust/latest/dg/localstack.html
fn use_localstack() -> bool {
    std::env::var("LOCALSTACK").unwrap_or_default() == "true"
}

async fn load_aws_config() -> SdkConfig {
    let mut shared_config = aws_config::defaults(aws_config::BehaviorVersion::latest());

    if use_localstack(){
        shared_config = shared_config.endpoint_url(std::env::var("LOCALSTACK_ENDPOINT").unwrap());
    }

    shared_config.load().await
}