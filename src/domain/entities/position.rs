use std::str::FromStr;

use chrono::{DateTime, NaiveDate};
use uuid::Uuid;

use crate::domain::entities::errors::PositionValueError;

#[derive(PartialEq, Clone, Debug)]
pub struct PositionUuid {
    id: Uuid,
}

impl Default for PositionUuid {
    fn default() -> Self {
        Self::new()
    }
}

impl PositionUuid {
    pub fn value(&self) -> Uuid {
        self.id
    }

    pub fn new() -> Self {
        PositionUuid { id: Uuid::new_v4() }
    }
}

impl FromStr for PositionUuid {
    type Err = PositionValueError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let id = Uuid::parse_str(s)?;
        Ok(PositionUuid { id })
    }
}

#[derive(PartialEq, Debug)]
pub struct Company {
    name: String,
}

impl Company {
    pub fn value(&self) -> &str {
        &self.name
    }

    pub fn new(name: &str) -> Self {
        Company {
            name: name.to_string(),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct RoleTitle {
    title: String,
}

impl RoleTitle {
    pub fn value(&self) -> &str {
        &self.title
    }

    pub fn new(title: &str) -> Self {
        RoleTitle {
            title: title.to_string(),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Description {
    description: String,
}

impl Description {
    pub fn value(&self) -> &str {
        &self.description
    }

    pub fn new(description: &str) -> Self {
        Description {
            description: description.to_string(),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct AppliedOn {
    applied_on: NaiveDate,
}

impl AppliedOn {
    pub fn value(&self) -> String {
        self.applied_on.to_string()
    }

    pub fn new(applied_on: &str) -> Result<Self, PositionValueError> {
        let parsed_date = DateTime::parse_from_rfc2822(applied_on)?;
        Ok(AppliedOn {
            applied_on: parsed_date.date_naive(),
        })
    }
}

#[derive(Debug, PartialEq)]
pub struct URL {
    url: String,
}

impl URL {
    pub fn value(&self) -> &str {
        &self.url
    }

    pub fn new(url: &str) -> Self {
        URL {
            url: url.to_string(),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct InitialComment {
    initial_comment: String,
}

impl InitialComment {
    pub fn value(&self) -> &str {
        &self.initial_comment
    }

    pub fn new(comment: &str) -> Self {
        InitialComment {
            initial_comment: comment.to_string(),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Position {
    pub id: PositionUuid,
    pub company: Company,
    pub role_title: RoleTitle,
    pub description: Description,
    pub applied_on: AppliedOn,
    pub url: URL,
    pub initial_comment: InitialComment,
}

impl Position {
    pub fn new(
        company: Company,
        role_title: RoleTitle,
        description: Description,
        applied_on: AppliedOn,
        url: URL,
        initial_comment: InitialComment,
    ) -> Result<Self, PositionValueError> {
        let uuid = PositionUuid::new().value().to_string();
        Self::new_with_uuid(
            &uuid,
            company,
            role_title,
            description,
            applied_on,
            url,
            initial_comment,
        )
    }

    pub fn new_with_uuid(
        uuid: &str,
        company: Company,
        role_title: RoleTitle,
        description: Description,
        applied_on: AppliedOn,
        url: URL,
        initial_comment: InitialComment,
    ) -> Result<Self, PositionValueError> {
        Ok(Position {
            id: PositionUuid::from_str(uuid)?,
            company,
            role_title,
            description,
            applied_on,
            url,
            initial_comment,
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::utils::fixtures::{TESTING_UUID, create_fixture_position};
    use uuid::uuid;

    use super::*;

    #[test]
    fn test_wrong_uuid() {
        let id = "123";
        let result = PositionUuid::from_str(id);

        assert!(matches!(result, Err(PositionValueError::InvalidUuid(_))));
    }

    #[test]
    fn test_wrong_date() {
        let date = "30-2-2027";
        let result = AppliedOn::new(date);

        assert!(matches!(result, Err(PositionValueError::InvalidDate(_))));
    }

    #[test]
    fn test_create_new_position() {
        let position = create_fixture_position();

        assert_eq!(position.id.value(), uuid!(TESTING_UUID));
        assert_eq!(position.company.value(), "hola");
        assert_eq!(position.role_title.value(), "im the role title");
        assert_eq!(
            position.description.value(),
            "Im the description of the position"
        );
        assert_eq!(position.applied_on.value(), "2003-07-01");
        assert_eq!(position.url.value(), "https://me-the.url");
        assert_eq!(
            position.initial_comment.value(),
            "... and I the initial comment"
        );
    }
}
