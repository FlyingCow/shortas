use serde_derive::Deserialize;

#[derive(Debug, Deserialize, Clone)]
#[allow(unused)]
pub struct Mongodb {
    pub connection_string: String,
    pub database_name: String,
    pub routes_collection: String,
    pub crypto_collection: String,
    pub user_settings_collection: String,
    #[serde(default = "default_challenges_collection")]
    pub challenges_collection: String,
}

fn default_challenges_collection() -> String {
    "acme_challenges".to_string()
}
