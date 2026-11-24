use super::super::domain::error::UserError;
use async_trait::async_trait;

use super::super::domain::entity::User;

#[async_trait]
pub trait UserService: Send + Sync {
    async fn find_by_id(&self, id: &str) -> Result<User, UserError>;
}
