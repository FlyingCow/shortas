//! Conversion definitions.

use chrono::{DateTime, Utc};
use rand::{distr::Alphanumeric, Rng};
use rust_decimal::Decimal;
use salvo::oapi::ToSchema;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::HashSet;
use uuid::Uuid;

use super::error::{ApiError, ErrorCode, Result};

pub const MAX_STEPS: usize = 20;
const MAX_NAME_LENGTH: usize = 100;
const MAX_KEY_LENGTH: usize = 64;
const MAX_ORIGINS: usize = 20;
const MAX_FILTERED_ROUTES: usize = 1000;

/// Active ISO 4217 currencies.
const CURRENCIES: &str = "AED AFN ALL AMD AOA ARS AUD AWG AZN BAM BBD BDT BGN BHD BIF BMD BND \
BOB BRL BSD BTN BWP BYN BZD CAD CDF CHF CLP CNY COP CRC CUP CVE CZK DJF DKK DOP DZD EGP ERN ETB \
EUR FJD FKP GBP GEL GHS GIP GMD GNF GTQ GYD HKD HNL HTG HUF IDR ILS INR IQD IRR ISK JMD JOD JPY \
KES KGS KHR KMF KPW KRW KWD KYD KZT LAK LBP LKR LRD LSL LYD MAD MDL MGA MKD MMK MNT MOP MRU MUR \
MVR MWK MXN MYR MZN NAD NGN NIO NOK NPR NZD OMR PAB PEN PGK PHP PKR PLN PYG QAR RON RSD RUB RWF \
SAR SBD SCR SDG SEK SGD SHP SLE SOS SRD SSP STN SVC SYP SZL THB TJS TMT TND TOP TRY TTD TWD TZS \
UAH UGX USD UYU UZS VED VES VND VUV WST XAF XCD XCG XOF XPF YER ZAR ZMW ZWG";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
pub enum ConversionStatus {
    Draft,
    Activating,
    Active,
    Paused,
    Archived,
}

impl ConversionStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Draft => "Draft",
            Self::Activating => "Activating",
            Self::Active => "Active",
            Self::Paused => "Paused",
            Self::Archived => "Archived",
        }
    }

    pub fn parse(s: &str) -> Option<Self> {
        Some(match s {
            "Draft" => Self::Draft,
            "Activating" => Self::Activating,
            "Active" => Self::Active,
            "Paused" => Self::Paused,
            "Archived" => Self::Archived,
            _ => return None,
        })
    }

    /// Whether routes covered by the conversion should get a click ID.
    pub fn covers_routes(&self) -> bool {
        matches!(self, Self::Activating | Self::Active)
    }

    fn can_become(&self, next: Self) -> bool {
        use ConversionStatus::*;
        matches!(
            (self, next),
            (Draft | Paused, Activating)
                | (Activating, Active)
                | (Activating | Active, Paused)
                | (Draft | Activating | Active | Paused, Archived)
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
pub enum CountingMode {
    /// A step counts once per click.
    OncePerClick,
    /// Every distinct event counts.
    Every,
}

impl CountingMode {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::OncePerClick => "OncePerClick",
            Self::Every => "Every",
        }
    }

    pub fn parse(s: &str) -> Option<Self> {
        match s {
            "OncePerClick" => Some(Self::OncePerClick),
            "Every" => Some(Self::Every),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ConversionStep {
    pub key: String,
    pub name: String,
    pub default_value: Option<Decimal>,
    pub archived: bool,
}

/// The part of a conversion its users edit.
#[derive(Debug, Clone, PartialEq)]
pub struct ConversionDefinition {
    pub name: String,
    pub currency: String,
    pub window_days: i32,
    pub counting: CountingMode,
    pub domain_id: Option<Uuid>,
    pub allowed_origins: Vec<String>,
    pub server_values_only: bool,
    /// Active steps in order. The last one is the conversion.
    pub steps: Vec<ConversionStep>,
    /// `None` covers every route in the workspace.
    pub route_filter: Option<Vec<Uuid>>,
}

impl ConversionDefinition {
    /// Checks everything that needs no lookup and names each invalid field.
    pub fn validate(&self) -> Result<()> {
        let mut invalid = Vec::new();
        let mut check = |ok: bool, field: String| {
            if !ok {
                invalid.push(field);
            }
        };

        check(valid_name(&self.name), "name".into());
        check(
            self.currency.len() == 3 && CURRENCIES.split(' ').any(|c| c == self.currency),
            "currency".into(),
        );
        check((1..=90).contains(&self.window_days), "windowDays".into());
        check(
            self.allowed_origins.len() <= MAX_ORIGINS
                && self.allowed_origins.iter().all(|o| valid_origin(o)),
            "allowedOrigins".into(),
        );
        check(
            self.route_filter
                .as_ref()
                .is_none_or(|r| !r.is_empty() && r.len() <= MAX_FILTERED_ROUTES),
            "routeIds".into(),
        );
        check((1..=MAX_STEPS).contains(&self.steps.len()), "steps".into());

        let mut keys = HashSet::new();
        for (i, step) in self.steps.iter().enumerate() {
            check(
                valid_step_key(&step.key) && keys.insert(step.key.as_str()),
                format!("steps[{i}].key"),
            );
            check(valid_name(&step.name), format!("steps[{i}].name"));
            check(
                step.default_value.is_none_or(valid_value),
                format!("steps[{i}].defaultValue"),
            );
        }

        if invalid.is_empty() {
            return Ok(());
        }
        Err(ApiError::with_details(
            ErrorCode::ValidationError,
            format!("Invalid fields: {}", invalid.join(", ")),
            invalid.join(","),
        ))
    }
}

fn valid_name(name: &str) -> bool {
    !name.trim().is_empty() && name.chars().count() <= MAX_NAME_LENGTH
}

fn valid_step_key(key: &str) -> bool {
    (1..=MAX_KEY_LENGTH).contains(&key.len())
        && key
            .bytes()
            .all(|b| b.is_ascii_lowercase() || b.is_ascii_digit() || b == b'_' || b == b'-')
}

/// Fits `NUMERIC(18, 4)` without rounding.
fn valid_value(value: Decimal) -> bool {
    !value.is_sign_negative()
        && value.normalize().scale() <= 4
        && value < Decimal::from(100_000_000_000_000_i64)
}

/// A bare web origin, spelled the way browsers send it.
fn valid_origin(origin: &str) -> bool {
    url::Url::parse(origin).is_ok_and(|url| {
        matches!(url.scheme(), "http" | "https") && url.origin().ascii_serialization() == origin
    })
}

#[derive(Debug, Clone, PartialEq)]
pub struct Conversion {
    pub id: Uuid,
    pub workspace_id: Uuid,
    pub definition: ConversionDefinition,
    /// Removed steps whose events are kept.
    pub archived_steps: Vec<ConversionStep>,
    pub status: ConversionStatus,
    pub public_key: String,
    pub secret_hash: String,
    pub version: i64,
    pub created_by: String,
    pub activated_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Conversion {
    /// A new draft with its secret key, which is not kept anywhere else.
    pub fn new(
        workspace_id: Uuid,
        definition: ConversionDefinition,
        created_by: String,
    ) -> (Self, String) {
        let public_key = format!("pk_{}", random_string(24));
        let (secret, secret_hash) = new_secret(&public_key);
        let now = Utc::now();
        let conversion = Self {
            id: Uuid::new_v4(),
            workspace_id,
            definition,
            archived_steps: Vec::new(),
            status: ConversionStatus::Draft,
            public_key,
            secret_hash,
            version: 1,
            created_by,
            activated_at: None,
            created_at: now,
            updated_at: now,
        };
        (conversion, secret)
    }

    /// Replaces the definition. Steps that may have events are archived
    /// rather than dropped, and come back when their key is used again.
    pub fn redefine(&mut self, definition: ConversionDefinition) {
        let kept: HashSet<&str> = definition.steps.iter().map(|s| s.key.as_str()).collect();
        self.archived_steps.retain(|s| !kept.contains(s.key.as_str()));

        if self.activated_at.is_some() {
            let removed = self
                .definition
                .steps
                .iter()
                .filter(|s| !kept.contains(s.key.as_str()))
                .map(|s| ConversionStep {
                    archived: true,
                    ..s.clone()
                });
            self.archived_steps.extend(removed);
        }
        self.definition = definition;
    }

    pub fn transition(&mut self, next: ConversionStatus) -> Result<()> {
        if !self.status.can_become(next) {
            return Err(ApiError::conflict(format!(
                "A conversion that is {} cannot become {}",
                self.status.as_str(),
                next.as_str()
            )));
        }
        self.status = next;
        if next == ConversionStatus::Activating && self.activated_at.is_none() {
            self.activated_at = Some(Utc::now());
        }
        Ok(())
    }

    /// Replaces the secret key and returns the new one.
    pub fn rotate_secret(&mut self) -> String {
        let (secret, hash) = new_secret(&self.public_key);
        self.secret_hash = hash;
        secret
    }
}

/// A secret key and its hash. The key contains the public key's ID, so a
/// server request finds its conversion without a second index.
fn new_secret(public_key: &str) -> (String, String) {
    let id = public_key.trim_start_matches("pk_");
    let key = format!("sk_{id}_{}", random_string(43));
    let hash = format!("{:x}", Sha256::digest(key.as_bytes()));
    (key, hash)
}

/// Alphanumeric characters from a cryptographically secure generator.
fn random_string(length: usize) -> String {
    rand::rng()
        .sample_iter(Alphanumeric)
        .take(length)
        .map(char::from)
        .collect()
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;

    pub(crate) fn step(key: &str) -> ConversionStep {
        ConversionStep {
            key: key.into(),
            name: key.into(),
            default_value: None,
            archived: false,
        }
    }

    pub(crate) fn definition() -> ConversionDefinition {
        ConversionDefinition {
            name: "Purchase".into(),
            currency: "EUR".into(),
            window_days: 30,
            counting: CountingMode::OncePerClick,
            domain_id: None,
            allowed_origins: vec!["https://shop.example".into()],
            server_values_only: false,
            steps: vec![step("cart"), step("purchase")],
            route_filter: None,
        }
    }

    fn invalid_fields(change: impl FnOnce(&mut ConversionDefinition)) -> String {
        let mut d = definition();
        change(&mut d);
        d.validate().unwrap_err().details.unwrap()
    }

    fn decimal(s: &str) -> Decimal {
        s.parse().unwrap()
    }

    #[test]
    fn accepts_valid_definitions() {
        definition().validate().unwrap();
        let mut d = definition();
        d.steps = (0..20).map(|i| step(&format!("s{i}"))).collect();
        d.steps[0].default_value = Some(decimal("99999999999999.9999"));
        d.window_days = 90;
        d.allowed_origins = vec!["http://localhost:3000".into()];
        d.route_filter = Some(vec![Uuid::new_v4()]);
        d.validate().unwrap();
    }

    #[test]
    fn names_each_invalid_field() {
        assert_eq!(invalid_fields(|d| d.steps.clear()), "steps");
        assert_eq!(
            invalid_fields(|d| d.steps = (0..21).map(|i| step(&format!("s{i}"))).collect()),
            "steps"
        );
        assert_eq!(
            invalid_fields(|d| d.steps = vec![step("a"), step("a")]),
            "steps[1].key"
        );
        assert_eq!(invalid_fields(|d| d.steps[0].key = "Bad key".into()), "steps[0].key");
        assert_eq!(invalid_fields(|d| d.steps[0].name = " ".into()), "steps[0].name");
        for currency in ["XXX", "eur", "", "EUR USD"] {
            assert_eq!(invalid_fields(|d| d.currency = currency.into()), "currency");
        }
        for days in [0, -1, 91] {
            assert_eq!(invalid_fields(|d| d.window_days = days), "windowDays");
        }
        for value in ["-1", "1.00001", "100000000000000"] {
            assert_eq!(
                invalid_fields(|d| d.steps[0].default_value = Some(decimal(value))),
                "steps[0].defaultValue"
            );
        }
        for origin in ["https://a.com/", "https://a.com/p", "a.com", "ftp://a.com", "*", "https://A.com"] {
            assert_eq!(
                invalid_fields(|d| d.allowed_origins = vec![origin.into()]),
                "allowedOrigins"
            );
        }
        assert_eq!(invalid_fields(|d| d.route_filter = Some(vec![])), "routeIds");
        assert_eq!(
            invalid_fields(|d| {
                d.name.clear();
                d.window_days = 0;
            }),
            "name,windowDays"
        );
    }

    #[test]
    fn secret_is_returned_once_and_only_its_hash_is_kept() {
        let (mut c, secret) = Conversion::new(Uuid::new_v4(), definition(), "u".into());
        assert!(secret.starts_with(&format!("sk_{}_", &c.public_key[3..])));
        assert_eq!(c.secret_hash, format!("{:x}", Sha256::digest(secret.as_bytes())));
        assert!(!format!("{c:?}").contains(&secret));

        let old_hash = c.secret_hash.clone();
        let rotated = c.rotate_secret();
        assert_ne!(rotated, secret);
        assert_ne!(c.secret_hash, old_hash);
    }

    #[test]
    fn removed_steps_are_archived_only_after_activation() {
        let (mut c, _) = Conversion::new(Uuid::new_v4(), definition(), "u".into());
        let without_cart = || ConversionDefinition {
            steps: vec![step("purchase")],
            ..definition()
        };

        c.redefine(without_cart());
        assert!(c.archived_steps.is_empty());

        c.redefine(definition());
        c.transition(ConversionStatus::Activating).unwrap();
        c.redefine(without_cart());
        assert_eq!(c.archived_steps.len(), 1);
        assert!(c.archived_steps[0].archived);

        c.redefine(definition());
        assert!(c.archived_steps.is_empty());
        assert!(c.definition.steps.iter().all(|s| !s.archived));
    }

    #[test]
    fn lifecycle_allows_only_listed_transitions() {
        use ConversionStatus::*;
        let allowed = [
            (Draft, Activating), (Paused, Activating), (Activating, Active), (Activating, Paused),
            (Active, Paused), (Draft, Archived), (Activating, Archived), (Active, Archived),
            (Paused, Archived),
        ];
        let all = [Draft, Activating, Active, Paused, Archived];
        for from in all {
            assert_eq!(ConversionStatus::parse(from.as_str()), Some(from));
            for to in all {
                let (mut c, _) = Conversion::new(Uuid::new_v4(), definition(), "u".into());
                c.status = from;
                assert_eq!(c.transition(to).is_ok(), allowed.contains(&(from, to)), "{from:?}->{to:?}");
            }
        }
    }
}
