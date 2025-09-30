use crate::adapters::clickhouse::clickstream_store::ClickHouseClickStreamStore;
use crate::settings::Settings;

#[derive(Clone)]
pub struct AppState {
    pub clickstream_store: ClickHouseClickStreamStore,
}

impl AppState {
    pub fn new(settings: &Settings) -> anyhow::Result<Self> {
        let clickstream_store = ClickHouseClickStreamStore::new(
            &settings.clickhouse.url,
            &settings.clickhouse.user,
            &settings.clickhouse.password,
            &settings.clickhouse.database
        )?;

        Ok(AppState {
            clickstream_store,
        })
    }
}
