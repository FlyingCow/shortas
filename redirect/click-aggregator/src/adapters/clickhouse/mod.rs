use std::{sync::Arc, time::Duration};

use anyhow::Result;
use chrono::{DateTime, Utc};
use clickhouse::{inserter::Inserter, Client, Row};
use serde::{Deserialize, Serialize};
use settings::ClickStreamStoreConfig;
use tokio::sync::Mutex;
use tokio_util::sync::CancellationToken;
use tracing::{debug, error, info, warn};

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
    // Geographic fields - non-nullable with default "_unknown"
    pub continent: String,
    pub country: String,
    pub location: String,
    // OS fields - non-nullable with default "_unknown"
    pub os_family: String,
    pub os_version: String,
    // User agent fields - non-nullable with default "_unknown"
    pub user_agent_family: String,
    pub user_agent_version: String,
    // Device fields - non-nullable with default "_unknown"
    pub device_brand: String,
    pub device_family: String,
    pub device_model: String,
    // Session fields - non-nullable with defaults
    #[serde(with = "clickhouse::serde::chrono::datetime64::millis")]
    pub session_first: DateTime<Utc>,
    pub session_clicks: u64,
    pub is_unique: u8,
    pub is_bot: u8,
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

        // Convert Option fields to non-nullable with default values
        const UNKNOWN: &str = "_unknown";
        let epoch = DateTime::from_timestamp(0, 0).unwrap();

        // Validate required fields - skip rows with empty required fields
        if click.id.is_empty() || click.owner_id.is_empty() || click.creator_id.is_empty() 
            || click.route_id.is_empty() || click.workspace_id.is_empty() 
            || click.dest.is_empty() || click.ip.is_empty() {
            warn!(
                "Skipping invalid clickstream item - missing required fields: id={}, owner_id={}, creator_id={}, route_id={}, workspace_id={}, dest={}, ip={}",
                click.id, click.owner_id, click.creator_id, click.route_id, 
                click.workspace_id, click.dest, click.ip
            );
            return Ok(()); // Skip this record
        }

        // Debug: Log valid records being written
        debug!(
            "Writing clickstream: id={}, route_id={}, workspace_id={}, dest={}, ip={}",
            click.id, click.route_id, click.workspace_id, click.dest, click.ip
        );

        let result = inserter.write(&ClickStreamItemRow {
            id: click.id,
            owner_id: click.owner_id,
            creator_id: click.creator_id,
            route_id: click.route_id,
            workspace_id: click.workspace_id,
            created: click.created,
            dest: click.dest,
            ip: click.ip,
            // Geographic fields with default
            continent: click.continent.unwrap_or_else(|| UNKNOWN.to_string()),
            country: click.country.unwrap_or_else(|| UNKNOWN.to_string()),
            location: click.location.unwrap_or_else(|| UNKNOWN.to_string()),
            // OS fields with default
            os_family: click.os_family.unwrap_or_else(|| UNKNOWN.to_string()),
            os_version: click.os_version.unwrap_or_else(|| UNKNOWN.to_string()),
            // User agent fields with default
            user_agent_family: click
                .user_agent_family
                .unwrap_or_else(|| UNKNOWN.to_string()),
            user_agent_version: click
                .user_agent_version
                .unwrap_or_else(|| UNKNOWN.to_string()),
            // Device fields with default
            device_brand: click.device_brand.unwrap_or_else(|| UNKNOWN.to_string()),
            device_family: click.device_family.unwrap_or_else(|| UNKNOWN.to_string()),
            device_model: click.device_model.unwrap_or_else(|| UNKNOWN.to_string()),
            // Session fields with defaults
            session_first: click.session_first.unwrap_or(epoch),
            session_clicks: click.session_clicks.unwrap_or(0) as u64,
            // Boolean to u8 conversion
            is_unique: if click.is_unique { 1 } else { 0 },
            is_bot: if click.is_bot { 1 } else { 0 },
        });
        
        // Handle write errors
        if let Err(e) = result {
            error!("Failed to write to ClickHouse inserter: {}", e);
            return Err(e.into());
        }

        let result = inserter.commit().await;
        // Handle write errors
        if let Err(e) = result {
            error!("Failed to write to ClickHouse inserter: {}", e);
            return Err(e.into());
        }

        debug!("Successfully queued clickstream record to inserter buffer");

        // Check if cancellation token is triggered for shutdown commit
        if self.token.is_cancelled() {
            info!("Cancellation triggered - forcing commit to ClickHouse");
            let r = inserter.commit().await;
            if let Err(e) = r {
                error!("Failed to commit to ClickHouse: {}", e);
                return Err(e.into());
            }
            info!("Successfully committed to ClickHouse on shutdown");
        }
        // Note: Auto-commits happen based on with_period and with_max_rows settings
        // configured during inserter initialization

        Ok(())
    }
}
