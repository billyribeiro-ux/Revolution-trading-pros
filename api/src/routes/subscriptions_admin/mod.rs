//! Subscriptions Admin Routes - Revolution Trading Pros
//! Apple ICT 7+ Principal Engineer Grade - January 2026
//!
//! Full admin CRUD for subscriptions and subscription plans.
//!
//! ── Split layout (R28-B2) ────────────────────────────────────────────
//! Pre-split monolith was `routes/subscriptions_admin.rs` (1,165 LOC).
//! Sub-modules:
//!   - `dto` — request DTOs + SQL row types (10 pub types)
//!   - `subscriptions` — list / get / create / update / delete
//!   - `subscription_lifecycle` — cancel / pause / resume / renew
//!   - `plans` — list_plans / get_plan / create_plan / update_plan /
//!     delete_plan / plan_stats
//!   - `price_changes` — change_plan_price + list_plan_price_history
//!     (Stripe-syncing money path; preserves BEGIN/COMMIT, audit log
//!     ordering, i64 cents)
//!
//! Public surface preserved: `subscriptions_router()`, `plans_router()`,
//! and the 10 re-exported DTOs (`ChangePriceRequest`, `PriceHistoryRow`,
//! `SubscriptionRow`, `SubscriptionPlanRow`, `SubscriptionListQuery`,
//! `PlanListQuery`, `CreateSubscriptionRequest`,
//! `UpdateSubscriptionRequest`, `CreatePlanRequest`, `UpdatePlanRequest`)
//! so `tests/subscriptions_admin_test.rs` continues to resolve the
//! `revolution_api::routes::subscriptions_admin::{...}` paths byte-for-byte.

use axum::{
    routing::{get, post},
    Router,
};

use crate::AppState;

mod dto;
mod plans;
mod price_changes;
mod subscription_lifecycle;
mod subscriptions;

// Re-export DTOs so integration tests can address them at
// `revolution_api::routes::subscriptions_admin::{...}` exactly as
// before the split.
pub use dto::{
    ChangePriceRequest, CreatePlanRequest, CreateSubscriptionRequest, PlanListQuery,
    PriceHistoryRow, SubscriptionListQuery, SubscriptionPlanRow, SubscriptionRow,
    UpdatePlanRequest, UpdateSubscriptionRequest,
};

/// Subscriptions admin router - mounted at /admin/subscriptions
pub fn subscriptions_router() -> Router<AppState> {
    Router::new()
        .route(
            "/",
            get(subscriptions::list_subscriptions).post(subscriptions::create_subscription),
        )
        .route(
            "/{id}",
            get(subscriptions::get_subscription)
                .put(subscriptions::update_subscription)
                .delete(subscriptions::delete_subscription),
        )
        .route(
            "/{id}/cancel",
            post(subscription_lifecycle::cancel_subscription),
        )
        .route(
            "/{id}/pause",
            post(subscription_lifecycle::pause_subscription),
        )
        .route(
            "/{id}/resume",
            post(subscription_lifecycle::resume_subscription),
        )
        .route(
            "/{id}/renew",
            post(subscription_lifecycle::renew_subscription),
        )
}

/// Subscription plans admin router - mounted at /admin/subscriptions/plans
pub fn plans_router() -> Router<AppState> {
    Router::new()
        .route("/", get(plans::list_plans).post(plans::create_plan))
        .route("/stats", get(plans::plan_stats))
        .route(
            "/{id}",
            get(plans::get_plan)
                .put(plans::update_plan)
                .delete(plans::delete_plan),
        )
        // Stripe-syncing price-change endpoints (admin only)
        .route("/{id}/price", post(price_changes::change_plan_price))
        .route(
            "/{id}/price-history",
            get(price_changes::list_plan_price_history),
        )
}
