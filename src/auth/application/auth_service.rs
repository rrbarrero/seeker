use std::str::FromStr;
use uuid::Uuid;

use crate::auth::{
    application::errors::AuthError,
    domain::{
        entities::user::{User, UserEmail},
        repositories::user_repository::IUserRepository,
    },
};
use crate::shared::domain::value_objects::UserUuid;

use crate::auth::application::email_queue_enqueuer::IEmailQueueEnqueuer;
use crate::auth::application::token_generator::ITokenGenerator;
use tracing::{error, info, warn};

pub struct AuthService {
    user_repository: Box<dyn IUserRepository>,
    token_generator: Box<dyn ITokenGenerator>,
    email_queue: Box<dyn IEmailQueueEnqueuer>,
    frontend_url: String,
}

impl AuthService {
    pub fn new(
        user_repository: Box<dyn IUserRepository>,
        token_generator: Box<dyn ITokenGenerator>,
        email_queue: Box<dyn IEmailQueueEnqueuer>,
        frontend_url: String,
    ) -> Self {
        Self {
            user_repository,
            token_generator,
            email_queue,
            frontend_url,
        }
    }

    pub async fn login(&self, email: &str, password: &str) -> Result<(String, bool), AuthError> {
        let user_email: UserEmail = UserEmail::new(email).map_err(AuthError::from)?;
        let user = self.user_repository.find_by_email(user_email).await;
        match user {
            Ok(Some(user)) => match user.verify_password(password) {
                Ok(true) => {
                    let token = self
                        .token_generator
                        .generate_token(&user.id.value().to_string(), user.email.value())?;
                    Ok((token, user.email_validated))
                }
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

        let saved_id = match self.user_repository.save(&user).await {
            Ok(user_id) => user_id,
            Err(err) => {
                let kind = auth_repo_error_kind(&err);
                if kind == "user_already_exists" {
                    warn!(error_kind = kind, error = %err, "auth_service.signup failed");
                } else {
                    error!(error_kind = kind, error = %err, "auth_service.signup failed");
                }
                return Err(AuthError::from(err));
            }
        };

        self.enqueue_verification_email(&user).await;

        Ok(saved_id)
    }

    pub async fn verify_email(&self, token: &str) -> Result<(), AuthError> {
        let user_id_str = self.token_generator.validate_token(token)?;

        let user_id = UserUuid::from_str(&user_id_str)
            .map_err(|e| AuthError::InternalError(e.to_string()))?;

        let mut user = self
            .user_repository
            .get(user_id)
            .await
            .map_err(|e| AuthError::InternalError(e.to_string()))?
            .ok_or(AuthError::UserNotFound)?;

        user.validate_email();

        self.user_repository
            .update(&user)
            .await
            .map_err(|e| AuthError::InternalError(e.to_string()))?;

        info!(user_id = %user_id_str, "Email verified successfully");
        Ok(())
    }

    async fn enqueue_verification_email(&self, user: &User) {
        let email_queue = &self.email_queue;

        let token = match self
            .token_generator
            .generate_token(&user.id.value().to_string(), user.email.value())
        {
            Ok(t) => t,
            Err(e) => {
                error!(error = %e, "Failed to generate verification token");
                return;
            }
        };

        let verification_link = format!("{}/auth/verify-email?token={}", self.frontend_url, token);

        let subject = "Verifica tu email";
        let body = format!(
            "Hola,\n\nPor favor verifica tu email haciendo clic en el siguiente enlace:\n\n{}\n\nEste enlace expirará en unas horas.\n\nSi no te has registrado, ignora este mensaje.",
            verification_link
        );

        if let Err(e) = email_queue
            .enqueue(user.email.value(), subject, &body)
            .await
        {
            error!(
                error = %e,
                email = user.email.value(),
                "Failed to enqueue verification email"
            );
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
    use std::sync::Mutex;
    use uuid::Uuid;

    struct MockTokenGenerator {
        last_user_id: Mutex<Option<String>>,
    }

    struct MockEmailQueue;
    #[async_trait::async_trait]
    impl IEmailQueueEnqueuer for MockEmailQueue {
        async fn enqueue(
            &self,
            _email: &str,
            _subject: &str,
            _body: &str,
        ) -> Result<(), AuthError> {
            Ok(())
        }
    }

    impl MockTokenGenerator {
        fn new() -> Self {
            Self {
                last_user_id: Mutex::new(None),
            }
        }
    }

    impl ITokenGenerator for MockTokenGenerator {
        fn generate_token(&self, user_id: &str, _email: &str) -> Result<String, AuthError> {
            *self.last_user_id.lock().unwrap() = Some(user_id.to_string());
            Ok("mock-token".to_string())
        }

        fn validate_token(&self, token: &str) -> Result<String, AuthError> {
            if token == "mock-token" {
                let user_id = self.last_user_id.lock().unwrap();
                Ok(user_id.clone().unwrap_or_default())
            } else if token == "expired-token" {
                Err(AuthError::TokenExpired)
            } else {
                Err(AuthError::InvalidToken)
            }
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
        let auth_service = AuthService::new(
            repo,
            Box::new(MockTokenGenerator::new()),
            Box::new(MockEmailQueue),
            "http://localhost:3000".to_string(),
        );

        let result = auth_service
            .login("test@example.com", "S0m3V3ryStr0ngP@ssw0rd!")
            .await;
        assert_eq!(result, Ok(("mock-token".to_string(), false)));
    }

    #[tokio::test]
    async fn test_auth_service_login_invalid_email() {
        let repo = Box::new(UserInMemoryRepository::default());
        let auth_service = AuthService::new(
            repo,
            Box::new(MockTokenGenerator::new()),
            Box::new(MockEmailQueue),
            "http://localhost:3000".to_string(),
        );

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
        let auth_service = AuthService::new(
            repo,
            Box::new(MockTokenGenerator::new()),
            Box::new(MockEmailQueue),
            "http://localhost:3000".to_string(),
        );

        let result = auth_service
            .login("test@example.com", "wrong-password")
            .await;
        assert_eq!(result, Err(AuthError::InvalidCredentials));
    }

    #[tokio::test]
    async fn test_auth_service_login_user_not_found() {
        let repo = Box::new(UserInMemoryRepository::default());
        let auth_service = AuthService::new(
            repo,
            Box::new(MockTokenGenerator::new()),
            Box::new(MockEmailQueue),
            "http://localhost:3000".to_string(),
        );

        let result = auth_service
            .login("nonexistent@example.com", "password")
            .await;
        assert_eq!(result, Err(AuthError::InvalidCredentials));
    }

    #[tokio::test]
    async fn test_auth_service_signup_success() {
        let repo = Box::new(UserInMemoryRepository::default());
        let auth_service = AuthService::new(
            repo,
            Box::new(MockTokenGenerator::new()),
            Box::new(MockEmailQueue),
            "http://localhost:3000".to_string(),
        );

        let result = auth_service
            .signup("test@example.com", "S0m3V3ryStr0ngP@ssw0rd!")
            .await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_auth_service_signup_invalid_email() {
        let repo = Box::new(UserInMemoryRepository::default());
        let auth_service = AuthService::new(
            repo,
            Box::new(MockTokenGenerator::new()),
            Box::new(MockEmailQueue),
            "http://localhost:3000".to_string(),
        );

        let result = auth_service.signup("invalid-email", "password").await;
        assert!(matches!(result, Err(AuthError::DomainError(_))));
    }

    #[tokio::test]
    async fn test_auth_service_signup_invalid_password() {
        let repo = Box::new(UserInMemoryRepository::default());
        let auth_service = AuthService::new(
            repo,
            Box::new(MockTokenGenerator::new()),
            Box::new(MockEmailQueue),
            "http://localhost:3000".to_string(),
        );

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
        let auth_service = AuthService::new(
            repo,
            Box::new(MockTokenGenerator::new()),
            Box::new(MockEmailQueue),
            "http://localhost:3000".to_string(),
        );

        let result = auth_service.signup(email, "S0m3V3ryStr0ngP@ssw0rd!").await;
        assert!(matches!(
            result,
            Err(AuthError::RepositoryError(
                AuthRepoError::UserAlreadyExists(_)
            ))
        ));
    }

    #[tokio::test]
    async fn test_verify_email_success() {
        let user_id = Uuid::new_v4();
        let user = User::new(
            &user_id.to_string(),
            "test@example.com",
            "S0m3V3ryStr0ngP@ssw0rd!",
        )
        .expect("Error creating user");

        assert!(!user.email_validated);

        let repo = Box::new(UserInMemoryRepository::default());
        repo.save(&user).await.unwrap();

        let token_gen = MockTokenGenerator::new();
        token_gen
            .generate_token(&user_id.to_string(), "test@example.com")
            .unwrap();

        let auth_service = AuthService::new(
            repo.clone(),
            Box::new(token_gen),
            Box::new(MockEmailQueue),
            "http://localhost:3000".to_string(),
        );

        let result = auth_service.verify_email("mock-token").await;
        assert!(result.is_ok());

        let updated_user = repo
            .get(UserUuid::from_str(&user_id.to_string()).unwrap())
            .await
            .unwrap()
            .unwrap();
        assert!(updated_user.email_validated);
    }

    #[tokio::test]
    async fn test_verify_email_user_not_found() {
        let repo = Box::new(UserInMemoryRepository::default());
        let non_existent_id = Uuid::new_v4();
        let token_gen = MockTokenGenerator::new();
        token_gen
            .generate_token(&non_existent_id.to_string(), "ghost@example.com")
            .unwrap();

        let auth_service = AuthService::new(
            repo,
            Box::new(token_gen),
            Box::new(MockEmailQueue),
            "http://localhost:3000".to_string(),
        );

        let result = auth_service.verify_email("mock-token").await;
        assert_eq!(result, Err(AuthError::UserNotFound));
    }

    #[tokio::test]
    async fn test_verify_email_invalid_token() {
        let repo = Box::new(UserInMemoryRepository::default());
        let auth_service = AuthService::new(
            repo,
            Box::new(MockTokenGenerator::new()),
            Box::new(MockEmailQueue),
            "http://localhost:3000".to_string(),
        );

        let result = auth_service.verify_email("bad-token").await;
        assert_eq!(result, Err(AuthError::InvalidToken));
    }
}
