//! User Model
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

/// ICT 11+: User model matching PRODUCTION database schema
/// Production DB uses INT8 for id and TIMESTAMP (not TIMESTAMPTZ) for dates
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct User {
    pub id: i64, // Production DB uses INT8, not UUID
    pub name: String,
    pub email: String,
    #[serde(skip_serializing)]
    pub password: String,
    pub role: String,
    pub created_at: NaiveDateTime, // Production uses TIMESTAMP, not TIMESTAMPTZ
    pub updated_at: NaiveDateTime,
    // Optional columns that may or may not exist
    #[sqlx(default)]
    pub first_name: Option<String>,
    #[sqlx(default)]
    pub last_name: Option<String>,
    #[sqlx(default)]
    pub avatar_url: Option<String>,
    #[sqlx(default)]
    pub email_verified_at: Option<NaiveDateTime>,
    #[serde(skip_serializing)]
    #[sqlx(default)]
    pub remember_token: Option<String>,
    #[sqlx(default)]
    pub stripe_customer_id: Option<String>,
}

impl User {
    /// ICT 11+: SQL columns to select - includes email_verified_at for auth checks
    pub const SELECT_COLUMNS: &'static str =
        "id, name, email, password, role, created_at, updated_at, email_verified_at";

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
