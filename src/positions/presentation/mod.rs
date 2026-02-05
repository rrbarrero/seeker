pub mod dtos;
pub mod errors;
pub mod handlers;
pub mod routes;

use std::sync::Arc;

use axum::Router;

use crate::{
    positions::{
        application::position_service::PositionService,
        presentation::routes::create_position_routes,
    },
    shared::config::Config,
};

pub fn build_router(service: Arc<PositionService>, config: Arc<Config>) -> Router {
    Router::new().nest("/positions", create_position_routes(service, config))
}
