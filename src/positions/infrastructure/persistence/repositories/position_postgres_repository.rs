use std::str::FromStr;

use async_trait::async_trait;
use chrono::{DateTime, Local, NaiveDate, NaiveDateTime, TimeZone, Utc};
use sqlx::postgres::PgPool;
use uuid::Uuid;

use crate::positions::domain::{
    entities::{
        position::{Position, PositionBuilder, PositionStatus, PositionUuid},
        position_error::{PositionRepositoryError, PositionValueError},
    },
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
}

pub struct PositionPostgresRepository {
    pool: PgPool,
}

impl PositionPostgresRepository {
    pub async fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    fn from_row(row: PositionRow) -> Result<Position, PositionValueError> {
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
            .build())
    }
}

#[async_trait]
impl IPositionRepository for PositionPostgresRepository {
    async fn save(&mut self, position: Position) -> Result<PositionUuid, PositionRepositoryError> {
        sqlx::query!(
            "INSERT INTO positions (id, user_id, company, role_title, description, applied_on, url, initial_comment, status, created_at, updated_at, deleted_at) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12)",
            position.id.value(),
            position.user_id.value(),
            position.company.value(),
            position.role_title.value(),
            position.description.value(),
            chrono::NaiveDate::parse_from_str(&position.applied_on.value(), "%Y-%m-%d").unwrap(),
            position.url.value(),
            position.initial_comment.value(),
            format!("{:?}", position.status),
            position.created_at.naive_utc(),
            position.updated_at.naive_utc(),
            position.deleted_at.map(|d| d.naive_utc()),
        )
        .execute(&self.pool)
        .await?;

        Ok(position.id)
    }

    async fn get(&self, _position_id: PositionUuid) -> Option<Position> {
        let result = sqlx::query_as!(
            PositionRow,
            "SELECT * FROM positions WHERE id = $1",
            _position_id.value()
        )
        .fetch_optional(&self.pool)
        .await;

        match result {
            Ok(Some(row)) => Self::from_row(row).ok(),
            Ok(None) => None,
            Err(e) => {
                eprintln!("Database error in get: {:?}", e);
                None
            }
        }
    }

    async fn get_all(&self) -> Vec<Position> {
        let result = sqlx::query_as!(PositionRow, "SELECT * FROM positions")
            .fetch_all(&self.pool)
            .await;

        match result {
            Ok(rows) => rows
                .into_iter()
                .map(Self::from_row)
                .collect::<Result<Vec<_>, _>>()
                .unwrap(),
            Err(e) => {
                eprintln!("Database error in get_all: {:?}", e);
                vec![]
            }
        }
    }

    async fn remove(&mut self, _position_uuid: PositionUuid) {
        let _ = sqlx::query!(
            "DELETE FROM positions WHERE id = $1",
            _position_uuid.value()
        )
        .execute(&self.pool)
        .await;
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::positions::domain::entities::position::PositionUuid;
    use crate::shared::{fixtures::create_fixture_position, test_utils::TestFactory};

    #[tokio::test]
    async fn test_save_position_postgres_repository() {
        let mut factory = TestFactory::new().await;

        let user = factory.create_random_user().await;

        let pool = factory.pool.clone();
        let mut repository = PositionPostgresRepository::new(pool).await;

        let mut position = create_fixture_position();

        position.id = PositionUuid::new();
        position.user_id = user.id;

        factory.created_positions.push(position.id.value());
        let result = repository.save(position).await;

        result.expect("Should save position");

        factory.teardown().await;
    }

    #[tokio::test]
    async fn test_get_position_postgres_repository() {
        let mut factory = TestFactory::new().await;

        let user = factory.create_random_user().await;

        let pool = factory.pool.clone();
        let mut repository = PositionPostgresRepository::new(pool).await;

        let mut position = create_fixture_position();

        position.id = PositionUuid::new();
        position.user_id = user.id;

        factory.created_positions.push(position.id.value());
        let result = repository.save(position).await;

        let position_id = result.expect("Should save position");

        let result = repository.get(position_id).await;

        assert!(result.is_some());

        factory.teardown().await;
    }

    #[tokio::test]
    async fn test_get_all_position_postgres_repository() {
        let mut factory = TestFactory::new().await;

        let user = factory.create_random_user().await;

        let pool = factory.pool.clone();
        let mut repository = PositionPostgresRepository::new(pool).await;

        let mut position = create_fixture_position();

        position.id = PositionUuid::new();
        position.user_id = user.id;

        factory.created_positions.push(position.id.value());
        let result = repository.save(position).await;

        let position_id = result.expect("Should save position");

        let result = repository.get(position_id).await;

        assert!(result.is_some());

        factory.teardown().await;
    }

    #[tokio::test]
    async fn test_remove_position_postgres_repository() {
        let mut factory = TestFactory::new().await;

        let user = factory.create_random_user().await;

        let pool = factory.pool.clone();
        let mut repository = PositionPostgresRepository::new(pool).await;

        let mut position = create_fixture_position();

        position.id = PositionUuid::new();
        position.user_id = user.id;

        factory.created_positions.push(position.id.value());
        let result = repository.save(position).await;

        let position_id = result.expect("Should save position");

        let _ = repository.remove(position_id.clone()).await;

        let result = repository.get(position_id).await;

        assert!(result.is_none());

        factory.teardown().await;
    }
}
