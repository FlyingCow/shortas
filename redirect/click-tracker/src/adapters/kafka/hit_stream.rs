use anyhow::Result;

use crate::{core::hit_stream::BaseHitStream, model::Hit};

use super::settings::HitStreamConfig;


#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct KafkaHitStream {
    settings: HitStreamConfig,
}

impl KafkaHitStream {
    pub fn new(settings: HitStreamConfig) -> Self {
        Self {
            settings,
        }
    }
}

#[async_trait::async_trait()]
impl BaseHitStream for KafkaHitStream {
    async fn pull(&self) -> Result<Vec<Hit>> {

        todo!("implement kafka provider!")
    }
}