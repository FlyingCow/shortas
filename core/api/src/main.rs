use anyhow::Result;

use std::env;
use std::net::TcpListener;

use api::run;

#[tokio::main]
async fn main() {
    
    let environment_file;
    if let Ok(e) = env::var("ENV") {
        environment_file = format!(".{}.env", e);
    } else {
        environment_file = String::from(".env");
    }

    dotenv::from_filename(environment_file).ok();



    let port = dotenv::var("PORT")
            .unwrap_or("8000".to_string());


    let listener = TcpListener::bind(format!("0.0.0.0:{}", port))
        .expect("Failed to bind random port");

    let _ = run(listener).await
        .unwrap()
        .await;
}
