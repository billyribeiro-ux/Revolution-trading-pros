//! Public payment config (Stripe publishable key) handler.
//!
//! R16-B (2026-05-20): extracted from the original 2,368-LOC
//! `payments.rs` as a pure structural move.

use axum::{extract::State, Json};
use serde_json::json;

use crate::AppState;

/// Get payment configuration (publishable key)
/// GET /api/payments/config
/// ICT 7 Fix: Returns both snake_case and camelCase for frontend compatibility
pub(super) async fn get_config(State(state): State<AppState>) -> Json<serde_json::Value> {
    Json(json!({
        // Snake_case (original)
        "publishable_key": state.config.stripe_publishable_key,
        // CamelCase (frontend compatibility)
        "publishableKey": state.config.stripe_publishable_key,
        "currency": "usd",
        "country": "US"
    }))
}
