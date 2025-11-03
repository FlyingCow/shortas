use std::sync::Arc;
use std::time::Duration;

use anyhow::Result;
use image::Luma;
use moka::future::Cache;
use qrcode::QrCode;

use crate::core::metrics::{Timer, METRICS};

use super::settings::QrCodeCacheSettings;

/// Moka-based cache for QR code images
///
/// This cache stores generated QR codes to avoid regenerating them for the same URLs.
/// The cache uses a weigher to account for the actual memory size of PNG images.
#[derive(Clone)]
pub struct MokaQrCodeCache {
    cache: Cache<String, Arc<Vec<u8>>>,
}

impl MokaQrCodeCache {
    /// Creates a new QR code cache with the specified settings
    ///
    /// # Arguments
    /// * `settings` - Cache configuration including capacity and TTL settings
    ///
    /// # Returns
    /// * A new MokaQrCodeCache instance
    pub fn new(settings: QrCodeCacheSettings) -> Self {
        let cache = Cache::builder()
            .max_capacity(settings.max_capacity)
            .time_to_live(Duration::from_secs(settings.time_to_live_minutes * 60))
            .time_to_idle(Duration::from_secs(settings.time_to_idle_minutes * 60))
            .weigher(|key: &String, value: &Arc<Vec<u8>>| -> u32 {
                // Weight by actual memory size (key string + PNG bytes)
                let total_bytes = key.len() + value.len();
                total_bytes.try_into().unwrap_or(u32::MAX)
            })
            .build();

        Self { cache }
    }

    /// Gets a QR code from cache or generates it if not cached
    ///
    /// # Arguments
    /// * `url` - The URL to encode in the QR code
    /// * `min_size` - Minimum dimensions for the QR code in pixels
    /// * `max_size` - Maximum dimensions for the QR code in pixels
    ///
    /// # Returns
    /// * `Result<Arc<Vec<u8>>>` - Arc-wrapped PNG data for the QR code
    pub async fn get_or_generate(
        &self,
        url: &str,
        min_size: u32,
        max_size: u32,
    ) -> Result<Arc<Vec<u8>>> {
        // Create cache key that includes size parameters to handle different sizes
        let cache_key = format!("{}:{}:{}", url, min_size, max_size);
        let timer = Timer::new();

        // Check cache first
        if let Some(cached) = self.cache.get(&cache_key).await {
            METRICS.qr_cache_hits.inc();
            timer.observe_duration_seconds(&METRICS.qr_cache_lookup_duration);
            return Ok(cached);
        }

        // Cache miss - generate QR code
        METRICS.qr_cache_misses.inc();
        let gen_timer = Timer::new();

        let qr_data = Self::generate_qr_code(url, min_size, max_size)?;
        let qr_arc = Arc::new(qr_data);

        gen_timer.observe_duration_seconds(&METRICS.qr_generation_duration);

        // Store in cache
        self.cache.insert(cache_key, qr_arc.clone()).await;
        timer.observe_duration_seconds(&METRICS.qr_cache_lookup_duration);

        Ok(qr_arc)
    }

    /// Generates a QR code as PNG data
    ///
    /// # Arguments
    /// * `url` - The URL to encode
    /// * `min_size` - Minimum dimensions in pixels
    /// * `max_size` - Maximum dimensions in pixels
    ///
    /// # Returns
    /// * `Result<Vec<u8>>` - PNG-encoded QR code image
    fn generate_qr_code(url: &str, min_size: u32, max_size: u32) -> Result<Vec<u8>> {
        // Create the QR code
        let code = QrCode::new(url.as_bytes())?;

        // Render the QR code as an image
        let image = code
            .render::<Luma<u8>>()
            .min_dimensions(min_size, min_size)
            .max_dimensions(max_size, max_size)
            .build();

        // Encode the image as PNG
        let mut png_data = Vec::new();
        {
            use image::ImageEncoder;
            let encoder = image::codecs::png::PngEncoder::new(&mut png_data);
            encoder.write_image(
                image.as_raw(),
                image.width(),
                image.height(),
                image::ExtendedColorType::L8,
            )?;
        }

        Ok(png_data)
    }

    /// Invalidates a specific QR code from the cache
    ///
    /// # Arguments
    /// * `url` - The URL whose QR code should be invalidated
    /// * `min_size` - Minimum dimensions used when generating
    /// * `max_size` - Maximum dimensions used when generating
    pub async fn invalidate(&self, url: &str, min_size: u32, max_size: u32) {
        let cache_key = format!("{}:{}:{}", url, min_size, max_size);
        self.cache.invalidate(&cache_key).await;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn should_generate_and_cache_qr_code() {
        let settings = QrCodeCacheSettings {
            max_capacity: 100,
            time_to_live_minutes: 60,
            time_to_idle_minutes: 30,
        };
        let cache = MokaQrCodeCache::new(settings);

        let url = "https://example.com";
        let result = cache.get_or_generate(url, 400, 800).await;

        assert!(result.is_ok());
        let png_data = result.unwrap();
        assert!(!png_data.is_empty());

        // Verify PNG magic bytes
        assert_eq!(&png_data[0..8], &[137, 80, 78, 71, 13, 10, 26, 10]);
    }

    #[tokio::test]
    async fn should_use_cache_on_second_request() {
        let settings = QrCodeCacheSettings {
            max_capacity: 100,
            time_to_live_minutes: 60,
            time_to_idle_minutes: 30,
        };
        let cache = MokaQrCodeCache::new(settings);

        let url = "https://example.com";

        // First request - cache miss
        let first = cache.get_or_generate(url, 400, 800).await.unwrap();

        // Second request - should hit cache
        let second = cache.get_or_generate(url, 400, 800).await.unwrap();

        // Both should point to same Arc'd data
        assert!(Arc::ptr_eq(&first, &second));
    }

    #[tokio::test]
    async fn should_handle_different_sizes() {
        let settings = QrCodeCacheSettings {
            max_capacity: 100,
            time_to_live_minutes: 60,
            time_to_idle_minutes: 30,
        };
        let cache = MokaQrCodeCache::new(settings);

        let url = "https://example.com";

        let small = cache.get_or_generate(url, 200, 400).await.unwrap();
        let large = cache.get_or_generate(url, 400, 800).await.unwrap();

        // Different sizes should produce different results
        assert_ne!(small.len(), large.len());
    }
}
