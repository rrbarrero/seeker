use std::{fmt::Display, str::FromStr};

use argon2::{Argon2, PasswordHasher, password_hash::SaltString};
use rand::rngs::OsRng;
use uuid::Uuid;
use zxcvbn::{Score, zxcvbn};

use crate::shared::domain::errors::SharedDomainError;

#[derive(PartialEq, Clone, Debug, Copy)]
pub struct UserUuid {
    id: Uuid,
}

impl Default for UserUuid {
    fn default() -> Self {
        Self::new()
    }
}

impl UserUuid {
    pub fn value(&self) -> Uuid {
        self.id
    }

    pub fn new() -> Self {
        UserUuid { id: Uuid::new_v4() }
    }
}

impl FromStr for UserUuid {
    type Err = SharedDomainError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let id = Uuid::parse_str(s)?;
        Ok(UserUuid { id })
    }
}

impl Display for UserUuid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.id)
    }
}

#[derive(PartialEq, Debug, Clone)]
pub struct UserPassword {
    password: String,
}

impl UserPassword {
    pub fn value(&self) -> &str {
        &self.password
    }

    pub fn set_password_already_hashed(password: &str) -> Self {
        Self {
            password: password.to_string(),
        }
    }

    pub fn new(password: &str) -> Result<Self, SharedDomainError> {
        (zxcvbn(password, &[]).score() >= Score::Three)
            .then(|| {
                let hashed = Self::hash_password(password)?;
                Ok(Self { password: hashed })
            })
            .ok_or_else(|| SharedDomainError::InvalidPassword(password.to_string()))?
    }

    pub fn hash_password(password: &str) -> Result<String, SharedDomainError> {
        let salt = SaltString::generate(&mut OsRng);

        let argon2 = Argon2::default();

        let password_hash = argon2
            .hash_password(password.as_bytes(), &salt)
            .map_err(|e| SharedDomainError::ErrorHashingPassword(e.to_string()))?
            .to_string();

        Ok(password_hash)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_user_uuid_new() {
        let uuid1 = UserUuid::new();
        let uuid2 = UserUuid::new();
        assert_ne!(uuid1, uuid2);
    }

    #[test]
    fn test_user_uuid_default() {
        let uuid = UserUuid::default();
        assert!(!uuid.to_string().is_empty());
    }

    #[test]
    fn test_user_uuid_value() {
        let uuid = UserUuid::new();
        let value = uuid.value();
        assert_eq!(value.to_string(), uuid.to_string());
    }

    #[test]
    fn test_user_uuid_from_str_valid() {
        let uuid_str = "550e8400-e29b-41d4-a716-446655440000";
        let result = UserUuid::from_str(uuid_str);
        if let Ok(uuid) = result {
            assert_eq!(uuid.to_string(), uuid_str);
        } else {
            panic!("Expected valid uuid");
        }
    }

    #[test]
    fn test_user_uuid_from_str_invalid() {
        let result = UserUuid::from_str("invalid-uuid");
        assert!(result.is_err());
    }

    #[test]
    fn test_user_uuid_display() {
        let uuid = UserUuid::new();
        let display = format!("{}", uuid);
        assert!(!display.is_empty());
    }

    #[test]
    fn test_user_password_new_valid() {
        let result = UserPassword::new("S0m3V3ryStr0ngP@ssw0rd!");
        assert!(result.is_ok());
    }

    #[test]
    fn test_user_password_new_weak() {
        let result = UserPassword::new("weak");
        assert!(result.is_err());
    }

    #[test]
    fn test_user_password_value() {
        let password = UserPassword::set_password_already_hashed("hashed_value");
        assert_eq!(password.value(), "hashed_value");
    }

    #[test]
    fn test_user_password_set_already_hashed() {
        let password = UserPassword::set_password_already_hashed("$argon2id$...");
        assert_eq!(password.value(), "$argon2id$...");
    }

    #[test]
    fn test_user_password_hash_password() {
        let result = UserPassword::hash_password("S0m3V3ryStr0ngP@ssw0rd!");
        match result {
            Ok(hash) => assert!(hash.starts_with("$argon2")),
            Err(_) => panic!("Expected successful hashing"),
        }
    }
}
