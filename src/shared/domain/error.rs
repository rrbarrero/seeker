use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum UserValueError {
    #[error("Wrong uuid format: `{0}`")]
    InvalidUuid(#[from] uuid::Error),

    #[error("Invalid email address: `{0}`")]
    InvalidEmail(String),

    #[error("Invalid email password: `{0}` is too weak")]
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

#[derive(Error, Debug, PartialEq)]
pub enum AuthRepositoryError {
    #[error("Database error: {0}")]
    DatabaseError(String),

    #[error("User not found")]
    NotFound,

    #[error("Domain error: {0}")]
    DomainError(#[from] UserValueError),

    #[error("User already exists")]
    UserAlreadyExists,
}

#[derive(Error, Debug)]
pub enum PositionRepositoryError {}
