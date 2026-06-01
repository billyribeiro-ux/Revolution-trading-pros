//! Daily email reminders job (Batch 6).
//!
//! Fires once a day at 09:00 UTC and queues two kinds of reminders:
//!
//!   1. **Subscription renewal reminders** — for active memberships
//!      whose `current_period_end` falls in [NOW + 6 days, NOW + 8 days].
//!      The 2-day window absorbs run-time drift if the scheduler is
//!      ever delayed; downstream dedup keeps things safe.
//!
//!   2. **Trial-ending reminders** — same shape, but watching the
//!      `trial_end` window. Stripe ALSO emits `customer.subscription.
//!      trial_will_end` 3 days before the trial ends, which goes
//!      through `payments::handle_trial_will_end`. To avoid
//!      double-sending, this job dedup-queries `email_logs` for a
//!      `trial-ending` row to the same address in the last 24h before
//!      enqueueing.
//!
//! Both passes go through `EmailService::send_transactional`, so when
//! `POSTMARK_TOKEN` is unset they write `email_logs` rows with
//! `status='skipped_no_token'` and skip the HTTP call.
//!
//! Mirrors the structure of `reconcile_stripe::spawn_scheduler` for
//! consistency: tokio task, sleep until next 09:00 UTC, run, loop.

use crate::AppState;
use serde_json::json;
use std::time::Duration;

/// Spawn the daily email reminders scheduler.
///
/// First run lands at the next 09:00 UTC after process start (NOT at
/// boot — same rationale as the reconcile job: avoid piling load on
/// cold start). Operators wanting an immediate run on a fresh deploy
/// should add a manual trigger endpoint or call `run_once` from a
/// shell. Currently we don't expose one — the daily cadence is the
/// product contract.
pub fn spawn_scheduler(state: AppState) {
    tokio::spawn(async move {
        loop {
            let secs_until_9am = secs_until_09_utc();
            tracing::info!(
                target: "email_reminders",
                event = "scheduler_sleep",
                seconds = %secs_until_9am,
                "Email reminders job sleeping until 09:00 UTC"
            );
            tokio::time::sleep(Duration::from_secs(secs_until_9am)).await;

            match run_once(&state).await {
                Ok((renewals, trials)) => {
                    tracing::info!(
                        target: "email_reminders",
                        event = "run_complete",
                        renewals_queued = %renewals,
                        trials_queued = %trials,
                        "Email reminders run finished"
                    );
                }
                Err(e) => {
                    tracing::error!(
                        target: "email_reminders",
                        event = "run_error",
                        error = %e,
                        "Email reminders run failed"
                    );
                }
            }
        }
    });
}

/// Seconds from now until the next 09:00 UTC.
fn secs_until_09_utc() -> u64 {
    let now = chrono::Utc::now();
    let target_hour = 9u32;
    let today_9am = now
        .date_naive()
        .and_hms_opt(target_hour, 0, 0)
        .map(|dt| chrono::DateTime::from_naive_utc_and_offset(dt, chrono::Utc))
        .unwrap_or(now);
    let candidate = if today_9am > now {
        today_9am
    } else {
        today_9am + chrono::Duration::days(1)
    };
    // max(1) guarantees non-negative; the cast is intentionally safe.
    #[allow(clippy::cast_sign_loss)]
    let secs = (candidate - now).num_seconds().max(1) as u64;
    secs
}

/// Run one pass. Returns (renewals_queued, trials_queued).
pub async fn run_once(state: &AppState) -> anyhow::Result<(usize, usize)> {
    tracing::info!(
        target: "email_reminders",
        event = "run_start",
        "Starting email reminders pass"
    );

    let renewals = run_renewal_reminders(state).await?;
    let trials = run_trial_ending_reminders(state).await?;
    Ok((renewals, trials))
}

/// Subscription renewal reminders: sub renews in 6-8 days from now.
///
/// 2-day window prevents missed sends if the scheduler is delayed by
/// a few hours; the dedup query against `email_logs.template_alias =
/// 'subscription-renewal-reminder'` in last 24h ensures a user
/// doesn't get two reminders if the job fires twice on the same day.
async fn run_renewal_reminders(state: &AppState) -> anyhow::Result<usize> {
    #[derive(sqlx::FromRow)]
    struct RenewalRow {
        email: String,
        name: String,
        plan_name: String,
        amount_dollars: f64,
        period_end: chrono::NaiveDateTime,
    }

    let rows: Vec<RenewalRow> = sqlx::query_as(
        r"SELECT
               u.email,
               COALESCE(u.name, u.email) AS name,
               mp.name AS plan_name,
               mp.price::FLOAT8 AS amount_dollars,
               um.current_period_end AS period_end
           FROM user_memberships um
           JOIN users u ON u.id = um.user_id
           JOIN membership_plans mp ON mp.id = um.plan_id
           WHERE um.status = 'active'
             AND um.cancel_at_period_end IS NOT TRUE
             AND um.current_period_end IS NOT NULL
             AND um.current_period_end BETWEEN NOW() + INTERVAL '6 days'
                                            AND NOW() + INTERVAL '8 days'
             AND NOT EXISTS (
                 SELECT 1 FROM email_logs el
                 WHERE el.to_email = u.email
                   AND el.template_alias = 'subscription-renewal-reminder'
                   AND el.queued_at > NOW() - INTERVAL '24 hours'
             )",
    )
    .fetch_all(&state.db.pool)
    .await?;

    let count = rows.len();
    for row in rows {
        let _ = state
            .services
            .email
            .send_transactional(
                &state.db.pool,
                &row.email,
                "subscription-renewal-reminder",
                json!({
                    "name": row.name,
                    "plan_name": row.plan_name,
                    "period_end": row.period_end.and_utc().to_rfc3339(),
                    "amount_dollars": row.amount_dollars,
                }),
            )
            .await;
    }

    tracing::info!(
        target: "email_reminders",
        event = "renewal_reminders_queued",
        count = %count,
        "Renewal reminders queued"
    );
    Ok(count)
}

/// Trial-ending reminders: trial_end is 2-4 days from now.
///
/// Stripe sends `customer.subscription.trial_will_end` 3 days out;
/// `handle_trial_will_end` writes a `trial-ending` email_log row.
/// This job's `NOT EXISTS` clause therefore skips users who already
/// got the Stripe-driven send today, and only catches the rare case
/// where the Stripe webhook didn't reach us (network, signature, etc).
async fn run_trial_ending_reminders(state: &AppState) -> anyhow::Result<usize> {
    #[derive(sqlx::FromRow)]
    struct TrialRow {
        email: String,
        name: String,
        plan_name: String,
        trial_end: chrono::NaiveDateTime,
    }

    // user_memberships doesn't have a dedicated trial_end column today,
    // but `current_period_end` during a trial is the trial end. We
    // restrict by `status = 'trialing'` so we don't catch ordinary
    // billing renewals.
    let rows: Vec<TrialRow> = sqlx::query_as(
        r"SELECT
               u.email,
               COALESCE(u.name, u.email) AS name,
               mp.name AS plan_name,
               um.current_period_end AS trial_end
           FROM user_memberships um
           JOIN users u ON u.id = um.user_id
           JOIN membership_plans mp ON mp.id = um.plan_id
           WHERE um.status = 'trialing'
             AND um.current_period_end IS NOT NULL
             AND um.current_period_end BETWEEN NOW() + INTERVAL '2 days'
                                            AND NOW() + INTERVAL '4 days'
             AND NOT EXISTS (
                 SELECT 1 FROM email_logs el
                 WHERE el.to_email = u.email
                   AND el.template_alias = 'trial-ending'
                   AND el.queued_at > NOW() - INTERVAL '24 hours'
             )",
    )
    .fetch_all(&state.db.pool)
    .await?;

    let count = rows.len();
    for row in rows {
        let _ = state
            .services
            .email
            .send_transactional(
                &state.db.pool,
                &row.email,
                "trial-ending",
                json!({
                    "name": row.name,
                    "plan_name": row.plan_name,
                    "trial_end_date": row.trial_end.and_utc().to_rfc3339(),
                }),
            )
            .await;
    }

    tracing::info!(
        target: "email_reminders",
        event = "trial_ending_reminders_queued",
        count = %count,
        "Trial-ending reminders queued"
    );
    Ok(count)
}
