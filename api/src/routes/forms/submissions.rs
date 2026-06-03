//! Submission management handlers (admin).
//!
//! Per-form submission list / get / delete, plus bulk status update,
//! bulk delete, and CSV export. The bulk delete + per-row delete each
//! wrap submission rows + the `forms.submission_count` counter UPDATE
//! in a single transaction so the counter cannot drift on a crash.

use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Json,
};
use serde_json::json;

use super::types::{
    BulkDeleteRequest, BulkUpdateStatusRequest, ExportQuery, FormRow, FormSubmissionRow,
    SubmissionListQuery, UpdateSubmissionStatusRequest,
};
use crate::{middleware::admin::AdminUser, AppState};

/// List form submissions (admin)
pub(super) async fn list_submissions(
    State(state): State<AppState>,
    AdminUser(_user): AdminUser,
    Path(form_id): Path<i64>,
    Query(query): Query<SubmissionListQuery>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let page = query.page.unwrap_or(1).max(1);
    let per_page = query.per_page.unwrap_or(20).min(100);
    let offset = (page - 1) * per_page;

    let submissions: Vec<FormSubmissionRow> = sqlx::query_as(
        r"
        SELECT id, form_id, data, COALESCE(status, 'unread') as status, ip_address, user_agent, created_at
        FROM form_submissions
        WHERE form_id = $1
          AND ($4::text IS NULL OR status = $4)
        ORDER BY created_at DESC
        LIMIT $2 OFFSET $3
        ",
    )
    .bind(form_id)
    .bind(per_page)
    .bind(offset)
    .bind(query.status.as_deref())
    .fetch_all(&state.db.pool)
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": e.to_string()})),
        )
    })?;

    let total: (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM form_submissions WHERE form_id = $1 AND ($2::text IS NULL OR status = $2)"
    )
        .bind(form_id)
        .bind(query.status.as_deref())
        .fetch_one(&state.db.pool)
        .await
        .unwrap_or((0,));

    Ok(Json(json!({
        "submissions": submissions,
        "data": submissions,
        "meta": {
            "current_page": page,
            "per_page": per_page,
            "total": total.0,
            "total_pages": (total.0 as f64 / per_page as f64).ceil() as i64
        }
    })))
}

/// Get single submission (admin)
pub(super) async fn get_submission(
    State(state): State<AppState>,
    AdminUser(_user): AdminUser,
    Path((form_id, submission_id)): Path<(i64, i64)>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let submission: FormSubmissionRow = sqlx::query_as(
        "SELECT id, form_id, data, ip_address, user_agent, created_at FROM form_submissions WHERE id = $1 AND form_id = $2"
    )
    .bind(submission_id)
    .bind(form_id)
    .fetch_optional(&state.db.pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()}))))?
    .ok_or_else(|| (StatusCode::NOT_FOUND, Json(json!({"error": "Submission not found"}))))?;

    Ok(Json(json!({"data": submission})))
}

/// Delete submission (admin)
pub(super) async fn delete_submission(
    State(state): State<AppState>,
    AdminUser(user): AdminUser,
    Path((form_id, submission_id)): Path<(i64, i64)>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    tracing::info!(
        target: "security",
        event = "submission_delete",
        user_id = %user.id,
        form_id = %form_id,
        submission_id = %submission_id,
        "Admin deleting submission"
    );

    // ICT 7 SAFETY: the DELETE on form_submissions and the counter UPDATE on
    // forms must agree atomically — otherwise the counter drifts forever on a
    // crash between the two statements.
    let mut tx = state.db.pool.begin().await.map_err(|e| {
        tracing::error!("tx start (delete_submission): {}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": "Database error"})),
        )
    })?;

    let result = sqlx::query("DELETE FROM form_submissions WHERE id = $1 AND form_id = $2")
        .bind(submission_id)
        .bind(form_id)
        .execute(&mut *tx)
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
            Json(json!({"error": "Submission not found"})),
        ));
    }

    // Update submission count
    sqlx::query("UPDATE forms SET submission_count = submission_count - 1 WHERE id = $1")
        .bind(form_id)
        .execute(&mut *tx)
        .await
        .map_err(|e| {
            tracing::error!("Failed to decrement form submission_count: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": "Failed to update submission count"})),
            )
        })?;

    tx.commit().await.map_err(|e| {
        tracing::error!("tx commit (delete_submission): {}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": "Database error"})),
        )
    })?;

    Ok(Json(json!({"message": "Submission deleted successfully"})))
}

/// Update submission status (admin) - ICT 7 Fix: Complete submission management
pub(super) async fn update_submission_status(
    State(state): State<AppState>,
    AdminUser(user): AdminUser,
    Path((form_id, submission_id)): Path<(i64, i64)>,
    Json(input): Json<UpdateSubmissionStatusRequest>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    tracing::info!(
        target: "security",
        event = "submission_status_update",
        user_id = %user.id,
        form_id = %form_id,
        submission_id = %submission_id,
        new_status = %input.status,
        "Admin updating submission status"
    );

    // Validate status value
    let valid_statuses = [
        "unread", "read", "starred", "archived", "spam", "complete", "partial",
    ];
    if !valid_statuses.contains(&input.status.as_str()) {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(json!({"error": "Invalid status value"})),
        ));
    }

    let result =
        sqlx::query("UPDATE form_submissions SET status = $1 WHERE id = $2 AND form_id = $3")
            .bind(&input.status)
            .bind(submission_id)
            .bind(form_id)
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
            Json(json!({"error": "Submission not found"})),
        ));
    }

    Ok(Json(json!({
        "success": true,
        "message": "Submission status updated"
    })))
}

/// Bulk update submission status (admin) - ICT 7 Fix
pub(super) async fn bulk_update_status(
    State(state): State<AppState>,
    AdminUser(user): AdminUser,
    Path(form_id): Path<i64>,
    Json(input): Json<BulkUpdateStatusRequest>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    tracing::info!(
        target: "security",
        event = "bulk_status_update",
        user_id = %user.id,
        form_id = %form_id,
        count = %input.submission_ids.len(),
        new_status = %input.status,
        "Admin bulk updating submission status"
    );

    // Validate status value
    let valid_statuses = [
        "unread", "read", "starred", "archived", "spam", "complete", "partial",
    ];
    if !valid_statuses.contains(&input.status.as_str()) {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(json!({"error": "Invalid status value"})),
        ));
    }

    if input.submission_ids.is_empty() {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(json!({"error": "No submission IDs provided"})),
        ));
    }

    // Build IN clause for submission IDs
    let placeholders: Vec<String> = input
        .submission_ids
        .iter()
        .enumerate()
        .map(|(i, _)| format!("${}", i + 3))
        .collect();

    let query = format!(
        "UPDATE form_submissions SET status = $1 WHERE form_id = $2 AND id IN ({})",
        placeholders.join(", ")
    );

    let mut q = sqlx::query(sqlx::AssertSqlSafe(query.as_str()))
        .bind(&input.status)
        .bind(form_id);

    for id in &input.submission_ids {
        q = q.bind(id);
    }

    let result = q.execute(&state.db.pool).await.map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": e.to_string()})),
        )
    })?;

    Ok(Json(json!({
        "success": true,
        "message": format!("{} submissions updated", result.rows_affected())
    })))
}

/// Bulk delete submissions (admin) - ICT 7 Fix
pub(super) async fn bulk_delete_submissions(
    State(state): State<AppState>,
    AdminUser(user): AdminUser,
    Path(form_id): Path<i64>,
    Json(input): Json<BulkDeleteRequest>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    tracing::info!(
        target: "security",
        event = "bulk_delete",
        user_id = %user.id,
        form_id = %form_id,
        count = %input.submission_ids.len(),
        "Admin bulk deleting submissions"
    );

    if input.submission_ids.is_empty() {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(json!({"error": "No submission IDs provided"})),
        ));
    }

    // Build IN clause for submission IDs
    let placeholders: Vec<String> = input
        .submission_ids
        .iter()
        .enumerate()
        .map(|(i, _)| format!("${}", i + 2))
        .collect();

    let query = format!(
        "DELETE FROM form_submissions WHERE form_id = $1 AND id IN ({})",
        placeholders.join(", ")
    );

    let mut q = sqlx::query(sqlx::AssertSqlSafe(query.as_str())).bind(form_id);

    for id in &input.submission_ids {
        q = q.bind(id);
    }

    // ICT 7 SAFETY: bulk DELETE on form_submissions plus the counter UPDATE on
    // forms must commit atomically; otherwise a crash between them permanently
    // desyncs the counter.
    let mut tx = state.db.pool.begin().await.map_err(|e| {
        tracing::error!("tx start (bulk_delete_submissions): {}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": "Database error"})),
        )
    })?;

    let result = q.execute(&mut *tx).await.map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": e.to_string()})),
        )
    })?;

    // Update submission count
    sqlx::query("UPDATE forms SET submission_count = submission_count - $1 WHERE id = $2")
        .bind(result.rows_affected() as i32)
        .bind(form_id)
        .execute(&mut *tx)
        .await
        .map_err(|e| {
            tracing::error!("Failed to decrement form submission_count (bulk): {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": "Failed to update submission count"})),
            )
        })?;

    tx.commit().await.map_err(|e| {
        tracing::error!("tx commit (bulk_delete_submissions): {}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": "Database error"})),
        )
    })?;

    Ok(Json(json!({
        "success": true,
        "message": format!("{} submissions deleted", result.rows_affected())
    })))
}

/// Export submissions as CSV (admin) - ICT 7 Fix
pub(super) async fn export_submissions(
    State(state): State<AppState>,
    AdminUser(user): AdminUser,
    Path(form_id): Path<i64>,
    Query(query): Query<ExportQuery>,
) -> Result<
    (
        StatusCode,
        [(axum::http::header::HeaderName, String); 2],
        String,
    ),
    (StatusCode, Json<serde_json::Value>),
> {
    tracing::info!(
        target: "security",
        event = "submission_export",
        user_id = %user.id,
        form_id = %form_id,
        format = ?query.format,
        "Admin exporting submissions"
    );

    let submissions: Vec<FormSubmissionRow> = sqlx::query_as(
        r"
        SELECT id, form_id, data, COALESCE(status, 'unread') as status, ip_address, user_agent, created_at
        FROM form_submissions
        WHERE form_id = $1
        ORDER BY created_at DESC
        ",
    )
    .bind(form_id)
    .fetch_all(&state.db.pool)
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": e.to_string()})),
        )
    })?;

    // Get form to know field names
    let form: Option<FormRow> = sqlx::query_as(
        "SELECT id, name, slug, description, fields, settings, is_published, submission_count, created_at, updated_at FROM forms WHERE id = $1"
    )
    .bind(form_id)
    .fetch_optional(&state.db.pool)
    .await
    .ok()
    .flatten();

    let form = form.ok_or_else(|| {
        (
            StatusCode::NOT_FOUND,
            Json(json!({"error": "Form not found"})),
        )
    })?;

    // Build CSV
    let mut csv = String::new();

    // Extract field names from form fields
    let headers = ["ID", "Status", "IP Address", "User Agent", "Submitted At"];
    let field_names: Vec<String> = if let Some(fields) = form.fields.as_array() {
        fields
            .iter()
            .filter_map(|f| f.get("name").and_then(|n| n.as_str()))
            .map(|s| s.to_string())
            .collect()
    } else {
        vec![]
    };

    // CSV Header
    csv.push_str(&headers.join(","));
    for name in &field_names {
        csv.push(',');
        csv.push_str(&escape_csv(name));
    }
    csv.push('\n');

    // CSV Rows
    for sub in submissions {
        csv.push_str(&format!("{}", sub.id));
        csv.push(',');
        csv.push_str(&escape_csv(sub.status.as_deref().unwrap_or("unread")));
        csv.push(',');
        csv.push_str(&escape_csv(sub.ip_address.as_deref().unwrap_or("")));
        csv.push(',');
        csv.push_str(&escape_csv(sub.user_agent.as_deref().unwrap_or("")));
        csv.push(',');
        csv.push_str(&sub.created_at.format("%Y-%m-%d %H:%M:%S").to_string());

        // Add field values
        for name in &field_names {
            csv.push(',');
            if let Some(val) = sub.data.get(name) {
                let val_str = match val {
                    serde_json::Value::String(s) => s.clone(),
                    serde_json::Value::Null => String::new(),
                    _ => val.to_string(),
                };
                csv.push_str(&escape_csv(&val_str));
            }
        }
        csv.push('\n');
    }

    let filename = format!("form-{form_id}-submissions.csv");

    Ok((
        StatusCode::OK,
        [
            (axum::http::header::CONTENT_TYPE, "text/csv".to_string()),
            (
                axum::http::header::CONTENT_DISPOSITION,
                format!("attachment; filename=\"{filename}\""),
            ),
        ],
        csv,
    ))
}

/// Helper function to escape CSV values
fn escape_csv(value: &str) -> String {
    if value.contains(',') || value.contains('"') || value.contains('\n') {
        format!("\"{}\"", value.replace('"', "\"\""))
    } else {
        value.to_string()
    }
}
