use crate::auth::application::errors::AuthError;

pub trait ITokenGenerator: Send + Sync {
    fn generate_token(&self, user_id: &str, email: &str) -> Result<String, AuthError>;
}
