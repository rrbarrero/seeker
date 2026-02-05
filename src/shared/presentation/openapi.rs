use utoipa::{
    Modify, OpenApi,
    openapi::security::{HttpAuthScheme, HttpBuilder, SecurityScheme},
};

use crate::auth::presentation::dtos::{LoginDto, SuccesfullLoginDto};
use crate::positions::presentation::dtos::{
    PositionResponseDto, PositionUuidDto, SavePositionRequestDto,
};

#[derive(OpenApi)]
#[openapi(
    paths(
        crate::auth::presentation::handlers::login,
        crate::positions::presentation::handlers::get_positions,
        crate::positions::presentation::handlers::get_position,
        crate::positions::presentation::handlers::save_position,
        crate::positions::presentation::handlers::remove_position,
    ),
    components(
        schemas(
            LoginDto,
            SuccesfullLoginDto,
            PositionResponseDto,
            PositionUuidDto,
            SavePositionRequestDto
        )
    ),
    modifiers(&SecurityAddon),
    tags(
        (name = "Auth", description = "Authentication endpoints"),
        (name = "Positions", description = "Job positions management")
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
