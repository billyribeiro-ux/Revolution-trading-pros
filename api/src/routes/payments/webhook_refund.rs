//! Refund / chargeback webhook handlers: `charge.refunded` and
//! `charge.dispute.created`.
//!
//! R16-B (2026-05-20): extracted from the original 2,368-LOC
//! `payments.rs` as a pure structural move. Full-refund access
//! revocation (memberships + course enrollments + indicator access),
//! payment_disputes ON CONFLICT DO NOTHING insert, security_events
//! audit row, and the operator chargeback notification email — all
//! preserved byte-for-byte. P0-4 + P2-C atomicity comments retained.

use axum::{http::StatusCode, Json};
use serde_json::json;

use crate::AppState;

// IDEMPOTENT-BY: top-level webhook_events dedup. The handler reads
// the charge's amount_refunded value (Stripe-sourced, monotonic) and
// SETs orders.refund_amount_cents to that exact value. Replaying the
// same event writes the same number; replaying a *different* refund
// event for the same charge is a state advance, not a duplicate.
pub(super) async fn handle_refund(
    state: &AppState,
    tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    event: &crate::services::stripe::WebhookEvent,
) -> Result<(), (StatusCode, Json<serde_json::Value>)> {
    let charge = &event.data.object;
    let payment_intent = charge["payment_intent"].as_str();

    // Stripe stores charge.amount and charge.amount_refunded in integer cents
    let refund_amount_cents: i64 = charge["amount_refunded"].as_i64().unwrap_or(0);
    let total_amount_cents: i64 = charge["amount"].as_i64().unwrap_or(0);
    let is_full_refund = refund_amount_cents > 0 && refund_amount_cents >= total_amount_cents;

    if let Some(pi) = payment_intent {
        let refund_status = if is_full_refund {
            "refunded"
        } else {
            "partial_refund"
        };

        sqlx::query(
            r"UPDATE orders SET
               status = $1,
               refund_amount = COALESCE(refund_amount, 0) + ($2::BIGINT / 100.0),
               refunded_at = CASE WHEN $1 = 'refunded' THEN NOW() ELSE refunded_at END,
               updated_at = NOW()
               WHERE payment_intent_id = $3",
        )
        .bind(refund_status)
        .bind(refund_amount_cents)
        .bind(pi)
        .execute(&mut **tx)
        .await
        .map_err(|e| {
            tracing::error!(target: "payments", error = %e, "Failed to update order on refund");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": "Failed to record refund"})),
            )
        })?;

        // Full refund: revoke access for subscriptions, courses, and
        // indicators.
        //
        // P0-4 / P2-C (audit FULL_REPO_AUDIT_2026-05-17): these were
        // `.ok()`-swallowed "best-effort" revocations on the pool. Inside the
        // single webhook transaction that is both incorrect (a failed
        // statement poisons the tx, so a later swallow can't actually save it)
        // and undesirable (a refund that records money-back but silently
        // leaves access live is exactly the partial-state bug P0-4 targets).
        // They are now part of the SAME tx and propagate as 5xx — order
        // refund-state and access revocation are all-or-nothing, and Stripe
        // retries the whole event on failure.
        if is_full_refund {
            // Revoke subscription membership
            sqlx::query(
                r"UPDATE user_memberships SET status = 'cancelled', cancelled_at = NOW(), updated_at = NOW()
                   WHERE id IN (
                       SELECT um.id FROM user_memberships um
                       JOIN orders o ON o.user_id = um.user_id
                       WHERE o.payment_intent_id = $1
                         AND um.status != 'cancelled'
                   )",
            )
            .bind(pi)
            .execute(&mut **tx)
            .await
            .map_err(|e| {
                tracing::error!(target: "payments", payment_intent = %pi, error = %e, "Failed to revoke membership on full refund — rolling back webhook tx");
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({"error": "Failed to revoke membership on refund"})),
                )
            })?;

            // Revoke course enrollment access
            sqlx::query(
                r"UPDATE user_course_enrollments SET is_active = false
                   WHERE user_id = (SELECT user_id FROM orders WHERE payment_intent_id = $1 LIMIT 1)
                     AND is_active = true",
            )
            .bind(pi)
            .execute(&mut **tx)
            .await
            .map_err(|e| {
                tracing::error!(target: "payments", payment_intent = %pi, error = %e, "Failed to revoke course access on full refund — rolling back webhook tx");
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({"error": "Failed to revoke course access on refund"})),
                )
            })?;

            // Revoke indicator access
            sqlx::query(
                r"UPDATE user_indicator_access SET is_active = false
                   WHERE user_id = (SELECT user_id FROM orders WHERE payment_intent_id = $1 LIMIT 1)
                     AND is_active = true",
            )
            .bind(pi)
            .execute(&mut **tx)
            .await
            .map_err(|e| {
                tracing::error!(target: "payments", payment_intent = %pi, error = %e, "Failed to revoke indicator access on full refund — rolling back webhook tx");
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({"error": "Failed to revoke indicator access on refund"})),
                )
            })?;

            tracing::info!(
                target: "payments",
                event = "access_revoked_on_refund",
                payment_intent = %pi,
                "Access revoked following full refund"
            );
        }

        tracing::info!(
            target: "payments",
            event = "charge_refunded",
            charge_id = %charge["id"].as_str().unwrap_or("unknown"),
            payment_intent = %pi,
            refund_amount_cents = %refund_amount_cents,
            total_amount_cents = %total_amount_cents,
            refund_status = %refund_status,
            "Refund processed"
        );

        // Batch 6: refund-confirmation email. Best-effort.
        #[derive(sqlx::FromRow)]
        struct RefundEmailInfo {
            email: String,
            name: String,
            order_number: String,
        }
        let info: Option<RefundEmailInfo> = sqlx::query_as(
            r"SELECT u.email, u.name, o.order_number
               FROM orders o
               JOIN users u ON u.id = o.user_id
               WHERE o.payment_intent_id = $1
               LIMIT 1",
        )
        .bind(pi)
        .fetch_optional(&mut **tx)
        .await
        .ok()
        .flatten();
        if let Some(i) = info {
            let refund_id = charge["refunds"]["data"][0]["id"]
                .as_str()
                .or_else(|| charge["id"].as_str())
                .unwrap_or("unknown");
            let _ = state
                .services
                .email
                .send_transactional(
                    &state.db.pool,
                    &i.email,
                    "refund-confirmation",
                    json!({
                        "name": i.name,
                        "product_name": format!("Order {}", i.order_number),
                        "amount_dollars": (refund_amount_cents as f64) / 100.0,
                        "refund_id": refund_id,
                        "order_number": i.order_number,
                        "is_full_refund": is_full_refund,
                    }),
                )
                .await;
        }
    }

    Ok(())
}

// IDEMPOTENT-BY: stripe_disputes(stripe_dispute_id) UNIQUE +
// ON CONFLICT DO NOTHING (see migration 055). Replaying the event
// hits the unique key and the row stays as it was — first-write-wins
// for dispute creation, which is the correct semantic (a dispute
// happens once; subsequent webhook events for the same dispute id
// are status updates, not creates).
pub(super) async fn handle_dispute_created(
    state: &AppState,
    tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    event: &crate::services::stripe::WebhookEvent,
) -> Result<(), (StatusCode, Json<serde_json::Value>)> {
    let dispute = &event.data.object;

    let dispute_id = dispute["id"].as_str().unwrap_or("unknown");
    let charge_id = dispute["charge"].as_str().unwrap_or("unknown");
    let reason = dispute["reason"].as_str().unwrap_or("unknown");
    let status = dispute["status"].as_str().unwrap_or("unknown");
    let amount_cents = dispute["amount"].as_i64().unwrap_or(0);
    let currency = dispute["currency"].as_str().unwrap_or("usd");
    let response_deadline = dispute["evidence_details"]["due_by"]
        .as_i64()
        .and_then(|ts| chrono::DateTime::from_timestamp(ts, 0));

    // Insert dispute record
    sqlx::query(
        r"INSERT INTO payment_disputes
           (stripe_dispute_id, stripe_charge_id, reason, status, amount_cents, currency, response_deadline)
           VALUES ($1, $2, $3, $4, $5, $6, $7)
           ON CONFLICT (stripe_dispute_id) DO NOTHING",
    )
    .bind(dispute_id)
    .bind(charge_id)
    .bind(reason)
    .bind(status)
    .bind(amount_cents)
    .bind(currency)
    .bind(response_deadline.map(|d| d.naive_utc()))
    .execute(&mut **tx)
    .await
    .map_err(|e| {
        tracing::error!(target: "payments", error = %e, "Failed to insert dispute record");
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": "Failed to record dispute"})),
        )
    })?;

    // Log to security_events. P0-4: now part of the webhook tx. A failed
    // statement inside a transaction aborts the whole tx, so the previous
    // `.ok()` "non-fatal" swallow could not actually keep the dispute row —
    // it is propagated so the security audit row and the dispute row are
    // atomic (Stripe retries the event on failure).
    sqlx::query(
        r"INSERT INTO security_events (event_type, details, created_at)
           VALUES ('chargeback_dispute', $1, NOW())",
    )
    .bind(json!({
        "dispute_id": dispute_id,
        "charge_id": charge_id,
        "reason": reason,
        "amount_cents": amount_cents,
        "currency": currency,
        "status": status,
        "severity": "high"
    }))
    .execute(&mut **tx)
    .await
    .map_err(|e| {
        tracing::error!(target: "payments", error = %e, "Failed to log dispute security_event — rolling back webhook tx");
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": "Failed to record dispute security event"})),
        )
    })?;

    tracing::warn!(
        target: "payments",
        event = "dispute_created",
        dispute_id = %dispute_id,
        charge_id = %charge_id,
        reason = %reason,
        amount_cents = %amount_cents,
        "Chargeback dispute created — admin notification required"
    );

    // Batch 6: notify the operator. Address comes from
    // ADMIN_NOTIFICATION_EMAIL env var; if unset we log a warning and
    // skip rather than send to a guessed address.
    if let Some(admin_email) = state.config.admin_notification_email.as_deref() {
        let _ = state
            .services
            .email
            .send_transactional(
                &state.db.pool,
                admin_email,
                "dispute-created",
                json!({
                    "dispute_id": dispute_id,
                    "charge_id": charge_id,
                    "amount_cents": amount_cents,
                    "amount_dollars": (amount_cents as f64) / 100.0,
                    "currency": currency,
                    "reason": reason,
                    "status": status,
                    "response_deadline": response_deadline.map(|d| d.to_rfc3339()),
                }),
            )
            .await;
    } else {
        tracing::warn!(
            target: "payments",
            event = "dispute_admin_notify_skipped",
            dispute_id = %dispute_id,
            "ADMIN_NOTIFICATION_EMAIL not set; dispute notification not sent"
        );
    }

    Ok(())
}
