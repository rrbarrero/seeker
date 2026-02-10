use std::str::FromStr;

use chrono::{DateTime, Local};
use uuid::Uuid;

use crate::{
    positions::domain::entities::position::PositionUuid,
    positions::domain::errors::CommentDomainError, shared::domain::value_objects::UserUuid,
};

#[derive(PartialEq, Clone, Debug, Copy)]
pub struct CommentUuid {
    id: Uuid,
}

impl Default for CommentUuid {
    fn default() -> Self {
        Self::new()
    }
}

impl CommentUuid {
    pub fn value(&self) -> Uuid {
        self.id
    }

    pub fn new() -> Self {
        CommentUuid { id: Uuid::new_v4() }
    }
}

impl FromStr for CommentUuid {
    type Err = CommentDomainError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let id = Uuid::parse_str(s)?;
        Ok(CommentUuid { id })
    }
}

impl std::fmt::Display for CommentUuid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.id)
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct CommentBody {
    body: String,
}

impl CommentBody {
    pub fn value(&self) -> &str {
        &self.body
    }

    pub fn new(body: &str) -> Self {
        CommentBody {
            body: body.to_string(),
        }
    }
}

impl std::fmt::Display for CommentBody {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.body)
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Comment {
    pub id: CommentUuid,
    pub position_id: PositionUuid,
    pub user_id: UserUuid,
    pub body: CommentBody,
    pub created_at: DateTime<Local>,
    pub updated_at: DateTime<Local>,
}

pub struct CommentBuilder {
    id: CommentUuid,
    position_id: PositionUuid,
    user_id: UserUuid,
    body: CommentBody,
    created_at: DateTime<Local>,
    updated_at: DateTime<Local>,
}

impl CommentBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_uuid(mut self, uuid: &str) -> Result<Self, CommentDomainError> {
        self.id = CommentUuid::from_str(uuid)?;
        Ok(self)
    }

    pub fn with_position_uuid(mut self, uuid: &str) -> Result<Self, CommentDomainError> {
        let id = Uuid::parse_str(uuid)?;
        self.position_id = PositionUuid::from_uuid(id);
        Ok(self)
    }

    pub fn with_user_uuid(mut self, uuid: &str) -> Result<Self, CommentDomainError> {
        self.user_id = UserUuid::from_str(uuid)?;
        Ok(self)
    }

    pub fn with_body(mut self, body: &str) -> Self {
        self.body = CommentBody::new(body);
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

    pub fn build(self) -> Comment {
        Comment {
            id: self.id,
            position_id: self.position_id,
            user_id: self.user_id,
            body: self.body,
            created_at: self.created_at,
            updated_at: self.updated_at,
        }
    }
}

impl Default for CommentBuilder {
    fn default() -> Self {
        Self {
            id: CommentUuid::new(),
            position_id: PositionUuid::new(),
            user_id: UserUuid::new(),
            body: CommentBody::new(""),
            created_at: Local::now(),
            updated_at: Local::now(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::shared::domain::errors::SharedDomainError;

    use super::*;

    #[test]
    fn test_wrong_uuid() {
        let id = "123";
        let result = CommentUuid::from_str(id);

        assert!(matches!(
            result,
            Err(CommentDomainError::Shared(SharedDomainError::InvalidUuid(
                _
            )))
        ));
    }

    #[test]
    fn test_comment_body_value() {
        let body = CommentBody::new("hello");
        assert_eq!(body.value(), "hello");
    }

    #[test]
    fn test_create_new_comment() {
        let comment = CommentBuilder::new().with_body("hello").build();

        assert_eq!(comment.body.value(), "hello");
    }
}
