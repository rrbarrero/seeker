use async_trait::async_trait;
use sqlx::postgres::PgPool;

use crate::positions::domain::{
    entities::{
        position::{Position, PositionUuid},
        position_error::{PositionRepositoryError, PositionValueError},
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
    async fn save(&mut self, _position: Position) -> Result<PositionUuid, PositionRepositoryError> {
        todo!()
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
