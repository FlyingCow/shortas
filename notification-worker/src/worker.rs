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

                    if redelivery_count(&delivery) >= self.settings.notifications_max_retries {
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
                "in_app" => deliver_in_app(&self.notification_repo, event, &template_set).await?,
                "email" => deliver_email(&self.user_repo, &self.email_sender, event, &template_set).await?,
                other => warn!("Unknown notification channel '{}', skipping", other),
            }
        }

        Ok(())
    }

}

/// Deliver the in-app channel: renders `template_set.in_app` and stores it
/// via `notification_repo`. A free function (rather than a `NotificationWorker`
/// method) so it only depends on the one repository it actually needs, which
/// keeps it unit-testable without a live S3-backed `TemplateStore`.
async fn deliver_in_app(
    notification_repo: &Arc<dyn NotificationRepository>,
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
    notification_repo.create(&notification).await?;

    info!(
        "Created in-app notification '{}' for user {}",
        event.notification_id, user_id
    );
    Ok(())
}

/// Deliver the email channel: resolves the recipient's address (falling back
/// to a `user_repo` lookup), renders `template_set.email_subject`/`email_body`,
/// and sends via `email_sender`. See `deliver_in_app` for why this is a free
/// function rather than a method.
async fn deliver_email(
    user_repo: &Arc<dyn UserRepository>,
    email_sender: &Arc<dyn EmailSender>,
    event: &NotificationEvent,
    template_set: &TemplateSet,
) -> anyhow::Result<()> {
    let to_email = match &event.recipient.email {
        Some(email) => email.clone(),
        None => {
            let user_id = event.recipient.user_id.as_ref().ok_or_else(|| {
                anyhow::anyhow!("Email channel needs recipient.email or recipient.user_id")
            })?;
            user_repo
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

    email_sender.send(&to_email, &subject, &body).await?;

    info!(
        "Sent email for notification '{}' to {}",
        event.notification_id, to_email
    );
    Ok(())
}

fn render(template: &str, context: &serde_json::Value) -> anyhow::Result<String> {
    let hb = Handlebars::new();
    Ok(hb.render_template(template, context)?.trim().to_string())
}

/// Count how many times RabbitMQ has already dead-lettered this message
/// back onto the main queue (the `x-death` header array length).
fn redelivery_count(delivery: &lapin::message::Delivery) -> i64 {
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

#[cfg(test)]
mod tests {
    use super::*;
    use lapin::message::Delivery;
    use lapin::types::{AMQPValue, FieldArray, FieldTable, LongString};
    use std::fs;

    fn fixture_path(name: &str) -> String {
        format!(
            "{}/../notification-templates/workspace.invite_created/{}",
            env!("CARGO_MANIFEST_DIR"),
            name
        )
    }

    fn delivery_with_headers(headers: Option<FieldTable>) -> Delivery {
        let mut properties = BasicProperties::default();
        if let Some(headers) = headers {
            properties = properties.with_headers(headers);
        }
        Delivery {
            delivery_tag: 1,
            exchange: "".into(),
            routing_key: "".into(),
            redelivered: false,
            properties,
            data: Vec::new(),
            acker: Default::default(),
        }
    }

    #[test]
    fn test_redelivery_count_no_headers_is_zero() {
        let delivery = delivery_with_headers(None);
        assert_eq!(redelivery_count(&delivery), 0);
    }

    #[test]
    fn test_redelivery_count_no_x_death_header_is_zero() {
        let mut headers = FieldTable::default();
        headers.insert("some-other-header".into(), AMQPValue::LongString("x".into()));
        let delivery = delivery_with_headers(Some(headers));
        assert_eq!(redelivery_count(&delivery), 0);
    }

    #[test]
    fn test_redelivery_count_reflects_x_death_array_length() {
        let mut x_death_entry = FieldTable::default();
        x_death_entry.insert("count".into(), AMQPValue::LongLongInt(1));

        let mut array = FieldArray::default();
        array.push(AMQPValue::FieldTable(x_death_entry.clone()));
        array.push(AMQPValue::FieldTable(x_death_entry));

        let mut headers = FieldTable::default();
        headers.insert("x-death".into(), AMQPValue::FieldArray(array));

        let delivery = delivery_with_headers(Some(headers));
        assert_eq!(redelivery_count(&delivery), 2);
    }

    #[test]
    fn test_redelivery_count_ignores_x_death_of_wrong_type() {
        let mut headers = FieldTable::default();
        headers.insert(
            "x-death".into(),
            AMQPValue::LongString(LongString::from("not-an-array")),
        );
        let delivery = delivery_with_headers(Some(headers));
        assert_eq!(redelivery_count(&delivery), 0);
    }

    #[test]
    fn test_render_substitutes_context_and_trims_whitespace() {
        let rendered = render("  Hello, {{name}}!  ", &serde_json::json!({"name": "Ada"}))
            .expect("render should succeed");
        assert_eq!(rendered, "Hello, Ada!");
    }

    #[test]
    fn test_render_missing_key_renders_empty_placeholder() {
        let rendered =
            render("Hello, {{name}}!", &serde_json::json!({})).expect("render should succeed");
        assert_eq!(rendered, "Hello, !");
    }

    #[test]
    fn test_render_real_invite_created_subject_template() {
        let template = fs::read_to_string(fixture_path("email.subject.hbs"))
            .expect("fixture template should exist");
        let context = serde_json::json!({
            "workspace_name": "Acme",
            "inviter_name": "Jane",
        });
        let rendered = render(&template, &context).expect("render should succeed");
        assert!(!rendered.is_empty());
    }

    // --- deliver_in_app / deliver_email ---

    use management_api::domain::entities::{NotificationRecipient, User};
    use management_api::domain::traits::{MockNotificationRepository, MockUserRepository};
    use management_api::infrastructure::email::MockEmailSender;
    use management_api::infrastructure::storage::TemplateMetadata;

    fn event(user_id: Option<&str>, email: Option<&str>) -> NotificationEvent {
        NotificationEvent::new(
            "workspace.invite_created",
            NotificationRecipient {
                user_id: user_id.map(String::from),
                email: email.map(String::from),
            },
            serde_json::json!({ "workspace_name": "Acme", "inviter_name": "Jane" }),
            uuid::Uuid::new_v4(),
        )
    }

    fn template_set(in_app: Option<&str>, email_subject: Option<&str>, email_body: Option<&str>) -> TemplateSet {
        TemplateSet {
            metadata: TemplateMetadata {
                channels: vec!["in_app".to_string(), "email".to_string()],
                required_context: vec![],
                description: String::new(),
            },
            email_subject: email_subject.map(String::from),
            email_body: email_body.map(String::from),
            in_app: in_app.map(String::from),
        }
    }

    #[tokio::test]
    async fn test_deliver_in_app_creates_rendered_notification() {
        let mut repo = MockNotificationRepository::new();
        repo.expect_create()
            .withf(|n| n.user_id == "user-1" && n.rendered_text.as_deref() == Some("Jane invited you to Acme"))
            .returning(|n| Ok(n.clone()));
        let repo: Arc<dyn NotificationRepository> = Arc::new(repo);

        let event = event(Some("user-1"), None);
        let templates = template_set(Some("{{inviter_name}} invited you to {{workspace_name}}"), None, None);

        deliver_in_app(&repo, &event, &templates).await.unwrap();
    }

    #[tokio::test]
    async fn test_deliver_in_app_skips_when_no_user_id() {
        let mut repo = MockNotificationRepository::new();
        repo.expect_create().times(0);
        let repo: Arc<dyn NotificationRepository> = Arc::new(repo);

        let event = event(None, Some("someone@example.com"));
        let templates = template_set(Some("hello"), None, None);

        deliver_in_app(&repo, &event, &templates).await.unwrap();
    }

    #[tokio::test]
    async fn test_deliver_in_app_skips_when_no_in_app_template() {
        let mut repo = MockNotificationRepository::new();
        repo.expect_create().times(0);
        let repo: Arc<dyn NotificationRepository> = Arc::new(repo);

        let event = event(Some("user-1"), None);
        let templates = template_set(None, None, None);

        deliver_in_app(&repo, &event, &templates).await.unwrap();
    }

    #[tokio::test]
    async fn test_deliver_email_uses_explicit_recipient_email() {
        let user_repo: Arc<dyn UserRepository> = Arc::new(MockUserRepository::new());

        let mut sender = MockEmailSender::new();
        sender
            .expect_send()
            .withf(|to, subject, _body| to == "explicit@example.com" && subject == "Jane invited you to join \"Acme\" on Shortas")
            .returning(|_, _, _| Ok(()));
        let sender: Arc<dyn EmailSender> = Arc::new(sender);

        let event = event(None, Some("explicit@example.com"));
        let templates = template_set(
            None,
            Some("{{inviter_name}} invited you to join \"{{workspace_name}}\" on Shortas"),
            Some("Click here"),
        );

        deliver_email(&user_repo, &sender, &event, &templates).await.unwrap();
    }

    #[tokio::test]
    async fn test_deliver_email_falls_back_to_user_repo_lookup() {
        let mut user_repo = MockUserRepository::new();
        user_repo.expect_get_by_id().returning(|id| {
            Ok(Some(User {
                id: id.to_string(),
                email: "looked-up@example.com".to_string(),
                name: None,
                created_at: chrono::Utc::now(),
                updated_at: chrono::Utc::now(),
            }))
        });
        let user_repo: Arc<dyn UserRepository> = Arc::new(user_repo);

        let mut sender = MockEmailSender::new();
        sender
            .expect_send()
            .withf(|to, _, _| to == "looked-up@example.com")
            .returning(|_, _, _| Ok(()));
        let sender: Arc<dyn EmailSender> = Arc::new(sender);

        let event = event(Some("user-1"), None);
        let templates = template_set(None, Some("subject"), Some("body"));

        deliver_email(&user_repo, &sender, &event, &templates).await.unwrap();
    }

    #[tokio::test]
    async fn test_deliver_email_errors_when_user_not_found() {
        let mut user_repo = MockUserRepository::new();
        user_repo.expect_get_by_id().returning(|_| Ok(None));
        let user_repo: Arc<dyn UserRepository> = Arc::new(user_repo);
        let sender: Arc<dyn EmailSender> = Arc::new(MockEmailSender::new());

        let event = event(Some("user-1"), None);
        let templates = template_set(None, Some("subject"), Some("body"));

        let err = deliver_email(&user_repo, &sender, &event, &templates).await.unwrap_err();
        assert!(err.to_string().contains("not found"));
    }

    #[tokio::test]
    async fn test_deliver_email_skips_when_templates_missing() {
        let user_repo: Arc<dyn UserRepository> = Arc::new(MockUserRepository::new());

        let mut sender = MockEmailSender::new();
        sender.expect_send().times(0);
        let sender: Arc<dyn EmailSender> = Arc::new(sender);

        let event = event(None, Some("explicit@example.com"));
        let templates = template_set(None, None, None); // no subject/body templates

        deliver_email(&user_repo, &sender, &event, &templates).await.unwrap();
    }
}
