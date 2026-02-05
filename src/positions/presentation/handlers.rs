use crate::{
    positions::{
        domain::entities::position::PositionUuid,
        presentation::{
            dtos::{PositionResponseDto, PositionUuidDto, SavePositionRequestDto},
            errors::PositionApiError,
            routes::PositionState,
        },
    },
    shared::infrastructure::http::auth_extractor::AuthenticatedUser,
};
use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};

pub async fn get_positions(
    _user: AuthenticatedUser,
    State(state): State<PositionState>,
) -> Result<Json<Vec<PositionResponseDto>>, PositionApiError> {
    let positions = state.service.get_positions().await?;
    let positions_dto = positions.iter().map(PositionResponseDto::from).collect();
    Ok(Json(positions_dto))
}

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

pub async fn save_position(
    _user: AuthenticatedUser,
    State(state): State<PositionState>,
    Json(payload): Json<SavePositionRequestDto>,
) -> Result<Json<PositionUuidDto>, PositionApiError> {
    let position_uuid = state.service.save(payload.to_new_position()?).await?;
    Ok(Json(position_uuid.into()))
}

pub async fn remove_position(
    _user: AuthenticatedUser,
    State(state): State<PositionState>,
    Path(position_id): Path<PositionUuidDto>,
) -> Result<StatusCode, PositionApiError> {
    state.service.remove(position_id.try_into()?).await?;
    Ok(StatusCode::NO_CONTENT)
}
