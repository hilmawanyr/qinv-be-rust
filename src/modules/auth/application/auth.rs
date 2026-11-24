use std::sync::Arc;

use async_trait::async_trait;
use bcrypt::verify;
use serde::{Deserialize, Serialize};

use crate::modules::{
    auth::{
        domain::{entity::Credential, error::AuthError},
        port::{inbound::AuthService, outbound::AuthRepo},
    },
    shared::encrypt::EncryptPort,
};

pub struct AuthServiceImpl {
    repo: Arc<dyn AuthRepo>,
    encrypt: Arc<dyn EncryptPort>,
}

impl AuthServiceImpl {
    pub fn new(r: Arc<dyn AuthRepo>, e: Arc<dyn EncryptPort>) -> Self {
        Self {
            repo: r,
            encrypt: e,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct TokenPayload {
    user_id: String,
    user_name: String,
}

#[async_trait]
impl AuthService for AuthServiceImpl {
    async fn sign_in(&self, c: Credential) -> Result<String, AuthError> {
        let find_user = self
            .repo
            .find_account(c.username)
            .await
            .map_err(|err| {
                tracing::error!("Repository error: {:?}", err);
                AuthError::Internal(err.to_string())
            })?
            .ok_or(AuthError::AccountNotFound)?;

        let is_valid = verify(c.password, &find_user.password)
            .map_err(|err| AuthError::Internal(err.to_string()))?;

        if !is_valid {
            return Err(AuthError::WrongPassword);
        }

        let token_payload = TokenPayload {
            user_id: String::from(find_user.id),
            user_name: String::from(find_user.realname),
        };

        let to_byte = serde_json::to_vec(&token_payload)
            .map_err(|err| AuthError::Internal(err.to_string()))?;

        let token = self
            .encrypt
            .encrypt(&to_byte)
            .map_err(|err| AuthError::Internal(err.to_string()))?;

        Ok(token)
    }
}
