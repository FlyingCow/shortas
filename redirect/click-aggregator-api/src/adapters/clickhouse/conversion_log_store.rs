use anyhow::{bail, Result};
use serde::{Deserialize, Serialize};

/// A conversion event, refund or rejected event.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, salvo::oapi::ToSchema)]
pub struct ConversionLogItem {
    pub id: String,
    /// `event` or `refund`.
    pub kind: String,
    /// `pending`, `attributed`, `unattributed`, `rejected`; for refunds
    /// `refunded`, `invalid` or `rejected`.
    pub status: String,
    pub reason: String,
    /// The invalid field, if the reason is one.
    pub field: String,
    pub source: String,
    pub test: bool,
    pub step: String,
    pub event_id: String,
    pub external_id: String,
    pub refund_id: String,
    pub click_id: String,
    pub route_id: String,
    /// Decimal string.
    pub value: Option<String>,
    /// The part of a refund that reduces revenue.
    pub applied: Option<String>,
    pub currency: String,
    /// ISO 8601.
    pub occurred_at: String,
    pub received_at: String,
}

#[derive(Debug, Clone, Serialize, salvo::oapi::ToSchema)]
pub struct ConversionLog {
    pub items: Vec<ConversionLogItem>,
    pub has_more: bool,
}

const SQL: &str = "
WITH events AS (
    SELECT id, kind, toString(status) AS status, toString(reason) AS reason, '' AS field,
        toString(source) AS source, toBool(test) AS test, step, event_id, external_id, refund_id, click_id,
        route_id, toString(value) AS value, CAST(NULL AS Nullable(String)) AS applied,
        toString(currency) AS currency, occurred_at, received_at
    FROM conversion_events FINAL
    WHERE workspace_id = {workspace_id:String} AND conversion_id = {conversion_id:String}
        AND kind = 'event'
    UNION ALL
    SELECT id, 'refund', if(invalid_reason = '', 'refunded', 'invalid'), toString(invalid_reason), '',
        toString(source), toBool(test), '', '', external_id, refund_id, '', original_route_id,
        toString(value), toString(applied), toString(currency), occurred_at, received_at
    FROM conversion_refunds(workspace_id = {workspace_id:String}, conversion_id = {conversion_id:String})
    UNION ALL
    SELECT id, kind, 'rejected', toString(reason), toString(field), toString(source), toBool(test), step,
        event_id, external_id, refund_id, click_id, '', toString(value), NULL, toString(currency),
        occurred_at, received_at
    FROM conversion_rejected FINAL
    WHERE workspace_id = {workspace_id:String} AND conversion_id = {conversion_id:String}
    UNION ALL
    SELECT id, 'event', 'pending', '', '', JSONExtractString(payload, 'source'),
        toBool(JSONExtractBool(payload, 'test')), JSONExtractString(payload, 'step'),
        JSONExtractString(payload, 'eventId'), JSONExtractString(payload, 'externalId'), '',
        JSONExtractString(payload, 'clickId'), '',
        JSONExtract(payload, 'value', 'Nullable(String)'), NULL,
        JSONExtractString(payload, 'currency'),
        parseDateTime64BestEffort(JSONExtractString(payload, 'occurredAt'), 3, 'UTC'),
        parseDateTime64BestEffort(JSONExtractString(payload, 'receivedAt'), 3, 'UTC')
    FROM conversion_pending FINAL
    WHERE resolved = 0 AND workspace_id = {workspace_id:String}
        AND conversion_id = {conversion_id:String}
)
SELECT * REPLACE (
    formatDateTime(occurred_at, '%Y-%m-%dT%H:%i:%S.%fZ', 'UTC') AS occurred_at,
    formatDateTime(received_at, '%Y-%m-%dT%H:%i:%S.%fZ', 'UTC') AS received_at
)
FROM events
ORDER BY events.received_at DESC, id DESC
LIMIT {limit:UInt32} OFFSET {offset:UInt32}
FORMAT JSONEachRow";

#[derive(Clone)]
pub struct ConversionLogStore {
    http_client: reqwest::Client,
    url: String,
    user: String,
    password: String,
    database: String,
}

impl ConversionLogStore {
    pub fn new(url: &str, user: &str, password: &str, database: &str) -> Self {
        Self {
            http_client: reqwest::Client::new(),
            url: url.into(),
            user: user.into(),
            password: password.into(),
            database: database.into(),
        }
    }

    /// Newest first.
    pub async fn read(
        &self,
        workspace_id: &str,
        conversion_id: &str,
        limit: u32,
        offset: u32,
    ) -> Result<ConversionLog> {
        // One more, to know whether there are more.
        let (rows, offset) = ((limit + 1).to_string(), offset.to_string());
        let response = self
            .http_client
            .post(&self.url)
            .query(&[
                ("user", self.user.as_str()),
                ("password", &self.password),
                ("database", &self.database),
                ("readonly", "2"),
                ("output_format_json_quote_decimals", "1"),
                ("param_workspace_id", workspace_id),
                ("param_conversion_id", conversion_id),
                ("param_limit", &rows),
                ("param_offset", &offset),
            ])
            .body(SQL)
            .send()
            .await?;
        let (status, text) = (response.status(), response.text().await?);
        if !status.is_success() {
            bail!("ClickHouse: {text}");
        }
        let mut items = text
            .lines()
            .map(serde_json::from_str)
            .collect::<Result<Vec<ConversionLogItem>, _>>()?;
        let has_more = items.len() > limit as usize;
        items.truncate(limit as usize);
        Ok(ConversionLog { items, has_more })
    }
}
