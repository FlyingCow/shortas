//! RabbitMQ topology for the generic notification pipeline, shared between
//! the publisher side (`OutboxProcessor`, in `management-api`) and the
//! consumer side (`notification-worker`, which depends on this crate as a
//! library). Declaring it from both sides is safe — `lapin`/RabbitMQ treat
//! repeated declarations with identical arguments as a no-op.
//!
//! Topology: a durable topic exchange fans out to the main queue. A failed
//! delivery is nacked (no requeue) so it dead-letters into the retry queue,
//! which holds it for `notifications_retry_delay_ms` (via per-queue TTL)
//! before dead-lettering it back onto the main exchange for another attempt.
//! After `notifications_max_retries` redeliveries (tracked by RabbitMQ's
//! `x-death` header), the consumer routes it to the dead queue instead of
//! nacking again.

use lapin::{
    options::{ExchangeDeclareOptions, QueueBindOptions, QueueDeclareOptions},
    types::{AMQPValue, FieldTable},
    Channel, ExchangeKind,
};

use crate::settings::RabbitMqSettings;

/// Declare the notifications exchange + main/retry/dead queues on `channel`.
pub async fn declare_notification_topology(
    channel: &Channel,
    settings: &RabbitMqSettings,
) -> anyhow::Result<()> {
    channel
        .exchange_declare(
            &settings.notifications_exchange,
            ExchangeKind::Topic,
            ExchangeDeclareOptions {
                durable: true,
                ..Default::default()
            },
            FieldTable::default(),
        )
        .await?;

    // Dedicated exchange the retry queue dead-letters back onto, bound with
    // a catch-all routing key so any notification_id routes back to the
    // main queue regardless of what it was originally published with.
    let retry_exchange = format!("{}.retry", settings.notifications_exchange);
    channel
        .exchange_declare(
            &retry_exchange,
            ExchangeKind::Fanout,
            ExchangeDeclareOptions {
                durable: true,
                ..Default::default()
            },
            FieldTable::default(),
        )
        .await?;

    let mut main_args = FieldTable::default();
    main_args.insert(
        "x-dead-letter-exchange".into(),
        AMQPValue::LongString(retry_exchange.clone().into()),
    );
    channel
        .queue_declare(
            &settings.notifications_queue,
            QueueDeclareOptions {
                durable: true,
                ..Default::default()
            },
            main_args,
        )
        .await?;
    channel
        .queue_bind(
            &settings.notifications_queue,
            &settings.notifications_exchange,
            "#",
            QueueBindOptions::default(),
            FieldTable::default(),
        )
        .await?;

    let mut retry_args = FieldTable::default();
    retry_args.insert(
        "x-dead-letter-exchange".into(),
        AMQPValue::LongString(settings.notifications_exchange.clone().into()),
    );
    retry_args.insert(
        "x-message-ttl".into(),
        AMQPValue::LongUInt(settings.notifications_retry_delay_ms),
    );
    channel
        .queue_declare(
            &settings.notifications_retry_queue,
            QueueDeclareOptions {
                durable: true,
                ..Default::default()
            },
            retry_args,
        )
        .await?;
    channel
        .queue_bind(
            &settings.notifications_retry_queue,
            &retry_exchange,
            "",
            QueueBindOptions::default(),
            FieldTable::default(),
        )
        .await?;

    channel
        .queue_declare(
            &settings.notifications_dead_queue,
            QueueDeclareOptions {
                durable: true,
                ..Default::default()
            },
            FieldTable::default(),
        )
        .await?;

    Ok(())
}
