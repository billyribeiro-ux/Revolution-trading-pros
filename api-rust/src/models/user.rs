//! User model and related types
//!
//! Manual validation instead of validator crate for WASM compatibility

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::error::ApiError;
use crate::utils::{deserialize_datetime, deserialize_option_datetime};

/// User entity
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: Uuid,
    pub email: String,
    pub name: String,
    #[serde(skip_serializing)]
    pub password_hash: String,
    #[serde(default, deserialize_with = "deserialize_option_datetime")]
    pub email_verified_at: Option<DateTime<Utc>>,
    pub role: UserRole,
    pub avatar_url: Option<String>,
    pub mfa_enabled: bool,
    #[serde(skip_serializing)]
    pub mfa_secret: Option<String>,
    #[serde(default, deserialize_with = "deserialize_option_datetime")]
    pub banned_at: Option<DateTime<Utc>>,
    pub ban_reason: Option<String>,
    #[serde(default, deserialize_with = "deserialize_option_datetime")]
    pub last_login_at: Option<DateTime<Utc>>,
    #[serde(deserialize_with = "deserialize_datetime")]
    pub created_at: DateTime<Utc>,
    #[serde(deserialize_with = "deserialize_datetime")]
    pub updated_at: DateTime<Utc>,
}

/// User role enum
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum UserRole {
    User,
    Member,
    Admin,
    SuperAdmin,
}

impl Default for UserRole {
    fn default() -> Self {
        Self::User
    }
}

impl UserRole {
    pub fn is_admin(&self) -> bool {
        matches!(self, Self::Admin | Self::SuperAdmin)
    }

    pub fn is_super_admin(&self) -> bool {
        matches!(self, Self::SuperAdmin)
    }
}

/// Registration request
#[derive(Debug, Deserialize)]
pub struct RegisterRequest {
    pub email: String,
    pub name: String,
    pub password: String,
}

impl RegisterRequest {
    /// Validate the registration request
    pub fn validate(&self) -> Result<(), ApiError> {
        if !is_valid_email(&self.email) {
            return Err(ApiError::Validation("Invalid email address".to_string()));
        }
        if self.name.trim().is_empty() {
            return Err(ApiError::Validation("Name is required".to_string()));
        }
        if self.password.len() < 8 {
            return Err(ApiError::Validation("Password must be at least 8 characters".to_string()));
        }
        Ok(())
    }
}

/// Login request
#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
    pub remember: Option<bool>,
}

impl LoginRequest {
    pub fn validate(&self) -> Result<(), ApiError> {
        if !is_valid_email(&self.email) {
            return Err(ApiError::Validation("Invalid email address".to_string()));
        }
        Ok(())
    }
}

/// MFA login request
#[derive(Debug, Deserialize)]
pub struct MfaLoginRequest {
    pub email: String,
    pub code: String,
    pub remember: Option<bool>,
}

/// Password reset request
#[derive(Debug, Deserialize)]
pub struct ForgotPasswordRequest {
    pub email: String,
}

impl ForgotPasswordRequest {
    pub fn validate(&self) -> Result<(), ApiError> {
        if !is_valid_email(&self.email) {
            return Err(ApiError::Validation("Invalid email address".to_string()));
        }
        Ok(())
    }
}

/// Password reset submission
#[derive(Debug, Deserialize)]
pub struct ResetPasswordRequest {
    pub token: String,
    pub email: String,
    pub password: String,
    pub password_confirmation: String,
}

impl ResetPasswordRequest {
    pub fn validate(&self) -> Result<(), ApiError> {
        if !is_valid_email(&self.email) {
            return Err(ApiError::Validation("Invalid email address".to_string()));
        }
        if self.password.len() < 8 {
            return Err(ApiError::Validation("Password must be at least 8 characters".to_string()));
        }
        if self.password != self.password_confirmation {
            return Err(ApiError::Validation("Passwords do not match".to_string()));
        }
        Ok(())
    }
}

/// Change password request
#[derive(Debug, Deserialize)]
pub struct ChangePasswordRequest {
    pub current_password: String,
    pub new_password: String,
    pub new_password_confirmation: String,
}

impl ChangePasswordRequest {
    pub fn validate(&self) -> Result<(), ApiError> {
        if self.new_password.len() < 8 {
            return Err(ApiError::Validation("Password must be at least 8 characters".to_string()));
        }
        if self.new_password != self.new_password_confirmation {
            return Err(ApiError::Validation("Passwords do not match".to_string()));
        }
        Ok(())
    }
}

/// Simple email validation (basic check)
fn is_valid_email(email: &str) -> bool {
    let email = email.trim();
    if email.is_empty() {
        return false;
    }
    // Basic check: contains @ and has parts before and after
    let parts: Vec<&str> = email.split('@').collect();
    if parts.len() != 2 {
        return false;
    }
    let local = parts[0];
    let domain = parts[1];
    !local.is_empty() && !domain.is_empty() && domain.contains('.')
}

/// Auth response with tokens
#[derive(Debug, Serialize)]
pub struct AuthResponse {
    pub user: UserPublic,
    pub token: String,
    pub refresh_token: Option<String>,
    pub expires_at: DateTime<Utc>,
    pub mfa_required: bool,
}

/// Public user data (safe to expose)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserPublic {
    pub id: Uuid,
    pub email: String,
    pub name: String,
    pub role: UserRole,
    pub avatar_url: Option<String>,
    pub email_verified: bool,
    pub mfa_enabled: bool,
    pub created_at: DateTime<Utc>,
}

impl From<User> for UserPublic {
    fn from(user: User) -> Self {
        Self {
            id: user.id,
            email: user.email,
            name: user.name,
            role: user.role,
            avatar_url: user.avatar_url,
            email_verified: user.email_verified_at.is_some(),
            mfa_enabled: user.mfa_enabled,
            created_at: user.created_at,
        }
    }
}

/// User session
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Session {
    pub id: Uuid,
    pub user_id: Uuid,
    pub token_hash: String,
    pub ip_address: Option<String>,
    pub user_agent: Option<String>,
    pub device_name: Option<String>,
    pub last_active_at: DateTime<Utc>,
    pub expires_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
}

/// Session info for listing
#[derive(Debug, Serialize)]
pub struct SessionInfo {
    pub id: Uuid,
    pub device_name: Option<String>,
    pub ip_address: Option<String>,
    pub last_active_at: DateTime<Utc>,
    pub is_current: bool,
    pub created_at: DateTime<Utc>,
}

/// JWT claims
#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,  // user_id
    pub email: String,
    pub role: UserRole,
    pub iat: i64,
    pub exp: i64,
    pub iss: String,
    pub aud: String,
}

/// Password reset token
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PasswordReset {
    pub id: Uuid,
    pub user_id: Uuid,
    pub token_hash: String,
    pub expires_at: DateTime<Utc>,
    pub used_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}

/// Email verification token
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmailVerification {
    pub id: Uuid,
    pub user_id: Uuid,
    pub token_hash: String,
    pub expires_at: DateTime<Utc>,
    pub verified_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}

/// User membership (product access)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserMembership {
    pub id: Uuid,
    pub user_id: Uuid,
    pub product_id: Uuid,
    pub product_name: String,
    pub product_type: String,
    pub status: MembershipStatus,
    pub starts_at: DateTime<Utc>,
    pub expires_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MembershipStatus {
    Active,
    Expired,
    Cancelled,
    Paused,
}

/// Security event for audit log
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityEvent {
    pub id: Uuid,
    pub user_id: Uuid,
    pub event_type: SecurityEventType,
    pub ip_address: Option<String>,
    pub user_agent: Option<String>,
    pub metadata: Option<serde_json::Value>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SecurityEventType {
    Login,
    LoginFailed,
    Logout,
    PasswordChanged,
    PasswordResetRequested,
    PasswordReset,
    MfaEnabled,
    MfaDisabled,
    SessionRevoked,
    AccountBanned,
    AccountUnbanned,
}
