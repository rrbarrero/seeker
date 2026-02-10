use std::sync::Arc;

use axum::{
    Json,
    extract::{Query, State},
};

use crate::auth::{
    application::auth_service::AuthService,
    presentation::{
        dtos::{LoginDto, SignupDto, SuccesfullLoginDto, UserUuidDto, VerifyEmailQuery},
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
    let (access_token, email_validated) = service.login(&payload.email, &payload.password).await?;
    Ok(Json(SuccesfullLoginDto {
        access_token,
        email_validated,
    }))
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

#[utoipa::path(
    get,
    path = "/auth/verify-email",
    params(
        ("token" = String, Query, description = "Email verification token")
    ),
    responses(
        (status = 200, description = "Email verified successfully"),
        (status = 401, description = "Invalid or expired token"),
        (status = 404, description = "User not found")
    ),
    tag = "Auth"
)]
pub async fn verify_email(
    State(service): State<Arc<AuthService>>,
    Query(params): Query<VerifyEmailQuery>,
) -> Result<Json<serde_json::Value>, AuthApiError> {
    service.verify_email(&params.token).await?;
    Ok(Json(
        serde_json::json!({ "message": "Email verified successfully" }),
    ))
}
