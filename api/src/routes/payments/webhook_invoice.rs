//! Invoice webhook handlers: `invoice.paid`, `invoice.payment_failed`.
//!
//! R16-B (2026-05-20): extracted from the original 2,368-LOC
//! `payments.rs` as a pure structural move. P2-C error-propagation
//! fixes (no more `.ok()`-swallowed DB writes after a successful
//! Stripe charge), grace-period accounting, failure-count bump,
//! dunning email best-effort dispatch — all preserved byte-for-byte.

use axum::{http::StatusCode, Json};
use serde_json::json;

use crate::AppState;

// IDEMPOTENT-BY: SET status='active' on user_memberships keyed by
// stripe_subscription_id. Replaying the event re-asserts the same
// status. No counters, no inserts.
pub(super) async fn handle_invoice_paid(
    _state: &AppState,
    tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    event: &crate::services::stripe::WebhookEvent,
) -> Result<(), (StatusCode, Json<serde_json::Value>)> {
    let invoice = &event.data.object;
    let subscription_id = invoice["subscription"].as_str();

    if let Some(sub_id) = subscription_id {
        // P2-C FIX (audit FULL_REPO_AUDIT_2026-05-17): was `.ok();` which
        // swallowed the DB error and returned 200 OK to Stripe — the
        // membership stayed `past_due` after a successful payment and Stripe
        // never retried. Now routed through the webhook `tx` and propagated as
        // 5xx (matching the `handle_subscription_updated` pattern) so the
        // transaction rolls back and Stripe redelivers.
        sqlx::query(
            "UPDATE user_memberships SET status = 'active', updated_at = NOW() WHERE stripe_subscription_id = $1"
        )
        .bind(sub_id)
        .execute(&mut **tx)
        .await
        .map_err(|e| {
            tracing::error!(
                target: "payments",
                subscription_id = %sub_id,
                error = %e,
                "DB write failed in invoice_paid — Stripe will retry"
            );
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": "Failed to mark subscription active"})),
            )
        })?;
    }

    tracing::info!(
        target: "payments",
        event = "invoice_paid",
        invoice_id = %invoice["id"].as_str().unwrap_or("unknown"),
        "Invoice paid"
    );

    Ok(())
}

// IDEMPOTENT-BY: top-level webhook_events dedup. The handler bumps
// payment_failure_count and stores grace_period_end on user_memberships;
// without dedup these would double-increment on retry. Stripe sends
// `invoice.payment_failed` once per attempt, but each retry attempt
// has a distinct event_id, so the top-level dedup is the right
// boundary — same event_id means same attempt and we skip.
pub(super) async fn handle_payment_failed(
    state: &AppState,
    tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    event: &crate::services::stripe::WebhookEvent,
) -> Result<(), (StatusCode, Json<serde_json::Value>)> {
    let invoice = &event.data.object;
    let subscription_id = invoice["subscription"].as_str();
    let customer_email = invoice["customer_email"].as_str();

    // Get payment attempt count and amount from invoice (Stripe stores invoice amounts in cents)
    let attempt_count: i32 = invoice["attempt_count"].as_i64().unwrap_or(1) as i32;
    let amount_due_cents: i64 = invoice["amount_due"].as_i64().unwrap_or(0);

    // ICT 7 Fix: Calculate grace period end (7 days from now for standard grace period)
    // Stripe typically retries 3-4 times over ~3 weeks, so 7 days is a reasonable initial grace period
    let grace_period_days = 7;
    let grace_period_end = chrono::Utc::now() + chrono::Duration::days(grace_period_days);

    if let Some(sub_id) = subscription_id {
        // P2-C FIX (audit FULL_REPO_AUDIT_2026-05-17): was `.ok();` which
        // swallowed the DB error and 200'd Stripe — the membership was never
        // marked `past_due`, no grace period / failure count recorded, and
        // Stripe did not retry, so dunning silently never started. Now routed
        // through the webhook `tx` and propagated as 5xx so the transaction
        // rolls back and Stripe redelivers (the email below is best-effort and
        // is not reached on the error path).
        sqlx::query(
            r"UPDATE user_memberships SET
               status = 'past_due',
               grace_period_end = CASE
                   WHEN grace_period_end IS NULL OR grace_period_end < NOW()
                   THEN $1
                   ELSE grace_period_end
               END,
               payment_failure_count = $2,
               last_payment_failure = NOW(),
               updated_at = NOW()
               WHERE stripe_subscription_id = $3",
        )
        .bind(grace_period_end.naive_utc())
        .bind(attempt_count)
        .bind(sub_id)
        .execute(&mut **tx)
        .await
        .map_err(|e| {
            tracing::error!(
                target: "payments",
                subscription_id = %sub_id,
                error = %e,
                "DB write failed in payment_failed — Stripe will retry"
            );
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": "Failed to record payment failure"})),
            )
        })?;

        // ICT 7 Fix: Send payment failed with grace period email notification
        {
            let email_service = &state.services.email;
            // Get user details from membership including plan price
            #[derive(sqlx::FromRow)]
            struct UserSubscription {
                email: String,
                name: String,
                plan_name: String,
                price_cents: Option<i64>,
            }

            let user_info: Option<UserSubscription> = sqlx::query_as(
                r"SELECT u.email, COALESCE(u.name, u.email) as name,
                   COALESCE(mp.name, 'Subscription') as plan_name,
                   (mp.price * 100)::BIGINT as price_cents
                   FROM user_memberships um
                   JOIN users u ON um.user_id = u.id
                   LEFT JOIN membership_plans mp ON um.plan_id = mp.id
                   WHERE um.stripe_subscription_id = $1
                   LIMIT 1",
            )
            .bind(sub_id)
            .fetch_optional(&mut **tx)
            .await
            .ok()
            .flatten();

            if let Some(info) = user_info {
                // Use amount from invoice, fallback to plan price (all cents)
                let payment_amount_cents: i64 = if amount_due_cents > 0 {
                    amount_due_cents
                } else {
                    info.price_cents.unwrap_or(0)
                };
                let grace_end_str = grace_period_end.format("%B %d, %Y").to_string();

                // Batch 6: failed-payment template via send_transactional.
                let retry_link = format!(
                    "{}/account/billing",
                    state.config.app_url.trim_end_matches('/')
                );
                let _ = email_service
                    .send_transactional(
                        &state.db.pool,
                        &info.email,
                        "failed-payment",
                        json!({
                            "name": info.name,
                            "plan_name": info.plan_name,
                            "amount_dollars": (payment_amount_cents as f64) / 100.0,
                            "retry_link": retry_link,
                            "dunning_period_end": grace_end_str,
                            "attempt_count": attempt_count,
                        }),
                    )
                    .await;

                tracing::info!(
                    target: "payments",
                    event = "payment_failed_email_sent",
                    email = %info.email,
                    subscription_id = %sub_id,
                    grace_period_end = %grace_end_str,
                    attempt_count = %attempt_count,
                    "Payment failed notification email queued"
                );
            } else if let Some(email) = customer_email {
                // Fallback when membership lookup fails — still notify.
                let grace_end_str = grace_period_end.format("%B %d, %Y").to_string();
                let retry_link = format!(
                    "{}/account/billing",
                    state.config.app_url.trim_end_matches('/')
                );
                let _ = email_service
                    .send_transactional(
                        &state.db.pool,
                        email,
                        "failed-payment",
                        json!({
                            "name": "Valued Customer",
                            "plan_name": "your subscription",
                            "amount_dollars": (amount_due_cents as f64) / 100.0,
                            "retry_link": retry_link,
                            "dunning_period_end": grace_end_str,
                            "attempt_count": attempt_count,
                        }),
                    )
                    .await;
            }
        }
    }

    tracing::warn!(
        target: "payments",
        event = "payment_failed",
        invoice_id = %invoice["id"].as_str().unwrap_or("unknown"),
        subscription_id = ?subscription_id,
        attempt_count = %attempt_count,
        amount_due_cents = %amount_due_cents,
        grace_period_end = %grace_period_end.format("%Y-%m-%d"),
        "Payment failed - grace period initialized"
    );

    Ok(())
}
