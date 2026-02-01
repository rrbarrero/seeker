use thiserror::Error;

use crate::shared::domain::error::{AuthRepositoryError, UserValueError};

#[derive(Debug, Error, PartialEq)]
pub enum AuthError {
    #[error("Invalid credentials")]
    InvalidCredentials,
    #[error("Invalid token")]
    InvalidToken,
    #[error("Token expired")]
    TokenExpired,
    #[error("Internal server error")]
    InternalServerError,
    #[error("Invalid email")]
    InvalidEmail,
    #[error("Invalid password")]
    InvalidPassword,
}

#[derive(Debug, Error, PartialEq)]
pub enum AuthRegisterError {
    #[error("Invalid user values")]
    InvalidUserValues(#[from] UserValueError),

    #[error("Error saving user")]
    ErrorSavingUser(#[from] AuthRepositoryError),
}
