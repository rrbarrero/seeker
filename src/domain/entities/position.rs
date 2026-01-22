use chrono::{DateTime, NaiveDate};
use uuid::Uuid;

use crate::domain::entities::errors::PositionValueError;

struct PositionUuid {
    id: Uuid,
}

impl PositionUuid {
    pub fn value(&self) -> Uuid {
        self.id
    }

    pub fn new() -> Self {
        PositionUuid { id: Uuid::new_v4() }
    }

    pub fn from_str(uuid: &str) -> Result<Self, PositionValueError> {
        let id = Uuid::parse_str(uuid)?;
        Ok(PositionUuid { id })
    }
}

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

pub struct Position {
    id: PositionUuid,
    company: Company,
    role_title: RoleTitle,
    description: Description,
    applied_on: AppliedOn,
    url: URL,
    initial_comment: InitialComment,
}

impl Position {
    pub fn new(
        company: Company,
        role_title: RoleTitle,
        description: Description,
        applied_on: AppliedOn,
        url: URL,
        initial_comment: InitialComment,
    ) -> Self {
        Position {
            id: PositionUuid::new(),
            company,
            role_title,
            description,
            applied_on,
            url,
            initial_comment,
        }
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
    use crate::utils::fixtures::TESTING_UUID;
    use uuid::uuid;

    use super::*;

    #[test]
    fn test_wrong_uuid(){
        let id = "123";
        let result: Result<PositionUuid, PositionValueError> = PositionUuid::from_str(id);

        assert!(matches!(result, Err(PositionValueError::InvalidUuid(_))));
    }

    #[test]
    fn test_create_new_position() {
        let company = Company::new("hola");
        let role_title = RoleTitle::new("im the role title");
        let description = Description::new("Im the description of the position");
        let applied_on = AppliedOn::new("Tue, 1 Jul 2003 10:52:37 +0200").unwrap();
        let url = URL::new("https://me-the.url");
        let initial_comment = InitialComment::new("... and I the initial comment");

        let position = Position::new_with_uuid(
            TESTING_UUID,
            company,
            role_title,
            description,
            applied_on,
            url,
            initial_comment,
        ).unwrap();

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
