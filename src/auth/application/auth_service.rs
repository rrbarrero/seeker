use uuid::Uuid;

use crate::auth::{
    application::errors::AuthError,
    domain::{
        entities::user::{User, UserEmail},
        repositories::user_repository::IUserRepository,
    },
};
use crate::shared::domain::value_objects::UserUuid;

pub struct AuthService {
    user_repository: Box<dyn IUserRepository>,
}

impl AuthService {
    pub fn new(user_repository: Box<dyn IUserRepository>) -> Self {
        Self { user_repository }
    }

    pub async fn login(&self, email: &str, password: &str) -> Result<User, AuthError> {
        let user_email: UserEmail = UserEmail::new(email).map_err(AuthError::from)?;
        let user = self.user_repository.find_by_email(user_email).await;
        match user {
            Ok(Some(user)) => match user.verify_password(password) {
                Ok(true) => Ok(user),
                Ok(false) => Err(AuthError::InvalidCredentials),
                Err(e) => Err(AuthError::from(e)),
            },
            _ => Err(AuthError::InvalidCredentials),
        }
    }

    pub async fn signup(&self, email: &str, password: &str) -> Result<UserUuid, AuthError> {
        let user_id = Uuid::new_v4().to_string();
        let user = User::new(&user_id, email, password)?;
        let user_id = self.user_repository.save(&user).await?;
        Ok(user_id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::auth::{
        application::errors::AuthError,
        domain::{entities::user::User, errors::AuthRepoError},
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
        assert_eq!(result, Ok(user));
    }

    #[tokio::test]
    async fn test_auth_service_login_invalid_email() {
        let repo = Box::new(UserInMemoryRepository::default());
        let auth_service = AuthService::new(repo);

        let result = auth_service.login("invalid-email", "password").await;
        assert!(matches!(result, Err(AuthError::DomainError(_))));
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
        assert_eq!(result, Err(AuthError::InvalidCredentials));
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

    #[tokio::test]
    async fn test_auth_service_signup_success() {
        let repo = Box::new(UserInMemoryRepository::default());
        let auth_service = AuthService::new(repo);

        let result = auth_service
            .signup("test@example.com", "S0m3V3ryStr0ngP@ssw0rd!")
            .await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_auth_service_signup_invalid_email() {
        let repo = Box::new(UserInMemoryRepository::default());
        let auth_service = AuthService::new(repo);

        let result = auth_service.signup("invalid-email", "password").await;
        assert!(matches!(result, Err(AuthError::DomainError(_))));
    }

    #[tokio::test]
    async fn test_auth_service_signup_invalid_password() {
        let repo = Box::new(UserInMemoryRepository::default());
        let auth_service = AuthService::new(repo);

        let result = auth_service.signup("test@example.com", "weak").await;
        assert!(matches!(result, Err(AuthError::DomainError(_))));
    }

    #[tokio::test]
    async fn test_auth_service_signup_user_already_exists() {
        let email = "test@example.com";
        let user_id = Uuid::new_v4();
        let user = User::new(&user_id.to_string(), email, "S0m3V3ryStr0ngP@ssw0rd!")
            .expect("Error creating user");
        let repo = Box::new(UserInMemoryRepository::default());
        repo.save(&user).await.unwrap();
        let auth_service = AuthService::new(repo);

        let result = auth_service.signup(email, "S0m3V3ryStr0ngP@ssw0rd!").await;
        println!("Result: {:?}", result);
        assert!(matches!(
            result,
            Err(AuthError::RepositoryError(
                AuthRepoError::UserAlreadyExists(_)
            ))
        ));
    }
}
