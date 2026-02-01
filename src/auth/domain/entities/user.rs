use std::str::FromStr;

use argon2::{Argon2, PasswordHash, PasswordVerifier};
use chrono::{Local, NaiveDate};

use email_address::EmailAddress;

use crate::shared::domain::{
    error::UserValueError,
    value_objects::{UserPassword, UserUuid},
};

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

    pub fn load_existing(
        id: &str,
        email: &str,
        password: &str,
        created: NaiveDate,
        updated: NaiveDate,
    ) -> Result<Self, UserValueError> {
        let id = UserUuid::from_str(id)?;
        let email = UserEmail::new(email)?;
        let password = UserPassword::set_password_already_hashed(password);
        Ok(Self {
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

    #[test]
    fn test_load_existing() -> Result<(), UserValueError> {
        let id = valid_id();
        let email = valid_email();
        let password = valid_password();
        let created = Local::now().naive_local().date();
        let updated = Local::now().naive_local().date();

        let user = User::load_existing(
            &id,
            email,
            &UserPassword::hash_password(password)?,
            created,
            updated,
        )?;

        assert_eq!(user.id.value().to_string(), id);
        assert_eq!(user.email.value(), email);
        assert_ne!(user.password.value(), password);
        assert_eq!(user.created, created);
        assert_eq!(user.updated, updated);

        assert!(user.verify_password(password)?);
        assert!(!user.verify_password("123")?);

        Ok(())
    }
}
