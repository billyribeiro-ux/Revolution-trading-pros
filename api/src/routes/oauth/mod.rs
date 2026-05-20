//! OAuth Routes - January 2026 Apple Principal Engineer ICT Level 7 Grade
//!
//! Implements secure OAuth 2.0 / OpenID Connect flows for:
//! - Google Sign-In (OpenID Connect)
//! - Apple Sign-In (Sign in with Apple)
//!
//! Security Features:
//! - PKCE (Proof Key for Code Exchange) for enhanced security
//! - State parameter for CSRF protection
//! - Nonce validation for Apple ID tokens
//! - Secure token storage (encrypted at rest)
//! - Account linking with existing users
//! - Full audit logging
//!
//! ## Module layout (R28-B split, 2026-05-20)
//!
//! | File           | Role                                                              |
//! |----------------|-------------------------------------------------------------------|
//! | `types.rs`     | Public DTOs (`OAuthProvider`, `OAuthInitResponse`, callback I/O)  |
//! | `crypto.rs`    | PKCE + random state/nonce helpers                                 |
//! | `apple_jwt.rs` | Apple JWKS fetch + ID token signature/nonce validation            |
//! | `user.rs`      | OAuth user create/link/login + token-mint with epoch fail-closed  |
//! | `session.rs`   | Cookie + redirect Response builder (FIX-C-1)                       |
//! | `google.rs`    | Google init + callback handlers                                    |
//! | `apple.rs`     | Apple init + callback handlers                                     |
//!
//! Public surface (pinned by `tests/oauth_test.rs`): `router()`,
//! `OAuthProvider`, `OAuthInitResponse`, `GoogleCallbackQuery`,
//! `AppleCallbackBody`.

mod apple;
mod apple_jwt;
mod crypto;
mod google;
mod session;
mod types;
mod user;

use axum::{
    routing::{get, post},
    Router,
};

use crate::AppState;

pub use types::{AppleCallbackBody, GoogleCallbackQuery, OAuthInitResponse, OAuthProvider};

// =============================================================================
// Router
// =============================================================================

/// Build the OAuth router
pub fn router() -> Router<AppState> {
    Router::new()
        // Google OAuth
        .route("/google", get(google::google_init))
        .route("/google/callback", get(google::google_callback))
        // Apple Sign-In
        .route("/apple", get(apple::apple_init))
        .route("/apple/callback", post(apple::apple_callback))
}
