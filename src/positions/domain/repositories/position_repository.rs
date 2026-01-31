use async_trait::async_trait;

use crate::positions::domain::entities::{
    position::{Position, PositionUuid},
    position_error::PositionRepositoryError,
};

#[async_trait]
pub trait IPositionRepository: Send + Sync {
    async fn get_all(&self) -> Result<Vec<Position>, PositionRepositoryError>;
    async fn get(
        &self,
        position_id: PositionUuid,
    ) -> Result<Option<Position>, PositionRepositoryError>;
    async fn save(&self, position: Position) -> Result<PositionUuid, PositionRepositoryError>;
    async fn remove(&self, position_uuid: PositionUuid) -> Result<(), PositionRepositoryError>;
}
