use serde_derive::Deserialize;

#[derive(Debug, Deserialize, Clone)]
#[allow(unused)]
pub struct HitStreamConfig {
    pub hosts: Vec<String>,
    pub topic: String,
    pub ack_timeout_secs: u64,
    pub batch_size: usize,
    pub consumers_count: usize,
    pub iteration_seconds: u64,
}

#[derive(Debug, Deserialize, Clone)]
#[allow(unused)]
pub struct Kafka {
    pub hit_stream: HitStreamConfig
}