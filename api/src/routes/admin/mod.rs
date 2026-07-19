//! Admin routes - Revolution Trading Pros
//! Apple ICT 11+ Principal Engineer Grade - December 2025
//!
//! Enterprise-grade admin API with role-based access control.
//! All routes require admin or super-admin role.
//!
//! R6-B SPLIT (2026-05-20): Originally a 2,466 LOC monolith
//! `routes/admin.rs`. Split into 5 sub-domain files preserving the
//! exact public API (`admin::router()`) byte-for-byte.

#![allow(clippy::type_complexity)]
#![allow(clippy::needless_borrows_for_generic_args)]

// Batch 5b: local `require_admin` and `require_super_admin` helpers
// removed. Admin/super-admin auth is now uniformly enforced via the
// `AdminUser` and `SuperAdminUser` extractors from
// `crate::middleware::admin`. The error shape changed slightly
// (plain text "Admin access required" vs the old JSON body with
// `your_role`); admin clients should treat 403 as "not authorized,"
// not parse the response body.

use axum::{
    routing::{delete, get, post},
    Router,
};

use crate::AppState;

mod campaigns_stats;
mod coupons;
mod memberships;
mod settings;
mod users;

pub fn router() -> Router<AppState> {
    Router::new()
        // Dashboard
        .route("/dashboard", get(campaigns_stats::dashboard_overview))
        // Analytics Dashboard - ICT 7 FIX
        .route(
            "/analytics/dashboard",
            get(campaigns_stats::analytics_dashboard),
        )
        // Posts stats - ICT 7 FIX
        .route("/posts/stats", get(campaigns_stats::posts_stats))
        // Site health - ICT 7 FIX
        .route("/site-health", get(campaigns_stats::site_health))
        // Products stats
        .route("/products/stats", get(campaigns_stats::products_stats))
        // Users
        .route("/users", get(users::list_users).post(users::create_user))
        .route("/users/stats", get(users::user_stats))
        .route(
            "/users/{id}",
            get(users::get_user)
                .put(users::update_user)
                .delete(users::delete_user),
        )
        .route("/users/{id}/ban", post(users::ban_user))
        .route("/users/{id}/unban", post(users::unban_user))
        .route(
            "/users/{id}/memberships",
            get(memberships::get_user_memberships_by_user),
        )
        .route(
            "/users/{id}/subscriptions",
            get(campaigns_stats::get_user_subscriptions),
        )
        // FIX-H-5 (2026-04-29): /users/:id/impersonate route removed
        // along with the handler. See docs/audits/SECURITY_GAPS_2026-04-29.md.
        // Memberships (admin management)
        .route("/membership-plans", get(memberships::list_all_plans))
        .route(
            "/user-memberships",
            get(memberships::list_user_memberships).post(memberships::grant_membership),
        )
        .route(
            "/user-memberships/{id}",
            get(memberships::get_user_membership)
                .put(memberships::update_user_membership)
                .delete(memberships::revoke_membership),
        )
        // Campaigns
        .route(
            "/campaigns",
            get(campaigns_stats::list_campaigns).post(campaigns_stats::create_campaign),
        )
        .route("/campaigns/{id}", delete(campaigns_stats::delete_campaign))
        // Coupons
        // FIX-2026-04-26 (P0-1, CC-1): mount the missing GET/PUT for /coupons/:id
        // so the admin coupon-edit page can load and save against the
        // migration-correct schema. See get_coupon/update_coupon in coupons.rs.
        .route(
            "/coupons",
            get(coupons::list_coupons).post(coupons::create_coupon),
        )
        .route(
            "/coupons/{id}",
            get(coupons::get_coupon)
                .put(coupons::update_coupon)
                .delete(coupons::delete_coupon),
        )
        // Batch 4 P2: backfill mirror for rows whose stripe_coupon_id is NULL
        .route(
            "/coupons/{id}/sync-to-stripe",
            post(coupons::sync_coupon_to_stripe),
        )
        .route("/coupons/validate/{code}", get(coupons::validate_coupon))
        // Batch 6: email diagnostics
        .route("/email/status", get(campaigns_stats::get_email_status))
        .route("/email/logs", get(campaigns_stats::list_email_logs))
        // Settings
        .route("/settings", get(settings::get_settings))
        .route(
            "/settings/{key}",
            get(settings::get_setting).put(settings::update_setting),
        )
}
