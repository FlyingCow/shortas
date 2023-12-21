use crate::adapters::spi::dynamo::mappers::route_mapper::RouteMapper;
use crate::application::{
    mappers::db_mapper::ToEntityMapper,
    repositories::routes_repository_abstract::RoutesRepositoryAbstract
};
use crate::domain::route_entity::RouteEntity;

use std::collections::HashMap;
use std::error::Error;

use async_trait::async_trait;
use aws_sdk_dynamodb::types::AttributeValue;
use aws_sdk_dynamodb::{Client, Error as DynamoError};

pub struct RoutesRepositoryConfig {
    client: Client,
    routes_table: String, 
}

pub struct RoutesRepository {
    config: RoutesRepositoryConfig,
}

#[async_trait(?Send)]
impl RoutesRepositoryAbstract for RoutesRepository {
    async fn get_route(
        &self,
        switch: String,
        link: String,
    ) -> Result<Option<RouteEntity>, Box<dyn Error>> {

        let item = self
            .config
            .client
            .get_item()
            .table_name(&self.config.routes_table)
            .set_key(Some(HashMap::from([
                ("link".to_string(), AttributeValue::S(link)),
                ("switch".to_string(), AttributeValue::S(switch))])))
            .send()
            .await;

        let result = match item {
            Ok(i) => Ok(i),
            Err(e) => {
                let err = e;
                Err(err)
            }
        };

        Ok(RouteMapper::to_entity(result.unwrap()))
    }

    async fn create_route(&self, route: RouteEntity) -> Result<(), Box<dyn Error>> {
        Ok(())
    }

    async fn update_route(&self, route: RouteEntity) -> Result<(), Box<dyn Error>> {
        Ok(())
    }

    async fn delete_route(&self, switch: String, link: String) -> Result<(), Box<dyn Error>> {
        Ok(())
    }
}


impl RoutesRepositoryConfig {
    pub fn new(client: Client, routes_table: String) -> Self {
        RoutesRepositoryConfig { 
            routes_table,
            client, }
    }
}

impl RoutesRepository {
    pub fn new(config: RoutesRepositoryConfig) -> Self {
        RoutesRepository { config }
    }
}
