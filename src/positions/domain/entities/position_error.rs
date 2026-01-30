use thiserror::Error;

#[derive(Error, Debug)]
pub enum PositionValueError {
    #[error("Wrong uuid format: `{0}`")]
    InvalidUuid(#[from] uuid::Error),

    #[error("Wrong date format: `{0}`")]
    InvalidDate(#[from] chrono::format::ParseError),
}

#[derive(Error, Debug)]
pub enum PositionRepositoryError {
    #[error("Database error: `{0}`")]
    DatabaseError(#[from] sqlx::Error),
}
