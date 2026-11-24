use async_trait::async_trait;
use sqlx::PgPool;

use crate::modules::auth::{
    domain::{
        entity::{Account, AccountStatus},
        error::AuthError,
    },
    port::outbound::AuthRepo,
};

pub struct AuthRepoImpl {
    db: PgPool,
}

impl AuthRepoImpl {
    pub fn new(pool: PgPool) -> Self {
        Self { db: pool }
    }
}

#[async_trait]
impl AuthRepo for AuthRepoImpl {
    async fn find_account(&self, cred: String) -> Result<Option<Account>, AuthError> {
        let row_user = sqlx::query!(
            r#"
                SELECT u.id, p.name, u.username, u.password, u.status FROM users u JOIN profiles p ON u.id = p.user_id WHERE u.username = $1 AND u.status = 'active'
            "#,
            cred,
        )
        .fetch_optional(&self.db)
        .await
        .map_err(|err| {
            tracing::error!("Database error: {:?}", err);
            AuthError::Internal(err.to_string())
        })?;

        let Some(row) = row_user else {
            return Ok(None);
        };

        let status = row.status.parse::<AccountStatus>()?;

        let user = Account {
            id: row.id,
            realname: row.name,
            username: row.username,
            password: row.password,
            status,
        };

        Ok(Some(user))
    }
}
