use axum::{
    extract::{FromRef, FromRequestParts},
    http::{header::AUTHORIZATION, request::Parts},
};
use chrono::Utc;
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, errors::ErrorKind};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::{
    auth::{application::errors::AuthError, domain::entities::user::User},
    shared::config::Config,
};

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    exp: usize,
    email: String,
}

pub fn create_jwt(user: &User, config: &Config) -> Result<String, AuthError> {
    let expiration = Utc::now().timestamp() + config.jwt_expiration_time;

    let claims = Claims {
        sub: user.id.value().to_string(),
        exp: expiration as usize,
        email: user.email.value().to_string(),
    };

    let token = jsonwebtoken::encode(&Header::default(), &claims, &get_encoding_key(config));

    match token {
        Ok(token) => Ok(token),
        Err(_) => Err(AuthError::InternalError(
            "Failed to create token".to_string(),
        )),
    }
}

pub fn validate_token(token: &str, config: &Config) -> Result<String, AuthError> {
    let token =
        jsonwebtoken::decode::<Claims>(token, &get_decoding_key(config), &Validation::default());

    match token {
        Ok(token) => Ok(token.claims.sub),
        Err(err) => match err.kind() {
            ErrorKind::ExpiredSignature => Err(AuthError::TokenExpired),
            ErrorKind::InvalidToken => Err(AuthError::InvalidToken),
            _ => Err(AuthError::InternalError(
                "Failed to decode token".to_string(),
            )),
        },
    }
}

fn get_encoding_key(config: &Config) -> EncodingKey {
    EncodingKey::from_secret(config.get_jwt_secret().as_bytes())
}

fn get_decoding_key(config: &Config) -> DecodingKey {
    DecodingKey::from_secret(config.get_jwt_secret().as_bytes())
}

pub struct AuthenticatedUser(pub String);

impl<S> FromRequestParts<S> for AuthenticatedUser
where
    S: Send + Sync,
    Arc<Config>: FromRef<S>,
{
    type Rejection = AuthError;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let auth_header = parts
            .headers
            .get(AUTHORIZATION)
            .and_then(|value| value.to_str().ok())
            .ok_or(AuthError::InvalidToken)?;

        let token = auth_header
            .strip_prefix("Bearer ")
            .ok_or(AuthError::InvalidToken)?;

        let config = Arc::<Config>::from_ref(state);
        let user_id = validate_token(token, &config)?;
        Ok(AuthenticatedUser(user_id))
    }
}

#[cfg(test)]
mod tests {
    use uuid::Uuid;

    use super::*;
    use crate::shared::config::Config;

    #[test]
    fn test_create_jwt() {
        let config = Config::test_default();
        let id = Uuid::new_v4();
        let mail = "test@test.com";
        let password = "S0m3V3ryStr0ngP@ssw0rd!";
        let user = User::new(&id.to_string(), mail, password).expect("Error creating testing user");

        let token = create_jwt(&user, &config);

        assert!(token.is_ok());
    }

    #[test]
    fn test_validate_token() {
        let config = Config::test_default();
        let id = Uuid::new_v4();
        let mail = "test@test.com";
        let password = "S0m3V3ryStr0ngP@ssw0rd!";
        let user = User::new(&id.to_string(), mail, password).expect("Error creating testing user");

        let token = create_jwt(&user, &config);

        assert!(token.is_ok());

        let token = validate_token(&token.unwrap(), &config);

        assert!(token.is_ok());
    }

    #[test]
    fn test_validate_token_invalid() {
        let config = Config::test_default();
        let token = "invalid";

        let token = validate_token(token, &config);

        assert!(token.is_err());
    }

    #[test]
    fn test_validate_token_expired() {
        let mut config = Config::test_default();
        config.jwt_expiration_time = -3600;
        let id = Uuid::new_v4();
        let mail = "test@test.com";
        let password = "S0m3V3ryStr0ngP@ssw0rd!";
        let user = User::new(&id.to_string(), mail, password).expect("Error creating testing user");

        let token = create_jwt(&user, &config);

        assert!(token.is_ok());

        let result = validate_token(&token.unwrap(), &config);

        assert_eq!(result.unwrap_err(), AuthError::TokenExpired);
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
        let id = Uuid::new_v4().to_string();
        let user = User::new(&id, "test@test.com", "S0m3V3ryStr0ngP@ssw0rd!").unwrap();
        let token = create_jwt(&user, &config).unwrap();

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

        assert_eq!(auth_user.0, id);
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

        assert!(matches!(result, Err(AuthError::InvalidToken)));
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

        assert!(matches!(result, Err(AuthError::InvalidToken)));
    }
}
