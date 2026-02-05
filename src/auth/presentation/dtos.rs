use utoipa::ToSchema;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, ToSchema)]
pub struct SuccesfullLoginDto {
    pub access_token: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, ToSchema)]
pub struct FailedLoginDto {
    pub message: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, ToSchema)]
pub struct LoginDto {
    pub email: String,
    pub password: String,
}
