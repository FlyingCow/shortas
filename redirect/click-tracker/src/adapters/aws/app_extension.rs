use aws_config::SdkConfig;
use tracing::info;

use crate::{
    // adapters::aws::{
    //     dynamo::{

    //     },
    //     kinesis::hit_registrar::KinesisHitRegistrar,
    // },
    adapters::aws::dynamo::user_settings_store::DynamoUserSettingsStore, app::AppBuilder
};

use super::settings::AWS;

async fn load_aws_config(settings: AWS) -> SdkConfig {
    let mut shared_config = aws_config::defaults(aws_config::BehaviorVersion::latest());

    if settings.local {
        let endpoint = settings
            .localstack_endpoint
            .unwrap_or("http://localhost:4566".to_string());

        info!("  {} -> {}", "localstack", endpoint);
        shared_config = shared_config.endpoint_url(endpoint);
    }

    shared_config.load().await
}

impl AppBuilder {
    pub async fn with_aws(&mut self) -> &mut Self {
        info!("{}", "WITH AWS PROVIDERS");

        let config = load_aws_config(self.settings.aws.clone()).await;

        let user_settings_store = Some(Box::new(DynamoUserSettingsStore::new(
            &config,
            self.settings.aws.dynamo.user_settings_table.clone(),
        )) as Box<_>);

        self.user_settings_store = user_settings_store;

        self
    }
}
