use crate::positions::domain::entities::position::PositionUuid;
use crate::shared::domain::errors::SharedDomainError;
use thiserror::Error;

#[derive(Error, Debug, PartialEq, Clone)]
pub enum PositionDomainError {
    #[error(transparent)]
    Shared(#[from] SharedDomainError),

    #[error("Invalid status: `{0}`")]
    InvalidStatus(String),

    #[error("Invalid user uuid: `{0}`")]
    InvalidUserUuid(String),
}

impl PositionDomainError {
    pub fn invalid_date(date: String) -> Self {
        Self::Shared(SharedDomainError::InvalidDate(date))
    }

    pub fn invalid_uuid() -> Self {
        match uuid::Uuid::parse_str("invalid") {
            Ok(_) => unreachable!(),
            Err(e) => Self::Shared(SharedDomainError::InvalidUuid(e)),
        }
    }
}

impl From<uuid::Error> for PositionDomainError {
    fn from(e: uuid::Error) -> Self {
        Self::Shared(SharedDomainError::InvalidUuid(e))
    }
}

#[derive(Error, Debug, PartialEq, Clone)]
pub enum PositionRepoError {
    #[error("Database error: `{0}`")]
    DatabaseError(String),

    #[error("Error converting from database: `{0}`")]
    ConversionError(#[from] PositionDomainError),

    #[error("Position not found: `{0}`")]
    NotFound(PositionUuid),
}
