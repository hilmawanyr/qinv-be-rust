use async_trait::async_trait;

use crate::modules::auth::domain::{entity::Credential, error::AuthError};

#[async_trait]
pub trait AuthService: Send + Sync {
    async fn sign_in(&self, cred: Credential) -> Result<String, AuthError>;
}
