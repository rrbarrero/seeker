use std::sync::Arc;

use axum::extract::FromRef;

use crate::positions::application::position_service::PositionService;

#[derive(Clone)]
pub struct AppState {
    position_service: Arc<PositionService>,
}

impl AppState {
    pub fn new(position_service: Arc<PositionService>) -> Self {
        Self { position_service }
    }
}

impl FromRef<AppState> for Arc<PositionService> {
    fn from_ref(state: &AppState) -> Self {
        state.position_service.clone()
    }
}
