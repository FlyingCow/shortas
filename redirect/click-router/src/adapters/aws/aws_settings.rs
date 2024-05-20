use serde_derive::Deserialize;

use crate::adapters::aws::dynamo::dynamo_settings::Dynamo;

#[derive(Debug, Deserialize, Clone)]
#[allow(unused)]
pub struct AWS {
    pub local: bool,
    pub localstack_endpoint: String,
    pub dynamo: Dynamo
}