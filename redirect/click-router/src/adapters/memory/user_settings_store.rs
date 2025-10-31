use std::collections::HashMap;
use std::sync::Arc;
use anyhow::Result;
use async_trait::async_trait;
use tokio::sync::RwLock;

use crate::core::UserSettingsStore;
use crate::model::UserSettings;

/// In-memory user settings store for testing and benchmarking
///
/// This store provides a simple HashMap-based implementation that doesn't
/// require any external dependencies like databases or caches. It's designed
/// for use in tests and benchmarks where you want full control over the data
/// and minimal setup overhead.
#[derive(Clone)]
pub struct InMemoryUserSettingsStore {
    settings: Arc<RwLock<HashMap<String, UserSettings>>>,
}

impl InMemoryUserSettingsStore {
    /// Create a new empty in-memory user settings store
    pub fn new() -> Self {
        Self {
            settings: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Create a store pre-populated with user settings
    pub fn with_settings(settings: Vec<(String, UserSettings)>) -> Self {
        let mut map = HashMap::new();
        for (user_id, user_settings) in settings {
            map.insert(user_id, user_settings);
        }

        Self {
            settings: Arc::new(RwLock::new(map)),
        }
    }

    /// Insert user settings into the store
    pub async fn insert(&self, user_id: &str, settings: UserSettings) {
        let mut store = self.settings.write().await;
        store.insert(user_id.to_string(), settings);
    }

    /// Remove user settings from the store
    pub async fn remove(&self, user_id: &str) {
        let mut store = self.settings.write().await;
        store.remove(user_id);
    }

    /// Clear all user settings from the store
    pub async fn clear(&self) {
        let mut store = self.settings.write().await;
        store.clear();
    }

    /// Get the number of user settings in the store
    pub async fn len(&self) -> usize {
        let store = self.settings.read().await;
        store.len()
    }
}

impl Default for InMemoryUserSettingsStore {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl UserSettingsStore for InMemoryUserSettingsStore {
    async fn get_user_settings(&self, user_id: &str) -> Result<Option<UserSettings>> {
        let settings = self.settings.read().await;
        Ok(settings.get(user_id).cloned())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_empty_store() {
        let store = InMemoryUserSettingsStore::new();
        let result = store.get_user_settings("user123").await.unwrap();
        assert!(result.is_none());
    }

    #[tokio::test]
    async fn test_insert_and_retrieve() {
        let store = InMemoryUserSettingsStore::new();
        let settings = UserSettings::default();

        store.insert("user123", settings.clone()).await;

        let result = store.get_user_settings("user123").await.unwrap();
        assert!(result.is_some());
    }

    #[tokio::test]
    async fn test_with_settings() {
        let settings = vec![
            ("user1".to_string(), UserSettings::default()),
            ("user2".to_string(), UserSettings::default()),
        ];

        let store = InMemoryUserSettingsStore::with_settings(settings);

        assert!(store.get_user_settings("user1").await.unwrap().is_some());
        assert!(store.get_user_settings("user2").await.unwrap().is_some());
        assert!(store.get_user_settings("user3").await.unwrap().is_none());
    }

    #[tokio::test]
    async fn test_remove() {
        let store = InMemoryUserSettingsStore::new();
        let settings = UserSettings::default();

        store.insert("user123", settings).await;
        assert!(store.get_user_settings("user123").await.unwrap().is_some());

        store.remove("user123").await;
        assert!(store.get_user_settings("user123").await.unwrap().is_none());
    }

    #[tokio::test]
    async fn test_clear() {
        let store = InMemoryUserSettingsStore::new();
        store.insert("user1", UserSettings::default()).await;
        store.insert("user2", UserSettings::default()).await;

        assert_eq!(store.len().await, 2);

        store.clear().await;
        assert_eq!(store.len().await, 0);
    }
}
