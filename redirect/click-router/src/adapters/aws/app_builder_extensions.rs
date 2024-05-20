use aws_config::SdkConfig;

use crate::{
    adapters::aws::dynamo::{
        dynamo_crypto_store::DynamoCryptoStore, dynamo_routes_store::DynamoRoutesStore, dynamo_user_settings_store::DynamoUserSettingsStore,
    },
    app_builder::AppBuilder
};

use super::aws_settings::AWS;

async fn load_aws_config(settings: AWS) -> SdkConfig {
    let mut shared_config = aws_config::defaults(aws_config::BehaviorVersion::latest());

    if settings.local {
        shared_config = shared_config.endpoint_url(&settings.localstack_endpoint);
    }

    shared_config.load().await
}

impl AppBuilder {
    pub async fn with_aws(&mut self) -> &mut Self {
        println!("{}", "WITH AWS PROVIDERS");

        let config = load_aws_config(self.settings.aws.clone()).await;
        let routes_store = Some(Box::new(DynamoRoutesStore::new(
            &config,
            self.settings.aws.dynamo.routes_table.clone(),
        ))as Box<_>);

        let crypto_store = Some(Box::new(DynamoCryptoStore::new(
            &config,
            self.settings.aws.dynamo.encryption_table.clone(),
        )) as Box<_>);

        let user_settings_store = Some(Box::new(DynamoUserSettingsStore::new(
            &config,
            self.settings.aws.dynamo.user_settings_table.clone(),
        )) as Box<_>);

        self.routes_store = routes_store;
        self.crypto_store = crypto_store;
        self.user_settings_store = user_settings_store;

        self
    }
}
