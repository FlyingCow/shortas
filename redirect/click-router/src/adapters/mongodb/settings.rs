use serde_derive::Deserialize;

fn default_challenges_collection() -> String {
    "acme_challenges".to_string()
}

#[derive(Default, Debug, Deserialize, Clone)]
#[allow(unused)]
pub struct Mongodb {
    pub uri: String,
    pub database: String,
    pub encryption_collection: String,
    pub routes_collection: String,
    pub hostname_mappings_collection: String,
    pub user_settings_collection: String,
    #[serde(default = "default_challenges_collection")]
    pub challenges_collection: String,
}
