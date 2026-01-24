use std::str::FromStr;

use chrono::{DateTime, Local, NaiveDate};
use uuid::Uuid;

use crate::{
    positions::domain::entities::position_error::PositionValueError,
    shared::domain::{error::UserValueError, value_objects::UserUuid},
};

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

impl Default for AppliedOn {
    fn default() -> Self {
        let parsed_date = DateTime::parse_from_rfc2822("Fri, 23 Jan 2026 10:10:10 +0200").unwrap();
        Self {
            applied_on: parsed_date.date_naive(),
        }
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
    pub user_id: UserUuid,
    pub company: Company,
    pub role_title: RoleTitle,
    pub description: Description,
    pub applied_on: AppliedOn,
    pub url: Url,
    pub initial_comment: InitialComment,
    pub status: PositionStatus,
    pub created_at: DateTime<Local>,
    pub updated_at: DateTime<Local>,
    pub deleted_at: Option<DateTime<Local>>,
}

pub struct PositionBuilder {
    id: PositionUuid,
    user_id: UserUuid,
    company: Company,
    role_title: RoleTitle,
    description: Description,
    applied_on: AppliedOn,
    url: Url,
    initial_comment: InitialComment,
    status: PositionStatus,
    created_at: DateTime<Local>,
    updated_at: DateTime<Local>,
    deleted_at: Option<DateTime<Local>>,
}

impl PositionBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_uuid(mut self, uuid: &str) -> Result<Self, PositionValueError> {
        self.id = PositionUuid::from_str(uuid)?;
        Ok(self)
    }

    pub fn with_user_uuid(mut self, uuid: &str) -> Result<Self, UserValueError> {
        self.user_id = UserUuid::from_str(uuid)?;
        Ok(self)
    }

    pub fn with_role_title(mut self, title: &str) -> Self {
        self.role_title = RoleTitle::new(title);
        self
    }

    pub fn with_company(mut self, company: &str) -> Self {
        self.company = Company::new(company);
        self
    }

    pub fn with_description(mut self, description: &str) -> Self {
        self.description = Description::new(description);
        self
    }

    pub fn with_applied_on(mut self, applied_on: &str) -> Result<Self, PositionValueError> {
        self.applied_on = AppliedOn::new(applied_on)?;
        Ok(self)
    }

    pub fn with_url(mut self, url: &str) -> Self {
        self.url = Url::new(url);
        self
    }

    pub fn with_initial_comment(mut self, initial_comment: &str) -> Self {
        self.initial_comment = InitialComment::new(initial_comment);
        self
    }

    pub fn with_status(mut self, status: PositionStatus) -> Self {
        self.status = status;
        self
    }

    pub fn with_created_at(mut self, created_at: DateTime<Local>) -> Self {
        self.created_at = created_at;
        self
    }

    pub fn with_updated_at(mut self, updated_at: DateTime<Local>) -> Self {
        self.updated_at = updated_at;
        self
    }

    pub fn with_deleted_at(mut self, deleted_at: DateTime<Local>) -> Self {
        self.deleted_at = Some(deleted_at);
        self
    }

    pub fn build(self) -> Position {
        Position {
            id: self.id,
            user_id: self.user_id,
            company: self.company,
            role_title: self.role_title,
            description: self.description,
            applied_on: self.applied_on,
            url: self.url,
            initial_comment: self.initial_comment,
            status: self.status,
            created_at: Local::now(),
            updated_at: Local::now(),
            deleted_at: None,
        }
    }
}

impl Default for PositionBuilder {
    fn default() -> Self {
        Self {
            id: PositionUuid::new(),
            user_id: UserUuid::new(),
            company: Company::new(""),
            role_title: RoleTitle::new(""),
            description: Description::new(""),
            applied_on: AppliedOn::default(),
            url: Url::new(""),
            initial_comment: InitialComment::new(""),
            status: PositionStatus::CvSent,
            created_at: Local::now(),
            updated_at: Local::now(),
            deleted_at: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::shared::fixtures::{TESTING_UUID_1, create_fixture_position};
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
        let date = "23-1-2026";
        let result = AppliedOn::new(date);

        assert!(matches!(result, Err(PositionValueError::InvalidDate(_))));
    }

    #[test]
    fn test_create_new_position() {
        let position = create_fixture_position();

        assert_eq!(position.id.value(), uuid!(TESTING_UUID_1));
        assert_eq!(position.company.value(), "hola");
        assert_eq!(position.role_title.value(), "im the role title");
        assert_eq!(
            position.description.value(),
            "Im the description of the position"
        );
        assert_eq!(position.applied_on.value(), "2026-01-23");
        assert_eq!(position.url.value(), "https://me-the.url");
        assert_eq!(
            position.initial_comment.value(),
            "... and I the initial comment"
        );
        assert_eq!(position.status, PositionStatus::PhoneScreenScheduled);
    }
}
