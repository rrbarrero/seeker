use async_trait::async_trait;

use crate::{
    auth::domain::entities::user::{User, UserEmail},
    shared::domain::{error::AuthRepositoryError, value_objects::UserUuid},
};

#[async_trait]
pub trait IUserRepository: Send + Sync {
    async fn get(&self, user_id: UserUuid) -> Result<Option<User>, AuthRepositoryError>;
    async fn save(&self, user: &User) -> Result<UserUuid, AuthRepositoryError>;
    async fn find_by_email(&self, email: UserEmail) -> Result<Option<User>, AuthRepositoryError>;
}
