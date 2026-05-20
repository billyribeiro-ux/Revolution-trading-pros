//! User route DTOs - Revolution Trading Pros
//!
//! R26-B (2026-05-20): the original 1,297-LOC `user.rs` was split
//! into this `user/` directory as a PURE STRUCTURAL MOVE. The DTOs
//! below are re-exported from `routes::user` so `tests/user_test.rs`
//! and any downstream consumers continue to resolve them at the same
//! path. Money fields remain `Option<i64>` cents end-to-end per the
//! "Money / cents — i64 ONLY, BIGINT ONLY" hard rule.
//!
//! Public types (re-exported by `user/mod.rs`):
//!   - `UserMembershipResponse`
//!   - `MembershipsResponse`
//!   - `CancelSubscriptionRequest`
//!   - `PaymentMethodResponse`
//!   - `AddPaymentMethodRequest`

use serde::{Deserialize, Serialize};

/// User membership response - matches frontend UserMembership interface
#[derive(Debug, Serialize, Deserialize)]
pub struct UserMembershipResponse {
    pub id: String,
    pub name: String,
    #[serde(rename = "type")]
    pub membership_type: String,
    pub slug: String,
    pub status: String,
    #[serde(rename = "membershipType", skip_serializing_if = "Option::is_none")]
    pub subscription_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon: Option<String>,
    #[serde(rename = "startDate")]
    pub start_date: String,
    #[serde(rename = "nextBillingDate", skip_serializing_if = "Option::is_none")]
    pub next_billing_date: Option<String>,
    #[serde(rename = "expiresAt", skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<String>,
    /// Plan price in integer cents (architecture standard §1.2).
    #[serde(skip_serializing_if = "Option::is_none", rename = "priceCents")]
    pub price_cents: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interval: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub features: Option<Vec<String>>,
}

/// Memberships list response - matches frontend UserMembershipsResponse
#[derive(Debug, Serialize)]
pub struct MembershipsResponse {
    pub memberships: Vec<UserMembershipResponse>,
}

/// Cancel subscription request
#[derive(Debug, Deserialize)]
pub struct CancelSubscriptionRequest {
    #[serde(default)]
    pub cancel_immediately: bool,
    pub reason: Option<String>,
}

/// Payment method response for frontend - matches PaymentMethod interface
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PaymentMethodResponse {
    pub id: String,
    #[serde(rename = "type")]
    pub method_type: String,
    pub brand: Option<String>,
    pub last4: Option<String>,
    pub expiry_month: Option<u32>,
    pub expiry_year: Option<u32>,
    pub is_default: bool,
    pub subscriptions: Vec<String>,
}

/// Add payment method request
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AddPaymentMethodRequest {
    pub payment_method_id: String,
    #[serde(default)]
    pub set_as_default: bool,
}

// ── Internal DB row helpers (pub(super) so all sub-modules can use) ──

/// Database row for user subscriptions
#[derive(Debug, sqlx::FromRow)]
pub(super) struct UserSubscriptionDbRow {
    pub(super) id: i64,
    #[allow(dead_code)]
    pub(super) user_id: i64,
    pub(super) plan_id: i64,
    pub(super) starts_at: chrono::NaiveDateTime,
    pub(super) expires_at: Option<chrono::NaiveDateTime>,
    pub(super) status: String,
    #[allow(dead_code)]
    pub(super) created_at: chrono::NaiveDateTime,
}

/// Database row for membership plan. Money is integer cents.
#[derive(Debug, sqlx::FromRow)]
pub(super) struct MembershipPlanDbRow {
    #[allow(dead_code)]
    pub(super) id: i64,
    pub(super) name: String,
    pub(super) slug: String,
    pub(super) price_cents: i64,
    pub(super) billing_cycle: String,
    pub(super) metadata: Option<serde_json::Value>,
    pub(super) features: Option<serde_json::Value>,
}
