use std::{fmt::Display, str::FromStr};

use argon2::{Argon2, PasswordHasher, password_hash::SaltString};
use rand::rngs::OsRng;
use uuid::Uuid;
use zxcvbn::{Score, zxcvbn};

use crate::shared::domain::error::UserValueError;

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
    type Err = UserValueError;

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

    pub fn new(password: &str) -> Result<Self, UserValueError> {
        (zxcvbn(password, &[]).score() >= Score::Three)
            .then(|| {
                let hashed = Self::hash_password(password)?;
                Ok(Self { password: hashed })
            })
            .ok_or_else(|| UserValueError::InvalidPassword(password.to_string()))?
    }

    pub fn hash_password(password: &str) -> Result<String, UserValueError> {
        let salt = SaltString::generate(&mut OsRng);

        let argon2 = Argon2::default();

        let password_hash = argon2
            .hash_password(password.as_bytes(), &salt)
            .map_err(UserValueError::ErrorHashingPassword)?
            .to_string();

        Ok(password_hash)
    }
}
