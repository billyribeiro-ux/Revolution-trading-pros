//! Durable CRM operational diagnostics; security audit retention is independent.
use crate::{AppState, middleware::admin::AdminUser, utils::errors::ApiError};
use axum::{
    Json, Router,
    extract::{Path, Query, State},
    routing::{get, post},
};
use chrono::{DateTime, NaiveDate, Utc};
use serde::Deserialize;
use serde_json::{Value, json};
use sqlx::{Postgres, QueryBuilder};

const LEVELS: &[&str] = &["debug", "info", "warning", "error", "critical"];
const CATEGORIES: &[&str] = &["email", "automation", "import", "api", "webhook", "system"];
#[derive(Default, Deserialize)]
#[serde(deny_unknown_fields)]
struct Filters {
    level: Option<String>,
    category: Option<String>,
    search: Option<String>,
    date_from: Option<String>,
    date_to: Option<String>,
    before_date: Option<String>,
    page: Option<i64>,
    per_page: Option<i64>,
}
fn db(error: sqlx::Error) -> ApiError {
    tracing::error!(%error,"CRM operational logs persistence failed");
    ApiError::database_error("Unable to access CRM operational logs")
}
fn date(value: &str, end: bool) -> Result<DateTime<Utc>, ApiError> {
    if let Ok(value) = DateTime::parse_from_rfc3339(value) {
        return Ok(value.with_timezone(&Utc));
    }
    let value = NaiveDate::parse_from_str(value, "%Y-%m-%d")
        .map_err(|_| ApiError::validation_error("Use an ISO date or timestamp for log filters"))?;
    let value = if end {
        value
            .succ_opt()
            .ok_or_else(|| ApiError::validation_error("Date is outside the supported range"))?
    } else {
        value
    };
    value
        .and_hms_opt(0, 0, 0)
        .map(|value| value.and_utc())
        .ok_or_else(|| ApiError::validation_error("Invalid log filter date"))
}
impl Filters {
    fn bounds(&self) -> Result<(i64, i64, i64), ApiError> {
        if self
            .level
            .as_deref()
            .is_some_and(|value| !LEVELS.contains(&value))
            || self
                .category
                .as_deref()
                .is_some_and(|value| !CATEGORIES.contains(&value))
        {
            return Err(ApiError::validation_error("Invalid log level or category"));
        }
        if self
            .search
            .as_ref()
            .is_some_and(|value| value.contains('\0') || value.chars().count() > 500)
        {
            return Err(ApiError::validation_error(
                "Search must be at most 500 characters",
            ));
        }
        let page = self.page.unwrap_or(1);
        let size = self.per_page.unwrap_or(50);
        if page < 1 || !(1..=200).contains(&size) {
            return Err(ApiError::validation_error(
                "Page must be positive and page size 1–200",
            ));
        }
        Ok((page, size, page.saturating_sub(1).saturating_mul(size)))
    }
    fn query(&self, sql: &'static str) -> Result<QueryBuilder<Postgres>, ApiError> {
        self.bounds()?;
        let mut query = QueryBuilder::new(sql);
        query.push(" WHERE TRUE");
        if let Some(level) = &self.level {
            query.push(" AND level=").push_bind(level.clone());
        }
        if let Some(category) = &self.category {
            query.push(" AND category=").push_bind(category.clone());
        }
        if let Some(search) = &self.search {
            query
                .push(" AND message ILIKE ")
                .push_bind(format!("%{}%", crate::utils::escape_like(search)));
        }
        if let Some(from) = &self.date_from {
            query
                .push(" AND created_at>=")
                .push_bind(date(from, false)?);
        }
        if let Some(to) = &self.date_to {
            query.push(" AND created_at<").push_bind(date(to, true)?);
        }
        if let Some(before) = &self.before_date {
            query
                .push(" AND created_at<")
                .push_bind(date(before, false)?);
        }
        Ok(query)
    }
}
async fn list(
    State(state): State<AppState>,
    _admin: AdminUser,
    Query(filters): Query<Filters>,
) -> Result<Json<Value>, ApiError> {
    let (page, size, offset) = filters.bounds()?;
    let total: i64 = filters
        .query("SELECT COUNT(*) FROM crm_operational_logs")?
        .build_query_scalar()
        .fetch_one(&state.db.pool)
        .await
        .map_err(db)?;
    let mut query=filters.query("SELECT jsonb_build_object('id',id::text,'level',level,'category',category,'message',message,'context',context,'user_id',user_id::text,'contact_id',contact_id::text,'created_at',created_at) FROM crm_operational_logs")?;
    query
        .push(" ORDER BY created_at DESC,id DESC LIMIT ")
        .push_bind(size)
        .push(" OFFSET ")
        .push_bind(offset);
    let rows: Vec<Value> = query
        .build_query_scalar()
        .fetch_all(&state.db.pool)
        .await
        .map_err(db)?;
    let pages = ((total + size - 1) / size).max(1);
    Ok(Json(
        json!({"data":rows,"meta":{"current_page":page,"per_page":size,"total":total,"total_pages":pages,"last_page":pages}}),
    ))
}
async fn get_one(
    State(state): State<AppState>,
    _admin: AdminUser,
    Path(id): Path<i64>,
) -> Result<Json<Value>, ApiError> {
    let row:Value=sqlx::query_scalar("SELECT to_jsonb(entry)||jsonb_build_object('id',id::text,'user_id',user_id::text,'contact_id',contact_id::text) FROM crm_operational_logs entry WHERE id=$1").bind(id).fetch_optional(&state.db.pool).await.map_err(db)?.ok_or_else(||ApiError::not_found("Log entry not found"))?;
    Ok(Json(row))
}
async fn stats(State(state): State<AppState>, _admin: AdminUser) -> Result<Json<Value>, ApiError> {
    let counts:Value=sqlx::query_scalar("SELECT jsonb_build_object('total',(SELECT COUNT(*) FROM crm_operational_logs),'by_level',COALESCE((SELECT jsonb_object_agg(level,n) FROM (SELECT level,COUNT(*) n FROM crm_operational_logs GROUP BY level) levels),'{}'::jsonb),'by_category',COALESCE((SELECT jsonb_object_agg(category,n) FROM (SELECT category,COUNT(*) n FROM crm_operational_logs GROUP BY category) categories),'{}'::jsonb))").fetch_one(&state.db.pool).await.map_err(db)?;
    Ok(Json(counts))
}
async fn clear(
    State(state): State<AppState>,
    _admin: AdminUser,
    Json(filters): Json<Filters>,
) -> Result<Json<Value>, ApiError> {
    let deleted = filters
        .query("DELETE FROM crm_operational_logs")?
        .build()
        .execute(&state.db.pool)
        .await
        .map_err(db)?
        .rows_affected();
    Ok(Json(json!({"deleted_count":deleted})))
}
pub(super) fn router() -> Router<AppState> {
    Router::new()
        .route("/logs", get(list))
        .route("/system-logs", get(list))
        .route("/logs/stats", get(stats))
        .route("/logs/clear", post(clear))
        .route("/logs/{id}", get(get_one))
}
