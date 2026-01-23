use crate::positions::domain::entities::{
    error::PositionRepositoryError,
    position::{Position, PositionUuid},
};

pub trait IPositionRepository {
    fn get_all(&self) -> Vec<Position>;
    fn get(&self, position_id: PositionUuid) -> Option<Position>;
    fn save(&mut self, position: Position) -> Result<PositionUuid, PositionRepositoryError>;
    fn remove(&mut self, position_uuid: PositionUuid);
}
