//! Payment Routes — Revolution Trading Pros
//! Apple ICT 11+ Principal Engineer Grade - December 2025
//!
//! R16-B (2026-05-20): the original 2,368-LOC `payments.rs` was split
//! into this `payments/` directory as a PURE STRUCTURAL MOVE. Every
//! handler, SQL statement, Stripe call site, error mapping,
//! `tracing` field, idempotency check, `webhook_events` dedup logic,
//! `BEGIN`/`COMMIT` transaction wrapper, audit-log insert, and money
//! invariant (`i64` cents end-to-end) is preserved byte-for-byte.
//!
//! Complete Stripe payment integration:
//! - Checkout session creation
//! - Customer portal access
//! - Webhook handling with order/subscription updates
//! - Refund processing
//!
//! Public API contract:
//!  - `pub fn router()` — mounted at `/payments` (see `routes::mod`).
//!  - DTOs re-exported below (consumed by `tests/payments_test.rs`):
//!    `RefundRequest`, `RefundResponse`.

use axum::{
    routing::{get, post},
    Router,
};

use crate::AppState;

mod config;
mod dtos;
mod invoice;
mod portal;
mod refund;
mod retry;
mod summary;
mod webhook;
mod webhook_checkout;
mod webhook_invoice;
mod webhook_refund;
mod webhook_subscription;

// Re-export the DTOs the integration tests import directly. Keeping
// `routes::payments::{RefundRequest, RefundResponse}` resolvable preserves
// the module's public API across the R16-B split.
pub use dtos::{RefundRequest, RefundResponse};

pub fn router() -> Router<AppState> {
    Router::new()
        // Batch 5a: `/checkout` removed. The duplicate handler at this path
        // diverged from `/api/checkout` (in routes/checkout.rs) and the
        // frontend has been on `/api/checkout` exclusively since the
        // payments-fix-2026-04 cleanup. The single canonical checkout
        // endpoint now lives in `routes/checkout.rs`.
        .route("/portal", post(portal::create_portal))
        .route("/webhook", post(webhook::webhook))
        .route("/refund", post(refund::create_refund))
        .route("/config", get(config::get_config))
        .route("/invoice", post(invoice::generate_invoice))
        .route("/retry", post(retry::retry_payment))
        // FIX-2026-04-26: payment summary for /admin/dashboard — replaces 404
        // when the page calls GET /api/payments/summary.
        .route("/summary", get(summary::get_summary))
}
