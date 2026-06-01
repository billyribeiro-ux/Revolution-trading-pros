// ═══════════════════════════════════════════════════════════════════════════
// TRADING ROOM SCHEDULES — ADMIN BULK + EXCEPTIONS
// Verbatim handlers from the monolithic `schedules.rs` (R28-B4 split):
//   - POST   /admin/schedules/bulk             → admin_bulk_schedules
//   - POST   /admin/schedules/exceptions       → admin_create_exception
//   - DELETE /admin/schedules/exceptions/:id   → admin_delete_exception
//
// `admin_bulk_schedules` is the multi-row import path used by the admin
// schedule-builder UI. The `Pool::begin()` → `tx.commit()` transaction
// wrapper is preserved per CLAUDE.md ("New mutations that touch >1 table
// need a Pool::begin() → tx.commit() transaction wrapper" — here, multi-
// row insert with optional pre-clear is the equivalent).
//
// `admin_create_exception` uses `ON CONFLICT (schedule_id, exception_date)
// DO UPDATE` — preserves the upsert semantics for re-scheduling an
// already-cancelled day.
// ═══════════════════════════════════════════════════════════════════════════

use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use serde_json::json;

use super::helpers::{parse_date, parse_time};
use super::types::{BulkScheduleRequest, CreateExceptionRequest, ScheduleExceptionRow};
use crate::{middleware::admin::AdminUser, AppState};

/// POST /api/admin/schedules/bulk
/// Bulk create/update schedules for a plan (admin)
pub(super) async fn admin_bulk_schedules(
    State(state): State<AppState>,
    admin: AdminUser,
    Json(input): Json<BulkScheduleRequest>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    tracing::info!(
        target: "security_audit",
        event = "admin_bulk_schedules",
        admin_id = %admin.0.id,
        plan_id = input.plan_id,
        schedule_count = input.schedules.len(),
        clear_existing = input.clear_existing.unwrap_or(false),
        "ICT 11+ AUDIT: Admin bulk updating schedules"
    );

    // Start transaction
    let mut tx = state.db.pool.begin().await.map_err(|e| {
        tracing::error!("Failed to start transaction: {}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": "Database error"})),
        )
    })?;

    // Clear existing if requested
    if input.clear_existing.unwrap_or(false) {
        sqlx::query("DELETE FROM trading_room_schedules WHERE plan_id = $1")
            .bind(input.plan_id)
            .execute(&mut *tx)
            .await
            .map_err(|e| {
                tracing::error!("Failed to clear existing schedules: {}", e);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({"error": "Failed to clear existing schedules"})),
                )
            })?;
    }

    // Insert new schedules
    let mut created_count = 0;
    for schedule in &input.schedules {
        let start_time = parse_time(&schedule.start_time)
            .map_err(|e| (StatusCode::BAD_REQUEST, Json(json!({"error": e}))))?;
        let end_time = parse_time(&schedule.end_time)
            .map_err(|e| (StatusCode::BAD_REQUEST, Json(json!({"error": e}))))?;

        sqlx::query(
            r"
            INSERT INTO trading_room_schedules (
                plan_id, title, description, trader_name, trader_id,
                day_of_week, start_time, end_time, timezone,
                is_recurring, room_url, room_type, created_by
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13)
            ",
        )
        .bind(input.plan_id)
        .bind(&schedule.title)
        .bind(&schedule.description)
        .bind(&schedule.trader_name)
        .bind(schedule.trader_id)
        .bind(schedule.day_of_week)
        .bind(start_time)
        .bind(end_time)
        .bind(schedule.timezone.as_deref().unwrap_or("America/New_York"))
        .bind(schedule.is_recurring.unwrap_or(true))
        .bind(&schedule.room_url)
        .bind(schedule.room_type.as_deref().unwrap_or("live"))
        .bind(admin.0.id)
        .execute(&mut *tx)
        .await
        .map_err(|e| {
            tracing::error!("Failed to create schedule: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": format!("Failed to create schedule: {}", e)})),
            )
        })?;

        created_count += 1;
    }

    // Commit transaction
    tx.commit().await.map_err(|e| {
        tracing::error!("Failed to commit transaction: {}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": "Database error"})),
        )
    })?;

    tracing::info!(
        target: "security_audit",
        event = "bulk_schedules_created",
        plan_id = input.plan_id,
        created_count = created_count,
        admin_id = %admin.0.id,
        "ICT 11+ AUDIT: Bulk schedules created successfully"
    );

    Ok(Json(json!({
        "message": "Schedules created successfully",
        "created_count": created_count
    })))
}

/// POST /api/admin/schedules/exceptions
/// Create a schedule exception (admin)
pub(super) async fn admin_create_exception(
    State(state): State<AppState>,
    admin: AdminUser,
    Json(input): Json<CreateExceptionRequest>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    tracing::info!(
        target: "security_audit",
        event = "admin_create_exception",
        admin_id = %admin.0.id,
        schedule_id = input.schedule_id,
        exception_type = %input.exception_type,
        "ICT 11+ AUDIT: Admin creating schedule exception"
    );

    let exception_date = parse_date(&input.exception_date)
        .map_err(|e| (StatusCode::BAD_REQUEST, Json(json!({"error": e}))))?;

    let new_start_time = input
        .new_start_time
        .as_ref()
        .map(|t| parse_time(t))
        .transpose()
        .map_err(|e| (StatusCode::BAD_REQUEST, Json(json!({"error": e}))))?;

    let new_end_time = input
        .new_end_time
        .as_ref()
        .map(|t| parse_time(t))
        .transpose()
        .map_err(|e| (StatusCode::BAD_REQUEST, Json(json!({"error": e}))))?;

    let exception: ScheduleExceptionRow = sqlx::query_as(
        r"
        INSERT INTO schedule_exceptions (
            schedule_id, exception_date, exception_type,
            new_start_time, new_end_time, new_trader_name, reason, created_by
        )
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
        ON CONFLICT (schedule_id, exception_date) DO UPDATE SET
            exception_type = EXCLUDED.exception_type,
            new_start_time = EXCLUDED.new_start_time,
            new_end_time = EXCLUDED.new_end_time,
            new_trader_name = EXCLUDED.new_trader_name,
            reason = EXCLUDED.reason
        RETURNING *
        ",
    )
    .bind(input.schedule_id)
    .bind(exception_date)
    .bind(&input.exception_type)
    .bind(new_start_time)
    .bind(new_end_time)
    .bind(&input.new_trader_name)
    .bind(&input.reason)
    .bind(admin.0.id)
    .fetch_one(&state.db.pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to create exception: {}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": format!("Failed to create exception: {}", e)})),
        )
    })?;

    tracing::info!(
        target: "security_audit",
        event = "exception_created",
        exception_id = exception.id,
        admin_id = %admin.0.id,
        "ICT 11+ AUDIT: Schedule exception created successfully"
    );

    Ok(Json(json!({
        "message": "Exception created successfully",
        "exception": exception
    })))
}

/// DELETE /api/admin/schedules/exceptions/:id
/// Delete a schedule exception (admin)
pub(super) async fn admin_delete_exception(
    State(state): State<AppState>,
    Path(id): Path<i64>,
    admin: AdminUser,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    tracing::info!(
        target: "security_audit",
        event = "admin_delete_exception",
        admin_id = %admin.0.id,
        exception_id = id,
        "ICT 11+ AUDIT: Admin deleting schedule exception"
    );

    let result = sqlx::query("DELETE FROM schedule_exceptions WHERE id = $1")
        .bind(id)
        .execute(&state.db.pool)
        .await
        .map_err(|e| {
            tracing::error!("Failed to delete exception: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": "Failed to delete exception"})),
            )
        })?;

    if result.rows_affected() == 0 {
        return Err((
            StatusCode::NOT_FOUND,
            Json(json!({"error": "Exception not found"})),
        ));
    }

    Ok(Json(json!({
        "message": "Exception deleted successfully"
    })))
}
