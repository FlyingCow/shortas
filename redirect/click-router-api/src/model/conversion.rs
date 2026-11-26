use salvo::oapi::ToSchema;
use serde::{Deserialize, Serialize};

/// A conversion definition as the router needs it.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct Conversion {
    pub public_key: String,
    pub conversion_id: String,
    pub workspace_id: String,
    /// Grows with every change. Older versions never replace newer ones.
    pub version: i64,
    pub active: bool,
    /// Hex SHA-256 of the secret key.
    pub secret_hash: String,
    pub currency: String,
    pub window_days: u32,
    pub counting: String,
    pub allowed_origins: Vec<String>,
    pub server_values_only: bool,
    pub steps: Vec<ConversionStep>,
    /// Route IDs, or `None` for every route in the workspace.
    pub route_filter: Option<Vec<String>>,
}

impl Conversion {
    /// Replaces a deleted conversion. No other version is newer, so a delayed
    /// update can't bring the conversion back.
    pub fn tombstone(public_key: String) -> Self {
        Self {
            public_key,
            version: i64::MAX,
            ..Default::default()
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct ConversionStep {
    pub key: String,
    pub order: u32,
    pub is_final: bool,
    /// Decimal string.
    pub default_value: Option<String>,
}
