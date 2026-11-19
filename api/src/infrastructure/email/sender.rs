//! Email sending trait.

use async_trait::async_trait;

/// Abstraction over how outbound email is actually delivered.
#[cfg_attr(any(test, feature = "mocks"), mockall::automock)]
#[async_trait]
pub trait EmailSender: Send + Sync {
    async fn send(&self, to: &str, subject: &str, html_body: &str) -> anyhow::Result<()>;
}
