use std::collections::HashMap;

use anyhow::Result;
use chrono::{DateTime, Utc};
use clickhouse::{Client, Compression, Row};
use serde::{Deserialize, Serialize};

use crate::adapters::clickhouse::settings::ClickStreamStoreConfig;

use super::{
    attribution::{decide, Click, Decision},
    record::ConversionRecord,
};

/// Values of `conversion_pending.resolved`. The highest wins.
pub mod state {
    /// Waiting for its click.
    pub const WAITING: u8 = 0;
    /// Counted as unattributed, and still re-checked.
    pub const LATE: u8 = 1;
    pub const DONE: u8 = 2;
}

#[derive(Clone)]
pub struct ConversionStore {
    client: Client,
    clicks_table: String,
}

#[derive(Debug, Row, Serialize)]
struct EventRow {
    workspace_id: String,
    conversion_id: String,
    dedupe_key: String,
    version: u64,
    id: String,
    kind: String,
    status: &'static str,
    reason: &'static str,
    source: String,
    test: u8,
    step: String,
    step_order: u32,
    is_final: u8,
    event_id: String,
    external_id: String,
    refund_id: String,
    click_id: String,
    value: Option<i64>,
    currency: String,
    #[serde(with = "clickhouse::serde::chrono::datetime64::millis")]
    occurred_at: DateTime<Utc>,
    #[serde(with = "clickhouse::serde::chrono::datetime64::millis")]
    received_at: DateTime<Utc>,
    #[serde(with = "clickhouse::serde::chrono::datetime64::millis")]
    click_at: DateTime<Utc>,
    route_id: String,
    country: String,
    device_family: String,
    os_family: String,
    user_agent_family: String,
}

#[derive(Debug, Row, Serialize)]
struct RejectedRow {
    workspace_id: String,
    conversion_id: String,
    id: String,
    reason: String,
    field: String,
    kind: String,
    source: String,
    test: u8,
    step: String,
    event_id: String,
    external_id: String,
    refund_id: String,
    click_id: String,
    value: Option<i64>,
    currency: String,
    #[serde(with = "clickhouse::serde::chrono::datetime64::millis")]
    occurred_at: DateTime<Utc>,
    #[serde(with = "clickhouse::serde::chrono::datetime64::millis")]
    received_at: DateTime<Utc>,
}

#[derive(Debug, Row, Serialize)]
struct PendingRow {
    id: String,
    workspace_id: String,
    conversion_id: String,
    resolved: u8,
    payload: String,
    #[serde(with = "clickhouse::serde::chrono::datetime64::millis")]
    inserted_at: DateTime<Utc>,
}

#[derive(Debug, Row, Deserialize)]
struct PendingReadRow {
    id: String,
    payload: String,
    #[serde(with = "clickhouse::serde::chrono::datetime64::millis")]
    inserted_at: DateTime<Utc>,
}

#[derive(Debug, Row, Deserialize)]
struct ClickRow {
    id: String,
    workspace_id: String,
    route_id: String,
    #[serde(with = "clickhouse::serde::chrono::datetime64::millis")]
    created: DateTime<Utc>,
    country: String,
    device_family: String,
    os_family: String,
    user_agent_family: String,
    is_trackable: u8,
}

/// An event waiting for its click, or late.
#[derive(Debug)]
pub struct Pending {
    pub id: String,
    /// `None` when unreadable.
    pub record: Option<ConversionRecord>,
    payload: String,
    pub stored_at: DateTime<Utc>,
}

/// Rows to insert together.
#[derive(Default)]
pub struct Batch {
    events: Vec<EventRow>,
    rejected: Vec<RejectedRow>,
    pending: Vec<PendingRow>,
}

impl Batch {
    /// Adds a record. An event with a click ID is decided by its click, given
    /// in `clicks` when stored, and otherwise waits for the reconciler.
    pub fn add(
        &mut self,
        record: &ConversionRecord,
        clicks: &HashMap<String, Click>,
        now: DateTime<Utc>,
    ) -> Result<&'static str> {
        if let Some(reason) = &record.rejection {
            // Rejected events may hold anything.
            let value = record.scaled_value().ok().flatten();
            let field = record.rejected_field.clone().unwrap_or_default();
            self.rejected.push(rejected_row(record, value, reason, field));
            return Ok("rejected");
        }
        let value = record.scaled_value()?;
        if record.is_refund() {
            self.events.push(event_row(record, value, None, ""));
            return Ok("refund");
        }
        let Some(click_id) = &record.click_id else {
            self.events.push(event_row(record, value, None, "no_click_id"));
            return Ok("unattributed");
        };
        match decide(record, clicks.get(click_id), now, now) {
            Decision::Wait => {
                self.pending.push(PendingRow {
                    id: record.id.clone(),
                    workspace_id: record.workspace_id.clone(),
                    conversion_id: record.conversion_id.clone(),
                    resolved: state::WAITING,
                    payload: serde_json::to_string(record)?,
                    inserted_at: now,
                });
                Ok("pending")
            }
            decision => self.decided(record, decision),
        }
    }

    /// Adds the rows of a verdict. Nothing while it waits.
    pub fn decided(&mut self, record: &ConversionRecord, decision: Decision) -> Result<&'static str> {
        let value = record.scaled_value()?;
        Ok(match decision {
            Decision::Wait => "waiting",
            Decision::Attributed(click) => {
                self.events.push(event_row(record, value, Some(click), ""));
                "attributed"
            }
            Decision::Unattributed(reason) => {
                self.events.push(event_row(record, value, None, reason));
                "unattributed"
            }
            Decision::Rejected(reason) => {
                self.rejected.push(rejected_row(record, value, reason, String::new()));
                "rejected"
            }
        })
    }

    /// Moves a pending event to `to`, one of [`state`]. Late events keep their
    /// record for the next re-check.
    pub fn mark(&mut self, pending: &Pending, to: u8, now: DateTime<Utc>) {
        let (workspace_id, conversion_id) = pending
            .record
            .as_ref()
            .map(|r| (r.workspace_id.clone(), r.conversion_id.clone()))
            .unwrap_or_default();
        self.pending.push(PendingRow {
            id: pending.id.clone(),
            workspace_id,
            conversion_id,
            resolved: to,
            payload: if to == state::LATE { pending.payload.clone() } else { String::new() },
            inserted_at: now,
        });
    }
}

fn event_row(
    r: &ConversionRecord,
    value: Option<i64>,
    click: Option<&Click>,
    reason: &'static str,
) -> EventRow {
    let of_click = |get: fn(&Click) -> &String| click.map(get).cloned().unwrap_or_default();
    EventRow {
        workspace_id: r.workspace_id.clone(),
        conversion_id: r.conversion_id.clone(),
        dedupe_key: r.dedupe_key(),
        version: r.version(click.is_some()),
        id: r.id.clone(),
        kind: r.kind.clone(),
        status: if click.is_some() { "attributed" } else { "unattributed" },
        reason,
        source: r.source.clone(),
        test: r.test.into(),
        step: r.step.clone(),
        step_order: r.step_order,
        is_final: r.is_final.into(),
        event_id: r.event_id.clone(),
        external_id: r.external_id.clone().unwrap_or_default(),
        refund_id: r.refund_id.clone().unwrap_or_default(),
        click_id: r.click_id.clone().unwrap_or_default(),
        value,
        currency: r.currency.clone(),
        occurred_at: r.occurred_at,
        received_at: r.received_at,
        click_at: click.map(|c| c.created).unwrap_or_default(),
        route_id: of_click(|c| &c.route_id),
        country: of_click(|c| &c.country),
        device_family: of_click(|c| &c.device_family),
        os_family: of_click(|c| &c.os_family),
        user_agent_family: of_click(|c| &c.user_agent_family),
    }
}

fn rejected_row(
    r: &ConversionRecord,
    value: Option<i64>,
    reason: &str,
    field: String,
) -> RejectedRow {
    RejectedRow {
        workspace_id: r.workspace_id.clone(),
        conversion_id: r.conversion_id.clone(),
        id: r.id.clone(),
        reason: reason.into(),
        field,
        kind: r.kind.clone(),
        source: r.source.clone(),
        test: r.test.into(),
        step: r.step.clone(),
        event_id: r.event_id.clone(),
        external_id: r.external_id.clone().unwrap_or_default(),
        refund_id: r.refund_id.clone().unwrap_or_default(),
        click_id: r.click_id.clone().unwrap_or_default(),
        value,
        currency: r.currency.clone(),
        occurred_at: r.occurred_at,
        received_at: r.received_at,
    }
}

impl ConversionStore {
    pub fn new(settings: &ClickStreamStoreConfig) -> Self {
        let mut client = Client::default()
            // Newer servers answer in ZSTD, which the client cannot read.
            .with_compression(Compression::None)
            .with_url(&settings.url)
            .with_database(&settings.database);
        if let Some(user) = &settings.user {
            client = client.with_user(user);
        }
        if let Some(password) = &settings.password {
            client = client.with_password(password);
        }
        Self {
            client,
            clicks_table: settings.table.clone(),
        }
    }

    /// Inserts results before the rows that stop their events from being
    /// matched again. If this fails half way, repeating it changes nothing.
    pub async fn insert(&self, batch: Batch) -> Result<()> {
        self.insert_rows("conversion_events", batch.events).await?;
        self.insert_rows("conversion_rejected", batch.rejected).await?;
        self.insert_rows("conversion_pending", batch.pending).await
    }

    async fn insert_rows<T: Row + Serialize>(&self, table: &str, rows: Vec<T>) -> Result<()> {
        if rows.is_empty() {
            return Ok(());
        }
        let mut insert = self.client.insert(table)?;
        for row in &rows {
            insert.write(row).await?;
        }
        Ok(insert.end().await?)
    }

    /// Pending events in state `of`, with IDs after `after`, in order.
    pub async fn pending(&self, of: u8, after: &str, limit: u32) -> Result<Vec<Pending>> {
        let rows: Vec<PendingReadRow> = self
            .client
            .query(
                "SELECT ?fields FROM conversion_pending FINAL \
                 WHERE resolved = ? AND id > ? ORDER BY id LIMIT ?",
            )
            .bind(of)
            .bind(after)
            .bind(limit)
            .fetch_all()
            .await?;
        Ok(rows
            .into_iter()
            .map(|row| Pending {
                // Rows resolved before late re-checks existed have no payload.
                record: (!row.payload.is_empty())
                    .then(|| serde_json::from_str(&row.payload))
                    .transpose()
                    .inspect_err(|e| tracing::error!(id = %row.id, error = %e, "Unreadable event"))
                    .ok()
                    .flatten(),
                payload: row.payload,
                id: row.id,
                stored_at: row.inserted_at,
            })
            .collect())
    }

    pub async fn clicks(&self, ids: &[&str]) -> Result<HashMap<String, Click>> {
        if ids.is_empty() {
            return Ok(HashMap::new());
        }
        let rows: Vec<ClickRow> = self
            .client
            .query(&format!(
                "SELECT ?fields FROM {} WHERE id IN ?",
                self.clicks_table
            ))
            .bind(ids)
            .fetch_all()
            .await?;
        Ok(rows
            .into_iter()
            .map(|row| {
                let click = Click {
                    workspace_id: row.workspace_id,
                    route_id: row.route_id,
                    created: row.created,
                    country: row.country,
                    device_family: row.device_family,
                    os_family: row.os_family,
                    user_agent_family: row.user_agent_family,
                    trackable: row.is_trackable == 1,
                };
                (row.id, click)
            })
            .collect())
    }
}
