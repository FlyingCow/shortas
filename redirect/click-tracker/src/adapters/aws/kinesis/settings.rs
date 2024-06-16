use serde_derive::Deserialize;

#[derive(Debug, Deserialize, Clone)]
#[allow(unused)]
pub struct HitStreamConfig {
    pub stream_name: String,
    pub partition_keys: Vec<String>,
    pub batch_size: usize,
    pub consumers_count: usize,
    pub iteration_seconds: u64,
}

#[derive(Debug, Deserialize, Clone)]
#[allow(unused)]
pub struct Kinesis {
    pub hit_stream: HitStreamConfig
}