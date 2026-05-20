//! Admin-issued refund handler.
//!
//! R16-B (2026-05-20): extracted from the original 2,368-LOC
//! `payments.rs` as a pure structural move. P2-A logical-op
//! idempotency key, P2-B admin-only gating, P2-C-adjacent
//! propagation of the post-Stripe DB update — all preserved
//! byte-for-byte.

use axum::{extract::State, http::StatusCode, Json};
use serde_json::json;

use super::dtos::{RefundRequest, RefundResponse};
use crate::{middleware::admin::AdminUser, AppState};

/// Process refund request
/// POST /api/payments/refund
///
/// P2-B FIX (audit FULL_REPO_AUDIT_2026-05-17): this endpoint was reachable by
/// the order OWNER (`user: User`) who could supply an arbitrary `amount` and
/// `reason` and was not rate-limited — a self-service unbounded-refund path on
/// a trading SaaS. Refund issuance is now ADMIN-GATED via the existing
/// `AdminUser` extractor (the safe default; same pattern used by
/// `get_summary` and `routes/orders.rs` refund admin op). Legitimate admin
/// refund behavior is preserved; the `amount`/`reason` request fields keep
/// working for admins. Owner self-refund is intentionally removed — customers
/// request refunds through support, an admin issues them.
pub(super) async fn create_refund(
    State(state): State<AppState>,
    AdminUser(admin): AdminUser,
    Json(input): Json<RefundRequest>,
) -> Result<Json<RefundResponse>, (StatusCode, Json<serde_json::Value>)> {
    #[derive(sqlx::FromRow)]
    struct OrderInfo {
        user_id: i64,
        payment_intent_id: Option<String>,
        status: String,
    }

    let order: OrderInfo =
        sqlx::query_as("SELECT user_id, payment_intent_id, status FROM orders WHERE id = $1")
            .bind(input.order_id)
            .fetch_optional(&state.db.pool)
            .await
            .map_err(|e| {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({"error": e.to_string()})),
                )
            })?
            .ok_or_else(|| {
                (
                    StatusCode::NOT_FOUND,
                    Json(json!({"error": "Order not found"})),
                )
            })?;

    // Check order status
    if order.status != "completed" {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(json!({"error": "Order cannot be refunded"})),
        ));
    }

    let payment_intent_id = order.payment_intent_id.ok_or_else(|| {
        (
            StatusCode::BAD_REQUEST,
            Json(json!({"error": "No payment found for this order"})),
        )
    })?;

    // Create refund via Stripe. P2-A: thread a deterministic logical-operation
    // key (the order id) so an admin double-submit / proxied retry of the SAME
    // logical refund reuses the SAME Stripe Idempotency-Key and Stripe collapses
    // the duplicate instead of issuing a second refund. The amount is folded in
    // so a deliberate follow-up partial refund of a different amount is a
    // distinct logical op.
    let logical_op = format!(
        "order:{}:amount:{}",
        input.order_id,
        input
            .amount
            .map(|a| a.to_string())
            .unwrap_or_else(|| "full".to_string())
    );
    let refund = state
        .services
        .stripe
        .create_refund_with_key(
            &payment_intent_id,
            input.amount,
            input.reason.as_deref(),
            &logical_op,
        )
        .await
        .map_err(|e| {
            tracing::error!("Stripe refund error: {}", e);
            (
                StatusCode::BAD_GATEWAY,
                Json(json!({"error": format!("Refund failed: {}", e)})),
            )
        })?;

    // Update order status. P2-C-adjacent: this multi-row-relevant write is
    // propagated (not `.ok()`-swallowed) so a DB failure after a successful
    // Stripe refund surfaces as 5xx for operator follow-up rather than a
    // silent "order still completed but money refunded" divergence.
    sqlx::query("UPDATE orders SET status = 'refunded', refunded_at = NOW(), updated_at = NOW() WHERE id = $1")
        .bind(input.order_id)
        .execute(&state.db.pool)
        .await
        .map_err(|e| {
            tracing::error!(
                target: "payments",
                order_id = %input.order_id,
                refund_id = %refund.id,
                error = %e,
                "Refund succeeded at Stripe but order status update failed"
            );
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": "Refund issued but order status update failed; reconcile manually"})),
            )
        })?;

    tracing::info!(
        target: "payments",
        event = "refund_processed",
        order_id = %input.order_id,
        admin_user_id = %admin.id,
        refund_id = %refund.id,
        amount = %refund.amount,
        "Refund processed by admin"
    );

    Ok(Json(RefundResponse {
        refund_id: refund.id,
        amount: refund.amount,
        status: refund.status,
    }))
}
