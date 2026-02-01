use axum::{http::StatusCode, response::IntoResponse};
use thiserror::Error;

use crate::auth::domain::entities::errors::AuthError;

#[derive(Error, Debug)]
pub enum AuthPresentationError {
    #[error("Auth error: `{0}`")]
    AuthError(#[from] AuthError),
}

impl IntoResponse for AuthPresentationError {
    fn into_response(self) -> axum::response::Response {
        match self {
            AuthPresentationError::AuthError(e) => {
                (StatusCode::BAD_REQUEST, e.to_string()).into_response()
            }
        }
    }
}
