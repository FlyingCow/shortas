//! SMTP email sender (used in dev against MailHog; could point at a real relay too).

use async_trait::async_trait;
use lettre::message::header::ContentType;
use lettre::{AsyncSmtpTransport, AsyncTransport, Message, Tokio1Executor};

use super::sender::EmailSender;
use crate::settings::EmailSettings;

pub struct SmtpEmailSender {
    transport: AsyncSmtpTransport<Tokio1Executor>,
    from_address: String,
}

impl SmtpEmailSender {
    pub fn new(settings: &EmailSettings) -> anyhow::Result<Self> {
        let transport = AsyncSmtpTransport::<Tokio1Executor>::builder_dangerous(&settings.smtp_host)
            .port(settings.smtp_port)
            .build();

        Ok(Self {
            transport,
            from_address: settings.from_address.clone(),
        })
    }
}

#[async_trait]
impl EmailSender for SmtpEmailSender {
    async fn send(&self, to: &str, subject: &str, html_body: &str) -> anyhow::Result<()> {
        let email = Message::builder()
            .from(self.from_address.parse()?)
            .to(to.parse()?)
            .subject(subject)
            .header(ContentType::TEXT_HTML)
            .body(html_body.to_string())?;

        self.transport.send(email).await?;
        Ok(())
    }
}
