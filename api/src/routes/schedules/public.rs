// ═══════════════════════════════════════════════════════════════════════════
// TRADING ROOM SCHEDULES — PUBLIC ENDPOINTS
// Verbatim handlers from the monolithic `schedules.rs` (R28-B4 split):
//   - GET  /rooms                       → get_trading_rooms
//   - GET  /:plan_slug                  → get_schedule_by_plan
//   - GET  /:plan_slug/upcoming         → get_upcoming_events
//
// Every SQL string, audit-log line, and response shape preserved
// byte-for-byte. The ICT 11+ AUDIT tracing target/strings are pinned by
// downstream log-parsing and must not drift.
// ═══════════════════════════════════════════════════════════════════════════

use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Json,
};
use chrono::{Datelike, Utc};
use serde_json::json;

use super::helpers::day_name;
use super::types::{
    ScheduleEventResponse, ScheduleExceptionRow, ScheduleQuery, ScheduleRow, TradingRoomPlan,
    UpcomingEventResponse,
};
use crate::AppState;

/// GET /api/schedules/rooms
/// Get all trading rooms available for schedule management
pub(super) async fn get_trading_rooms(
    State(state): State<AppState>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    tracing::info!(
        target: "security_audit",
        event = "get_trading_rooms",
        "ICT 11+ AUDIT: Fetching trading rooms for schedule management"
    );

    let rooms: Vec<TradingRoomPlan> = sqlx::query_as(
        r"
        SELECT
            id,
            name,
            slug,
            metadata->>'type' as type
        FROM membership_plans
        WHERE is_active = true
          AND (metadata->>'type' = 'trading-room' OR metadata->>'type' = 'alert-service')
        ORDER BY name
        ",
    )
    .fetch_all(&state.db.pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to fetch trading rooms: {}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": "Failed to fetch trading rooms"})),
        )
    })?;

    Ok(Json(json!({
        "rooms": rooms,
        "count": rooms.len()
    })))
}

/// GET /api/schedules/:plan_slug
/// Get weekly schedule for a specific trading room
pub(super) async fn get_schedule_by_plan(
    State(state): State<AppState>,
    Path(plan_slug): Path<String>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    tracing::info!(
        target: "security_audit",
        event = "get_schedule",
        plan_slug = %plan_slug,
        "ICT 11+ AUDIT: Fetching schedule for trading room"
    );

    // Get plan info
    let plan: Option<TradingRoomPlan> = sqlx::query_as(
        "SELECT id, name, slug, metadata->>'type' as type FROM membership_plans WHERE slug = $1",
    )
    .bind(&plan_slug)
    .fetch_optional(&state.db.pool)
    .await
    .map_err(|e| {
        tracing::error!("Database error: {}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": "Database error"})),
        )
    })?;

    let plan = plan.ok_or_else(|| {
        (
            StatusCode::NOT_FOUND,
            Json(json!({"error": "Trading room not found"})),
        )
    })?;

    // Get schedules
    let schedules: Vec<ScheduleRow> = sqlx::query_as(
        r"
        SELECT
            id, plan_id, title, description, trader_name, trader_id,
            day_of_week, start_time, end_time, timezone,
            is_recurring, effective_from, effective_until,
            is_active, room_url, room_type, created_at, updated_at
        FROM trading_room_schedules
        WHERE plan_id = $1 AND is_active = true
        ORDER BY day_of_week, start_time
        ",
    )
    .bind(plan.id)
    .fetch_all(&state.db.pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to fetch schedules: {}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": "Failed to fetch schedules"})),
        )
    })?;

    let events: Vec<ScheduleEventResponse> = schedules
        .into_iter()
        .map(|s| ScheduleEventResponse {
            id: s.id,
            title: s.title,
            description: s.description,
            trader_name: s.trader_name,
            day_of_week: s.day_of_week,
            day_name: day_name(s.day_of_week),
            start_time: s.start_time.format("%H:%M").to_string(),
            end_time: s.end_time.format("%H:%M").to_string(),
            timezone: s.timezone.unwrap_or_else(|| "America/New_York".to_string()),
            room_type: s.room_type,
        })
        .collect();

    Ok(Json(json!({
        "plan": {
            "id": plan.id,
            "name": plan.name,
            "slug": plan.slug,
            "type": plan.plan_type
        },
        "timezone": "America/New_York",
        "events": events,
        "count": events.len()
    })))
}

/// GET /api/schedules/:plan_slug/upcoming
/// Get upcoming events for the next N days
pub(super) async fn get_upcoming_events(
    State(state): State<AppState>,
    Path(plan_slug): Path<String>,
    Query(query): Query<ScheduleQuery>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let days = query.days.unwrap_or(7);

    tracing::info!(
        target: "security_audit",
        event = "get_upcoming_events",
        plan_slug = %plan_slug,
        days = days,
        "ICT 11+ AUDIT: Fetching upcoming events"
    );

    // Get plan info
    let plan: Option<TradingRoomPlan> = sqlx::query_as(
        "SELECT id, name, slug, metadata->>'type' as type FROM membership_plans WHERE slug = $1",
    )
    .bind(&plan_slug)
    .fetch_optional(&state.db.pool)
    .await
    .map_err(|e| {
        tracing::error!("Database error: {}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": "Database error"})),
        )
    })?;

    let plan = plan.ok_or_else(|| {
        (
            StatusCode::NOT_FOUND,
            Json(json!({"error": "Trading room not found"})),
        )
    })?;

    // Get schedules with date calculation
    let schedules: Vec<ScheduleRow> = sqlx::query_as(
        r"
        SELECT
            id, plan_id, title, description, trader_name, trader_id,
            day_of_week, start_time, end_time, timezone,
            is_recurring, effective_from, effective_until,
            is_active, room_url, room_type, created_at, updated_at
        FROM trading_room_schedules
        WHERE plan_id = $1 AND is_active = true
        ORDER BY day_of_week, start_time
        ",
    )
    .bind(plan.id)
    .fetch_all(&state.db.pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to fetch schedules: {}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": "Failed to fetch schedules"})),
        )
    })?;

    // Get exceptions for the date range
    let today = Utc::now().date_naive();
    let end_date = today + chrono::Duration::days(days as i64);

    let exceptions: Vec<ScheduleExceptionRow> = sqlx::query_as(
        r"
        SELECT id, schedule_id, exception_date, exception_type,
               new_start_time, new_end_time, new_trader_name, reason, created_at
        FROM schedule_exceptions
        WHERE schedule_id IN (SELECT id FROM trading_room_schedules WHERE plan_id = $1)
          AND exception_date >= $2 AND exception_date <= $3
        ",
    )
    .bind(plan.id)
    .bind(today)
    .bind(end_date)
    .fetch_all(&state.db.pool)
    .await
    .unwrap_or_default();

    // Build upcoming events
    let mut upcoming_events: Vec<UpcomingEventResponse> = Vec::new();

    for day_offset in 0..=days {
        let event_date = today + chrono::Duration::days(day_offset as i64);
        let day_of_week = event_date.weekday().num_days_from_sunday() as i32;

        for schedule in &schedules {
            if schedule.day_of_week != day_of_week {
                continue;
            }

            // Check effective dates
            if let Some(from) = schedule.effective_from {
                if event_date < from {
                    continue;
                }
            }
            if let Some(until) = schedule.effective_until {
                if event_date > until {
                    continue;
                }
            }

            // Check for exceptions
            let exception = exceptions
                .iter()
                .find(|e| e.schedule_id == schedule.id && e.exception_date == event_date);

            let is_cancelled = exception
                .map(|e| e.exception_type == "cancelled")
                .unwrap_or(false);

            if is_cancelled {
                continue;
            }

            let start_time = exception
                .and_then(|e| e.new_start_time)
                .unwrap_or(schedule.start_time);
            let end_time = exception
                .and_then(|e| e.new_end_time)
                .unwrap_or(schedule.end_time);
            let trader_name = exception
                .and_then(|e| e.new_trader_name.clone())
                .or_else(|| schedule.trader_name.clone());

            let timezone = schedule
                .timezone
                .clone()
                .unwrap_or_else(|| "America/New_York".to_string());

            // Format datetime
            let date_time = format!(
                "{}T{}",
                event_date.format("%Y-%m-%d"),
                start_time.format("%H:%M:%S")
            );

            upcoming_events.push(UpcomingEventResponse {
                id: schedule.id,
                title: schedule.title.clone(),
                trader_name,
                event_date: event_date.format("%Y-%m-%d").to_string(),
                start_time: start_time.format("%H:%M").to_string(),
                end_time: end_time.format("%H:%M").to_string(),
                date_time,
                timezone,
                is_cancelled: false,
            });
        }
    }

    // Sort by date and time
    upcoming_events.sort_by(|a, b| a.date_time.cmp(&b.date_time));

    Ok(Json(json!({
        "plan": {
            "id": plan.id,
            "name": plan.name,
            "slug": plan.slug
        },
        "events": upcoming_events,
        "count": upcoming_events.len(),
        "days_ahead": days
    })))
}
