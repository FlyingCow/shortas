use serde::{Deserialize, Serialize};

/// A conversion definition, as stored by click-router-api.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Conversion {
    pub public_key: String,
    pub conversion_id: String,
    pub workspace_id: String,
    pub version: i64,
    /// Only active conversions accept live events.
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

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConversionStep {
    pub key: String,
    pub order: u32,
    pub is_final: bool,
    /// Decimal string.
    pub default_value: Option<String>,
}
