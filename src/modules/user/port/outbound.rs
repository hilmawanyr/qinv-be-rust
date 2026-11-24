use async_trait::async_trait;

use super::super::domain::entity::User;

#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn find_by_id(&self, id: &str) -> Option<User>;
}
