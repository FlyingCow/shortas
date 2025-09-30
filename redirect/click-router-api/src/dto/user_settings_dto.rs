use serde::{Deserialize, Serialize};
use salvo::oapi::ToSchema;

/// User settings DTO for API responses
/// 
/// This DTO provides a clean API interface for user settings,
/// excluding sensitive information like API keys and internal IDs.
#[derive(Serialize, Deserialize, Debug, Clone, ToSchema)]
pub struct UserSettingsDto {
    /// User's email address
    pub email: String,
    /// Current active status
    pub status: String,
    /// Debug mode enabled
    pub debug: bool,
    /// Overflow handling enabled
    pub overflow: bool,
    /// Skip tracking parameters
    pub skip_tracking: Vec<String>,
    /// Allowed request parameters
    pub allowed_request_params: Vec<String>,
    /// Allowed destination parameters
    pub allowed_destination_params: Vec<String>,
}

impl UserSettingsDto {
    /// Create a new UserSettingsDto
    pub fn new(
        email: String,
        status: String,
        debug: bool,
        overflow: bool,
        skip_tracking: Vec<String>,
        allowed_request_params: Vec<String>,
        allowed_destination_params: Vec<String>,
    ) -> Self {
        Self {
            email,
            status,
            debug,
            overflow,
            skip_tracking,
            allowed_request_params,
            allowed_destination_params,
        }
    }

    /// Create a UserSettingsDto with default values
    pub fn default() -> Self {
        Self {
            email: String::new(),
            status: "active".to_string(),
            debug: false,
            overflow: false,
            skip_tracking: Vec::new(),
            allowed_request_params: Vec::new(),
            allowed_destination_params: Vec::new(),
        }
    }

    /// Builder method for email
    pub fn email(mut self, email: String) -> Self {
        self.email = email;
        self
    }

    /// Builder method for status
    pub fn status(mut self, status: String) -> Self {
        self.status = status;
        self
    }

    /// Builder method for debug
    pub fn debug(mut self, debug: bool) -> Self {
        self.debug = debug;
        self
    }

    /// Builder method for overflow
    pub fn overflow(mut self, overflow: bool) -> Self {
        self.overflow = overflow;
        self
    }

    /// Builder method for skip_tracking
    pub fn skip_tracking(mut self, skip_tracking: Vec<String>) -> Self {
        self.skip_tracking = skip_tracking;
        self
    }

    /// Builder method for allowed_request_params
    pub fn allowed_request_params(mut self, allowed_request_params: Vec<String>) -> Self {
        self.allowed_request_params = allowed_request_params;
        self
    }

    /// Builder method for allowed_destination_params
    pub fn allowed_destination_params(mut self, allowed_destination_params: Vec<String>) -> Self {
        self.allowed_destination_params = allowed_destination_params;
        self
    }

    /// Check if the DTO is valid
    pub fn is_valid(&self) -> bool {
        !self.email.is_empty() && !self.status.is_empty()
    }

    /// Get the status as a string
    pub fn get_status(&self) -> &str {
        &self.status
    }

    /// Check if debug mode is enabled
    pub fn is_debug_enabled(&self) -> bool {
        self.debug
    }

    /// Check if overflow handling is enabled
    pub fn is_overflow_enabled(&self) -> bool {
        self.overflow
    }

    /// Get the number of skip tracking parameters
    pub fn skip_tracking_count(&self) -> usize {
        self.skip_tracking.len()
    }

    /// Get the number of allowed request parameters
    pub fn allowed_request_params_count(&self) -> usize {
        self.allowed_request_params.len()
    }

    /// Get the number of allowed destination parameters
    pub fn allowed_destination_params_count(&self) -> usize {
        self.allowed_destination_params.len()
    }
}

impl Default for UserSettingsDto {
    fn default() -> Self {
        Self::default()
    }
}

/// Conversion from UserSettings to UserSettingsDto
impl From<crate::model::user_settings::UserSettings> for UserSettingsDto {
    fn from(user_settings: crate::model::user_settings::UserSettings) -> Self {
        Self {
            email: user_settings.user_email,
            status: match user_settings.active_status {
                crate::model::user_settings::ActiveStatus::Active => "active".to_string(),
                crate::model::user_settings::ActiveStatus::Blocked => "blocked".to_string(),
            },
            debug: user_settings.debug,
            overflow: user_settings.overflow,
            skip_tracking: user_settings.skip,
            allowed_request_params: user_settings.allowed_request_params,
            allowed_destination_params: user_settings.allowed_destination_params,
        }
    }
}

/// Conversion from &UserSettings to UserSettingsDto
impl From<&crate::model::user_settings::UserSettings> for UserSettingsDto {
    fn from(user_settings: &crate::model::user_settings::UserSettings) -> Self {
        Self {
            email: user_settings.user_email.clone(),
            status: match user_settings.active_status {
                crate::model::user_settings::ActiveStatus::Active => "active".to_string(),
                crate::model::user_settings::ActiveStatus::Blocked => "blocked".to_string(),
            },
            debug: user_settings.debug,
            overflow: user_settings.overflow,
            skip_tracking: user_settings.skip.clone(),
            allowed_request_params: user_settings.allowed_request_params.clone(),
            allowed_destination_params: user_settings.allowed_destination_params.clone(),
        }
    }
}

/// Conversion from UserSettingsDto to UserSettings
impl Into<crate::model::user_settings::UserSettings> for UserSettingsDto {
    fn into(self) -> crate::model::user_settings::UserSettings {
        use crate::model::user_settings::{UserSettings, ActiveStatus};
        
        UserSettings::new(
            String::new(), // user_id will be set by the service layer
            self.email,
            None, // api_key is not exposed in DTO
            match self.status.as_str() {
                "active" => ActiveStatus::Active,
                "blocked" => ActiveStatus::Blocked,
                _ => ActiveStatus::Active,
            },
            self.debug,
            self.overflow,
            self.skip_tracking,
            self.allowed_request_params,
            self.allowed_destination_params,
        )
    }
}
