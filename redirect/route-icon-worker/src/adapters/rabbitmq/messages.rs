use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ChangeAction {
    Created,
    Updated,
    Deleted,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RouteChangedMessage {
    pub route_id: String,
    pub action: ChangeAction,
    pub public: RouteDto,
    #[serde(default)]
    pub private: RouteChangedPrivate,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RouteChangedPrivate {
    pub previous_dest: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RouteDto {
    pub switch: String,
    pub link: String,
    pub dest: Option<String>,
    pub dest_format: String,
    pub code: Option<u16>,
    pub ttl: Option<u64>,
    pub status: String,
    pub terminal: String,
    #[serde(default)]
    pub policy: Value,
    pub properties: RoutePropertiesDto,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct RoutePropertiesDto {
    pub route_id: Option<String>,
    pub domain_id: Option<String>,
    pub owner_id: Option<String>,
    pub creator_id: Option<String>,
    pub workspace_id: Option<String>,
    pub scripts: Option<Vec<String>>,
    pub tags: Option<Vec<String>>,
    pub custom: Option<Value>,
    pub native: Option<Value>,
    pub bundling: Option<Value>,
    #[serde(default)]
    pub opengraph: bool,
    #[serde(default)]
    pub allow_debug: bool,
}

impl RouteChangedMessage {
    pub fn owner_id(&self) -> Option<&str> {
        self.public.properties.owner_id.as_deref()
    }

    pub fn dest(&self) -> Option<&str> {
        self.public.dest.as_deref()
    }

    pub fn previous_dest(&self) -> Option<&str> {
        self.private.previous_dest.as_deref()
    }
}
