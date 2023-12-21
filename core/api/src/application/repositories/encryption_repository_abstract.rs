use async_trait::async_trait;

use crate::domain::encryption::Encryption;

#[cfg(test)]
use mockall::{predicate::*, *};
use std::error::Error;

#[cfg_attr(test, automock)]
#[async_trait(&Send)]
pub trait EncryptionRepositoryAbstract {
    async fn get_encryption(&self, domain: String) -> Result<Encryption, Box<dyn Error>>;
    async fn create_encryption(&self, encryption: Encryption) -> Result<(), Box<dyn Error>>; 
    async fn update_encryption(&self, encryption: Encryption) -> Result<(), Box<dyn Error>>;
    async fn delete_encryption(&self, domain: String) -> Result<(), Box<dyn Error>>;
}