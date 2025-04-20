use serde_derive::Deserialize;

#[derive(Debug, Deserialize, Clone)]
#[allow(unused)]
pub struct Dynamo {
    pub encryption_table: String,
    pub routes_table: String,
    pub hostname_mappings_table: String,
    pub user_settings_table: String,
}

#[derive(Debug, Deserialize, Clone)]
#[allow(unused)]
pub struct AWS {
    pub local: bool,
    pub localstack_endpoint: Option<String>,
    pub dynamo: Dynamo,
}
