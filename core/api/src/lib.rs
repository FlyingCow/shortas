use actix_web::dev::Server;
use anyhow::Result;
use std::net::TcpListener;

pub mod domain;
pub mod adapters;
pub mod application;
pub mod infrastructure;

extern crate dotenv;
extern crate log;

pub async fn run(listener: TcpListener) -> Result<Server>{
    infrastructure::server(listener).await
}