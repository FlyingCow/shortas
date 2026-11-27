use serde_derive::Deserialize;

use crate::adapters::fluvio::settings::ClickStreamConfig;

#[derive(Debug, Deserialize, Clone)]
#[serde(default)]
pub struct Conversions {
    pub enabled: bool,
    pub stream: ClickStreamConfig,
    /// How often waiting events are matched again.
    pub reconcile_millis: u64,
    /// How often late events, whose click was not found in time, are
    /// matched again.
    pub late_recheck_millis: u64,
}

impl Default for Conversions {
    fn default() -> Self {
        Self {
            enabled: true,
            stream: ClickStreamConfig {
                host: "sc:9003".into(),
                topic: "conversion-stream-main".into(),
                consumer: "conversions".into(),
            },
            reconcile_millis: 5_000,
            late_recheck_millis: 60_000,
        }
    }
}
