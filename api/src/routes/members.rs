//! Members Controller - ICT 11+ Principal Engineer
//!
//! Admin member management with filtering, statistics, and analytics.

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

use crate::{utils::errors::ApiError, AppState};

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

/// Sanitize user input to prevent SQL injection
/// ICT 7 SECURITY FIX: Proper escaping for dynamic SQL queries
fn sanitize_sql_string(input: &str) -> String {
    input
        .replace('\\', "\\\\") // Escape backslashes first
        .replace('\'', "''") // Escape single quotes (SQL standard)
        .replace('\0', "") // Remove null bytes
        .replace('\n', " ") // Remove newlines
        .replace('\r', " ") // Remove carriage returns
        .chars()
        .filter(|c| !c.is_control()) // Remove other control characters
        .collect()
}

/// Validate date format (YYYY-MM-DD) to prevent injection via date fields
fn is_valid_date_format(date: &str) -> bool {
    let parts: Vec<&str> = date.split('-').collect();
    if parts.len() != 3 {
        return false;
    }
    parts[0].len() == 4
        && parts[1].len() == 2
        && parts[2].len() == 2
        && parts.iter().all(|p| p.chars().all(|c| c.is_ascii_digit()))
}

/// GET /admin/members - List all members with filtering
#[tracing::instrument(skip(state))]
pub async fn index(
    State(state): State<AppState>,
    Query(params): Query<MemberQuery>,
) -> Result<Json<serde_json::Value>, ApiError> {
    let mut query = String::from(
        "SELECT id, name, email, created_at, updated_at 
         FROM users WHERE 1=1",
    );
    let mut conditions = Vec::new();

    // Search filter - ICT 7 SECURITY: Use sanitize function
    if let Some(search) = &params.search {
        let escaped = sanitize_sql_string(search);
        // Limit search length to prevent DoS
        if escaped.len() <= 100 {
            conditions.push(format!(
                "(name ILIKE '%{}%' OR email ILIKE '%{}%')",
                escaped, escaped
            ));
        }
    }

    // Date range filters - ICT 7 SECURITY: Validate date format
    if let Some(date_from) = &params.date_from {
        if is_valid_date_format(date_from) {
            conditions.push(format!("created_at >= '{}'", date_from));
        }
    }
    if let Some(date_to) = &params.date_to {
        if is_valid_date_format(date_to) {
            conditions.push(format!("created_at <= '{}'", date_to));
        }
    }

    if !conditions.is_empty() {
        query.push_str(&format!(" AND {}", conditions.join(" AND ")));
    }

    // Sorting
    let allowed_columns = ["created_at", "updated_at", "name", "email", "id"];
    let sort_by = params.sort_by.as_deref().unwrap_or("created_at");
    let sort_dir = params.sort_dir.as_deref().unwrap_or("desc");

    let sort_column = if allowed_columns.contains(&sort_by) {
        sort_by
    } else {
        "created_at"
    };
    let sort_direction = if sort_dir.eq_ignore_ascii_case("asc") {
        "ASC"
    } else {
        "DESC"
    };

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
pub async fn stats(State(state): State<AppState>) -> Result<Json<serde_json::Value>, ApiError> {
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
         AND user_id NOT IN (SELECT DISTINCT user_id FROM user_memberships WHERE status = 'active')"
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
#[tracing::instrument(skip(state))]
pub async fn services(State(state): State<AppState>) -> Result<Json<serde_json::Value>, ApiError> {
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
#[tracing::instrument(skip(_state))]
pub async fn email_templates(
    State(_state): State<AppState>,
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
/// ICT 7 FIX: Added missing endpoint that frontend expects
#[tracing::instrument(skip(state))]
pub async fn members_by_service(
    State(state): State<AppState>,
    Path(service_id): Path<i64>,
    Query(params): Query<MemberQuery>,
) -> Result<Json<serde_json::Value>, ApiError> {
    // Get service info
    let service_info: Option<(i64, Option<String>, Option<String>)> = sqlx::query_as(
        "SELECT DISTINCT COALESCE(product_id, id) as id, product_name, 'subscription' as type
         FROM user_memberships WHERE COALESCE(product_id, id) = $1 LIMIT 1",
    )
    .bind(service_id)
    .fetch_optional(state.db.pool())
    .await
    .map_err(|e| ApiError::database_error(&e.to_string()))?;

    let (svc_id, svc_name, svc_type) = service_info.unwrap_or((service_id, None, None));

    // Build query for members with this service
    let per_page = params.per_page.unwrap_or(25).min(100);
    let page = params.page.unwrap_or(1).max(1);
    let offset = (page - 1) * per_page;

    let mut conditions = vec![format!(
        "us.product_id = {} OR us.id = {}",
        service_id, service_id
    )];

    if let Some(status) = &params.status {
        conditions.push(format!("us.status = '{}'", status.replace('\'', "''")));
    }

    if let Some(search) = &params.search {
        let escaped = search.replace('\'', "''");
        conditions.push(format!(
            "(u.name ILIKE '%{}%' OR u.email ILIKE '%{}%')",
            escaped, escaped
        ));
    }

    let where_clause = conditions.join(" AND ");

    // Get members
    let members: Vec<Member> = sqlx::query_as(&format!(
        "SELECT DISTINCT u.id, u.name, u.email, u.created_at, u.updated_at
         FROM users u
         JOIN user_memberships us ON u.id = us.user_id
         WHERE {}
         ORDER BY u.created_at DESC
         LIMIT {} OFFSET {}",
        where_clause, per_page, offset
    ))
    .fetch_all(state.db.pool())
    .await
    .map_err(|e| ApiError::database_error(&e.to_string()))?;

    // Get total count
    let total: i64 = sqlx::query_scalar(&format!(
        "SELECT COUNT(DISTINCT u.id)
         FROM users u
         JOIN user_memberships us ON u.id = us.user_id
         WHERE {}",
        where_clause
    ))
    .fetch_one(state.db.pool())
    .await
    .unwrap_or(0);

    // Get stats
    let active_count: i64 = sqlx::query_scalar(&format!(
        "SELECT COUNT(DISTINCT u.id) FROM users u
         JOIN user_memberships us ON u.id = us.user_id
         WHERE ({}) AND us.status = 'active'",
        where_clause
    ))
    .fetch_one(state.db.pool())
    .await
    .unwrap_or(0);

    let trial_count: i64 = sqlx::query_scalar(&format!(
        "SELECT COUNT(DISTINCT u.id) FROM users u
         JOIN user_memberships us ON u.id = us.user_id
         WHERE ({}) AND us.status = 'trial'",
        where_clause
    ))
    .fetch_one(state.db.pool())
    .await
    .unwrap_or(0);

    let total_revenue: f64 = sqlx::query_scalar(&format!(
        "SELECT COALESCE(SUM(us.price), 0) FROM user_memberships us
         WHERE {}",
        conditions[0] // Just the service filter
    ))
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
/// ICT 7 FIX: Added missing endpoint that frontend expects
#[tracing::instrument(skip(state))]
pub async fn churned_members(
    State(state): State<AppState>,
    Query(params): Query<MemberQuery>,
) -> Result<Json<serde_json::Value>, ApiError> {
    let per_page = params.per_page.unwrap_or(25).min(100);
    let page = params.page.unwrap_or(1).max(1);
    let offset = (page - 1) * per_page;

    let mut conditions = Vec::new();

    if let Some(search) = &params.search {
        let escaped = search.replace('\'', "''");
        conditions.push(format!(
            "(u.name ILIKE '%{}%' OR u.email ILIKE '%{}%')",
            escaped, escaped
        ));
    }

    if let Some(date_from) = &params.date_from {
        conditions.push(format!(
            "us.cancelled_at >= '{}'",
            date_from.replace('\'', "''")
        ));
    }
    if let Some(date_to) = &params.date_to {
        conditions.push(format!(
            "us.cancelled_at <= '{}'",
            date_to.replace('\'', "''")
        ));
    }

    let extra_conditions = if conditions.is_empty() {
        String::new()
    } else {
        format!(" AND {}", conditions.join(" AND "))
    };

    // Get churned members (have cancelled/expired subs, no active subs)
    let members: Vec<Member> = sqlx::query_as(&format!(
        "SELECT DISTINCT u.id, u.name, u.email, u.created_at, u.updated_at
         FROM users u
         JOIN user_memberships us ON u.id = us.user_id
         WHERE us.status IN ('cancelled', 'expired')
         AND u.id NOT IN (
             SELECT DISTINCT user_id FROM user_memberships WHERE status = 'active'
         ){}
         ORDER BY us.cancelled_at DESC NULLS LAST, u.created_at DESC
         LIMIT {} OFFSET {}",
        extra_conditions, per_page, offset
    ))
    .fetch_all(state.db.pool())
    .await
    .map_err(|e| ApiError::database_error(&e.to_string()))?;

    // Get total count
    let total: i64 = sqlx::query_scalar(&format!(
        "SELECT COUNT(DISTINCT u.id)
         FROM users u
         JOIN user_memberships us ON u.id = us.user_id
         WHERE us.status IN ('cancelled', 'expired')
         AND u.id NOT IN (
             SELECT DISTINCT user_id FROM user_memberships WHERE status = 'active'
         ){}",
        extra_conditions
    ))
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
/// ICT 7 FIX: Added missing endpoint that frontend expects
#[tracing::instrument(skip(state))]
pub async fn export_members(
    State(state): State<AppState>,
    Query(params): Query<MemberQuery>,
) -> Result<axum::response::Response, ApiError> {
    use axum::http::header;
    use axum::response::IntoResponse;

    let mut conditions = Vec::new();

    if let Some(status) = &params.status {
        match status.as_str() {
            "active" => conditions.push("u.id IN (SELECT DISTINCT user_id FROM user_memberships WHERE status = 'active')".to_string()),
            "churned" => conditions.push("u.id IN (SELECT DISTINCT user_id FROM user_memberships WHERE status IN ('cancelled', 'expired')) AND u.id NOT IN (SELECT DISTINCT user_id FROM user_memberships WHERE status = 'active')".to_string()),
            "trial" => conditions.push("u.id IN (SELECT DISTINCT user_id FROM user_memberships WHERE status = 'trial')".to_string()),
            _ => {}
        }
    }

    let where_clause = if conditions.is_empty() {
        "1=1".to_string()
    } else {
        conditions.join(" AND ")
    };

    let members: Vec<Member> = sqlx::query_as(&format!(
        "SELECT u.id, u.name, u.email, u.created_at, u.updated_at
         FROM users u
         WHERE {}
         ORDER BY u.created_at DESC
         LIMIT 10000",
        where_clause
    ))
    .fetch_all(state.db.pool())
    .await
    .map_err(|e| ApiError::database_error(&e.to_string()))?;

    // Build CSV
    let mut csv = String::from("id,name,email,created_at,updated_at\n");
    for m in members {
        csv.push_str(&format!(
            "{},{},{},{},{}\n",
            m.id,
            m.name.as_deref().unwrap_or("").replace(',', ";"),
            m.email.replace(',', ";"),
            m.created_at.map(|d| d.to_string()).unwrap_or_default(),
            m.updated_at.map(|d| d.to_string()).unwrap_or_default()
        ));
    }

    let response = (
        [
            (header::CONTENT_TYPE, "text/csv"),
            (
                header::CONTENT_DISPOSITION,
                "attachment; filename=\"members.csv\"",
            ),
        ],
        csv,
    );

    Ok(response.into_response())
}

/// GET /admin/members/:id - Get single member details
#[tracing::instrument(skip(state))]
pub async fn show(
    State(state): State<AppState>,
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
            // Get subscription info
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
