use std::str::FromStr;

use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier, password_hash::SaltString};
use chrono::{Local, NaiveDate};
use rand::rngs::OsRng;
use zxcvbn::{Score, zxcvbn};

use email_address::EmailAddress;

use crate::shared::domain::{error::UserValueError, value_objects::UserUuid};

#[derive(PartialEq, Debug, Clone)]
pub struct UserEmail {
    email: String,
}

impl UserEmail {
    pub fn value(&self) -> &str {
        &self.email
    }

    pub fn new(email: &str) -> Result<Self, UserValueError> {
        EmailAddress::is_valid(email)
            .then(|| Self {
                email: email.to_string(),
            })
            .ok_or_else(|| UserValueError::InvalidEmail(email.to_string()))
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

#[derive(PartialEq, Debug, Clone)]
pub struct User {
    pub id: UserUuid,
    pub email: UserEmail,
    password: UserPassword,
    pub created: NaiveDate,
    pub updated: NaiveDate,
}

impl User {
    pub fn password(&self) -> &UserPassword {
        &self.password
    }

    pub fn new(id: &str, email: &str, password: &str) -> Result<Self, UserValueError> {
        let id = UserUuid::from_str(id)?;
        let email = UserEmail::new(email)?;
        let password = UserPassword::new(password)?;
        let now = Local::now().naive_local().date();
        let created = now;
        let updated = now;
        Ok(User {
            id,
            email,
            password,
            created,
            updated,
        })
    }

    pub fn verify_password(&self, password: &str) -> Result<bool, UserValueError> {
        Ok(Argon2::default()
            .verify_password(
                password.as_bytes(),
                &PasswordHash::try_from(self.password.value())?,
            )
            .is_ok())
    }
}

#[cfg(test)]
mod tests {
    use uuid::Uuid;

    use super::*;

    // Helper to get consistent valid data for tests
    fn valid_email() -> &'static str {
        "test@example.com"
    }
    fn valid_password() -> &'static str {
        "S0m3V3ryStr0ngP@ssw0rd!"
    }
    fn valid_id() -> String {
        Uuid::new_v4().to_string()
    }

    #[test]
    fn test_wrong_uuid() {
        let id = "123";
        let result = UserUuid::from_str(id);

        assert!(matches!(result, Err(UserValueError::InvalidUuid(_))));
    }

    #[test]
    fn test_wrong_email() {
        let email = "123";
        let result = UserEmail::new(email);

        assert!(matches!(result, Err(UserValueError::InvalidEmail(_))));
    }

    #[test]
    fn test_wrong_password() {
        let password = "123";
        let result = User::new(&valid_id(), valid_email(), password);

        assert!(matches!(result, Err(UserValueError::InvalidPassword(_))));
    }

    #[test]
    fn test_user() {
        let result = User::new(&valid_id(), valid_email(), valid_password());

        assert!(result.is_ok());
    }

    #[test]
    fn test_check_password() -> Result<(), UserValueError> {
        let password = valid_password();
        let user = User::new(&valid_id(), valid_email(), password)?;
        let result = user.verify_password(password);

        assert!(matches!(result, Ok(true)));
        Ok(())
    }

    #[test]
    fn test_check_password_wrong() -> Result<(), UserValueError> {
        let user = User::new(&valid_id(), valid_email(), valid_password())?;
        let result = user.verify_password("123");

        assert!(matches!(result, Ok(false)));
        Ok(())
    }

    #[test]
    fn test_not_ascii_password() -> Result<(), UserValueError> {
        let password = "ñÑ☢️fhadsfhKJHlkfhjvnluYu,....";
        let user = User::new(&valid_id(), valid_email(), password)?;
        let result = user.verify_password(password);

        assert!(matches!(result, Ok(true)));
        Ok(())
    }
}
