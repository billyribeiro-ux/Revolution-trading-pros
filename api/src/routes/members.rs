//! Members Controller - ICT 11+ Principal Engineer
//!
//! Admin member management with filtering, statistics, and analytics.
//! SECURITY: All queries use parameterized bindings to prevent SQL injection.

#![allow(clippy::collapsible_str_replace)]
#![allow(clippy::type_complexity)]

use axum::{
    extract::{Path, Query, State},
    response::Json,
    routing::get,
    Router,
};
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

use crate::{middleware::admin::AdminUser, utils::errors::ApiError, AppState};

#[derive(Debug, Serialize, FromRow)]
pub struct Member {
    pub id: i64,
    pub name: Option<String>,
    pub email: String,
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

/// Validate date format (YYYY-MM-DD) - ICT Level 7 Security
fn is_valid_date_format(date: &str) -> bool {
    if date.len() != 10 {
        return false;
    }
    let parts: Vec<&str> = date.split('-').collect();
    if parts.len() != 3 {
        return false;
    }
    parts[0].len() == 4
        && parts[1].len() == 2
        && parts[2].len() == 2
        && parts.iter().all(|p| p.chars().all(|c| c.is_ascii_digit()))
}

/// Validate sort column against allowlist - ICT Level 7 Security
fn validate_sort_column(column: &str) -> &'static str {
    match column {
        "created_at" => "created_at",
        "updated_at" => "updated_at",
        "name" => "name",
        "email" => "email",
        "id" => "id",
        _ => "created_at",
    }
}

/// Validate sort direction - ICT Level 7 Security
fn validate_sort_direction(dir: &str) -> &'static str {
    if dir.eq_ignore_ascii_case("asc") {
        "ASC"
    } else {
        "DESC"
    }
}

/// GET /admin/members - List all members with filtering
/// ICT Level 7 SECURITY: All filters use parameterized queries
#[tracing::instrument(skip(state, _admin))]
pub async fn index(
    State(state): State<AppState>,
    _admin: AdminUser,
    Query(params): Query<MemberQuery>,
) -> Result<Json<serde_json::Value>, ApiError> {
    let per_page = params.per_page.unwrap_or(25).min(100);
    let page = params.page.unwrap_or(1).max(1);
    let offset = (page - 1) * per_page;

    let sort_column = validate_sort_column(params.sort_by.as_deref().unwrap_or("created_at"));
    let sort_direction = validate_sort_direction(params.sort_dir.as_deref().unwrap_or("desc"));

    // Prepare search pattern
    let search_pattern = params
        .search
        .as_ref()
        .filter(|s| !s.is_empty() && s.len() <= 100)
        .map(|s| format!("%{}%", s));

    // Validate dates
    let date_from = params
        .date_from
        .as_ref()
        .filter(|d| is_valid_date_format(d))
        .cloned();
    let date_to = params
        .date_to
        .as_ref()
        .filter(|d| is_valid_date_format(d))
        .cloned();

    // Build dynamic query with parameterized bindings
    // Using a single query with COALESCE/NULL checks for optional filters
    let members: Vec<Member> = sqlx::query_as(
        &format!(
            r#"
            SELECT id, name, email, created_at, updated_at
            FROM users
            WHERE
                ($1::TEXT IS NULL OR (name ILIKE $1 OR email ILIKE $1))
                AND ($2::DATE IS NULL OR created_at >= $2::DATE)
                AND ($3::DATE IS NULL OR created_at <= $3::DATE + INTERVAL '1 day')
            ORDER BY {} {}
            LIMIT $4 OFFSET $5
            "#,
            sort_column, sort_direction
        ),
    )
    .bind(&search_pattern)
    .bind(&date_from)
    .bind(&date_to)
    .bind(per_page)
    .bind(offset)
    .fetch_all(state.db.pool())
    .await
    .map_err(|e| ApiError::database_error(&e.to_string()))?;

    // Get total count with same filters
    let total: i64 = sqlx::query_scalar(
        r#"
        SELECT COUNT(*)
        FROM users
        WHERE
            ($1::TEXT IS NULL OR (name ILIKE $1 OR email ILIKE $1))
            AND ($2::DATE IS NULL OR created_at >= $2::DATE)
            AND ($3::DATE IS NULL OR created_at <= $3::DATE + INTERVAL '1 day')
        "#,
    )
    .bind(&search_pattern)
    .bind(&date_from)
    .bind(&date_to)
    .fetch_one(state.db.pool())
    .await
    .map_err(|e| ApiError::database_error(&e.to_string()))?;

    let last_page = (total as f64 / per_page as f64).ceil() as i64;

    Ok(Json(serde_json::json!({
        "members": members,
        "pagination": {
            "total": total,
            "per_page": per_page,
            "current_page": page,
            "last_page": last_page.max(1)
        }
    })))
}

/// GET /admin/members/stats - Comprehensive member statistics
#[tracing::instrument(skip(state, _admin))]
pub async fn stats(
    State(state): State<AppState>,
    _admin: AdminUser,
) -> Result<Json<serde_json::Value>, ApiError> {
    // Total members
    let total_members: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM users")
        .fetch_one(state.db.pool())
        .await
        .unwrap_or(0);

    // New this month
    let new_this_month: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM users WHERE created_at >= date_trunc('month', CURRENT_DATE)",
    )
    .fetch_one(state.db.pool())
    .await
    .unwrap_or(0);

    // New last month
    let new_last_month: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM users
         WHERE created_at >= date_trunc('month', CURRENT_DATE - INTERVAL '1 month')
         AND created_at < date_trunc('month', CURRENT_DATE)",
    )
    .fetch_one(state.db.pool())
    .await
    .unwrap_or(0);

    // Growth rate
    let growth_rate = if new_last_month > 0 {
        ((new_this_month - new_last_month) as f64 / new_last_month as f64 * 100.0 * 10.0).round()
            / 10.0
    } else {
        0.0
    };

    // Active subscribers (users with active subscriptions)
    let active_subscribers: i64 = sqlx::query_scalar(
        "SELECT COUNT(DISTINCT user_id) FROM user_memberships WHERE status = 'active'",
    )
    .fetch_one(state.db.pool())
    .await
    .unwrap_or(0);

    // Trial members
    let trial_members: i64 = sqlx::query_scalar(
        "SELECT COUNT(DISTINCT user_id) FROM user_memberships WHERE status = 'trial'",
    )
    .fetch_one(state.db.pool())
    .await
    .unwrap_or(0);

    // Churned members
    let churned_members: i64 = sqlx::query_scalar(
        "SELECT COUNT(DISTINCT user_id) FROM user_memberships
         WHERE status IN ('cancelled', 'expired')
         AND user_id NOT IN (SELECT DISTINCT user_id FROM user_memberships WHERE status = 'active')",
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
        ), 0) FROM user_memberships WHERE status = 'active'",
    )
    .fetch_one(state.db.pool())
    .await
    .unwrap_or(0.0);

    // Total revenue
    let total_revenue: f64 =
        sqlx::query_scalar("SELECT COALESCE(SUM(price), 0) FROM user_memberships")
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

/// GET /admin/members/services - Get all services/products for filtering
#[tracing::instrument(skip(state, _admin))]
pub async fn services(
    State(state): State<AppState>,
    _admin: AdminUser,
) -> Result<Json<serde_json::Value>, ApiError> {
    // Get distinct products/services that members can subscribe to
    let services: Vec<(i64, Option<String>, Option<String>, Option<f64>, i64)> = sqlx::query_as(
        r#"
        SELECT
            COALESCE(us.product_id, us.id) as id,
            us.product_name as name,
            'subscription' as type,
            us.price,
            COUNT(DISTINCT us.user_id) as members_count
        FROM user_memberships us
        WHERE us.status = 'active'
        GROUP BY COALESCE(us.product_id, us.id), us.product_name, us.price
        ORDER BY members_count DESC
        LIMIT 50
        "#,
    )
    .fetch_all(state.db.pool())
    .await
    .unwrap_or_default();

    let services_json: Vec<serde_json::Value> = services
        .into_iter()
        .map(|(id, name, svc_type, price, count)| {
            serde_json::json!({
                "id": id,
                "name": name.unwrap_or_else(|| format!("Service {}", id)),
                "type": svc_type.unwrap_or_else(|| "subscription".to_string()),
                "price": price.unwrap_or(0.0),
                "is_active": true,
                "members_count": count
            })
        })
        .collect();

    Ok(Json(serde_json::json!({
        "services": services_json
    })))
}

/// GET /admin/members/email-templates - Get email templates for member communications
#[tracing::instrument(skip(_state, _admin))]
pub async fn email_templates(
    State(_state): State<AppState>,
    _admin: AdminUser,
) -> Result<Json<serde_json::Value>, ApiError> {
    // Return preset templates for member communications
    let preset_templates = vec![
        serde_json::json!({
            "id": "welcome",
            "name": "Welcome Email",
            "subject": "Welcome to Revolution Trading Pros!",
            "category": "onboarding",
            "is_active": true,
            "is_preset": true
        }),
        serde_json::json!({
            "id": "winback",
            "name": "Win-back Campaign",
            "subject": "We miss you! Come back with 20% off",
            "category": "retention",
            "is_active": true,
            "is_preset": true
        }),
        serde_json::json!({
            "id": "renewal_reminder",
            "name": "Renewal Reminder",
            "subject": "Your subscription is expiring soon",
            "category": "billing",
            "is_active": true,
            "is_preset": true
        }),
        serde_json::json!({
            "id": "promo",
            "name": "Promotional Offer",
            "subject": "Special offer just for you!",
            "category": "marketing",
            "is_active": true,
            "is_preset": true
        }),
        serde_json::json!({
            "id": "free_trial",
            "name": "Free Trial Invitation",
            "subject": "Try our premium features free for 14 days",
            "category": "acquisition",
            "is_active": true,
            "is_preset": true
        }),
    ];

    Ok(Json(serde_json::json!({
        "templates": [],
        "preset_templates": preset_templates
    })))
}

/// GET /admin/members/service/:id - Get members by service/product ID
/// ICT Level 7 SECURITY: All filters use parameterized queries
#[tracing::instrument(skip(state, _admin))]
pub async fn members_by_service(
    State(state): State<AppState>,
    _admin: AdminUser,
    Path(service_id): Path<i64>,
    Query(params): Query<MemberQuery>,
) -> Result<Json<serde_json::Value>, ApiError> {
    // Get service info using parameterized query
    let service_info: Option<(i64, Option<String>, Option<String>)> = sqlx::query_as(
        "SELECT DISTINCT COALESCE(product_id, id) as id, product_name, 'subscription' as type
         FROM user_memberships WHERE COALESCE(product_id, id) = $1 LIMIT 1",
    )
    .bind(service_id)
    .fetch_optional(state.db.pool())
    .await
    .map_err(|e| ApiError::database_error(&e.to_string()))?;

    let (svc_id, svc_name, svc_type) = service_info.unwrap_or((service_id, None, None));

    let per_page = params.per_page.unwrap_or(25).min(100);
    let page = params.page.unwrap_or(1).max(1);
    let offset = (page - 1) * per_page;

    // Prepare search pattern
    let search_pattern = params
        .search
        .as_ref()
        .filter(|s| !s.is_empty() && s.len() <= 100)
        .map(|s| format!("%{}%", s));

    // Validate status against allowlist
    let status_filter = params.status.as_ref().and_then(|s| {
        match s.as_str() {
            "active" | "trial" | "cancelled" | "expired" | "pending" | "past_due" => Some(s.clone()),
            _ => None,
        }
    });

    // Get members using parameterized queries
    let members: Vec<Member> = sqlx::query_as(
        r#"
        SELECT DISTINCT u.id, u.name, u.email, u.created_at, u.updated_at
        FROM users u
        JOIN user_memberships us ON u.id = us.user_id
        WHERE (us.product_id = $1 OR us.id = $1)
            AND ($2::TEXT IS NULL OR us.status = $2)
            AND ($3::TEXT IS NULL OR (u.name ILIKE $3 OR u.email ILIKE $3))
        ORDER BY u.created_at DESC
        LIMIT $4 OFFSET $5
        "#,
    )
    .bind(service_id)
    .bind(&status_filter)
    .bind(&search_pattern)
    .bind(per_page)
    .bind(offset)
    .fetch_all(state.db.pool())
    .await
    .map_err(|e| ApiError::database_error(&e.to_string()))?;

    // Get total count
    let total: i64 = sqlx::query_scalar(
        r#"
        SELECT COUNT(DISTINCT u.id)
        FROM users u
        JOIN user_memberships us ON u.id = us.user_id
        WHERE (us.product_id = $1 OR us.id = $1)
            AND ($2::TEXT IS NULL OR us.status = $2)
            AND ($3::TEXT IS NULL OR (u.name ILIKE $3 OR u.email ILIKE $3))
        "#,
    )
    .bind(service_id)
    .bind(&status_filter)
    .bind(&search_pattern)
    .fetch_one(state.db.pool())
    .await
    .unwrap_or(0);

    // Get stats
    let active_count: i64 = sqlx::query_scalar(
        r#"
        SELECT COUNT(DISTINCT u.id) FROM users u
        JOIN user_memberships us ON u.id = us.user_id
        WHERE (us.product_id = $1 OR us.id = $1) AND us.status = 'active'
        "#,
    )
    .bind(service_id)
    .fetch_one(state.db.pool())
    .await
    .unwrap_or(0);

    let trial_count: i64 = sqlx::query_scalar(
        r#"
        SELECT COUNT(DISTINCT u.id) FROM users u
        JOIN user_memberships us ON u.id = us.user_id
        WHERE (us.product_id = $1 OR us.id = $1) AND us.status = 'trial'
        "#,
    )
    .bind(service_id)
    .fetch_one(state.db.pool())
    .await
    .unwrap_or(0);

    let total_revenue: f64 = sqlx::query_scalar(
        r#"
        SELECT COALESCE(SUM(us.price), 0) FROM user_memberships us
        WHERE us.product_id = $1 OR us.id = $1
        "#,
    )
    .bind(service_id)
    .fetch_one(state.db.pool())
    .await
    .unwrap_or(0.0);

    let last_page = (total as f64 / per_page as f64).ceil() as i64;

    Ok(Json(serde_json::json!({
        "service": {
            "id": svc_id,
            "name": svc_name.unwrap_or_else(|| format!("Service {}", svc_id)),
            "type": svc_type.unwrap_or_else(|| "subscription".to_string())
        },
        "stats": {
            "total_members": total,
            "active_members": active_count,
            "trial_members": trial_count,
            "churned_members": total - active_count - trial_count,
            "total_revenue": (total_revenue * 100.0).round() / 100.0
        },
        "members": members,
        "pagination": {
            "total": total,
            "per_page": per_page,
            "current_page": page,
            "last_page": last_page.max(1)
        }
    })))
}

/// GET /admin/members/churned - Get churned/past members
/// ICT Level 7 SECURITY: All filters use parameterized queries
#[tracing::instrument(skip(state, _admin))]
pub async fn churned_members(
    State(state): State<AppState>,
    _admin: AdminUser,
    Query(params): Query<MemberQuery>,
) -> Result<Json<serde_json::Value>, ApiError> {
    let per_page = params.per_page.unwrap_or(25).min(100);
    let page = params.page.unwrap_or(1).max(1);
    let offset = (page - 1) * per_page;

    // Prepare search pattern
    let search_pattern = params
        .search
        .as_ref()
        .filter(|s| !s.is_empty() && s.len() <= 100)
        .map(|s| format!("%{}%", s));

    // Validate dates
    let date_from = params
        .date_from
        .as_ref()
        .filter(|d| is_valid_date_format(d))
        .cloned();
    let date_to = params
        .date_to
        .as_ref()
        .filter(|d| is_valid_date_format(d))
        .cloned();

    // Get churned members using parameterized queries
    let members: Vec<Member> = sqlx::query_as(
        r#"
        SELECT DISTINCT u.id, u.name, u.email, u.created_at, u.updated_at
        FROM users u
        JOIN user_memberships us ON u.id = us.user_id
        WHERE us.status IN ('cancelled', 'expired')
        AND u.id NOT IN (
            SELECT DISTINCT user_id FROM user_memberships WHERE status = 'active'
        )
        AND ($1::TEXT IS NULL OR (u.name ILIKE $1 OR u.email ILIKE $1))
        AND ($2::DATE IS NULL OR us.cancelled_at >= $2::DATE)
        AND ($3::DATE IS NULL OR us.cancelled_at <= $3::DATE + INTERVAL '1 day')
        ORDER BY us.cancelled_at DESC NULLS LAST, u.created_at DESC
        LIMIT $4 OFFSET $5
        "#,
    )
    .bind(&search_pattern)
    .bind(&date_from)
    .bind(&date_to)
    .bind(per_page)
    .bind(offset)
    .fetch_all(state.db.pool())
    .await
    .map_err(|e| ApiError::database_error(&e.to_string()))?;

    // Get total count
    let total: i64 = sqlx::query_scalar(
        r#"
        SELECT COUNT(DISTINCT u.id)
        FROM users u
        JOIN user_memberships us ON u.id = us.user_id
        WHERE us.status IN ('cancelled', 'expired')
        AND u.id NOT IN (
            SELECT DISTINCT user_id FROM user_memberships WHERE status = 'active'
        )
        AND ($1::TEXT IS NULL OR (u.name ILIKE $1 OR u.email ILIKE $1))
        AND ($2::DATE IS NULL OR us.cancelled_at >= $2::DATE)
        AND ($3::DATE IS NULL OR us.cancelled_at <= $3::DATE + INTERVAL '1 day')
        "#,
    )
    .bind(&search_pattern)
    .bind(&date_from)
    .bind(&date_to)
    .fetch_one(state.db.pool())
    .await
    .unwrap_or(0);

    // Stats
    let this_month: i64 = sqlx::query_scalar(
        "SELECT COUNT(DISTINCT user_id) FROM user_memberships
         WHERE status IN ('cancelled', 'expired')
         AND cancelled_at >= date_trunc('month', CURRENT_DATE)",
    )
    .fetch_one(state.db.pool())
    .await
    .unwrap_or(0);

    let lost_revenue: f64 = sqlx::query_scalar(
        "SELECT COALESCE(SUM(mp.price), 0)
         FROM user_memberships um
         JOIN membership_plans mp ON um.plan_id = mp.id
         WHERE um.status IN ('cancelled', 'expired')",
    )
    .fetch_one(state.db.pool())
    .await
    .unwrap_or(0.0);

    let last_page = (total as f64 / per_page as f64).ceil() as i64;

    Ok(Json(serde_json::json!({
        "stats": {
            "total_churned": total,
            "churned_this_month": this_month,
            "lost_revenue": (lost_revenue * 100.0).round() / 100.0,
            "avg_lifetime_days": 0 // Would need more complex calculation
        },
        "members": members,
        "pagination": {
            "total": total,
            "per_page": per_page,
            "current_page": page,
            "last_page": last_page.max(1)
        }
    })))
}

/// GET /admin/members/export - Export members as CSV
/// ICT Level 7 SECURITY: Status filter uses allowlist validation
#[tracing::instrument(skip(state, _admin))]
pub async fn export_members(
    State(state): State<AppState>,
    _admin: AdminUser,
    Query(params): Query<MemberQuery>,
) -> Result<axum::response::Response, ApiError> {
    use axum::http::header;
    use axum::response::IntoResponse;

    // Validate status against allowlist and select appropriate query
    let members: Vec<Member> = match params.status.as_deref() {
        Some("active") => {
            sqlx::query_as(
                r#"
                SELECT u.id, u.name, u.email, u.created_at, u.updated_at
                FROM users u
                WHERE u.id IN (SELECT DISTINCT user_id FROM user_memberships WHERE status = 'active')
                ORDER BY u.created_at DESC
                LIMIT 10000
                "#,
            )
            .fetch_all(state.db.pool())
            .await
        }
        Some("churned") => {
            sqlx::query_as(
                r#"
                SELECT u.id, u.name, u.email, u.created_at, u.updated_at
                FROM users u
                WHERE u.id IN (SELECT DISTINCT user_id FROM user_memberships WHERE status IN ('cancelled', 'expired'))
                AND u.id NOT IN (SELECT DISTINCT user_id FROM user_memberships WHERE status = 'active')
                ORDER BY u.created_at DESC
                LIMIT 10000
                "#,
            )
            .fetch_all(state.db.pool())
            .await
        }
        Some("trial") => {
            sqlx::query_as(
                r#"
                SELECT u.id, u.name, u.email, u.created_at, u.updated_at
                FROM users u
                WHERE u.id IN (SELECT DISTINCT user_id FROM user_memberships WHERE status = 'trial')
                ORDER BY u.created_at DESC
                LIMIT 10000
                "#,
            )
            .fetch_all(state.db.pool())
            .await
        }
        _ => {
            sqlx::query_as(
                r#"
                SELECT u.id, u.name, u.email, u.created_at, u.updated_at
                FROM users u
                ORDER BY u.created_at DESC
                LIMIT 10000
                "#,
            )
            .fetch_all(state.db.pool())
            .await
        }
    }
    .map_err(|e| ApiError::database_error(&e.to_string()))?;

    // Build CSV with proper escaping
    let mut csv = String::from("id,name,email,created_at,updated_at\n");
    for m in members {
        // Properly escape CSV fields (handle commas, quotes, newlines)
        let name = escape_csv_field(m.name.as_deref().unwrap_or(""));
        let email = escape_csv_field(&m.email);
        csv.push_str(&format!(
            "{},{},{},{},{}\n",
            m.id,
            name,
            email,
            m.created_at.map(|d| d.to_string()).unwrap_or_default(),
            m.updated_at.map(|d| d.to_string()).unwrap_or_default()
        ));
    }

    let response = (
        [
            (header::CONTENT_TYPE, "text/csv; charset=utf-8"),
            (
                header::CONTENT_DISPOSITION,
                "attachment; filename=\"members.csv\"",
            ),
        ],
        csv,
    );

    Ok(response.into_response())
}

/// Properly escape CSV field values
fn escape_csv_field(value: &str) -> String {
    if value.contains(',') || value.contains('"') || value.contains('\n') || value.contains('\r') {
        format!("\"{}\"", value.replace('"', "\"\""))
    } else {
        value.to_string()
    }
}

/// GET /admin/members/:id - Get single member details
#[tracing::instrument(skip(state, _admin))]
pub async fn show(
    State(state): State<AppState>,
    _admin: AdminUser,
    Path(id): Path<i64>,
) -> Result<Json<serde_json::Value>, ApiError> {
    let member: Option<Member> = sqlx::query_as(
        "SELECT id, name, email, created_at, updated_at
         FROM users WHERE id = $1",
    )
    .bind(id)
    .fetch_optional(state.db.pool())
    .await
    .map_err(|e| ApiError::database_error(&e.to_string()))?;

    match member {
        Some(m) => {
            // Get subscription info using parameterized query
            let subscriptions: Vec<serde_json::Value> = sqlx::query_as::<
                _,
                (
                    i64,
                    String,
                    Option<f64>,
                    Option<String>,
                    Option<NaiveDateTime>,
                ),
            >(
                "SELECT id, status, price, billing_period, created_at
                 FROM user_memberships WHERE user_id = $1 ORDER BY created_at DESC",
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
        }
        None => Err(ApiError::not_found("Member not found")),
    }
}

/// Build the members router
/// ICT Level 7: All routes require AdminUser authentication
pub fn router() -> Router<AppState> {
    Router::new()
        .route("/admin/members", get(index))
        .route("/admin/members/stats", get(stats))
        .route("/admin/members/services", get(services))
        .route("/admin/members/churned", get(churned_members))
        .route("/admin/members/export", get(export_members))
        .route("/admin/members/email-templates", get(email_templates))
        .route("/admin/members/service/:id", get(members_by_service))
        .route("/admin/members/:id", get(show))
}
