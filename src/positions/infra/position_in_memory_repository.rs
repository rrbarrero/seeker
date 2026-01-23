use std::sync::{Arc, RwLock};

use crate::positions::domain::entities::{
    errors::PositionRepositoryError,
    interfaces::IPositionRepository,
    position::{Position, PositionUuid},
};

#[derive(Clone)]
pub struct PositionInMemoryRepository {
    positions: Arc<RwLock<Vec<Position>>>,
}

impl Default for PositionInMemoryRepository {
    fn default() -> Self {
        PositionInMemoryRepository {
            positions: Arc::new(RwLock::new(vec![])),
        }
    }
}

impl IPositionRepository for PositionInMemoryRepository {
    fn get(&self, position_id: PositionUuid) -> Option<Position> {
        self.positions
            .read()
            .unwrap()
            .iter()
            .find(|&p| p.id == position_id)
            .cloned()
    }
    fn get_all(&self) -> Vec<Position> {
        self.positions.read().unwrap().clone()
    }
    fn remove(&mut self, position_uuid: PositionUuid) {
        self.positions
            .write()
            .unwrap()
            .retain(|p| p.id != position_uuid);
    }
    fn save(&mut self, position: Position) -> Result<PositionUuid, PositionRepositoryError> {
        let uuid = position.id.clone();
        self.positions.write().unwrap().push(position);
        Ok(uuid)
    }
}

#[cfg(test)]
mod tests {
    use crate::utils::fixtures::{TESTING_UUID, create_fixture_position};
    use std::str::FromStr;

    use super::*;

    fn create_positions_repo_for_testing(position: Option<Position>) -> PositionInMemoryRepository {
        let mut repo = PositionInMemoryRepository::default();

        if let Some(p) = position {
            let _ = repo.save(p);
        }

        repo
    }

    #[test]
    fn test_get_position() {
        let repo = create_positions_repo_for_testing(Some(create_fixture_position()));

        let position_id = PositionUuid::from_str(TESTING_UUID).unwrap();
        let position = repo.get(position_id).unwrap();

        let expected_position = create_fixture_position();

        assert_eq!(position, expected_position);
    }

    #[test]
    fn test_save_position() {
        let mut repo = create_positions_repo_for_testing(None);

        let position_uuid = repo.save(create_fixture_position());

        assert_eq!(
            position_uuid.unwrap(),
            PositionUuid::from_str(TESTING_UUID).unwrap()
        );

        assert_eq!(repo.get_all().len(), 1);
    }
}
