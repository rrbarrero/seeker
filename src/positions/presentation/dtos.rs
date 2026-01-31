use std::str::FromStr;

use crate::positions::domain::entities::position::{Position, PositionUuid};

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct PositionDto {
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

impl From<&Position> for PositionDto {
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

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct PositionUuidDto {
    id: String,
}

impl Into<PositionUuid> for PositionUuidDto {
    fn into(self) -> PositionUuid {
        PositionUuid::from_str(&self.id).unwrap()
    }
}
