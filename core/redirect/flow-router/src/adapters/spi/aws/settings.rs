use serde_derive::Deserialize;

use crate::adapters::spi::aws::dynamo::settings::Dynamo;

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct AWS {
    pub local: bool,
    pub localstack_endpoint: String,
    pub dynamo: Dynamo
}