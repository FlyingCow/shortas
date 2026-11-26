use crate::adapters::clickhouse::clickstream_store::ClickHouseClickStreamStore;
use crate::adapters::clickhouse::conversion_log_store::ConversionLogStore;
use crate::settings::Settings;

#[derive(Clone)]
pub struct AppState {
    pub clickstream_store: ClickHouseClickStreamStore,
    pub conversion_log_store: ConversionLogStore,
}

impl AppState {
    pub fn new(settings: &Settings) -> anyhow::Result<Self> {
        let clickstream_store = ClickHouseClickStreamStore::new(
            &settings.clickhouse.url,
            &settings.clickhouse.user,
            &settings.clickhouse.password,
            &settings.clickhouse.database
        )?;

        let conversion_log_store = ConversionLogStore::new(
            &settings.clickhouse.url,
            &settings.clickhouse.user,
            &settings.clickhouse.password,
            &settings.clickhouse.database,
        );

        Ok(AppState {
            clickstream_store,
            conversion_log_store,
        })
    }
}
