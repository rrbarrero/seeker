use std::sync::Arc;

use axum::{Json, extract::State};

use crate::auth::{
    application::auth_service::AuthService,
    presentation::{
        dtos::{LoginDto, SignupDto, SuccesfullLoginDto, UserUuidDto},
        errors::AuthApiError,
    },
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
    Json(payload): Json<LoginDto>,
) -> Result<Json<SuccesfullLoginDto>, AuthApiError> {
    let access_token = service.login(&payload.email, &payload.password).await?;
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
