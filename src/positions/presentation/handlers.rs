use std::sync::Arc;

use crate::positions::{
    application::position_service::PositionService,
    domain::entities::position::PositionUuid,
    presentation::{
        dtos::{PositionResponseDto, PositionUuidDto, SavePositionRequestDto},
        errors::PositionPresentationError,
    },
};
use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};

pub async fn get_positions(
    State(service): State<Arc<PositionService>>,
) -> Result<Json<Vec<PositionResponseDto>>, PositionPresentationError> {
    let positions = service.get_positions().await?;
    let positions_dto = positions.iter().map(PositionResponseDto::from).collect();
    Ok(Json(positions_dto))
}

pub async fn get_position(
    State(service): State<Arc<PositionService>>,
    Path(position_id): Path<PositionUuidDto>,
) -> Result<Json<PositionResponseDto>, PositionPresentationError> {
    let id: PositionUuid = position_id.try_into()?;
    let position = service.get_position(id).await?;
    match position {
        Some(position) => Ok(Json(PositionResponseDto::from(&position))),
        None => Err(PositionPresentationError::PositionNotFound(id)),
    }
}

pub async fn save_position(
    State(service): State<Arc<PositionService>>,
    Json(payload): Json<SavePositionRequestDto>,
) -> Result<Json<PositionUuidDto>, PositionPresentationError> {
    let position_uuid = service.save(payload.to_new_position()?).await?;
    Ok(Json(position_uuid.into()))
}

pub async fn remove_position(
    State(service): State<Arc<PositionService>>,
    Path(position_id): Path<PositionUuidDto>,
) -> Result<StatusCode, PositionPresentationError> {
    service.remove(position_id.try_into()?).await?;
    Ok(StatusCode::NO_CONTENT)
}
