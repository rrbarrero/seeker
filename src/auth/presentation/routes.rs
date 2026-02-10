use axum::{
    Router,
    extract::FromRef,
    routing::{get, post},
};
use std::sync::Arc;

use crate::{
    auth::{
        application::auth_service::AuthService,
        presentation::handlers::{login, signup, verify_email},
    },
    shared::config::Config,
};

#[derive(Clone)]
struct AuthState {
    service: Arc<AuthService>,
    config: Arc<Config>,
}

impl FromRef<AuthState> for Arc<AuthService> {
    fn from_ref(state: &AuthState) -> Self {
        state.service.clone()
    }
}

impl FromRef<AuthState> for Arc<Config> {
    fn from_ref(state: &AuthState) -> Self {
        state.config.clone()
    }
}

pub fn create_auth_routes(service: Arc<AuthService>, config: Arc<Config>) -> Router {
    let state = AuthState { service, config };
    Router::new()
        .route("/login", post(login))
        .route("/signup", post(signup))
        .route("/verify-email", get(verify_email))
        .with_state(state)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::auth::domain::entities::user::User;
    use crate::auth::presentation::dtos::LoginDto;
    use crate::auth::{
        domain::repositories::user_repository::IUserRepository,
        presentation::dtos::SuccesfullLoginDto,
    };
    use axum::{
        body::Body,
        http::{Request, StatusCode, header},
    };
    use tower::ServiceExt;
    use uuid::Uuid;

    use crate::auth::application::auth_service::AuthService;
    use crate::auth::application::email_queue_enqueuer::IEmailQueueEnqueuer;
    use crate::auth::application::errors::AuthError;
    use crate::auth::application::token_generator::ITokenGenerator;
    use crate::auth::presentation::dtos::{SignupDto, UserUuidDto};
    use crate::composition_root::create_user_in_memory_repository;
    use crate::shared::fixtures::{valid_email, valid_password};
    use std::sync::Mutex;

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
            } else {
                Err(AuthError::InvalidToken)
            }
        }
    }

    async fn setup_router() -> Router {
        let repo = create_user_in_memory_repository().await;
        let user = User::new(&Uuid::new_v4().to_string(), valid_email(), valid_password()).unwrap();
        repo.save(&user).await.unwrap();

        let config = Arc::new(Config::test_default());
        let token_generator = Box::new(MockTokenGenerator::new());
        let email_queue = Box::new(MockEmailQueue);
        let service = Arc::new(AuthService::new(
            Box::new(repo),
            token_generator,
            email_queue,
            "http://localhost:3000".to_string(),
        ));
        create_auth_routes(service, config)
    }

    fn json_request(uri: &str, method: &str, body: impl serde::Serialize) -> Request<Body> {
        Request::builder()
            .uri(uri)
            .method(method)
            .header(header::CONTENT_TYPE, "application/json")
            .body(Body::from(serde_json::to_string(&body).unwrap()))
            .unwrap()
    }

    #[tokio::test]
    async fn test_login_success() {
        let app = setup_router().await;

        let login_dto = LoginDto {
            email: valid_email().to_string(),
            password: valid_password().to_string(),
        };

        let response = app
            .oneshot(json_request("/login", "POST", login_dto))
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
        let body = axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .unwrap();
        let token = serde_json::from_slice::<SuccesfullLoginDto>(&body).unwrap();
        assert!(!token.access_token.is_empty());
    }

    #[tokio::test]
    async fn test_login_invalid_email() {
        let app = setup_router().await;

        let login_dto = LoginDto {
            email: "invalid-email".to_string(),
            password: valid_password().to_string(),
        };

        let response = app
            .oneshot(json_request("/login", "POST", login_dto))
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    }

    #[tokio::test]
    async fn test_login_invalid_password() {
        let app = setup_router().await;

        let login_dto = LoginDto {
            email: valid_email().to_string(),
            password: "wrong-password".to_string(),
        };

        let response = app
            .oneshot(json_request("/login", "POST", login_dto))
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    }

    #[tokio::test]
    async fn test_login_user_not_found() {
        let app = setup_router().await;

        let login_dto = LoginDto {
            email: "nonexistent@example.com".to_string(),
            password: valid_password().to_string(),
        };

        let response = app
            .oneshot(json_request("/login", "POST", login_dto))
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    }

    #[tokio::test]
    async fn test_signup_success() {
        let app = setup_router().await;

        let signup_dto = SignupDto {
            email: "newuser@example.com".to_string(),
            password: valid_password().to_string(),
        };

        let response = app
            .oneshot(json_request("/signup", "POST", signup_dto))
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
        let body = axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .unwrap();
        let user_uuid = serde_json::from_slice::<UserUuidDto>(&body).unwrap();
        assert!(!user_uuid.user_uuid.is_empty());
    }

    #[tokio::test]
    async fn test_signup_invalid_input_data() {
        let app = setup_router().await;

        let signup_dto = SignupDto {
            email: "invalid-email".to_string(),
            password: "short".to_string(),
        };

        let response = app
            .oneshot(json_request("/signup", "POST", signup_dto))
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    }
}
