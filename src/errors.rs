use thiserror::Error;

use sqlx::Error as SqlxError;

#[derive(Error, Debug)]
pub enum RepositoryError {
    #[error("Sqlx error: {0}")]
    SqlxError(#[from] SqlxError),

    #[error("Validation Error")]
    ValidationError(&'static str),

    #[error("Entity Not Found: {0}")]
    NotFoundError(&'static str),
}
