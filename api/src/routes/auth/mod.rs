//! Authentication routes - December 2025 ICT11+ Principal Engineer Grade
//!
//! Implements complete authentication contract for frontend:
//! - POST /register - Create new user account (requires email verification)
//! - POST /login - Authenticate user (requires verified email)
//! - POST /refresh - Refresh access token
//! - POST /logout - Invalidate session
//! - GET /me - Get current user
//! - POST /forgot-password - Request password reset
//! - POST /reset-password - Reset password with token
//! - GET /verify-email - Verify email with token
//! - POST /resend-verification - Resend verification email
//!
//! R12-B (2026-05-20): the original 1,827-LOC `auth.rs` was split into this
//! `auth/` directory as a pure structural move; every handler, log target,
//! audit event, and rate-limit gate is preserved byte-for-byte.

use axum::{
    routing::{get, post},
    Router,
};

use crate::AppState;

mod helpers;
mod login;
mod password;
mod refresh;
mod register;
mod session;
mod verify;

// P1-3 (FULL_REPO_AUDIT_2026-05-17): re-export the spoof-resistant client-IP
// resolver so external consumers (notably `tests/client_ip_test.rs`) keep
// resolving `routes::auth::resolve_client_ip` at the same path.
pub use helpers::resolve_client_ip;

/// Build the auth router
pub fn router() -> Router<AppState> {
    Router::new()
        .route("/register", post(register::register))
        .route("/login", post(login::login))
        .route("/refresh", post(refresh::refresh))
        .route("/me", get(session::me))
        .route("/logout", post(session::logout))
        .route("/logout-all", post(session::logout_all)) // ICT L11+ Security: Force logout everywhere
        .route("/forgot-password", post(password::forgot_password))
        .route("/reset-password", post(password::reset_password))
        .route("/verify-email", get(verify::verify_email))
        .route("/resend-verification", post(verify::resend_verification))
}

/// Router for /logout at top level (frontend compatibility)
pub fn logout_router() -> Router<AppState> {
    Router::new().route("/", post(session::logout))
}
