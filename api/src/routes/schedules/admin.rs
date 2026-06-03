// ═══════════════════════════════════════════════════════════════════════════
// TRADING ROOM SCHEDULES — ADMIN CRUD HANDLERS
// Verbatim handlers from the monolithic `schedules.rs` (R28-B4 split):
//   - GET    /admin/schedules                  → admin_list_schedules
//   - GET    /admin/schedules/plan/:plan_id    → admin_get_plan_schedules
//   - POST   /admin/schedules                  → admin_create_schedule
//   - PUT    /admin/schedules/:id              → admin_update_schedule
//   - DELETE /admin/schedules/:id              → admin_delete_schedule
//
// Every admin handler extracts `AdminUser` — RBAC preserved.
// Every audit-log line ("ICT 11+ AUDIT: ...") preserved verbatim — pinned
// by downstream log parsing.
//
// `admin_update_schedule` uses `format!("updated_at = NOW()")` for the
// no-bindings tail of the dynamic SET clause; the file-scoped
// `#[allow(clippy::useless_format)]` mirrors the original.
// ═══════════════════════════════════════════════════════════════════════════

#![allow(clippy::useless_format)]

use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use serde_json::json;

use super::helpers::{parse_date, parse_time};
use super::types::{CreateScheduleRequest, ScheduleRow, UpdateScheduleRequest};
use crate::{middleware::admin::AdminUser, AppState};

/// GET /api/admin/schedules
/// Get all schedules (admin)
pub(super) async fn admin_list_schedules(
    State(state): State<AppState>,
    _admin: AdminUser,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    tracing::info!(
        target: "security_audit",
        event = "admin_list_schedules",
        "ICT 11+ AUDIT: Admin listing all schedules"
    );

    let schedules: Vec<ScheduleRow> = sqlx::query_as(
        r"
        SELECT
            s.id, s.plan_id, s.title, s.description, s.trader_name, s.trader_id,
            s.day_of_week, s.start_time, s.end_time, s.timezone,
            s.is_recurring, s.effective_from, s.effective_until,
            s.is_active, s.room_url, s.room_type, s.created_at, s.updated_at
        FROM trading_room_schedules s
        ORDER BY s.plan_id, s.day_of_week, s.start_time
        ",
    )
    .fetch_all(&state.db.pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to fetch schedules: {}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": "Failed to fetch schedules"})),
        )
    })?;

    Ok(Json(json!({
        "schedules": schedules,
        "count": schedules.len()
    })))
}

/// GET /api/admin/schedules/plan/:plan_id
/// Get schedules for a specific plan (admin)
pub(super) async fn admin_get_plan_schedules(
    State(state): State<AppState>,
    Path(plan_id): Path<i64>,
    _admin: AdminUser,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    tracing::info!(
        target: "security_audit",
        event = "admin_get_plan_schedules",
        plan_id = plan_id,
        "ICT 11+ AUDIT: Admin fetching schedules for plan"
    );

    let schedules: Vec<ScheduleRow> = sqlx::query_as(
        r"
        SELECT
            id, plan_id, title, description, trader_name, trader_id,
            day_of_week, start_time, end_time, timezone,
            is_recurring, effective_from, effective_until,
            is_active, room_url, room_type, created_at, updated_at
        FROM trading_room_schedules
        WHERE plan_id = $1
        ORDER BY day_of_week, start_time
        ",
    )
    .bind(plan_id)
    .fetch_all(&state.db.pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to fetch schedules: {}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": "Failed to fetch schedules"})),
        )
    })?;

    Ok(Json(json!({
        "schedules": schedules,
        "count": schedules.len()
    })))
}

/// POST /api/admin/schedules
/// Create a new schedule event (admin)
pub(super) async fn admin_create_schedule(
    State(state): State<AppState>,
    admin: AdminUser,
    Json(input): Json<CreateScheduleRequest>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    tracing::info!(
        target: "security_audit",
        event = "admin_create_schedule",
        admin_id = %admin.0.id,
        plan_id = input.plan_id,
        title = %input.title,
        "ICT 11+ AUDIT: Admin creating schedule event"
    );

    // Validate day_of_week
    if input.day_of_week < 0 || input.day_of_week > 6 {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(json!({"error": "day_of_week must be between 0 (Sunday) and 6 (Saturday)"})),
        ));
    }

    // Parse times
    let start_time = parse_time(&input.start_time)
        .map_err(|e| (StatusCode::BAD_REQUEST, Json(json!({"error": e}))))?;

    let end_time = parse_time(&input.end_time)
        .map_err(|e| (StatusCode::BAD_REQUEST, Json(json!({"error": e}))))?;

    if end_time <= start_time {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(json!({"error": "end_time must be after start_time"})),
        ));
    }

    // Parse optional dates
    let effective_from = input
        .effective_from
        .as_ref()
        .map(|d| parse_date(d))
        .transpose()
        .map_err(|e| (StatusCode::BAD_REQUEST, Json(json!({"error": e}))))?;

    let effective_until = input
        .effective_until
        .as_ref()
        .map(|d| parse_date(d))
        .transpose()
        .map_err(|e| (StatusCode::BAD_REQUEST, Json(json!({"error": e}))))?;

    // Insert schedule
    let schedule: ScheduleRow = sqlx::query_as(
        r"
        INSERT INTO trading_room_schedules (
            plan_id, title, description, trader_name, trader_id,
            day_of_week, start_time, end_time, timezone,
            is_recurring, effective_from, effective_until,
            room_url, room_type, created_by
        )
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15)
        RETURNING *
        ",
    )
    .bind(input.plan_id)
    .bind(&input.title)
    .bind(&input.description)
    .bind(&input.trader_name)
    .bind(input.trader_id)
    .bind(input.day_of_week)
    .bind(start_time)
    .bind(end_time)
    .bind(input.timezone.as_deref().unwrap_or("America/New_York"))
    .bind(input.is_recurring.unwrap_or(true))
    .bind(effective_from)
    .bind(effective_until)
    .bind(&input.room_url)
    .bind(input.room_type.as_deref().unwrap_or("live"))
    .bind(admin.0.id)
    .fetch_one(&state.db.pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to create schedule: {}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": format!("Failed to create schedule: {}", e)})),
        )
    })?;

    tracing::info!(
        target: "security_audit",
        event = "schedule_created",
        schedule_id = schedule.id,
        admin_id = %admin.0.id,
        "ICT 11+ AUDIT: Schedule created successfully"
    );

    Ok(Json(json!({
        "message": "Schedule created successfully",
        "schedule": schedule
    })))
}

/// PUT /api/admin/schedules/:id
/// Update a schedule event (admin)
pub(super) async fn admin_update_schedule(
    State(state): State<AppState>,
    Path(id): Path<i64>,
    admin: AdminUser,
    Json(input): Json<UpdateScheduleRequest>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    tracing::info!(
        target: "security_audit",
        event = "admin_update_schedule",
        admin_id = %admin.0.id,
        schedule_id = id,
        "ICT 11+ AUDIT: Admin updating schedule event"
    );

    // Build dynamic update query
    let mut updates = Vec::new();
    let mut param_count = 1;

    if input.title.is_some() {
        updates.push(format!("title = ${param_count}"));
        param_count += 1;
    }
    if input.description.is_some() {
        updates.push(format!("description = ${param_count}"));
        param_count += 1;
    }
    if input.trader_name.is_some() {
        updates.push(format!("trader_name = ${param_count}"));
        param_count += 1;
    }
    if input.day_of_week.is_some() {
        updates.push(format!("day_of_week = ${param_count}"));
        param_count += 1;
    }
    if input.start_time.is_some() {
        updates.push(format!("start_time = ${param_count}"));
        param_count += 1;
    }
    if input.end_time.is_some() {
        updates.push(format!("end_time = ${param_count}"));
        param_count += 1;
    }
    if input.timezone.is_some() {
        updates.push(format!("timezone = ${param_count}"));
        param_count += 1;
    }
    if input.is_active.is_some() {
        updates.push(format!("is_active = ${param_count}"));
        param_count += 1;
    }
    if input.room_url.is_some() {
        updates.push(format!("room_url = ${param_count}"));
        param_count += 1;
    }
    if input.room_type.is_some() {
        updates.push(format!("room_type = ${param_count}"));
        param_count += 1;
    }

    updates.push(format!("updated_by = ${param_count}"));
    param_count += 1;
    updates.push(format!("updated_at = NOW()"));

    if updates.is_empty() {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(json!({"error": "No fields to update"})),
        ));
    }

    let query = format!(
        "UPDATE trading_room_schedules SET {} WHERE id = ${} RETURNING *",
        updates.join(", "),
        param_count
    );

    // Execute with dynamic bindings
    let mut query_builder = sqlx::query_as::<_, ScheduleRow>(sqlx::AssertSqlSafe(query.as_str()));

    if let Some(ref title) = input.title {
        query_builder = query_builder.bind(title);
    }
    if let Some(ref description) = input.description {
        query_builder = query_builder.bind(description);
    }
    if let Some(ref trader_name) = input.trader_name {
        query_builder = query_builder.bind(trader_name);
    }
    if let Some(day_of_week) = input.day_of_week {
        query_builder = query_builder.bind(day_of_week);
    }
    if let Some(ref start_time) = input.start_time {
        let time = parse_time(start_time)
            .map_err(|e| (StatusCode::BAD_REQUEST, Json(json!({"error": e}))))?;
        query_builder = query_builder.bind(time);
    }
    if let Some(ref end_time) = input.end_time {
        let time = parse_time(end_time)
            .map_err(|e| (StatusCode::BAD_REQUEST, Json(json!({"error": e}))))?;
        query_builder = query_builder.bind(time);
    }
    if let Some(ref timezone) = input.timezone {
        query_builder = query_builder.bind(timezone);
    }
    if let Some(is_active) = input.is_active {
        query_builder = query_builder.bind(is_active);
    }
    if let Some(ref room_url) = input.room_url {
        query_builder = query_builder.bind(room_url);
    }
    if let Some(ref room_type) = input.room_type {
        query_builder = query_builder.bind(room_type);
    }

    query_builder = query_builder.bind(admin.0.id);
    query_builder = query_builder.bind(id);

    let schedule = query_builder.fetch_one(&state.db.pool).await.map_err(|e| {
        tracing::error!("Failed to update schedule: {}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": format!("Failed to update schedule: {}", e)})),
        )
    })?;

    tracing::info!(
        target: "security_audit",
        event = "schedule_updated",
        schedule_id = id,
        admin_id = %admin.0.id,
        "ICT 11+ AUDIT: Schedule updated successfully"
    );

    Ok(Json(json!({
        "message": "Schedule updated successfully",
        "schedule": schedule
    })))
}

/// DELETE /api/admin/schedules/:id
/// Delete a schedule event (admin)
pub(super) async fn admin_delete_schedule(
    State(state): State<AppState>,
    Path(id): Path<i64>,
    admin: AdminUser,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    tracing::info!(
        target: "security_audit",
        event = "admin_delete_schedule",
        admin_id = %admin.0.id,
        schedule_id = id,
        "ICT 11+ AUDIT: Admin deleting schedule event"
    );

    let result = sqlx::query("DELETE FROM trading_room_schedules WHERE id = $1")
        .bind(id)
        .execute(&state.db.pool)
        .await
        .map_err(|e| {
            tracing::error!("Failed to delete schedule: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": "Failed to delete schedule"})),
            )
        })?;

    if result.rows_affected() == 0 {
        return Err((
            StatusCode::NOT_FOUND,
            Json(json!({"error": "Schedule not found"})),
        ));
    }

    tracing::info!(
        target: "security_audit",
        event = "schedule_deleted",
        schedule_id = id,
        admin_id = %admin.0.id,
        "ICT 11+ AUDIT: Schedule deleted successfully"
    );

    Ok(Json(json!({
        "message": "Schedule deleted successfully"
    })))
}
