//! Calendar view and schedule history (audit log) handlers.
//!
//! Two read-only handlers that surface aggregated views over the
//! schedules + releases data:
//! - `get_calendar`: window-based calendar (combines schedule+release entries)
//! - `get_history`:  paginated audit log filterable by entity

use axum::{
    extract::{Query, State},
    http::StatusCode,
    Json,
};

use crate::{models::User, AppState};

use super::{
    api_error, require_cms_editor, ApiResult, CalendarEntry, CalendarQuery, CmsScheduleHistory,
    HistoryListResponse, HistoryQuery,
};

/// Get schedule calendar view
#[utoipa::path(
    get,
    path = "/api/cms/schedules/calendar",
    params(CalendarQuery),
    responses(
        (status = 200, description = "Calendar entries", body = Vec<CalendarEntry>),
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Forbidden")
    ),
    tag = "CMS Scheduling",
    security(("bearer_auth" = []))
)]
pub(super) async fn get_calendar(
    State(state): State<AppState>,
    user: User,
    Query(query): Query<CalendarQuery>,
) -> ApiResult<Vec<CalendarEntry>> {
    require_cms_editor(&user)?;

    let entries: Vec<CalendarEntry> =
        sqlx::query_as("SELECT * FROM cms_get_schedule_calendar($1, $2, $3)")
            .bind(query.start_date)
            .bind(query.end_date)
            .bind(query.content_type_id)
            .fetch_all(&state.db.pool)
            .await
            .map_err(|e| api_error(StatusCode::INTERNAL_SERVER_ERROR, &e.to_string()))?;

    Ok(Json(entries))
}

/// Get schedule history/audit log
#[utoipa::path(
    get,
    path = "/api/cms/schedules/history",
    params(HistoryQuery),
    responses(
        (status = 200, description = "History entries", body = HistoryListResponse),
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Forbidden")
    ),
    tag = "CMS Scheduling",
    security(("bearer_auth" = []))
)]
pub(super) async fn get_history(
    State(state): State<AppState>,
    user: User,
    Query(query): Query<HistoryQuery>,
) -> ApiResult<HistoryListResponse> {
    require_cms_editor(&user)?;

    let limit = query.limit.min(100);

    let mut sql = String::from("SELECT * FROM cms_schedule_history WHERE 1=1");

    // Build parameterized query to prevent SQL injection
    let mut param_idx = 1;
    let mut conditions: Vec<(&str, String)> = Vec::new();

    if let Some(schedule_id) = query.schedule_id {
        sql.push_str(&format!(" AND schedule_id = ${param_idx}"));
        conditions.push(("uuid", schedule_id.to_string()));
        param_idx += 1;
    }
    if let Some(release_id) = query.release_id {
        sql.push_str(&format!(" AND release_id = ${param_idx}"));
        conditions.push(("uuid", release_id.to_string()));
        param_idx += 1;
    }
    if let Some(content_id) = query.content_id {
        sql.push_str(&format!(" AND content_id = ${param_idx}"));
        conditions.push(("uuid", content_id.to_string()));
        param_idx += 1;
    }
    if let Some(ref event_type) = query.event_type {
        sql.push_str(&format!(" AND event_type = ${param_idx}"));
        conditions.push(("string", event_type.clone()));
        param_idx += 1;
    }

    sql.push_str(&format!(
        " ORDER BY created_at DESC LIMIT ${} OFFSET ${}",
        param_idx,
        param_idx + 1
    ));

    // Build query with all bindings
    let mut query_builder = sqlx::query_as::<_, CmsScheduleHistory>(&sql);
    for (_, val) in &conditions {
        query_builder = query_builder.bind(val);
    }
    query_builder = query_builder.bind(limit).bind(query.offset);

    let history: Vec<CmsScheduleHistory> = query_builder
        .fetch_all(&state.db.pool)
        .await
        .map_err(|e| api_error(StatusCode::INTERNAL_SERVER_ERROR, &e.to_string()))?;

    // Get total for pagination with parameterized query
    let mut count_sql = String::from("SELECT COUNT(*) FROM cms_schedule_history WHERE 1=1");
    let mut count_param_idx = 1;

    if query.schedule_id.is_some() {
        count_sql.push_str(&format!(" AND schedule_id = ${count_param_idx}"));
        count_param_idx += 1;
    }
    if query.release_id.is_some() {
        count_sql.push_str(&format!(" AND release_id = ${count_param_idx}"));
    }

    let mut count_query = sqlx::query_as::<_, (i64,)>(&count_sql);
    if let Some(schedule_id) = query.schedule_id {
        count_query = count_query.bind(schedule_id.to_string());
    }
    if let Some(release_id) = query.release_id {
        count_query = count_query.bind(release_id.to_string());
    }

    let total: (i64,) = count_query.fetch_one(&state.db.pool).await.unwrap_or((0,));

    Ok(Json(HistoryListResponse {
        history,
        total: total.0,
        has_more: query.offset + limit < total.0,
    }))
}
