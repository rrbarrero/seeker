use thiserror::Error;

#[derive(Error, Debug)]
pub enum UserValueError {
    #[error("Wrong uuid format: `{0}`")]
    InvalidUuid(#[from] uuid::Error),
    #[error("Invalid email address: `{0}`")]
    InvalidEmail(String),
    #[error("Invalid email password: `{0}` is too weak")]
    InvalidPassword(String),
}

#[derive(Error, Debug)]
pub enum PositionRepositoryError {}
