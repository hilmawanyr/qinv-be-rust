use std::str;

use uuid::Uuid;

use crate::modules::auth::domain::error::AuthError;

pub enum AccountStatus {
    Active,
    Inactive,
    Disabled,
}

impl str::FromStr for AccountStatus {
    type Err = AuthError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "active" => Ok(Self::Active),
            "inactive" => Ok(Self::Inactive),
            "disabled" => Ok(Self::Disabled),
            err => Err(AuthError::Internal(err.to_string())),
        }
    }
}

pub struct Credential {
    pub username: String,
    pub password: String,
}

pub struct Account {
    pub id: Uuid,
    pub realname: String,
    pub username: String,
    pub password: String,
    pub status: AccountStatus,
}
