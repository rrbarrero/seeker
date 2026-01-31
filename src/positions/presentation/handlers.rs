use std::sync::Arc;

use crate::positions::{
    application::position_service::PositionService,
    domain::entities::position::{Position, PositionUuid},
    presentation::{
        dtos::{PositionDto, PositionUuidDto},
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
) -> Result<Json<Vec<PositionDto>>, PositionPresentationError> {
    let positions = service.get_positions().await?;
    let positions_dto = positions
        .iter()
        .map(|position| PositionDto::from(position))
        .collect();
    Ok(Json(positions_dto))
}

pub async fn get_position(
    State(service): State<Arc<PositionService>>,
    Path(position_id): Path<PositionUuidDto>,
) -> Result<Json<PositionDto>, PositionPresentationError> {
    let position = service.get_position(position_id.clone().into()).await?;
    match position {
        Some(position) => Ok(Json(PositionDto::from(&position))),
        None => Err(PositionPresentationError::PositionNotFound(
            position_id.into(),
        )),
    }
}

pub async fn save_position(
    State(service): State<Arc<PositionService>>,
    position: Position,
) -> Result<Json<PositionUuid>, PositionPresentationError> {
    let position_uuid = service.save(position).await?;
    Ok(Json(position_uuid))
}

pub async fn remove_position(
    State(service): State<Arc<PositionService>>,
    Path(position_uuid): Path<PositionUuid>,
) -> Result<StatusCode, PositionPresentationError> {
    service.remove(position_uuid).await?;
    Ok(StatusCode::NO_CONTENT)
}
