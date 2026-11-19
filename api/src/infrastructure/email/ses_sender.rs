//! AWS SES email sender (used in production). Credentials come from the
//! ECS task's IAM role via the default AWS credential chain.

use async_trait::async_trait;
use aws_config::BehaviorVersion;
use aws_sdk_sesv2::types::{Body, Content, Destination, EmailContent, Message};
use aws_sdk_sesv2::Client;

use super::sender::EmailSender;
use crate::settings::EmailSettings;

pub struct SesEmailSender {
    client: Client,
    from_address: String,
}

impl SesEmailSender {
    pub async fn new(settings: &EmailSettings) -> anyhow::Result<Self> {
        let config = aws_config::defaults(BehaviorVersion::latest())
            .region(aws_config::Region::new(settings.ses_region.clone()))
            .load()
            .await;

        Ok(Self {
            client: Client::new(&config),
            from_address: settings.from_address.clone(),
        })
    }
}

#[async_trait]
impl EmailSender for SesEmailSender {
    async fn send(&self, to: &str, subject: &str, html_body: &str) -> anyhow::Result<()> {
        let content = EmailContent::builder()
            .simple(
                Message::builder()
                    .subject(Content::builder().data(subject).build()?)
                    .body(
                        Body::builder()
                            .html(Content::builder().data(html_body).build()?)
                            .build(),
                    )
                    .build(),
            )
            .build();

        self.client
            .send_email()
            .from_email_address(&self.from_address)
            .destination(Destination::builder().to_addresses(to).build())
            .content(content)
            .send()
            .await?;

        Ok(())
    }
}
