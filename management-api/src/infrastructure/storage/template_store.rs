//! S3-backed notification template store. Templates are plain files under
//! `notification-templates/<notification_id>/` in the same bucket
//! `MinioService` already uses, synced from the `notification-templates/`
//! directory in the repo as part of deploy. Shared between `management-api`
//! (not currently a reader, but keeps the store colocated with its S3
//! client) and `notification-worker` (the actual reader), since the worker
//! depends on this crate as a library.

use std::sync::Arc;
use std::time::Duration;

use moka::future::Cache;
use serde::Deserialize;

use super::MinioService;

/// Parsed `metadata.json` for one notification type.
#[derive(Debug, Clone, Deserialize)]
pub struct TemplateMetadata {
    /// Channels to render/deliver, e.g. ["email", "in_app"].
    pub channels: Vec<String>,
    /// Context keys the templates reference; validated before rendering.
    #[serde(default)]
    pub required_context: Vec<String>,
    #[serde(default)]
    pub description: String,
}

/// All template files for one notification type, as raw Handlebars source
/// (compiling is the caller's job, so this stays a dumb value type to cache).
#[derive(Debug, Clone)]
pub struct TemplateSet {
    pub metadata: TemplateMetadata,
    pub email_subject: Option<String>,
    pub email_body: Option<String>,
    pub in_app: Option<String>,
}

impl TemplateSet {
    /// Check that `context` has every key `metadata.required_context` needs.
    pub fn validate_context(&self, context: &serde_json::Value) -> anyhow::Result<()> {
        let obj = context.as_object();
        for key in &self.metadata.required_context {
            let present = obj.map(|o| o.contains_key(key)).unwrap_or(false);
            if !present {
                anyhow::bail!("Missing required context key '{}'", key);
            }
        }
        Ok(())
    }
}

/// Fetches and caches `TemplateSet`s from S3.
pub struct TemplateStore {
    storage: Arc<MinioService>,
    cache: Cache<String, Arc<TemplateSet>>,
}

impl TemplateStore {
    pub fn new(storage: Arc<MinioService>) -> Self {
        let cache = Cache::builder()
            .time_to_live(Duration::from_secs(60))
            .build();
        Self { storage, cache }
    }

    /// Get the template set for `notification_id`, from cache or S3.
    pub async fn get(&self, notification_id: &str) -> anyhow::Result<Arc<TemplateSet>> {
        if let Some(cached) = self.cache.get(notification_id).await {
            return Ok(cached);
        }

        let set = Arc::new(self.load(notification_id).await?);
        self.cache
            .insert(notification_id.to_string(), set.clone())
            .await;
        Ok(set)
    }

    async fn load(&self, notification_id: &str) -> anyhow::Result<TemplateSet> {
        let prefix = format!("notification-templates/{}", notification_id);

        let metadata_raw = self
            .storage
            .get_object_text(&format!("{}/metadata.json", prefix))
            .await?
            .ok_or_else(|| {
                anyhow::anyhow!(
                    "No template metadata for notification_id '{}'",
                    notification_id
                )
            })?;
        let metadata: TemplateMetadata = serde_json::from_str(&metadata_raw)?;

        let email_subject = self
            .storage
            .get_object_text(&format!("{}/email.subject.hbs", prefix))
            .await?;
        let email_body = self
            .storage
            .get_object_text(&format!("{}/email.body.hbs", prefix))
            .await?;
        let in_app = self
            .storage
            .get_object_text(&format!("{}/in_app.hbs", prefix))
            .await?;

        Ok(TemplateSet {
            metadata,
            email_subject,
            email_body,
            in_app,
        })
    }
}
