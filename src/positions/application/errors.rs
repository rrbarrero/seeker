use crate::positions::domain::errors::{
    CommentDomainError, CommentRepoError, PositionDomainError, PositionRepoError,
};
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

#[derive(Error, Debug, PartialEq, Clone)]
pub enum CommentServiceError {
    #[error("Domain error: `{0}`")]
    DomainError(#[from] CommentDomainError),

    #[error("Repository error: `{0}`")]
    RepositoryError(#[from] CommentRepoError),

    #[error("Internal error: `{0}`")]
    InternalError(String),
}
