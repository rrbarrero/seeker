use crate::positions::{
    application::errors::PositionServiceError,
    domain::entities::position::{Position, PositionUuid},
    domain::repositories::position_repository::IPositionRepository,
};

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

    pub async fn update(&self, position: Position) -> Result<(), PositionServiceError> {
        self.repo.update(position).await?;
        Ok(())
    }

    pub async fn remove(&self, position_uuid: PositionUuid) -> Result<(), PositionServiceError> {
        self.repo.remove(position_uuid).await?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        positions::infrastructure::persistence::repositories::position_in_memory_repository::PositionInMemoryRepository,
        shared::fixtures::create_fixture_position,
    };

    fn create_service() -> PositionService {
        let repo = Box::new(PositionInMemoryRepository::default());
        PositionService::new(repo)
    }

    #[tokio::test]
    async fn test_get_positions_empty() {
        let service = create_service();

        let result = service.get_positions().await;

        assert!(result.is_ok());
        assert_eq!(result.unwrap().len(), 0);
    }

    #[tokio::test]
    async fn test_save_position_success() {
        let service = create_service();
        let position = create_fixture_position();
        let expected_id = position.id;

        let result = service.save(position).await;

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), expected_id);
    }

    #[tokio::test]
    async fn test_get_positions_after_save() {
        let service = create_service();
        let position = create_fixture_position();
        service.save(position).await.unwrap();

        let result = service.get_positions().await;

        assert!(result.is_ok());
        assert_eq!(result.unwrap().len(), 1);
    }

    #[tokio::test]
    async fn test_get_position_by_id() {
        let service = create_service();
        let position = create_fixture_position();
        let position_id = position.id;
        service.save(position.clone()).await.unwrap();

        let result = service.get_position(position_id).await;

        assert!(result.is_ok());
        let found = result.unwrap();
        assert!(found.is_some());
        assert_eq!(found.unwrap().id, position_id);
    }

    #[tokio::test]
    async fn test_get_position_not_found() {
        let service = create_service();
        let random_id = PositionUuid::new();

        let result = service.get_position(random_id).await;

        assert!(result.is_ok());
        assert!(result.unwrap().is_none());
    }

    #[tokio::test]
    async fn test_update_position_success() {
        let service = create_service();
        let mut position = create_fixture_position();
        let position_id = position.id;
        service.save(position.clone()).await.unwrap();

        position.company =
            crate::positions::domain::entities::position::Company::new("Updated Company");
        let result = service.update(position).await;

        assert!(result.is_ok());

        let updated = service.get_position(position_id).await.unwrap().unwrap();
        assert_eq!(updated.company.value(), "Updated Company");
    }

    #[tokio::test]
    async fn test_update_position_not_found() {
        let service = create_service();
        let position = create_fixture_position();

        let result = service.update(position).await;

        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_remove_position_success() {
        let service = create_service();
        let position = create_fixture_position();
        let position_id = position.id;
        service.save(position).await.unwrap();

        let result = service.remove(position_id).await;

        assert!(result.is_ok());

        let found = service.get_position(position_id).await.unwrap().unwrap();
        assert!(found.is_deleted());
    }

    #[tokio::test]
    async fn test_save_multiple_positions() {
        let service = create_service();
        let position1 = create_fixture_position();
        let position2 = create_fixture_position();

        service.save(position1).await.unwrap();
        service.save(position2).await.unwrap();

        let result = service.get_positions().await.unwrap();
        assert_eq!(result.len(), 2);
    }
}
