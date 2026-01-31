use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use thiserror::Error;

use crate::positions::domain::entities::position::PositionUuid;
use crate::positions::domain::entities::position_error::PositionServiceError;

#[derive(Error, Debug)]
pub enum PositionPresentationError {
    #[error("Service error: `{0}`")]
    ServiceError(#[from] PositionServiceError),

    #[error("Position not found: `{0}`")]
    PositionNotFound(PositionUuid),
}

impl IntoResponse for PositionPresentationError {
    fn into_response(self) -> Response {
        match self {
            PositionPresentationError::ServiceError(error) => {
                (StatusCode::INTERNAL_SERVER_ERROR, error.to_string()).into_response()
            }
            PositionPresentationError::PositionNotFound(position_uuid) => {
                (StatusCode::NOT_FOUND, position_uuid.to_string()).into_response()
            }
        }
    }
}
