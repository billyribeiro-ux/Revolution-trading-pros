//! Notification scheduling — renewal reminders, trial-ending alerts,
//! cancellation confirmation emails.
//!
//! R15-B (2026-05-20): extracted from the original 1,690-LOC
//! `subscriptions.rs` as a pure structural move. All SQL, error
//! mapping, AdminUser gating, and email send-sites preserved
//! byte-for-byte.

use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use serde_json::json;

use crate::{middleware::admin::AdminUser, models::User, AppState};

// ═══════════════════════════════════════════════════════════════════════════
// NOTIFICATION SCHEDULING ENDPOINTS - ICT 7 Fix
// ═══════════════════════════════════════════════════════════════════════════

/// Send renewal reminder notifications
/// POST /subscriptions/notifications/renewal-reminders
/// Called by cron job/scheduled task - sends reminders 3 days before renewal
pub(super) async fn send_renewal_reminders(
    State(state): State<AppState>,
    _admin: AdminUser,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let email_service = &state.services.email;

    // Find subscriptions renewing in 3 days that haven't been reminded yet
    #[derive(sqlx::FromRow)]
    struct RenewalSubscription {
        id: i64,
        user_email: String,
        user_name: String,
        plan_name: String,
        plan_price_cents: i64,
        current_period_end: chrono::NaiveDateTime,
    }

    let subscriptions: Vec<RenewalSubscription> = sqlx::query_as(
        r"
        SELECT
            um.id,
            u.email as user_email,
            COALESCE(u.name, u.email) as user_name,
            mp.name as plan_name,
            (mp.price * 100)::BIGINT as plan_price_cents,
            um.current_period_end
        FROM user_memberships um
        JOIN users u ON um.user_id = u.id
        JOIN membership_plans mp ON um.plan_id = mp.id
        WHERE um.status = 'active'
        AND um.cancel_at_period_end = false
        AND um.current_period_end BETWEEN NOW() + INTERVAL '2 days' AND NOW() + INTERVAL '4 days'
        AND um.renewal_reminder_sent_at IS NULL
        ORDER BY um.current_period_end ASC
        LIMIT 100
        ",
    )
    .fetch_all(&state.db.pool)
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": e.to_string()})),
        )
    })?;

    let mut sent_count = 0;
    let mut failed_count = 0;

    for sub in &subscriptions {
        let renewal_date = sub.current_period_end.format("%B %d, %Y").to_string();

        match email_service
            .send_renewal_reminder(
                &sub.user_email,
                &sub.user_name,
                &sub.plan_name,
                sub.plan_price_cents,
                &renewal_date,
            )
            .await
        {
            Ok(_) => {
                // Mark as reminded
                sqlx::query(
                    "UPDATE user_memberships SET renewal_reminder_sent_at = NOW() WHERE id = $1",
                )
                .bind(sub.id)
                .execute(&state.db.pool)
                .await
                .ok();
                sent_count += 1;
            }
            Err(e) => {
                tracing::error!(
                    target: "subscriptions",
                    subscription_id = %sub.id,
                    error = %e,
                    "Failed to send renewal reminder"
                );
                failed_count += 1;
            }
        }
    }

    tracing::info!(
        target: "subscriptions",
        event = "renewal_reminders_sent",
        sent = %sent_count,
        failed = %failed_count,
        total = %subscriptions.len(),
        "Renewal reminder batch completed"
    );

    Ok(Json(json!({
        "success": true,
        "sent": sent_count,
        "failed": failed_count,
        "total_processed": subscriptions.len()
    })))
}

/// Send trial ending notifications
/// POST /subscriptions/notifications/trial-ending
/// Called by cron job/scheduled task - sends notifications 1 day before trial ends
pub(super) async fn send_trial_ending_notifications(
    State(state): State<AppState>,
    _admin: AdminUser,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let email_service = &state.services.email;

    // Find trial subscriptions ending in 1-2 days that haven't been notified
    #[derive(sqlx::FromRow)]
    struct TrialSubscription {
        id: i64,
        user_email: String,
        user_name: String,
        plan_name: String,
        plan_price_cents: i64,
        billing_cycle: String,
        trial_ends_at: chrono::NaiveDateTime,
    }

    let subscriptions: Vec<TrialSubscription> = sqlx::query_as(
        r"
        SELECT
            um.id,
            u.email as user_email,
            COALESCE(u.name, u.email) as user_name,
            mp.name as plan_name,
            (mp.price * 100)::BIGINT as plan_price_cents,
            mp.billing_cycle,
            um.trial_ends_at
        FROM user_memberships um
        JOIN users u ON um.user_id = u.id
        JOIN membership_plans mp ON um.plan_id = mp.id
        WHERE um.status IN ('trial', 'trialing')
        AND um.trial_ends_at BETWEEN NOW() AND NOW() + INTERVAL '2 days'
        AND um.trial_ending_reminder_sent_at IS NULL
        ORDER BY um.trial_ends_at ASC
        LIMIT 100
        ",
    )
    .fetch_all(&state.db.pool)
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": e.to_string()})),
        )
    })?;

    let mut sent_count = 0;
    let mut failed_count = 0;

    for sub in &subscriptions {
        let trial_end_date = sub.trial_ends_at.format("%B %d, %Y").to_string();

        match email_service
            .send_trial_ending_soon(
                &sub.user_email,
                &sub.user_name,
                &sub.plan_name,
                &trial_end_date,
                sub.plan_price_cents,
                &sub.billing_cycle,
            )
            .await
        {
            Ok(_) => {
                // Mark as notified
                sqlx::query(
                    "UPDATE user_memberships SET trial_ending_reminder_sent_at = NOW() WHERE id = $1"
                )
                .bind(sub.id)
                .execute(&state.db.pool)
                .await
                .ok();
                sent_count += 1;
            }
            Err(e) => {
                tracing::error!(
                    target: "subscriptions",
                    subscription_id = %sub.id,
                    error = %e,
                    "Failed to send trial ending notification"
                );
                failed_count += 1;
            }
        }
    }

    tracing::info!(
        target: "subscriptions",
        event = "trial_ending_notifications_sent",
        sent = %sent_count,
        failed = %failed_count,
        total = %subscriptions.len(),
        "Trial ending notification batch completed"
    );

    Ok(Json(json!({
        "success": true,
        "sent": sent_count,
        "failed": failed_count,
        "total_processed": subscriptions.len()
    })))
}

/// Send cancellation confirmation email
/// POST /subscriptions/:id/send-cancellation-email
pub(super) async fn send_cancellation_email(
    State(state): State<AppState>,
    user: User,
    Path(id): Path<i64>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let email_service = &state.services.email;

    // Get subscription details
    #[derive(sqlx::FromRow)]
    struct CancelledSubscription {
        plan_name: String,
        current_period_end: Option<chrono::NaiveDateTime>,
        cancel_at_period_end: Option<bool>,
        status: String,
    }

    let sub: CancelledSubscription = sqlx::query_as(
        r"
        SELECT mp.name as plan_name, um.current_period_end, um.cancel_at_period_end, um.status
        FROM user_memberships um
        JOIN membership_plans mp ON um.plan_id = mp.id
        WHERE um.id = $1 AND um.user_id = $2
        ",
    )
    .bind(id)
    .bind(user.id)
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

    let user_name = if user.name.is_empty() {
        user.email.clone()
    } else {
        user.name.clone()
    };

    // Determine if cancelled immediately or at period end
    let cancel_immediately =
        sub.status == "cancelled" && !sub.cancel_at_period_end.unwrap_or(false);

    let access_end = if cancel_immediately {
        "immediately".to_string()
    } else {
        sub.current_period_end
            .map(|d| d.format("%B %d, %Y").to_string())
            .unwrap_or_else(|| "immediately".to_string())
    };

    email_service
        .send_subscription_cancelled(
            &user.email,
            &user_name,
            &sub.plan_name,
            &access_end,
            cancel_immediately,
        )
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": e.to_string()})),
            )
        })?;

    Ok(Json(json!({
        "success": true,
        "message": "Cancellation email sent"
    })))
}
