use crate::shared::domain::{errors::SharedDomainError, value_objects::UserUuid};
use thiserror::Error;

#[derive(Error, Debug, PartialEq, Clone)]
pub enum AuthDomainError {
    #[error(transparent)]
    Shared(#[from] SharedDomainError),

    #[error("Internal error: {0}")]
    InternalError(String),
}

impl AuthDomainError {
    pub fn invalid_email(email: String) -> Self {
        Self::Shared(SharedDomainError::InvalidEmail(email))
    }

    pub fn invalid_password(password: String) -> Self {
        Self::Shared(SharedDomainError::InvalidPassword(password))
    }

    pub fn invalid_uuid() -> Self {
        match uuid::Uuid::parse_str("invalid") {
            Ok(_) => unreachable!(),
            Err(e) => Self::Shared(SharedDomainError::InvalidUuid(e)),
        }
    }
}

impl From<uuid::Error> for AuthDomainError {
    fn from(e: uuid::Error) -> Self {
        Self::Shared(SharedDomainError::InvalidUuid(e))
    }
}

#[derive(Error, Debug, PartialEq, Clone)]
pub enum AuthRepoError {
    #[error("Database error: `{0}`")]
    DatabaseError(String),

    #[error("Error converting from database: `{0}`")]
    ConversionError(#[from] AuthDomainError),

    #[error("User not found: `{0}`")]
    NotFound(UserUuid),

    #[error("User with email `{0}` already exists")]
    UserAlreadyExists(String),
}
