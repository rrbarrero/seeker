use async_trait::async_trait;

use crate::{
    auth::domain::entities::user::User,
    shared::domain::{error::AuthRepositoryError, value_objects::UserUuid},
};

#[async_trait]
pub trait IUserRepository {
    async fn get(&self, user_id: UserUuid) -> Result<Option<User>, AuthRepositoryError>;
    async fn save(&mut self, user: &User) -> Result<UserUuid, AuthRepositoryError>;
}
