use thiserror::Error;

#[derive(Debug, Error)]
pub enum DbErrors {
    #[error("Connection timeout")]
    ConnTimeOut,

    #[error("Access denied")]
    AccessDenied,

    #[error("DB internal error")]
    Internal,
}
