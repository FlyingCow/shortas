use std::sync::Arc;
use tokio::sync::mpsc;
use tracing::{error, info, warn};

use crate::adapters::{ChangeAction, ImageStore, RouteChangedMessage};
use crate::core::FaviconScraper;
use crate::settings::WorkerSettings;

pub struct IconWorker {
    message_rx: mpsc::Receiver<RouteChangedMessage>,
    image_store: Arc<ImageStore>,
    scraper: FaviconScraper,
}

impl IconWorker {
    pub fn new(
        message_rx: mpsc::Receiver<RouteChangedMessage>,
        image_store: Arc<ImageStore>,
        settings: &WorkerSettings,
    ) -> anyhow::Result<Self> {
        let scraper = FaviconScraper::new(
            settings.request_timeout_seconds,
            settings.max_image_size_bytes,
        )?;

        Ok(Self {
            message_rx,
            image_store,
            scraper,
        })
    }

    pub async fn run(mut self) {
        info!("Icon worker started");

        while let Some(msg) = self.message_rx.recv().await {
            self.handle_message(msg).await;
        }

        info!("Icon worker stopped");
    }

    async fn handle_message(&self, msg: RouteChangedMessage) {
        let route_id = &msg.route_id;
        let owner_id = match msg.owner_id() {
            Some(id) => id,
            None => {
                warn!("Route {} has no owner_id, skipping", route_id);
                return;
            }
        };

        match msg.action {
            ChangeAction::Created => {
                info!("Processing created route: {}", route_id);
                if let Some(dest) = msg.dest() {
                    self.scrape_and_store(owner_id, route_id, dest).await;
                } else {
                    info!("Route {} has no destination, skipping", route_id);
                }
            }
            ChangeAction::Updated => {
                let old_dest = msg.previous_dest();
                let new_dest = msg.dest();

                // Only process if destination changed
                if old_dest != new_dest {
                    info!(
                        "Processing updated route: {} (dest changed: {:?} -> {:?})",
                        route_id, old_dest, new_dest
                    );
                    if let Some(dest) = new_dest {
                        self.scrape_and_store(owner_id, route_id, dest).await;
                    } else {
                        // Destination was removed, delete the icon
                        self.delete_icon(owner_id, route_id).await;
                    }
                } else {
                    info!(
                        "Route {} destination unchanged, skipping icon update",
                        route_id
                    );
                }
            }
            ChangeAction::Deleted => {
                info!("Processing deleted route: {}", route_id);
                self.delete_icon(owner_id, route_id).await;
            }
        }
    }

    async fn scrape_and_store(&self, owner_id: &str, route_id: &str, dest: &str) {
        info!("Scraping favicon for route {} from {}", route_id, dest);

        match self.scraper.scrape_favicon(dest).await {
            Ok(result) => {
                info!(
                    "Scraped favicon for route {} ({} bytes, {})",
                    route_id,
                    result.data.len(),
                    result.content_type
                );

                if let Err(e) = self
                    .image_store
                    .upload_icon(owner_id, route_id, result.data, &result.content_type)
                    .await
                {
                    error!("Failed to upload icon for route {}: {}", route_id, e);
                }
            }
            Err(e) => {
                warn!("Failed to scrape favicon for route {}: {}", route_id, e);
            }
        }
    }

    async fn delete_icon(&self, owner_id: &str, route_id: &str) {
        if let Err(e) = self.image_store.delete_icon(owner_id, route_id).await {
            // Log but don't fail - the icon might not exist
            warn!("Failed to delete icon for route {}: {}", route_id, e);
        }
    }
}
