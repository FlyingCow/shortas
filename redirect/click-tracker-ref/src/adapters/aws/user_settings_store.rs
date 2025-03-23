use std::collections::HashMap;

use anyhow::Result;

use aws_config::SdkConfig;
use aws_sdk_dynamodb::Client;
use aws_sdk_dynamodb::operation::get_item::GetItemOutput;
use aws_sdk_dynamodb::types::AttributeValue;

use crate::core::{ActiveStatus, UserSettings, UserSettingsStore};

const ACTIVE: &'static str = "active";
const BLOCKED: &'static str = "blocked";

#[derive(Clone, Debug)]
pub struct DynamoUserSettingsStore {
    client: Client,
    user_settings_table: String,
}

impl DynamoUserSettingsStore {
    pub fn new(sdk_config: &SdkConfig, user_settings_table: String) -> Self {
        Self {
            user_settings_table,
            client: Client::new(sdk_config),
        }
    }

    fn to_entity(&self, model: GetItemOutput) -> Result<Option<UserSettings>> {
        model.item.map_or(Ok(None), |item| {
            let user_id = String::from(item.get("user_id").unwrap().as_s().unwrap());

            let user_email = String::from(item.get("user_email").unwrap().as_s().unwrap());

            let api_key = item
                .get("api_key")
                .map_or(None, |item| Some(String::from(item.as_s().unwrap())));

            let active_status =
                item.get("status").map_or(ActiveStatus::Active, |item| {
                    match item.as_s().unwrap().as_str() {
                        ACTIVE => ActiveStatus::Active,
                        BLOCKED => ActiveStatus::Blocked,
                        _ => ActiveStatus::Active,
                    }
                });

            let debug = item.get("debug").map_or(false, |d| *d.as_bool().unwrap());

            let overflow = item
                .get("overflow")
                .map_or(false, |d| *d.as_bool().unwrap());

            let skip = item
                .get("skip")
                .map_or(vec![], |d| d.as_ss().unwrap().clone());

            let allowed_request_params = item
                .get("request_params")
                .map_or(vec![], |d| d.as_ss().unwrap().clone());

            let allowed_destination_params = item
                .get("destination_params")
                .map_or(vec![], |d| d.as_ss().unwrap().clone());

            Ok(Some(UserSettings::new(
                user_id,
                user_email,
                api_key,
                active_status,
                debug,
                overflow,
                skip,
                allowed_request_params,
                allowed_destination_params,
            )))
        })
    }
}

#[async_trait::async_trait]
impl UserSettingsStore for DynamoUserSettingsStore {
    async fn invalidate(&self, _: &str) -> Result<()> {
        Ok(())
    }
    async fn get(&self, user_id: &str) -> Result<Option<UserSettings>> {
        let item = self
            .client
            .get_item()
            .table_name(&self.user_settings_table)
            .set_key(Some(HashMap::from([(
                String::from("user_id"),
                AttributeValue::S(String::from(user_id)),
            )])))
            .send()
            .await?;

        Ok(self.to_entity(item)?)
    }
}
