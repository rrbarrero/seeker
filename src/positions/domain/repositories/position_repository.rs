use async_trait::async_trait;

use crate::positions::domain::entities::position::{Position, PositionUuid};
use crate::positions::domain::errors::PositionRepoError;

#[async_trait]
pub trait IPositionRepository: Send + Sync {
    async fn get_all(&self) -> Result<Vec<Position>, PositionRepoError>;
    async fn get(&self, position_id: PositionUuid) -> Result<Option<Position>, PositionRepoError>;
    async fn save(&self, position: Position) -> Result<PositionUuid, PositionRepoError>;
    async fn remove(&self, position_uuid: PositionUuid) -> Result<(), PositionRepoError>;
}
