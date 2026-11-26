//! Records of the conversion stream, as click-router publishes them.

use anyhow::{ensure, Context, Result};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

const VALUE_SCALE: usize = 4;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConversionRecord {
    pub id: String,
    /// `event` or `refund`.
    pub kind: String,
    pub rejection: Option<String>,
    pub rejected_field: Option<String>,
    pub conversion_id: String,
    pub workspace_id: String,
    pub source: String,
    pub test: bool,
    pub step: String,
    pub step_order: u32,
    pub is_final: bool,
    pub event_id: String,
    pub external_id: Option<String>,
    pub refund_id: Option<String>,
    pub click_id: Option<String>,
    /// Decimal string.
    pub value: Option<String>,
    pub currency: String,
    pub counting: String,
    pub window_days: u32,
    pub route_filter: Option<Vec<String>>,
    pub occurred_at: DateTime<Utc>,
    pub received_at: DateTime<Utc>,
}

impl ConversionRecord {
    pub fn is_refund(&self) -> bool {
        self.kind == "refund"
    }

    /// What counts as the same event. Tests never replace live events.
    pub fn dedupe_key(&self) -> String {
        let test = if self.test { "test:" } else { "" };
        if self.is_refund() {
            return format!("{test}refund:{}", self.refund_id.as_deref().unwrap_or(&self.id));
        }
        let identity = match (&self.click_id, &self.external_id) {
            (Some(click_id), _) if self.counting == "OncePerClick" => format!("click:{click_id}"),
            (_, Some(external_id)) => format!("external:{external_id}"),
            _ => format!("event:{}", self.event_id),
        };
        format!("{test}step:{}:{identity}", self.step)
    }

    /// Ranks rows with the same key: the earliest received wins. Of repeats of
    /// one event, an attributed one does.
    pub fn version(&self, attributed: bool) -> u64 {
        let received = self.received_at.timestamp_millis().clamp(0, 1 << 60) as u64;
        (u64::MAX / 2 - received) * 2 + u64::from(attributed)
    }

    /// The value in units of `Decimal(18, 4)`.
    pub fn scaled_value(&self) -> Result<Option<i64>> {
        self.value.as_deref().map(scaled).transpose()
    }
}

fn scaled(value: &str) -> Result<i64> {
    let (integer, fraction) = value.split_once('.').unwrap_or((value, ""));
    let digits = |s: &str| s.bytes().all(|b| b.is_ascii_digit());
    ensure!(
        !integer.is_empty()
            && integer.len() <= 14
            && fraction.len() <= VALUE_SCALE
            && digits(integer)
            && digits(fraction),
        "invalid value"
    );
    format!("{integer}{fraction:0<VALUE_SCALE$}").parse().context("invalid value")
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;

    pub(crate) fn record(id: &str) -> ConversionRecord {
        let now = Utc::now();
        ConversionRecord {
            id: id.into(),
            kind: "event".into(),
            rejection: None,
            rejected_field: None,
            conversion_id: "c".into(),
            workspace_id: "w".into(),
            source: "server".into(),
            test: false,
            step: "purchase".into(),
            step_order: 1,
            is_final: true,
            event_id: id.into(),
            external_id: None,
            refund_id: None,
            click_id: Some("click".into()),
            value: Some("49".into()),
            currency: "EUR".into(),
            counting: "OncePerClick".into(),
            window_days: 30,
            route_filter: None,
            occurred_at: now,
            received_at: now,
        }
    }

    #[test]
    fn reads_what_the_router_writes() {
        let json = r#"{"id":"01M","kind":"event","rejection":null,"rejectedField":null,
            "conversionId":"c","workspaceId":"w","source":"browser","test":false,"step":"cart",
            "stepOrder":0,"isFinal":false,"eventId":"e","externalId":null,"refundId":null,
            "clickId":"C","value":"5","currency":"EUR","counting":"Every","windowDays":30,
            "routeFilter":["r"],"occurredAt":"2026-11-25T18:52:35.251977728Z",
            "receivedAt":"2026-11-25T18:52:35.251977728Z"}"#;
        let record: ConversionRecord = serde_json::from_str(json).unwrap();
        assert_eq!(record.click_id.as_deref(), Some("C"));
        assert_eq!(record.route_filter, Some(vec!["r".to_string()]));
    }

    #[test]
    fn keys_follow_the_counting_mode() {
        let mut r = record("e");
        r.external_id = Some("o1".into());
        assert_eq!(r.dedupe_key(), "step:purchase:click:click");
        r.click_id = None;
        assert_eq!(r.dedupe_key(), "step:purchase:external:o1");
        r.external_id = None;
        assert_eq!(r.dedupe_key(), "step:purchase:event:e");

        let mut r = record("e");
        r.counting = "Every".into();
        assert_eq!(r.dedupe_key(), "step:purchase:event:e");
        r.external_id = Some("o1".into());
        assert_eq!(r.dedupe_key(), "step:purchase:external:o1");
        r.source = "browser".into();
        assert_eq!(r.dedupe_key(), "step:purchase:external:o1");
        r.test = true;
        assert_eq!(r.dedupe_key(), "test:step:purchase:external:o1");
        r.kind = "refund".into();
        r.refund_id = Some("f".into());
        assert_eq!(r.dedupe_key(), "test:refund:f");
    }

    #[test]
    fn earlier_receipt_wins_then_attribution() {
        let earlier = record("a");
        let mut later = record("b");
        later.received_at = earlier.received_at + chrono::Duration::milliseconds(1);
        assert!(earlier.version(false) > later.version(true));
        assert!(earlier.version(true) > earlier.version(false));
    }

    #[test]
    fn values_are_scaled_exactly() {
        for (value, expected) in [("49", 490_000), ("49.9", 499_000), ("0.0001", 1),
                                  ("99999999999999.9999", 999_999_999_999_999_999)] {
            assert_eq!(scaled(value).unwrap(), expected);
        }
        for value in ["", "-1", "1.00001", "x", "100000000000000", "1.-1"] {
            assert!(scaled(value).is_err(), "{value}");
        }
    }
}
