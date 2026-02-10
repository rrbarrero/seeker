use crate::auth::application::{errors::AuthError, token_generator::ITokenGenerator};
use crate::shared::config::Config;
use crate::shared::infrastructure::http::auth_extractor::{create_jwt, validate_token};
use std::sync::Arc;

pub struct JwtTokenGenerator {
    config: Arc<Config>,
}

impl JwtTokenGenerator {
    pub fn new(config: Arc<Config>) -> Self {
        Self { config }
    }
}

impl ITokenGenerator for JwtTokenGenerator {
    fn generate_token(&self, user_id: &str, email: &str) -> Result<String, AuthError> {
        create_jwt(user_id, email, &self.config)
            .map_err(|e| AuthError::InternalError(e.to_string()))
    }

    fn validate_token(&self, token: &str) -> Result<String, AuthError> {
        validate_token(token, &self.config).map_err(|e| match e {
            crate::shared::infrastructure::http::auth_extractor::AuthExtractorError::TokenExpired => {
                AuthError::TokenExpired
            }
            _ => AuthError::InvalidToken,
        })
    }
}
