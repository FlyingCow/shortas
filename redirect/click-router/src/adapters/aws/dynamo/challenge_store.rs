use std::collections::HashMap;

use anyhow::Result;
use aws_config::SdkConfig;
use aws_sdk_dynamodb::types::AttributeValue;
use aws_sdk_dynamodb::Client;

use crate::core::challenge::{Challenge, ChallengeStore};

#[derive(Clone, Debug)]
pub struct DynamoChallengeStore {
    client: Client,
    challenges_table: String,
}

impl DynamoChallengeStore {
    pub fn new(sdk_config: &SdkConfig, challenges_table: String) -> Self {
        Self {
            challenges_table,
            client: Client::new(sdk_config),
        }
    }
}

#[async_trait::async_trait()]
impl ChallengeStore for DynamoChallengeStore {
    async fn get_challenge(&self, domain: &str, token: &str) -> Result<Option<Challenge>> {
        let item = self
            .client
            .get_item()
            .table_name(&self.challenges_table)
            .set_key(Some(HashMap::from([
                (
                    "domain".to_string(),
                    AttributeValue::S(domain.to_ascii_lowercase()),
                ),
                ("token".to_string(), AttributeValue::S(token.to_string())),
            ])))
            .send()
            .await?;

        match item.item {
            Some(item) => {
                let domain = item
                    .get("domain")
                    .and_then(|v| v.as_s().ok())
                    .map(|s| s.clone())
                    .unwrap_or_default();

                let token = item
                    .get("token")
                    .and_then(|v| v.as_s().ok())
                    .map(|s| s.clone())
                    .unwrap_or_default();

                let key_authorization = item
                    .get("key_authorization")
                    .and_then(|v| v.as_s().ok())
                    .map(|s| s.clone())
                    .unwrap_or_default();

                Ok(Some(Challenge {
                    domain,
                    token,
                    key_authorization,
                }))
            }
            None => Ok(None),
        }
    }
}
