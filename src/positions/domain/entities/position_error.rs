use thiserror::Error;

use crate::positions::domain::entities::position::PositionUuid;

#[derive(Error, Debug)]
pub enum PositionValueError {
    #[error("Wrong uuid format: `{0}`")]
    InvalidUuid(#[from] uuid::Error),

    #[error("Wrong date format: `{0}`")]
    InvalidDate(String),

    #[error("Invalid status: `{0}`")]
    InvalidStatus(String),

    #[error("Invalid user uuid: `{0}`")]
    InvalidUserUuid(String),
}

#[derive(Error, Debug)]
pub enum PositionRepositoryError {
    #[error("Database error: `{0}`")]
    DatabaseError(String),

    #[error("Error converting from database: `{0}`")]
    ConversionError(#[from] PositionValueError),

    #[error("Position not found: `{0}`")]
    NotFound(PositionUuid),
}

#[derive(Error, Debug)]
pub enum PositionServiceError {
    #[error("Repository error: `{0}`")]
    RepositoryError(#[from] PositionRepositoryError),
}
