use serde_derive::Deserialize;

use crate::adapters::aws::dynamo::settings::Dynamo;

use super::kinesis::settings::Kinesis;

#[derive(Debug, Deserialize, Clone)]
#[allow(unused)]
pub struct AWS {
    pub local: bool,
    pub localstack_endpoint: Option<String>,
    pub dynamo: Dynamo,
    pub kinesis: Kinesis,
}