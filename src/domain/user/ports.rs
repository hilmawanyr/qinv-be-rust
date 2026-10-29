use super::error::UserError;
use async_trait::async_trait;

use super::entity::User;

#[async_trait]
pub trait UserService: Send + Sync {
    async fn find_by_id(&self, id: &str) -> Result<User, UserError>;
}

#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn find_by_id(&self, id: &str) -> Option<User>;
}
