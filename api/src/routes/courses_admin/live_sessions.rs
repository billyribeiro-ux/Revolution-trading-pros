//! Course live-session endpoints
//! Extracted from `courses_admin.rs` in R14-B split (2026-05-20).

use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use chrono::{DateTime, NaiveDate, NaiveTime, Utc};
use serde_json::json;
use tracing::{error, info};

use crate::middleware::admin::AdminUser;
use crate::models::course_enhanced::*;
use crate::AppState;

// ═══════════════════════════════════════════════════════════════════════════════════
// LIVE SESSION ENDPOINTS
// ═══════════════════════════════════════════════════════════════════════════════════

/// Create a live session
pub(super) async fn create_live_session(
    State(state): State<AppState>,
    AdminUser(admin): AdminUser,
    Path(course_id): Path<i64>,
    Json(input): Json<CreateLiveSessionRequest>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    info!(target: "security", event = "live_session_create", admin_id = %admin.id, course_id = %course_id, "Admin creating live session");
    let session_date =
        NaiveDate::parse_from_str(&input.session_date, "%Y-%m-%d").map_err(|_| {
            (
                StatusCode::BAD_REQUEST,
                Json(json!({"error": "Invalid date format. Use YYYY-MM-DD"})),
            )
        })?;

    let session_time: Option<NaiveTime> = input.session_time.as_ref().and_then(|t| {
        NaiveTime::parse_from_str(t, "%H:%M:%S")
            .ok()
            .or_else(|| NaiveTime::parse_from_str(t, "%H:%M").ok())
    });

    let replay_until: Option<DateTime<Utc>> = input.replay_available_until.as_ref().and_then(|d| {
        DateTime::parse_from_rfc3339(d)
            .ok()
            .map(|dt| dt.with_timezone(&Utc))
    });

    // Get max sort order
    let max_order: (Option<i32>,) =
        sqlx::query_as("SELECT MAX(sort_order) FROM course_live_sessions WHERE course_id = $1")
            .bind(course_id)
            .fetch_one(&state.db.pool)
            .await
            .unwrap_or((None,));

    let sort_order = max_order.0.unwrap_or(0) + 1;

    let session: CourseLiveSession = sqlx::query_as(
        r"INSERT INTO course_live_sessions
           (course_id, section_id, title, description, session_date, session_time,
            video_url, bunny_video_guid, thumbnail_url, replay_available_until,
            is_published, sort_order)
           VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12)
           RETURNING *",
    )
    .bind(course_id)
    .bind(input.section_id)
    .bind(&input.title)
    .bind(&input.description)
    .bind(session_date)
    .bind(session_time)
    .bind(&input.video_url)
    .bind(&input.bunny_video_guid)
    .bind(&input.thumbnail_url)
    .bind(replay_until)
    .bind(input.is_published.unwrap_or(true))
    .bind(sort_order)
    .fetch_one(&state.db.pool)
    .await
    .map_err(|e| {
        error!("Failed to create live session: {}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": e.to_string()})),
        )
    })?;

    info!(
        "Created live session: {} for course {}",
        session.title, course_id
    );

    Ok(Json(json!({
        "success": true,
        "session": {
            "id": session.id,
            "title": session.title,
            "session_date": session.session_date.to_string()
        }
    })))
}

/// Bulk create live sessions
pub(super) async fn bulk_create_live_sessions(
    State(state): State<AppState>,
    AdminUser(admin): AdminUser,
    Path(course_id): Path<i64>,
    Json(input): Json<BulkLiveSessionsRequest>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    info!(target: "security", event = "live_sessions_bulk_create", admin_id = %admin.id, course_id = %course_id, count = %input.sessions.len(), "Admin bulk creating live sessions");
    let mut created = 0;

    for (i, session) in input.sessions.iter().enumerate() {
        let session_date = match NaiveDate::parse_from_str(&session.session_date, "%Y-%m-%d") {
            Ok(d) => d,
            Err(_) => continue,
        };

        let result = sqlx::query(
            r"INSERT INTO course_live_sessions
               (course_id, section_id, title, session_date, video_url, bunny_video_guid, sort_order)
               VALUES ($1, $2, $3, $4, $5, $6, $7)",
        )
        .bind(course_id)
        .bind(input.section_id)
        .bind(&session.title)
        .bind(session_date)
        .bind(&session.video_url)
        .bind(&session.bunny_video_guid)
        .bind(i as i32 + 1)
        .execute(&state.db.pool)
        .await;

        if result.is_ok() {
            created += 1;
        }
    }

    Ok(Json(json!({
        "success": true,
        "created": created,
        "total": input.sessions.len()
    })))
}

/// Delete a live session
pub(super) async fn delete_live_session(
    State(state): State<AppState>,
    AdminUser(admin): AdminUser,
    Path((course_id, session_id)): Path<(i64, i64)>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    info!(target: "security", event = "live_session_delete", admin_id = %admin.id, course_id = %course_id, session_id = %session_id, "Admin deleting live session");
    let result = sqlx::query("DELETE FROM course_live_sessions WHERE id = $1 AND course_id = $2")
        .bind(session_id)
        .bind(course_id)
        .execute(&state.db.pool)
        .await
        .map_err(|e| {
            error!("Failed to delete live session: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": e.to_string()})),
            )
        })?;

    if result.rows_affected() == 0 {
        return Err((
            StatusCode::NOT_FOUND,
            Json(json!({"error": "Live session not found"})),
        ));
    }

    Ok(Json(json!({"success": true})))
}
