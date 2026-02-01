use std::sync::Arc;

use axum::{
    Router,
    routing::{delete, get, post},
};

use crate::positions::{
    application::position_service::PositionService,
    presentation::handlers::{get_position, get_positions, remove_position, save_position},
};

pub fn create_position_routes(service: Arc<PositionService>) -> Router {
    Router::new()
        .route("/", get(get_positions))
        .route("/{id}", get(get_position))
        .route("/", post(save_position))
        .route("/{id}", delete(remove_position))
        .with_state(service)
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
    fn setup_router() -> Router {
        let repo = PositionInMemoryRepository::default();
        let service = Arc::new(PositionService::new(Box::new(repo)));
        create_position_routes(service)
    }

    #[tokio::test]
    async fn test_get_positions_empty() {
        let app = setup_router();

        let response = app
            .oneshot(Request::builder().uri("/").body(Body::empty()).unwrap())
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
        let app = setup_router();

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
            "initial_comment": "Applied via LinkedIn",
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
        let app = create_position_routes(service);

        let uri = format!("/{}", id);
        let response = app
            .oneshot(Request::builder().uri(&uri).body(Body::empty()).unwrap())
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
        let app = create_position_routes(service);

        let uri = format!("/{}", id);
        let response = app
            .oneshot(
                Request::builder()
                    .method("DELETE")
                    .uri(&uri)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::NO_CONTENT);
    }
}
