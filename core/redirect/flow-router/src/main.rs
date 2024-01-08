
use flow_router::log::*;
//use flow_router::core::app_builder::{ AppBuilder, CryptoBuilder };
use flow_router::adapters::spi::aws::app_builder_extension::{DynamoBuilder, AwsBuilder};
use aws_config::SdkConfig;
use flow_router::settings::Settings;

#[tokio::main]
async fn main() {
    init_logger();

    // DynamoBuilder::new(sdk_config, encryption_table);

    // AppBuilder::new(
    //     CryptoBuilder::new(crypto_store, crypto_cache)
    // );


    dotenv::dotenv().ok();

    let settings = Settings::new().unwrap();
    
    let aws = AwsBuilder::new(settings.aws).await;
    warn!("Starting Redirect!");
}