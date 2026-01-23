use std::str::FromStr;

extern crate zxcvbn;

use zxcvbn::{Score, zxcvbn};

use email_address::EmailAddress;
use uuid::Uuid;

use crate::auth::domain::entities::error::UserValueError;

#[derive(PartialEq, Clone, Debug)]
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
        let estimate = zxcvbn(password, &[]);
        if estimate.score() >= Score::Three {
            Ok(UserPassword {
                password: password.to_string(),
            })
        } else {
            Err(UserValueError::InvalidPassword(password.to_string()))
        }
    }
}

#[derive(PartialEq, Debug, Clone)]
pub struct User {
    pub id: UserUuid,
    pub email: UserEmail,
    password: UserPassword,
}

impl User {
    pub fn new(id: &str, email: &str, password: &str) -> Result<Self, UserValueError> {
        let id = UserUuid::from_str(id)?;
        let email = UserEmail::new(email)?;
        let password = UserPassword::new(password)?;
        Ok(User {
            id,
            email,
            password,
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        auth::domain::entities::error::UserValueError,
        shared::fixtures::{TESTING_EMAIL, TESTING_PASSWORD, TESTING_UUID},
    };

    use super::*;

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
        let result = User::new(TESTING_UUID, TESTING_EMAIL, password);

        assert!(matches!(result, Err(UserValueError::InvalidPassword(_))));
    }

    #[test]
    fn test_user() {
        let result = User::new(TESTING_UUID, TESTING_EMAIL, TESTING_PASSWORD);

        assert!(matches!(result, Ok(_)));
    }
}
