use std::str::FromStr;
use std::sync::Arc;

use crate::{
    auth::domain::repositories::user_repository::IUserRepository,
    shared::{
        domain::value_objects::UserUuid, infrastructure::http::auth_extractor::UserStatusChecker,
    },
};

pub struct UserStatusCheckerImpl {
    user_repository: Arc<Box<dyn IUserRepository>>,
}

impl UserStatusCheckerImpl {
    pub fn new(user_repository: Arc<Box<dyn IUserRepository>>) -> Self {
        Self { user_repository }
    }
}

#[async_trait::async_trait]
impl UserStatusChecker for UserStatusCheckerImpl {
    async fn is_account_disabled(&self, user_id: &str) -> bool {
        let uuid = match UserUuid::from_str(user_id) {
            Ok(uuid) => uuid,
            Err(_) => return true, // Treat invalid UUIDs as "account disabled" or invalid
        };

        match self.user_repository.get(uuid).await {
            Ok(Some(user)) => user.account_disabled,
            Ok(None) => true, // Treat non-existent users as disabled/invalid
            Err(_) => true,   // Treat errors as disabled (fail safe)
        }
    }
}
