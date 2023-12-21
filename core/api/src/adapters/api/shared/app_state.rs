use crate::adapters::spi::dynamo::dynamo_routes_repository::RoutesRepository;

pub struct AppState {
    pub app_name: String,
    pub routes_repository: RoutesRepository
}