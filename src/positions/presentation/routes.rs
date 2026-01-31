use std::sync::Arc;

use axum::{
    Router,
    routing::{delete, get, post},
};

use crate::positions::{
    application::position_service::PositionService,
    presentation::handlers::{get_position, get_positions, remove_position, save_position},
};

pub fn position_routes(service: Arc<PositionService>) -> Router<Arc<PositionService>> {
    Router::new()
        .route("/", get(get_positions))
        .route("/:position_id", get(get_position))
        .route("/", post(save_position))
        .route("/:position_id", delete(remove_position))
        .with_state(service)
}
