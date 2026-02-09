use std::str::FromStr;

use async_trait::async_trait;
use chrono::{DateTime, Local, NaiveDate, NaiveDateTime, TimeZone, Utc};
use sqlx::postgres::PgPool;
use uuid::Uuid;

use crate::positions::domain::{
    entities::position::{Position, PositionBuilder, PositionStatus, PositionUuid},
    errors::{PositionDomainError, PositionRepoError},
    repositories::position_repository::IPositionRepository,
};

struct PositionRow {
    id: Uuid,
    user_id: Uuid,
    company: String,
    role_title: String,
    description: String,
    applied_on: NaiveDate,
    url: String,
    initial_comment: String,
    status: String,
    created_at: NaiveDateTime,
    updated_at: NaiveDateTime,
    deleted_at: Option<NaiveDateTime>,
    deleted: bool,
}

pub struct PositionPostgresRepository {
    pool: PgPool,
}

impl PositionPostgresRepository {
    pub async fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    fn from_row(row: PositionRow) -> Result<Position, PositionDomainError> {
        Ok(PositionBuilder::new()
            .with_uuid(&row.id.to_string())?
            .with_user_uuid(&row.user_id.to_string())?
            .with_company(&row.company)
            .with_role_title(&row.role_title)
            .with_description(&row.description)
            .with_applied_on_date(row.applied_on)
            .with_url(&row.url)
            .with_initial_comment(&row.initial_comment)
            .with_status(PositionStatus::from_str(&row.status)?)
            .with_created_at(DateTime::<Local>::from(
                Utc.from_utc_datetime(&row.created_at),
            ))
            .with_updated_at(DateTime::<Local>::from(
                Utc.from_utc_datetime(&row.updated_at),
            ))
            .with_optional_deleted_at(
                row.deleted_at
                    .map(|d| DateTime::<Local>::from(Utc.from_utc_datetime(&d))),
            )
            .with_deleted(row.deleted)
            .build())
    }
}

#[async_trait]
impl IPositionRepository for PositionPostgresRepository {
    async fn save(&self, position: Position) -> Result<PositionUuid, PositionRepoError> {
        sqlx::query!(
            "INSERT INTO positions (id, user_id, company, role_title, description, applied_on, url, initial_comment, status, created_at, updated_at, deleted_at, deleted) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13)",
            position.id.value(),
            position.user_id.value(),
            position.company.value(),
            position.role_title.value(),
            position.description.value(),
            position.applied_on.date(),
            position.url.value(),
            position.initial_comment.value(),
            format!("{:?}", position.status),
            position.created_at.naive_utc(),
            position.updated_at.naive_utc(),
            position.deleted_at.map(|d| d.naive_utc()),
            position.deleted,
        )
        .execute(&self.pool)
        .await
        .map_err(|e| PositionRepoError::DatabaseError(e.to_string()))?;

        Ok(position.id)
    }

    async fn update(&self, position: Position) -> Result<(), PositionRepoError> {
        let result = sqlx::query!(
            "UPDATE positions SET company = $1, role_title = $2, description = $3, applied_on = $4, url = $5, initial_comment = $6, status = $7, updated_at = $8 WHERE id = $9",
            position.company.value(),
            position.role_title.value(),
            position.description.value(),
            position.applied_on.date(),
            position.url.value(),
            position.initial_comment.value(),
            format!("{:?}", position.status),
            position.updated_at.naive_utc(),
            position.id.value(),
        )
        .execute(&self.pool)
        .await;

        match result {
            Ok(result) => {
                if result.rows_affected() == 0 {
                    Err(PositionRepoError::NotFound(position.id))
                } else {
                    Ok(())
                }
            }
            Err(e) => Err(PositionRepoError::DatabaseError(e.to_string())),
        }
    }

    async fn get(&self, _position_id: PositionUuid) -> Result<Option<Position>, PositionRepoError> {
        let result = sqlx::query_as!(
            PositionRow,
            "SELECT * FROM positions WHERE id = $1",
            _position_id.value()
        )
        .fetch_optional(&self.pool)
        .await;

        match result {
            Ok(Some(row)) => Self::from_row(row).map(Some).map_err(Into::into),
            Ok(None) => Ok(None),
            Err(e) => Err(PositionRepoError::DatabaseError(e.to_string())),
        }
    }

    async fn get_all(&self) -> Result<Vec<Position>, PositionRepoError> {
        let result = sqlx::query_as!(PositionRow, "SELECT * FROM positions")
            .fetch_all(&self.pool)
            .await;

        match result {
            Ok(rows) => rows
                .into_iter()
                .map(Self::from_row)
                .collect::<Result<Vec<_>, _>>()
                .map_err(Into::into),
            Err(e) => Err(PositionRepoError::DatabaseError(e.to_string())),
        }
    }

    async fn remove(&self, _position_uuid: PositionUuid) -> Result<(), PositionRepoError> {
        let result = sqlx::query!(
            "UPDATE positions SET deleted = true, deleted_at = NOW() WHERE id = $1",
            _position_uuid.value()
        )
        .execute(&self.pool)
        .await;

        match result {
            Ok(_) => Ok(()),
            Err(e) => Err(PositionRepoError::DatabaseError(e.to_string())),
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::positions::domain::entities::position::PositionUuid;
    use crate::shared::fixtures::create_fixture_position;
    use crate::shared::infrastructure::test_factory::TestFactory;

    #[tokio::test]
    async fn test_save_position_postgres_repository() {
        let mut factory = TestFactory::new().await;

        let user = factory.create_random_user().await;

        let pool = factory.pool.clone();
        let repository = PositionPostgresRepository::new(pool).await;

        let mut position = create_fixture_position();

        position.id = PositionUuid::new();
        position.user_id = user.id;

        factory.track_position(position.id.value());
        let result = repository.save(position).await;

        result.expect("Should save position");
    }

    #[tokio::test]
    async fn test_get_position_postgres_repository() {
        let mut factory = TestFactory::new().await;

        let user = factory.create_random_user().await;

        let pool = factory.pool.clone();
        let repository = PositionPostgresRepository::new(pool).await;

        let mut position = create_fixture_position();

        position.id = PositionUuid::new();
        position.user_id = user.id;

        factory.track_position(position.id.value());
        let result = repository.save(position).await;

        let position_id = result.expect("Should save position");

        let result = repository.get(position_id).await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_get_all_position_postgres_repository() {
        let mut factory = TestFactory::new().await;

        let user = factory.create_random_user().await;

        let pool = factory.pool.clone();
        let repository = PositionPostgresRepository::new(pool).await;

        let mut position = create_fixture_position();
        position.id = PositionUuid::new();
        position.user_id = user.id;

        repository
            .save(position)
            .await
            .expect("Should save position");

        let result = repository.get_all().await;

        assert!(result.is_ok());
        let positions = result.unwrap();
        assert!(!positions.is_empty());
        assert!(positions.iter().any(|p| p.user_id == user.id));
    }

    #[tokio::test]
    async fn test_update_position_postgres_repository() {
        let mut factory = TestFactory::new().await;

        let user = factory.create_random_user().await;

        let pool = factory.pool.clone();
        let repository = PositionPostgresRepository::new(pool).await;

        let mut position = create_fixture_position();
        position.id = PositionUuid::new();
        position.user_id = user.id;

        factory.track_position(position.id.value());
        repository
            .save(position.clone())
            .await
            .expect("Should save position");

        position.company = crate::positions::domain::entities::position::Company::new("Updated Co");
        position.updated_at = chrono::Local::now();

        repository
            .update(position.clone())
            .await
            .expect("Should update position");

        let updated = repository
            .get(position.id)
            .await
            .expect("Should get position")
            .expect("Position should exist");

        assert_eq!(updated.company.value(), "Updated Co");
    }

    #[tokio::test]
    async fn test_remove_position_postgres_repository() {
        let mut factory = TestFactory::new().await;

        let user = factory.create_random_user().await;

        let pool = factory.pool.clone();
        let repository = PositionPostgresRepository::new(pool).await;

        let mut position = create_fixture_position();

        position.id = PositionUuid::new();
        position.user_id = user.id;

        factory.track_position(position.id.value());
        let result = repository.save(position).await;

        let position_id = result.expect("Should save position");

        let _ = repository.remove(position_id).await;

        let result = repository.get(position_id).await;

        assert!(result.is_ok());
        assert!(
            result
                .expect("The Result should be Ok")
                .expect("The Position should not be None")
                .is_deleted()
        );
    }

    #[tokio::test]
    async fn test_repository_contract() {
        let mut factory = TestFactory::new().await;
        let user = factory.create_random_user().await;
        let pool = factory.pool.clone();
        let repository = PositionPostgresRepository::new(pool).await;

        let mut position = create_fixture_position();
        position.user_id = user.id;

        crate::positions::infrastructure::persistence::repositories::common_repository_tests::assert_repository_behavior(
            Box::new(repository),
            position,
        )
        .await;

        factory.teardown().await;
    }
}
