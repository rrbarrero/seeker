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

#[cfg(test)]
mod tests {
    use super::*;
    use axum::http::StatusCode;

    #[test]
    fn test_invalid_email_creates_domain_error() {
        let error = AuthError::invalid_email("bad@".to_string());
        assert!(matches!(error, AuthError::DomainError(_)));
    }

    #[test]
    fn test_invalid_password_creates_domain_error() {
        let error = AuthError::invalid_password("weak".to_string());
        assert!(matches!(error, AuthError::DomainError(_)));
    }

    #[test]
    fn test_status_code_from_internal_error() {
        let error = AuthError::InternalError("test".to_string());
        assert_eq!(StatusCode::from(error), StatusCode::INTERNAL_SERVER_ERROR);
    }

    #[test]
    fn test_status_code_from_invalid_credentials() {
        let error = AuthError::InvalidCredentials;
        assert_eq!(StatusCode::from(error), StatusCode::UNAUTHORIZED);
    }

    #[test]
    fn test_status_code_from_user_already_exists() {
        let error = AuthError::UserAlreadyExists;
        assert_eq!(StatusCode::from(error), StatusCode::BAD_REQUEST);
    }

    #[test]
    fn test_status_code_from_token_expired() {
        let error = AuthError::TokenExpired;
        assert_eq!(StatusCode::from(error), StatusCode::UNAUTHORIZED);
    }

    #[test]
    fn test_status_code_from_invalid_token() {
        let error = AuthError::InvalidToken;
        assert_eq!(StatusCode::from(error), StatusCode::UNAUTHORIZED);
    }

    #[test]
    fn test_auth_error_display() {
        let error = AuthError::InvalidCredentials;
        assert_eq!(
            error.to_string(),
            "Authentication failed: invalid credentials"
        );
    }
}
