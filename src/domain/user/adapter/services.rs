use async_trait::async_trait;

use crate::domain::user::{
    entity::User,
    error::UserError,
    ports::{UserRepository, UserService},
};

pub struct UserServiceImpl<R: UserRepository> {
    repo: R,
}

impl<R: UserRepository> UserServiceImpl<R> {
    pub fn new(repo: R) -> Self {
        Self { repo }
    }
}

#[async_trait]
impl<R: UserRepository> UserService for UserServiceImpl<R> {
    async fn find_by_id(&self, id: &str) -> Result<User, UserError> {
        let get_user = self.repo.find_by_id(id).await;

        match get_user {
            Some(user) => Ok(user),
            None => Err(UserError::NotFound {
                id: String::from(id),
            }),
        }
    }
}
