use std::{sync::Arc, time::Duration};

use anyhow::Result;
use chrono::{DateTime, Utc};
use clickhouse::{inserter::Inserter, Client, Row};
use serde::{Deserialize, Serialize};
use settings::ClickStreamStoreConfig;
use tokio::sync::Mutex;
use tokio_util::sync::CancellationToken;

use crate::core::ClickStreamItem;

use super::ClickStreamStore;

pub mod settings;

#[derive(Clone)]
pub struct ClickhouseClickStreamStore {
    inserter: Arc<Mutex<Inserter<ClickStreamItemRow>>>,
    token: CancellationToken,
}

#[derive(Clone, Default, Debug, Serialize, Deserialize)]
pub struct ClickStreamItemRow {
    pub id: String,
    pub owner_id: String,
    pub creator_id: String,
    pub route_id: String,
    pub workspace_id: String,
    #[serde(with = "clickhouse::serde::chrono::datetime64::millis")]
    pub created: DateTime<Utc>,
    pub dest: String,
    pub ip: String,
    pub continent: Option<String>,
    pub country: Option<String>,
    pub location: Option<String>,
    pub os_family: Option<String>,
    pub os_version: Option<String>,
    pub user_agent_family: Option<String>,
    pub user_agent_version: Option<String>,
    pub device_brand: Option<String>,
    pub device_family: Option<String>,
    pub device_model: Option<String>,
    #[serde(with = "clickhouse::serde::chrono::datetime64::millis::option")]
    pub session_first: Option<DateTime<Utc>>,
    pub session_clicks: Option<u128>,
    pub is_unique: bool,
    pub is_bot: bool,
}

impl Row for ClickStreamItemRow {
    const COLUMN_NAMES: &'static [&'static str] = &[
        "id",
        "owner_id",
        "creator_id",
        "route_id",
        "workspace_id",
        "created",
        "dest",
        "ip",
        "continent",
        "country",
        "location",
        "os_family",
        "os_version",
        "user_agent_family",
        "user_agent_version",
        "device_brand",
        "device_family",
        "device_model",
        "session_first",
        "session_clicks",
        "is_unique",
        "is_bot",
    ];
}

impl ClickhouseClickStreamStore {
    pub async fn new(settings: ClickStreamStoreConfig, token: CancellationToken) -> Result<Self> {
        let mut client = Client::default()
            .with_url(&settings.url)
            .with_database(&settings.database);

        if let Some(user) = &settings.user {
            client = client.with_user(user);
        }

        if let Some(password) = &settings.password {
            client = client.with_password(password);
        }

        let inserter = client
            .inserter::<ClickStreamItemRow>(&settings.table)?
            // Slice the stream into chunks (one `INSERT` per chunk) by time.
            // See documentation of `with_period` for details.
            .with_period(Some(Duration::from_millis(settings.period_millis)))
            // If you have a lot of parallel inserters (e.g. on multiple nodes),
            // it's reasonable to add some bias to the period to spread the load.
            .with_period_bias(settings.period_bias)
            // We also can use other limits. This is useful when the stream is
            // recovered after a long time of inactivity (e.g. restart of service or CH).
            .with_max_rows(settings.max_rows);

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

        inserter.write(&ClickStreamItemRow {
            id: click.id,
            owner_id: click.owner_id,
            creator_id: click.creator_id,
            route_id: click.route_id,
            workspace_id: click.workspace_id,
            created: click.created,
            dest: click.dest,
            ip: click.ip,
            continent: click.continent,
            country: click.country,
            location: click.location,
            os_family: click.os_family,
            os_version: click.os_version,
            user_agent_family: click.user_agent_family,
            user_agent_version: click.user_agent_version,
            device_brand: click.device_brand,
            device_family: click.device_family,
            device_model: click.device_model,
            session_first: click.session_first,
            session_clicks: click.session_clicks,
            is_unique: click.is_unique,
            is_bot: click.is_bot,
        })?;
        // {0:"Code: 33. DB::Exception: Cannot read all data. Bytes read: 108. Bytes expected: 112.: (at row 34)
        // : While executing BinaryRowInputFormat. (CANNOT_READ_ALL_DATA) (version 25.4.4.25 (official build))"}
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
