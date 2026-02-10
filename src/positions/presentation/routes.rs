use std::sync::Arc;

use axum::{
    Router,
    extract::FromRef,
    routing::{delete, get, post, put},
};

use crate::positions::presentation::comment_routes::create_comment_routes;
use crate::{
    positions::{
        application::{comment_service::CommentService, position_service::PositionService},
        presentation::handlers::{
            get_position, get_positions, remove_position, save_position, update_position,
        },
    },
    shared::config::Config,
};

#[derive(Clone)]
pub struct PositionState {
    pub service: Arc<PositionService>,
    pub comment_service: Arc<CommentService>,
    pub config: Arc<Config>,
}

impl FromRef<PositionState> for Arc<PositionService> {
    fn from_ref(state: &PositionState) -> Self {
        state.service.clone()
    }
}

impl FromRef<PositionState> for Arc<CommentService> {
    fn from_ref(state: &PositionState) -> Self {
        state.comment_service.clone()
    }
}

impl FromRef<PositionState> for Arc<Config> {
    fn from_ref(state: &PositionState) -> Self {
        state.config.clone()
    }
}

pub fn create_position_routes(
    service: Arc<PositionService>,
    comment_service: Arc<CommentService>,
    config: Arc<Config>,
) -> Router {
    let state = PositionState {
        service,
        comment_service,
        config,
    };
    Router::new()
        .route("/", get(get_positions))
        .route("/{id}", get(get_position))
        .route("/", post(save_position))
        .route("/{id}", put(update_position))
        .route("/{id}", delete(remove_position))
        .nest("/{position_id}/comments", create_comment_routes())
        .with_state(state)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::positions::{
        domain::entities::position::PositionBuilder,
        domain::repositories::position_repository::IPositionRepository,
        infrastructure::persistence::repositories::position_in_memory_repository::PositionInMemoryRepository,
    };
    use axum::{
        body::Body,
        http::{Request, StatusCode},
    };
    use tower::ServiceExt; // Needed for calling .oneshot() on the router
    use uuid::Uuid;

    // Helper to setup the router with an in-memory repository
    fn setup_router() -> (Router, Arc<Config>) {
        let repo = PositionInMemoryRepository::default();
        let service = Arc::new(PositionService::new(Box::new(repo)));
        let comment_service = Arc::new(CommentService::new(Box::new(
            crate::positions::infrastructure::persistence::repositories::comment_in_memory_repository::CommentInMemoryRepository::default(),
        )));
        let config = Arc::new(Config::test_default());
        (
            create_position_routes(service, comment_service, config.clone()),
            config,
        )
    }

    fn get_auth_header(config: &Config) -> String {
        let token = crate::shared::infrastructure::http::auth_extractor::create_jwt(
            &Uuid::new_v4().to_string(),
            "test@example.com",
            config,
        )
        .unwrap();
        format!("Bearer {}", token)
    }

    #[tokio::test]
    async fn test_get_positions_empty() {
        let (app, config) = setup_router();

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/")
                    .header("Authorization", get_auth_header(&config))
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let body_bytes = axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .unwrap();
        assert_eq!(&body_bytes[..], b"[]");
    }

    #[tokio::test]
    async fn test_save_position() {
        let (app, config) = setup_router();

        let user_id = Uuid::new_v4();

        // precise-dto structure
        let body_json = format!(
            r#"{{
            "user_id": "{}",
            "company": "Rust Corp",
            "role_title": "Senior Rust Developer",
            "description": "Senior Rust Developer needed",
            "applied_on": "Fri, 27 Oct 2023 12:00:00 +0000", 
            "url": "https://rust.com/jobs/1",
            "status": "CvSent"
        }}"#,
            user_id
        );

        let response = app
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/")
                    .header("content-type", "application/json")
                    .header("Authorization", get_auth_header(&config))
                    .body(Body::from(body_json))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn test_get_position_by_id() {
        let repo = PositionInMemoryRepository::default();
        let position = PositionBuilder::new().with_role_title("Test Role").build();
        let id = position.id;

        let _ = repo.save(position.clone()).await;
        let service = Arc::new(PositionService::new(Box::new(repo)));
        let comment_service = Arc::new(CommentService::new(Box::new(
            crate::positions::infrastructure::persistence::repositories::comment_in_memory_repository::CommentInMemoryRepository::default(),
        )));
        let config = Arc::new(Config::test_default());
        let app = create_position_routes(service, comment_service, config.clone());

        let uri = format!("/{}", id);
        let response = app
            .oneshot(
                Request::builder()
                    .uri(&uri)
                    .header("Authorization", get_auth_header(&config))
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn test_remove_position() {
        let repo = PositionInMemoryRepository::default();
        let position = PositionBuilder::new().with_role_title("Test Role").build();
        let id = position.id;

        let _ = repo.save(position).await;
        let service = Arc::new(PositionService::new(Box::new(repo)));
        let comment_service = Arc::new(CommentService::new(Box::new(
            crate::positions::infrastructure::persistence::repositories::comment_in_memory_repository::CommentInMemoryRepository::default(),
        )));
        let config = Arc::new(Config::test_default());
        let app = create_position_routes(service, comment_service, config.clone());

        let uri = format!("/{}", id);
        let response = app
            .oneshot(
                Request::builder()
                    .method("DELETE")
                    .uri(&uri)
                    .header("Authorization", get_auth_header(&config))
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::NO_CONTENT);
    }

    #[tokio::test]
    async fn test_update_position() {
        let repo = PositionInMemoryRepository::default();
        let position = PositionBuilder::new().with_role_title("Test Role").build();
        let id = position.id;

        let _ = repo.save(position).await;
        let service = Arc::new(PositionService::new(Box::new(repo)));
        let comment_service = Arc::new(CommentService::new(Box::new(
            crate::positions::infrastructure::persistence::repositories::comment_in_memory_repository::CommentInMemoryRepository::default(),
        )));
        let config = Arc::new(Config::test_default());
        let app = create_position_routes(service, comment_service, config.clone());

        let body_json = r#"
        {
            "company": "Updated Co",
            "role_title": "Updated Role",
            "description": "Updated description",
            "applied_on": "Fri, 27 Oct 2023 12:00:00 +0000",
            "url": "https://example.com/jobs/1",
            "status": "CvSent"
        }
        "#;

        let uri = format!("/{}", id);
        let response = app
            .oneshot(
                Request::builder()
                    .method("PUT")
                    .uri(&uri)
                    .header("content-type", "application/json")
                    .header("Authorization", get_auth_header(&config))
                    .body(Body::from(body_json))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }
}
