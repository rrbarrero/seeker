use std::sync::Arc;

use async_trait::async_trait;
use tokio::sync::RwLock;

use crate::{
    auth::domain::{
        entities::user::{User, UserEmail},
        repositories::user_repository::IUserRepository,
    },
    shared::domain::{error::AuthRepositoryError, value_objects::UserUuid},
};

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
    async fn get(&self, user_id: UserUuid) -> Result<Option<User>, AuthRepositoryError> {
        Ok(self
            .users
            .read()
            .await
            .iter()
            .find(|u| u.id == user_id)
            .cloned())
    }
    async fn save(&self, user: &User) -> Result<UserUuid, AuthRepositoryError> {
        let user_id = user.id;
        if let Some(_) = self
            .users
            .read()
            .await
            .iter()
            .find(|u| u.id == user_id || u.email == user.email)
        {
            return Err(AuthRepositoryError::UserAlreadyExists);
        }
        self.users.write().await.push(user.clone());
        Ok(user_id)
    }
    async fn find_by_email(&self, email: UserEmail) -> Result<Option<User>, AuthRepositoryError> {
        Ok(self
            .users
            .read()
            .await
            .iter()
            .find(|u| u.email == email)
            .cloned())
    }
}

#[cfg(test)]
mod tests {
    use uuid::Uuid;

    use super::*;

    // Helpers
    fn valid_email() -> &'static str {
        "test@example.com"
    }
    fn valid_password() -> &'static str {
        "S0m3V3ryStr0ngP@ssw0rd!"
    }
    fn valid_id() -> String {
        Uuid::new_v4().to_string()
    }

    #[tokio::test]
    async fn test_user_save() -> Result<(), AuthRepositoryError> {
        let repo = UserInMemoryRepository::default();

        let id = valid_id();
        let user = User::new(&id, valid_email(), valid_password())?;
        let user_id_copy = user.id;

        let user_uuid = repo.save(&user).await?;

        assert_eq!(user_uuid, user_id_copy);
        Ok(())
    }

    #[tokio::test]
    async fn test_get_user() -> Result<(), AuthRepositoryError> {
        let repo = UserInMemoryRepository::default();

        let user = User::new(&valid_id(), valid_email(), valid_password())?;
        let user_uuid = repo.clone().save(&user).await?;

        let current_user = repo
            .get(user_uuid)
            .await
            .expect("Should not error on get")
            .expect("Result user was expected at this point!");

        assert_eq!(current_user, user);
        Ok(())
    }

    #[tokio::test]
    async fn test_repository_contract() -> Result<(), AuthRepositoryError> {
        let repo = UserInMemoryRepository::default();
        let user = User::new(&valid_id(), valid_email(), valid_password())?;

        crate::auth::infrastructure::persistence::repositories::common_repository_tests::assert_user_repository_behavior(
            Box::new(repo),
            user,
        )
        .await;
        Ok(())
    }

    #[tokio::test]
    async fn test_find_by_email_in_memory_repository() -> Result<(), AuthRepositoryError> {
        let repo = UserInMemoryRepository::default();

        let user = User::new(&valid_id(), valid_email(), valid_password())?;
        let _ = repo.clone().save(&user).await?;

        let current_user = repo
            .find_by_email(user.clone().email)
            .await
            .expect("Should not error on get")
            .expect("Result user was expected at this point!");

        assert_eq!(current_user, user);
        Ok(())
    }
}
