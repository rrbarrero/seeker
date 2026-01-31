use crate::positions::domain::entities::position::{Position, PositionUuid};
use crate::positions::domain::entities::position_error::PositionServiceError;
use crate::positions::domain::repositories::position_repository::IPositionRepository;

pub struct PositionService {
    repo: Box<dyn IPositionRepository>,
}

impl PositionService {
    pub fn new(repo: Box<dyn IPositionRepository>) -> Self {
        Self { repo }
    }

    pub async fn get_positions(&self) -> Result<Vec<Position>, PositionServiceError> {
        let positions = self.repo.get_all().await?;
        Ok(positions)
    }

    pub async fn get_position(
        &self,
        position_id: PositionUuid,
    ) -> Result<Option<Position>, PositionServiceError> {
        let position = self.repo.get(position_id).await?;
        Ok(position)
    }

    pub async fn save(&self, position: Position) -> Result<PositionUuid, PositionServiceError> {
        let position_uuid = self.repo.save(position).await?;
        Ok(position_uuid)
    }

    pub async fn remove(&self, position_uuid: PositionUuid) -> Result<(), PositionServiceError> {
        self.repo.remove(position_uuid).await?;
        Ok(())
    }
}
