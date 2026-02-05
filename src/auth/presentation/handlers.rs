use std::sync::Arc;

use axum::{Json, extract::State};

use crate::{
    auth::{
        application::auth_service::AuthService,
        presentation::{
            dtos::{LoginDto, SignupDto, SuccesfullLoginDto, UserUuidDto},
            errors::AuthApiError,
        },
    },
    shared::{config::Config, infrastructure::http::auth_extractor::create_jwt},
};

#[utoipa::path(
    post,
    path = "/auth/login",
    request_body = LoginDto,
    responses(
        (status = 200, description = "Login successful", body = SuccesfullLoginDto),
        (status = 401, description = "Invalid credentials")
    ),
    tag = "Auth"
)]
pub async fn login(
    State(service): State<Arc<AuthService>>,
    State(config): State<Arc<Config>>,
    Json(payload): Json<LoginDto>,
) -> Result<Json<SuccesfullLoginDto>, AuthApiError> {
    let user = service.login(&payload.email, &payload.password).await?;
    let access_token = create_jwt(&user.id.value().to_string(), user.email.value(), &config)
        .map_err(|e| {
            AuthApiError::AuthError(crate::auth::application::errors::AuthError::InternalError(
                e.to_string(),
            ))
        })?;
    Ok(Json(SuccesfullLoginDto { access_token }))
}

#[utoipa::path(
    post,
    path = "/auth/signup",
    request_body = SignupDto,
    responses(
        (status = 201, description = "User created successfully", body = UserUuidDto),
        (status = 400, description = "Invalid input data")
    ),
    tag = "Auth"
)]
pub async fn signup(
    State(service): State<Arc<AuthService>>,
    Json(payload): Json<SignupDto>,
) -> Result<Json<UserUuidDto>, AuthApiError> {
    let user_uuid = service.signup(&payload.email, &payload.password).await?;
    Ok(Json(UserUuidDto {
        user_uuid: user_uuid.to_string(),
    }))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::auth::domain::errors::AuthDomainError;
    use crate::auth::presentation::dtos::{LoginDto, SignupDto};
    use crate::composition_root::create_user_in_memory_repository;
    use crate::shared::config::Config;
    use crate::shared::fixtures::{valid_email, valid_password};
    use std::sync::Arc;

    #[tokio::test]
    async fn test_login_success() -> Result<(), AuthDomainError> {
        let service = Arc::new(AuthService::new(Box::new(
            create_user_in_memory_repository().await,
        )));
        let email = valid_email();
        let password = valid_password();

        let _ = service.signup(email, password).await;

        let config = Arc::new(Config::default());
        let login_dto = LoginDto {
            email: email.to_string(),
            password: password.to_string(),
        };
        let service_state = State(service);
        let config_state = State(config);
        let result = login(service_state, config_state, Json(login_dto)).await;
        assert!(result.is_ok());
        Ok(())
    }

    #[tokio::test]
    async fn test_login_invalid_credentials() -> Result<(), AuthDomainError> {
        let service = Arc::new(AuthService::new(Box::new(
            create_user_in_memory_repository().await,
        )));
        let config = Arc::new(Config::default());
        let login_dto = LoginDto {
            email: "test@example.com".to_string(),
            password: "wrong-password".to_string(),
        };
        let service_state = State(service);
        let config_state = State(config);
        let result = login(service_state, config_state, Json(login_dto)).await;
        assert!(result.is_err());
        Ok(())
    }

    #[tokio::test]
    async fn test_signup_success() -> Result<(), AuthDomainError> {
        let service = Arc::new(AuthService::new(Box::new(
            create_user_in_memory_repository().await,
        )));
        let signup_dto = SignupDto {
            email: valid_email().to_string(),
            password: valid_password().to_string(),
        };

        let service_state = State(service);
        let result = signup(service_state, Json(signup_dto)).await;

        assert!(result.is_ok());
        Ok(())
    }

    #[tokio::test]
    async fn test_signup_invalid_input_data() -> Result<(), AuthDomainError> {
        let service = Arc::new(AuthService::new(Box::new(
            create_user_in_memory_repository().await,
        )));
        let signup_dto = SignupDto {
            email: "invalid-email".to_string(),
            password: "password".to_string(),
        };
        let service_state = State(service);
        let result = signup(service_state, Json(signup_dto)).await;

        assert!(result.is_err());
        Ok(())
    }
}
