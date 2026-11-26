//! Conversion events and refunds: reading and checking what clients send.

use chrono::{DateTime, Duration, Utc};
use serde::Serialize;
use serde_json::{Map, Value};
use ulid::Ulid;

use crate::model::{Conversion, ConversionStep};

pub const MAX_STEP_LENGTH: usize = 64;
const MAX_CLICK_ID_LENGTH: usize = 64;
const MAX_ID_LENGTH: usize = 128;
const MAX_AGE_DAYS: i64 = 90;
/// Clocks differ a little. Such times are stored as the time of receipt.
const CLOCK_SKEW_SECONDS: i64 = 60;
/// Values are stored as `Decimal(18, 4)`.
const MAX_INTEGER_DIGITS: usize = 14;
const MAX_FRACTION_DIGITS: usize = 4;

/// Active ISO 4217 currencies, as in the Management API.
const CURRENCIES: &str = "AED AFN ALL AMD AOA ARS AUD AWG AZN BAM BBD BDT BGN BHD BIF BMD BND \
BOB BRL BSD BTN BWP BYN BZD CAD CDF CHF CLP CNY COP CRC CUP CVE CZK DJF DKK DOP DZD EGP ERN ETB \
EUR FJD FKP GBP GEL GHS GIP GMD GNF GTQ GYD HKD HNL HTG HUF IDR ILS INR IQD IRR ISK JMD JOD JPY \
KES KGS KHR KMF KPW KRW KWD KYD KZT LAK LBP LKR LRD LSL LYD MAD MDL MGA MKD MMK MNT MOP MRU MUR \
MVR MWK MXN MYR MZN NAD NGN NIO NOK NPR NZD OMR PAB PEN PGK PHP PKR PLN PYG QAR RON RSD RUB RWF \
SAR SBD SCR SDG SEK SGD SHP SLE SOS SRD SSP STN SVC SYP SZL THB TJS TMT TND TOP TRY TTD TWD TZS \
UAH UGX USD UYU UZS VED VES VND VUV WST XAF XCD XCG XOF XPF YER ZAR ZMW ZWG";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum Source {
    Browser,
    Server,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum Kind {
    Event,
    Refund,
}

/// Why an event was refused.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Rejection {
    NotActive,
    OriginNotAllowed,
    Invalid(&'static str),
}

impl Rejection {
    pub fn reason(&self) -> &'static str {
        match self {
            Self::NotActive => "not_active",
            Self::OriginNotAllowed => "origin_not_allowed",
            Self::Invalid(_) => "invalid",
        }
    }

    pub fn field(&self) -> Option<&'static str> {
        match self {
            Self::Invalid(field) => Some(field),
            _ => None,
        }
    }
}

/// A record of the conversion stream. The definition's settings are included,
/// so later changes to it don't affect events already received.
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ConversionRecord {
    /// Unique per request.
    pub id: String,
    pub kind: Kind,
    /// Set on rejected events, which are only logged.
    pub rejection: Option<&'static str>,
    pub rejected_field: Option<&'static str>,
    pub conversion_id: String,
    pub workspace_id: String,
    pub source: Source,
    pub test: bool,
    /// Empty for refunds, and for events whose step could not be read.
    pub step: String,
    pub step_order: u32,
    pub is_final: bool,
    /// The client's, or else `id`.
    pub event_id: String,
    /// For refunds, the external ID of the refunded event.
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

/// Reads `body` as `kind`. The record is returned even when it is rejected, so
/// the event log can show it.
pub fn read(
    conversion: &Conversion,
    kind: Kind,
    source: Source,
    origin: Option<&str>,
    body: &Map<String, Value>,
    now: DateTime<Utc>,
) -> (ConversionRecord, Option<Rejection>) {
    let id = Ulid::new().to_string();
    let mut record = ConversionRecord {
        event_id: id.clone(),
        id,
        kind,
        rejection: None,
        rejected_field: None,
        conversion_id: conversion.conversion_id.clone(),
        workspace_id: conversion.workspace_id.clone(),
        source,
        test: false,
        step: String::new(),
        step_order: 0,
        is_final: false,
        external_id: None,
        refund_id: None,
        click_id: None,
        value: None,
        currency: conversion.currency.clone(),
        counting: conversion.counting.clone(),
        window_days: conversion.window_days,
        route_filter: conversion.route_filter.clone(),
        occurred_at: now,
        received_at: now,
    };

    let rejection = fill(&mut record, conversion, origin, body).err();
    if let Some(rejection) = &rejection {
        record.rejection = Some(rejection.reason());
        record.rejected_field = rejection.field();
        record.route_filter = None;
    }
    (record, rejection)
}

fn fill(
    record: &mut ConversionRecord,
    conversion: &Conversion,
    origin: Option<&str>,
    body: &Map<String, Value>,
) -> Result<(), Rejection> {
    record.test = match body.get("test") {
        None | Some(Value::Null) => false,
        Some(Value::Bool(test)) => *test,
        Some(_) => return Err(Rejection::Invalid("test")),
    };
    // Read before any check, so the log names the step that was refused.
    let step = text(body, "step", MAX_STEP_LENGTH);
    if let Ok(Some(step)) = &step {
        record.step = step.clone();
    }

    // Tests are never counted, so an install can be checked before activation.
    if !conversion.active && !record.test {
        return Err(Rejection::NotActive);
    }
    // Tests are allowed from anywhere too, so the dashboard can send them.
    if record.source == Source::Browser && !record.test && !origin_allowed(conversion, origin) {
        return Err(Rejection::OriginNotAllowed);
    }

    match record.kind {
        Kind::Event => {
            let found = step?
                .and_then(|key| conversion.steps.iter().find(|s| s.key == key))
                .ok_or(Rejection::Invalid("step"))?;
            fill_event(record, conversion, found, body)?;
        }
        Kind::Refund => {
            record.step.clear();
            record.refund_id =
                Some(text(body, "refundId", MAX_ID_LENGTH)?.ok_or(Rejection::Invalid("refundId"))?);
            record.external_id = Some(
                text(body, "externalId", MAX_ID_LENGTH)?.ok_or(Rejection::Invalid("externalId"))?,
            );
            let value = value(body)?.filter(|v| v.bytes().any(|b| matches!(b, b'1'..=b'9')));
            record.value = Some(value.ok_or(Rejection::Invalid("value"))?);
            if let Some(currency) = currency(body)? {
                record.currency = currency;
            }
        }
    }

    if record.source == Source::Server {
        record.occurred_at = occurred_at(body, record.received_at)?;
    }
    Ok(())
}

fn fill_event(
    record: &mut ConversionRecord,
    conversion: &Conversion,
    step: &ConversionStep,
    body: &Map<String, Value>,
) -> Result<(), Rejection> {
    record.step_order = step.order;
    record.is_final = step.is_final;
    if let Some(event_id) = text(body, "eventId", MAX_ID_LENGTH)? {
        record.event_id = event_id;
    }
    record.external_id = text(body, "externalId", MAX_ID_LENGTH)?;
    record.click_id = text(body, "clickId", MAX_CLICK_ID_LENGTH)?;

    let value_ignored = record.source == Source::Browser && conversion.server_values_only;
    let submitted = if value_ignored { None } else { value(body)? };
    match submitted {
        Some(value) => {
            record.value = Some(value);
            if let Some(currency) = currency(body)? {
                record.currency = currency;
            }
        }
        // Defaults are in the conversion's currency.
        None => {
            if !value_ignored {
                currency(body)?;
            }
            record.value = step.default_value.clone();
        }
    }
    Ok(())
}

fn origin_allowed(conversion: &Conversion, origin: Option<&str>) -> bool {
    conversion.allowed_origins.is_empty()
        || origin.is_some_and(|origin| conversion.allowed_origins.iter().any(|o| o == origin))
}

/// A non-empty string without control characters, or `None` when left out.
fn text(
    body: &Map<String, Value>,
    field: &'static str,
    max_length: usize,
) -> Result<Option<String>, Rejection> {
    match body.get(field) {
        None | Some(Value::Null) => Ok(None),
        Some(Value::String(s))
            if !s.is_empty() && s.len() <= max_length && !s.chars().any(char::is_control) =>
        {
            Ok(Some(s.clone()))
        }
        Some(_) => Err(Rejection::Invalid(field)),
    }
}

fn currency(body: &Map<String, Value>) -> Result<Option<String>, Rejection> {
    let currency = text(body, "currency", 3)?;
    match &currency {
        Some(c) if c.len() != 3 || !CURRENCIES.split(' ').any(|known| known == c) => {
            Err(Rejection::Invalid("currency"))
        }
        _ => Ok(currency),
    }
}

fn value(body: &Map<String, Value>) -> Result<Option<String>, Rejection> {
    let value = match body.get("value") {
        None | Some(Value::Null) => return Ok(None),
        // Rounded, because sums in JavaScript are not exact: 0.1 + 0.2
        Some(Value::Number(n)) => n
            .as_f64()
            .filter(|n| *n >= 0.0)
            .map(|n| format!("{:.*}", MAX_FRACTION_DIGITS, n.abs())),
        Some(Value::String(s)) => Some(s.clone()),
        Some(_) => None,
    };
    value
        .and_then(|v| decimal(&v))
        .map(Some)
        .ok_or(Rejection::Invalid("value"))
}

/// A plain non-negative decimal that fits, without needless zeros.
fn decimal(s: &str) -> Option<String> {
    let (integer, fraction) = match s.split_once('.') {
        Some((_, "")) => return None,
        Some(parts) => parts,
        None => (s, ""),
    };
    let digits = |s: &str| s.bytes().all(|b| b.is_ascii_digit());
    if integer.is_empty() || !digits(integer) || !digits(fraction) {
        return None;
    }
    let integer = integer.trim_start_matches('0');
    let fraction = fraction.trim_end_matches('0');
    if integer.len() > MAX_INTEGER_DIGITS || fraction.len() > MAX_FRACTION_DIGITS {
        return None;
    }
    let integer = if integer.is_empty() { "0" } else { integer };
    Some(if fraction.is_empty() {
        integer.to_string()
    } else {
        format!("{integer}.{fraction}")
    })
}

fn occurred_at(
    body: &Map<String, Value>,
    now: DateTime<Utc>,
) -> Result<DateTime<Utc>, Rejection> {
    let Some(text) = text(body, "occurredAt", MAX_ID_LENGTH)? else {
        return Ok(now);
    };
    let at = DateTime::parse_from_rfc3339(&text)
        .map_err(|_| Rejection::Invalid("occurredAt"))?
        .with_timezone(&Utc);
    if at > now + Duration::seconds(CLOCK_SKEW_SECONDS) || at < now - Duration::days(MAX_AGE_DAYS) {
        return Err(Rejection::Invalid("occurredAt"));
    }
    Ok(at.min(now))
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use serde_json::json;

    pub(crate) fn conversion() -> Conversion {
        let step = |key: &str, order, is_final, default_value: Option<&str>| ConversionStep {
            key: key.into(),
            order,
            is_final,
            default_value: default_value.map(Into::into),
        };
        Conversion {
            public_key: "pk_abc".into(),
            conversion_id: "conversion".into(),
            workspace_id: "workspace".into(),
            version: 1,
            active: true,
            secret_hash: String::new(),
            currency: "EUR".into(),
            window_days: 30,
            counting: "Every".into(),
            allowed_origins: vec![],
            server_values_only: false,
            steps: vec![step("cart", 0, false, Some("5")), step("purchase", 1, true, None)],
            route_filter: Some(vec!["r1".into()]),
        }
    }

    fn read_as(
        conversion: &Conversion,
        kind: Kind,
        source: Source,
        body: Value,
    ) -> (ConversionRecord, Option<Rejection>) {
        let body = body.as_object().unwrap();
        read(conversion, kind, source, Some("https://shop.example"), body, Utc::now())
    }

    fn event(source: Source, body: Value) -> (ConversionRecord, Option<Rejection>) {
        read_as(&conversion(), Kind::Event, source, body)
    }

    fn accepted(source: Source, body: Value) -> ConversionRecord {
        let (record, rejection) = event(source, body);
        assert_eq!(rejection, None);
        record
    }

    fn invalid_field(body: Value) -> &'static str {
        let (record, rejection) = event(Source::Server, body);
        assert_eq!(record.rejection, Some("invalid"));
        rejection.unwrap().field().unwrap()
    }

    #[test]
    fn reads_an_event_with_the_settings_it_was_received_under() {
        let record = accepted(
            Source::Browser,
            json!({"step": "purchase", "clickId": "C", "externalId": "o-1", "eventId": "e-1",
                   "value": 49.9, "currency": "USD"}),
        );
        assert_eq!((record.step_order, record.is_final), (1, true));
        assert_eq!(record.click_id.as_deref(), Some("C"));
        assert_eq!(record.external_id.as_deref(), Some("o-1"));
        assert_eq!(record.event_id, "e-1");
        assert_eq!(record.value.as_deref(), Some("49.9"));
        assert_eq!(record.currency, "USD");
        assert_eq!((record.counting.as_str(), record.window_days), ("Every", 30));
        assert_eq!(record.route_filter, Some(vec!["r1".to_string()]));
    }

    #[test]
    fn generates_an_event_id_when_there_is_none() {
        let record = accepted(Source::Server, json!({"step": "cart"}));
        assert_eq!(record.event_id, record.id);
    }

    #[test]
    fn missing_values_use_the_default_in_the_conversion_currency() {
        let record = accepted(Source::Server, json!({"step": "cart", "currency": "USD"}));
        assert_eq!(record.value.as_deref(), Some("5"));
        assert_eq!(record.currency, "EUR");
        assert_eq!(accepted(Source::Server, json!({"step": "purchase"})).value, None);
        let zero = accepted(Source::Server, json!({"step": "cart", "value": 0}));
        assert_eq!(zero.value.as_deref(), Some("0"));
    }

    #[test]
    fn values_are_exact_decimals() {
        for (value, expected) in [
            (json!(49), "49"),
            (json!(0.1 + 0.2), "0.3"),
            (json!(1e-5), "0"),
            (json!("007.5000"), "7.5"),
            (json!("99999999999999.9999"), "99999999999999.9999"),
        ] {
            let record = accepted(Source::Server, json!({"step": "cart", "value": value}));
            assert_eq!(record.value.as_deref(), Some(expected));
        }
        for value in [
            json!(-1), json!(-0.00001), json!("-1"), json!("NaN"), json!("Infinity"), json!(1e14),
            json!(1e300), json!("1.00001"), json!("100000000000000"), json!(""), json!("."),
            json!("1."), json!(".5"), json!("1e3"), json!("1,5"), json!(true), json!([1]),
        ] {
            assert_eq!(invalid_field(json!({"step": "cart", "value": value})), "value", "{value}");
        }
    }

    #[test]
    fn names_the_invalid_field() {
        assert_eq!(invalid_field(json!({})), "step");
        assert_eq!(invalid_field(json!({"step": "nope"})), "step");
        assert_eq!(invalid_field(json!({"step": 1})), "step");
        for currency in [json!("XXX"), json!("eur"), json!(""), json!("R U"), json!(1)] {
            assert_eq!(invalid_field(json!({"step": "cart", "currency": currency})), "currency");
        }
        for (field, max) in [("eventId", 128), ("externalId", 128), ("clickId", 64)] {
            for value in [json!(""), json!("a\nb"), json!(5), json!("x".repeat(max + 1))] {
                assert_eq!(invalid_field(json!({"step": "cart", field: value})), field);
            }
        }
        assert_eq!(invalid_field(json!({"step": "cart", "test": "yes"})), "test");
    }

    #[test]
    fn servers_may_state_a_recent_past_time() {
        let now = Utc::now();
        let at = |seconds: i64| (now + Duration::seconds(seconds)).to_rfc3339();
        let body = |at: String| json!({"step": "cart", "occurredAt": at});
        let day = 24 * 3600;

        let record = accepted(Source::Server, body(at(-89 * day)));
        assert!(record.occurred_at < now - Duration::days(88));
        // Small clock differences
        let record = accepted(Source::Server, body(at(30)));
        assert_eq!(record.occurred_at, record.received_at);

        for at in [at(120), at(-91 * day), "yesterday".into()] {
            assert_eq!(invalid_field(body(at)), "occurredAt");
        }
        // Browsers can't choose
        let record = accepted(Source::Browser, body(at(-80 * day)));
        assert_eq!(record.occurred_at, record.received_at);
    }

    #[test]
    fn origins_limit_browsers_only() {
        let mut c = conversion();
        c.allowed_origins = vec!["https://other.example".into()];
        let body = || json!({"step": "cart"});
        let (record, rejection) = read_as(&c, Kind::Event, Source::Browser, body());
        assert_eq!(rejection, Some(Rejection::OriginNotAllowed));
        assert_eq!((record.step.as_str(), record.rejection), ("cart", Some("origin_not_allowed")));
        assert_eq!(read_as(&c, Kind::Event, Source::Server, body()).1, None);

        c.allowed_origins.push("https://shop.example".into());
        assert_eq!(read_as(&c, Kind::Event, Source::Browser, body()).1, None);
        let body = body();
        let (_, rejection) =
            read(&c, Kind::Event, Source::Browser, None, body.as_object().unwrap(), Utc::now());
        assert_eq!(rejection, Some(Rejection::OriginNotAllowed));

        let test = json!({"step": "cart", "test": true});
        let (record, rejection) =
            read(&c, Kind::Event, Source::Browser, None, test.as_object().unwrap(), Utc::now());
        assert_eq!((record.test, rejection), (true, None));
    }

    #[test]
    fn server_only_values_ignore_the_browser() {
        let mut c = conversion();
        c.server_values_only = true;
        let body = || json!({"step": "cart", "value": 1000, "currency": "USD"});
        let (record, rejection) = read_as(&c, Kind::Event, Source::Browser, body());
        assert_eq!(rejection, None);
        assert_eq!((record.value.as_deref(), record.currency.as_str()), (Some("5"), "EUR"));
        let (record, _) = read_as(&c, Kind::Event, Source::Server, body());
        assert_eq!((record.value.as_deref(), record.currency.as_str()), (Some("1000"), "USD"));
    }

    #[test]
    fn inactive_conversions_accept_only_tests() {
        let mut c = conversion();
        c.active = false;
        let (_, rejection) = read_as(&c, Kind::Event, Source::Server, json!({"step": "cart"}));
        assert_eq!(rejection, Some(Rejection::NotActive));
        let body = json!({"step": "cart", "test": true});
        let (record, rejection) = read_as(&c, Kind::Event, Source::Server, body);
        assert_eq!((record.test, rejection), (true, None));
    }

    #[test]
    fn refunds_need_ids_and_an_amount() {
        let refund = |body| read_as(&conversion(), Kind::Refund, Source::Server, body);
        let (record, rejection) =
            refund(json!({"refundId": "f-1", "externalId": "o-1", "value": "20", "step": "cart"}));
        assert_eq!(rejection, None);
        assert_eq!(record.refund_id.as_deref(), Some("f-1"));
        assert_eq!(record.external_id.as_deref(), Some("o-1"));
        assert_eq!((record.value.as_deref(), record.currency.as_str()), (Some("20"), "EUR"));
        assert_eq!(record.step, "");

        for (body, field) in [
            (json!({"externalId": "o", "value": 1}), "refundId"),
            (json!({"refundId": "f", "value": 1}), "externalId"),
            (json!({"refundId": "f", "externalId": "o"}), "value"),
            (json!({"refundId": "f", "externalId": "o", "value": "0.00"}), "value"),
            (json!({"refundId": "f", "externalId": "o", "value": -1}), "value"),
        ] {
            assert_eq!(refund(body).1, Some(Rejection::Invalid(field)));
        }
    }
}
