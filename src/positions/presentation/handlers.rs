use crate::{
    positions::{
        domain::entities::position::PositionUuid,
        presentation::{
            dtos::{PositionResponseDto, PositionUuidDto, SavePositionRequestDto},
            errors::PositionApiError,
            routes::PositionState,
        },
    },
    shared::{
        domain::value_objects::UserUuid, infrastructure::http::auth_extractor::AuthenticatedUser,
    },
};
use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};
use std::str::FromStr;

#[utoipa::path(
    get,
    path = "/positions",
    responses(
        (status = 200, description = "List all positions", body = [PositionResponseDto]),
        (status = 401, description = "Unauthorized")
    ),
    security(
        ("bearer_auth" = [])
    ),
    tag = "Positions"
)]
pub async fn get_positions(
    _user: AuthenticatedUser,
    State(state): State<PositionState>,
) -> Result<Json<Vec<PositionResponseDto>>, PositionApiError> {
    let positions = state.service.get_positions().await?;
    let positions_dto = positions.iter().map(PositionResponseDto::from).collect();
    Ok(Json(positions_dto))
}

#[utoipa::path(
    get,
    path = "/positions/{id}",
    params(
        ("id" = String, Path, description = "Position ID")
    ),
    responses(
        (status = 200, description = "Position found", body = PositionResponseDto),
        (status = 404, description = "Position not found"),
        (status = 401, description = "Unauthorized")
    ),
    security(
        ("bearer_auth" = [])
    ),
    tag = "Positions"
)]
pub async fn get_position(
    _user: AuthenticatedUser,
    State(state): State<PositionState>,
    Path(position_id): Path<PositionUuidDto>,
) -> Result<Json<PositionResponseDto>, PositionApiError> {
    let id: PositionUuid = position_id.try_into()?;
    let position = state.service.get_position(id).await?;
    match position {
        Some(position) => Ok(Json(PositionResponseDto::from(&position))),
        None => Err(PositionApiError::PositionNotFound(id)),
    }
}

#[utoipa::path(
    post,
    path = "/positions",
    request_body = SavePositionRequestDto,
    responses(
        (status = 200, description = "Position saved", body = PositionResponseDto),
        (status = 401, description = "Unauthorized")
    ),
    security(
        ("bearer_auth" = [])
    ),
    tag = "Positions"
)]
pub async fn save_position(
    user: AuthenticatedUser,
    State(state): State<PositionState>,
    Json(payload): Json<SavePositionRequestDto>,
) -> Result<Json<PositionResponseDto>, PositionApiError> {
    let user_id = UserUuid::from_str(&user.0)?;
    let position = payload.to_new_position(user_id)?;
    state.service.save(position.clone()).await?;
    Ok(Json(PositionResponseDto::from(&position)))
}

#[utoipa::path(
    delete,
    path = "/positions/{id}",
    params(
        ("id" = String, Path, description = "Position ID")
    ),
    responses(
        (status = 204, description = "Position removed"),
        (status = 404, description = "Position not found"),
        (status = 401, description = "Unauthorized")
    ),
    security(
        ("bearer_auth" = [])
    ),
    tag = "Positions"
)]
pub async fn remove_position(
    _user: AuthenticatedUser,
    State(state): State<PositionState>,
    Path(position_id): Path<PositionUuidDto>,
) -> Result<StatusCode, PositionApiError> {
    state.service.remove(position_id.try_into()?).await?;
    Ok(StatusCode::NO_CONTENT)
}
