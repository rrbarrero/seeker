use std::sync::Arc;

use async_trait::async_trait;
use tokio::sync::RwLock;

use crate::positions::domain::entities::{
    position::{Position, PositionUuid},
    position_error::PositionRepositoryError,
};
use crate::positions::domain::repositories::position_repository::IPositionRepository;

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

#[async_trait]
impl IPositionRepository for PositionInMemoryRepository {
    async fn get(
        &self,
        position_id: PositionUuid,
    ) -> Result<Option<Position>, PositionRepositoryError> {
        Ok(self
            .positions
            .read()
            .await
            .iter()
            .find(|&p| p.id == position_id)
            .cloned())
    }

    async fn get_all(&self) -> Result<Vec<Position>, PositionRepositoryError> {
        Ok(self.positions.read().await.clone())
    }

    async fn remove(&mut self, position_uuid: PositionUuid) -> Result<(), PositionRepositoryError> {
        if let Some(position) = self
            .positions
            .write()
            .await
            .iter_mut()
            .find(|p| p.id == position_uuid)
        {
            position.deleted = true;
            position.deleted_at = Some(chrono::Local::now());
        }
        Ok(())
    }

    async fn save(&mut self, position: Position) -> Result<PositionUuid, PositionRepositoryError> {
        let uuid = position.id;
        self.positions.write().await.push(position);
        Ok(uuid)
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        positions::domain::entities::position::PositionBuilder,
        shared::fixtures::create_fixture_position,
    };

    use super::*;

    async fn create_positions_repo_for_testing(
        position: Option<Position>,
    ) -> PositionInMemoryRepository {
        let mut repo = PositionInMemoryRepository::default();

        if let Some(p) = position {
            let _ = repo.save(p).await;
        }

        repo
    }

    #[tokio::test]
    async fn test_get_position() {
        let expected_position = create_fixture_position();
        let repo = create_positions_repo_for_testing(Some(expected_position.clone())).await;

        let position = repo
            .get(expected_position.id)
            .await
            .expect("Should get position");

        assert_eq!(position, Some(expected_position));
    }

    #[tokio::test]
    async fn test_save_position() {
        let mut repo = create_positions_repo_for_testing(None).await;
        let position = create_fixture_position();
        let expected_id = position.id;

        let position_uuid = repo.save(position).await;

        assert_eq!(position_uuid.expect("Error saving position"), expected_id);

        assert_eq!(
            repo.get_all()
                .await
                .expect("Error getting all positions")
                .len(),
            1
        );
    }

    #[tokio::test]
    async fn test_save_with_concurrency() {
        let repo = PositionInMemoryRepository::default();
        let num_tasks = 10;
        let mut handles = vec![];

        for i in 0..num_tasks {
            let mut repo_clone = repo.clone();

            let handle = tokio::spawn(async move {
                let pos = PositionBuilder::new()
                    .with_role_title(&format!("Role {}", i))
                    .build();
                repo_clone.save(pos).await.expect("Error saving position");
            });
            handles.push(handle);
        }

        for handle in handles {
            handle.await.expect("Error joining handle");
        }

        assert_eq!(
            repo.get_all()
                .await
                .expect("Error getting all positions")
                .len(),
            num_tasks
        );
    }

    #[tokio::test]
    async fn test_remove_position() {
        let position = create_fixture_position();
        let mut repo = create_positions_repo_for_testing(Some(position.clone())).await;

        let _ = repo.remove(position.id).await;

        assert_eq!(
            repo.get_all()
                .await
                .expect("Error getting all positions")
                .len(),
            1
        );

        assert!(
            repo.get(position.id)
                .await
                .expect("Should be Ok")
                .expect("Should be a position")
                .is_deleted()
        );
    }

    #[tokio::test]
    async fn test_remove_position_not_found() {
        let mut repo = create_positions_repo_for_testing(Some(create_fixture_position())).await;

        let position_id = PositionUuid::new();
        let _ = repo.remove(position_id).await;

        assert_eq!(
            repo.get_all()
                .await
                .expect("Error getting all positions")
                .len(),
            1
        );
    }

    #[tokio::test]
    async fn test_get_all_positions() {
        let repo = create_positions_repo_for_testing(Some(create_fixture_position())).await;

        assert_eq!(
            repo.get_all()
                .await
                .expect("Error getting all positions")
                .len(),
            1
        );
    }

    #[tokio::test]
    async fn test_get_all_positions_empty() {
        let repo = create_positions_repo_for_testing(None).await;

        assert_eq!(
            repo.get_all()
                .await
                .expect("Error getting all positions")
                .len(),
            0
        );
    }

    #[tokio::test]
    async fn test_get_position_not_found() {
        let repo = create_positions_repo_for_testing(Some(create_fixture_position())).await;

        let position_id = PositionUuid::new();
        let position = repo.get(position_id).await.unwrap();

        assert_eq!(position, None);
    }

    #[tokio::test]
    async fn test_repository_contract() {
        let repo = PositionInMemoryRepository::default();
        let position = create_fixture_position();

        crate::positions::infrastructure::persistence::repositories::common_repository_tests::assert_repository_behavior(
            Box::new(repo),
            position,
        )
        .await;
    }
}
