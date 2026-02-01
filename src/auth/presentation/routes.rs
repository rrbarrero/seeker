use axum::{Router, extract::FromRef, routing::post};
use std::sync::Arc;

use crate::{
    auth::{application::auth_service::AuthService, presentation::handlers::login},
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
    Router::new().route("/login", post(login)).with_state(state)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::auth::domain::entities::user::User;
    use crate::auth::presentation::dtos::LoginDto;
    use crate::auth::{
        domain::repositories::user_repository::IUserRepository,
        infrastructure::persistence::repositories::user_in_memory_repository::UserInMemoryRepository,
        presentation::dtos::SuccesfullLoginDto,
    };
    use axum::{
        body::Body,
        http::{Request, StatusCode, header},
    };
    use tower::ServiceExt;
    use uuid::Uuid;

    async fn setup_router() -> Router {
        let repo = UserInMemoryRepository::default();
        let user = User::new(
            &Uuid::new_v4().to_string(),
            "test@example.com",
            "S0m3V3ryStr0ngP@ssw0rd!",
        )
        .unwrap();
        repo.save(&user).await.unwrap();

        let service = Arc::new(AuthService::new(Box::new(repo)));
        let config = Arc::new(Config::test_default());
        create_auth_routes(service, config)
    }

    fn post_request(dto: LoginDto) -> Request<Body> {
        Request::builder()
            .uri("/login")
            .method("POST")
            .header(header::CONTENT_TYPE, "application/json")
            .body(Body::from(serde_json::to_string(&dto).unwrap()))
            .unwrap()
    }

    #[tokio::test]
    async fn test_login_success() {
        let app = setup_router().await;

        let login_dto = LoginDto {
            email: "test@example.com".to_string(),
            password: "S0m3V3ryStr0ngP@ssw0rd!".to_string(),
        };

        let response = app.oneshot(post_request(login_dto)).await.unwrap();

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
            password: "S0m3V3ryStr0ngP@ssw0rd!".to_string(),
        };

        let response = app.oneshot(post_request(login_dto)).await.unwrap();

        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    }

    #[tokio::test]
    async fn test_login_invalid_password() {
        let app = setup_router().await;

        let login_dto = LoginDto {
            email: "test@example.com".to_string(),
            password: "wrong-password".to_string(),
        };

        let response = app.oneshot(post_request(login_dto)).await.unwrap();

        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    }

    #[tokio::test]
    async fn test_login_user_not_found() {
        let app = setup_router().await;

        let login_dto = LoginDto {
            email: "nonexistent@example.com".to_string(),
            password: "S0m3V3ryStr0ngP@ssw0rd!".to_string(),
        };

        let response = app.oneshot(post_request(login_dto)).await.unwrap();

        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    }
}
