use std::collections::HashMap;

use aws_config::SdkConfig;
use aws_sdk_dynamodb::operation::get_item::GetItemOutput;
use aws_sdk_dynamodb::types::AttributeValue;
use aws_sdk_dynamodb::Client;


use crate::core::base_crypto_store::{BaseCryptoStore, Result};
use crate::domain::Keycert;

#[derive(Clone, Debug)]
pub struct CryptoStore {
    client: Client,
    encryption_table: String,
}

impl CryptoStore {
    pub fn new(sdk_config: &SdkConfig, encryption_table: String) -> Self {
        Self {
            encryption_table,
            client: Client::new(sdk_config),
        }
    }

    fn to_entity(&self, model: GetItemOutput) -> Option<Keycert> {
        model.item.map_or(None, |item| {
            let key_str = item.get("key").unwrap().as_s().unwrap();
            let cert_str = item.get("cert").unwrap().as_s().unwrap();

            Some(Keycert::new().key(key_str.as_bytes()).cert(cert_str.as_bytes()))
        })
    }
}

impl BaseCryptoStore for CryptoStore {
    async fn get_certificate(&self, server_name: &str) -> Result<Option<Keycert>> {
        let item = self
            .client
            .get_item()
            .table_name(&self.encryption_table)
            .set_key(Some(HashMap::from([(
                "hostname".to_string(),
                AttributeValue::S(server_name.to_ascii_lowercase()),
            )])))
            .send()
            .await;

        let result = match item {
            Ok(item_output) => Ok(self.to_entity(item_output)),
            Err(e) => {

                println!("status: {}", e.raw_response().unwrap().status());
                Err(e)
            }
        };

        Ok(result?)
    }
}
