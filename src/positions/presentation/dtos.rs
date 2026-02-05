use std::str::FromStr;

use serde::Deserialize;

use crate::{
    positions::{
        domain::entities::position::{
            AppliedOn, Company, Description, InitialComment, Position, PositionStatus,
            PositionUuid, RoleTitle, Url,
        },
        presentation::errors::PositionApiError,
    },
    shared::domain::value_objects::UserUuid,
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
    pub initial_comment: String,
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
            initial_comment: position.initial_comment.to_string(),
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

#[derive(Deserialize, ToSchema)]
pub struct SavePositionRequestDto {
    pub company: String,
    pub role_title: String,
    pub description: String,
    pub applied_on: String,
    pub url: String,
    pub initial_comment: String,
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
            initial_comment: InitialComment::new(&self.initial_comment),
            status: PositionStatus::from_str(&self.status)?,
            created_at: chrono::Local::now(),
            updated_at: chrono::Local::now(),
            deleted_at: None,
            deleted: false,
        };
        Ok(position)
    }
}
