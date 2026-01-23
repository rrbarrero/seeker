use crate::domain::entities::{
    errors::PositionRepositoryError,
    interfaces::IPositionRepository,
    position::{Position, PositionUuid},
};

pub struct PositionInMemoryRepository {
    positions: Vec<Position>,
}

impl IPositionRepository for PositionInMemoryRepository {
    fn get(&self, position_id: PositionUuid) -> Option<&Position> {
        self.positions.iter().find(|&p| p.id == position_id)
    }
    fn get_all(&self) -> &Vec<Position> {
        &self.positions
    }
    fn remove(&mut self, position_uuid: PositionUuid) {
        self.positions.retain(|p| p.id != position_uuid);
    }
    fn save(&mut self, position: Position) -> Result<PositionUuid, PositionRepositoryError> {
        let uuid = position.id.clone();
        self.positions.push(position);
        Ok(uuid)
    }
}

#[cfg(test)]
mod tests {
    use crate::utils::fixtures::{TESTING_UUID, create_fixture_position};
    use std::str::FromStr;

    use super::*;

    fn create_testing_repo() -> PositionInMemoryRepository {
        PositionInMemoryRepository {
            positions: vec![create_fixture_position()],
        }
    }

    #[test]
    fn test_get_position() {
        let repo = create_testing_repo();

        let position_id = PositionUuid::from_str(TESTING_UUID).unwrap();
        let position = repo.get(position_id).unwrap();

        let expected_position = create_fixture_position();

        assert_eq!(*position, expected_position);
    }
}
