#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SuccesfullLoginDto {
    pub access_token: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct FailedLoginDto {
    pub message: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct LoginDto {
    pub email: String,
    pub password: String,
}
