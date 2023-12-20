use async_trait::async_trait;

use crate::{
    application::{
        repositories::routes_repository_abstract::RoutesRepositoryAbstract,
        usecases::interfaces::AbstractUseCase, 
        utils::error_handling_utils::ErrorHandlingUtils,
    },
    domain::{error::ApiError, route_entity::RouteEntity},
};

pub struct GetRouteBySwitchAndLink<'a> {
    switch: String,
    link: String,
    repository: &'a dyn RoutesRepositoryAbstract,
}

impl<'a> GetRouteBySwitchAndLink<'a> {
    pub fn new(
        switch: String,
        link: String,
        repository: &'a dyn RoutesRepositoryAbstract,
    ) -> Self {
        GetRouteBySwitchAndLink {
            switch,
            link,
            repository,
        }
    }
}

#[async_trait(?Send)]
impl<'a> AbstractUseCase<RouteEntity> for GetRouteBySwitchAndLink<'a> {
    async fn execute(self) -> Result<RouteEntity, ApiError> {
        let route_result = self.repository.get_route(self.switch, self.link).await;

        let route_option = match route_result {
            Ok(route) => route,
            Err(e) => return Err(ErrorHandlingUtils::application_error(
                "Cannot get route",
                Some(e),
            )),
        };

        route_option.map_or(Err(ErrorHandlingUtils::not_found()), |r| Ok(r))
    }
}
