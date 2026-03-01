use anyhow::Result;
use mongodb::bson::doc;
use mongodb::{options::ClientOptions, Client, Collection};
use serde::{Deserialize, Serialize};
use std::time::Duration;

use crate::adapters::mongodb::settings::Mongodb;
use crate::core::challenge::{Challenge, ChallengeStore};

#[derive(Clone, Debug, Serialize, Deserialize)]
struct ChallengeDocument {
    pub domain: String,
    pub token: String,
    pub key_authorization: String,
}

#[derive(Clone, Debug)]
pub struct MongodbChallengeStore {
    collection: Collection<ChallengeDocument>,
}

impl MongodbChallengeStore {
    pub async fn new(settings: &Mongodb) -> Self {
        let mut client_options = ClientOptions::parse(&settings.uri)
            .await
            .expect("Failed to parse MongoDB URI");

        client_options.min_pool_size = Some(10);
        client_options.max_pool_size = Some(50);
        client_options.max_idle_time = Some(Duration::from_secs(300));
        client_options.connect_timeout = Some(Duration::from_secs(5));
        client_options.server_selection_timeout = Some(Duration::from_secs(5));

        let client =
            Client::with_options(client_options).expect("Failed to create MongoDB client");

        let db = client.database(&settings.database);
        let collection = db.collection::<ChallengeDocument>(&settings.challenges_collection);
        Self { collection }
    }
}

#[async_trait::async_trait()]
impl ChallengeStore for MongodbChallengeStore {
    async fn get_challenge(&self, domain: &str, token: &str) -> Result<Option<Challenge>> {
        let filter = doc! {
            "domain": domain.to_lowercase(),
            "token": token
        };

        match self.collection.find_one(filter).await? {
            Some(doc) => Ok(Some(Challenge {
                domain: doc.domain,
                token: doc.token,
                key_authorization: doc.key_authorization,
            })),
            None => Ok(None),
        }
    }
}
