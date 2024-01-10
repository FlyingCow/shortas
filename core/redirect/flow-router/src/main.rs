use flow_router::log::*;
use flow_router::Settings;
use flow_router::AppBuilder;

#[tokio::main]
async fn main() {
    init_logger();

    dotenv::dotenv().ok();

    let settings = Settings::new().unwrap();
    
    AppBuilder::default(settings)
        .bind(([127, 0, 0, 1], 1337).into())
        .bind(([127, 0, 0, 1], 1338).into())
        .bind_tls(([127, 0, 0, 1], 4434).into())
        .run().await;

    warn!("Starting Redirect!");
}