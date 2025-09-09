use serde_derive::Deserialize;

#[derive(Debug, Deserialize, Clone)]
#[allow(unused)]
pub struct Dynamo {
    pub user_settings_table: String,
}

#[derive(Debug, Deserialize, Clone)]
#[allow(unused)]
pub struct AWS {
    pub local: bool,
    pub localstack_endpoint: Option<String>,
    pub dynamo: Dynamo,
}
