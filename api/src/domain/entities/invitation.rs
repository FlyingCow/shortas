//! Workspace invitation entity.

use base64::Engine as _;
use chrono::{DateTime, Duration, Utc};
use rand::RngCore;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use uuid::Uuid;

/// Invitation lifecycle status.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum InvitationStatus {
    Pending,
    Accepted,
    Declined,
    Expired,
    Revoked,
}

impl InvitationStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            InvitationStatus::Pending => "Pending",
            InvitationStatus::Accepted => "Accepted",
            InvitationStatus::Declined => "Declined",
            InvitationStatus::Expired => "Expired",
            InvitationStatus::Revoked => "Revoked",
        }
    }

    pub fn from_str(s: &str) -> Self {
        match s {
            "Accepted" => InvitationStatus::Accepted,
            "Declined" => InvitationStatus::Declined,
            "Expired" => InvitationStatus::Expired,
            "Revoked" => InvitationStatus::Revoked,
            _ => InvitationStatus::Pending,
        }
    }
}

/// A pending (or resolved) invitation to join a workspace.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkspaceInvitation {
    pub id: Uuid,
    pub workspace_id: Uuid,
    pub invited_email: String,
    pub invited_user_id: Option<String>,
    /// "Admin" or "Member" ("User" in the UI). Never "Owner".
    pub role: String,
    pub invited_by: String,
    pub token_hash: String,
    pub status: InvitationStatus,
    pub expires_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
    pub responded_at: Option<DateTime<Utc>>,
}

/// Default invitation lifetime.
const INVITATION_TTL_DAYS: i64 = 7;

impl WorkspaceInvitation {
    /// Generate a new random invite token (returned once, raw) and its hash.
    /// Only the hash is ever persisted; the raw token travels in the email link.
    pub fn generate_token() -> (String, String) {
        let mut bytes = [0u8; 32];
        rand::rng().fill_bytes(&mut bytes);
        let token = base64::engine::general_purpose::URL_SAFE_NO_PAD.encode(bytes);
        let hash = Self::hash_token(&token);
        (token, hash)
    }

    /// Hash a raw token for lookup/storage comparison.
    pub fn hash_token(token: &str) -> String {
        let mut hasher = Sha256::new();
        hasher.update(token.as_bytes());
        format!("{:x}", hasher.finalize())
    }

    /// Create a new pending invitation.
    pub fn new(
        workspace_id: Uuid,
        invited_email: String,
        invited_user_id: Option<String>,
        role: String,
        invited_by: String,
        token_hash: String,
    ) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            workspace_id,
            invited_email,
            invited_user_id,
            role,
            invited_by,
            token_hash,
            status: InvitationStatus::Pending,
            expires_at: now + Duration::days(INVITATION_TTL_DAYS),
            created_at: now,
            responded_at: None,
        }
    }

    pub fn is_pending(&self) -> bool {
        self.status == InvitationStatus::Pending && self.expires_at > Utc::now()
    }

    pub fn is_expired(&self) -> bool {
        self.status == InvitationStatus::Pending && self.expires_at <= Utc::now()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_token_hash_roundtrip() {
        let (token, hash) = WorkspaceInvitation::generate_token();
        assert_eq!(WorkspaceInvitation::hash_token(&token), hash);
        assert_ne!(token, hash);
    }

    #[test]
    fn test_pending_and_expired() {
        let mut invite = WorkspaceInvitation::new(
            Uuid::new_v4(),
            "a@example.com".to_string(),
            None,
            "Member".to_string(),
            "owner-1".to_string(),
            "hash".to_string(),
        );
        assert!(invite.is_pending());
        assert!(!invite.is_expired());

        invite.expires_at = Utc::now() - Duration::seconds(1);
        assert!(!invite.is_pending());
        assert!(invite.is_expired());
    }
}
