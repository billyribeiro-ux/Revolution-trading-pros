// ═══════════════════════════════════════════════════════════════════════════
// TRADING ROOM SCHEDULES API
// ICT 11+ Apple Principal Engineer Grade
// January 2026
//
// Per-room schedule management system with:
// - Public endpoints for fetching schedules
// - Admin endpoints for CRUD operations
// - Comprehensive audit logging
// - Exception handling for holidays/cancellations

#![allow(clippy::useless_format)]
// ═══════════════════════════════════════════════════════════════════════════

use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    routing::{delete, get, post, put},
    Json, Router,
};
use chrono::{Datelike, NaiveDate, NaiveTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::json;
use sqlx::FromRow;

use crate::{middleware::admin::AdminUser, AppState};

// ═══════════════════════════════════════════════════════════════════════════
// TYPE DEFINITIONS
// ═══════════════════════════════════════════════════════════════════════════

/// Schedule event from database
#[derive(Debug, Serialize, FromRow)]
pub struct ScheduleRow {
    pub id: i64,
    pub plan_id: i64,
    pub title: String,
    pub description: Option<String>,
    pub trader_name: Option<String>,
    pub trader_id: Option<i64>,
    pub day_of_week: i32,
    pub start_time: NaiveTime,
    pub end_time: NaiveTime,
    pub timezone: Option<String>,
    pub is_recurring: Option<bool>,
    pub effective_from: Option<NaiveDate>,
    pub effective_until: Option<NaiveDate>,
    pub is_active: Option<bool>,
    pub room_url: Option<String>,
    pub room_type: Option<String>,
    pub created_at: Option<chrono::NaiveDateTime>,
    pub updated_at: Option<chrono::NaiveDateTime>,
}

/// Schedule exception from database
#[derive(Debug, Serialize, FromRow)]
pub struct ScheduleExceptionRow {
    pub id: i64,
    pub schedule_id: i64,
    pub exception_date: NaiveDate,
    pub exception_type: String,
    pub new_start_time: Option<NaiveTime>,
    pub new_end_time: Option<NaiveTime>,
    pub new_trader_name: Option<String>,
    pub reason: Option<String>,
    pub created_at: Option<chrono::NaiveDateTime>,
}

/// Membership plan for dropdown
#[derive(Debug, Serialize, FromRow)]
pub struct TradingRoomPlan {
    pub id: i64,
    pub name: String,
    pub slug: String,
    #[sqlx(rename = "type")]
    pub plan_type: Option<String>,
}

/// Public schedule response for frontend
#[derive(Debug, Serialize)]
pub struct ScheduleEventResponse {
    pub id: i64,
    pub title: String,
    pub description: Option<String>,
    pub trader_name: Option<String>,
    pub day_of_week: i32,
    pub day_name: String,
    pub start_time: String,
    pub end_time: String,
    pub timezone: String,
    pub room_type: Option<String>,
}

/// Upcoming event with calculated date
#[derive(Debug, Serialize)]
pub struct UpcomingEventResponse {
    pub id: i64,
    pub title: String,
    pub trader_name: Option<String>,
    pub event_date: String,
    pub start_time: String,
    pub end_time: String,
    pub date_time: String, // Full ISO datetime
    pub timezone: String,
    pub is_cancelled: bool,
}

// ═══════════════════════════════════════════════════════════════════════════
// REQUEST/RESPONSE TYPES
// ═══════════════════════════════════════════════════════════════════════════

#[derive(Debug, Deserialize)]
pub struct ScheduleQuery {
    pub days: Option<i32>,
}

#[derive(Debug, Deserialize)]
pub struct CreateScheduleRequest {
    pub plan_id: i64,
    pub title: String,
    pub description: Option<String>,
    pub trader_name: Option<String>,
    pub trader_id: Option<i64>,
    pub day_of_week: i32,
    pub start_time: String, // HH:MM format
    pub end_time: String,   // HH:MM format
    pub timezone: Option<String>,
    pub is_recurring: Option<bool>,
    pub effective_from: Option<String>, // YYYY-MM-DD
    pub effective_until: Option<String>,
    pub room_url: Option<String>,
    pub room_type: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateScheduleRequest {
    pub title: Option<String>,
    pub description: Option<String>,
    pub trader_name: Option<String>,
    pub trader_id: Option<i64>,
    pub day_of_week: Option<i32>,
    pub start_time: Option<String>,
    pub end_time: Option<String>,
    pub timezone: Option<String>,
    pub is_recurring: Option<bool>,
    pub effective_from: Option<String>,
    pub effective_until: Option<String>,
    pub is_active: Option<bool>,
    pub room_url: Option<String>,
    pub room_type: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct CreateExceptionRequest {
    pub schedule_id: i64,
    pub exception_date: String, // YYYY-MM-DD
    pub exception_type: String, // cancelled, rescheduled, special
    pub new_start_time: Option<String>,
    pub new_end_time: Option<String>,
    pub new_trader_name: Option<String>,
    pub reason: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct BulkScheduleRequest {
    pub plan_id: i64,
    pub schedules: Vec<CreateScheduleRequest>,
    pub clear_existing: Option<bool>,
}

// ═══════════════════════════════════════════════════════════════════════════
// HELPER FUNCTIONS
// ═══════════════════════════════════════════════════════════════════════════

fn day_name(day: i32) -> String {
    match day {
        0 => "Sunday".to_string(),
        1 => "Monday".to_string(),
        2 => "Tuesday".to_string(),
        3 => "Wednesday".to_string(),
        4 => "Thursday".to_string(),
        5 => "Friday".to_string(),
        6 => "Saturday".to_string(),
        _ => "Unknown".to_string(),
    }
}

fn parse_time(time_str: &str) -> Result<NaiveTime, String> {
    NaiveTime::parse_from_str(time_str, "%H:%M")
        .or_else(|_| NaiveTime::parse_from_str(time_str, "%H:%M:%S"))
        .map_err(|e| format!("Invalid time format: {}", e))
}

fn parse_date(date_str: &str) -> Result<NaiveDate, String> {
    NaiveDate::parse_from_str(date_str, "%Y-%m-%d")
        .map_err(|e| format!("Invalid date format: {}", e))
}

// ═══════════════════════════════════════════════════════════════════════════
// PUBLIC ENDPOINTS
// ═══════════════════════════════════════════════════════════════════════════

/// GET /api/schedules/rooms
/// Get all trading rooms available for schedule management
async fn get_trading_rooms(
    State(state): State<AppState>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    tracing::info!(
        target: "security_audit",
        event = "get_trading_rooms",
        "ICT 11+ AUDIT: Fetching trading rooms for schedule management"
    );

    let rooms: Vec<TradingRoomPlan> = sqlx::query_as(
        r#"
        SELECT 
            id, 
            name, 
            slug,
            metadata->>'type' as type
        FROM membership_plans 
        WHERE is_active = true 
          AND (metadata->>'type' = 'trading-room' OR metadata->>'type' = 'alert-service')
        ORDER BY name
        "#,
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
async fn get_schedule_by_plan(
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
        r#"
        SELECT 
            id, plan_id, title, description, trader_name, trader_id,
            day_of_week, start_time, end_time, timezone,
            is_recurring, effective_from, effective_until,
            is_active, room_url, room_type, created_at, updated_at
        FROM trading_room_schedules
        WHERE plan_id = $1 AND is_active = true
        ORDER BY day_of_week, start_time
        "#,
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
async fn get_upcoming_events(
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
        r#"
        SELECT 
            id, plan_id, title, description, trader_name, trader_id,
            day_of_week, start_time, end_time, timezone,
            is_recurring, effective_from, effective_until,
            is_active, room_url, room_type, created_at, updated_at
        FROM trading_room_schedules
        WHERE plan_id = $1 AND is_active = true
        ORDER BY day_of_week, start_time
        "#,
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
        r#"
        SELECT id, schedule_id, exception_date, exception_type, 
               new_start_time, new_end_time, new_trader_name, reason, created_at
        FROM schedule_exceptions
        WHERE schedule_id IN (SELECT id FROM trading_room_schedules WHERE plan_id = $1)
          AND exception_date >= $2 AND exception_date <= $3
        "#,
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

// ═══════════════════════════════════════════════════════════════════════════
// ADMIN ENDPOINTS
// ═══════════════════════════════════════════════════════════════════════════

/// GET /api/admin/schedules
/// Get all schedules (admin)
async fn admin_list_schedules(
    State(state): State<AppState>,
    _admin: AdminUser,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    tracing::info!(
        target: "security_audit",
        event = "admin_list_schedules",
        "ICT 11+ AUDIT: Admin listing all schedules"
    );

    let schedules: Vec<ScheduleRow> = sqlx::query_as(
        r#"
        SELECT 
            s.id, s.plan_id, s.title, s.description, s.trader_name, s.trader_id,
            s.day_of_week, s.start_time, s.end_time, s.timezone,
            s.is_recurring, s.effective_from, s.effective_until,
            s.is_active, s.room_url, s.room_type, s.created_at, s.updated_at
        FROM trading_room_schedules s
        ORDER BY s.plan_id, s.day_of_week, s.start_time
        "#,
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
async fn admin_get_plan_schedules(
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
        r#"
        SELECT 
            id, plan_id, title, description, trader_name, trader_id,
            day_of_week, start_time, end_time, timezone,
            is_recurring, effective_from, effective_until,
            is_active, room_url, room_type, created_at, updated_at
        FROM trading_room_schedules
        WHERE plan_id = $1
        ORDER BY day_of_week, start_time
        "#,
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
async fn admin_create_schedule(
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
        r#"
        INSERT INTO trading_room_schedules (
            plan_id, title, description, trader_name, trader_id,
            day_of_week, start_time, end_time, timezone,
            is_recurring, effective_from, effective_until,
            room_url, room_type, created_by
        )
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15)
        RETURNING *
        "#,
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
async fn admin_update_schedule(
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
        updates.push(format!("title = ${}", param_count));
        param_count += 1;
    }
    if input.description.is_some() {
        updates.push(format!("description = ${}", param_count));
        param_count += 1;
    }
    if input.trader_name.is_some() {
        updates.push(format!("trader_name = ${}", param_count));
        param_count += 1;
    }
    if input.day_of_week.is_some() {
        updates.push(format!("day_of_week = ${}", param_count));
        param_count += 1;
    }
    if input.start_time.is_some() {
        updates.push(format!("start_time = ${}", param_count));
        param_count += 1;
    }
    if input.end_time.is_some() {
        updates.push(format!("end_time = ${}", param_count));
        param_count += 1;
    }
    if input.timezone.is_some() {
        updates.push(format!("timezone = ${}", param_count));
        param_count += 1;
    }
    if input.is_active.is_some() {
        updates.push(format!("is_active = ${}", param_count));
        param_count += 1;
    }
    if input.room_url.is_some() {
        updates.push(format!("room_url = ${}", param_count));
        param_count += 1;
    }
    if input.room_type.is_some() {
        updates.push(format!("room_type = ${}", param_count));
        param_count += 1;
    }

    updates.push(format!("updated_by = ${}", param_count));
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
    let mut query_builder = sqlx::query_as::<_, ScheduleRow>(&query);

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
async fn admin_delete_schedule(
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

/// POST /api/admin/schedules/bulk
/// Bulk create/update schedules for a plan (admin)
async fn admin_bulk_schedules(
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
            r#"
            INSERT INTO trading_room_schedules (
                plan_id, title, description, trader_name, trader_id,
                day_of_week, start_time, end_time, timezone,
                is_recurring, room_url, room_type, created_by
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13)
            "#,
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
async fn admin_create_exception(
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
        r#"
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
        "#,
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
async fn admin_delete_exception(
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

// ═══════════════════════════════════════════════════════════════════════════
// ROUTER
// ═══════════════════════════════════════════════════════════════════════════

/// Public schedule routes
pub fn router() -> Router<AppState> {
    Router::new()
        .route("/rooms", get(get_trading_rooms))
        .route("/:plan_slug", get(get_schedule_by_plan))
        .route("/:plan_slug/upcoming", get(get_upcoming_events))
}

/// Admin schedule routes
pub fn admin_router() -> Router<AppState> {
    Router::new()
        .route("/", get(admin_list_schedules).post(admin_create_schedule))
        .route("/plan/:plan_id", get(admin_get_plan_schedules))
        .route(
            "/:id",
            put(admin_update_schedule).delete(admin_delete_schedule),
        )
        .route("/bulk", post(admin_bulk_schedules))
        .route("/exceptions", post(admin_create_exception))
        .route("/exceptions/:id", delete(admin_delete_exception))
}
