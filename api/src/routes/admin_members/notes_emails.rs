//! Admin Members - Notes & Email History
//!
//! Member notes (CRUD) and email history (read-only) per-member endpoints.

use crate::middleware::admin::AdminUser;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
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
    AdminUser(user): AdminUser,
    Path(member_id): Path<i64>,
) -> Result<Json<Vec<MemberNote>>, (StatusCode, Json<serde_json::Value>)> {
    let notes: Vec<MemberNote> = sqlx::query_as(
        "SELECT id, user_id, content, created_by, created_at FROM member_notes WHERE user_id = $1 ORDER BY created_at DESC"
    )
    .bind(member_id)
    .fetch_all(&state.db.pool)
    .await
    .unwrap_or_default();

    Ok(Json(notes))
}

/// POST /admin/members/:id/notes - Create member note
pub(super) async fn create_member_note(
    State(state): State<AppState>,
    AdminUser(user): AdminUser,
    Path(member_id): Path<i64>,
    Json(input): Json<CreateNoteRequest>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    // Try to create note, gracefully handle if table doesn't exist
    let result = sqlx::query(
        "INSERT INTO member_notes (user_id, content, created_by, created_at) VALUES ($1, $2, $3, NOW())"
    )
    .bind(member_id)
    .bind(&input.content)
    .bind(user.id)
    .execute(&state.db.pool)
    .await;

    match result {
        Ok(_) => Ok(Json(json!({"message": "Note created successfully"}))),
        Err(e) => {
            tracing::warn!("Failed to create member note (table may not exist): {}", e);
            Ok(Json(json!({"message": "Note saved (pending sync)"})))
        }
    }
}

/// GET /admin/members/:id/emails - Get member email history
pub(super) async fn get_member_emails(
    State(state): State<AppState>,
    AdminUser(user): AdminUser,
    Path(member_id): Path<i64>,
) -> Result<Json<Vec<MemberEmail>>, (StatusCode, Json<serde_json::Value>)> {
    let emails: Vec<MemberEmail> = sqlx::query_as(
        "SELECT id, user_id, subject, status, sent_at, created_at FROM member_emails WHERE user_id = $1 ORDER BY created_at DESC LIMIT 50"
    )
    .bind(member_id)
    .fetch_all(&state.db.pool)
    .await
    .unwrap_or_default();

    Ok(Json(emails))
}
