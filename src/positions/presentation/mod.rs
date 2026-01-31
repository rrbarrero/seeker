pub mod dtos;
pub mod errors;
pub mod handlers;
pub mod routes;

use std::sync::Arc;

use axum::Router;

use crate::positions::{
    application::position_service::PositionService, presentation::routes::position_routes,
};

pub fn build_router(service: Arc<PositionService>) -> Router {
    Router::new().nest("/positions", position_routes(service))
}
