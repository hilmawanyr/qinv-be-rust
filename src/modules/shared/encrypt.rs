use thiserror::Error;

#[derive(Debug, Error)]
pub enum EncryptError {
    #[error("invalid encryption configuration")]
    InvalidConfig,

    #[error("invalid encrypted payload")]
    InvalidPayload,

    #[error("encryption failed")]
    EncryptionFailed,
}

pub trait EncryptPort: Send + Sync {
    fn encrypt(&self, payload: &[u8]) -> Result<String, EncryptError>;
    fn decrypt(&self, value: &str) -> Result<Option<Vec<u8>>, EncryptError>;
}
