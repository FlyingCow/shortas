use serde_derive::Deserialize;

#[derive(Debug, Deserialize, Clone)]
#[allow(unused)]
pub struct ClickStreamConfig {
    pub host: String,
    pub topic: String,
    pub consumer: String,
}

#[derive(Debug, Deserialize, Clone)]
#[allow(unused)]
pub struct Fluvio {
    pub click_stream: ClickStreamConfig,
}
