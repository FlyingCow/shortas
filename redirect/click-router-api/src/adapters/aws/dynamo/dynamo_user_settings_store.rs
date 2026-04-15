use std::collections::HashMap;

use anyhow::Result;

use aws_config::SdkConfig;
use aws_sdk_dynamodb::operation::get_item::GetItemOutput;
use aws_sdk_dynamodb::types::AttributeValue;
use aws_sdk_dynamodb::Client;

use crate::core::UserSettingsStore;
use crate::model::{ActiveStatus, UserSettings};

const ACTIVE: &str = "active";
const BLOCKED: &str = "blocked";

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

#[async_trait::async_trait()]
impl UserSettingsStore for DynamoUserSettingsStore {
    async fn store_user_settings(&self, user_settings: &UserSettings) -> Result<()> {
        let mut request = self
            .client
            .put_item()
            .table_name(&self.user_settings_table)
            .item("user_id", AttributeValue::S(user_settings.user_id.clone()))
            .item("user_email", AttributeValue::S(user_settings.user_email.clone()))
            .item("status", AttributeValue::S(match user_settings.active_status {
                ActiveStatus::Active => ACTIVE.to_string(),
                ActiveStatus::Blocked => BLOCKED.to_string(),
            }))
            .item("debug", AttributeValue::Bool(user_settings.debug))
            .item("overflow", AttributeValue::Bool(user_settings.overflow));

        if let Some(api_key) = &user_settings.api_key {
            request = request.item("api_key", AttributeValue::S(api_key.clone()));
        }

        if !user_settings.skip.is_empty() {
            request = request.item("skip", AttributeValue::Ss(user_settings.skip.clone()));
        }

        if !user_settings.allowed_request_params.is_empty() {
            request = request.item("request_params", AttributeValue::Ss(user_settings.allowed_request_params.clone()));
        }

        if !user_settings.allowed_destination_params.is_empty() {
            request = request.item("destination_params", AttributeValue::Ss(user_settings.allowed_destination_params.clone()));
        }

        request
            .condition_expression("attribute_not_exists(user_id)")
            .send()
            .await?;

        Ok(())
    }

    async fn update_user_settings(&self, user_settings: &UserSettings) -> Result<()> {
        let mut request = self
            .client
            .put_item()
            .table_name(&self.user_settings_table)
            .item("user_id", AttributeValue::S(user_settings.user_id.clone()))
            .item("user_email", AttributeValue::S(user_settings.user_email.clone()))
            .item("status", AttributeValue::S(match user_settings.active_status {
                ActiveStatus::Active => ACTIVE.to_string(),
                ActiveStatus::Blocked => BLOCKED.to_string(),
            }))
            .item("debug", AttributeValue::Bool(user_settings.debug))
            .item("overflow", AttributeValue::Bool(user_settings.overflow));

        if let Some(api_key) = &user_settings.api_key {
            request = request.item("api_key", AttributeValue::S(api_key.clone()));
        }

        if !user_settings.skip.is_empty() {
            request = request.item("skip", AttributeValue::Ss(user_settings.skip.clone()));
        }

        if !user_settings.allowed_request_params.is_empty() {
            request = request.item("request_params", AttributeValue::Ss(user_settings.allowed_request_params.clone()));
        }

        if !user_settings.allowed_destination_params.is_empty() {
            request = request.item("destination_params", AttributeValue::Ss(user_settings.allowed_destination_params.clone()));
        }

        request.send().await?;
        Ok(())
    }

    async fn delete_user_settings(&self, user_settings: &UserSettings) -> Result<()> {
        self.client
            .delete_item()
            .table_name(&self.user_settings_table)
            .key("user_id", AttributeValue::S(user_settings.user_id.clone()))
            .send()
            .await?;
        Ok(())
    }

    async fn get_user_settings(&self, user_id: &str) -> Result<Option<UserSettings>> {
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
    async fn invalidate_user_settings(&self, _: &str) -> Result<()> {
        Ok(())
    }
}
