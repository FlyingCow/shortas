//! Email sending abstraction. SMTP (MailHog) in dev, SES in prod.

pub mod sender;
pub mod ses_sender;
pub mod smtp_sender;

pub use sender::*;
pub use ses_sender::*;
pub use smtp_sender::*;

use std::sync::Arc;

use crate::settings::EmailSettings;

/// Build the configured email sender ("smtp" for dev/MailHog, "ses" for prod).
pub async fn build_email_sender(settings: &EmailSettings) -> anyhow::Result<Arc<dyn EmailSender>> {
    match settings.provider.as_str() {
        "smtp" => Ok(Arc::new(SmtpEmailSender::new(settings)?)),
        _ => Ok(Arc::new(SesEmailSender::new(settings).await?)),
    }
}
