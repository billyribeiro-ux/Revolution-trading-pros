//! User Resource
use serde::Serialize;

use crate::models::User;

/// ICT 11+: Production DB uses INT8 for user IDs
#[derive(Debug, Serialize)]
pub struct UserResource {
    pub id: i64,
    pub name: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub email: String,
    pub avatar_url: Option<String>,
    pub role: String,
    pub is_admin: bool,
    pub is_verified: bool,
    pub created_at: String,
    pub updated_at: String,
}

impl From<User> for UserResource {
    fn from(user: User) -> Self {
        // Call methods first before moving fields
        let is_admin = user.is_admin();
        let is_verified = user.is_verified();
        let created_at = user.created_at.format("%Y-%m-%dT%H:%M:%S").to_string();
        let updated_at = user.updated_at.format("%Y-%m-%dT%H:%M:%S").to_string();

        Self {
            id: user.id,
            name: user.name,
            first_name: user.first_name,
            last_name: user.last_name,
            email: user.email,
            avatar_url: user.avatar_url,
            role: user.role,
            is_admin,
            is_verified,
            created_at,
            updated_at,
        }
    }
}
