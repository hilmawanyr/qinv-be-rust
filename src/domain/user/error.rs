use thiserror::Error;

#[derive(Debug, Error)]
pub enum UserError {
    #[error("User {id} not found")]
    NotFound { id: String },

    #[error("User {id} suspended")]
    Suspended { id: String },
}
