use aws_config::SdkConfig;
use crate::adapters::spi::aws::{ 
    settings::AWS,
    dynamo::crypto_store::CryptoStore,
    dynamo::routes_store::RoutesStore 
};

use super::dynamo::settings::Dynamo;

pub struct DynamoBuilder {
    pub crypto_store: CryptoStore,
    pub routes_store: RoutesStore
}

pub struct AwsBuilder {
    pub dynamo: DynamoBuilder
}

async fn load_aws_config(settings: &AWS) -> SdkConfig {
    let mut shared_config = aws_config::defaults(aws_config::BehaviorVersion::latest());

    if settings.local {
        shared_config = shared_config.endpoint_url(&settings.localstack_endpoint);
    }

    shared_config.load().await
}

impl DynamoBuilder {
    pub async fn new (sdk_config: &SdkConfig, settings: Dynamo) -> Self {
        Self {
            crypto_store: CryptoStore::new(sdk_config, settings.encryption_table),
            routes_store: RoutesStore::new(sdk_config, settings.routes_table)
        }
    }
}

impl AwsBuilder {
    pub async fn new (settings: AWS) -> Self {
        let sdk_config: &SdkConfig = &load_aws_config(&settings).await;
        Self {
            dynamo: DynamoBuilder::new(sdk_config, settings.dynamo).await
        }
    }
}