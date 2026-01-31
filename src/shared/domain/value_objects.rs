use std::{fmt::Display, str::FromStr};

use uuid::Uuid;

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
