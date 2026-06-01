//! Admin-only subscription endpoints — metrics & CSV/JSON export.
//!
//! R15-B (2026-05-20): extracted from the original 1,690-LOC
//! `subscriptions.rs` as a pure structural move. All SQL, error
//! mapping, AdminUser gating, and money handling preserved
//! byte-for-byte.

use axum::{extract::State, http::StatusCode, Json};
use serde_json::json;

use crate::{middleware::admin::AdminUser, AppState};

/// Subscription metrics (admin)
/// ICT 7+ Enterprise: Complete MRR/ARR/churn/LTV calculations
pub(super) async fn get_metrics(
    State(state): State<AppState>,
    _admin: AdminUser,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    // Basic counts
    let active: (i64,) =
        sqlx::query_as("SELECT COUNT(*) FROM user_memberships WHERE status = 'active'")
            .fetch_one(&state.db.pool)
            .await
            .map_err(|e| {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({"error": e.to_string()})),
                )
            })?;

    let cancelled: (i64,) =
        sqlx::query_as("SELECT COUNT(*) FROM user_memberships WHERE status = 'cancelled'")
            .fetch_one(&state.db.pool)
            .await
            .map_err(|e| {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({"error": e.to_string()})),
                )
            })?;

    let expired: (i64,) =
        sqlx::query_as("SELECT COUNT(*) FROM user_memberships WHERE status = 'expired'")
            .fetch_one(&state.db.pool)
            .await
            .map_err(|e| {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({"error": e.to_string()})),
                )
            })?;

    let trial: (i64,) =
        sqlx::query_as("SELECT COUNT(*) FROM user_memberships WHERE status = 'trial'")
            .fetch_one(&state.db.pool)
            .await
            .unwrap_or((0,));

    let paused: (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM user_memberships WHERE status = 'paused' OR status = 'on-hold'",
    )
    .fetch_one(&state.db.pool)
    .await
    .unwrap_or((0,));

    let pending_cancel: (i64,) =
        sqlx::query_as("SELECT COUNT(*) FROM user_memberships WHERE cancel_at_period_end = true AND status = 'active'")
            .fetch_one(&state.db.pool)
            .await
            .unwrap_or((0,));

    // MRR in integer cents — sum active+trialing memberships' plan prices, normalized to monthly
    let mrr_result: Option<(i64,)> = sqlx::query_as(
        r"
        SELECT COALESCE(SUM(
            CASE mp.billing_cycle
                WHEN 'monthly'   THEN (mp.price * 100)::BIGINT
                WHEN 'quarterly' THEN ((mp.price * 100) / 3)::BIGINT
                WHEN 'annual'    THEN ((mp.price * 100) / 12)::BIGINT
                WHEN 'yearly'    THEN ((mp.price * 100) / 12)::BIGINT
                ELSE (mp.price * 100)::BIGINT
            END
        ), 0)::BIGINT as mrr_cents
        FROM user_memberships um
        JOIN membership_plans mp ON um.plan_id = mp.id
        WHERE um.status IN ('active', 'trialing')
        ",
    )
    .fetch_optional(&state.db.pool)
    .await
    .unwrap_or(None);

    let mrr_cents: i64 = mrr_result.map(|r| r.0).unwrap_or(0);
    let arr_cents: i64 = mrr_cents.saturating_mul(12);

    // New subscriptions this month
    let new_this_month: (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM user_memberships WHERE starts_at >= date_trunc('month', CURRENT_DATE)"
    )
    .fetch_one(&state.db.pool)
    .await
    .unwrap_or((0,));

    // Cancelled this month
    let cancelled_this_month: (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM user_memberships WHERE cancelled_at >= date_trunc('month', CURRENT_DATE)"
    )
    .fetch_one(&state.db.pool)
    .await
    .unwrap_or((0,));

    // ICT 7+ Enterprise: Calculate churn rate
    // Churn = (Cancelled this month / Active at start of month) * 100
    let active_start_of_month: (i64,) = sqlx::query_as(
        r"
        SELECT COUNT(*) FROM user_memberships
        WHERE starts_at < date_trunc('month', CURRENT_DATE)
        AND (cancelled_at IS NULL OR cancelled_at >= date_trunc('month', CURRENT_DATE))
        AND status IN ('active', 'cancelled')
        ",
    )
    .fetch_one(&state.db.pool)
    .await
    .unwrap_or((1,)); // Avoid division by zero

    let churn_rate = if active_start_of_month.0 > 0 {
        (cancelled_this_month.0 as f64 / active_start_of_month.0 as f64) * 100.0
    } else {
        0.0
    };

    // ICT 7+ Enterprise: Calculate Average Lifetime Value (LTV)
    // LTV = Average subscription duration * ARPU (Average Revenue Per User)
    let avg_duration_months: Option<(f64,)> = sqlx::query_as(
        r"
        SELECT COALESCE(AVG(
            EXTRACT(EPOCH FROM (COALESCE(cancelled_at, NOW()) - starts_at)) / (30.0 * 24.0 * 3600.0)
        ), 0)
        FROM user_memberships
        WHERE starts_at IS NOT NULL
        ",
    )
    .fetch_optional(&state.db.pool)
    .await
    .unwrap_or(None);

    let avg_subscription_months = avg_duration_months.map(|r| r.0).unwrap_or(0.0);
    // ARPU and LTV in integer cents (avg-months × ARPU is fractional → keep float for the
    // ARPU-derived figures, but the inputs and the final reported amounts are cents).
    let arpu_cents: i64 = if active.0 > 0 {
        mrr_cents / active.0
    } else {
        0
    };
    let ltv_cents: i64 = (avg_subscription_months * arpu_cents as f64).round() as i64;

    // Failed payments (subscriptions in grace period or past_due)
    let failed_payments: (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM user_memberships WHERE status = 'past_due' OR (grace_period_end IS NOT NULL AND grace_period_end > NOW())"
    )
    .fetch_one(&state.db.pool)
    .await
    .unwrap_or((0,));

    // Upcoming renewals (next 7 days)
    let upcoming_renewals: (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM user_memberships WHERE status = 'active' AND current_period_end BETWEEN NOW() AND NOW() + INTERVAL '7 days'"
    )
    .fetch_one(&state.db.pool)
    .await
    .unwrap_or((0,));

    Ok(Json(json!({
        // Basic counts
        "total_active": active.0,
        "total_cancelled": cancelled.0,
        "total_expired": expired.0,
        "total_trial": trial.0,
        "total_paused": paused.0,
        "pending_cancel": pending_cancel.0,

        // Revenue metrics — integer cents
        "mrr_cents": mrr_cents,
        "arr_cents": arr_cents,
        "monthly_recurring_revenue_cents": mrr_cents,
        "annual_recurring_revenue_cents": arr_cents,

        // Growth metrics
        "new_this_month": new_this_month.0,
        "cancelled_this_month": cancelled_this_month.0,
        "net_growth": new_this_month.0 - cancelled_this_month.0,

        // Churn metrics
        "churn_rate": (churn_rate * 100.0).round() / 100.0,

        // LTV metrics — integer cents
        "average_lifetime_value_cents": ltv_cents,
        "average_subscription_months": (avg_subscription_months * 10.0).round() / 10.0,
        "arpu_cents": arpu_cents,

        // Health metrics
        "failed_payments": failed_payments.0,
        "upcoming_renewals_7_days": upcoming_renewals.0,

        // Totals
        "total_subscriptions": active.0 + cancelled.0 + expired.0 + trial.0 + paused.0
    })))
}

/// GET /subscriptions/export - Export subscriptions as CSV
/// ICT 7 SECURITY FIX: Added AdminUser authentication - export is admin-only
pub(super) async fn export_subscriptions(
    State(state): State<AppState>,
    _admin: AdminUser,
    axum::extract::Query(params): axum::extract::Query<std::collections::HashMap<String, String>>,
) -> Result<axum::response::Response, (StatusCode, Json<serde_json::Value>)> {
    use axum::http::header;
    use axum::response::IntoResponse;

    let format = params.get("format").map(|s| s.as_str()).unwrap_or("csv");

    // Fetch subscriptions with user info
    #[allow(clippy::type_complexity)]
    let subscriptions: Vec<(
        i64,
        i64,
        Option<String>,
        String,
        Option<chrono::NaiveDateTime>,
        Option<chrono::NaiveDateTime>,
        Option<f64>,
        Option<String>,
        Option<String>,
    )> = sqlx::query_as(
        r"
        SELECT
            us.id,
            us.user_id,
            u.email,
            us.status,
            us.starts_at,
            us.expires_at,
            us.price,
            us.product_name,
            us.billing_period
        FROM user_memberships us
        LEFT JOIN users u ON us.user_id = u.id
        ORDER BY us.created_at DESC
        LIMIT 10000
        ",
    )
    .fetch_all(state.db.pool())
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": e.to_string()})),
        )
    })?;

    if format == "json" {
        let json_data: Vec<serde_json::Value> = subscriptions
            .iter()
            .map(
                |(id, user_id, email, status, starts, expires, price, name, period)| {
                    json!({
                        "id": id,
                        "user_id": user_id,
                        "email": email,
                        "status": status,
                        "starts_at": starts,
                        "expires_at": expires,
                        "price": price,
                        "product_name": name,
                        "billing_period": period
                    })
                },
            )
            .collect();

        // ICT 7: Surface a serialization failure as a 500 instead of returning
        // an empty 200 body — silent failure here would mask a real defect.
        let body = serde_json::to_string(&json_data).map_err(|e| {
            tracing::error!("Failed to serialize subscription export JSON: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": "Failed to encode export"})),
            )
        })?;
        let response = ([(header::CONTENT_TYPE, "application/json")], body);
        return Ok(response.into_response());
    }

    // CSV format
    let mut csv = String::from(
        "id,user_id,email,status,starts_at,expires_at,price,product_name,billing_period\n",
    );
    for (id, user_id, email, status, starts, expires, price, name, period) in subscriptions {
        csv.push_str(&format!(
            "{},{},{},{},{},{},{},{},{}\n",
            id,
            user_id,
            email.as_deref().unwrap_or("").replace(',', ";"),
            status,
            starts.map(|d| d.to_string()).unwrap_or_default(),
            expires.map(|d| d.to_string()).unwrap_or_default(),
            price.unwrap_or(0.0),
            name.as_deref().unwrap_or("").replace(',', ";"),
            period.as_deref().unwrap_or("")
        ));
    }

    let response = (
        [
            (header::CONTENT_TYPE, "text/csv"),
            (
                header::CONTENT_DISPOSITION,
                "attachment; filename=\"subscriptions.csv\"",
            ),
        ],
        csv,
    );

    Ok(response.into_response())
}
