//! User-related entities.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// User profile from JWT claims.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserProfile {
    /// Keycloak user ID (sub claim).
    pub id: String,
    /// User's email.
    pub email: Option<String>,
    /// User's name.
    pub name: Option<String>,
    /// User's preferred username.
    pub preferred_username: Option<String>,
    /// Whether email is verified.
    pub email_verified: bool,
    /// User's roles.
    pub roles: Vec<String>,
}

impl UserProfile {
    /// Check if user has admin role.
    pub fn is_admin(&self) -> bool {
        self.roles.iter().any(|r| r == "admin" || r == "Admin")
    }
}

/// User initialization/onboarding state.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserOnboarding {
    pub user_id: String,
    pub completed: bool,
    pub current_step: i32,
    pub steps_completed: Vec<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl UserOnboarding {
    /// Create a new onboarding state.
    pub fn new(user_id: String) -> Self {
        Self {
            user_id,
            completed: false,
            current_step: 0,
            steps_completed: Vec::new(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }

    /// Mark a step as completed.
    pub fn complete_step(&mut self, step: &str) {
        if !self.steps_completed.contains(&step.to_string()) {
            self.steps_completed.push(step.to_string());
            self.current_step = self.steps_completed.len() as i32;
            self.updated_at = Utc::now();
        }
    }

    /// Mark onboarding as fully completed.
    pub fn complete(&mut self) {
        self.completed = true;
        self.updated_at = Utc::now();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_user_profile_admin() {
        let admin = UserProfile {
            id: "123".to_string(),
            email: Some("admin@example.com".to_string()),
            name: None,
            preferred_username: None,
            email_verified: true,
            roles: vec!["admin".to_string()],
        };
        assert!(admin.is_admin());

        let user = UserProfile {
            id: "456".to_string(),
            email: Some("user@example.com".to_string()),
            name: None,
            preferred_username: None,
            email_verified: true,
            roles: vec!["user".to_string()],
        };
        assert!(!user.is_admin());
    }

    #[test]
    fn test_onboarding() {
        let mut onboarding = UserOnboarding::new("user-123".to_string());
        assert!(!onboarding.completed);
        assert_eq!(onboarding.current_step, 0);

        onboarding.complete_step("create_workspace");
        assert_eq!(onboarding.current_step, 1);
        assert!(onboarding.steps_completed.contains(&"create_workspace".to_string()));

        onboarding.complete();
        assert!(onboarding.completed);
    }
}
