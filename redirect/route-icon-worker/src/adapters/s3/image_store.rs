use anyhow::Result;
use aws_sdk_s3::{
    config::{BehaviorVersion, Credentials, Region},
    primitives::ByteStream,
    Client,
};
use tracing::{error, info};

use crate::settings::S3Settings;

#[derive(Clone)]
pub struct ImageStore {
    client: Client,
    bucket: String,
}

impl ImageStore {
    pub async fn new(settings: &S3Settings) -> Result<Self> {
        let region = Region::new(settings.region.clone());

        // Build S3 config based on whether explicit credentials are provided
        let client = if let (Some(access_key), Some(secret_key)) = (&settings.access_key, &settings.secret_key) {
            // Use explicit credentials (for MinIO/LocalStack)
            let credentials = Credentials::new(
                access_key,
                secret_key,
                None,
                None,
                "route-icon-worker",
            );

            let mut config_builder = aws_sdk_s3::Config::builder()
                .behavior_version(BehaviorVersion::latest())
                .region(region)
                .credentials_provider(credentials)
                .force_path_style(settings.use_path_style);

            // Set custom endpoint if provided (MinIO/LocalStack)
            if let Some(endpoint) = &settings.endpoint {
                config_builder = config_builder.endpoint_url(endpoint);
            }

            Client::from_conf(config_builder.build())
        } else {
            // Use AWS default credential chain (IAM roles, env vars, etc.)
            info!("Using AWS default credential chain for S3 access");
            let aws_config = aws_config::from_env()
                .region(region)
                .load()
                .await;

            let mut config_builder = aws_sdk_s3::config::Builder::from(&aws_config)
                .behavior_version(BehaviorVersion::latest())
                .force_path_style(settings.use_path_style);

            // Set custom endpoint if provided
            if let Some(endpoint) = &settings.endpoint {
                config_builder = config_builder.endpoint_url(endpoint);
            }

            Client::from_conf(config_builder.build())
        };

        Ok(Self {
            client,
            bucket: settings.bucket.clone(),
        })
    }

    pub async fn upload_icon(
        &self,
        owner_id: &str,
        route_id: &str,
        data: Vec<u8>,
        content_type: &str,
    ) -> Result<()> {
        let key = format!("{}/{}/fav.ico", owner_id, route_id);

        info!(
            "Uploading icon to s3://{}/{} ({} bytes, {})",
            self.bucket,
            key,
            data.len(),
            content_type
        );

        self.client
            .put_object()
            .bucket(&self.bucket)
            .key(&key)
            .body(ByteStream::from(data))
            .content_type(content_type)
            .send()
            .await
            .map_err(|e| {
                error!("Failed to upload icon: {}", e);
                anyhow::anyhow!("S3 upload failed: {}", e)
            })?;

        info!("Successfully uploaded icon to {}/{}", self.bucket, key);
        Ok(())
    }

    pub async fn delete_icon(&self, owner_id: &str, route_id: &str) -> Result<()> {
        let key = format!("{}/{}/fav.ico", owner_id, route_id);

        info!("Deleting icon from s3://{}/{}", self.bucket, key);

        self.client
            .delete_object()
            .bucket(&self.bucket)
            .key(&key)
            .send()
            .await
            .map_err(|e| {
                error!("Failed to delete icon: {}", e);
                anyhow::anyhow!("S3 delete failed: {}", e)
            })?;

        info!("Successfully deleted icon from {}/{}", self.bucket, key);
        Ok(())
    }
}
