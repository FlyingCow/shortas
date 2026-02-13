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
        let credentials = Credentials::new(
            &settings.access_key,
            &settings.secret_key,
            None,
            None,
            "route-icon-worker",
        );

        let config = aws_sdk_s3::Config::builder()
            .behavior_version(BehaviorVersion::latest())
            .endpoint_url(&settings.endpoint)
            .region(Region::new(settings.region.clone()))
            .credentials_provider(credentials)
            .force_path_style(true)
            .build();

        let client = Client::from_conf(config);

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
