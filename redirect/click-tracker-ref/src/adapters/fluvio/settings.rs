use serde_derive::Deserialize;

#[derive(Debug, Deserialize, Clone)]
#[allow(unused)]
pub struct HitStreamConfig {
    pub host: String,
    pub topic: String,
    pub consumer: String,
}

#[derive(Debug, Deserialize, Clone)]
#[allow(unused)]
pub struct ClickAggsConfig {
    pub host: String,
    pub topic: String,
    pub linger_millis: u64,
    pub batch_size_bytes: usize,
}

#[derive(Debug, Deserialize, Clone)]
#[allow(unused)]
pub struct Fluvio {
    pub hit_stream: HitStreamConfig,
    pub click_aggs: ClickAggsConfig,
}
