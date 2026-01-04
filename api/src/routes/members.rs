//! Members Controller - ICT 11+ Principal Engineer
//!
//! Admin member management with filtering, statistics, and analytics.

use axum::{
    extract::{Path, Query, State},
    response::Json,
    routing::get,
    Router,
};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use chrono::NaiveDateTime;

use crate::{
    utils::errors::ApiError,
    AppState,
};

#[derive(Debug, Serialize, FromRow)]
pub struct Member {
    pub id: i64,
    pub name: Option<String>,
    pub email: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Debug, Deserialize)]
pub struct MemberQuery {
    pub search: Option<String>,
    pub status: Option<String>,
    pub date_from: Option<String>,
    pub date_to: Option<String>,
    pub product_id: Option<i64>,
    pub plan_id: Option<i64>,
    pub sort_by: Option<String>,
    pub sort_dir: Option<String>,
    pub per_page: Option<i64>,
    pub page: Option<i64>,
}

/// GET /admin/members - List all members with filtering
#[tracing::instrument(skip(state))]
pub async fn index(
    State(state): State<AppState>,
    Query(params): Query<MemberQuery>,
) -> Result<Json<serde_json::Value>, ApiError> {
    let mut query = String::from(
        "SELECT id, name, email, first_name, last_name, created_at, updated_at 
         FROM users WHERE 1=1"
    );
    let mut conditions = Vec::new();

    // Search filter
    if let Some(search) = &params.search {
        let escaped = search.replace('\'', "''");
        conditions.push(format!(
            "(name ILIKE '%{}%' OR email ILIKE '%{}%' OR first_name ILIKE '%{}%' OR last_name ILIKE '%{}%')",
            escaped, escaped, escaped, escaped
        ));
    }

    // Date range filters
    if let Some(date_from) = &params.date_from {
        conditions.push(format!("created_at >= '{}'", date_from.replace('\'', "''")));
    }
    if let Some(date_to) = &params.date_to {
        conditions.push(format!("created_at <= '{}'", date_to.replace('\'', "''")));
    }

    if !conditions.is_empty() {
        query.push_str(&format!(" AND {}", conditions.join(" AND ")));
    }

    // Sorting
    let allowed_columns = ["created_at", "updated_at", "name", "email", "first_name", "last_name", "id"];
    let sort_by = params.sort_by.as_deref().unwrap_or("created_at");
    let sort_dir = params.sort_dir.as_deref().unwrap_or("desc");

    let sort_column = if allowed_columns.contains(&sort_by) { sort_by } else { "created_at" };
    let sort_direction = if sort_dir.eq_ignore_ascii_case("asc") { "ASC" } else { "DESC" };

    query.push_str(&format!(" ORDER BY {} {}", sort_column, sort_direction));

    // Pagination
    let per_page = params.per_page.unwrap_or(25).min(100);
    let page = params.page.unwrap_or(1).max(1);
    let offset = (page - 1) * per_page;

    // Get total count
    let count_query = format!(
        "SELECT COUNT(*) FROM users WHERE 1=1{}",
        if !conditions.is_empty() {
            format!(" AND {}", conditions.join(" AND "))
        } else {
            String::new()
        }
    );

    let total: i64 = sqlx::query_scalar(&count_query)
        .fetch_one(state.db.pool())
        .await
        .map_err(|e| ApiError::database_error(&e.to_string()))?;

    query.push_str(&format!(" LIMIT {} OFFSET {}", per_page, offset));

    let members: Vec<Member> = sqlx::query_as(&query)
        .fetch_all(state.db.pool())
        .await
        .map_err(|e| ApiError::database_error(&e.to_string()))?;

    let last_page = (total as f64 / per_page as f64).ceil() as i64;

    Ok(Json(serde_json::json!({
        "members": members,
        "pagination": {
            "total": total,
            "per_page": per_page,
            "current_page": page,
            "last_page": last_page
        }
    })))
}

/// GET /admin/members/stats - Comprehensive member statistics
#[tracing::instrument(skip(state))]
pub async fn stats(
    State(state): State<AppState>,
) -> Result<Json<serde_json::Value>, ApiError> {
    // Total members
    let total_members: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM users")
        .fetch_one(state.db.pool())
        .await
        .unwrap_or(0);

    // New this month
    let new_this_month: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM users WHERE created_at >= date_trunc('month', CURRENT_DATE)"
    )
        .fetch_one(state.db.pool())
        .await
        .unwrap_or(0);

    // New last month
    let new_last_month: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM users 
         WHERE created_at >= date_trunc('month', CURRENT_DATE - INTERVAL '1 month')
         AND created_at < date_trunc('month', CURRENT_DATE)"
    )
        .fetch_one(state.db.pool())
        .await
        .unwrap_or(0);

    // Growth rate
    let growth_rate = if new_last_month > 0 {
        ((new_this_month - new_last_month) as f64 / new_last_month as f64 * 100.0 * 10.0).round() / 10.0
    } else {
        0.0
    };

    // Active subscribers (users with active subscriptions)
    let active_subscribers: i64 = sqlx::query_scalar(
        "SELECT COUNT(DISTINCT user_id) FROM user_subscriptions WHERE status = 'active'"
    )
        .fetch_one(state.db.pool())
        .await
        .unwrap_or(0);

    // Trial members
    let trial_members: i64 = sqlx::query_scalar(
        "SELECT COUNT(DISTINCT user_id) FROM user_subscriptions WHERE status = 'trial'"
    )
        .fetch_one(state.db.pool())
        .await
        .unwrap_or(0);

    // Churned members
    let churned_members: i64 = sqlx::query_scalar(
        "SELECT COUNT(DISTINCT user_id) FROM user_subscriptions 
         WHERE status IN ('cancelled', 'expired')
         AND user_id NOT IN (SELECT DISTINCT user_id FROM user_subscriptions WHERE status = 'active')"
    )
        .fetch_one(state.db.pool())
        .await
        .unwrap_or(0);

    // MRR calculation (simplified)
    let mrr: f64 = sqlx::query_scalar(
        "SELECT COALESCE(SUM(
            CASE 
                WHEN billing_period = 'yearly' THEN price / 12
                WHEN billing_period = 'quarterly' THEN price / 3
                ELSE price
            END
        ), 0) FROM user_subscriptions WHERE status = 'active'"
    )
        .fetch_one(state.db.pool())
        .await
        .unwrap_or(0.0);

    // Total revenue
    let total_revenue: f64 = sqlx::query_scalar(
        "SELECT COALESCE(SUM(price), 0) FROM user_subscriptions"
    )
        .fetch_one(state.db.pool())
        .await
        .unwrap_or(0.0);

    Ok(Json(serde_json::json!({
        "overview": {
            "total_members": total_members,
            "new_this_month": new_this_month,
            "new_last_month": new_last_month,
            "growth_rate": growth_rate
        },
        "subscriptions": {
            "active": active_subscribers,
            "trial": trial_members,
            "churned": churned_members,
            "churn_rate": 0.0 // Would need more complex calculation
        },
        "revenue": {
            "mrr": (mrr * 100.0).round() / 100.0,
            "total": (total_revenue * 100.0).round() / 100.0,
            "avg_ltv": 0.0 // Would need more complex calculation
        }
    })))
}

/// GET /admin/members/:id - Get single member details
#[tracing::instrument(skip(state))]
pub async fn show(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> Result<Json<serde_json::Value>, ApiError> {
    let member: Option<Member> = sqlx::query_as(
        "SELECT id, name, email, first_name, last_name, created_at, updated_at 
         FROM users WHERE id = $1"
    )
    .bind(id)
    .fetch_optional(state.db.pool())
    .await
    .map_err(|e| ApiError::database_error(&e.to_string()))?;

    match member {
        Some(m) => {
            // Get subscription info
            let subscriptions: Vec<serde_json::Value> = sqlx::query_as::<_, (i64, String, Option<f64>, Option<String>, Option<NaiveDateTime>)>(
                "SELECT id, status, price, billing_period, created_at 
                 FROM user_subscriptions WHERE user_id = $1 ORDER BY created_at DESC"
            )
            .bind(id)
            .fetch_all(state.db.pool())
            .await
            .unwrap_or_default()
            .into_iter()
            .map(|(id, status, price, period, created)| {
                serde_json::json!({
                    "id": id,
                    "status": status,
                    "price": price,
                    "billing_period": period,
                    "created_at": created
                })
            })
            .collect();

            Ok(Json(serde_json::json!({
                "member": m,
                "subscriptions": subscriptions,
                "stats": {
                    "total_subscriptions": subscriptions.len(),
                    "active_subscriptions": subscriptions.iter().filter(|s| s["status"] == "active").count()
                }
            })))
        },
        None => Err(ApiError::not_found("Member not found")),
    }
}

/// Build the members router
pub fn router() -> Router<AppState> {
    Router::new()
        .route("/admin/members", get(index))
        .route("/admin/members/stats", get(stats))
        .route("/admin/members/:id", get(show))
}
