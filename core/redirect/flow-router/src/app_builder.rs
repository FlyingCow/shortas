use crate::adapters::spi::aws::app_builder_extension::AwsBuilder;
use crate::adapters::spi::moka::app_builder_extension::MokaBuilder;
use crate::core::default::CryptoManager;

use crate::settings::Settings;

pub struct AppBuilder{

}

impl AppBuilder {
    pub async fn new(){

        let settings = Settings::new().unwrap();

        let aws = AwsBuilder::new(settings.aws).await;
        let moka = MokaBuilder::new(settings.moka).await;

        let crypto_manager = CryptoManager::new(
            aws.dynamo.crypto_store,
            moka.crypto_cache
        ); 
    }
}