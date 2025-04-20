use serde_derive::Deserialize;

#[derive(Debug, Deserialize, Clone)]
#[allow(unused)]
pub struct Redis {
    pub host: String,
    pub password: String,
}
