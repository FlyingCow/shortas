use std::sync::Arc;

use anyhow::Result;
use http::StatusCode;

use crate::adapters::moka::qr_code_cache::MokaQrCodeCache;
use crate::core::{
    flow_module::{FlowModule, FlowStepContinuation},
    flow_router::{FlowRouter, FlowRouterContext, FlowRouterResult},
};

const IS_QR_REQUEST: &str = "is_qr_request";
const ORIGINAL_PATH: &str = "original_path";
const QR_SUFFIX: &str = ".qr";

/// QR Code generation module with caching support
///
/// This module intercepts requests ending with `.qr` suffix and generates
/// QR codes for the destination URLs. The module uses a Moka cache to avoid
/// regenerating QR codes for the same URLs.
#[derive(Clone)]
pub struct QrCodeModule {
    cache: Arc<MokaQrCodeCache>,
    min_size: u32,
    max_size: u32,
}

impl QrCodeModule {
    /// Creates a new QR code module with the specified cache and size parameters
    ///
    /// # Arguments
    /// * `cache` - The Moka cache for storing generated QR codes
    /// * `min_size` - Minimum dimensions for QR codes in pixels (default: 400)
    /// * `max_size` - Maximum dimensions for QR codes in pixels (default: 800)
    pub fn new(cache: Arc<MokaQrCodeCache>, min_size: u32, max_size: u32) -> Self {
        Self {
            cache,
            min_size,
            max_size,
        }
    }

    /// Creates a new QR code module with default size parameters
    pub fn with_defaults(cache: Arc<MokaQrCodeCache>) -> Self {
        Self::new(cache, 400, 800)
    }

    /// Strips the .qr suffix from a path if present
    ///
    /// # Arguments
    /// * `path` - The path to check
    ///
    /// # Returns
    /// * `Some(&str)` - The path without the .qr suffix
    /// * `None` - If the path doesn't end with .qr
    fn strip_qr_suffix(path: &str) -> Option<&str> {
        path.strip_suffix(QR_SUFFIX)
    }
}

#[async_trait::async_trait()]
impl FlowModule for QrCodeModule {
    async fn init(
        &self,
        context: &mut FlowRouterContext,
        flow_router: &FlowRouter,
    ) -> Result<FlowStepContinuation> {
        // Check if the path ends with .qr
        if let Some(original_path) = Self::strip_qr_suffix(&context.in_route.path) {
            // Clone the path early to avoid borrow checker issues
            let original_path_owned = original_path.to_string();

            // Mark this as a QR request to skip stats registration
            let out_route = flow_router.get_main_route(&original_path_owned, context).await?;

            if let Some(route) = out_route {
                // Wrap route in Arc to avoid expensive cloning later
                context.main_route = Some(Arc::new(route));
                context.add_bool(IS_QR_REQUEST, true);
                context.add_string(ORIGINAL_PATH, original_path_owned);
            }
        }

        Ok(FlowStepContinuation::Continue)
    }

    async fn handle_start(
        &self,
        context: &mut FlowRouterContext,
        _flow_router: &FlowRouter,
    ) -> Result<FlowStepContinuation> {
        // Only process if this is a QR request
        if !context.is_data_true(IS_QR_REQUEST) {
            return Ok(FlowStepContinuation::Continue);
        }

        // Build the full URL for the original path (without .qr suffix)
        if let Some(original_path) = context.get_string(ORIGINAL_PATH) {
            // Construct the full URL: scheme://host:port/path
            let qr_url = format!(
                "{}://{}:{}/{}",
                context.in_route.scheme,
                context.in_route.host,
                context.in_route.port,
                original_path
            );

            match self
                .cache
                .get_or_generate(&qr_url, self.min_size, self.max_size)
                .await
            {
                Ok(png_data) => {
                    // Clone the Arc'd data (cheap operation)
                    context.result = Some(FlowRouterResult::Image(
                        (*png_data).clone(),
                        "image/png".to_string(),
                        StatusCode::OK,
                    ));
                    return Ok(FlowStepContinuation::Break);
                }
                Err(e) => {
                    tracing::error!("Failed to generate QR code for {}: {}", qr_url, e);
                    context.result = Some(FlowRouterResult::PlainText(
                        "Failed to generate QR code".to_string(),
                        StatusCode::INTERNAL_SERVER_ERROR,
                    ));
                    return Ok(FlowStepContinuation::Break);
                }
            }
        }

        Ok(FlowStepContinuation::Continue)
    }

    async fn handle_register(
        &self,
        context: &mut FlowRouterContext,
        _flow_router: &FlowRouter,
    ) -> Result<FlowStepContinuation> {
        // Skip stats registration if this is a QR request
        if context.is_data_true(IS_QR_REQUEST) {
            return Ok(FlowStepContinuation::Break);
        }

        Ok(FlowStepContinuation::Continue)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::adapters::moka::qr_code_cache::MokaQrCodeCache;
    use crate::adapters::moka::settings::QrCodeCacheSettings;

    #[test]
    fn should_strip_qr_suffix() {
        assert_eq!(QrCodeModule::strip_qr_suffix("test.qr"), Some("test"));
        assert_eq!(
            QrCodeModule::strip_qr_suffix("path/to/link.qr"),
            Some("path/to/link")
        );
        assert_eq!(QrCodeModule::strip_qr_suffix("noqr"), None);
        assert_eq!(QrCodeModule::strip_qr_suffix("test"), None);
    }

    #[tokio::test]
    async fn should_generate_qr_code_via_cache() {
        let settings = QrCodeCacheSettings {
            max_capacity: 100,
            time_to_live_minutes: 60,
            time_to_idle_minutes: 30,
        };
        let cache = Arc::new(MokaQrCodeCache::new(settings));
        let module = QrCodeModule::with_defaults(cache);

        // Generate via cache
        let result = module
            .cache
            .get_or_generate("https://example.com", 400, 800)
            .await;

        assert!(result.is_ok());
        let png_data = result.unwrap();
        assert!(!png_data.is_empty());

        // Check PNG magic bytes
        assert_eq!(&png_data[0..8], &[137, 80, 78, 71, 13, 10, 26, 10]);
    }
}
