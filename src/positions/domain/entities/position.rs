use std::str::FromStr;

use chrono::{DateTime, NaiveDate};
use uuid::Uuid;

use crate::positions::domain::entities::errors::PositionValueError;

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

#[derive(PartialEq, Debug, Clone)]
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

#[derive(Debug, PartialEq, Clone)]
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

#[derive(Debug, PartialEq, Clone)]
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

#[derive(Debug, PartialEq, Clone)]
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

#[derive(Debug, PartialEq, Clone)]
pub struct Url {
    url: String,
}

impl Url {
    pub fn value(&self) -> &str {
        &self.url
    }

    pub fn new(url: &str) -> Self {
        Url {
            url: url.to_string(),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
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

#[derive(Debug, PartialEq, Clone)]
pub enum PositionStatus {
    CvSent,
    PhoneScreenScheduled,
    TechnicalInterview,
    OfferReceived,
    Rejected,
    Withdrawn,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Position {
    pub id: PositionUuid,
    pub company: Company,
    pub role_title: RoleTitle,
    pub description: Description,
    pub applied_on: AppliedOn,
    pub url: Url,
    pub initial_comment: InitialComment,
    pub status: PositionStatus,
}

pub struct PositionBuilder {
    position: Position,
}

impl PositionBuilder {
    pub fn new() -> Self {
        let id = PositionUuid::new();
        let company = Company::new("");
        let role_title = RoleTitle::new("");
        let description = Description::new("");
        let applied_on = AppliedOn::new("Tue, 1 Jul 2003 10:52:37 +0200").unwrap();
        let url = Url::new("");
        let initial_comment = InitialComment::new("");
        let status = PositionStatus::CvSent;
        PositionBuilder {
            position: Position {
                id,
                company,
                role_title,
                description,
                applied_on,
                url,
                initial_comment,
                status,
            },
        }
    }
    pub fn with_uuid(&mut self, uuid: &str) -> &mut Self {
        self.position.id = PositionUuid::from_str(uuid).unwrap();
        self
    }
    pub fn with_role_title(&mut self, title: &str) -> &mut Self {
        self.position.role_title = RoleTitle::new(title);
        self
    }
    pub fn with_company(&mut self, company: &str) -> &mut Self {
        self.position.company = Company::new(company);
        self
    }
    pub fn with_description(&mut self, description: &str) -> &mut Self {
        self.position.description = Description::new(description);
        self
    }
    pub fn with_applied_on(&mut self, applied_on: &str) -> &mut Self {
        self.position.applied_on = AppliedOn::new(applied_on).unwrap();
        self
    }
    pub fn with_url(&mut self, url: &str) -> &mut Self {
        self.position.url = Url::new(url);
        self
    }
    pub fn with_initial_comment(&mut self, initial_comment: &str) -> &mut Self {
        self.position.initial_comment = InitialComment::new(initial_comment);
        self
    }
    pub fn with_status(&mut self, status: PositionStatus) -> &mut Self {
        self.position.status = status;
        self
    }
    pub fn build(&self) -> Position {
        self.position.clone()
    }
}

impl Default for PositionBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use crate::shared::fixtures::{TESTING_UUID, create_fixture_position};
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
        assert_eq!(position.status, PositionStatus::PhoneScreenScheduled);
    }
}
