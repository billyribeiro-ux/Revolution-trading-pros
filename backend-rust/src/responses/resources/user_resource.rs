//! User Resource
use serde::Serialize;
use uuid::Uuid;

use crate::models::User;

#[derive(Debug, Serialize)]
pub struct UserResource {
    pub id: Uuid,
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
        Self {
            id: user.id,
            name: user.name,
            first_name: user.first_name,
            last_name: user.last_name,
            email: user.email,
            avatar_url: user.avatar_url,
            role: user.role.clone(),
            is_admin: user.is_admin(),
            is_verified: user.is_verified(),
            created_at: user.created_at.to_rfc3339(),
            updated_at: user.updated_at.to_rfc3339(),
        }
    }
}
