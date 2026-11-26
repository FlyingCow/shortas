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

use crate::adapters::moka::conversion_cache::MokaConversionCache;
use crate::adapters::{CryptoCacheType, RoutesCacheType, UserSettingsCacheType};
use crate::core::{crypto::CryptoCache, routes::RoutesCache, user_settings::UserSettingsCache};

use super::{
    messages::{CertificateChangedMessage, ConversionChangedMessage, RouteChangedMessage, UserSettingsChangedMessage},
    settings::RabbitMqSettings,
};

pub fn start_cache_invalidation_consumer(
    settings: RabbitMqSettings,
    routes_cache: RoutesCacheType,
    user_settings_cache: UserSettingsCacheType,
    crypto_cache: Option<CryptoCacheType>,
    conversion_cache: Option<MokaConversionCache>,
) {
    tokio::spawn(async move {
        loop {
            info!("RabbitMQ consumer connecting...");
            match run_consumer(
                &settings,
                &routes_cache,
                &user_settings_cache,
                &crypto_cache,
                &conversion_cache,
            )
            .await {
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
    crypto_cache: &Option<CryptoCacheType>,
    conversion_cache: &Option<MokaConversionCache>,
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

    channel
        .exchange_declare(
            &settings.certificate_exchange,
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
            &settings.conversion_exchange,
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

    let certificates_queue = channel
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

    let conversions_queue = channel
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
            certificates_queue.name().as_str(),
            &settings.certificate_exchange,
            "",
            QueueBindOptions::default(),
            FieldTable::default(),
        )
        .await?;

    channel
        .queue_bind(
            conversions_queue.name().as_str(),
            &settings.conversion_exchange,
            "",
            QueueBindOptions::default(),
            FieldTable::default(),
        )
        .await?;

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

    let mut certificates_consumer = channel
        .basic_consume(
            certificates_queue.name().as_str(),
            "click-router-certificates",
            BasicConsumeOptions {
                no_ack: false,
                ..Default::default()
            },
            FieldTable::default(),
        )
        .await?;

    let mut conversions_consumer = channel
        .basic_consume(
            conversions_queue.name().as_str(),
            "click-router-conversions",
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
            Some(delivery) = certificates_consumer.next() => {
                match delivery {
                    Ok(delivery) => {
                        if let Ok(msg) = serde_json::from_slice::<CertificateChangedMessage>(&delivery.data) {
                            info!("Cache invalidation: certificate {:?} domain={}", msg.action, msg.domain);
                            if let Some(crypto_cache) = crypto_cache {
                                if let Err(e) = crypto_cache.invalidate(&msg.domain).await {
                                    warn!("Failed to invalidate certificate cache: {}", e);
                                }
                            }
                        } else {
                            warn!("Failed to deserialize certificate changed message");
                        }
                        delivery.ack(BasicAckOptions::default()).await?;
                    }
                    Err(e) => {
                        warn!("Certificate consumer delivery error: {}", e);
                        return Err(e);
                    }
                }
            }
            Some(delivery) = conversions_consumer.next() => {
                match delivery {
                    Ok(delivery) => {
                        if let Ok(msg) = serde_json::from_slice::<ConversionChangedMessage>(&delivery.data) {
                            info!("Cache invalidation: conversion {}", msg.public_key);
                            if let Some(conversion_cache) = conversion_cache {
                                conversion_cache.invalidate(&msg.public_key).await;
                            }
                        } else {
                            warn!("Failed to deserialize conversion changed message");
                        }
                        delivery.ack(BasicAckOptions::default()).await?;
                    }
                    Err(e) => {
                        warn!("Conversion consumer delivery error: {}", e);
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
