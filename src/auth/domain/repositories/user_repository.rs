use async_trait::async_trait;

use crate::auth::domain::entities::user::{User, UserEmail};
use crate::auth::domain::errors::AuthRepoError;
use crate::shared::domain::value_objects::UserUuid;

#[async_trait]
pub trait IUserRepository: Send + Sync {
    async fn get(&self, user_id: UserUuid) -> Result<Option<User>, AuthRepoError>;
    async fn save(&self, user: &User) -> Result<UserUuid, AuthRepoError>;
    async fn update(&self, user: &User) -> Result<(), AuthRepoError>;
    async fn find_by_email(&self, email: UserEmail) -> Result<Option<User>, AuthRepoError>;
}
