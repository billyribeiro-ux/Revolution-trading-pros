//! Stripe Reconciliation Job
//!
//! Runs daily (03:00 UTC) to catch drift between Stripe subscription state and
//! `user_memberships`. Missed webhooks, network failures, or in-flight updates can
//! leave the two sources out of sync; this job is the backstop.
//!
//! Algorithm (per §14 of docs/architecture/PAYMENTS_ARCHITECTURE_STANDARD.md):
//!   1. Fetch all active/trialing/past_due subs from Stripe (paginated).
//!   2. For each DB row with a stripe_subscription_id:
//!      a. If the sub no longer appears in Stripe → mark cancelled, log.
//!      b. If status drifted → update, log.
//!      c. If cancel_at_period_end drifted → update, log.
//!      d. If current_period_end drifted by >60 s → update, log.
//!   3. Insert one reconciliation_log row with summary + per-discrepancy JSON.

use crate::AppState;
use chrono::NaiveDateTime;
use serde_json::{json, Value};
use std::collections::HashMap;
use std::time::Duration;

/// Spawn the daily reconciliation scheduler.
///
/// Schedule: sleeps until the next 03:00 UTC, runs one pass, then loops.
///
/// Note (Batch 7 §AB verification): the comment used to claim "runs
/// once immediately at startup" — that was wrong. The actual loop
/// sleeps first, runs second. This is intentional in production
/// (don't pile a heavy Stripe-list call onto cold boot) but means
/// **operators must not assume a fresh deploy has reconciled
/// recently**. To force a run on demand, call `POST
/// /api/admin/reconciliation/run` (see `routes/reconciliation.rs`).
pub fn spawn_scheduler(state: AppState) {
    tokio::spawn(async move {
        loop {
            // Sleep until the next 03:00 UTC.
            let secs_until_3am = secs_until_03_utc();
            tracing::info!(
                target: "reconcile",
                event = "scheduler_sleep",
                seconds = %secs_until_3am,
                "Reconciliation job sleeping until 03:00 UTC"
            );
            tokio::time::sleep(Duration::from_secs(secs_until_3am)).await;

            match run_once(&state).await {
                Ok(discrepancies) => {
                    tracing::info!(
                        target: "reconcile",
                        event = "run_complete",
                        discrepancies = %discrepancies,
                        "Reconciliation run finished"
                    );
                }
                Err(e) => {
                    tracing::error!(
                        target: "reconcile",
                        event = "run_error",
                        error = %e,
                        "Reconciliation run failed"
                    );
                }
            }
        }
    });
}

/// Seconds from now until the next 03:00 UTC.
fn secs_until_03_utc() -> u64 {
    let now = chrono::Utc::now();
    let target_hour = 3u32;
    let today_3am = now
        .date_naive()
        .and_hms_opt(target_hour, 0, 0)
        .map(|dt| chrono::DateTime::from_naive_utc_and_offset(dt, chrono::Utc))
        .unwrap_or(now);

    let candidate = if today_3am > now {
        today_3am
    } else {
        today_3am + chrono::Duration::days(1)
    };

    // max(1) guarantees non-negative; the cast is intentionally safe.
    #[allow(clippy::cast_sign_loss)]
    let secs = (candidate - now).num_seconds().max(1) as u64;
    secs
}

/// Run one reconciliation pass. Returns the number of discrepancies corrected.
/// Exposed as `pub` so the admin trigger endpoint can call it directly.
pub async fn run_once(state: &AppState) -> anyhow::Result<usize> {
    tracing::info!(target: "reconcile", event = "run_start", "Starting reconciliation pass");

    // ── 1. Fetch all live subscriptions from Stripe ─────────────────────────
    let stripe = &state.services.stripe;
    let mut stripe_subs: HashMap<String, crate::services::stripe::StripeSubscription> =
        HashMap::new();

    for status in &["active", "trialing", "past_due"] {
        match stripe.list_subscriptions(status).await {
            Ok(subs) => {
                tracing::info!(
                    target: "reconcile",
                    status = %status,
                    count = %subs.len(),
                    "Fetched Stripe subscriptions"
                );
                for s in subs {
                    stripe_subs.insert(s.id.clone(), s);
                }
            }
            Err(e) => {
                tracing::error!(target: "reconcile", status = %status, error = %e, "Failed to list Stripe subs");
                return Err(e);
            }
        }
    }

    // ── 2. Fetch all DB rows that have a stripe_subscription_id ─────────────
    #[derive(sqlx::FromRow)]
    struct MembershipRow {
        id: i64,
        stripe_subscription_id: String,
        status: String,
        cancel_at_period_end: Option<bool>,
        current_period_end: Option<NaiveDateTime>,
    }

    let db_rows: Vec<MembershipRow> = sqlx::query_as(
        r"SELECT id, stripe_subscription_id, status, cancel_at_period_end, current_period_end
           FROM user_memberships
           WHERE stripe_subscription_id IS NOT NULL
             AND status NOT IN ('cancelled', 'expired')",
    )
    .fetch_all(&state.db.pool)
    .await?;

    // ── 3. Compare and fix ───────────────────────────────────────────────────
    let mut log_entries: Vec<Value> = Vec::new();

    for row in &db_rows {
        match stripe_subs.get(&row.stripe_subscription_id) {
            None => {
                // Sub is not in Stripe at all → it was deleted/cancelled and we missed the webhook.
                let entry = fix_missing_in_stripe(state, row.id, &row.stripe_subscription_id).await;
                log_entries.push(entry);
            }
            Some(stripe_sub) => {
                // Map Stripe status to our canonical status
                let expected_status = match stripe_sub.status.as_str() {
                    "active" => "active",
                    "trialing" => "trial",
                    "past_due" => "past_due",
                    "canceled" | "cancelled" => "cancelled",
                    "unpaid" => "unpaid",
                    _ => "active",
                };

                // a. Status drift
                if row.status != expected_status {
                    let entry = fix_status(
                        state,
                        row.id,
                        &row.stripe_subscription_id,
                        &row.status,
                        expected_status,
                    )
                    .await;
                    log_entries.push(entry);
                }

                // b. cancel_at_period_end drift
                let db_cape = row.cancel_at_period_end.unwrap_or(false);
                if db_cape != stripe_sub.cancel_at_period_end {
                    let entry = fix_cancel_at_period_end(
                        state,
                        row.id,
                        &row.stripe_subscription_id,
                        db_cape,
                        stripe_sub.cancel_at_period_end,
                    )
                    .await;
                    log_entries.push(entry);
                }

                // c. current_period_end drift (>60 s tolerance for clock skew)
                let (_, stripe_end_ts) = stripe_sub.get_current_period();
                if stripe_end_ts != 0 {
                    let stripe_end =
                        chrono::DateTime::from_timestamp(stripe_end_ts, 0).map(|d| d.naive_utc());
                    if let Some(stripe_end_naive) = stripe_end {
                        let drift = row
                            .current_period_end
                            .map(|db_end| (db_end - stripe_end_naive).num_seconds().abs());
                        if drift.is_none_or(|d| d > 60) {
                            let entry = fix_period_end(
                                state,
                                row.id,
                                &row.stripe_subscription_id,
                                row.current_period_end,
                                stripe_end_naive,
                            )
                            .await;
                            log_entries.push(entry);
                        }
                    }
                }
            }
        }
    }

    let discrepancies = log_entries.len();

    // ── 4. Write reconciliation_log row ─────────────────────────────────────
    sqlx::query(
        r"INSERT INTO reconciliation_log (run_at, discrepancies_found, log)
           VALUES (NOW(), $1, $2)",
    )
    .bind(i32::try_from(discrepancies).unwrap_or(i32::MAX))
    .bind(serde_json::Value::Array(log_entries))
    .execute(&state.db.pool)
    .await?;

    tracing::info!(
        target: "reconcile",
        event = "run_complete",
        discrepancies = %discrepancies,
        stripe_subs_fetched = %stripe_subs.len(),
        db_rows_checked = %db_rows.len(),
        "Reconciliation pass complete"
    );

    Ok(discrepancies)
}

// ── Fix helpers ──────────────────────────────────────────────────────────────

async fn fix_missing_in_stripe(state: &AppState, row_id: i64, sub_id: &str) -> Value {
    let result = sqlx::query(
        "UPDATE user_memberships SET status = 'cancelled', updated_at = NOW() WHERE id = $1",
    )
    .bind(row_id)
    .execute(&state.db.pool)
    .await;

    let fixed = result.is_ok();
    tracing::warn!(
        target: "reconcile",
        kind = "missing_in_stripe",
        membership_id = %row_id,
        sub_id = %sub_id,
        fixed = %fixed,
        "Membership has no matching Stripe subscription"
    );

    json!({
        "kind": "missing_in_stripe",
        "membership_id": row_id,
        "stripe_subscription_id": sub_id,
        "action": "set status=cancelled",
        "fixed": fixed,
    })
}

async fn fix_status(
    state: &AppState,
    row_id: i64,
    sub_id: &str,
    before: &str,
    after: &str,
) -> Value {
    let result =
        sqlx::query("UPDATE user_memberships SET status = $1, updated_at = NOW() WHERE id = $2")
            .bind(after)
            .bind(row_id)
            .execute(&state.db.pool)
            .await;

    let fixed = result.is_ok();
    tracing::warn!(
        target: "reconcile",
        kind = "status_drift",
        membership_id = %row_id,
        sub_id = %sub_id,
        before = %before,
        after = %after,
        fixed = %fixed,
        "Status drifted between Stripe and DB"
    );

    json!({
        "kind": "status_drift",
        "membership_id": row_id,
        "stripe_subscription_id": sub_id,
        "before": before,
        "after": after,
        "fixed": fixed,
    })
}

async fn fix_cancel_at_period_end(
    state: &AppState,
    row_id: i64,
    sub_id: &str,
    before: bool,
    after: bool,
) -> Value {
    let result = sqlx::query(
        "UPDATE user_memberships SET cancel_at_period_end = $1, updated_at = NOW() WHERE id = $2",
    )
    .bind(after)
    .bind(row_id)
    .execute(&state.db.pool)
    .await;

    let fixed = result.is_ok();
    tracing::warn!(
        target: "reconcile",
        kind = "cape_drift",
        membership_id = %row_id,
        sub_id = %sub_id,
        before = %before,
        after = %after,
        fixed = %fixed,
        "cancel_at_period_end drifted"
    );

    json!({
        "kind": "cancel_at_period_end_drift",
        "membership_id": row_id,
        "stripe_subscription_id": sub_id,
        "before": before,
        "after": after,
        "fixed": fixed,
    })
}

async fn fix_period_end(
    state: &AppState,
    row_id: i64,
    sub_id: &str,
    before: Option<NaiveDateTime>,
    after: NaiveDateTime,
) -> Value {
    let result = sqlx::query(
        "UPDATE user_memberships SET current_period_end = $1, updated_at = NOW() WHERE id = $2",
    )
    .bind(after)
    .bind(row_id)
    .execute(&state.db.pool)
    .await;

    let fixed = result.is_ok();
    tracing::warn!(
        target: "reconcile",
        kind = "period_end_drift",
        membership_id = %row_id,
        sub_id = %sub_id,
        fixed = %fixed,
        "current_period_end drifted"
    );

    json!({
        "kind": "period_end_drift",
        "membership_id": row_id,
        "stripe_subscription_id": sub_id,
        "before": before.map(|d| d.to_string()),
        "after": after.to_string(),
        "fixed": fixed,
    })
}
