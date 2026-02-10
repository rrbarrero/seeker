use utoipa::{
    Modify, OpenApi,
    openapi::security::{HttpAuthScheme, HttpBuilder, SecurityScheme},
};

use crate::auth::presentation::dtos::{LoginDto, SignupDto, SuccesfullLoginDto, UserUuidDto};
use crate::positions::presentation::dtos::{
    CommentResponseDto, CommentUuidDto, PositionResponseDto, PositionUuidDto,
    SaveCommentRequestDto, SavePositionRequestDto, UpdateCommentRequestDto,
    UpdatePositionRequestDto,
};

#[derive(OpenApi)]
#[openapi(
    paths(
        crate::auth::presentation::handlers::login,
        crate::auth::presentation::handlers::signup,
        crate::positions::presentation::handlers::get_positions,
        crate::positions::presentation::handlers::get_position,
        crate::positions::presentation::handlers::save_position,
        crate::positions::presentation::handlers::update_position,
        crate::positions::presentation::handlers::remove_position,
        crate::positions::presentation::comment_handlers::get_comments_for_position,
        crate::positions::presentation::comment_handlers::get_comment,
        crate::positions::presentation::comment_handlers::save_comment,
        crate::positions::presentation::comment_handlers::update_comment,
        crate::positions::presentation::comment_handlers::remove_comment,
    ),
    components(
        schemas(
            LoginDto,
            SignupDto,
            SuccesfullLoginDto,
            UserUuidDto,
            PositionResponseDto,
            PositionUuidDto,
            SavePositionRequestDto,
            UpdatePositionRequestDto,
            CommentResponseDto,
            CommentUuidDto,
            SaveCommentRequestDto,
            UpdateCommentRequestDto
        )
    ),
    modifiers(&SecurityAddon),
    tags(
        (name = "Auth", description = "Authentication endpoints"),
        (name = "Positions", description = "Job positions management"),
        (name = "Comments", description = "Comments for positions")
    )
)]
pub struct ApiDoc;

struct SecurityAddon;

impl Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        if let Some(components) = openapi.components.as_mut() {
            components.add_security_scheme(
                "bearer_auth",
                SecurityScheme::Http(
                    HttpBuilder::new()
                        .scheme(HttpAuthScheme::Bearer)
                        .bearer_format("JWT")
                        .build(),
                ),
            )
        }
    }
}
