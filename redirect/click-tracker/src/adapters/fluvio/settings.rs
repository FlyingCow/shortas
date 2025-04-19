use serde_derive::Deserialize;

#[derive(Debug, Deserialize, Clone)]
#[allow(unused)]
pub struct HitStreamConfig {
    pub host: String,
    pub topic: String,
}

#[derive(Debug, Deserialize, Clone)]
#[allow(unused)]
pub struct ClickAggsConfig {
    pub host: String,
    pub topic: String,
    pub batch_size: usize,
    pub linger: u64,
}

#[derive(Debug, Deserialize, Clone)]
#[allow(unused)]
pub struct Fluvio {
    pub hit_stream: HitStreamConfig,
    pub click_aggs: ClickAggsConfig,
}
