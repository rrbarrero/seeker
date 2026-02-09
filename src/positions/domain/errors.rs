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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_invalid_date_error() {
        let error = PositionDomainError::invalid_date("bad-date".to_string());
        assert!(matches!(error, PositionDomainError::Shared(_)));
        assert!(error.to_string().contains("bad-date"));
    }

    #[test]
    fn test_invalid_uuid_error() {
        let error = PositionDomainError::invalid_uuid();
        assert!(matches!(error, PositionDomainError::Shared(_)));
    }

    #[test]
    fn test_invalid_status_error() {
        let error = PositionDomainError::InvalidStatus("unknown".to_string());
        assert_eq!(error.to_string(), "Invalid status: `unknown`");
    }

    #[test]
    fn test_invalid_user_uuid_error() {
        let error = PositionDomainError::InvalidUserUuid("bad-uuid".to_string());
        assert_eq!(error.to_string(), "Invalid user uuid: `bad-uuid`");
    }

    #[test]
    fn test_uuid_error_conversion() {
        let uuid_error = uuid::Uuid::parse_str("invalid").unwrap_err();
        let error = PositionDomainError::from(uuid_error);
        assert!(matches!(error, PositionDomainError::Shared(_)));
    }

    #[test]
    fn test_position_repo_database_error() {
        let error = PositionRepoError::DatabaseError("connection failed".to_string());
        assert_eq!(error.to_string(), "Database error: `connection failed`");
    }

    #[test]
    fn test_position_repo_not_found_error() {
        let position_id = PositionUuid::new();
        let error = PositionRepoError::NotFound(position_id);
        assert!(error.to_string().contains("Position not found"));
    }

    #[test]
    fn test_position_repo_conversion_error() {
        let domain_error = PositionDomainError::InvalidStatus("bad".to_string());
        let error = PositionRepoError::from(domain_error);
        assert!(matches!(error, PositionRepoError::ConversionError(_)));
    }
}
