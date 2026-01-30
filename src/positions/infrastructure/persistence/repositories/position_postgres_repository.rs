use async_trait::async_trait;
use sqlx::postgres::PgPool;

use crate::positions::domain::{
    entities::{
        position::{Position, PositionUuid},
        position_error::PositionRepositoryError,
    },
    repositories::position_repository::IPositionRepository,
};

pub struct PositionPostgresRepository {
    pool: PgPool,
}

impl PositionPostgresRepository {
    pub async fn new(pool: PgPool) -> Self {
        Self { pool }
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
            chrono::NaiveDate::parse_from_str(&position.applied_on.value(), "%Y-%m-%d").unwrap(), // Assuming value() returns string YYYY-MM-DD
            position.url.value(),
            position.initial_comment.value(),
            format!("{:?}", position.status), // Storing enum as string
            position.created_at.naive_utc(),
            position.updated_at.naive_utc(),
            position.deleted_at.map(|d| d.naive_utc()),
        )
        .execute(&self.pool)
        .await?;

        Ok(position.id)
    }

    async fn get(&self, _position_id: PositionUuid) -> Option<Position> {
        todo!()
    }

    async fn get_all(&self) -> Vec<Position> {
        todo!()
    }

    async fn remove(&mut self, _position_uuid: PositionUuid) {
        todo!()
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

        let result = repository.save(position).await;

        assert!(result.is_ok());

        factory.teardown().await;
    }
}
