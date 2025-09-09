use serde_derive::Deserialize;

#[derive(Default, Debug, Deserialize, Clone)]
#[allow(unused)]
pub struct Mongodb {
    pub uri: String,
    pub database: String,
    pub user_settings_collection: String,
}
