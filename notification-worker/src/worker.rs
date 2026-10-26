//! RabbitMQ consume loop: render + deliver one `NotificationEvent` per
//! message, across whatever channels its template declares.

use std::sync::Arc;

use futures::StreamExt;
use handlebars::Handlebars;
use lapin::{
    options::{BasicAckOptions, BasicConsumeOptions, BasicNackOptions, BasicPublishOptions},
    types::{AMQPValue, FieldTable},
    BasicProperties, Connection, ConnectionProperties,
};
use tokio::sync::broadcast;
use tracing::{error, info, warn};

use management_api::domain::entities::{Notification, NotificationEvent};
use management_api::domain::traits::{NotificationRepository, UserRepository};
use management_api::infrastructure::email::EmailSender;
use management_api::infrastructure::messaging::declare_notification_topology;
use management_api::infrastructure::storage::{TemplateSet, TemplateStore};
use management_api::settings::RabbitMqSettings;

pub struct NotificationWorker {
    settings: RabbitMqSettings,
    notification_repo: Arc<dyn NotificationRepository>,
    user_repo: Arc<dyn UserRepository>,
    email_sender: Arc<dyn EmailSender>,
    template_store: Arc<TemplateStore>,
}

impl NotificationWorker {
    pub fn new(
        settings: RabbitMqSettings,
        notification_repo: Arc<dyn NotificationRepository>,
        user_repo: Arc<dyn UserRepository>,
        email_sender: Arc<dyn EmailSender>,
        template_store: Arc<TemplateStore>,
    ) -> Self {
        Self {
            settings,
            notification_repo,
            user_repo,
            email_sender,
            template_store,
        }
    }

    pub async fn run(&self, mut shutdown: broadcast::Receiver<()>) -> anyhow::Result<()> {
        let connection =
            Connection::connect(&self.settings.url, ConnectionProperties::default()).await?;
        let channel = connection.create_channel().await?;

        declare_notification_topology(&channel, &self.settings).await?;

        let mut consumer = channel
            .basic_consume(
                &self.settings.notifications_queue,
                "notification-worker",
                BasicConsumeOptions::default(),
                FieldTable::default(),
            )
            .await?;

        info!(
            "notification-worker consuming queue '{}'",
            self.settings.notifications_queue
        );

        loop {
            tokio::select! {
                _ = shutdown.recv() => {
                    info!("Shutting down notification worker");
                    break;
                }
                delivery = consumer.next() => {
                    let Some(Ok(delivery)) = delivery else { continue };

                    if self.redelivery_count(&delivery) >= self.settings.notifications_max_retries {
                        self.dead_letter(&channel, &delivery.data).await;
                        let _ = channel.basic_ack(delivery.delivery_tag, BasicAckOptions::default()).await;
                        continue;
                    }

                    match serde_json::from_slice::<NotificationEvent>(&delivery.data) {
                        Ok(event) => {
                            if let Err(e) = self.handle_event(&event).await {
                                warn!("Failed to process notification '{}': {}", event.notification_id, e);
                                let _ = channel
                                    .basic_nack(
                                        delivery.delivery_tag,
                                        BasicNackOptions { requeue: false, ..Default::default() },
                                    )
                                    .await;
                                continue;
                            }
                            let _ = channel.basic_ack(delivery.delivery_tag, BasicAckOptions::default()).await;
                        }
                        Err(e) => {
                            warn!("Failed to parse notification event, dead-lettering: {}", e);
                            self.dead_letter(&channel, &delivery.data).await;
                            let _ = channel.basic_ack(delivery.delivery_tag, BasicAckOptions::default()).await;
                        }
                    }
                }
            }
        }

        Ok(())
    }

    /// Count how many times RabbitMQ has already dead-lettered this message
    /// back onto the main queue (the `x-death` header array length).
    fn redelivery_count(&self, delivery: &lapin::message::Delivery) -> i64 {
        delivery
            .properties
            .headers()
            .as_ref()
            .and_then(|h| h.inner().get("x-death"))
            .and_then(|v| match v {
                AMQPValue::FieldArray(arr) => Some(arr.as_slice().len() as i64),
                _ => None,
            })
            .unwrap_or(0)
    }

    /// Publish a message straight to the dead queue (bypassing the
    /// exchange/retry loop entirely) for manual inspection.
    async fn dead_letter(&self, channel: &lapin::Channel, body: &[u8]) {
        let result = channel
            .basic_publish(
                "",
                &self.settings.notifications_dead_queue,
                BasicPublishOptions::default(),
                body,
                BasicProperties::default().with_delivery_mode(2),
            )
            .await;

        if let Err(e) = result {
            error!("Failed to dead-letter notification message: {}", e);
        }
    }

    async fn handle_event(&self, event: &NotificationEvent) -> anyhow::Result<()> {
        let template_set = self.template_store.get(&event.notification_id).await?;
        template_set.validate_context(&event.context)?;

        for channel in &template_set.metadata.channels {
            match channel.as_str() {
                "in_app" => self.deliver_in_app(event, &template_set).await?,
                "email" => self.deliver_email(event, &template_set).await?,
                other => warn!("Unknown notification channel '{}', skipping", other),
            }
        }

        Ok(())
    }

    async fn deliver_in_app(
        &self,
        event: &NotificationEvent,
        template_set: &TemplateSet,
    ) -> anyhow::Result<()> {
        let Some(user_id) = &event.recipient.user_id else {
            info!(
                "Skipping in_app delivery for '{}': recipient has no user_id",
                event.notification_id
            );
            return Ok(());
        };

        let Some(in_app_template) = &template_set.in_app else {
            warn!(
                "Notification '{}' declares in_app channel but has no in_app.hbs",
                event.notification_id
            );
            return Ok(());
        };

        let rendered_text = render(in_app_template, &event.context)?;

        let notification = Notification::with_rendered_text(
            user_id.clone(),
            event.notification_id.clone(),
            event.context.clone(),
            rendered_text,
        );
        self.notification_repo.create(&notification).await?;

        info!(
            "Created in-app notification '{}' for user {}",
            event.notification_id, user_id
        );
        Ok(())
    }

    async fn deliver_email(
        &self,
        event: &NotificationEvent,
        template_set: &TemplateSet,
    ) -> anyhow::Result<()> {
        let to_email = match &event.recipient.email {
            Some(email) => email.clone(),
            None => {
                let user_id = event.recipient.user_id.as_ref().ok_or_else(|| {
                    anyhow::anyhow!("Email channel needs recipient.email or recipient.user_id")
                })?;
                self.user_repo
                    .get_by_id(user_id)
                    .await?
                    .ok_or_else(|| {
                        anyhow::anyhow!("User '{}' not found for email delivery", user_id)
                    })?
                    .email
            }
        };

        let (Some(subject_template), Some(body_template)) =
            (&template_set.email_subject, &template_set.email_body)
        else {
            warn!(
                "Notification '{}' declares email channel but is missing email.subject.hbs/email.body.hbs",
                event.notification_id
            );
            return Ok(());
        };

        let subject = render(subject_template, &event.context)?;
        let body = render(body_template, &event.context)?;

        self.email_sender.send(&to_email, &subject, &body).await?;

        info!(
            "Sent email for notification '{}' to {}",
            event.notification_id, to_email
        );
        Ok(())
    }
}

fn render(template: &str, context: &serde_json::Value) -> anyhow::Result<String> {
    let hb = Handlebars::new();
    Ok(hb.render_template(template, context)?.trim().to_string())
}
