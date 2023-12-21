use crate::application::mappers::db_mapper::ToEntityMapper;
use crate::domain::route_entity::RouteEntity;

use aws_sdk_dynamodb::operation::get_item::GetItemOutput;

pub struct RouteMapper {}

impl ToEntityMapper<GetItemOutput, RouteEntity> for RouteMapper {
    fn to_entity(model: GetItemOutput) -> Option<RouteEntity> {
        model.item.map(|item| 
            RouteEntity::new(
                item.get("switch").unwrap().as_s().unwrap().to_ascii_lowercase(),
                item.get("link").unwrap().as_s().unwrap().to_ascii_lowercase(),
                item.get("dest").unwrap().as_s().map_or(
                    None, 
                    |d| Some(d.to_ascii_lowercase())))
            )
    }
}
