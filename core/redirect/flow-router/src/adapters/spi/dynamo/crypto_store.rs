use std::collections::HashMap;

use aws_config::SdkConfig;
use aws_sdk_dynamodb::types::AttributeValue;
use aws_sdk_dynamodb::{Client, Error as DynamoError};

use crate::domain::Keycert;
use crate::core::base_crypto_store::{ 
    BaseCryptoStore, 
    CryptoStoreError, 
    Result
};

#[derive(Clone, Debug)]
pub struct CryptoStore {
    client: Client,
    encryption_table: String
}

impl CryptoStore {
    pub fn new(sdk_config: &SdkConfig, encryption_table: String) -> Self {
        Self {
            encryption_table,
            client: Client::new(sdk_config),
        }
    }
}

impl BaseCryptoStore for CryptoStore {
    async fn get_certificate(&self, server_name: &str) -> Result<Option<Keycert>> {
        Ok(Some(
            Keycert::new()
        ))
    }
}