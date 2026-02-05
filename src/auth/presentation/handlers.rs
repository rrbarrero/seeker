use std::sync::Arc;

use axum::{Json, extract::State};

use crate::{
    auth::{
        application::auth_service::AuthService,
        presentation::{
            dtos::{LoginDto, SuccesfullLoginDto},
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
