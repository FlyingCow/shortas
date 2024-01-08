use serde_derive::Deserialize;

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct Dynamo {
    pub encryption_table: String,
    pub routes_table: String,
    pub hostname_mappings_table: String,
    pub user_settings_table: String
}