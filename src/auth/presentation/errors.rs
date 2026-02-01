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
