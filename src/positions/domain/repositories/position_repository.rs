use async_trait::async_trait;

use crate::positions::domain::entities::{
    position::{Position, PositionUuid},
    position_error::PositionRepositoryError,
};

#[async_trait]
pub trait IPositionRepository {
    async fn get_all(&self) -> Vec<Position>;
    async fn get(&self, position_id: PositionUuid) -> Option<Position>;
    async fn save(&mut self, position: Position) -> Result<PositionUuid, PositionRepositoryError>;
    async fn remove(&mut self, position_uuid: PositionUuid);
}
