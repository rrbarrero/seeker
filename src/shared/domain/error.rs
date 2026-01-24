use thiserror::Error;

#[derive(Error, Debug)]
pub enum UserValueError {
    #[error("Wrong uuid format: `{0}`")]
    InvalidUuid(#[from] uuid::Error),
    #[error("Invalid email address: `{0}`")]
    InvalidEmail(String),
    #[error("Invalid email password: `{0}` is too weak")]
    InvalidPassword(String),
    #[error("Error hashing the password: {0:?}")]
    ErrorHashingPassword(#[from] argon2::password_hash::Error),
}

#[derive(Error, Debug)]
pub enum PositionRepositoryError {}
