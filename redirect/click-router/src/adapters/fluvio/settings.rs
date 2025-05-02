use serde_derive::Deserialize;
#[derive(Default, Debug, Deserialize, Clone)]
#[allow(unused)]
pub struct HitStreamConfig {
    pub host: String,
    pub topic: String,
    pub batch_size: usize,
    pub linger: u64,
}
#[derive(Default, Debug, Deserialize, Clone)]
#[allow(unused)]
pub struct Fluvio {
    pub hit_stream: HitStreamConfig,
}
