use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use thiserror::Error;

use crate::positions::{
    application::errors::{CommentServiceError, PositionServiceError},
    domain::entities::position::PositionUuid,
    domain::errors::{CommentDomainError, PositionDomainError},
};
use crate::{
    positions::domain::entities::comment::CommentUuid, shared::domain::errors::SharedDomainError,
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

#[derive(Error, Debug)]
pub enum CommentApiError {
    #[error("Service error: `{0}`")]
    ServiceError(#[from] CommentServiceError),

    #[error("Position service error: `{0}`")]
    PositionServiceError(#[from] PositionServiceError),

    #[error("Forbidden")]
    Forbidden,

    #[error("Comment not found: `{0}`")]
    CommentNotFound(CommentUuid),

    #[error("Position not found: `{0}`")]
    PositionNotFound(PositionUuid),

    #[error("Invalid comment value: `{0}`")]
    CommentDomainError(#[from] CommentDomainError),

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

impl IntoResponse for CommentApiError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            CommentApiError::ServiceError(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()),
            CommentApiError::PositionServiceError(e) => {
                (StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
            }
            CommentApiError::Forbidden => (StatusCode::FORBIDDEN, "Forbidden".to_string()),
            CommentApiError::CommentNotFound(uuid) => (
                StatusCode::NOT_FOUND,
                format!("Comment not found: {}", uuid),
            ),
            CommentApiError::PositionNotFound(uuid) => (
                StatusCode::NOT_FOUND,
                format!("Position not found: {}", uuid),
            ),
            CommentApiError::CommentDomainError(e) => (StatusCode::BAD_REQUEST, e.to_string()),
            CommentApiError::SharedDomainError(e) => (StatusCode::BAD_REQUEST, e.to_string()),
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

    #[test]
    fn test_comment_not_found_response() {
        let uuid = CommentUuid::new();
        let error = CommentApiError::CommentNotFound(uuid);
        let response = error.into_response();
        assert_eq!(response_status(response), StatusCode::NOT_FOUND);
    }

    #[test]
    fn test_comment_domain_error_response() {
        let domain_error = CommentDomainError::invalid_uuid();
        let error = CommentApiError::from(domain_error);
        let response = error.into_response();
        assert_eq!(response_status(response), StatusCode::BAD_REQUEST);
    }

    #[test]
    fn test_comment_api_error_display() {
        let uuid = CommentUuid::new();
        let error = CommentApiError::CommentNotFound(uuid);
        assert!(error.to_string().contains("Comment not found"));
    }

    #[test]
    fn test_comment_forbidden_response() {
        let error = CommentApiError::Forbidden;
        let response = error.into_response();
        assert_eq!(response_status(response), StatusCode::FORBIDDEN);
    }
}
