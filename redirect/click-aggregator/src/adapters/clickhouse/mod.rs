use std::{sync::Arc, time::Duration};

use anyhow::Result;
use clickhouse::{inserter::Inserter, sql::Identifier, Client, Row};
use settings::ClickStreamStoreConfig;
use tokio::sync::Mutex;
use tokio_util::sync::CancellationToken;

use crate::core::ClickStreamItem;

use super::ClickStreamStore;

pub mod settings;

#[derive(Clone)]
pub struct ClickhouseClickStreamStore {
    inserter: Arc<Mutex<Inserter<ClickStreamItem>>>,
    token: CancellationToken,
}

impl Row for ClickStreamItem {
    const COLUMN_NAMES: &'static [&'static str] = &[
        "id",
        "owner_id",
        "creator_id",
        "route_id",
        "workspace_id",
        // "dest",
        // "ip",
        // "continent",
        // "country",
        // "location",
        // "os_family",
        // "os_version",
        // "user_agent_family",
        // "user_agent_version",
        // "device_brand",
        // "device_family",
        // "device_model",
        // "session_first",
        // "session_clicks",
        // "is_unique",
        // "is_bot",
    ];
}

impl ClickhouseClickStreamStore {
    pub async fn new(settings: ClickStreamStoreConfig, token: CancellationToken) -> Result<Self> {
        let client = Client::default()
            .with_url(&settings.url)
            .with_database(settings.database);

        let inserter = client
            .inserter::<ClickStreamItem>(&settings.table)?
            // Slice the stream into chunks (one `INSERT` per chunk) by time.
            // See documentation of `with_period` for details.
            .with_period(Some(Duration::from_millis(settings.period_millis)))
            // If you have a lot of parallel inserters (e.g. on multiple nodes),
            // it's reasonable to add some bias to the period to spread the load.
            .with_period_bias(settings.period_bias)
            // We also can use other limits. This is useful when the stream is
            // recovered after a long time of inactivity (e.g. restart of service or CH).
            .with_max_rows(settings.max_rows);

        client
            .query(
                "
                CREATE OR REPLACE TABLE ?
                (
                    id String,
                    owner_id String,
                    creator_id String,
                    route_id String,
                    workspace_id String
                )
                ENGINE = MergeTree
                ORDER BY id",
            )
            .bind(Identifier(&settings.table))
            .with_option("allow_experimental_variant_type", "1")
            // This is required only if we are mixing similar types in the Variant definition
            // In this case, this is various Int/UInt types, Float32/Float64, and String/FixedString
            // Omit this option if there are no similar types in the definition
            .with_option("allow_suspicious_variant_types", "1")
            .execute()
            .await?;
        Ok(Self {
            inserter: Arc::new(Mutex::new(inserter)),
            token,
        })
    }
}

#[async_trait::async_trait]
impl ClickStreamStore for ClickhouseClickStreamStore {
    async fn register(&mut self, click: ClickStreamItem) -> Result<()> {
        let mut inserter = self.inserter.lock().await;

        let r = inserter.write(&click);
        if r.is_err() {
            println!("{}", "error");
        }

        let r = inserter.commit().await;
        if r.is_err() {
            println!("{}", "error");
        }

        if self.token.is_cancelled() {
            inserter.commit().await?;
        }

        Ok(())
    }
}
