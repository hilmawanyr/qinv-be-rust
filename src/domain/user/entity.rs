use chrono::{DateTime, Utc};

pub enum UserStatus {
    Active,
    Suspended { reason: String },
}

pub struct User {
    pub id: String,
    pub name: String,
    pub avatar: String,
    pub status: UserStatus,
    pub created_at: DateTime<Utc>,
}
