use async_trait::async_trait;

use crate::modules::auth::domain::{entity::Account, error::AuthError};

#[async_trait]
pub trait AuthRepo: Send + Sync {
    async fn find_account(&self, username: String) -> Result<Option<Account>, AuthError>;
}
