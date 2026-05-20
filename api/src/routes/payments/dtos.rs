//! Payment DTOs — shared request/response types.
//!
//! R16-B (2026-05-20): extracted from the original 2,368-LOC
//! `payments.rs` as a pure structural move. `RefundRequest` and
//! `RefundResponse` are re-exported from `mod.rs` so the public path
//! `routes::payments::{RefundRequest, RefundResponse}` is preserved
//! byte-for-byte (the integration tests in `tests/payments_test.rs`
//! depend on it).

use serde::{Deserialize, Serialize};

// Batch 5a: CreateCheckoutRequest / CheckoutItem / CheckoutResponse
// removed alongside the duplicate `create_checkout` handler. The
// canonical request/response shapes for checkout now live in
// `routes/checkout.rs` (`CheckoutRequest` / `CartItem`).

#[derive(Deserialize)]
pub struct CreatePortalRequest {
    pub return_url: String,
}

#[derive(Serialize)]
pub struct PortalResponse {
    pub url: String,
}

#[derive(Deserialize)]
pub struct RefundRequest {
    pub order_id: i64,
    pub amount: Option<i64>,
    pub reason: Option<String>,
}

#[derive(Serialize)]
pub struct RefundResponse {
    pub refund_id: String,
    pub amount: i64,
    pub status: String,
}

/// Invoice generation request
#[derive(Debug, serde::Deserialize)]
pub struct GenerateInvoiceRequest {
    pub order_id: i64,
}

/// Retry payment request
#[derive(Debug, serde::Deserialize)]
pub struct RetryPaymentRequest {
    pub subscription_id: String,
    pub payment_method_id: Option<String>,
}

/// Aggregated payment metrics returned to the admin dashboard.
///
/// All monetary values are integer cents per architecture standard §1.2.
/// Frontend formatCurrency helper handles cents→dollars at render.
#[derive(Debug, Serialize)]
pub struct PaymentSummary {
    /// Revenue this month (alias of `revenue_month_cents`), in cents.
    pub revenue_cents: i64,
    /// Revenue today (cents).
    pub revenue_today_cents: i64,
    /// Revenue over the last 7 days (cents).
    pub revenue_week_cents: i64,
    /// Revenue over the current calendar month (cents).
    pub revenue_month_cents: i64,
    /// Monthly recurring revenue (cents).
    pub mrr_cents: i64,
    /// Count of `orders.status = 'pending'`.
    pub pending_count: i64,
    /// Count of `orders.status = 'failed'`.
    pub failed_count: i64,
    /// Server timestamp the snapshot was generated at.
    pub last_updated: chrono::DateTime<chrono::Utc>,
}
