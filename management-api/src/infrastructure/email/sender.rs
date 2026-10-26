//! Email sending trait.

use async_trait::async_trait;

/// Abstraction over how outbound email is actually delivered.
#[async_trait]
pub trait EmailSender: Send + Sync {
    async fn send(&self, to: &str, subject: &str, html_body: &str) -> anyhow::Result<()>;
}
