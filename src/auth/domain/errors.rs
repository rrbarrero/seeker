use crate::shared::domain::{errors::SharedDomainError, value_objects::UserUuid};
use thiserror::Error;

#[derive(Error, Debug, PartialEq, Clone)]
pub enum AuthDomainError {
    #[error(transparent)]
    Shared(#[from] SharedDomainError),

    #[error("Internal error: {0}")]
    InternalError(String),
}

#[derive(Error, Debug, PartialEq, Clone)]
pub enum EmailError {
    #[error("Failed to send email: {0}")]
    SendError(String),
    #[error("Invalid email data: {0}")]
    InvalidData(String),
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_invalid_email_creates_shared_error() {
        let error = AuthDomainError::invalid_email("bad@".to_string());
        assert!(matches!(error, AuthDomainError::Shared(_)));
        assert!(error.to_string().contains("bad@"));
    }

    #[test]
    fn test_invalid_password_creates_shared_error() {
        let error = AuthDomainError::invalid_password("weak".to_string());
        assert!(matches!(error, AuthDomainError::Shared(_)));
    }

    #[test]
    fn test_invalid_uuid_creates_shared_error() {
        let error = AuthDomainError::invalid_uuid();
        assert!(matches!(error, AuthDomainError::Shared(_)));
    }

    #[test]
    fn test_internal_error() {
        let error = AuthDomainError::InternalError("test".to_string());
        assert_eq!(error.to_string(), "Internal error: test");
    }

    #[test]
    fn test_uuid_error_conversion() {
        let uuid_error = uuid::Uuid::parse_str("invalid").unwrap_err();
        let error = AuthDomainError::from(uuid_error);
        assert!(matches!(error, AuthDomainError::Shared(_)));
    }

    #[test]
    fn test_auth_repo_database_error() {
        let error = AuthRepoError::DatabaseError("connection failed".to_string());
        assert_eq!(error.to_string(), "Database error: `connection failed`");
    }

    #[test]
    fn test_auth_repo_not_found_error() {
        let user_id = UserUuid::new();
        let error = AuthRepoError::NotFound(user_id);
        assert!(error.to_string().contains("User not found"));
    }

    #[test]
    fn test_auth_repo_user_already_exists() {
        let error = AuthRepoError::UserAlreadyExists("test@example.com".to_string());
        assert!(error.to_string().contains("already exists"));
    }

    #[test]
    fn test_auth_repo_conversion_error() {
        let domain_error = AuthDomainError::InternalError("test".to_string());
        let error = AuthRepoError::from(domain_error);
        assert!(matches!(error, AuthRepoError::ConversionError(_)));
    }
}
