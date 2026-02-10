pub mod comment_handlers;
pub mod comment_routes;
pub mod dtos;
pub mod errors;
pub mod handlers;
pub mod routes;

use std::sync::Arc;

use axum::Router;

use crate::{
    positions::{
        application::{comment_service::CommentService, position_service::PositionService},
        presentation::routes::create_position_routes,
    },
    shared::config::Config,
};

pub fn build_router(
    service: Arc<PositionService>,
    comment_service: Arc<CommentService>,
    config: Arc<Config>,
) -> Router {
    Router::new().nest(
        "/positions",
        create_position_routes(service, comment_service, config),
    )
}
