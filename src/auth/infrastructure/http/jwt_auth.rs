use chrono::Utc;
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, errors::ErrorKind};
use serde::{Deserialize, Serialize};

use crate::{
    auth::domain::entities::{errors::AuthError, user::User},
    shared::config::Config,
};

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    exp: usize,
}

pub fn create_jwt(user: &User, config: &Config) -> Result<String, AuthError> {
    let experiation = Utc::now().timestamp() + config.jwt_expiration_time;

    let claims = Claims {
        sub: user.id.value().to_string(),
        exp: experiation as usize,
    };

    let token = jsonwebtoken::encode(&Header::default(), &claims, &get_encoding_key(config));

    match token {
        Ok(token) => Ok(token),
        Err(_) => Err(AuthError::InternalServerError),
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
            _ => Err(AuthError::InternalServerError),
        },
    }
}

fn get_encoding_key(config: &Config) -> EncodingKey {
    EncodingKey::from_secret(config.get_jwt_secret().as_bytes())
}

fn get_decoding_key(config: &Config) -> DecodingKey {
    DecodingKey::from_secret(config.get_jwt_secret().as_bytes())
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
}
