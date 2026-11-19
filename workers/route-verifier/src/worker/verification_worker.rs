use chrono::{Duration, Utc};
use std::sync::Arc;
use tokio::time::interval;
use tracing::{error, info, warn};

use crate::adapters::api::app_state::AppState;
use crate::adapters::rabbitmq::messages::RouteStatusChangedMessage;
use crate::settings::WorkerSettings;

pub struct VerificationWorker {
    app_state: Arc<AppState>,
    settings: WorkerSettings,
}

impl VerificationWorker {
    pub fn new(app_state: Arc<AppState>, settings: WorkerSettings) -> Self {
        Self { app_state, settings }
    }

    pub async fn run(self) {
        info!(
            "Route verification worker started (interval: {}s, batch size: {})",
            self.settings.check_interval_seconds, self.settings.batch_size
        );

        let mut ticker =
            interval(std::time::Duration::from_secs(self.settings.check_interval_seconds));

        loop {
            ticker.tick().await;
            self.process_batch().await;
        }
    }

    async fn process_batch(&self) {
        let now = Utc::now();
        info!("Running verification batch at {}", now);

        let routes = match self
            .app_state
            .route_store
            .get_routes_for_verification(now, self.settings.batch_size)
            .await
        {
            Ok(r) => r,
            Err(e) => {
                error!("Failed to get routes for verification: {}", e);
                return;
            }
        };

        if routes.is_empty() {
            info!("No routes due for verification");
            return;
        }

        info!("Processing {} routes for safety verification", routes.len());

        for route in routes {
            let route_id = &route.id;

            // Check all destinations against Safe Browsing
            if route.destinations.is_empty() {
                warn!("Route {} has no destinations, skipping", route_id);
                self.update_route_timestamps(route_id, false).await;
                continue;
            }

            let result = match self
                .app_state
                .safe_browsing_client
                .check_urls(&route.destinations)
                .await
            {
                Ok(r) => r,
                Err(e) => {
                    error!("Failed to check URLs for route {}: {}", route_id, e);
                    continue;
                }
            };

            let previous_status = route.status.clone();
            let was_blocked = route.is_blocked();

            if !result.is_safe {
                // URL is unsafe - block the route
                let threat_type = result.first_threat_type().unwrap_or("UNKNOWN");
                let threat_url = result.first_threat_url();
                let reason = format!("Safe Browsing: {}", threat_type);

                info!(
                    "Route {} ({}) flagged as unsafe: {} - blocking",
                    route_id, route.link, reason
                );

                // Update local status in route-verifier's collection
                if let Err(e) = self
                    .app_state
                    .route_store
                    .update_route_status(route_id, "Blocked", Some(&reason))
                    .await
                {
                    error!("Failed to update route status {}: {}", route_id, e);
                }

                // Publish status change to RabbitMQ
                // - Management API consumes this to update PostgreSQL and sync to click-router
                // - Click-router can also consume this directly to update its cache
                if let Some(ref publisher) = self.app_state.rabbitmq_publisher {
                    let next_check = Utc::now()
                        + Duration::hours(self.settings.blocked_recheck_interval_hours);

                    publisher
                        .publish_route_status_changed(&RouteStatusChangedMessage {
                            route_id: route_id.clone(),
                            link: route.link.clone(),
                            owner_id: route.owner_id.clone(),
                            workspace_id: route.workspace_id.clone(),
                            previous_status,
                            new_status: "Blocked".to_string(),
                            blocked_reason: Some(reason.clone()),
                            threat_type: Some(threat_type.to_string()),
                            threat_url: threat_url.map(|s| s.to_string()),
                            checked_at: Utc::now().timestamp_millis(),
                            next_check_at: Some(next_check.timestamp_millis()),
                        })
                        .await;
                }

                // Update safety check timestamps (blocked routes are rechecked more frequently)
                self.update_route_timestamps(route_id, true).await;
            } else {
                // URL is safe
                if was_blocked {
                    // Route was previously blocked but is now safe - could auto-unblock here
                    // For now, we just update the timestamps and let manual review handle unblocking
                    info!(
                        "Route {} ({}) was blocked but is now safe - manual review required",
                        route_id, route.link
                    );
                }

                // Update safety check timestamps (safe routes are rechecked less frequently)
                self.update_route_timestamps(route_id, false).await;
            }

            info!(
                "Route {} ({}) verification complete (safe: {})",
                route_id, route.link, result.is_safe
            );
        }
    }

    async fn update_route_timestamps(&self, route_id: &str, is_blocked: bool) {
        let now = Utc::now();
        let recheck_hours = if is_blocked {
            self.settings.blocked_recheck_interval_hours
        } else {
            self.settings.recheck_interval_hours
        };
        let next_check = now + Duration::hours(recheck_hours);

        if let Err(e) = self
            .app_state
            .route_store
            .update_safety_check_timestamps(route_id, now, next_check)
            .await
        {
            error!(
                "Failed to update safety check timestamps for route {}: {}",
                route_id, e
            );
        }
    }
}
