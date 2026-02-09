use crate::shared::config::Config;
use axum::{
    Json,
    extract::{FromRef, FromRequestParts},
    http::{StatusCode, header::AUTHORIZATION, request::Parts},
    response::IntoResponse,
};
use chrono::Utc;
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, errors::ErrorKind};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use thiserror::Error;

pub fn create_jwt(sub: &str, email: &str, config: &Config) -> Result<String, AuthExtractorError> {
    let expiration = Utc::now().timestamp() + config.jwt_expiration_time;

    let claims = Claims {
        sub: sub.to_string(),
        exp: expiration as usize,
        email: email.to_string(),
    };

    let token = jsonwebtoken::encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(config.get_jwt_secret().as_bytes()),
    );

    match token {
        Ok(token) => Ok(token),
        Err(_) => Err(AuthExtractorError::InternalError(
            "Failed to create token".to_string(),
        )),
    }
}

#[derive(Debug, Error, PartialEq, Clone)]
pub enum AuthExtractorError {
    #[error("Token expired")]
    TokenExpired,
    #[error("Invalid token")]
    InvalidToken,
    #[error("Internal error: {0}")]
    InternalError(String),
}

impl IntoResponse for AuthExtractorError {
    fn into_response(self) -> axum::response::Response {
        let status = match self {
            AuthExtractorError::TokenExpired | AuthExtractorError::InvalidToken => {
                StatusCode::UNAUTHORIZED
            }
            AuthExtractorError::InternalError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        };
        let body = serde_json::json!({
            "error": self.to_string(),
        });
        (status, Json(body)).into_response()
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
    pub email: String,
}

pub struct AuthenticatedUser(pub String);

impl<S> FromRequestParts<S> for AuthenticatedUser
where
    S: Send + Sync,
    Arc<Config>: FromRef<S>,
{
    type Rejection = AuthExtractorError;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let auth_header = parts
            .headers
            .get(AUTHORIZATION)
            .and_then(|value| value.to_str().ok())
            .ok_or(AuthExtractorError::InvalidToken)?;

        let token = auth_header
            .strip_prefix("Bearer ")
            .ok_or(AuthExtractorError::InvalidToken)?;

        let config = Arc::<Config>::from_ref(state);
        let user_id = validate_token(token, &config)?;
        Ok(AuthenticatedUser(user_id))
    }
}

pub fn validate_token(token: &str, config: &Config) -> Result<String, AuthExtractorError> {
    let token_data = jsonwebtoken::decode::<Claims>(
        token,
        &DecodingKey::from_secret(config.get_jwt_secret().as_bytes()),
        &Validation::default(),
    );

    match token_data {
        Ok(data) => Ok(data.claims.sub),
        Err(err) => match err.kind() {
            ErrorKind::ExpiredSignature => Err(AuthExtractorError::TokenExpired),
            ErrorKind::InvalidToken => Err(AuthExtractorError::InvalidToken),
            _ => Err(AuthExtractorError::InternalError(
                "Failed to decode token".to_string(),
            )),
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use uuid::Uuid;

    #[test]
    fn test_create_jwt() {
        let config = Config::test_default();
        let sub = Uuid::new_v4().to_string();
        let email = "test@test.com";

        let token = create_jwt(&sub, email, &config);

        assert!(token.is_ok());
    }

    #[test]
    fn test_validate_token() {
        let config = Config::test_default();
        let sub = Uuid::new_v4().to_string();
        let email = "test@test.com";

        let token = create_jwt(&sub, email, &config).unwrap();
        let result = validate_token(&token, &config);

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), sub);
    }

    #[test]
    fn test_validate_token_invalid() {
        let config = Config::test_default();
        let token = "invalid-token";

        let result = validate_token(token, &config);

        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), AuthExtractorError::InvalidToken);
    }

    #[test]
    fn test_validate_token_expired() {
        let mut config = Config::test_default();
        config.jwt_expiration_time = -3600;
        let sub = Uuid::new_v4().to_string();
        let email = "test@test.com";

        let token = create_jwt(&sub, email, &config).unwrap();
        let result = validate_token(&token, &config);

        assert_eq!(result.unwrap_err(), AuthExtractorError::TokenExpired);
    }

    #[derive(Clone)]
    struct TestState {
        config: Arc<Config>,
    }

    impl FromRef<TestState> for Arc<Config> {
        fn from_ref(state: &TestState) -> Self {
            state.config.clone()
        }
    }

    #[tokio::test]
    async fn test_authenticated_user_extractor_success() {
        let config = Config::test_default();
        let sub = Uuid::new_v4().to_string();
        let email = "test@test.com";
        let token = create_jwt(&sub, email, &config).unwrap();

        let state = TestState {
            config: Arc::new(config),
        };

        let (mut parts, _) = axum::http::Request::builder()
            .header(AUTHORIZATION, format!("Bearer {}", token))
            .body(())
            .unwrap()
            .into_parts();

        let auth_user = AuthenticatedUser::from_request_parts(&mut parts, &state)
            .await
            .unwrap();

        assert_eq!(auth_user.0, sub);
    }

    #[tokio::test]
    async fn test_authenticated_user_extractor_invalid_token() {
        let config = Config::test_default();
        let state = TestState {
            config: Arc::new(config),
        };

        let (mut parts, _) = axum::http::Request::builder()
            .header(AUTHORIZATION, "Bearer invalid-token")
            .body(())
            .unwrap()
            .into_parts();

        let result = AuthenticatedUser::from_request_parts(&mut parts, &state).await;

        assert!(matches!(result, Err(AuthExtractorError::InvalidToken)));
    }

    #[tokio::test]
    async fn test_authenticated_user_extractor_missing_header() {
        let config = Config::test_default();
        let state = TestState {
            config: Arc::new(config),
        };

        let (mut parts, _) = axum::http::Request::builder()
            .body(())
            .unwrap()
            .into_parts();

        let result = AuthenticatedUser::from_request_parts(&mut parts, &state).await;

        assert!(matches!(result, Err(AuthExtractorError::InvalidToken)));
    }

    #[test]
    fn test_auth_extractor_error_response_token_expired() {
        let error = AuthExtractorError::TokenExpired;
        let response = error.into_response();
        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    }

    #[test]
    fn test_auth_extractor_error_invalid_token() {
        let error = AuthExtractorError::InvalidToken;
        let response = error.into_response();
        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    }

    #[test]
    fn test_auth_extractor_error_internal() {
        let error = AuthExtractorError::InternalError("test".to_string());
        let response = error.into_response();
        assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
    }
}
