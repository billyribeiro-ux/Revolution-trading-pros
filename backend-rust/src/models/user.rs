//! User Model
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct User {
    pub id: Uuid,
    pub name: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub email: String,
    #[serde(skip_serializing)]
    pub password: String,
    pub avatar_url: Option<String>,
    pub role: String,
    pub email_verified_at: Option<DateTime<Utc>>,
    #[serde(skip_serializing)]
    pub remember_token: Option<String>,
    pub stripe_customer_id: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl User {
    /// ICT 11+: SQL columns to select (excludes deleted_at which may not exist in DB)
    pub const SELECT_COLUMNS: &'static str = "id, name, first_name, last_name, email, password, avatar_url, role, email_verified_at, remember_token, stripe_customer_id, created_at, updated_at";

    pub fn is_admin(&self) -> bool {
        self.role == "admin" || self.role == "super-admin"
    }

    pub fn is_verified(&self) -> bool {
        self.email_verified_at.is_some()
    }
}

#[derive(Debug, Clone)]
pub struct CreateUser {
    pub name: String,
    pub email: String,
    pub password: String,
}

#[derive(Debug, Clone, Default)]
pub struct UpdateUser {
    pub name: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub email: Option<String>,
    pub password: Option<String>,
    pub avatar_url: Option<String>,
}
