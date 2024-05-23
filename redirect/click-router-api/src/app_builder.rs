use actix_web::{get, post, App, HttpResponse, HttpServer, Responder};
use anyhow::Result;

use crate::{core::BaseRoutesStore, settings::Settings};

#[derive(Clone)]
pub struct AppBuilder {
    pub(super) settings: Settings,
    pub(super) routes_store: Option<Box<dyn BaseRoutesStore + 'static>>,
}

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

pub struct Api {}
impl Api {
    fn new() -> Self {
        Api {}
    }

    pub async fn run(&self) -> Result<()> {
        HttpServer::new(|| {
            App::new().service(hello).service(echo)
            //.route("/hey", web::get().to(manual_hello))
        })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await?;

        Ok(())
    }
}

impl AppBuilder {
    pub fn new(settings: Settings) -> Self {
        Self {
            settings,
            routes_store: None,
        }
    }

    pub fn build(&self) -> Result<Api> {
        println!("{}", "BUILDING");

        let router = Api::new();

        Ok(router)
    }
}
