use async_trait::async_trait;

use crate::auth::domain::entities::{
    user::{User, UserUuid},
    user_error::UserValueError,
};

#[async_trait]
pub trait IUserRepository {
    async fn get(&self, user_id: UserUuid) -> Option<User>;
    async fn save(&mut self, user: User) -> Result<UserUuid, UserValueError>;
}
