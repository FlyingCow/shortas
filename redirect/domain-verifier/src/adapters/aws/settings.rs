use serde_derive::Deserialize;

#[derive(Default, Debug, Deserialize, Clone)]
#[allow(unused)]
pub struct AwsSettings {
    #[serde(default)]
    pub local: bool,
    pub dynamo: Option<DynamoSettings>,
}

#[derive(Default, Debug, Deserialize, Clone)]
#[allow(unused)]
pub struct DynamoSettings {
    pub domains_table: String,
}
