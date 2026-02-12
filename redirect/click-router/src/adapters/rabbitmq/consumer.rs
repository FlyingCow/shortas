use futures::StreamExt;
use lapin::{
    options::{
        BasicAckOptions, BasicConsumeOptions, ExchangeDeclareOptions, QueueBindOptions,
        QueueDeclareOptions,
    },
    types::FieldTable,
    Connection, ConnectionProperties, ExchangeKind,
};
use tracing::{info, warn};

use crate::adapters::{RoutesCacheType, UserSettingsCacheType};
use crate::core::{routes::RoutesCache, user_settings::UserSettingsCache};

use super::{
    messages::{RouteChangedMessage, UserSettingsChangedMessage},
    settings::RabbitMqSettings,
};

pub fn start_cache_invalidation_consumer(
    settings: RabbitMqSettings,
    routes_cache: RoutesCacheType,
    user_settings_cache: UserSettingsCacheType,
) {
    tokio::spawn(async move {
        loop {
            info!("RabbitMQ consumer connecting...");
            match run_consumer(&settings, &routes_cache, &user_settings_cache).await {
                Ok(_) => {
                    warn!("RabbitMQ consumer disconnected, reconnecting...");
                }
                Err(e) => {
                    warn!("RabbitMQ consumer error: {}, reconnecting...", e);
                }
            }
            tokio::time::sleep(tokio::time::Duration::from_secs(settings.reconnect_seconds)).await;
        }
    });
}

async fn run_consumer(
    settings: &RabbitMqSettings,
    routes_cache: &RoutesCacheType,
    user_settings_cache: &UserSettingsCacheType,
) -> Result<(), lapin::Error> {
    let conn = Connection::connect(&settings.uri, ConnectionProperties::default()).await?;
    let channel = conn.create_channel().await?;

    // Declare fanout exchanges (idempotent)
    channel
        .exchange_declare(
            &settings.route_exchange,
            ExchangeKind::Fanout,
            ExchangeDeclareOptions {
                durable: true,
                ..Default::default()
            },
            FieldTable::default(),
        )
        .await?;

    channel
        .exchange_declare(
            &settings.user_settings_exchange,
            ExchangeKind::Fanout,
            ExchangeDeclareOptions {
                durable: true,
                ..Default::default()
            },
            FieldTable::default(),
        )
        .await?;

    // Declare exclusive auto-delete queues
    let routes_queue = channel
        .queue_declare(
            "",
            QueueDeclareOptions {
                exclusive: true,
                auto_delete: true,
                ..Default::default()
            },
            FieldTable::default(),
        )
        .await?;

    let user_settings_queue = channel
        .queue_declare(
            "",
            QueueDeclareOptions {
                exclusive: true,
                auto_delete: true,
                ..Default::default()
            },
            FieldTable::default(),
        )
        .await?;

    // Bind queues to exchanges
    channel
        .queue_bind(
            routes_queue.name().as_str(),
            &settings.route_exchange,
            "",
            QueueBindOptions::default(),
            FieldTable::default(),
        )
        .await?;

    channel
        .queue_bind(
            user_settings_queue.name().as_str(),
            &settings.user_settings_exchange,
            "",
            QueueBindOptions::default(),
            FieldTable::default(),
        )
        .await?;

    // Start consuming from both queues
    let mut routes_consumer = channel
        .basic_consume(
            routes_queue.name().as_str(),
            "click-router-routes",
            BasicConsumeOptions {
                no_ack: false,
                ..Default::default()
            },
            FieldTable::default(),
        )
        .await?;

    let mut user_settings_consumer = channel
        .basic_consume(
            user_settings_queue.name().as_str(),
            "click-router-user-settings",
            BasicConsumeOptions {
                no_ack: false,
                ..Default::default()
            },
            FieldTable::default(),
        )
        .await?;

    info!(
        "RabbitMQ consumer connected, queues: {}, {}",
        routes_queue.name(),
        user_settings_queue.name()
    );

    // Process messages from both consumers concurrently
    loop {
        tokio::select! {
            Some(delivery) = routes_consumer.next() => {
                match delivery {
                    Ok(delivery) => {
                        if let Ok(msg) = serde_json::from_slice::<RouteChangedMessage>(&delivery.data) {
                            if let Some((switch, link)) = msg.switch_link() {
                                info!(
                                    "Cache invalidation: route {:?} route_id={} switch={} link={}",
                                    msg.action, msg.route_id, switch, link
                                );
                                if let Err(e) = routes_cache.invalidate(&switch, &link).await {
                                    warn!("Failed to invalidate route cache: {}", e);
                                }
                            } else {
                                warn!("Route changed message missing switch/link in public payload");
                            }
                        } else {
                            warn!("Failed to deserialize route changed message");
                        }
                        delivery.ack(BasicAckOptions::default()).await?;
                    }
                    Err(e) => {
                        warn!("Route consumer delivery error: {}", e);
                        return Err(e);
                    }
                }
            }
            Some(delivery) = user_settings_consumer.next() => {
                match delivery {
                    Ok(delivery) => {
                        if let Ok(msg) = serde_json::from_slice::<UserSettingsChangedMessage>(&delivery.data) {
                            info!(
                                "Cache invalidation: user_settings {:?} user_id={}",
                                msg.action, msg.user_id
                            );
                            if let Err(e) = user_settings_cache.invalidate(&msg.user_id).await {
                                warn!("Failed to invalidate user settings cache: {}", e);
                            }
                        } else {
                            warn!("Failed to deserialize user settings changed message");
                        }
                        delivery.ack(BasicAckOptions::default()).await?;
                    }
                    Err(e) => {
                        warn!("User settings consumer delivery error: {}", e);
                        return Err(e);
                    }
                }
            }
            else => {
                break;
            }
        }
    }

    Ok(())
}
