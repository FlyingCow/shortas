use serde_derive::Deserialize;

#[derive(Default, Debug, Deserialize, Clone)]
#[allow(unused)]
pub struct Mongodb {
    pub uri: String,
    pub database: String,
    pub encryption_collection: String,
    pub routes_collection: String,
    pub hostname_mappings_collection: String,
    pub user_settings_collection: String,
}
