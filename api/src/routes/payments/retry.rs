//! Payment retry handler (subscription past_due dunning recovery).
//!
//! R16-B (2026-05-20): extracted from the original 2,368-LOC
//! `payments.rs` as a pure structural move. Ownership check,
//! past_due gate, Stripe call site, P2-C error propagation,
//! and `tracing` fields preserved byte-for-byte.

use axum::{extract::State, http::StatusCode, Json};
use serde_json::json;

use super::dtos::RetryPaymentRequest;
use crate::{models::User, AppState};

/// Retry a failed subscription payment
/// POST /api/payments/retry
pub(super) async fn retry_payment(
    State(state): State<AppState>,
    user: User,
    Json(input): Json<RetryPaymentRequest>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    // Verify subscription belongs to user
    #[derive(sqlx::FromRow)]
    struct MembershipRow {
        id: i64,
        user_id: i64,
        stripe_subscription_id: Option<String>,
        stripe_customer_id: Option<String>,
        status: String,
    }

    let membership: MembershipRow = sqlx::query_as(
        "SELECT id, user_id, stripe_subscription_id, stripe_customer_id, status FROM user_memberships WHERE stripe_subscription_id = $1"
    )
    .bind(&input.subscription_id)
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
            Json(json!({"error": "Subscription not found"})),
        )
    })?;

    // Check ownership
    if membership.user_id != user.id {
        return Err((
            StatusCode::FORBIDDEN,
            Json(json!({"error": "Access denied"})),
        ));
    }

    // Check if subscription is in past_due status
    if membership.status != "past_due" {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(json!({"error": "Subscription is not in past_due status"})),
        ));
    }

    // Update payment method if provided
    if let Some(pm_id) = &input.payment_method_id {
        if let Some(customer_id) = &membership.stripe_customer_id {
            state
                .services
                .stripe
                .update_default_payment_method(customer_id, pm_id)
                .await
                .map_err(|e| {
                    (
                        StatusCode::BAD_GATEWAY,
                        Json(json!({"error": format!("Failed to update payment method: {}", e)})),
                    )
                })?;
        }
    }

    // Retry the latest invoice
    let sub_id = membership.stripe_subscription_id.ok_or_else(|| {
        (
            StatusCode::BAD_REQUEST,
            Json(json!({"error": "No Stripe subscription ID found"})),
        )
    })?;

    let result = state
        .services
        .stripe
        .retry_subscription_payment(&sub_id)
        .await
        .map_err(|e| {
            tracing::error!(target: "payments", error = %e, "Payment retry failed");
            (
                StatusCode::BAD_GATEWAY,
                Json(json!({"error": format!("Payment retry failed: {}", e)})),
            )
        })?;

    // Update membership status if successful.
    //
    // P2-C FIX (audit FULL_REPO_AUDIT_2026-05-17): was `.ok();` which
    // swallowed the DB error after Stripe had already CHARGED the customer —
    // the membership stayed `past_due` and nothing surfaced the divergence.
    // Now propagated as 5xx so the caller sees the failure and an operator can
    // reconcile (the payment did go through at Stripe; this is a DB-write
    // failure, not a payment failure).
    if result.success {
        sqlx::query(
            "UPDATE user_memberships SET status = 'active', payment_failure_count = 0, updated_at = NOW() WHERE id = $1"
        )
        .bind(membership.id)
        .execute(&state.db.pool)
        .await
        .map_err(|e| {
            tracing::error!(
                target: "payments",
                membership_id = %membership.id,
                subscription_id = %input.subscription_id,
                error = %e,
                "Payment retry succeeded at Stripe but membership status update failed"
            );
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": "Payment retried but membership update failed; reconcile manually"})),
            )
        })?;

        tracing::info!(
            target: "payments",
            event = "payment_retry_success",
            subscription_id = %input.subscription_id,
            user_id = %user.id,
            "Payment retry successful"
        );
    } else {
        tracing::warn!(
            target: "payments",
            event = "payment_retry_failed",
            subscription_id = %input.subscription_id,
            user_id = %user.id,
            error = ?result.error,
            "Payment retry failed"
        );
    }

    Ok(Json(json!({
        "success": result.success,
        "message": if result.success { "Payment successful" } else { "Payment failed" },
        "error": result.error
    })))
}
