use crate::adapters::api::{
    routes::routes_payload::RoutePayload,
    routes::routes_presenter::RoutePresenter
};
use crate::application::mappers::api_mapper::ApiMapper;
use crate::domain::route_entity::RouteEntity;

pub struct RoutePresenterMapper {}

impl ApiMapper<RouteEntity, RoutePresenter, RoutePayload> for RoutePresenterMapper {
    fn to_api(entity: RouteEntity) -> RoutePresenter {
        RoutePresenter {
            switch: entity.switch,
            link: entity.link,
            dest: entity.dest.unwrap_or_default(),
        }
    }

    fn to_entity(payload: RoutePayload) -> RouteEntity {
        panic!("Not Implemented!");
    }
}

