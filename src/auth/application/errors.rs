use crate::auth::domain::errors::{AuthDomainError, AuthRepoError};
use thiserror::Error;

#[derive(Debug, Error, PartialEq, Clone)]
pub enum AuthError {
    #[error("Internal error: `{0}`")]
    InternalError(String),

    #[error("Authentication failed: invalid credentials")]
    InvalidCredentials,

    #[error("Registration failed: user already exists")]
    UserAlreadyExists,

    #[error("Token expired")]
    TokenExpired,

    #[error("Invalid token")]
    InvalidToken,

    #[error("Domain error: `{0}`")]
    DomainError(#[from] AuthDomainError),

    #[error("Repository error: `{0}`")]
    RepositoryError(#[from] AuthRepoError),
}

impl AuthError {
    pub fn invalid_email(email: String) -> Self {
        Self::DomainError(AuthDomainError::invalid_email(email))
    }

    pub fn invalid_password(password: String) -> Self {
        Self::DomainError(AuthDomainError::invalid_password(password))
    }
}

impl From<AuthError> for axum::http::StatusCode {
    fn from(error: AuthError) -> Self {
        match error {
            AuthError::InternalError(_) => Self::INTERNAL_SERVER_ERROR,
            AuthError::InvalidCredentials => Self::UNAUTHORIZED,
            AuthError::UserAlreadyExists => Self::BAD_REQUEST,
            AuthError::TokenExpired => Self::UNAUTHORIZED,
            AuthError::InvalidToken => Self::UNAUTHORIZED,
            AuthError::DomainError(_) => Self::BAD_REQUEST,
            AuthError::RepositoryError(_) => Self::INTERNAL_SERVER_ERROR,
        }
    }
}

impl axum::response::IntoResponse for AuthError {
    fn into_response(self) -> axum::response::Response {
        let status = axum::http::StatusCode::from(self.clone());
        let body = serde_json::json!({
            "error": self.to_string(),
        });
        (status, axum::Json(body)).into_response()
    }
}
