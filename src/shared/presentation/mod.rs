pub mod openapi;

use serde::Serialize;

#[derive(Serialize)]
pub struct ApiErrorResponse {
    pub message: String,
}
