use serde_derive::Deserialize;

#[derive(Debug, Deserialize, Clone)]
#[allow(unused)]
pub struct ClickStreamConfig {
    pub hosts: Vec<String>,
    pub topic: String,
}

#[derive(Debug, Deserialize, Clone)]
#[allow(unused)]
pub struct Kafka {
    pub click_stream: ClickStreamConfig,
}
