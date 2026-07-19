//! User routes (singular) — Revolution Trading Pros
//! Routes for /api/user/* endpoints expected by frontend
//!
//! R26-B (2026-05-20): the original 1,297-LOC `user.rs` was split
//! into this `user/` directory as a PURE STRUCTURAL MOVE. Every
//! handler, SQL statement, error mapping, Stripe call site, tracing
//! field, password-verification flow, single-tx invariant, and money
//! cents conversion (`(price * 100)::BIGINT`) is preserved
//! byte-for-byte. No behavioural change.
//!
//! Endpoints:
//! - GET    /profile             - Get current user profile
//! - PUT    /profile             - Update profile (with password change & email verification)
//! - POST   /avatar              - Upload avatar image
//! - DELETE /avatar              - Remove avatar
//! - POST   /deactivate          - Self-service account deactivation
//! - GET    /memberships         - Get user's active memberships
//! - GET    /memberships/:id     - Get membership details
//! - POST   /memberships/:id/cancel - Cancel membership
//! - GET    /payment-methods     - Get saved payment methods
//! - POST   /payment-methods     - Add payment method
//! - DELETE /payment-methods/:id - Remove payment method
//!
//! Public API contract (re-exported below — consumed by
//! `tests/user_test.rs`):
//!   `UserMembershipResponse`, `MembershipsResponse`,
//!   `CancelSubscriptionRequest`, `PaymentMethodResponse`,
//!   `AddPaymentMethodRequest`.

use axum::{
    routing::{get, post},
    Router,
};

use crate::AppState;

mod account;
mod avatar;
mod dtos;
mod memberships;
mod payment_methods;
mod profile;

// Re-export the DTOs the integration tests import directly. Keeping
// `routes::user::{...}` resolvable preserves the module's public API
// across the R26-B split.
pub use dtos::{
    AddPaymentMethodRequest, CancelSubscriptionRequest, MembershipsResponse, PaymentMethodResponse,
    UserMembershipResponse,
};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/memberships", get(memberships::get_memberships))
        .route("/memberships/{id}", get(memberships::get_membership_details))
        .route(
            "/memberships/{id}/cancel",
            post(memberships::cancel_membership),
        )
        .route("/profile", get(profile::get_profile))
        .route("/profile", axum::routing::put(profile::update_profile))
        .route("/avatar", post(avatar::upload_avatar))
        .route("/avatar", axum::routing::delete(avatar::delete_avatar))
        .route("/deactivate", post(account::deactivate_account))
        .route(
            "/payment-methods",
            get(payment_methods::get_payment_methods),
        )
        .route(
            "/payment-methods",
            post(payment_methods::add_payment_method),
        )
        .route(
            "/payment-methods/{id}",
            axum::routing::delete(payment_methods::delete_payment_method),
        )
}
