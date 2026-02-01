use crate::positions::domain::errors::{PositionDomainError, PositionRepoError};
use thiserror::Error;

#[derive(Error, Debug, PartialEq, Clone)]
pub enum PositionServiceError {
    #[error("Domain error: `{0}`")]
    DomainError(#[from] PositionDomainError),

    #[error("Repository error: `{0}`")]
    RepositoryError(#[from] PositionRepoError),

    #[error("Internal error: `{0}`")]
    InternalError(String),
}
