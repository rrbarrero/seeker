use async_trait::async_trait;

use crate::positions::domain::entities::comment::{Comment, CommentUuid};
use crate::positions::domain::entities::position::PositionUuid;
use crate::positions::domain::errors::CommentRepoError;

#[async_trait]
pub trait ICommentRepository: Send + Sync {
    async fn get(&self, comment_id: CommentUuid) -> Result<Option<Comment>, CommentRepoError>;
    async fn get_by_position(
        &self,
        position_id: PositionUuid,
    ) -> Result<Vec<Comment>, CommentRepoError>;
    async fn save(&self, comment: Comment) -> Result<CommentUuid, CommentRepoError>;
    async fn update(&self, comment: Comment) -> Result<(), CommentRepoError>;
    async fn remove(&self, comment_id: CommentUuid) -> Result<(), CommentRepoError>;
}
