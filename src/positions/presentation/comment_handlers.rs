use std::str::FromStr;

use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};
use uuid::Uuid;

use crate::{
    positions::{
        domain::entities::comment::CommentUuid,
        domain::entities::position::{Position, PositionUuid},
        presentation::{
            dtos::{CommentResponseDto, SaveCommentRequestDto, UpdateCommentRequestDto},
            errors::CommentApiError,
            routes::PositionState,
        },
    },
    shared::{
        domain::{errors::SharedDomainError, value_objects::UserUuid},
        infrastructure::http::auth_extractor::AuthenticatedUser,
    },
};

fn parse_position_id(position_id: &str) -> Result<PositionUuid, CommentApiError> {
    let id = Uuid::parse_str(position_id).map_err(SharedDomainError::from)?;
    Ok(PositionUuid::from_uuid(id))
}

/// Validates that the authenticated user owns the position.
/// Returns the position if the user is the owner, or an appropriate error otherwise.
async fn assert_position_owner(
    state: &PositionState,
    position_id: PositionUuid,
    user_id: &UserUuid,
) -> Result<Position, CommentApiError> {
    let position = state.service.get_position(position_id).await?;

    match position {
        Some(p) if p.user_id == *user_id => Ok(p),
        Some(_) => Err(CommentApiError::Forbidden),
        None => Err(CommentApiError::PositionNotFound(position_id)),
    }
}

#[utoipa::path(
    get,
    path = "/positions/{position_id}/comments",
    params(
        ("position_id" = String, Path, description = "Position ID")
    ),
    responses(
        (status = 200, description = "List comments for a position", body = [CommentResponseDto]),
        (status = 403, description = "Forbidden"),
        (status = 404, description = "Position not found"),
        (status = 401, description = "Unauthorized")
    ),
    security(
        ("bearer_auth" = [])
    ),
    tag = "Comments"
)]
pub async fn get_comments_for_position(
    user: AuthenticatedUser,
    State(state): State<PositionState>,
    Path(position_id): Path<String>,
) -> Result<Json<Vec<CommentResponseDto>>, CommentApiError> {
    let user_id = UserUuid::from_str(&user.0)?;
    let position_id = parse_position_id(&position_id)?;
    assert_position_owner(&state, position_id, &user_id).await?;

    let comments = state
        .comment_service
        .get_comments_for_position(position_id)
        .await?;
    let comments_dto = comments.iter().map(CommentResponseDto::from).collect();
    Ok(Json(comments_dto))
}

#[utoipa::path(
    get,
    path = "/positions/{position_id}/comments/{comment_id}",
    params(
        ("position_id" = String, Path, description = "Position ID"),
        ("comment_id" = String, Path, description = "Comment ID")
    ),
    responses(
        (status = 200, description = "Comment found", body = CommentResponseDto),
        (status = 404, description = "Comment not found"),
        (status = 403, description = "Forbidden"),
        (status = 401, description = "Unauthorized")
    ),
    security(
        ("bearer_auth" = [])
    ),
    tag = "Comments"
)]
pub async fn get_comment(
    user: AuthenticatedUser,
    State(state): State<PositionState>,
    Path((position_id, comment_id)): Path<(String, String)>,
) -> Result<Json<CommentResponseDto>, CommentApiError> {
    let user_id = UserUuid::from_str(&user.0)?;
    let position_id = parse_position_id(&position_id)?;
    assert_position_owner(&state, position_id, &user_id).await?;

    let comment_id = CommentUuid::from_str(&comment_id)?;
    let comment = state.comment_service.get_comment(comment_id).await?;
    match comment {
        Some(comment) if comment.position_id == position_id => {
            Ok(Json(CommentResponseDto::from(&comment)))
        }
        _ => Err(CommentApiError::CommentNotFound(comment_id)),
    }
}

#[utoipa::path(
    post,
    path = "/positions/{position_id}/comments",
    params(
        ("position_id" = String, Path, description = "Position ID")
    ),
    request_body = SaveCommentRequestDto,
    responses(
        (status = 201, description = "Comment saved", body = CommentResponseDto),
        (status = 403, description = "Forbidden"),
        (status = 404, description = "Position not found"),
        (status = 401, description = "Unauthorized")
    ),
    security(
        ("bearer_auth" = [])
    ),
    tag = "Comments"
)]
pub async fn save_comment(
    user: AuthenticatedUser,
    State(state): State<PositionState>,
    Path(position_id): Path<String>,
    Json(payload): Json<SaveCommentRequestDto>,
) -> Result<(StatusCode, Json<CommentResponseDto>), CommentApiError> {
    let user_id = UserUuid::from_str(&user.0)?;
    let position_id = parse_position_id(&position_id)?;
    assert_position_owner(&state, position_id, &user_id).await?;

    let comment = payload.to_new_comment(user_id, position_id)?;
    state.comment_service.save(comment.clone()).await?;
    Ok((
        StatusCode::CREATED,
        Json(CommentResponseDto::from(&comment)),
    ))
}

#[utoipa::path(
    put,
    path = "/positions/{position_id}/comments/{comment_id}",
    params(
        ("position_id" = String, Path, description = "Position ID"),
        ("comment_id" = String, Path, description = "Comment ID")
    ),
    request_body = UpdateCommentRequestDto,
    responses(
        (status = 200, description = "Comment updated", body = CommentResponseDto),
        (status = 404, description = "Comment not found"),
        (status = 403, description = "Forbidden"),
        (status = 401, description = "Unauthorized")
    ),
    security(
        ("bearer_auth" = [])
    ),
    tag = "Comments"
)]
pub async fn update_comment(
    user: AuthenticatedUser,
    State(state): State<PositionState>,
    Path((position_id, comment_id)): Path<(String, String)>,
    Json(payload): Json<UpdateCommentRequestDto>,
) -> Result<Json<CommentResponseDto>, CommentApiError> {
    let user_id = UserUuid::from_str(&user.0)?;
    let position_id = parse_position_id(&position_id)?;
    assert_position_owner(&state, position_id, &user_id).await?;

    let comment_id = CommentUuid::from_str(&comment_id)?;
    let existing = state.comment_service.get_comment(comment_id).await?;
    let Some(existing) = existing else {
        return Err(CommentApiError::CommentNotFound(comment_id));
    };

    if existing.position_id != position_id {
        return Err(CommentApiError::CommentNotFound(comment_id));
    }

    let updated = payload.to_updated_comment(existing)?;
    state.comment_service.update(updated.clone()).await?;
    Ok(Json(CommentResponseDto::from(&updated)))
}

#[utoipa::path(
    delete,
    path = "/positions/{position_id}/comments/{comment_id}",
    params(
        ("position_id" = String, Path, description = "Position ID"),
        ("comment_id" = String, Path, description = "Comment ID")
    ),
    responses(
        (status = 204, description = "Comment removed"),
        (status = 404, description = "Comment not found"),
        (status = 403, description = "Forbidden"),
        (status = 401, description = "Unauthorized")
    ),
    security(
        ("bearer_auth" = [])
    ),
    tag = "Comments"
)]
pub async fn remove_comment(
    user: AuthenticatedUser,
    State(state): State<PositionState>,
    Path((position_id, comment_id)): Path<(String, String)>,
) -> Result<StatusCode, CommentApiError> {
    let user_id = UserUuid::from_str(&user.0)?;
    let position_id = parse_position_id(&position_id)?;
    assert_position_owner(&state, position_id, &user_id).await?;

    let comment_id = CommentUuid::from_str(&comment_id)?;
    let existing = state.comment_service.get_comment(comment_id).await?;
    let Some(existing) = existing else {
        return Err(CommentApiError::CommentNotFound(comment_id));
    };

    if existing.position_id != position_id {
        return Err(CommentApiError::CommentNotFound(comment_id));
    }

    state.comment_service.remove(comment_id).await?;
    Ok(StatusCode::NO_CONTENT)
}
