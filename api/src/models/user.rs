//! User model - December 2025 ICT11+ Principal Engineer Grade
//!
//! Implements full authentication contract expected by frontend:
//! - Complete AuthResponse with refresh_token and session_id
//! - Complete UserResponse with role, permissions, email_verified, avatar_url, mfa_enabled

use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

/// User database model
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct User {
    pub id: i64,
    pub email: String,
    #[serde(skip_serializing)]
    #[sqlx(rename = "password")]
    pub password_hash: String,
    pub name: String,
    #[sqlx(default)]
    pub role: Option<String>,
    #[sqlx(default)]
    pub email_verified_at: Option<NaiveDateTime>,
    #[sqlx(default)]
    pub avatar_url: Option<String>,
    #[sqlx(default)]
    pub mfa_enabled: Option<bool>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

/// Request to create a new user
#[derive(Debug, Deserialize)]
pub struct CreateUser {
    pub email: String,
    pub password: String,
    pub name: String,
}

/// Request to login
#[derive(Debug, Deserialize)]
pub struct LoginUser {
    pub email: String,
    pub password: String,
    #[serde(default)]
    pub remember: Option<bool>,
    #[serde(default)]
    pub device_name: Option<String>,
}

/// Request to refresh token
#[derive(Debug, Deserialize)]
pub struct RefreshTokenRequest {
    pub refresh_token: String,
}

/// Request for password reset
#[derive(Debug, Deserialize)]
pub struct ForgotPasswordRequest {
    pub email: String,
}

/// Request to reset password with token
#[derive(Debug, Deserialize)]
pub struct ResetPasswordRequest {
    pub token: String,
    pub email: String,
    pub password: String,
    pub password_confirmation: String,
}

/// Authentication response - matches frontend AuthResponse interface
/// Frontend expects: token, refresh_token, session_id, user, expires_in
#[derive(Debug, Serialize)]
pub struct AuthResponse {
    pub token: String,
    pub refresh_token: String,
    pub session_id: String,
    pub user: UserResponse,
    pub expires_in: i64,  // Token expiration in seconds
}

/// Token refresh response
#[derive(Debug, Serialize)]
pub struct RefreshTokenResponse {
    pub token: String,
    pub refresh_token: String,
    pub expires_in: i64,
}

/// User response - matches frontend User interface
/// Frontend expects: id, email, name, role, email_verified, avatar_url, mfa_enabled, created_at, updated_at
#[derive(Debug, Serialize)]
pub struct UserResponse {
    pub id: i64,
    pub email: String,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    pub role: String,
    pub roles: Vec<String>,
    pub permissions: Vec<String>,
    pub email_verified: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar_url: Option<String>,
    pub mfa_enabled: bool,
    pub is_admin: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

/// Simple message response
#[derive(Debug, Serialize)]
pub struct MessageResponse {
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub success: Option<bool>,
}

impl From<User> for UserResponse {
    fn from(user: User) -> Self {
        let role = user.role.clone().unwrap_or_else(|| "user".to_string());
        let is_admin = role == "admin" || role == "super_admin";
        
        // Parse name into first/last name
        let name_parts: Vec<&str> = user.name.split_whitespace().collect();
        let first_name = name_parts.first().map(|s| s.to_string());
        let last_name = if name_parts.len() > 1 {
            Some(name_parts[1..].join(" "))
        } else {
            None
        };
        
        Self {
            id: user.id,
            email: user.email,
            name: user.name,
            first_name,
            last_name,
            role: role.clone(),
            roles: vec![role.clone()],
            permissions: if is_admin {
                vec![
                    "admin.access".to_string(),
                    "users.manage".to_string(),
                    "posts.manage".to_string(),
                    "products.manage".to_string(),
                ]
            } else {
                vec![]
            },
            email_verified: user.email_verified_at.is_some(),
            avatar_url: user.avatar_url,
            mfa_enabled: user.mfa_enabled.unwrap_or(false),
            is_admin,
            created_at: user.created_at,
            updated_at: user.updated_at,
        }
    }
}
