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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_invalid_uuid_error() {
        let uuid_error = uuid::Uuid::parse_str("invalid").unwrap_err();
        let error = SharedDomainError::from(uuid_error);
        assert!(error.to_string().contains("Wrong uuid format"));
    }

    #[test]
    fn test_invalid_email_error() {
        let error = SharedDomainError::InvalidEmail("bad@".to_string());
        assert_eq!(error.to_string(), "Invalid email address: `bad@`");
    }

    #[test]
    fn test_invalid_password_error() {
        let error = SharedDomainError::InvalidPassword("weak".to_string());
        assert!(error.to_string().contains("too weak"));
    }

    #[test]
    fn test_error_hashing_password() {
        let error = SharedDomainError::ErrorHashingPassword("hash failed".to_string());
        assert!(error.to_string().contains("hash failed"));
    }

    #[test]
    fn test_invalid_date_error() {
        let error = SharedDomainError::InvalidDate("2026-99-99".to_string());
        assert!(error.to_string().contains("2026-99-99"));
    }

    #[test]
    fn test_invalid_datetime_error() {
        let error = SharedDomainError::InvalidDateTime;
        assert_eq!(error.to_string(), "Invalid date/time value");
    }

    #[test]
    fn test_internal_error() {
        let error = SharedDomainError::InternalError("something broke".to_string());
        assert!(error.to_string().contains("something broke"));
    }
}
