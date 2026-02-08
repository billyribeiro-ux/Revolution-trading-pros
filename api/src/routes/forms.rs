//! Forms Admin Routes - Revolution Trading Pros
//! Apple ICT 7+ Principal Engineer Grade - January 2026
//!
//! Full CRUD for forms and form submissions with admin protection.

use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    routing::{delete, get, post, put},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::{middleware::admin::AdminUser, AppState};

// ═══════════════════════════════════════════════════════════════════════════
// DATA TYPES
// ═══════════════════════════════════════════════════════════════════════════

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct FormRow {
    pub id: i64,
    pub name: String,
    pub slug: String,
    pub description: Option<String>,
    pub fields: serde_json::Value,
    pub settings: serde_json::Value,
    pub is_published: bool,
    pub submission_count: i32,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct FormSubmissionRow {
    pub id: i64,
    pub form_id: i64,
    pub data: serde_json::Value,
    pub status: Option<String>,
    pub ip_address: Option<String>,
    pub user_agent: Option<String>,
    pub created_at: chrono::NaiveDateTime,
}

#[derive(Debug, Deserialize)]
pub struct FormListQuery {
    pub page: Option<i64>,
    pub per_page: Option<i64>,
    pub search: Option<String>,
    pub is_published: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct CreateFormRequest {
    pub name: String,
    pub description: Option<String>,
    pub fields: serde_json::Value,
    pub settings: Option<serde_json::Value>,
    pub is_published: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateFormRequest {
    pub name: Option<String>,
    pub description: Option<String>,
    pub fields: Option<serde_json::Value>,
    pub settings: Option<serde_json::Value>,
    pub is_published: Option<bool>,
}

// ═══════════════════════════════════════════════════════════════════════════
// FORM CRUD HANDLERS
// ═══════════════════════════════════════════════════════════════════════════

/// List all forms (admin)
async fn list_forms(
    State(state): State<AppState>,
    AdminUser(_user): AdminUser,
    Query(query): Query<FormListQuery>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let page = query.page.unwrap_or(1).max(1);
    let per_page = query.per_page.unwrap_or(20).min(100);
    let offset = (page - 1) * per_page;

    let search_pattern: Option<String> = query.search.as_ref().map(|s| format!("%{}%", s));

    let forms: Vec<FormRow> = sqlx::query_as(
        r#"
        SELECT id, name, slug, description, fields, settings, is_published, 
               COALESCE(submission_count, 0) as submission_count, created_at, updated_at
        FROM forms
        WHERE ($1::boolean IS NULL OR is_published = $1)
          AND ($2::text IS NULL OR name ILIKE $2 OR description ILIKE $2)
        ORDER BY created_at DESC
        LIMIT $3 OFFSET $4
        "#,
    )
    .bind(query.is_published)
    .bind(search_pattern.as_deref())
    .bind(per_page)
    .bind(offset)
    .fetch_all(&state.db.pool)
    .await
    .map_err(|e| {
        tracing::error!("Database error in list_forms: {}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": "Database error"})),
        )
    })?;

    let total: (i64,) = sqlx::query_as(
        r#"
        SELECT COUNT(*) FROM forms
        WHERE ($1::boolean IS NULL OR is_published = $1)
          AND ($2::text IS NULL OR name ILIKE $2 OR description ILIKE $2)
        "#,
    )
    .bind(query.is_published)
    .bind(search_pattern.as_deref())
    .fetch_one(&state.db.pool)
    .await
    .unwrap_or((0,));

    Ok(Json(json!({
        "data": forms,
        "meta": {
            "current_page": page,
            "per_page": per_page,
            "total": total.0,
            "total_pages": (total.0 as f64 / per_page as f64).ceil() as i64
        }
    })))
}

/// Get single form by ID (admin)
async fn get_form(
    State(state): State<AppState>,
    AdminUser(_user): AdminUser,
    Path(id): Path<i64>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let form: FormRow = sqlx::query_as(
        r#"
        SELECT id, name, slug, description, fields, settings, is_published,
               COALESCE(submission_count, 0) as submission_count, created_at, updated_at
        FROM forms WHERE id = $1
        "#,
    )
    .bind(id)
    .fetch_optional(&state.db.pool)
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": e.to_string()})),
        )
    })?
    .ok_or_else(|| {
        (
            StatusCode::NOT_FOUND,
            Json(json!({"error": "Form not found"})),
        )
    })?;

    Ok(Json(json!({"data": form})))
}

/// Create new form (admin)
async fn create_form(
    State(state): State<AppState>,
    AdminUser(user): AdminUser,
    Json(input): Json<CreateFormRequest>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    tracing::info!(
        target: "security",
        event = "form_create",
        user_id = %user.id,
        form_name = %input.name,
        "Admin creating form"
    );

    let slug = slug::slugify(&input.name);
    let settings = input.settings.unwrap_or(json!({}));

    let form: FormRow = sqlx::query_as(
        r#"
        INSERT INTO forms (name, slug, description, fields, settings, is_published, submission_count, created_at, updated_at)
        VALUES ($1, $2, $3, $4, $5, $6, 0, NOW(), NOW())
        RETURNING id, name, slug, description, fields, settings, is_published, submission_count, created_at, updated_at
        "#
    )
    .bind(&input.name)
    .bind(&slug)
    .bind(&input.description)
    .bind(&input.fields)
    .bind(&settings)
    .bind(input.is_published.unwrap_or(false))
    .fetch_one(&state.db.pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to create form: {}", e);
        (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()})))
    })?;

    Ok(Json(
        json!({"data": form, "message": "Form created successfully"}),
    ))
}

/// Update form (admin)
async fn update_form(
    State(state): State<AppState>,
    AdminUser(user): AdminUser,
    Path(id): Path<i64>,
    Json(input): Json<UpdateFormRequest>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    tracing::info!(
        target: "security",
        event = "form_update",
        user_id = %user.id,
        form_id = %id,
        "Admin updating form"
    );

    // Build dynamic update - simplified approach
    let form: FormRow = sqlx::query_as(
        r#"
        UPDATE forms SET
            name = COALESCE($2, name),
            slug = CASE WHEN $2 IS NOT NULL THEN LOWER(REPLACE($2, ' ', '-')) ELSE slug END,
            description = COALESCE($3, description),
            fields = COALESCE($4, fields),
            settings = COALESCE($5, settings),
            is_published = COALESCE($6, is_published),
            updated_at = NOW()
        WHERE id = $1
        RETURNING id, name, slug, description, fields, settings, is_published, submission_count, created_at, updated_at
        "#
    )
    .bind(id)
    .bind(&input.name)
    .bind(&input.description)
    .bind(&input.fields)
    .bind(&input.settings)
    .bind(input.is_published)
    .fetch_optional(&state.db.pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()}))))?
    .ok_or_else(|| (StatusCode::NOT_FOUND, Json(json!({"error": "Form not found"}))))?;

    Ok(Json(
        json!({"data": form, "message": "Form updated successfully"}),
    ))
}

/// Delete form (admin)
async fn delete_form(
    State(state): State<AppState>,
    AdminUser(user): AdminUser,
    Path(id): Path<i64>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    tracing::info!(
        target: "security",
        event = "form_delete",
        user_id = %user.id,
        form_id = %id,
        "Admin deleting form"
    );

    // Delete submissions first
    sqlx::query("DELETE FROM form_submissions WHERE form_id = $1")
        .bind(id)
        .execute(&state.db.pool)
        .await
        .ok();

    let result = sqlx::query("DELETE FROM forms WHERE id = $1")
        .bind(id)
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
            Json(json!({"error": "Form not found"})),
        ));
    }

    Ok(Json(json!({"message": "Form deleted successfully"})))
}

/// Publish form (admin)
async fn publish_form(
    State(state): State<AppState>,
    AdminUser(user): AdminUser,
    Path(id): Path<i64>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    tracing::info!(target: "security", event = "form_publish", user_id = %user.id, form_id = %id, "Publishing form");

    let form: FormRow = sqlx::query_as(
        r#"
        UPDATE forms SET is_published = true, updated_at = NOW()
        WHERE id = $1
        RETURNING id, name, slug, description, fields, settings, is_published, submission_count, created_at, updated_at
        "#
    )
    .bind(id)
    .fetch_optional(&state.db.pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()}))))?
    .ok_or_else(|| (StatusCode::NOT_FOUND, Json(json!({"error": "Form not found"}))))?;

    Ok(Json(json!({"data": form, "message": "Form published"})))
}

/// Unpublish form (admin)
async fn unpublish_form(
    State(state): State<AppState>,
    AdminUser(user): AdminUser,
    Path(id): Path<i64>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    tracing::info!(target: "security", event = "form_unpublish", user_id = %user.id, form_id = %id, "Unpublishing form");

    let form: FormRow = sqlx::query_as(
        r#"
        UPDATE forms SET is_published = false, updated_at = NOW()
        WHERE id = $1
        RETURNING id, name, slug, description, fields, settings, is_published, submission_count, created_at, updated_at
        "#
    )
    .bind(id)
    .fetch_optional(&state.db.pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()}))))?
    .ok_or_else(|| (StatusCode::NOT_FOUND, Json(json!({"error": "Form not found"}))))?;

    Ok(Json(json!({"data": form, "message": "Form unpublished"})))
}

/// Duplicate form (admin)
async fn duplicate_form(
    State(state): State<AppState>,
    AdminUser(user): AdminUser,
    Path(id): Path<i64>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    tracing::info!(target: "security", event = "form_duplicate", user_id = %user.id, form_id = %id, "Duplicating form");

    let form: FormRow = sqlx::query_as(
        r#"
        INSERT INTO forms (name, slug, description, fields, settings, is_published, submission_count, created_at, updated_at)
        SELECT CONCAT(name, ' (Copy)'), CONCAT(slug, '-copy-', EXTRACT(EPOCH FROM NOW())::int), 
               description, fields, settings, false, 0, NOW(), NOW()
        FROM forms WHERE id = $1
        RETURNING id, name, slug, description, fields, settings, is_published, submission_count, created_at, updated_at
        "#
    )
    .bind(id)
    .fetch_optional(&state.db.pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()}))))?
    .ok_or_else(|| (StatusCode::NOT_FOUND, Json(json!({"error": "Form not found"}))))?;

    Ok(Json(json!({"data": form, "message": "Form duplicated"})))
}

/// Get form stats (admin)
async fn form_stats(
    State(state): State<AppState>,
    AdminUser(_user): AdminUser,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let total: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM forms")
        .fetch_one(&state.db.pool)
        .await
        .unwrap_or((0,));

    let published: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM forms WHERE is_published = true")
        .fetch_one(&state.db.pool)
        .await
        .unwrap_or((0,));

    let total_submissions: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM form_submissions")
        .fetch_one(&state.db.pool)
        .await
        .unwrap_or((0,));

    Ok(Json(json!({
        "data": {
            "total": total.0,
            "published": published.0,
            "draft": total.0 - published.0,
            "total_submissions": total_submissions.0
        }
    })))
}

/// Get available field types - ICT 7 Complete field type list
async fn field_types(
    AdminUser(_user): AdminUser,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    Ok(Json(json!({
        "data": {
            "types": [
                {"id": "text", "label": "Text Input", "icon": "type", "category": "basic"},
                {"id": "email", "label": "Email", "icon": "mail", "category": "basic"},
                {"id": "phone", "label": "Phone", "icon": "phone", "category": "basic"},
                {"id": "tel", "label": "Phone (Tel)", "icon": "phone", "category": "basic"},
                {"id": "number", "label": "Number", "icon": "hash", "category": "basic"},
                {"id": "url", "label": "URL/Website", "icon": "link", "category": "basic"},
                {"id": "textarea", "label": "Text Area", "icon": "align-left", "category": "basic"},
                {"id": "select", "label": "Dropdown", "icon": "chevron-down", "category": "choice"},
                {"id": "checkbox", "label": "Checkbox", "icon": "check-square", "category": "choice"},
                {"id": "radio", "label": "Radio Buttons", "icon": "circle", "category": "choice"},
                {"id": "date", "label": "Date", "icon": "calendar", "category": "datetime"},
                {"id": "time", "label": "Time", "icon": "clock", "category": "datetime"},
                {"id": "datetime", "label": "Date & Time", "icon": "calendar", "category": "datetime"},
                {"id": "file", "label": "File Upload", "icon": "upload", "category": "advanced"},
                {"id": "signature", "label": "Signature", "icon": "edit-3", "category": "advanced"},
                {"id": "address", "label": "Address", "icon": "map-pin", "category": "advanced"},
                {"id": "range", "label": "Range Slider", "icon": "sliders", "category": "advanced"},
                {"id": "rating", "label": "Star Rating", "icon": "star", "category": "advanced"},
                {"id": "color", "label": "Color Picker", "icon": "droplet", "category": "advanced"},
                {"id": "hidden", "label": "Hidden Field", "icon": "eye-off", "category": "utility"},
                {"id": "heading", "label": "Section Heading", "icon": "heading", "category": "layout"},
                {"id": "divider", "label": "Divider", "icon": "minus", "category": "layout"},
                {"id": "html", "label": "HTML Content", "icon": "code", "category": "layout"}
            ]
        }
    })))
}

// ═══════════════════════════════════════════════════════════════════════════
// FORM SUBMISSIONS HANDLERS
// ═══════════════════════════════════════════════════════════════════════════

#[derive(Debug, Deserialize)]
pub struct SubmissionListQuery {
    pub page: Option<i64>,
    pub per_page: Option<i64>,
    pub status: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateSubmissionStatusRequest {
    pub status: String,
}

#[derive(Debug, Deserialize)]
pub struct BulkUpdateStatusRequest {
    pub submission_ids: Vec<i64>,
    pub status: String,
}

#[derive(Debug, Deserialize)]
pub struct BulkDeleteRequest {
    pub submission_ids: Vec<i64>,
}

#[derive(Debug, Deserialize)]
pub struct ExportQuery {
    pub format: Option<String>,
}

/// List form submissions (admin)
async fn list_submissions(
    State(state): State<AppState>,
    AdminUser(_user): AdminUser,
    Path(form_id): Path<i64>,
    Query(query): Query<SubmissionListQuery>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let page = query.page.unwrap_or(1).max(1);
    let per_page = query.per_page.unwrap_or(20).min(100);
    let offset = (page - 1) * per_page;

    let submissions: Vec<FormSubmissionRow> = sqlx::query_as(
        r#"
        SELECT id, form_id, data, COALESCE(status, 'unread') as status, ip_address, user_agent, created_at
        FROM form_submissions
        WHERE form_id = $1
          AND ($4::text IS NULL OR status = $4)
        ORDER BY created_at DESC
        LIMIT $2 OFFSET $3
        "#,
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
async fn get_submission(
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
async fn delete_submission(
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

    let result = sqlx::query("DELETE FROM form_submissions WHERE id = $1 AND form_id = $2")
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

    // Update submission count
    sqlx::query("UPDATE forms SET submission_count = submission_count - 1 WHERE id = $1")
        .bind(form_id)
        .execute(&state.db.pool)
        .await
        .ok();

    Ok(Json(json!({"message": "Submission deleted successfully"})))
}

/// Update submission status (admin) - ICT 7 Fix: Complete submission management
async fn update_submission_status(
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
async fn bulk_update_status(
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

    let mut q = sqlx::query(&query).bind(&input.status).bind(form_id);

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
async fn bulk_delete_submissions(
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

    let mut q = sqlx::query(&query).bind(form_id);

    for id in &input.submission_ids {
        q = q.bind(id);
    }

    let result = q.execute(&state.db.pool).await.map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": e.to_string()})),
        )
    })?;

    // Update submission count
    sqlx::query("UPDATE forms SET submission_count = submission_count - $1 WHERE id = $2")
        .bind(result.rows_affected() as i32)
        .bind(form_id)
        .execute(&state.db.pool)
        .await
        .ok();

    Ok(Json(json!({
        "success": true,
        "message": format!("{} submissions deleted", result.rows_affected())
    })))
}

/// Export submissions as CSV (admin) - ICT 7 Fix
async fn export_submissions(
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
        r#"
        SELECT id, form_id, data, COALESCE(status, 'unread') as status, ip_address, user_agent, created_at
        FROM form_submissions
        WHERE form_id = $1
        ORDER BY created_at DESC
        "#,
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

    let filename = format!("form-{}-submissions.csv", form_id);

    Ok((
        StatusCode::OK,
        [
            (axum::http::header::CONTENT_TYPE, "text/csv".to_string()),
            (
                axum::http::header::CONTENT_DISPOSITION,
                format!("attachment; filename=\"{}\"", filename),
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

// ═══════════════════════════════════════════════════════════════════════════
// PUBLIC HANDLERS (for /api/forms - frontend compatibility)
// ═══════════════════════════════════════════════════════════════════════════

/// List published forms (public) - GET /forms
async fn list_public_forms(
    State(state): State<AppState>,
    Query(query): Query<FormListQuery>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let page = query.page.unwrap_or(1).max(1);
    let per_page = query.per_page.unwrap_or(20).min(100);
    let offset = (page - 1) * per_page;

    let search_pattern = query.search.as_ref().map(|s| format!("%{}%", s));

    let forms: Vec<FormRow> = sqlx::query_as(
        r#"
        SELECT id, name, slug, description, fields, settings, is_published, submission_count, created_at, updated_at
        FROM forms
        WHERE is_published = true
          AND ($1::text IS NULL OR name ILIKE $1 OR description ILIKE $1)
        ORDER BY created_at DESC
        LIMIT $2 OFFSET $3
        "#
    )
    .bind(search_pattern.as_deref())
    .bind(per_page)
    .bind(offset)
    .fetch_all(&state.db.pool)
    .await
    .unwrap_or_default();

    let total: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM forms WHERE is_published = true AND ($1::text IS NULL OR name ILIKE $1 OR description ILIKE $1)"
    )
    .bind(search_pattern.as_deref())
    .fetch_one(&state.db.pool)
    .await
    .unwrap_or(0);

    Ok(Json(json!({
        "success": true,
        "data": {
            "data": forms,
            "total": total,
            "per_page": per_page,
            "current_page": page
        },
        "forms": forms,
        "total": total
    })))
}

/// Get a single published form by ID or slug (public)
async fn get_public_form(
    State(state): State<AppState>,
    Path(id_or_slug): Path<String>,
) -> Result<Json<FormRow>, (StatusCode, Json<serde_json::Value>)> {
    let form: Option<FormRow> = if let Ok(id) = id_or_slug.parse::<i64>() {
        sqlx::query_as("SELECT * FROM forms WHERE id = $1 AND is_published = true")
            .bind(id)
            .fetch_optional(&state.db.pool)
            .await
            .ok()
            .flatten()
    } else {
        sqlx::query_as("SELECT * FROM forms WHERE slug = $1 AND is_published = true")
            .bind(&id_or_slug)
            .fetch_optional(&state.db.pool)
            .await
            .ok()
            .flatten()
    };

    form.map(Json).ok_or_else(|| {
        (
            StatusCode::NOT_FOUND,
            Json(json!({"error": "Form not found"})),
        )
    })
}

/// Submit a form (public) - POST /forms/:slug/submit
/// ICT 7 Fix: Complete implementation with validation, webhook, and email notifications
async fn submit_form(
    State(state): State<AppState>,
    Path(slug): Path<String>,
    Json(data): Json<serde_json::Value>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    // ICT 7 Spam Protection: Check honeypot field
    // If _hp_website field is filled, it's a bot (humans can't see this field)
    if let Some(honeypot) = data.get("_hp_website") {
        if let Some(hp_str) = honeypot.as_str() {
            if !hp_str.is_empty() {
                tracing::warn!(
                    target: "security",
                    event = "spam_blocked",
                    form_slug = %slug,
                    "Honeypot triggered - spam submission blocked"
                );
                // Silently accept to not tip off bots, but don't save
                return Ok(Json(json!({
                    "success": true,
                    "message": "Form submitted successfully"
                })));
            }
        }
    }

    // Also check for _hp_email honeypot variant
    if let Some(honeypot) = data.get("_hp_email") {
        if let Some(hp_str) = honeypot.as_str() {
            if !hp_str.is_empty() {
                tracing::warn!(
                    target: "security",
                    event = "spam_blocked",
                    form_slug = %slug,
                    "Honeypot triggered - spam submission blocked"
                );
                return Ok(Json(json!({
                    "success": true,
                    "message": "Form submitted successfully"
                })));
            }
        }
    }

    // Remove honeypot fields from submission data
    let mut clean_data = data.clone();
    if let Some(obj) = clean_data.as_object_mut() {
        obj.remove("_hp_website");
        obj.remove("_hp_email");
        obj.remove("_hp_name");
    }

    // Find form by slug
    let form: Option<FormRow> =
        sqlx::query_as("SELECT * FROM forms WHERE slug = $1 AND is_published = true")
            .bind(&slug)
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

    // ICT 7 Fix: Server-side validation
    let validation_errors = validate_form_submission(&form.fields, &clean_data);
    if !validation_errors.is_empty() {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(json!({
                "success": false,
                "message": "Validation failed",
                "errors": validation_errors
            })),
        ));
    }

    // Insert submission
    let submission_result: (i64,) = sqlx::query_as(
        "INSERT INTO form_submissions (form_id, data, status, created_at) VALUES ($1, $2, 'unread', NOW()) RETURNING id"
    )
        .bind(form.id)
        .bind(&clean_data)
        .fetch_one(&state.db.pool)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": e.to_string()})),
            )
        })?;

    // Update submission count
    sqlx::query("UPDATE forms SET submission_count = submission_count + 1 WHERE id = $1")
        .bind(form.id)
        .execute(&state.db.pool)
        .await
        .ok();

    // ICT 7 Fix: Email notification via Postmark
    if let Some(settings) = form.settings.as_object() {
        if settings
            .get("send_email")
            .and_then(|v| v.as_bool())
            .unwrap_or(false)
        {
            if let Some(email_to) = settings.get("email_to").and_then(|v| v.as_str()) {
                if let Some(ref email_service) = state.services.email {
                    let subject = format!("New Form Submission: {}", form.name);
                    let html_body = build_form_notification_email(&form.name, &clean_data);

                    if let Err(e) = email_service
                        .send(email_to, &subject, &html_body, None)
                        .await
                    {
                        tracing::error!(
                            target: "forms",
                            event = "email_send_error",
                            form_id = %form.id,
                            error = %e,
                            "Failed to send form notification email"
                        );
                    } else {
                        tracing::info!(
                            target: "forms",
                            event = "email_notification_sent",
                            form_id = %form.id,
                            email_to = %email_to,
                            "Form submission email notification sent"
                        );
                    }
                }
            }
        }

        // ICT 7 Fix: Webhook integration
        if let Some(webhook_url) = settings.get("webhook_url").and_then(|v| v.as_str()) {
            if !webhook_url.is_empty() {
                let webhook_payload = json!({
                    "event": "form_submission",
                    "form_id": form.id,
                    "form_name": form.name,
                    "form_slug": form.slug,
                    "submission_id": submission_result.0,
                    "data": clean_data,
                    "timestamp": chrono::Utc::now().to_rfc3339()
                });

                // Fire-and-forget webhook call (don't block response)
                let webhook_url = webhook_url.to_string();
                let client = reqwest::Client::new();
                tokio::spawn(async move {
                    match client
                        .post(&webhook_url)
                        .header("Content-Type", "application/json")
                        .header("X-Webhook-Event", "form_submission")
                        .json(&webhook_payload)
                        .timeout(std::time::Duration::from_secs(10))
                        .send()
                        .await
                    {
                        Ok(response) => {
                            if response.status().is_success() {
                                tracing::info!(
                                    target: "forms",
                                    event = "webhook_success",
                                    url = %webhook_url,
                                    "Webhook delivered successfully"
                                );
                            } else {
                                tracing::warn!(
                                    target: "forms",
                                    event = "webhook_error",
                                    url = %webhook_url,
                                    status = %response.status(),
                                    "Webhook returned non-success status"
                                );
                            }
                        }
                        Err(e) => {
                            tracing::error!(
                                target: "forms",
                                event = "webhook_failed",
                                url = %webhook_url,
                                error = %e,
                                "Failed to deliver webhook"
                            );
                        }
                    }
                });
            }
        }
    }

    // Get success message from settings or use default
    let success_message = form
        .settings
        .as_object()
        .and_then(|s| s.get("success_message"))
        .and_then(|v| v.as_str())
        .unwrap_or("Form submitted successfully");

    // Get redirect URL if configured
    let redirect_url = form
        .settings
        .as_object()
        .and_then(|s| s.get("redirect_url"))
        .and_then(|v| v.as_str());

    let mut response = json!({
        "success": true,
        "message": success_message,
        "submission_id": submission_result.0.to_string()
    });

    if let Some(url) = redirect_url {
        response["redirect_url"] = json!(url);
    }

    Ok(Json(response))
}

/// ICT 7 Fix: Server-side form field validation
fn validate_form_submission(
    fields: &serde_json::Value,
    data: &serde_json::Value,
) -> std::collections::HashMap<String, Vec<String>> {
    let mut errors: std::collections::HashMap<String, Vec<String>> =
        std::collections::HashMap::new();

    if let Some(fields_array) = fields.as_array() {
        for field in fields_array {
            let field_name = field.get("name").and_then(|v| v.as_str()).unwrap_or("");
            let field_label = field
                .get("label")
                .and_then(|v| v.as_str())
                .unwrap_or(field_name);
            let required = field
                .get("required")
                .and_then(|v| v.as_bool())
                .unwrap_or(false);
            let field_type = field
                .get("field_type")
                .and_then(|v| v.as_str())
                .unwrap_or("text");
            let validation = field.get("validation");

            let value = data.get(field_name);
            let value_str = value.and_then(|v| v.as_str()).unwrap_or("");
            let is_empty = value.is_none()
                || value_str.is_empty()
                || (value.is_some() && value.unwrap().is_null());

            // Required field validation
            if required && is_empty {
                errors
                    .entry(field_name.to_string())
                    .or_default()
                    .push(format!("{} is required", field_label));
                continue;
            }

            // Skip further validation if empty and not required
            if is_empty {
                continue;
            }

            // Email validation
            if field_type == "email" && !value_str.is_empty()
                && (!value_str.contains('@') || !value_str.contains('.')) {
                    errors
                        .entry(field_name.to_string())
                        .or_default()
                        .push(format!("{} must be a valid email address", field_label));
                }

            // Min/max length validation
            if let Some(validation) = validation {
                if let Some(min_length) = validation.get("min_length").and_then(|v| v.as_u64()) {
                    if (value_str.len() as u64) < min_length {
                        errors
                            .entry(field_name.to_string())
                            .or_default()
                            .push(format!(
                                "{} must be at least {} characters",
                                field_label, min_length
                            ));
                    }
                }

                if let Some(max_length) = validation.get("max_length").and_then(|v| v.as_u64()) {
                    if (value_str.len() as u64) > max_length {
                        errors
                            .entry(field_name.to_string())
                            .or_default()
                            .push(format!(
                                "{} must be no more than {} characters",
                                field_label, max_length
                            ));
                    }
                }

                // Pattern validation
                if let Some(pattern) = validation.get("pattern").and_then(|v| v.as_str()) {
                    if let Ok(regex) = regex::Regex::new(pattern) {
                        if !regex.is_match(value_str) {
                            let message = validation
                                .get("pattern_message")
                                .and_then(|v| v.as_str())
                                .unwrap_or("Invalid format");
                            errors
                                .entry(field_name.to_string())
                                .or_default()
                                .push(message.to_string());
                        }
                    }
                }

                // Number validation (min/max)
                if field_type == "number" {
                    if let Ok(num_value) = value_str.parse::<f64>() {
                        if let Some(min) = validation.get("min").and_then(|v| v.as_f64()) {
                            if num_value < min {
                                errors
                                    .entry(field_name.to_string())
                                    .or_default()
                                    .push(format!("{} must be at least {}", field_label, min));
                            }
                        }
                        if let Some(max) = validation.get("max").and_then(|v| v.as_f64()) {
                            if num_value > max {
                                errors
                                    .entry(field_name.to_string())
                                    .or_default()
                                    .push(format!("{} must be no more than {}", field_label, max));
                            }
                        }
                    }
                }
            }
        }
    }

    errors
}

/// ICT 7 Fix: Build HTML email for form notification
fn build_form_notification_email(form_name: &str, data: &serde_json::Value) -> String {
    let mut fields_html = String::new();

    if let Some(obj) = data.as_object() {
        for (key, value) in obj {
            let value_str = match value {
                serde_json::Value::String(s) => s.clone(),
                serde_json::Value::Null => "Not provided".to_string(),
                serde_json::Value::Array(arr) => arr
                    .iter()
                    .filter_map(|v| v.as_str())
                    .collect::<Vec<_>>()
                    .join(", "),
                _ => value.to_string(),
            };
            fields_html.push_str(&format!(
                r#"<tr><td style="padding: 10px 15px; border-bottom: 1px solid #eee; font-weight: 600; color: #333; width: 30%;">{}</td><td style="padding: 10px 15px; border-bottom: 1px solid #eee; color: #666;">{}</td></tr>"#,
                key.replace('_', " ").to_uppercase(),
                html_escape(&value_str)
            ));
        }
    }

    format!(
        r#"<!DOCTYPE html>
<html>
<head>
    <meta charset="utf-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>New Form Submission</title>
</head>
<body style="font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif; margin: 0; padding: 20px; background-color: #f5f5f5;">
    <div style="max-width: 600px; margin: 0 auto; background-color: #ffffff; border-radius: 8px; overflow: hidden; box-shadow: 0 2px 4px rgba(0,0,0,0.1);">
        <div style="background: linear-gradient(135deg, #667eea 0%, #764ba2 100%); color: white; padding: 30px; text-align: center;">
            <h1 style="margin: 0; font-size: 24px;">New Form Submission</h1>
            <p style="margin: 10px 0 0; opacity: 0.9;">{}</p>
        </div>
        <div style="padding: 30px;">
            <table style="width: 100%; border-collapse: collapse;">
                {}
            </table>
            <div style="margin-top: 30px; padding-top: 20px; border-top: 1px solid #eee; text-align: center; color: #999; font-size: 12px;">
                <p>This email was sent from Revolution Trading Pros</p>
                <p>Submitted at: {}</p>
            </div>
        </div>
    </div>
</body>
</html>"#,
        form_name,
        fields_html,
        chrono::Utc::now().format("%Y-%m-%d %H:%M:%S UTC")
    )
}

/// Helper to escape HTML characters
fn html_escape(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#x27;")
}

/// Track form view (public) - POST /forms/:slug/view
/// ICT 7 Fix: Form analytics - track views for conversion rate calculation
async fn track_form_view(
    State(state): State<AppState>,
    Path(slug): Path<String>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    // Update view count (we'll add a view_count column if not exists)
    sqlx::query(
        r#"
        UPDATE forms
        SET settings = jsonb_set(
            COALESCE(settings, '{}'::jsonb),
            '{view_count}',
            (COALESCE((settings->>'view_count')::int, 0) + 1)::text::jsonb
        )
        WHERE slug = $1 AND is_published = true
        "#,
    )
    .bind(&slug)
    .execute(&state.db.pool)
    .await
    .ok();

    Ok(Json(json!({"success": true})))
}

/// Get form analytics (admin) - GET /forms/:id/analytics
/// ICT 7 Fix: Comprehensive form analytics
async fn get_form_analytics(
    State(state): State<AppState>,
    AdminUser(_user): AdminUser,
    Path(form_id): Path<i64>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    // Get form with settings
    let form: Option<FormRow> = sqlx::query_as("SELECT * FROM forms WHERE id = $1")
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

    // Get view count from settings
    let views = form
        .settings
        .as_object()
        .and_then(|s| s.get("view_count"))
        .and_then(|v| v.as_i64())
        .unwrap_or(0);

    // Get submission count
    let submissions = form.submission_count as i64;

    // Calculate conversion rate
    let conversion_rate = if views > 0 {
        (submissions as f64 / views as f64) * 100.0
    } else {
        0.0
    };

    // Get submission trends (last 30 days)
    let trends: Vec<(String, i64)> = sqlx::query_as(
        r#"
        SELECT DATE(created_at)::text as date, COUNT(*) as count
        FROM form_submissions
        WHERE form_id = $1 AND created_at > NOW() - INTERVAL '30 days'
        GROUP BY DATE(created_at)
        ORDER BY date DESC
        "#,
    )
    .bind(form_id)
    .fetch_all(&state.db.pool)
    .await
    .unwrap_or_default();

    // Get status breakdown
    let status_breakdown: Vec<(String, i64)> = sqlx::query_as(
        r#"
        SELECT COALESCE(status, 'unread') as status, COUNT(*) as count
        FROM form_submissions
        WHERE form_id = $1
        GROUP BY status
        "#,
    )
    .bind(form_id)
    .fetch_all(&state.db.pool)
    .await
    .unwrap_or_default();

    Ok(Json(json!({
        "views": views,
        "unique_views": views, // Same as views for now
        "submissions": submissions,
        "conversions": submissions,
        "conversion_rate": format!("{:.1}", conversion_rate),
        "avg_time_to_complete": 0, // Would need JS tracking
        "submission_trends": trends.into_iter().map(|(d, c)| json!({"date": d, "count": c})).collect::<Vec<_>>(),
        "status_breakdown": status_breakdown.into_iter().map(|(s, c)| json!({"status": s, "count": c})).collect::<Vec<_>>(),
        "field_drop_offs": {},
        "device_breakdown": {},
        "referrer_breakdown": {},
        "geographic_breakdown": {}
    })))
}

// ═══════════════════════════════════════════════════════════════════════════
// ROUTER
// ═══════════════════════════════════════════════════════════════════════════

/// Public forms router - /api/forms (frontend compatibility)
pub fn public_router() -> Router<AppState> {
    Router::new()
        .route("/", get(list_public_forms))
        .route("/:id", get(get_public_form))
        .route("/:slug/submit", post(submit_form))
        .route("/:slug/view", post(track_form_view))
}

pub fn admin_router() -> Router<AppState> {
    Router::new()
        // Form CRUD
        .route("/", get(list_forms).post(create_form))
        .route("/stats", get(form_stats))
        .route("/field-types", get(field_types))
        .route("/:id", get(get_form).put(update_form).delete(delete_form))
        .route("/:id/publish", post(publish_form))
        .route("/:id/unpublish", post(unpublish_form))
        .route("/:id/duplicate", post(duplicate_form))
        .route("/:id/analytics", get(get_form_analytics))
        // Submissions
        .route("/:form_id/submissions", get(list_submissions))
        .route("/:form_id/submissions/export", get(export_submissions))
        .route(
            "/:form_id/submissions/bulk-update-status",
            post(bulk_update_status),
        )
        .route(
            "/:form_id/submissions/bulk-delete",
            post(bulk_delete_submissions),
        )
        .route(
            "/:form_id/submissions/:submission_id",
            get(get_submission).delete(delete_submission),
        )
        .route(
            "/:form_id/submissions/:submission_id/status",
            put(update_submission_status),
        )
}
