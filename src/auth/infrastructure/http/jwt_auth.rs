use chrono::Utc;
use jsonwebtoken::Header;
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
    let experiation = Utc::now().timestamp() + 60 * 60 * 24 * 7;

    let claims = Claims {
        sub: user.id.value().to_string(),
        exp: experiation as usize,
    };

    let token = jsonwebtoken::encode(&Header::default(), &claims, &config.jwt_secret());

    match token {
        Ok(token) => Ok(token),
        Err(_) => Err(AuthError::InternalServerError),
    }
}

#[cfg(test)]
mod tests {
    use uuid::Uuid;

    use super::*;
    use crate::shared::config::Config;

    #[test]
    fn test_create_jwt() {
        let config = Config::default();
        let id = Uuid::new_v4();
        let mail = "test@test.com";
        let password = "S0m3V3ryStr0ngP@ssw0rd!";
        let user = User::new(&id.to_string(), mail, password).expect("Error creating testing user");

        let token = create_jwt(&user, &config);

        assert!(token.is_ok());
    }
}
