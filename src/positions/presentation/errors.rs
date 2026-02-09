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
    positions::domain::entities::position::PositionUuid, shared::domain::errors::SharedDomainError,
    shared::presentation::ApiErrorResponse,
};

#[derive(Error, Debug)]
pub enum PositionApiError {
    #[error("Service error: `{0}`")]
    ServiceError(#[from] PositionServiceError),

    #[error("Position not found: `{0}`")]
    PositionNotFound(PositionUuid),

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
            PositionApiError::PositionDomainError(e) => (StatusCode::BAD_REQUEST, e.to_string()),
            PositionApiError::SharedDomainError(e) => (StatusCode::BAD_REQUEST, e.to_string()),
        };

        (status, Json(ApiErrorResponse { message })).into_response()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::body::Body;
    use axum::http::Response;

    fn response_status(response: Response<Body>) -> StatusCode {
        response.status()
    }

    #[test]
    fn test_position_not_found_response() {
        let uuid = PositionUuid::new();
        let error = PositionApiError::PositionNotFound(uuid);
        let response = error.into_response();
        assert_eq!(response_status(response), StatusCode::NOT_FOUND);
    }

    #[test]
    fn test_position_domain_error_response() {
        let domain_error = PositionDomainError::InvalidStatus("bad".to_string());
        let error = PositionApiError::from(domain_error);
        let response = error.into_response();
        assert_eq!(response_status(response), StatusCode::BAD_REQUEST);
    }

    #[test]
    fn test_shared_domain_error_response() {
        let shared_error = SharedDomainError::InvalidDate("bad-date".to_string());
        let error = PositionApiError::from(shared_error);
        let response = error.into_response();
        assert_eq!(response_status(response), StatusCode::BAD_REQUEST);
    }

    #[test]
    fn test_position_api_error_display() {
        let uuid = PositionUuid::new();
        let error = PositionApiError::PositionNotFound(uuid);
        assert!(error.to_string().contains("Position not found"));
    }
}
