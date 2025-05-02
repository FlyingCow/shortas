use serde_derive::Deserialize;
#[derive(Default, Debug, Deserialize, Clone)]
#[allow(unused)]
pub struct UAParser {
    pub yaml: String,
}
