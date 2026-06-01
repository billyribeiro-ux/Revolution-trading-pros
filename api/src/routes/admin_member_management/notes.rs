//! Member notes — list, create, delete.

use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use serde_json::json;

use crate::middleware::admin::AdminUser;
use crate::AppState;

use super::{CreateNoteRequest, MemberNote};

/// GET /admin/member-management/:id/notes - Get member notes
/// ICT 7 SECURITY: Uses AdminUser extractor for automatic authorization
pub(super) async fn get_notes(
    State(state): State<AppState>,
    admin: AdminUser,
    Path(id): Path<i64>,
) -> Result<Json<Vec<MemberNote>>, (StatusCode, Json<serde_json::Value>)> {
    let _ = &admin; // Admin authorization handled by extractor

    let notes: Vec<MemberNote> = sqlx::query_as(
        r"
        SELECT mn.id, mn.user_id, mn.content, mn.created_by, u.name as created_by_name, mn.created_at
        FROM member_notes mn
        LEFT JOIN users u ON u.id = mn.created_by
        WHERE mn.user_id = $1
        ORDER BY mn.created_at DESC
        ",
    )
    .bind(id)
    .fetch_all(&state.db.pool)
    .await
    .unwrap_or_default();

    Ok(Json(notes))
}

/// POST /admin/member-management/:id/notes - Create note
/// ICT 7 SECURITY: Uses AdminUser extractor for automatic authorization
pub(super) async fn create_note(
    State(state): State<AppState>,
    admin: AdminUser,
    Path(id): Path<i64>,
    Json(input): Json<CreateNoteRequest>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    if input.content.trim().is_empty() {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(json!({"error": "Note content cannot be empty"})),
        ));
    }

    let note: MemberNote = sqlx::query_as(
        r"
        INSERT INTO member_notes (user_id, content, created_by, created_at)
        VALUES ($1, $2, $3, NOW())
        RETURNING id, user_id, content, created_by, NULL::text as created_by_name, created_at
        ",
    )
    .bind(id)
    .bind(&input.content)
    .bind(admin.0.id)
    .fetch_one(&state.db.pool)
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": e.to_string()})),
        )
    })?;

    Ok(Json(json!({
        "message": "Note created successfully",
        "note": note
    })))
}

/// DELETE /admin/member-management/:id/notes/:note_id - Delete note
/// ICT 7 SECURITY: Uses AdminUser extractor for automatic authorization
pub(super) async fn delete_note(
    State(state): State<AppState>,
    admin: AdminUser,
    Path((member_id, note_id)): Path<(i64, i64)>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let _ = &admin; // Admin authorization handled by extractor

    let result = sqlx::query("DELETE FROM member_notes WHERE id = $1 AND user_id = $2")
        .bind(note_id)
        .bind(member_id)
        .execute(&state.db.pool)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": e.to_string()})),
            )
        })?;

    if result.rows_affected() == 0 {
        return Err((
            StatusCode::NOT_FOUND,
            Json(json!({"error": "Note not found"})),
        ));
    }

    Ok(Json(json!({
        "message": "Note deleted successfully"
    })))
}
