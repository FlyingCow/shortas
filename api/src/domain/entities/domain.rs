//! Domain types for route domain management.

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
    fn test_domain_verification_status_as_str() {
        assert_eq!(DomainVerificationStatus::Pending.as_str(), "Pending");
        assert_eq!(DomainVerificationStatus::Verified.as_str(), "Verified");
        assert_eq!(DomainVerificationStatus::Failed.as_str(), "Failed");
    }

    #[test]
    fn test_domain_verification_status_from_str() {
        assert_eq!(DomainVerificationStatus::from_str("Verified"), DomainVerificationStatus::Verified);
        assert_eq!(DomainVerificationStatus::from_str("Failed"), DomainVerificationStatus::Failed);
        assert_eq!(DomainVerificationStatus::from_str("Pending"), DomainVerificationStatus::Pending);
        assert_eq!(DomainVerificationStatus::from_str("garbage"), DomainVerificationStatus::Pending);
    }

    #[test]
    fn test_route_domain_new_normalizes_name_and_defaults_to_pending() {
        let domain = RouteDomain::new("Example.COM".to_string(), "user-1".to_string());
        assert_eq!(domain.name, "example.com");
        assert_eq!(domain.owner_id, "user-1");
        assert!(!domain.is_verified());
        assert_eq!(domain.verification_reason, "not_checked");
        assert!(domain.created_at.is_some());
    }

    #[test]
    fn test_route_domain_is_verified() {
        let mut domain = RouteDomain::new("example.com".to_string(), "user-1".to_string());
        assert!(!domain.is_verified());
        domain.verification_status = DomainVerificationStatus::Verified;
        assert!(domain.is_verified());
    }

    #[test]
    fn test_route_domain_can_use() {
        let mut domain = RouteDomain::new("example.com".to_string(), "owner-1".to_string());
        assert!(domain.can_use("owner-1"));
        assert!(!domain.can_use("someone-else"));

        domain.is_shared = true;
        assert!(domain.can_use("someone-else"));
    }

    #[test]
    fn test_certificate_new_sets_fresh_id_and_created_at() {
        let domain_id = Uuid::new_v4();
        let cert = Certificate::new("key".to_string(), "cert".to_string(), "owner-1".to_string(), domain_id);
        assert_eq!(cert.key, "key");
        assert_eq!(cert.cert, "cert");
        assert_eq!(cert.domain_id, domain_id);
        assert!(cert.created_at.is_some());
        assert!(!cert.id.is_nil());
    }

    #[test]
    fn test_workspace_is_system() {
        let system_workspace = Workspace::new("System".to_string(), "System".to_string());
        assert!(system_workspace.is_system());

        let user_workspace = Workspace::new("My Workspace".to_string(), "User".to_string());
        assert!(!user_workspace.is_system());
    }

    #[test]
    fn test_user_workspace_roles() {
        let workspace_id = Uuid::new_v4();
        let owner = UserWorkspace::new("user-1".to_string(), workspace_id, "Owner".to_string());
        assert!(owner.is_owner());
        assert!(owner.is_admin());

        let admin = UserWorkspace::new("user-2".to_string(), workspace_id, "Admin".to_string());
        assert!(!admin.is_owner());
        assert!(admin.is_admin());

        let member = UserWorkspace::new("user-3".to_string(), workspace_id, "Member".to_string());
        assert!(!member.is_owner());
        assert!(!member.is_admin());
    }
}
