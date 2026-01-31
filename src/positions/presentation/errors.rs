use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use thiserror::Error;

use crate::positions::domain::entities::position_error::{
    PositionServiceError, PositionValueError,
};
use crate::{
    positions::domain::entities::position::PositionUuid, shared::domain::error::UserValueError,
};

#[derive(Error, Debug)]
pub enum PositionPresentationError {
    #[error("Service error: `{0}`")]
    ServiceError(#[from] PositionServiceError),

    #[error("Position not found: `{0}`")]
    PositionNotFound(PositionUuid),

    #[error("Invalid user uuid: `{0}`")]
    UserValueError(#[from] UserValueError),

    #[error("Invalid position value: `{0}`")]
    PositionValueError(#[from] PositionValueError),
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
            PositionPresentationError::UserValueError(user_uuid) => {
                (StatusCode::BAD_REQUEST, user_uuid.to_string()).into_response()
            }
            PositionPresentationError::PositionValueError(position_uuid) => {
                (StatusCode::BAD_REQUEST, position_uuid.to_string()).into_response()
            }
        }
    }
}
