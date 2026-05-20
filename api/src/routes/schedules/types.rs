// ═══════════════════════════════════════════════════════════════════════════
// TRADING ROOM SCHEDULES — TYPE DEFINITIONS
// Split out from monolithic `schedules.rs` (R28-B4). Pure structural move:
// every type, every field, every attribute identical to the pre-split source.
//
// The `schedules_test.rs` contract (R12-D) pins these on the wire:
//   - `ScheduleRow.id` / `.plan_id` / `.trader_id` are i64 (BIGSERIAL)
//   - `ScheduleRow.day_of_week` is i32 (0..=6, CLAUDE.md Reserved exception)
//   - `ScheduleExceptionRow.id` / `.schedule_id` are i64
//   - `TradingRoomPlan.plan_type` has `#[sqlx(rename = "type")]` — sqlx-only,
//     does NOT leak to serde wire (wire stays `plan_type`)
// ═══════════════════════════════════════════════════════════════════════════

use chrono::{NaiveDate, NaiveTime};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

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
