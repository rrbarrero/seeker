use std::str::FromStr;

use serde::Deserialize;
use uuid::Uuid;

use crate::{
    positions::{
        domain::entities::comment::{Comment, CommentBody, CommentUuid},
        domain::entities::position::{
            AppliedOn, Company, Description, Position, PositionStatus, PositionUuid, RoleTitle, Url,
        },
        presentation::errors::{CommentApiError, PositionApiError},
    },
    shared::domain::{errors::SharedDomainError, value_objects::UserUuid},
};
use utoipa::ToSchema;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, ToSchema)]
pub struct PositionResponseDto {
    pub id: String,
    pub user_id: String,
    pub company: String,
    pub role_title: String,
    pub description: String,
    pub applied_on: String,
    pub url: String,
    pub status: String,
    pub created_at: String,
    pub updated_at: String,
    pub deleted_at: Option<String>,
    pub deleted: bool,
}

impl From<&Position> for PositionResponseDto {
    fn from(position: &Position) -> Self {
        Self {
            id: position.id.to_string(),
            user_id: position.user_id.to_string(),
            company: position.company.to_string(),
            role_title: position.role_title.to_string(),
            description: position.description.to_string(),
            applied_on: position.applied_on.to_string(),
            url: position.url.to_string(),
            status: position.status.to_string(),
            created_at: position.created_at.to_string(),
            updated_at: position.updated_at.to_string(),
            deleted_at: position.deleted_at.map(|date| date.to_string()),
            deleted: position.deleted,
        }
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, ToSchema)]
pub struct PositionUuidDto {
    id: String,
}

impl TryFrom<PositionUuidDto> for PositionUuid {
    type Error = PositionApiError;

    fn try_from(val: PositionUuidDto) -> Result<Self, Self::Error> {
        Ok(PositionUuid::from_str(&val.id)?)
    }
}

impl From<PositionUuid> for PositionUuidDto {
    fn from(val: PositionUuid) -> Self {
        Self {
            id: val.to_string(),
        }
    }
}

impl PositionUuidDto {
    pub fn to_position_uuid(&self) -> Result<PositionUuid, SharedDomainError> {
        let id = Uuid::parse_str(&self.id)?;
        Ok(PositionUuid::from_uuid(id))
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, ToSchema)]
pub struct CommentResponseDto {
    pub id: String,
    pub position_id: String,
    pub user_id: String,
    pub body: String,
    pub created_at: String,
    pub updated_at: String,
}

impl From<&Comment> for CommentResponseDto {
    fn from(comment: &Comment) -> Self {
        Self {
            id: comment.id.to_string(),
            position_id: comment.position_id.to_string(),
            user_id: comment.user_id.to_string(),
            body: comment.body.to_string(),
            created_at: comment.created_at.to_string(),
            updated_at: comment.updated_at.to_string(),
        }
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, ToSchema)]
pub struct CommentUuidDto {
    id: String,
}

impl TryFrom<CommentUuidDto> for CommentUuid {
    type Error = CommentApiError;

    fn try_from(val: CommentUuidDto) -> Result<Self, Self::Error> {
        Ok(CommentUuid::from_str(&val.id)?)
    }
}

impl From<CommentUuid> for CommentUuidDto {
    fn from(val: CommentUuid) -> Self {
        Self {
            id: val.to_string(),
        }
    }
}

#[derive(Deserialize, ToSchema)]
pub struct SavePositionRequestDto {
    pub company: String,
    pub role_title: String,
    pub description: String,
    pub applied_on: String,
    pub url: String,
    pub status: String,
}

impl SavePositionRequestDto {
    pub fn to_new_position(&self, user_id: UserUuid) -> Result<Position, PositionApiError> {
        let position = Position {
            id: PositionUuid::new(),
            user_id,
            company: Company::new(&self.company),
            role_title: RoleTitle::new(&self.role_title),
            description: Description::new(&self.description),
            applied_on: AppliedOn::new(&self.applied_on)?,
            url: Url::new(&self.url),
            status: PositionStatus::from_str(&self.status)?,
            created_at: chrono::Local::now(),
            updated_at: chrono::Local::now(),
            deleted_at: None,
            deleted: false,
        };
        Ok(position)
    }
}

#[derive(Deserialize, ToSchema)]
pub struct UpdatePositionRequestDto {
    pub company: String,
    pub role_title: String,
    pub description: String,
    pub applied_on: String,
    pub url: String,
    pub status: String,
}

impl UpdatePositionRequestDto {
    pub fn to_updated_position(&self, existing: Position) -> Result<Position, PositionApiError> {
        let position = Position {
            id: existing.id,
            user_id: existing.user_id,
            company: Company::new(&self.company),
            role_title: RoleTitle::new(&self.role_title),
            description: Description::new(&self.description),
            applied_on: AppliedOn::new(&self.applied_on)?,
            url: Url::new(&self.url),
            status: PositionStatus::from_str(&self.status)?,
            created_at: existing.created_at,
            updated_at: chrono::Local::now(),
            deleted_at: existing.deleted_at,
            deleted: existing.deleted,
        };
        Ok(position)
    }
}

#[derive(Deserialize, ToSchema)]
pub struct SaveCommentRequestDto {
    pub body: String,
}

impl SaveCommentRequestDto {
    pub fn to_new_comment(
        &self,
        user_id: UserUuid,
        position_id: PositionUuid,
    ) -> Result<Comment, CommentApiError> {
        let comment = Comment {
            id: CommentUuid::new(),
            position_id,
            user_id,
            body: CommentBody::new(&self.body),
            created_at: chrono::Local::now(),
            updated_at: chrono::Local::now(),
        };
        Ok(comment)
    }
}

#[derive(Deserialize, ToSchema)]
pub struct UpdateCommentRequestDto {
    pub body: String,
}

impl UpdateCommentRequestDto {
    pub fn to_updated_comment(&self, existing: Comment) -> Result<Comment, CommentApiError> {
        let comment = Comment {
            id: existing.id,
            position_id: existing.position_id,
            user_id: existing.user_id,
            body: CommentBody::new(&self.body),
            created_at: existing.created_at,
            updated_at: chrono::Local::now(),
        };
        Ok(comment)
    }
}
