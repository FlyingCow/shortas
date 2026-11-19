//! Local user directory repository trait.

use async_trait::async_trait;

use crate::domain::entities::{Result, User};

/// Repository for the local shadow user directory (used for invite search).
#[cfg_attr(any(test, feature = "mocks"), mockall::automock)]
#[async_trait]
pub trait UserRepository: Send + Sync {
    /// Insert or update a user's known profile fields.
    async fn upsert<'a>(&self, id: &str, email: &str, name: Option<&'a str>) -> Result<User>;

    /// Get a user by Keycloak ID.
    async fn get_by_id(&self, id: &str) -> Result<Option<User>>;

    /// Get a user by exact email (case-insensitive).
    async fn get_by_email(&self, email: &str) -> Result<Option<User>>;

    /// Search users by email prefix/substring, capped at `limit`.
    async fn search_by_email(&self, query: &str, limit: i32) -> Result<Vec<User>>;
}
