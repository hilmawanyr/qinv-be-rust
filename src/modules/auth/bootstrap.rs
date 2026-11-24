use std::sync::Arc;

use sqlx::PgPool;

use crate::modules::{
    auth::{
        adapter::repository::pg_auth_repo::AuthRepoImpl, application::auth::AuthServiceImpl,
        port::inbound::AuthService,
    },
    shared::encrypt::EncryptPort,
};

pub fn create_auth_service(db: PgPool, enc: Arc<dyn EncryptPort>) -> Arc<dyn AuthService> {
    let repo = Arc::new(AuthRepoImpl::new(db));
    Arc::new(AuthServiceImpl::new(repo, enc))
}
