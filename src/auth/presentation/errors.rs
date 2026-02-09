use axum::{Json, http::StatusCode, response::IntoResponse};
use thiserror::Error;

use crate::auth::application::errors::AuthError;
use crate::shared::presentation::ApiErrorResponse;

#[derive(Error, Debug)]
pub enum AuthApiError {
    #[error("Auth error: `{0}`")]
    AuthError(#[from] AuthError),
}

impl IntoResponse for AuthApiError {
    fn into_response(self) -> axum::response::Response {
        let (status, message) = match self {
            AuthApiError::AuthError(e) => (StatusCode::BAD_REQUEST, e.to_string()),
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
    fn test_auth_api_error_response() {
        let auth_error = AuthError::InvalidCredentials;
        let api_error = AuthApiError::from(auth_error);
        let response = api_error.into_response();
        assert_eq!(response_status(response), StatusCode::BAD_REQUEST);
    }

    #[test]
    fn test_auth_api_error_display() {
        let auth_error = AuthError::InvalidCredentials;
        let api_error = AuthApiError::from(auth_error);
        assert!(api_error.to_string().contains("invalid credentials"));
    }
}
