use std::sync::Arc;

use axum::{Json, extract::State};

use crate::{
    auth::{
        application::auth_service::AuthService,
        infrastructure::http::jwt_auth::create_jwt,
        presentation::{
            dtos::{LoginDto, SuccesfullLoginDto},
            errors::AuthApiError,
        },
    },
    shared::config::Config,
};

pub async fn login(
    State(service): State<Arc<AuthService>>,
    State(config): State<Arc<Config>>,
    Json(payload): Json<LoginDto>,
) -> Result<Json<SuccesfullLoginDto>, AuthApiError> {
    let user = service.login(&payload.email, &payload.password).await?;
    let access_token = create_jwt(&user, &config).map_err(AuthApiError::AuthError)?;
    Ok(Json(SuccesfullLoginDto { access_token }))
}
