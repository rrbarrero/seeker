use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use thiserror::Error;

use crate::positions::{
    application::errors::PositionServiceError, domain::errors::PositionDomainError,
};
use crate::{
    auth::domain::errors::AuthDomainError, positions::domain::entities::position::PositionUuid,
    shared::domain::errors::SharedDomainError, shared::presentation::ApiErrorResponse,
};

#[derive(Error, Debug)]
pub enum PositionApiError {
    #[error("Service error: `{0}`")]
    ServiceError(#[from] PositionServiceError),

    #[error("Position not found: `{0}`")]
    PositionNotFound(PositionUuid),

    #[error("Invalid user uuid: `{0}`")]
    AuthDomainError(#[from] AuthDomainError),

    #[error("Invalid position value: `{0}`")]
    PositionDomainError(#[from] PositionDomainError),

    #[error("Domain error: `{0}`")]
    SharedDomainError(#[from] SharedDomainError),
}

impl IntoResponse for PositionApiError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            PositionApiError::ServiceError(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()),
            PositionApiError::PositionNotFound(uuid) => (
                StatusCode::NOT_FOUND,
                format!("Position not found: {}", uuid),
            ),
            PositionApiError::AuthDomainError(e) => (StatusCode::BAD_REQUEST, e.to_string()),
            PositionApiError::PositionDomainError(e) => (StatusCode::BAD_REQUEST, e.to_string()),
            PositionApiError::SharedDomainError(e) => (StatusCode::BAD_REQUEST, e.to_string()),
        };

        (status, Json(ApiErrorResponse { message })).into_response()
    }
}
