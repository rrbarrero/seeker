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
