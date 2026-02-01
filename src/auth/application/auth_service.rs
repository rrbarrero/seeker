use crate::auth::{
    domain::{entities::user::UserEmail, repositories::user_repository::IUserRepository},
    infrastructure::http::errors::AuthError,
};

pub struct AuthService {
    user_repository: Box<dyn IUserRepository>,
}

impl AuthService {
    pub fn new(user_repository: Box<dyn IUserRepository>) -> Self {
        Self { user_repository }
    }

    pub async fn login(&self, email: &str, password: &str) -> Result<String, AuthError> {
        let user_email: UserEmail = UserEmail::new(email).map_err(|_| AuthError::InvalidEmail)?;
        let user = self.user_repository.find_by_email(user_email).await;
        match user {
            Ok(Some(user)) => match user.verify_password(password) {
                Ok(true) => Ok(user.id.to_string()),
                Ok(false) => Err(AuthError::InvalidPassword),
                Err(_) => Err(AuthError::InternalServerError),
            },
            _ => Err(AuthError::InvalidCredentials),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::auth::{
        domain::entities::user::User,
        infrastructure::persistence::repositories::user_in_memory_repository::UserInMemoryRepository,
    };
    use uuid::Uuid;

    #[tokio::test]
    async fn test_auth_service_login_success() {
        let user_id = Uuid::new_v4();
        let user = User::new(
            &user_id.to_string(),
            "test@example.com",
            "S0m3V3ryStr0ngP@ssw0rd!",
        )
        .expect("Error creating user");
        let repo = Box::new(UserInMemoryRepository::default());
        repo.save(&user).await.unwrap();
        let auth_service = AuthService::new(repo);

        let result = auth_service
            .login("test@example.com", "S0m3V3ryStr0ngP@ssw0rd!")
            .await;
        assert_eq!(result, Ok(user_id.to_string()));
    }

    #[tokio::test]
    async fn test_auth_service_login_invalid_email() {
        let repo = Box::new(UserInMemoryRepository::default());
        let auth_service = AuthService::new(repo);

        let result = auth_service.login("invalid-email", "password").await;
        assert_eq!(result, Err(AuthError::InvalidEmail));
    }

    #[tokio::test]
    async fn test_auth_service_login_invalid_password() {
        let user_id = Uuid::new_v4();
        let user = User::new(
            &user_id.to_string(),
            "test@example.com",
            "S0m3V3ryStr0ngP@ssw0rd!",
        )
        .expect("Error creating user");
        let repo = Box::new(UserInMemoryRepository::default());
        repo.save(&user).await.unwrap();
        let auth_service = AuthService::new(repo);

        let result = auth_service
            .login("test@example.com", "wrong-password")
            .await;
        assert_eq!(result, Err(AuthError::InvalidPassword));
    }

    #[tokio::test]
    async fn test_auth_service_login_user_not_found() {
        let repo = Box::new(UserInMemoryRepository::default());
        let auth_service = AuthService::new(repo);

        let result = auth_service
            .login("nonexistent@example.com", "password")
            .await;
        assert_eq!(result, Err(AuthError::InvalidCredentials));
    }
}
