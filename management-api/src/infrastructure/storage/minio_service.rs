//! MinIO/S3 service for object storage (QR codes, images).

use aws_config::BehaviorVersion;
use aws_sdk_s3::presigning::PresigningConfig;
use aws_sdk_s3::Client;
use chrono::{DateTime, Duration, Utc};
use std::time::Duration as StdDuration;

use crate::settings::MinioSettings;

/// MinIO/S3 storage service.
pub struct MinioService {
    /// Client for server-side operations (uses internal endpoint)
    client: Client,
    /// Client for generating presigned URLs (uses public endpoint)
    presign_client: Client,
    bucket: String,
}

/// Presigned URL response.
#[derive(Debug, Clone)]
pub struct PresignedUrl {
    pub url: String,
    pub expires_at: DateTime<Utc>,
}

impl MinioService {
    /// Create a new MinIO service.
    pub async fn new(settings: &MinioSettings) -> anyhow::Result<Self> {
        let credentials = aws_sdk_s3::config::Credentials::new(
            &settings.access_key,
            &settings.secret_key,
            None,
            None,
            "minio",
        );

        // Client for server-side operations (internal endpoint)
        let config = aws_config::defaults(BehaviorVersion::latest())
            .endpoint_url(&settings.endpoint)
            .region(aws_config::Region::new(settings.region.clone()))
            .credentials_provider(credentials.clone())
            .load()
            .await;

        let s3_config = aws_sdk_s3::config::Builder::from(&config)
            .force_path_style(true)
            .build();

        let client = Client::from_conf(s3_config);

        // Client for presigned URLs (public endpoint for browser access)
        let public_endpoint = settings
            .public_endpoint
            .clone()
            .unwrap_or_else(|| settings.endpoint.clone());

        let presign_config = aws_config::defaults(BehaviorVersion::latest())
            .endpoint_url(&public_endpoint)
            .region(aws_config::Region::new(settings.region.clone()))
            .credentials_provider(credentials)
            .load()
            .await;

        let presign_s3_config = aws_sdk_s3::config::Builder::from(&presign_config)
            .force_path_style(true)
            .build();

        let presign_client = Client::from_conf(presign_s3_config);

        Ok(Self {
            client,
            presign_client,
            bucket: settings.bucket.clone(),
        })
    }

    /// Generate a presigned URL for uploading a file.
    /// Uses the public endpoint client so signatures are valid for browser uploads.
    pub async fn get_upload_url(
        &self,
        key: &str,
        content_type: &str,
        expires_in_seconds: u64,
    ) -> anyhow::Result<PresignedUrl> {
        let presigning_config = PresigningConfig::builder()
            .expires_in(StdDuration::from_secs(expires_in_seconds))
            .build()?;

        let presigned = self
            .presign_client
            .put_object()
            .bucket(&self.bucket)
            .key(key)
            .content_type(content_type)
            .presigned(presigning_config)
            .await?;

        let expires_at = Utc::now() + Duration::seconds(expires_in_seconds as i64);

        Ok(PresignedUrl {
            url: presigned.uri().to_string(),
            expires_at,
        })
    }

    /// Generate a presigned URL for downloading a file.
    /// Uses the public endpoint client so URLs are accessible from browsers.
    pub async fn get_download_url(
        &self,
        key: &str,
        expires_in_seconds: u64,
    ) -> anyhow::Result<PresignedUrl> {
        let presigning_config = PresigningConfig::builder()
            .expires_in(StdDuration::from_secs(expires_in_seconds))
            .build()?;

        let presigned = self
            .presign_client
            .get_object()
            .bucket(&self.bucket)
            .key(key)
            .presigned(presigning_config)
            .await?;

        let expires_at = Utc::now() + Duration::seconds(expires_in_seconds as i64);

        Ok(PresignedUrl {
            url: presigned.uri().to_string(),
            expires_at,
        })
    }

    /// Delete an object.
    pub async fn delete_object(&self, key: &str) -> anyhow::Result<()> {
        self.client
            .delete_object()
            .bucket(&self.bucket)
            .key(key)
            .send()
            .await?;

        Ok(())
    }

    /// Check if an object exists.
    pub async fn object_exists(&self, key: &str) -> anyhow::Result<bool> {
        let result = self
            .client
            .head_object()
            .bucket(&self.bucket)
            .key(key)
            .send()
            .await;

        match result {
            Ok(_) => Ok(true),
            Err(_) => Ok(false),
        }
    }

    /// Generate QR code upload URL for a route.
    /// Key format: {owner_id}/{route_id}/qr.svg (matches C# API)
    pub async fn get_qr_upload_url(&self, owner_id: &str, route_id: &str) -> anyhow::Result<PresignedUrl> {
        let key = format!("{}/{}/qr.svg", owner_id, route_id);
        self.get_upload_url(&key, "image/svg+xml", 900).await // 15 minutes
    }

    /// Generate QR logo upload URL for a route.
    /// Key format: {owner_id}/{route_id}/qr-logo.{extension} (matches C# API)
    pub async fn get_qr_logo_upload_url(
        &self,
        owner_id: &str,
        route_id: &str,
        extension: &str,
    ) -> anyhow::Result<PresignedUrl> {
        let key = format!("{}/{}/qr-logo.{}", owner_id, route_id, extension);
        let content_type = match extension {
            "png" => "image/png",
            "jpg" | "jpeg" => "image/jpeg",
            "gif" => "image/gif",
            "webp" => "image/webp",
            "svg" => "image/svg+xml",
            _ => "image/png",
        };
        self.get_upload_url(&key, content_type, 900).await // 15 minutes
    }

    /// Get QR code download URL for a route.
    /// Key format: {owner_id}/{route_id}/qr.svg (matches C# API)
    pub async fn get_qr_download_url(&self, owner_id: &str, route_id: &str) -> anyhow::Result<Option<PresignedUrl>> {
        let key = format!("{}/{}/qr.svg", owner_id, route_id);
        if self.object_exists(&key).await? {
            Ok(Some(self.get_download_url(&key, 3600).await?))
        } else {
            Ok(None)
        }
    }

    /// Health check.
    pub async fn health_check(&self) -> anyhow::Result<bool> {
        let result = self.client.list_buckets().send().await;
        Ok(result.is_ok())
    }
}
