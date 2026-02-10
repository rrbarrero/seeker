use async_trait::async_trait;
use chrono::{DateTime, Local, NaiveDateTime, TimeZone, Utc};
use sqlx::postgres::PgPool;
use tracing::{error, warn};
use uuid::Uuid;

use crate::positions::domain::{
    entities::comment::{Comment, CommentBuilder, CommentUuid},
    entities::position::PositionUuid,
    errors::{CommentDomainError, CommentRepoError},
    repositories::comment_repository::ICommentRepository,
};

struct CommentRow {
    id: Uuid,
    position_id: Uuid,
    user_id: Uuid,
    body: String,
    created_at: NaiveDateTime,
    updated_at: NaiveDateTime,
}

pub struct CommentPostgresRepository {
    pool: PgPool,
}

impl CommentPostgresRepository {
    pub async fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    fn from_row(row: CommentRow) -> Result<Comment, CommentDomainError> {
        Ok(CommentBuilder::new()
            .with_uuid(&row.id.to_string())?
            .with_position_uuid(&row.position_id.to_string())?
            .with_user_uuid(&row.user_id.to_string())?
            .with_body(&row.body)
            .with_created_at(DateTime::<Local>::from(
                Utc.from_utc_datetime(&row.created_at),
            ))
            .with_updated_at(DateTime::<Local>::from(
                Utc.from_utc_datetime(&row.updated_at),
            ))
            .build())
    }
}

#[async_trait]
impl ICommentRepository for CommentPostgresRepository {
    async fn get(&self, comment_id: CommentUuid) -> Result<Option<Comment>, CommentRepoError> {
        let result = sqlx::query_as!(
            CommentRow,
            "SELECT id, position_id, user_id, body, created_at, updated_at FROM comments WHERE id = $1",
            comment_id.value()
        )
        .fetch_optional(&self.pool)
        .await;

        match result {
            Ok(Some(row)) => match Self::from_row(row) {
                Ok(comment) => Ok(Some(comment)),
                Err(err) => {
                    error!(
                        comment_id = %comment_id.value(),
                        error_kind = "conversion_error",
                        "comment_repo.get failed"
                    );
                    Err(CommentRepoError::from(err))
                }
            },
            Ok(None) => Ok(None),
            Err(e) => {
                error!(
                    comment_id = %comment_id.value(),
                    error_kind = "database_error",
                    error = %e,
                    "comment_repo.get failed"
                );
                Err(CommentRepoError::DatabaseError(e.to_string()))
            }
        }
    }

    async fn get_by_position(
        &self,
        position_id: PositionUuid,
    ) -> Result<Vec<Comment>, CommentRepoError> {
        let result = sqlx::query_as!(
            CommentRow,
            "SELECT id, position_id, user_id, body, created_at, updated_at FROM comments WHERE position_id = $1 ORDER BY created_at ASC",
            position_id.value()
        )
        .fetch_all(&self.pool)
        .await;

        match result {
            Ok(rows) => rows
                .into_iter()
                .map(Self::from_row)
                .collect::<Result<Vec<_>, _>>()
                .map_err(|err| {
                    error!(
                        position_id = %position_id.value(),
                        error_kind = "conversion_error",
                        "comment_repo.get_by_position failed"
                    );
                    CommentRepoError::from(err)
                }),
            Err(e) => {
                error!(
                    position_id = %position_id.value(),
                    error_kind = "database_error",
                    error = %e,
                    "comment_repo.get_by_position failed"
                );
                Err(CommentRepoError::DatabaseError(e.to_string()))
            }
        }
    }

    async fn save(&self, comment: Comment) -> Result<CommentUuid, CommentRepoError> {
        let comment_id = comment.id;
        let user_id = comment.user_id;
        let position_id = comment.position_id;

        sqlx::query!(
            "INSERT INTO comments (id, position_id, user_id, body, created_at, updated_at) VALUES ($1, $2, $3, $4, $5, $6)",
            comment.id.value(),
            comment.position_id.value(),
            comment.user_id.value(),
            comment.body.value(),
            comment.created_at.naive_utc(),
            comment.updated_at.naive_utc(),
        )
        .execute(&self.pool)
        .await
        .map_err(|e| {
            error!(
                comment_id = %comment_id.value(),
                position_id = %position_id.value(),
                user_id = %user_id.value(),
                error_kind = "database_error",
                error = %e,
                "comment_repo.save failed"
            );
            CommentRepoError::DatabaseError(e.to_string())
        })?;

        Ok(comment.id)
    }

    async fn update(&self, comment: Comment) -> Result<(), CommentRepoError> {
        let result = sqlx::query!(
            "UPDATE comments SET body = $1, updated_at = $2 WHERE id = $3",
            comment.body.value(),
            comment.updated_at.naive_utc(),
            comment.id.value(),
        )
        .execute(&self.pool)
        .await;

        match result {
            Ok(result) => {
                if result.rows_affected() == 0 {
                    warn!(
                        comment_id = %comment.id.value(),
                        error_kind = "not_found",
                        "comment_repo.update failed"
                    );
                    Err(CommentRepoError::NotFound(comment.id))
                } else {
                    Ok(())
                }
            }
            Err(e) => {
                error!(
                    comment_id = %comment.id.value(),
                    error_kind = "database_error",
                    error = %e,
                    "comment_repo.update failed"
                );
                Err(CommentRepoError::DatabaseError(e.to_string()))
            }
        }
    }

    async fn remove(&self, comment_id: CommentUuid) -> Result<(), CommentRepoError> {
        let result = sqlx::query!("DELETE FROM comments WHERE id = $1", comment_id.value())
            .execute(&self.pool)
            .await;

        match result {
            Ok(_) => Ok(()),
            Err(e) => {
                error!(
                    comment_id = %comment_id.value(),
                    error_kind = "database_error",
                    error = %e,
                    "comment_repo.remove failed"
                );
                Err(CommentRepoError::DatabaseError(e.to_string()))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::positions::domain::entities::position::PositionUuid;
    use crate::positions::domain::repositories::position_repository::IPositionRepository;
    use crate::positions::infrastructure::persistence::repositories::position_postgres_repository::PositionPostgresRepository;
    use crate::shared::fixtures::{create_fixture_comment, create_fixture_position};
    use crate::shared::infrastructure::test_factory::TestFactory;

    async fn create_position_for_user(
        factory: &mut TestFactory,
        user_id: crate::shared::domain::value_objects::UserUuid,
    ) -> crate::positions::domain::entities::position::Position {
        let pool = factory.pool.clone();
        let repository = PositionPostgresRepository::new(pool).await;

        let mut position = create_fixture_position();
        position.id = PositionUuid::new();
        position.user_id = user_id;

        factory.track_position(position.id.value());
        repository
            .save(position.clone())
            .await
            .expect("Should save position");

        position
    }

    #[tokio::test]
    async fn test_save_comment_postgres_repository() {
        let mut factory = TestFactory::new().await;

        let user = factory.create_random_user().await;
        let position = create_position_for_user(&mut factory, user.id).await;

        let pool = factory.pool.clone();
        let repository = CommentPostgresRepository::new(pool).await;

        let mut comment = create_fixture_comment();
        comment.id = CommentUuid::new();
        comment.user_id = user.id;
        comment.position_id = position.id;

        factory.track_comment(comment.id.value());
        let result = repository.save(comment).await;

        result.expect("Should save comment");
    }

    #[tokio::test]
    async fn test_get_comment_postgres_repository() {
        let mut factory = TestFactory::new().await;
        let user = factory.create_random_user().await;
        let position = create_position_for_user(&mut factory, user.id).await;

        let pool = factory.pool.clone();
        let repository = CommentPostgresRepository::new(pool).await;

        let mut comment = create_fixture_comment();
        comment.id = CommentUuid::new();
        comment.user_id = user.id;
        comment.position_id = position.id;

        factory.track_comment(comment.id.value());
        let result = repository.save(comment).await;

        let comment_id = result.expect("Should save comment");

        let result = repository.get(comment_id).await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_get_comments_by_position_postgres_repository() {
        let mut factory = TestFactory::new().await;
        let user = factory.create_random_user().await;
        let position = create_position_for_user(&mut factory, user.id).await;

        let pool = factory.pool.clone();
        let repository = CommentPostgresRepository::new(pool).await;

        let mut comment1 = create_fixture_comment();
        comment1.id = CommentUuid::new();
        comment1.user_id = user.id;
        comment1.position_id = position.id;
        let position_id = comment1.position_id;

        let mut comment2 = create_fixture_comment();
        comment2.id = CommentUuid::new();
        comment2.user_id = user.id;
        comment2.position_id = comment1.position_id;

        factory.track_comment(comment1.id.value());
        factory.track_comment(comment2.id.value());

        repository
            .save(comment1)
            .await
            .expect("Should save comment");
        repository
            .save(comment2)
            .await
            .expect("Should save comment");

        let result = repository.get_by_position(position_id).await;

        assert!(result.is_ok());
        let comments = result.unwrap();
        assert_eq!(comments.len(), 2);
    }

    #[tokio::test]
    async fn test_update_comment_postgres_repository() {
        let mut factory = TestFactory::new().await;
        let user = factory.create_random_user().await;
        let position = create_position_for_user(&mut factory, user.id).await;

        let pool = factory.pool.clone();
        let repository = CommentPostgresRepository::new(pool).await;

        let mut comment = create_fixture_comment();
        comment.id = CommentUuid::new();
        comment.user_id = user.id;
        comment.position_id = position.id;

        factory.track_comment(comment.id.value());
        repository
            .save(comment.clone())
            .await
            .expect("Should save comment");

        comment.body = crate::positions::domain::entities::comment::CommentBody::new("Updated");
        comment.updated_at = chrono::Local::now();

        repository
            .update(comment.clone())
            .await
            .expect("Should update comment");

        let updated = repository
            .get(comment.id)
            .await
            .expect("Should get comment")
            .expect("Comment should exist");

        assert_eq!(updated.body.value(), "Updated");
    }

    #[tokio::test]
    async fn test_remove_comment_postgres_repository() {
        let mut factory = TestFactory::new().await;
        let user = factory.create_random_user().await;
        let position = create_position_for_user(&mut factory, user.id).await;

        let pool = factory.pool.clone();
        let repository = CommentPostgresRepository::new(pool).await;

        let mut comment = create_fixture_comment();
        comment.id = CommentUuid::new();
        comment.user_id = user.id;
        comment.position_id = position.id;

        factory.track_comment(comment.id.value());
        let result = repository.save(comment).await;

        let comment_id = result.expect("Should save comment");

        let _ = repository.remove(comment_id).await;

        let result = repository.get(comment_id).await;

        assert!(result.is_ok());
        assert!(result.unwrap().is_none());
    }

    #[tokio::test]
    async fn test_repository_contract() {
        let mut factory = TestFactory::new().await;
        let user = factory.create_random_user().await;
        let position = create_position_for_user(&mut factory, user.id).await;
        let pool = factory.pool.clone();
        let repository = CommentPostgresRepository::new(pool).await;

        let mut comment = create_fixture_comment();
        comment.user_id = user.id;
        comment.position_id = position.id;

        crate::positions::infrastructure::persistence::repositories::comment_repository_tests::assert_repository_behavior(
            Box::new(repository),
            comment,
        )
        .await;

        factory.teardown().await;
    }
}
