use thiserror::Error;

#[derive(Error, Debug, PartialEq, Clone)]
pub enum SharedDomainError {
    #[error("Wrong uuid format: `{0}`")]
    InvalidUuid(#[from] uuid::Error),

    #[error("Invalid email address: `{0}`")]
    InvalidEmail(String),

    #[error("Invalid password: `{0}` is too weak")]
    InvalidPassword(String),

    #[error("Error hashing password: {0}")]
    ErrorHashingPassword(String),

    #[error("Invalid date: {0}")]
    InvalidDate(String),

    #[error("Invalid date/time value")]
    InvalidDateTime,

    #[error("Internal error: {0}")]
    InternalError(String),
}
