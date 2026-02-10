use crate::positions::{
    application::errors::CommentServiceError,
    domain::entities::comment::{Comment, CommentUuid},
    domain::entities::position::PositionUuid,
    domain::repositories::comment_repository::ICommentRepository,
};

pub struct CommentService {
    repo: Box<dyn ICommentRepository>,
}

impl CommentService {
    pub fn new(repo: Box<dyn ICommentRepository>) -> Self {
        Self { repo }
    }

    pub async fn get_comment(
        &self,
        comment_id: CommentUuid,
    ) -> Result<Option<Comment>, CommentServiceError> {
        let comment = self.repo.get(comment_id).await?;
        Ok(comment)
    }

    pub async fn get_comments_for_position(
        &self,
        position_id: PositionUuid,
    ) -> Result<Vec<Comment>, CommentServiceError> {
        let comments = self.repo.get_by_position(position_id).await?;
        Ok(comments)
    }

    pub async fn save(&self, comment: Comment) -> Result<CommentUuid, CommentServiceError> {
        let comment_id = self.repo.save(comment).await?;
        Ok(comment_id)
    }

    pub async fn update(&self, comment: Comment) -> Result<(), CommentServiceError> {
        self.repo.update(comment).await?;
        Ok(())
    }

    pub async fn remove(&self, comment_id: CommentUuid) -> Result<(), CommentServiceError> {
        self.repo.remove(comment_id).await?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        positions::infrastructure::persistence::repositories::comment_in_memory_repository::CommentInMemoryRepository,
        shared::fixtures::create_fixture_comment,
    };

    fn create_service() -> CommentService {
        let repo = Box::new(CommentInMemoryRepository::default());
        CommentService::new(repo)
    }

    #[tokio::test]
    async fn test_get_comments_empty() {
        let service = create_service();
        let position_id = PositionUuid::new();

        let result = service.get_comments_for_position(position_id).await;

        assert!(result.is_ok());
        assert_eq!(result.unwrap().len(), 0);
    }

    #[tokio::test]
    async fn test_save_comment_success() {
        let service = create_service();
        let comment = create_fixture_comment();
        let expected_id = comment.id;

        let result = service.save(comment).await;

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), expected_id);
    }

    #[tokio::test]
    async fn test_get_comments_after_save() {
        let service = create_service();
        let comment = create_fixture_comment();
        let position_id = comment.position_id;

        service.save(comment).await.unwrap();

        let result = service.get_comments_for_position(position_id).await;

        assert!(result.is_ok());
        assert_eq!(result.unwrap().len(), 1);
    }

    #[tokio::test]
    async fn test_get_comment_by_id() {
        let service = create_service();
        let comment = create_fixture_comment();
        let comment_id = comment.id;

        service.save(comment).await.unwrap();

        let result = service.get_comment(comment_id).await;

        assert!(result.is_ok());
        let found = result.unwrap();
        assert!(found.is_some());
        assert_eq!(found.unwrap().id, comment_id);
    }

    #[tokio::test]
    async fn test_get_comment_not_found() {
        let service = create_service();
        let random_id = CommentUuid::new();

        let result = service.get_comment(random_id).await;

        assert!(result.is_ok());
        assert!(result.unwrap().is_none());
    }

    #[tokio::test]
    async fn test_update_comment_success() {
        let service = create_service();
        let mut comment = create_fixture_comment();
        let comment_id = comment.id;

        service.save(comment.clone()).await.unwrap();

        comment.body = crate::positions::domain::entities::comment::CommentBody::new("Updated");
        let result = service.update(comment).await;

        assert!(result.is_ok());

        let updated = service.get_comment(comment_id).await.unwrap().unwrap();
        assert_eq!(updated.body.value(), "Updated");
    }

    #[tokio::test]
    async fn test_update_comment_not_found() {
        let service = create_service();
        let comment = create_fixture_comment();

        let result = service.update(comment).await;

        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_remove_comment_success() {
        let service = create_service();
        let comment = create_fixture_comment();
        let comment_id = comment.id;

        service.save(comment).await.unwrap();

        let result = service.remove(comment_id).await;

        assert!(result.is_ok());

        let found = service.get_comment(comment_id).await.unwrap();
        assert!(found.is_none());
    }

    #[tokio::test]
    async fn test_save_multiple_comments() {
        let service = create_service();
        let comment1 = create_fixture_comment();
        let mut comment2 = create_fixture_comment();
        comment2.position_id = comment1.position_id;
        let position_id = comment2.position_id;

        service.save(comment1).await.unwrap();
        service.save(comment2).await.unwrap();

        let result = service
            .get_comments_for_position(position_id)
            .await
            .unwrap();
        assert_eq!(result.len(), 2);
    }
}
