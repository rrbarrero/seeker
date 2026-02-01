use std::sync::Arc;

use axum::{Json, extract::State};

use crate::{
    auth::{
        application::auth_service::AuthService,
        infrastructure::http::jwt_auth::create_jwt,
        presentation::{
            dtos::{LoginDto, SuccesfullLoginDto},
            error::AuthPresentationError,
        },
    },
    shared::config::Config,
};

pub async fn login(
    State(service): State<Arc<AuthService>>,
    State(config): State<Arc<Config>>,
    Json(payload): Json<LoginDto>,
) -> Result<Json<SuccesfullLoginDto>, AuthPresentationError> {
    let user = service.login(&payload.email, &payload.password).await?;
    let access_token = create_jwt(&user, &config).map_err(AuthPresentationError::AuthError)?;
    Ok(Json(SuccesfullLoginDto { access_token }))
}
