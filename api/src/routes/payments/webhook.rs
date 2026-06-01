//! Stripe webhook dispatcher.
//!
//! R16-B (2026-05-20): extracted from the original 2,368-LOC
//! `payments.rs` as a pure structural move. Signature verification,
//! idempotency dedup, transaction wrapping, and `event.type` match
//! arms preserved byte-for-byte. Handler call sites now reference
//! `super::webhook_*` modules; their bodies are unchanged.

use axum::{
    body::Bytes,
    extract::State,
    http::{HeaderMap, StatusCode},
    Json,
};
use serde_json::json;

use crate::AppState;

/// Handle Stripe webhooks
/// POST /api/payments/webhook
pub(super) async fn webhook(
    State(state): State<AppState>,
    headers: HeaderMap,
    body: Bytes,
) -> Result<StatusCode, (StatusCode, Json<serde_json::Value>)> {
    // Get signature header
    let signature = headers
        .get("stripe-signature")
        .and_then(|v| v.to_str().ok())
        .ok_or_else(|| {
            (
                StatusCode::BAD_REQUEST,
                Json(json!({"error": "Missing signature"})),
            )
        })?;

    // PE7 invariant 2A: prefer DB-stored webhook secret over env, so admin
    // rotation in /admin/settings takes effect without a redeploy.
    let env_scope = state.config.environment.clone();
    let stripe_for_webhook = state
        .services
        .credentials
        .stripe_client(&state.db.pool, &env_scope)
        .await;

    // ICT 7 Fix: Actually verify webhook signature for production security
    match stripe_for_webhook.verify_webhook(&body, signature) {
        Ok(true) => {
            tracing::debug!(target: "payments", "Webhook signature verified successfully");
        }
        Ok(false) => {
            tracing::error!(target: "payments", "Webhook signature verification failed");
            return Err((
                StatusCode::UNAUTHORIZED,
                Json(json!({"error": "Invalid webhook signature"})),
            ));
        }
        Err(e) => {
            // If webhook secret not configured, reject in production, warn in dev
            if e.to_string().contains("not configured") {
                // FIX-2026-04-26 (Priority 8): read environment from state.config (parsed
                // once at startup) instead of std::env::var per-request. Eliminates
                // race conditions where the env-var name diverges from runtime config and
                // ensures webhook dev-mode bypass never accidentally triggers in prod due
                // to per-request env mutation.
                // Original lines:
                // let environment = std::env::var("ENVIRONMENT").unwrap_or_else(|_| "production".to_string());
                // if environment != "development" && environment != "dev" {
                let env_name = state.config.environment.as_str();
                let is_dev = env_name.starts_with("dev");
                if !is_dev {
                    tracing::error!(
                        target: "payments",
                        environment = %env_name,
                        "Webhook secret not configured in production - rejecting webhook"
                    );
                    return Err((
                        StatusCode::INTERNAL_SERVER_ERROR,
                        Json(json!({"error": "Webhook secret not configured"})),
                    ));
                }
                tracing::warn!(
                    target: "payments",
                    environment = %env_name,
                    "Webhook secret not configured - skipping signature verification (dev mode)"
                );
            } else {
                tracing::error!(target: "payments", "Webhook verification error: {}", e);
                return Err((
                    StatusCode::BAD_REQUEST,
                    Json(json!({"error": "Webhook verification failed"})),
                ));
            }
        }
    }

    // Parse the webhook event — require valid UTF-8 (Stripe always sends UTF-8 JSON)
    let body_str = std::str::from_utf8(&body).map_err(|_| {
        (
            StatusCode::BAD_REQUEST,
            Json(json!({"error": "Webhook payload is not valid UTF-8"})),
        )
    })?;
    let event = state
        .services
        .stripe
        .parse_webhook_event(body_str)
        .map_err(|e| {
            tracing::error!("Webhook parse error: {}", e);
            (
                StatusCode::BAD_REQUEST,
                Json(json!({"error": "Invalid webhook payload"})),
            )
        })?;

    tracing::info!(
        target: "payments",
        event = "webhook_received",
        event_type = %event.event_type,
        event_id = %event.id,
        "Processing Stripe webhook"
    );

    // P0-4 FIX (audit FULL_REPO_AUDIT_2026-05-17) — correct replay-after-
    // partial-failure.
    //
    // OLD behaviour (the bug): the dedup row was INSERTed *before* the handler
    // ran and any subsequent retry was dropped purely on row PRESENCE
    // (`ON CONFLICT DO NOTHING` → no row returned → "duplicate, skip"). A
    // handler that died mid-way (e.g. after the orders UPDATE but before access
    // grants) therefore left the customer charged with no access AND a dedup
    // row that caused every Stripe retry to be skipped — the partial write was
    // never repaired.
    //
    // NEW behaviour: we record/observe the event row, but the skip decision is
    // made on `processed_at IS NOT NULL` (genuinely *completed*), NOT on mere
    // presence. A row that exists with `processed_at IS NULL` means a previous
    // attempt did not finish — we fall through and REPROCESS it. The handler's
    // multi-table writes run inside ONE transaction; `processed_at` is only
    // stamped AFTER that transaction commits, and that stamp is the very last
    // statement of the committing transaction (so "processed" and the business
    // rows are atomically all-or-nothing). If the handler/tx fails we return
    // 5xx with `processed_at` still NULL, Stripe retries, and the next delivery
    // re-runs the handler from a clean slate.
    let payload_value: serde_json::Value =
        serde_json::from_str(body_str).unwrap_or(serde_json::Value::Null);

    // Upsert the observation row (idempotent) and read back whether it has
    // already been fully processed. `processed_at` is the authority.
    let already_processed: Option<bool> = sqlx::query_scalar(
        r"INSERT INTO webhook_events (event_id, event_type, payload)
           VALUES ($1, $2, $3)
           ON CONFLICT (event_id) DO UPDATE SET event_type = EXCLUDED.event_type
           RETURNING (processed_at IS NOT NULL)",
    )
    .bind(&event.id)
    .bind(&event.event_type)
    .bind(&payload_value)
    .fetch_optional(&state.db.pool)
    .await
    .map_err(|e| {
        tracing::error!(target: "payments", "Failed to record webhook event: {}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": "Database error recording webhook"})),
        )
    })?
    .flatten();

    if already_processed == Some(true) {
        tracing::info!(
            target: "payments",
            event_id = %event.id,
            "Webhook event already processed — skipping (idempotent replay)"
        );
        return Ok(StatusCode::OK);
    }

    if already_processed == Some(false) {
        tracing::warn!(
            target: "payments",
            event_id = %event.id,
            event_type = %event.event_type,
            "Webhook event seen but not previously completed — reprocessing prior partial/failed delivery"
        );
    }

    // Open ONE transaction for the whole handler. Every multi-table business
    // write goes through `&mut tx`; nothing is durable until `tx.commit()`.
    // The final statement of this tx flips `webhook_events.processed_at`, so
    // the "processed" marker and the business rows commit atomically.
    let mut tx = state.db.pool.begin().await.map_err(|e| {
        tracing::error!(target: "payments", "Failed to open webhook transaction: {}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": "Database error starting webhook transaction"})),
        )
    })?;

    // Handle different event types — all DB writes are routed through `tx`.
    match event.event_type.as_str() {
        "checkout.session.completed" => {
            super::webhook_checkout::handle_checkout_completed(&state, &mut tx, &event).await?;
        }
        "customer.subscription.created" => {
            super::webhook_subscription::handle_subscription_created(&state, &mut tx, &event)
                .await?;
        }
        "customer.subscription.updated" => {
            super::webhook_subscription::handle_subscription_updated(&state, &mut tx, &event)
                .await?;
        }
        "customer.subscription.deleted" => {
            super::webhook_subscription::handle_subscription_deleted(&state, &mut tx, &event)
                .await?;
        }
        "invoice.paid" => {
            super::webhook_invoice::handle_invoice_paid(&state, &mut tx, &event).await?;
        }
        "invoice.payment_failed" => {
            super::webhook_invoice::handle_payment_failed(&state, &mut tx, &event).await?;
        }
        "charge.refunded" => {
            super::webhook_refund::handle_refund(&state, &mut tx, &event).await?;
        }
        "charge.dispute.created" => {
            super::webhook_refund::handle_dispute_created(&state, &mut tx, &event).await?;
        }
        "customer.subscription.trial_will_end" => {
            super::webhook_subscription::handle_trial_will_end(&state, &mut tx, &event).await?;
        }
        _ => {
            tracing::debug!("Unhandled webhook event: {}", event.event_type);
        }
    }

    // Mark event as processed AS THE LAST STATEMENT OF THE SAME TRANSACTION,
    // then commit. Either the business rows AND this stamp both land, or
    // neither does and Stripe's retry reprocesses (processed_at still NULL).
    sqlx::query("UPDATE webhook_events SET processed_at = NOW() WHERE event_id = $1")
        .bind(&event.id)
        .execute(&mut *tx)
        .await
        .map_err(|e| {
            tracing::error!(target: "payments", "Failed to mark webhook processed: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": "Failed to finalize webhook processing"})),
            )
        })?;

    tx.commit().await.map_err(|e| {
        tracing::error!(target: "payments", "Failed to commit webhook transaction: {}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": "Failed to commit webhook processing"})),
        )
    })?;

    Ok(StatusCode::OK)
}
