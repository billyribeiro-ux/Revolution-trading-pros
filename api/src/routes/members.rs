//! Members Controller - ICT 11+ Principal Engineer
//!
//! Admin member management with filtering, statistics, and analytics.
//! SECURITY: All queries use parameterized bindings to prevent SQL injection.

#![allow(clippy::collapsible_str_replace)]
#![allow(clippy::type_complexity)]

use axum::{
    Router,
    extract::{Path, Query, State},
    response::Json,
    routing::get,
};
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

use crate::{AppState, middleware::admin::AdminUser, utils::errors::ApiError};

#[derive(Debug, Serialize, FromRow)]
pub struct Member {
    pub id: i64,
    pub name: Option<String>,
    pub email: String,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct MemberQuery {
    pub segment: Option<i64>,
    pub tag: Option<i64>,
    pub spending_tier: Option<String>,
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
    use crate::services::member_segmentation as evaluation;
    let size = params.per_page.unwrap_or(25);
    let page = params.page.unwrap_or(1);
    if !(1..=100).contains(&size) || !(1..=10_000).contains(&page) {
        return Err(ApiError::validation_error(
            "Invalid member page or page size",
        ));
    }
    let selection =
        super::member_selection::MemberSelection::new(&state.db.pool, params.clone()).await?;
    let build = |prefix| selection.query(prefix);
    let total: i64 = build("SELECT COUNT(*)")?
        .build_query_scalar()
        .fetch_one(&state.db.pool)
        .await
        .map_err(evaluation::storage)?;
    let mut query = build(
        "SELECT to_jsonb(m) || jsonb_build_object('status_label',initcap(replace(m.status,'_',' ')))",
    )?;
    let (sort, direction) = selection.order()?;
    query
        .push(" ORDER BY ")
        .push(sort)
        .push(direction)
        .push(",m.id LIMIT ")
        .push_bind(size)
        .push(" OFFSET ")
        .push_bind((page - 1) * size);
    let members: Vec<serde_json::Value> = query
        .build_query_scalar()
        .fetch_all(&state.db.pool)
        .await
        .map_err(evaluation::storage)?;
    Ok(Json(
        serde_json::json!({"members":members,"pagination":{"total":total,"per_page":size,"current_page":page,"last_page":((total+size-1)/size).max(1)}}),
    ))
}

/// GET /admin/members/stats - Comprehensive member statistics
#[tracing::instrument(skip(state, _admin))]
pub async fn stats(
    State(state): State<AppState>,
    _admin: AdminUser,
) -> Result<Json<serde_json::Value>, ApiError> {
    Ok(Json(super::member_statistics::load(&state).await?))
}

/// GET /admin/members/services - Paged membership-plan catalog, including inactive plans.
#[tracing::instrument(skip(state, _admin))]
pub async fn services(
    State(state): State<AppState>,
    _admin: AdminUser,
    Query(params): Query<super::member_services::CatalogQuery>,
) -> Result<Json<serde_json::Value>, ApiError> {
    Ok(Json(super::member_services::catalog(&state, params).await?))
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

/// GET /admin/members/service/:id - Exact membership-plan-local cohort.
#[tracing::instrument(skip(state, _admin))]
pub async fn members_by_service(
    State(state): State<AppState>,
    _admin: AdminUser,
    Path(service_id): Path<i64>,
    Query(params): Query<super::member_services::ServiceMemberQuery>,
) -> Result<Json<serde_json::Value>, ApiError> {
    Ok(Json(
        super::member_services::members(&state, service_id, params).await?,
    ))
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
        .map(|s| format!("%{s}%"));

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
        r"
        SELECT u.id, u.name, u.email, u.created_at, u.updated_at
        FROM users u
        JOIN user_memberships us ON u.id = us.user_id
        WHERE u.deleted_at IS NULL AND COALESCE(LOWER(u.role),'')<>'deleted' AND us.status IN ('cancelled', 'expired')
        AND u.id NOT IN (
            SELECT DISTINCT user_id FROM user_memberships WHERE status = 'active'
        )
        AND ($1::TEXT IS NULL OR (u.name ILIKE $1 OR u.email ILIKE $1))
        AND ($2::DATE IS NULL OR us.cancelled_at >= $2::DATE)
        AND ($3::DATE IS NULL OR us.cancelled_at <= $3::DATE + INTERVAL '1 day')
        GROUP BY u.id, u.name, u.email, u.created_at, u.updated_at
        ORDER BY MAX(us.cancelled_at) DESC NULLS LAST, u.created_at DESC
        LIMIT $4 OFFSET $5
        ",
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
        r"
        SELECT COUNT(DISTINCT u.id)
        FROM users u
        JOIN user_memberships us ON u.id = us.user_id
        WHERE u.deleted_at IS NULL AND COALESCE(LOWER(u.role),'')<>'deleted' AND us.status IN ('cancelled', 'expired')
        AND u.id NOT IN (
            SELECT DISTINCT user_id FROM user_memberships WHERE status = 'active'
        )
        AND ($1::TEXT IS NULL OR (u.name ILIKE $1 OR u.email ILIKE $1))
        AND ($2::DATE IS NULL OR us.cancelled_at >= $2::DATE)
        AND ($3::DATE IS NULL OR us.cancelled_at <= $3::DATE + INTERVAL '1 day')
        ",
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

    let lost_revenue_cents: i64 = sqlx::query_scalar(
        r"SELECT COALESCE(SUM((mp.price * 100)::BIGINT), 0)::BIGINT
           FROM user_memberships um
           JOIN membership_plans mp ON um.plan_id = mp.id
           WHERE um.status IN ('cancelled', 'expired')",
    )
    .fetch_one(state.db.pool())
    .await
    .unwrap_or(0);

    let last_page = (total as f64 / per_page as f64).ceil() as i64;

    Ok(Json(serde_json::json!({
        "stats": {
            "total_churned": total,
            "churned_this_month": this_month,
            "lost_revenue_cents": lost_revenue_cents,
            "avg_lifetime_days": 0
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

/// GET /admin/members/export - CSV snapshot with the established column layout.
pub async fn export_members(
    state: State<AppState>,
    admin: AdminUser,
    query: Query<crate::routes::admin_member_management::ExportQuery>,
) -> Result<axum::response::Response, ApiError> {
    crate::routes::admin_member_management::export::export_members_csv(state, admin, query).await
}

/// GET /admin/members/:id - Get single member details
#[tracing::instrument(skip(state, admin))]
pub async fn show(
    state: State<AppState>,
    admin: AdminUser,
    path: Path<i64>,
) -> Result<axum::response::Response, ApiError> {
    crate::routes::admin_member_management::read_handlers::show(state, admin, path).await
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
        .route("/admin/members/service/{id}", get(members_by_service))
        .route("/admin/members/{id}", get(show))
}
