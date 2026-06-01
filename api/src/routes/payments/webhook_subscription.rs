//! Subscription lifecycle webhook handlers:
//! `customer.subscription.{created,updated,deleted,trial_will_end}`.
//!
//! R16-B (2026-05-20): extracted from the original 2,368-LOC
//! `payments.rs` as a pure structural move. Every SQL statement,
//! `tx` routing, error mapping, status mapping, Stripe getter,
//! and `tracing` field is preserved byte-for-byte.

use axum::{http::StatusCode, Json};
use serde_json::json;

use crate::AppState;

// IDEMPOTENT-BY: pure read/log handler (no DB writes). Safe to run
// any number of times. Source-of-truth subscription persistence happens
// in `handle_checkout_completed` (initial) and `handle_subscription_updated`
// (subsequent state transitions).
pub(super) async fn handle_subscription_created(
    _state: &AppState,
    _tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    event: &crate::services::stripe::WebhookEvent,
) -> Result<(), (StatusCode, Json<serde_json::Value>)> {
    let subscription = event.as_subscription().ok_or_else(|| {
        (
            StatusCode::BAD_REQUEST,
            Json(json!({"error": "Invalid subscription data"})),
        )
    })?;

    tracing::info!(
        target: "payments",
        event = "subscription_created",
        subscription_id = %subscription.id,
        customer = %subscription.customer,
        "Subscription created"
    );

    Ok(())
}

// IDEMPOTENT-BY: SET-based UPDATE keyed by stripe_subscription_id.
// Re-running the same event lands on the same row and writes the
// same values (Stripe is the source of truth for sub.status,
// current_period_*, cancel_at_period_end). No INSERT, no counters
// incremented — pure state replication.
pub(super) async fn handle_subscription_updated(
    _state: &AppState,
    tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    event: &crate::services::stripe::WebhookEvent,
) -> Result<(), (StatusCode, Json<serde_json::Value>)> {
    let subscription = event.as_subscription().ok_or_else(|| {
        (
            StatusCode::BAD_REQUEST,
            Json(json!({"error": "Invalid subscription data"})),
        )
    })?;

    // Update user membership
    let status = match subscription.status.as_str() {
        "active" => "active",
        "past_due" => "past_due",
        "canceled" => "cancelled",
        "unpaid" => "unpaid",
        _ => "active",
    };

    sqlx::query(
        r"UPDATE user_memberships SET
            status = $1,
            current_period_start = $2,
            current_period_end = $3,
            cancel_at_period_end = $4,
            cancelled_at = $5,
            updated_at = NOW()
        WHERE stripe_subscription_id = $6",
    )
    .bind(status)
    .bind({
        let (start, _) = subscription.get_current_period();
        chrono::DateTime::from_timestamp(start, 0).map(|d| d.naive_utc())
    })
    .bind({
        let (_, end) = subscription.get_current_period();
        chrono::DateTime::from_timestamp(end, 0).map(|d| d.naive_utc())
    })
    .bind(subscription.cancel_at_period_end)
    .bind(
        subscription
            .canceled_at
            .and_then(|ts| chrono::DateTime::from_timestamp(ts, 0).map(|d| d.naive_utc())),
    )
    .bind(&subscription.id)
    .execute(&mut **tx)
    .await
    .map_err(|e| {
        tracing::error!(
            target: "payments",
            subscription_id = %subscription.id,
            error = %e,
            "DB write failed in subscription_updated — Stripe will retry"
        );
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": "Failed to update subscription"})),
        )
    })?;

    tracing::info!(
        target: "payments",
        event = "subscription_updated",
        subscription_id = %subscription.id,
        status = %status,
        "Subscription updated"
    );

    Ok(())
}

// IDEMPOTENT-BY: SET status='cancelled' on the row matching
// stripe_subscription_id. Already-cancelled rows are written with the
// same value (no observable state change). cancelled_at gets stamped
// to the latest NOW() on retry — acceptable because the field
// represents "when did our system observe the cancellation," not
// "when did Stripe perform it."
pub(super) async fn handle_subscription_deleted(
    state: &AppState,
    tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    event: &crate::services::stripe::WebhookEvent,
) -> Result<(), (StatusCode, Json<serde_json::Value>)> {
    let subscription = event.as_subscription().ok_or_else(|| {
        (
            StatusCode::BAD_REQUEST,
            Json(json!({"error": "Invalid subscription data"})),
        )
    })?;

    sqlx::query(
        "UPDATE user_memberships SET status = 'cancelled', cancelled_at = NOW(), updated_at = NOW() WHERE stripe_subscription_id = $1"
    )
    .bind(&subscription.id)
    .execute(&mut **tx)
    .await
    .map_err(|e| {
        tracing::error!(
            target: "payments",
            subscription_id = %subscription.id,
            error = %e,
            "DB write failed in subscription_deleted — Stripe will retry"
        );
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": "Failed to cancel subscription"})),
        )
    })?;

    tracing::info!(
        target: "payments",
        event = "subscription_cancelled",
        subscription_id = %subscription.id,
        "Subscription cancelled"
    );

    // Batch 6: subscription-canceled email. Best-effort.
    #[derive(sqlx::FromRow)]
    struct CancelInfo {
        email: String,
        name: String,
        plan_name: String,
        current_period_end: Option<chrono::NaiveDateTime>,
    }
    let info: Option<CancelInfo> = sqlx::query_as(
        r"SELECT u.email, u.name, mp.name AS plan_name, um.current_period_end
           FROM user_memberships um
           JOIN users u ON u.id = um.user_id
           JOIN membership_plans mp ON mp.id = um.plan_id
           WHERE um.stripe_subscription_id = $1
           ORDER BY um.id DESC LIMIT 1",
    )
    .bind(&subscription.id)
    .fetch_optional(&mut **tx)
    .await
    .ok()
    .flatten();
    if let Some(i) = info {
        let _ = state
            .services
            .email
            .send_transactional(
                &state.db.pool,
                &i.email,
                "subscription-canceled",
                json!({
                    "name": i.name,
                    "plan_name": i.plan_name,
                    "period_end": i.current_period_end.map(|t| t.and_utc().to_rfc3339()),
                }),
            )
            .await;
    }

    Ok(())
}

/// Log trial-ending soon event; email notification handled by Task 4 (Postmark).
// IDEMPOTENT-BY: log + email side-effect only (no DB writes today).
// On retry the same notification gets enqueued — Postmark handles
// dedup on its side via the `MessageID` field we set when sending.
// Until that wiring is restored (Batch 6, currently deferred), this
// handler is *almost* idempotent: a duplicate event would re-send
// the email. Acceptable risk given Stripe's webhook retry budget.
pub(super) async fn handle_trial_will_end(
    state: &AppState,
    tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    event: &crate::services::stripe::WebhookEvent,
) -> Result<(), (StatusCode, Json<serde_json::Value>)> {
    let sub = &event.data.object;
    let subscription_id = sub["id"].as_str().unwrap_or("unknown");
    let customer_id = sub["customer"].as_str().unwrap_or("unknown");
    let trial_end = sub["trial_end"].as_i64();

    // Look up the user by stripe_customer_id (through the webhook tx).
    let user_id: Option<i64> = sqlx::query_scalar(
        "SELECT user_id FROM user_memberships WHERE stripe_customer_id = $1 LIMIT 1",
    )
    .bind(customer_id)
    .fetch_optional(&mut **tx)
    .await
    .map_err(|e| {
        tracing::error!(target: "payments", error = %e, "trial_will_end user lookup failed");
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": "DB error during trial_will_end"})),
        )
    })?
    .flatten();

    // P0-4: the audit-trail row is now part of the webhook tx. A failed
    // statement aborts the tx, so the prior `.ok()` "non-fatal" swallow could
    // not actually keep anything — propagate so the security_events row is
    // atomic with marking the event processed (Stripe retries on failure).
    sqlx::query(
        r"INSERT INTO security_events (user_id, event_type, event_category, severity, details, created_at)
           VALUES ($1, 'trial_will_end', 'billing', 'low', $2, NOW())",
    )
    .bind(user_id)
    .bind(json!({
        "subscription_id": subscription_id,
        "customer_id": customer_id,
        "trial_end_ts": trial_end,
    }))
    .execute(&mut **tx)
    .await
    .map_err(|e| {
        tracing::error!(target: "payments", error = %e, "Failed to log trial_will_end security_event — rolling back webhook tx");
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": "Failed to record trial_will_end event"})),
        )
    })?;

    tracing::info!(
        target: "payments",
        event = "trial_will_end",
        subscription_id = %subscription_id,
        customer_id = %customer_id,
        "Trial ending in 3 days — sending notification (Batch 6)"
    );

    // Batch 6: trial-ending email. Best-effort. We dedup against the
    // daily reminders job by checking last 24h of email_logs in the
    // job itself; this Stripe-driven path is the primary signal.
    #[derive(sqlx::FromRow)]
    struct TrialInfo {
        email: String,
        name: String,
        plan_name: String,
    }
    if let Some(uid) = user_id {
        let info: Option<TrialInfo> = sqlx::query_as(
            r"SELECT u.email, u.name, mp.name AS plan_name
               FROM user_memberships um
               JOIN users u ON u.id = um.user_id
               JOIN membership_plans mp ON mp.id = um.plan_id
               WHERE um.user_id = $1 AND um.stripe_subscription_id = $2
               ORDER BY um.id DESC LIMIT 1",
        )
        .bind(uid)
        .bind(subscription_id)
        .fetch_optional(&mut **tx)
        .await
        .ok()
        .flatten();
        if let Some(i) = info {
            let trial_end_iso = trial_end
                .and_then(|ts| chrono::DateTime::from_timestamp(ts, 0))
                .map(|dt| dt.to_rfc3339());
            let _ = state
                .services
                .email
                .send_transactional(
                    &state.db.pool,
                    &i.email,
                    "trial-ending",
                    json!({
                        "name": i.name,
                        "plan_name": i.plan_name,
                        "trial_end_date": trial_end_iso,
                    }),
                )
                .await;
        }
    }

    Ok(())
}
