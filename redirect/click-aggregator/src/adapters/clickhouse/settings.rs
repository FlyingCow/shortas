use serde_derive::Deserialize;

#[derive(Debug, Deserialize, Clone)]
#[allow(unused)]
pub struct ClickStreamStoreConfig {
    pub url: String,
    pub max_rows: u64,
    pub period_millis: u64,
    pub period_bias: f64,
    pub database: String,
    pub table: String,
}

#[derive(Debug, Deserialize, Clone)]
#[allow(unused)]
pub struct Clickhouse {
    pub click_stream_store: ClickStreamStoreConfig,
}
