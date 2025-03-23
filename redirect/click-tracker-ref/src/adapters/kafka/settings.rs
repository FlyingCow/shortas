use serde_derive::Deserialize;

#[derive(Debug, Deserialize, Clone)]
#[allow(unused)]
pub struct HitStreamConfig {
    pub hosts: Vec<String>,
    pub topic: String
}

#[derive(Debug, Deserialize, Clone)]
#[allow(unused)]
pub struct ClickAggsConfig {
    pub hosts: Vec<String>,
    pub topic: String,
    pub ack_timeout_secs: u64,
}

#[derive(Debug, Deserialize, Clone)]
#[allow(unused)]
pub struct Kafka {
    pub hit_stream: HitStreamConfig,
    pub click_aggs: ClickAggsConfig
}