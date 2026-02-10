use axum::{
    Router,
    routing::{delete, get, post, put},
};

use crate::positions::presentation::comment_handlers::{
    get_comment, get_comments_for_position, remove_comment, save_comment, update_comment,
};
use crate::positions::presentation::routes::PositionState;

pub fn create_comment_routes() -> Router<PositionState> {
    Router::new()
        .route("/", get(get_comments_for_position))
        .route("/", post(save_comment))
        .route("/{comment_id}", get(get_comment))
        .route("/{comment_id}", put(update_comment))
        .route("/{comment_id}", delete(remove_comment))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::positions::{
        application::{comment_service::CommentService, position_service::PositionService},
        domain::entities::position::PositionBuilder,
        infrastructure::persistence::repositories::{
            comment_in_memory_repository::CommentInMemoryRepository,
            position_in_memory_repository::PositionInMemoryRepository,
        },
        presentation::routes::create_position_routes,
    };
    use crate::shared::config::Config;
    use axum::{
        body::Body,
        http::{Request, StatusCode},
    };
    use tower::ServiceExt;
    use uuid::Uuid;

    fn get_auth_header_for_user(config: &Config, user_id: &Uuid) -> String {
        let token = crate::shared::infrastructure::http::auth_extractor::create_jwt(
            &user_id.to_string(),
            "test@example.com",
            config,
        )
        .unwrap();
        format!("Bearer {}", token)
    }

    async fn setup_router_with_position(
        user_id: &Uuid,
    ) -> (
        Router,
        Config,
        crate::positions::domain::entities::position::PositionUuid,
    ) {
        use crate::positions::domain::repositories::position_repository::IPositionRepository;

        let position_repo = PositionInMemoryRepository::default();
        let comment_repo = CommentInMemoryRepository::default();

        // Create a position owned by user_id
        let position = PositionBuilder::new()
            .with_user_uuid(&user_id.to_string())
            .expect("valid uuid")
            .with_company("Test Corp")
            .with_role_title("Engineer")
            .build();
        let position_id = position.id;

        position_repo.save(position).await.unwrap();

        let position_repo: Box<dyn IPositionRepository> = Box::new(position_repo);
        let comment_repo: Box<
            dyn crate::positions::domain::repositories::comment_repository::ICommentRepository,
        > = Box::new(comment_repo);
        let position_service = PositionService::new(position_repo);
        let comment_service = CommentService::new(comment_repo);
        let config = Config::test_default();
        let app = create_position_routes(
            std::sync::Arc::new(position_service),
            std::sync::Arc::new(comment_service),
            std::sync::Arc::new(config.clone()),
        );
        (app, config, position_id)
    }

    #[tokio::test]
    async fn test_get_comments_empty() {
        let user_id = Uuid::new_v4();
        let (app, config, position_id) = setup_router_with_position(&user_id).await;
        let uri = format!("/{}/comments", position_id);
        let auth_header = get_auth_header_for_user(&config, &user_id);

        let response = app
            .oneshot(
                Request::builder()
                    .uri(&uri)
                    .header("Authorization", auth_header)
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
    async fn test_get_comments_forbidden_for_non_owner() {
        let owner_id = Uuid::new_v4();
        let other_id = Uuid::new_v4();
        let (app, config, position_id) = setup_router_with_position(&owner_id).await;
        let uri = format!("/{}/comments", position_id);
        let auth_header = get_auth_header_for_user(&config, &other_id);

        let response = app
            .oneshot(
                Request::builder()
                    .uri(&uri)
                    .header("Authorization", auth_header)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::FORBIDDEN);
    }

    #[tokio::test]
    async fn test_save_comment() {
        let user_id = Uuid::new_v4();
        let (app, config, position_id) = setup_router_with_position(&user_id).await;
        let uri = format!("/{}/comments", position_id);
        let auth_header = get_auth_header_for_user(&config, &user_id);

        let body_json = r#"{"body": "Hello comment"}"#;

        let response = app
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri(&uri)
                    .header("content-type", "application/json")
                    .header("Authorization", auth_header)
                    .body(Body::from(body_json))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::CREATED);
    }

    #[tokio::test]
    async fn test_save_comment_forbidden_for_non_owner() {
        let owner_id = Uuid::new_v4();
        let other_id = Uuid::new_v4();
        let (app, config, position_id) = setup_router_with_position(&owner_id).await;
        let uri = format!("/{}/comments", position_id);
        let auth_header = get_auth_header_for_user(&config, &other_id);

        let body_json = r#"{"body": "Hello comment"}"#;

        let response = app
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri(&uri)
                    .header("content-type", "application/json")
                    .header("Authorization", auth_header)
                    .body(Body::from(body_json))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::FORBIDDEN);
    }

    #[tokio::test]
    async fn test_get_comment_by_id() {
        let user_id = Uuid::new_v4();
        let (app, config, position_id) = setup_router_with_position(&user_id).await;
        let auth_header = get_auth_header_for_user(&config, &user_id);

        let create_uri = format!("/{}/comments", position_id);
        let body_json = r#"{"body": "Hello comment"}"#;

        let response = app
            .clone()
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri(&create_uri)
                    .header("content-type", "application/json")
                    .header("Authorization", auth_header.clone())
                    .body(Body::from(body_json))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::CREATED);

        let body_bytes = axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .unwrap();
        let json: serde_json::Value = serde_json::from_slice(&body_bytes).unwrap();
        let comment_id = json.get("id").unwrap().as_str().unwrap();

        let uri = format!("/{}/comments/{}", position_id, comment_id);
        let response = app
            .oneshot(
                Request::builder()
                    .uri(&uri)
                    .header("Authorization", auth_header)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn test_update_comment() {
        let user_id = Uuid::new_v4();
        let (app, config, position_id) = setup_router_with_position(&user_id).await;
        let auth_header = get_auth_header_for_user(&config, &user_id);

        let create_uri = format!("/{}/comments", position_id);
        let body_json = r#"{"body": "Hello comment"}"#;

        let response = app
            .clone()
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri(&create_uri)
                    .header("content-type", "application/json")
                    .header("Authorization", auth_header.clone())
                    .body(Body::from(body_json))
                    .unwrap(),
            )
            .await
            .unwrap();

        let body_bytes = axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .unwrap();
        let json: serde_json::Value = serde_json::from_slice(&body_bytes).unwrap();
        let comment_id = json.get("id").unwrap().as_str().unwrap();

        let update_uri = format!("/{}/comments/{}", position_id, comment_id);
        let update_body = r#"{"body": "Updated body"}"#;

        let response = app
            .oneshot(
                Request::builder()
                    .method("PUT")
                    .uri(&update_uri)
                    .header("content-type", "application/json")
                    .header("Authorization", auth_header)
                    .body(Body::from(update_body))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn test_update_comment_forbidden_for_non_owner() {
        let owner_id = Uuid::new_v4();
        let other_id = Uuid::new_v4();
        let (app, config, position_id) = setup_router_with_position(&owner_id).await;
        let owner_auth = get_auth_header_for_user(&config, &owner_id);
        let other_auth = get_auth_header_for_user(&config, &other_id);

        let create_uri = format!("/{}/comments", position_id);
        let body_json = r#"{"body": "Hello comment"}"#;

        let response = app
            .clone()
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri(&create_uri)
                    .header("content-type", "application/json")
                    .header("Authorization", owner_auth)
                    .body(Body::from(body_json))
                    .unwrap(),
            )
            .await
            .unwrap();

        let body_bytes = axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .unwrap();
        let json: serde_json::Value = serde_json::from_slice(&body_bytes).unwrap();
        let comment_id = json.get("id").unwrap().as_str().unwrap();

        let update_uri = format!("/{}/comments/{}", position_id, comment_id);
        let update_body = r#"{"body": "Updated body"}"#;

        let response = app
            .oneshot(
                Request::builder()
                    .method("PUT")
                    .uri(&update_uri)
                    .header("content-type", "application/json")
                    .header("Authorization", other_auth)
                    .body(Body::from(update_body))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::FORBIDDEN);
    }

    #[tokio::test]
    async fn test_remove_comment() {
        let user_id = Uuid::new_v4();
        let (app, config, position_id) = setup_router_with_position(&user_id).await;
        let auth_header = get_auth_header_for_user(&config, &user_id);

        let create_uri = format!("/{}/comments", position_id);
        let body_json = r#"{"body": "Hello comment"}"#;

        let response = app
            .clone()
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri(&create_uri)
                    .header("content-type", "application/json")
                    .header("Authorization", auth_header.clone())
                    .body(Body::from(body_json))
                    .unwrap(),
            )
            .await
            .unwrap();

        let body_bytes = axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .unwrap();
        let json: serde_json::Value = serde_json::from_slice(&body_bytes).unwrap();
        let comment_id = json.get("id").unwrap().as_str().unwrap();

        let delete_uri = format!("/{}/comments/{}", position_id, comment_id);
        let response = app
            .oneshot(
                Request::builder()
                    .method("DELETE")
                    .uri(&delete_uri)
                    .header("Authorization", auth_header)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::NO_CONTENT);
    }

    #[tokio::test]
    async fn test_remove_comment_forbidden_for_non_owner() {
        let owner_id = Uuid::new_v4();
        let other_id = Uuid::new_v4();
        let (app, config, position_id) = setup_router_with_position(&owner_id).await;
        let owner_auth = get_auth_header_for_user(&config, &owner_id);
        let other_auth = get_auth_header_for_user(&config, &other_id);

        let create_uri = format!("/{}/comments", position_id);
        let body_json = r#"{"body": "Hello comment"}"#;

        let response = app
            .clone()
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri(&create_uri)
                    .header("content-type", "application/json")
                    .header("Authorization", owner_auth)
                    .body(Body::from(body_json))
                    .unwrap(),
            )
            .await
            .unwrap();

        let body_bytes = axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .unwrap();
        let json: serde_json::Value = serde_json::from_slice(&body_bytes).unwrap();
        let comment_id = json.get("id").unwrap().as_str().unwrap();

        let delete_uri = format!("/{}/comments/{}", position_id, comment_id);
        let response = app
            .oneshot(
                Request::builder()
                    .method("DELETE")
                    .uri(&delete_uri)
                    .header("Authorization", other_auth)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::FORBIDDEN);
    }
}
