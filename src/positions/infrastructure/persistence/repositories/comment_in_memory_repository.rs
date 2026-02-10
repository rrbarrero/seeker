use std::sync::Arc;

use async_trait::async_trait;
use tokio::sync::RwLock;

use crate::positions::domain::{
    entities::comment::{Comment, CommentUuid},
    entities::position::PositionUuid,
    errors::CommentRepoError,
    repositories::comment_repository::ICommentRepository,
};

#[derive(Clone)]
pub struct CommentInMemoryRepository {
    comments: Arc<RwLock<Vec<Comment>>>,
}

impl Default for CommentInMemoryRepository {
    fn default() -> Self {
        CommentInMemoryRepository {
            comments: Arc::new(RwLock::new(vec![])),
        }
    }
}

#[async_trait]
impl ICommentRepository for CommentInMemoryRepository {
    async fn get(&self, comment_id: CommentUuid) -> Result<Option<Comment>, CommentRepoError> {
        Ok(self
            .comments
            .read()
            .await
            .iter()
            .find(|&c| c.id == comment_id)
            .cloned())
    }

    async fn get_by_position(
        &self,
        position_id: PositionUuid,
    ) -> Result<Vec<Comment>, CommentRepoError> {
        Ok(self
            .comments
            .read()
            .await
            .iter()
            .filter(|c| c.position_id == position_id)
            .cloned()
            .collect())
    }

    async fn save(&self, comment: Comment) -> Result<CommentUuid, CommentRepoError> {
        let uuid = comment.id;
        self.comments.write().await.push(comment);
        Ok(uuid)
    }

    async fn update(&self, comment: Comment) -> Result<(), CommentRepoError> {
        let mut comments = self.comments.write().await;
        if let Some(existing) = comments.iter_mut().find(|c| c.id == comment.id) {
            *existing = comment;
            Ok(())
        } else {
            Err(CommentRepoError::NotFound(comment.id))
        }
    }

    async fn remove(&self, comment_id: CommentUuid) -> Result<(), CommentRepoError> {
        let mut comments = self.comments.write().await;
        comments.retain(|c| c.id != comment_id);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::shared::fixtures::create_fixture_comment;

    use super::*;

    async fn create_comments_repo_for_testing(
        comment: Option<Comment>,
    ) -> CommentInMemoryRepository {
        let repo = CommentInMemoryRepository::default();

        if let Some(c) = comment {
            let _ = repo.save(c).await;
        }

        repo
    }

    #[tokio::test]
    async fn test_get_comment() {
        let expected_comment = create_fixture_comment();
        let repo = create_comments_repo_for_testing(Some(expected_comment.clone())).await;

        let comment = repo
            .get(expected_comment.id)
            .await
            .expect("Should get comment");

        assert_eq!(comment, Some(expected_comment));
    }

    #[tokio::test]
    async fn test_save_comment() {
        let repo = create_comments_repo_for_testing(None).await;
        let comment = create_fixture_comment();
        let expected_id = comment.id;
        let position_id = comment.position_id;

        let comment_uuid = repo.save(comment).await;

        assert_eq!(comment_uuid.expect("Error saving comment"), expected_id);

        assert_eq!(
            repo.get_by_position(position_id)
                .await
                .expect("Error getting comments")
                .len(),
            1
        );
    }

    #[tokio::test]
    async fn test_save_with_concurrency() {
        let repo = CommentInMemoryRepository::default();
        let num_tasks = 10;
        let mut handles = vec![];

        for i in 0..num_tasks {
            let repo_clone = repo.clone();

            let handle = tokio::spawn(async move {
                let mut comment = create_fixture_comment();
                comment.body = crate::positions::domain::entities::comment::CommentBody::new(
                    &format!("Body {}", i),
                );
                repo_clone
                    .save(comment)
                    .await
                    .expect("Error saving comment");
            });
            handles.push(handle);
        }

        for handle in handles {
            handle.await.expect("Error joining handle");
        }

        let all = repo.comments.read().await;
        assert_eq!(all.len(), num_tasks);
    }

    #[tokio::test]
    async fn test_update_comment() {
        let repo = create_comments_repo_for_testing(None).await;
        let mut comment = create_fixture_comment();
        let id = comment.id;

        let _ = repo.save(comment.clone()).await;
        comment.body = crate::positions::domain::entities::comment::CommentBody::new("Updated");

        repo.update(comment).await.expect("Should update comment");

        let updated = repo
            .get(id)
            .await
            .expect("Should get comment")
            .expect("Comment should exist");

        assert_eq!(updated.body.value(), "Updated");
    }

    #[tokio::test]
    async fn test_remove_comment() {
        let comment = create_fixture_comment();
        let repo = create_comments_repo_for_testing(Some(comment.clone())).await;

        let _ = repo.remove(comment.id).await;

        let found = repo.get(comment.id).await.expect("Should be Ok");
        assert!(found.is_none());
    }

    #[tokio::test]
    async fn test_remove_comment_not_found() {
        let repo = create_comments_repo_for_testing(Some(create_fixture_comment())).await;

        let comment_id = CommentUuid::new();
        let _ = repo.remove(comment_id).await;

        let remaining = repo.comments.read().await;
        assert_eq!(remaining.len(), 1);
    }

    #[tokio::test]
    async fn test_get_comments_by_position() {
        let comment1 = create_fixture_comment();
        let mut comment2 = create_fixture_comment();
        comment2.position_id = comment1.position_id;
        let repo = create_comments_repo_for_testing(Some(comment1.clone())).await;
        let _ = repo.save(comment2.clone()).await;

        let result = repo
            .get_by_position(comment1.position_id)
            .await
            .expect("Should get comments by position");

        assert_eq!(result.len(), 2);
    }

    #[tokio::test]
    async fn test_repository_contract() {
        let repo = CommentInMemoryRepository::default();
        let comment = create_fixture_comment();

        crate::positions::infrastructure::persistence::repositories::comment_repository_tests::assert_repository_behavior(
            Box::new(repo),
            comment,
        )
        .await;
    }
}
