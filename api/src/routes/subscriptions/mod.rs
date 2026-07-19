//! Subscription routes — Revolution Trading Pros
//! Apple ICT 11+ Principal Engineer Grade - December 2025
//!
//! R15-B (2026-05-20): the original 1,690-LOC `subscriptions.rs` was split
//! into this `subscriptions/` directory as a pure structural move; every
//! handler, SQL statement, error mapping, Stripe call site, and money
//! invariant is preserved byte-for-byte.
//!
//! Public API contract:
//!  - `pub fn router()` — mounted at `/subscriptions`
//!  - `pub fn my_router()` — mounted at `/my/subscriptions`
//!  - DTOs re-exported below (consumed by `tests/subscriptions_test.rs`):
//!    `MembershipPlanRow`, `UserSubscriptionRow`,
//!    `CreateSubscriptionRequest`, `CancelSubscriptionRequest`,
//!    `ChangePlanRequest`, `ProrationPreview`.

use axum::{
    routing::{get, post},
    Router,
};

use crate::AppState;

mod admin;
mod change_plan;
mod dtos;
mod lifecycle;
mod my_subs;
mod notifications;
mod plans;

// Re-export the DTOs the integration tests import directly. Keeping
// `routes::subscriptions::{MembershipPlanRow, ...}` resolvable preserves
// the module's public API across the R15-B split.
pub use dtos::{
    CancelSubscriptionRequest, ChangePlanRequest, CreateSubscriptionRequest,
    MembershipPlanExtended, MembershipPlanRow, ProrationPreview, UserSubscriptionRow,
    UserSubscriptionWithPlanRow,
};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/plans", get(plans::list_plans))
        .route("/plans/{slug}", get(plans::get_plan))
        .route("/room/{room_slug}/plans", get(plans::get_room_plans))
        .route("/my", get(my_subs::get_my_subscriptions))
        .route("/my/active", get(my_subs::get_active_subscription))
        .route("/export", get(admin::export_subscriptions))
        .route("/", post(lifecycle::create_subscription))
        .route("/{id}/cancel", post(lifecycle::cancel_subscription))
        .route("/{id}/change-plan", post(change_plan::change_plan))
        .route(
            "/{id}/preview-change",
            post(change_plan::preview_plan_change),
        )
        .route("/{id}/reactivate", post(lifecycle::reactivate_subscription))
        .route(
            "/{id}/send-cancellation-email",
            post(notifications::send_cancellation_email),
        )
        .route("/metrics", get(admin::get_metrics))
        // ICT 7 Fix: Notification scheduling endpoints (admin-only, for cron jobs)
        .route(
            "/notifications/renewal-reminders",
            post(notifications::send_renewal_reminders),
        )
        .route(
            "/notifications/trial-ending",
            post(notifications::send_trial_ending_notifications),
        )
}

/// Router for /my/subscriptions path (frontend compatibility)
pub fn my_router() -> Router<AppState> {
    Router::new()
        .route("/", get(my_subs::get_my_subscriptions))
        .route("/{id}", get(my_subs::get_subscription_by_id))
}
