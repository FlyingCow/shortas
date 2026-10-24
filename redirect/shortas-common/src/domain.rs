//! Domain types for route domain management.
//!
//! These types define domains that can host routes, including
//! verification status, custom pages, and DNS configuration.

use chrono::{DateTime, Utc};
use salvo::oapi::ToSchema;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Domain verification status.
#[derive(Default, Serialize, Deserialize, Debug, Clone, PartialEq, ToSchema)]
pub enum DomainVerificationStatus {
    #[default]
    Pending,
    Verified,
    Failed,
}

impl DomainVerificationStatus {
    /// Get the status as a string for database storage.
    pub fn as_str(&self) -> &'static str {
        match self {
            DomainVerificationStatus::Pending => "Pending",
            DomainVerificationStatus::Verified => "Verified",
            DomainVerificationStatus::Failed => "Failed",
        }
    }

    /// Parse from string.
    pub fn from_str(s: &str) -> Self {
        match s {
            "Verified" => DomainVerificationStatus::Verified,
            "Failed" => DomainVerificationStatus::Failed,
            _ => DomainVerificationStatus::Pending,
        }
    }
}

/// DNS record configuration for domain verification.
#[derive(Default, Serialize, Deserialize, Debug, Clone, PartialEq, ToSchema)]
pub struct DnsConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub txt_record: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cname_target: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub a_record: Option<String>,
}

/// Route domain entity.
#[derive(Default, Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct RouteDomain {
    pub id: Uuid,
    /// Domain name (lowercase normalized).
    pub name: String,
    /// Owner's user ID.
    pub owner_id: String,
    /// Whether this domain is shared across users.
    #[serde(default)]
    pub is_shared: bool,
    /// Current verification status.
    #[serde(default)]
    pub verification_status: DomainVerificationStatus,
    /// Reason for current verification status.
    #[serde(default)]
    pub verification_reason: String,
    /// Last verification check timestamp.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_verification_check: Option<DateTime<Utc>>,
    /// Next scheduled verification check.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_verification_check: Option<DateTime<Utc>>,
    /// Custom index page URL.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_index_url: Option<String>,
    /// Custom 404 page URL.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_not_found_url: Option<String>,
    /// DNS configuration for verification.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dns_config: Option<DnsConfig>,
    /// Domain creation timestamp.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<DateTime<Utc>>,
    /// Last update timestamp.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<DateTime<Utc>>,
}

impl RouteDomain {
    /// Create a new domain with the given name and owner.
    pub fn new(name: String, owner_id: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            name: name.to_lowercase(),
            owner_id,
            verification_status: DomainVerificationStatus::Pending,
            verification_reason: "not_checked".to_string(),
            created_at: Some(Utc::now()),
            updated_at: Some(Utc::now()),
            ..Default::default()
        }
    }

    /// Check if the domain is verified.
    pub fn is_verified(&self) -> bool {
        matches!(self.verification_status, DomainVerificationStatus::Verified)
    }

    /// Check if the user can use this domain.
    pub fn can_use(&self, user_id: &str) -> bool {
        self.is_shared || self.owner_id == user_id
    }
}

/// Certificate entity for SSL/TLS.
#[derive(Default, Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Certificate {
    pub id: Uuid,
    /// PEM-encoded private key.
    pub key: String,
    /// PEM-encoded certificate.
    pub cert: String,
    /// OCSP response (optional).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ocsp_resp: Option<String>,
    /// Owner's user ID.
    pub owner_id: String,
    /// Associated domain ID.
    pub domain_id: Uuid,
    /// Certificate expiry date.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<DateTime<Utc>>,
    /// Certificate creation timestamp.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<DateTime<Utc>>,
}

impl Certificate {
    /// Create a new certificate.
    pub fn new(key: String, cert: String, owner_id: String, domain_id: Uuid) -> Self {
        Self {
            id: Uuid::new_v4(),
            key,
            cert,
            owner_id,
            domain_id,
            created_at: Some(Utc::now()),
            ..Default::default()
        }
    }
}

/// Workspace entity for multi-tenancy.
#[derive(Default, Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Workspace {
    pub id: Uuid,
    pub name: String,
    #[serde(default)]
    pub description: String,
    /// "System" or "User".
    #[serde(default)]
    pub workspace_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<DateTime<Utc>>,
}

impl Workspace {
    /// Create a new workspace.
    pub fn new(name: String, workspace_type: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            name,
            workspace_type,
            created_at: Some(Utc::now()),
            updated_at: Some(Utc::now()),
            ..Default::default()
        }
    }

    /// Check if this is a system workspace.
    pub fn is_system(&self) -> bool {
        self.workspace_type == "System"
    }
}

/// User-workspace membership.
#[derive(Default, Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct UserWorkspace {
    pub id: Uuid,
    /// Keycloak user ID.
    pub user_id: String,
    pub workspace_id: Uuid,
    /// "Owner", "Admin", or "Member".
    #[serde(default)]
    pub role: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub joined_at: Option<DateTime<Utc>>,
}

impl UserWorkspace {
    /// Create a new user-workspace membership.
    pub fn new(user_id: String, workspace_id: Uuid, role: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            user_id,
            workspace_id,
            role,
            joined_at: Some(Utc::now()),
        }
    }

    /// Check if user is owner.
    pub fn is_owner(&self) -> bool {
        self.role == "Owner"
    }

    /// Check if user is admin or owner.
    pub fn is_admin(&self) -> bool {
        self.role == "Admin" || self.role == "Owner"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_domain_creation() {
        let domain = RouteDomain::new("Example.COM".to_string(), "user-123".to_string());
        assert_eq!(domain.name, "example.com");
        assert_eq!(domain.owner_id, "user-123");
        assert!(!domain.is_verified());
    }

    #[test]
    fn test_domain_can_use() {
        let domain = RouteDomain {
            owner_id: "user-123".to_string(),
            is_shared: false,
            ..Default::default()
        };
        assert!(domain.can_use("user-123"));
        assert!(!domain.can_use("user-456"));

        let shared_domain = RouteDomain {
            owner_id: "user-123".to_string(),
            is_shared: true,
            ..Default::default()
        };
        assert!(shared_domain.can_use("user-456"));
    }

    #[test]
    fn test_workspace_creation() {
        let workspace = Workspace::new("Test Workspace".to_string(), "User".to_string());
        assert!(!workspace.is_system());

        let system = Workspace::new("System".to_string(), "System".to_string());
        assert!(system.is_system());
    }

    #[test]
    fn test_user_workspace_roles() {
        let owner = UserWorkspace::new("user-1".to_string(), Uuid::new_v4(), "Owner".to_string());
        assert!(owner.is_owner());
        assert!(owner.is_admin());

        let admin = UserWorkspace::new("user-2".to_string(), Uuid::new_v4(), "Admin".to_string());
        assert!(!admin.is_owner());
        assert!(admin.is_admin());

        let member = UserWorkspace::new("user-3".to_string(), Uuid::new_v4(), "Member".to_string());
        assert!(!member.is_owner());
        assert!(!member.is_admin());
    }
}
