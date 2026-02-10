use uuid::Uuid;

use crate::auth::{
    application::errors::AuthError,
    domain::{
        entities::user::{User, UserEmail},
        repositories::user_repository::IUserRepository,
    },
};
use crate::shared::domain::value_objects::UserUuid;

use crate::auth::application::token_generator::ITokenGenerator;
use tracing::{error, warn};

pub struct AuthService {
    user_repository: Box<dyn IUserRepository>,
    token_generator: Box<dyn ITokenGenerator>,
}

impl AuthService {
    pub fn new(
        user_repository: Box<dyn IUserRepository>,
        token_generator: Box<dyn ITokenGenerator>,
    ) -> Self {
        Self {
            user_repository,
            token_generator,
        }
    }

    pub async fn login(&self, email: &str, password: &str) -> Result<String, AuthError> {
        let user_email: UserEmail = UserEmail::new(email).map_err(AuthError::from)?;
        let user = self.user_repository.find_by_email(user_email).await;
        match user {
            Ok(Some(user)) => match user.verify_password(password) {
                Ok(true) => self
                    .token_generator
                    .generate_token(&user.id.value().to_string(), user.email.value()),
                Ok(false) => {
                    warn!(
                        error_kind = "invalid_credentials",
                        "auth_service.login failed"
                    );
                    Err(AuthError::InvalidCredentials)
                }
                Err(e) => {
                    warn!(
                        error_kind = "password_verification_failed",
                        error = %e,
                        "auth_service.login failed"
                    );
                    Err(AuthError::from(e))
                }
            },
            Ok(None) => {
                warn!(
                    error_kind = "invalid_credentials",
                    "auth_service.login failed"
                );
                Err(AuthError::InvalidCredentials)
            }
            Err(err) => {
                error!(
                    error_kind = auth_repo_error_kind(&err),
                    error = %err,
                    "auth_service.login repository error"
                );
                Err(AuthError::InternalError(err.to_string()))
            }
        }
    }

    pub async fn signup(&self, email: &str, password: &str) -> Result<UserUuid, AuthError> {
        let user_id = Uuid::new_v4().to_string();
        let user = match User::new(&user_id, email, password) {
            Ok(user) => user,
            Err(err) => {
                warn!(
                    error_kind = auth_domain_error_kind(&err),
                    error = %err,
                    "auth_service.signup failed"
                );
                return Err(AuthError::from(err));
            }
        };

        match self.user_repository.save(&user).await {
            Ok(user_id) => Ok(user_id),
            Err(err) => {
                let kind = auth_repo_error_kind(&err);
                if kind == "user_already_exists" {
                    warn!(error_kind = kind, error = %err, "auth_service.signup failed");
                } else {
                    error!(error_kind = kind, error = %err, "auth_service.signup failed");
                }
                Err(AuthError::from(err))
            }
        }
    }
}

fn auth_domain_error_kind(err: &crate::auth::domain::errors::AuthDomainError) -> &'static str {
    use crate::auth::domain::errors::AuthDomainError;
    match err {
        AuthDomainError::Shared(_) => "shared_domain_error",
        AuthDomainError::InternalError(_) => "internal_error",
    }
}

fn auth_repo_error_kind(err: &crate::auth::domain::errors::AuthRepoError) -> &'static str {
    use crate::auth::domain::errors::AuthRepoError;
    match err {
        AuthRepoError::DatabaseError(_) => "database_error",
        AuthRepoError::ConversionError(_) => "conversion_error",
        AuthRepoError::NotFound(_) => "not_found",
        AuthRepoError::UserAlreadyExists(_) => "user_already_exists",
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::auth::{
        domain::{entities::user::User, errors::AuthRepoError},
        infrastructure::persistence::repositories::user_in_memory_repository::UserInMemoryRepository,
    };
    use uuid::Uuid;

    struct MockTokenGenerator;
    impl ITokenGenerator for MockTokenGenerator {
        fn generate_token(&self, _user_id: &str, _email: &str) -> Result<String, AuthError> {
            Ok("mock-token".to_string())
        }
    }

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
        let auth_service = AuthService::new(repo, Box::new(MockTokenGenerator));

        let result = auth_service
            .login("test@example.com", "S0m3V3ryStr0ngP@ssw0rd!")
            .await;
        assert_eq!(result, Ok("mock-token".to_string()));
    }

    #[tokio::test]
    async fn test_auth_service_login_invalid_email() {
        let repo = Box::new(UserInMemoryRepository::default());
        let auth_service = AuthService::new(repo, Box::new(MockTokenGenerator));

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
        let auth_service = AuthService::new(repo, Box::new(MockTokenGenerator));

        let result = auth_service
            .login("test@example.com", "wrong-password")
            .await;
        assert_eq!(result, Err(AuthError::InvalidCredentials));
    }

    #[tokio::test]
    async fn test_auth_service_login_user_not_found() {
        let repo = Box::new(UserInMemoryRepository::default());
        let auth_service = AuthService::new(repo, Box::new(MockTokenGenerator));

        let result = auth_service
            .login("nonexistent@example.com", "password")
            .await;
        assert_eq!(result, Err(AuthError::InvalidCredentials));
    }

    #[tokio::test]
    async fn test_auth_service_signup_success() {
        let repo = Box::new(UserInMemoryRepository::default());
        let auth_service = AuthService::new(repo, Box::new(MockTokenGenerator));

        let result = auth_service
            .signup("test@example.com", "S0m3V3ryStr0ngP@ssw0rd!")
            .await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_auth_service_signup_invalid_email() {
        let repo = Box::new(UserInMemoryRepository::default());
        let auth_service = AuthService::new(repo, Box::new(MockTokenGenerator));

        let result = auth_service.signup("invalid-email", "password").await;
        assert!(matches!(result, Err(AuthError::DomainError(_))));
    }

    #[tokio::test]
    async fn test_auth_service_signup_invalid_password() {
        let repo = Box::new(UserInMemoryRepository::default());
        let auth_service = AuthService::new(repo, Box::new(MockTokenGenerator));

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
        let auth_service = AuthService::new(repo, Box::new(MockTokenGenerator));

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
