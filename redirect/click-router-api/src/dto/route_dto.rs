use serde::{Deserialize, Serialize};
use salvo::oapi::ToSchema;
use serde_json::Value;

/// Route DTO for API responses
/// 
/// This DTO provides a clean API interface for routes,
/// simplifying complex nested structures for better API usability.
#[derive(Serialize, Deserialize, Debug, Clone, ToSchema)]
pub struct RouteDto {
    /// Route switch identifier
    pub switch: String,
    /// Route link/URL
    pub link: String,
    /// Destination URL (optional)
    pub dest: Option<String>,
    /// Destination format
    pub dest_format: String,
    /// HTTP status code (optional)
    pub code: Option<u16>,
    /// Time to live in seconds (optional)
    pub ttl: Option<u128>,
    /// Route status
    pub status: String,
    /// Routing terminal type
    pub terminal: String,
    /// Route properties
    pub properties: RoutePropertiesDto,
}

/// Route properties DTO for API responses
#[derive(Serialize, Deserialize, Debug, Clone, ToSchema)]
pub struct RoutePropertiesDto {
    /// Route ID (optional)
    pub route_id: Option<String>,
    /// Domain ID (optional)
    pub domain_id: Option<String>,
    /// Owner ID (optional)
    pub owner_id: Option<String>,
    /// Creator ID (optional)
    pub creator_id: Option<String>,
    /// Workspace ID (optional)
    pub workspace_id: Option<String>,
    /// Scripts (optional)
    pub scripts: Option<Vec<String>>,
    /// Tags (optional)
    pub tags: Option<Vec<String>>,
    /// Custom properties (optional)
    pub custom: Option<Value>,
    /// Native properties (optional)
    pub native: Option<Value>,
    /// Bundling properties (optional)
    pub bundling: Option<Value>,
    /// OpenGraph enabled
    pub opengraph: bool,
    /// Debug allowed
    pub allow_debug: bool,
}

impl RouteDto {
    /// Create a new RouteDto
    pub fn new(
        switch: String,
        link: String,
        dest: Option<String>,
        dest_format: String,
        code: Option<u16>,
        ttl: Option<u128>,
        status: String,
        terminal: String,
        properties: RoutePropertiesDto,
    ) -> Self {
        Self {
            switch,
            link,
            dest,
            dest_format,
            code,
            ttl,
            status,
            terminal,
            properties,
        }
    }

    /// Create a RouteDto with default values
    pub fn default() -> Self {
        Self {
            switch: String::new(),
            link: String::new(),
            dest: None,
            dest_format: "Http".to_string(),
            code: None,
            ttl: None,
            status: "Active".to_string(),
            terminal: "External".to_string(),
            properties: RoutePropertiesDto::default(),
        }
    }

    /// Builder method for switch
    pub fn switch(mut self, switch: String) -> Self {
        self.switch = switch;
        self
    }

    /// Builder method for link
    pub fn link(mut self, link: String) -> Self {
        self.link = link;
        self
    }

    /// Builder method for dest
    pub fn dest(mut self, dest: Option<String>) -> Self {
        self.dest = dest;
        self
    }

    /// Builder method for dest_format
    pub fn dest_format(mut self, dest_format: String) -> Self {
        self.dest_format = dest_format;
        self
    }

    /// Builder method for code
    pub fn code(mut self, code: Option<u16>) -> Self {
        self.code = code;
        self
    }

    /// Builder method for ttl
    pub fn ttl(mut self, ttl: Option<u128>) -> Self {
        self.ttl = ttl;
        self
    }

    /// Builder method for status
    pub fn status(mut self, status: String) -> Self {
        self.status = status;
        self
    }

    /// Builder method for terminal
    pub fn terminal(mut self, terminal: String) -> Self {
        self.terminal = terminal;
        self
    }

    /// Builder method for properties
    pub fn properties(mut self, properties: RoutePropertiesDto) -> Self {
        self.properties = properties;
        self
    }

    /// Check if the DTO is valid
    pub fn is_valid(&self) -> bool {
        !self.switch.is_empty() && !self.link.is_empty()
    }

    /// Get the switch value
    pub fn get_switch(&self) -> &str {
        &self.switch
    }

    /// Get the link value
    pub fn get_link(&self) -> &str {
        &self.link
    }

    /// Check if destination is set
    pub fn has_destination(&self) -> bool {
        self.dest.is_some()
    }

    /// Get the destination value
    pub fn get_destination(&self) -> Option<&String> {
        self.dest.as_ref()
    }

    /// Check if status is active
    pub fn is_active(&self) -> bool {
        self.status == "Active"
    }

    /// Check if status is blocked
    pub fn is_blocked(&self) -> bool {
        self.status.starts_with("Blocked")
    }

    /// Get the terminal type
    pub fn get_terminal(&self) -> &str {
        &self.terminal
    }

    /// Check if terminal is external
    pub fn is_external(&self) -> bool {
        self.terminal == "External"
    }

    /// Check if terminal is internal
    pub fn is_internal(&self) -> bool {
        self.terminal == "Internal"
    }

    /// Check if terminal is middleware
    pub fn is_middleware(&self) -> bool {
        self.terminal == "Middleware"
    }
}

impl Default for RouteDto {
    fn default() -> Self {
        Self::default()
    }
}

impl Default for RoutePropertiesDto {
    fn default() -> Self {
        Self {
            route_id: None,
            domain_id: None,
            owner_id: None,
            creator_id: None,
            workspace_id: None,
            scripts: None,
            tags: None,
            custom: None,
            native: None,
            bundling: None,
            opengraph: false,
            allow_debug: false,
        }
    }
}

/// Conversion from Route to RouteDto
impl From<crate::model::route::Route> for RouteDto {
    fn from(route: crate::model::route::Route) -> Self {
        Self {
            switch: route.switch,
            link: route.link,
            dest: route.dest,
            dest_format: match route.dest_format {
                crate::model::route::DestinationFormat::Http => "Http".to_string(),
                crate::model::route::DestinationFormat::Native => "Native".to_string(),
            },
            code: route.code,
            ttl: route.ttl,
            status: match &route.status {
                crate::model::route::RouteStatus::Active => "Active".to_string(),
                crate::model::route::RouteStatus::Blocked(reason) => {
                    match reason {
                        crate::model::route::BlockedReason::Resoned(msg) => format!("Blocked: {}", msg),
                        crate::model::route::BlockedReason::Unknown => "Blocked: Unknown".to_string(),
                    }
                }
            },
            terminal: match route.terminal {
                crate::model::route::RoutingTerminal::External => "External".to_string(),
                crate::model::route::RoutingTerminal::Internal => "Internal".to_string(),
                crate::model::route::RoutingTerminal::Middleware => "Middleware".to_string(),
            },
            properties: RoutePropertiesDto {
                route_id: route.properties.route_id,
                domain_id: route.properties.domain_id,
                owner_id: route.properties.owner_id,
                creator_id: route.properties.creator_id,
                workspace_id: route.properties.workspace_id,
                scripts: route.properties.scripts,
                tags: route.properties.tags,
                custom: route.properties.custom,
                native: route.properties.native,
                bundling: route.properties.bundling,
                opengraph: route.properties.opengraph,
                allow_debug: route.properties.allow_debug,
            },
        }
    }
}

/// Conversion from &Route to RouteDto
impl From<&crate::model::route::Route> for RouteDto {
    fn from(route: &crate::model::route::Route) -> Self {
        Self {
            switch: route.switch.clone(),
            link: route.link.clone(),
            dest: route.dest.clone(),
            dest_format: match route.dest_format {
                crate::model::route::DestinationFormat::Http => "Http".to_string(),
                crate::model::route::DestinationFormat::Native => "Native".to_string(),
            },
            code: route.code,
            ttl: route.ttl,
            status: match &route.status {
                crate::model::route::RouteStatus::Active => "Active".to_string(),
                crate::model::route::RouteStatus::Blocked(reason) => {
                    match reason {
                        crate::model::route::BlockedReason::Resoned(msg) => format!("Blocked: {}", msg),
                        crate::model::route::BlockedReason::Unknown => "Blocked: Unknown".to_string(),
                    }
                }
            },
            terminal: match route.terminal {
                crate::model::route::RoutingTerminal::External => "External".to_string(),
                crate::model::route::RoutingTerminal::Internal => "Internal".to_string(),
                crate::model::route::RoutingTerminal::Middleware => "Middleware".to_string(),
            },
            properties: RoutePropertiesDto {
                route_id: route.properties.route_id.clone(),
                domain_id: route.properties.domain_id.clone(),
                owner_id: route.properties.owner_id.clone(),
                creator_id: route.properties.creator_id.clone(),
                workspace_id: route.properties.workspace_id.clone(),
                scripts: route.properties.scripts.clone(),
                tags: route.properties.tags.clone(),
                custom: route.properties.custom.clone(),
                native: route.properties.native.clone(),
                bundling: route.properties.bundling.clone(),
                opengraph: route.properties.opengraph,
                allow_debug: route.properties.allow_debug,
            },
        }
    }
}

/// Conversion from RouteDto to Route
impl Into<crate::model::route::Route> for RouteDto {
    fn into(self) -> crate::model::route::Route {
        use crate::model::route::{Route, RouteProperties, DestinationFormat, RouteStatus, RoutingTerminal, BlockedReason};
        
        Route {
            switch: self.switch,
            link: self.link,
            dest: self.dest,
            dest_format: match self.dest_format.as_str() {
                "Native" => DestinationFormat::Native,
                _ => DestinationFormat::Http,
            },
            code: self.code,
            ttl: self.ttl,
            status: match self.status.as_str() {
                "Active" => RouteStatus::Active,
                status if status.starts_with("Blocked:") => {
                    let reason = status.strip_prefix("Blocked: ").unwrap_or("Unknown");
                    RouteStatus::Blocked(BlockedReason::Resoned(reason.to_string()))
                }
                _ => RouteStatus::Blocked(BlockedReason::Unknown),
            },
            terminal: match self.terminal.as_str() {
                "Internal" => RoutingTerminal::Internal,
                "Middleware" => RoutingTerminal::Middleware,
                _ => RoutingTerminal::External,
            },
            policy: crate::model::route::RoutingPolicy::Basic, // Default policy
            properties: RouteProperties {
                route_id: self.properties.route_id,
                domain_id: self.properties.domain_id,
                owner_id: self.properties.owner_id,
                creator_id: self.properties.creator_id,
                workspace_id: self.properties.workspace_id,
                scripts: self.properties.scripts,
                tags: self.properties.tags,
                custom: self.properties.custom,
                native: self.properties.native,
                bundling: self.properties.bundling,
                opengraph: self.properties.opengraph,
                allow_debug: self.properties.allow_debug,
            },
        }
    }
}
