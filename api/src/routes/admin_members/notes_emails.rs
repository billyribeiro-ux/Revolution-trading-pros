//! Admin Members - Notes & Email History
//!
//! Member notes (CRUD) and email history (read-only) per-member endpoints.

use crate::middleware::admin::AdminUser;
use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use sqlx::FromRow;

use crate::AppState;

#[derive(Debug, Serialize, FromRow)]
pub struct MemberNote {
    pub id: i64,
    pub user_id: i64,
    pub content: String,
    pub created_by: Option<i64>,
    pub created_at: chrono::NaiveDateTime,
}

#[derive(Debug, Deserialize)]
pub struct CreateNoteRequest {
    pub content: String,
}

#[derive(Debug, Serialize, FromRow)]
pub struct MemberEmail {
    pub id: i64,
    pub user_id: i64,
    pub subject: String,
    pub status: String,
    pub sent_at: Option<chrono::NaiveDateTime>,
    pub created_at: chrono::NaiveDateTime,
}

/// GET /admin/members/:id/notes - Get member notes
pub(super) async fn get_member_notes(
    State(state): State<AppState>,
    AdminUser(_user): AdminUser,
    Path(member_id): Path<i64>,
) -> Result<Json<Vec<MemberNote>>, (StatusCode, Json<serde_json::Value>)> {
    let notes: Vec<MemberNote> = sqlx::query_as(
        r"SELECT id, user_id, note AS content, created_by,
                 (created_at AT TIME ZONE 'UTC') AS created_at
          FROM member_notes
          WHERE user_id = $1
          ORDER BY created_at DESC",
    )
    .bind(member_id)
    .fetch_all(&state.db.pool)
    .await
    .map_err(crate::utils::errors::db_error)?;

    Ok(Json(notes))
}

/// POST /admin/members/:id/notes - Create member note
pub(super) async fn create_member_note(
    State(state): State<AppState>,
    AdminUser(user): AdminUser,
    Path(member_id): Path<i64>,
    Json(input): Json<CreateNoteRequest>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    sqlx::query(
        "INSERT INTO member_notes (user_id, note, created_by, created_at, updated_at) VALUES ($1, $2, $3, NOW(), NOW())",
    )
    .bind(member_id)
    .bind(&input.content)
    .bind(user.id)
    .execute(&state.db.pool)
    .await
    .map_err(crate::utils::errors::db_error)?;

    Ok(Json(json!({"message": "Note created successfully"})))
}

/// GET /admin/members/:id/emails - Get member email history
pub(super) async fn get_member_emails(
    State(state): State<AppState>,
    AdminUser(_user): AdminUser,
    Path(member_id): Path<i64>,
) -> Result<Json<Vec<MemberEmail>>, (StatusCode, Json<serde_json::Value>)> {
    let emails: Vec<MemberEmail> = sqlx::query_as(
        r"SELECT id, COALESCE(user_id, $1) AS user_id, subject, status,
                 (sent_at AT TIME ZONE 'UTC') AS sent_at,
                 created_at
          FROM email_logs
          WHERE user_id = $1
          ORDER BY created_at DESC
          LIMIT 50",
    )
    .bind(member_id)
    .fetch_all(&state.db.pool)
    .await
    .map_err(crate::utils::errors::db_error)?;

    Ok(Json(emails))
}
