use std::sync::Arc;

use async_trait::async_trait;
use tokio::sync::RwLock;

use crate::{auth::domain::{
    entities::user::User,
    repositories::user_repository::IUserRepository,
}, shared::domain::{error::UserValueError, value_objects::UserUuid}};

#[derive(Clone)]
pub struct UserInMemoryRepository {
    users: Arc<RwLock<Vec<User>>>,
}
impl Default for UserInMemoryRepository {
    fn default() -> Self {
        UserInMemoryRepository {
            users: Arc::new(RwLock::new(vec![])),
        }
    }
}

#[async_trait]
impl IUserRepository for UserInMemoryRepository {
    async fn get(&self, user_id: UserUuid) -> Option<User> {
        self.users
            .read()
            .await
            .iter()
            .find(|u| u.id == user_id)
            .cloned()
    }
    async fn save(&mut self, user: User) -> Result<UserUuid, UserValueError> {
        let user_id = user.id.clone();
        self.users.write().await.push(user);
        Ok(user_id)
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use crate::shared::fixtures::{TESTING_EMAIL, TESTING_PASSWORD, TESTING_UUID_1};

    use super::*;

    #[tokio::test]
    async fn test_user_save() -> Result<(), UserValueError> {
        let mut repo = UserInMemoryRepository::default();

        let user = User::new(TESTING_UUID_1, TESTING_EMAIL, TESTING_PASSWORD)?;
        let user_uuid = repo.save(user).await?;

        assert_eq!(user_uuid, UserUuid::from_str(TESTING_UUID_1)?);
        Ok(())
    }

    #[tokio::test]
    async fn test_get_user() -> Result<(), UserValueError> {
        let mut repo = UserInMemoryRepository::default();

        let expected_user = User::new(TESTING_UUID_1, TESTING_EMAIL, TESTING_PASSWORD)?;
        let user_uuid = repo.save(expected_user.clone()).await?;

        let current_user = repo
            .get(user_uuid)
            .await
            .expect("Result user was expected at this point!");

        assert_eq!(current_user, expected_user);
        Ok(())
    }
}
