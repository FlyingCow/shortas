use anyhow::Result;

use aws_config::SdkConfig;

use aws_sdk_kinesis::{/*types::ShardIteratorType,*/ Client};

use crate::{core::hit_stream::BaseHitStream, model::Hit};

use super::settings::HitStreamConfig;


#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct KinesisHitStream {
    client: Client,
    settings: HitStreamConfig,
}

impl KinesisHitStream {
    pub fn new(sdk_config: &SdkConfig, settings: HitStreamConfig) -> Self {
        Self {
            settings,
            client: Client::new(sdk_config),
        }
    }
}

#[async_trait::async_trait()]
impl BaseHitStream for KinesisHitStream {
    async fn pull(&self) -> Result<Vec<Hit>> {

        todo!("implement kinesis provider!")
        // let describe_result = self.client.describe_stream()
        //     .stream_name(&self.settings.stream_name)
        //     .send()
        //     .await?
        //     .stream_description.unwrap();

        // let shard_iter = self.client.get_shard_iterator()
        //     .stream_name(&self.settings.stream_name)
        //     .shard_id(describe_result.shards().first().unwrap().shard_id.clone())
        //     .shard_iterator_type(ShardIteratorType::Latest)
        //     .send()
        //     .await?
        //     .shard_iterator.unwrap();

        // let items = self.client
        //     .get_records()
        //     .shard_iterator(shard_iter)
        //     .limit(100)
        //     .send()
        //     .await?;
        // Ok(vec![])
    }
}