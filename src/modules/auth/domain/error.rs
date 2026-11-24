use thiserror::Error;

#[derive(Debug, Error)]
pub enum AuthError {
    #[error("Account not found")]
    AccountNotFound,

    #[error("Wrong password")]
    WrongPassword,

    #[error("Account was disabled")]
    AccountDisabled,

    #[error("Internal error")]
    Internal(String),
}
