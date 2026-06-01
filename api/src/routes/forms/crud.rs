//! Form CRUD handlers (admin).
//!
//! list / get / create / update / delete / publish / unpublish /
//! duplicate / stats / field_types — every mutating handler emits a
//! security-tagged tracing event with the acting admin's user id.

use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Json,
};
use serde_json::json;

use super::types::{CreateFormRequest, FormListQuery, FormRow, UpdateFormRequest};
use crate::{middleware::admin::AdminUser, AppState};

/// List all forms (admin)
pub(super) async fn list_forms(
    State(state): State<AppState>,
    AdminUser(_user): AdminUser,
    Query(query): Query<FormListQuery>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let page = query.page.unwrap_or(1).max(1);
    let per_page = query.per_page.unwrap_or(20).min(100);
    let offset = (page - 1) * per_page;

    let search_pattern: Option<String> = query.search.as_ref().map(|s| format!("%{s}%"));

    let forms: Vec<FormRow> = sqlx::query_as(
        r"
        SELECT id, name, slug, description, fields, settings, is_published,
               COALESCE(submission_count, 0) as submission_count, created_at, updated_at
        FROM forms
        WHERE ($1::boolean IS NULL OR is_published = $1)
          AND ($2::text IS NULL OR name ILIKE $2 OR description ILIKE $2)
        ORDER BY created_at DESC
        LIMIT $3 OFFSET $4
        ",
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
        r"
        SELECT COUNT(*) FROM forms
        WHERE ($1::boolean IS NULL OR is_published = $1)
          AND ($2::text IS NULL OR name ILIKE $2 OR description ILIKE $2)
        ",
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
pub(super) async fn get_form(
    State(state): State<AppState>,
    AdminUser(_user): AdminUser,
    Path(id): Path<i64>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let form: FormRow = sqlx::query_as(
        r"
        SELECT id, name, slug, description, fields, settings, is_published,
               COALESCE(submission_count, 0) as submission_count, created_at, updated_at
        FROM forms WHERE id = $1
        ",
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
pub(super) async fn create_form(
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
        r"
        INSERT INTO forms (name, slug, description, fields, settings, is_published, submission_count, created_at, updated_at)
        VALUES ($1, $2, $3, $4, $5, $6, 0, NOW(), NOW())
        RETURNING id, name, slug, description, fields, settings, is_published, submission_count, created_at, updated_at
        "
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
pub(super) async fn update_form(
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
        r"
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
        "
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
pub(super) async fn delete_form(
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

    // ICT 7 SAFETY: wrap submissions+forms cascade delete in a transaction so a
    // crash between the two DELETEs cannot leave submissions rows orphaned with
    // their form already gone (or, worse, the inverse: form gone but submissions
    // still referenced by stale FKs).
    let mut tx = state.db.pool.begin().await.map_err(|e| {
        tracing::error!("tx start (delete_form): {}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": "Database error"})),
        )
    })?;

    // Delete submissions first
    sqlx::query("DELETE FROM form_submissions WHERE form_id = $1")
        .bind(id)
        .execute(&mut *tx)
        .await
        .map_err(|e| {
            tracing::error!("Failed to delete form_submissions: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": "Failed to delete submissions"})),
            )
        })?;

    let result = sqlx::query("DELETE FROM forms WHERE id = $1")
        .bind(id)
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
            Json(json!({"error": "Form not found"})),
        ));
    }

    tx.commit().await.map_err(|e| {
        tracing::error!("tx commit (delete_form): {}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": "Database error"})),
        )
    })?;

    Ok(Json(json!({"message": "Form deleted successfully"})))
}

/// Publish form (admin)
pub(super) async fn publish_form(
    State(state): State<AppState>,
    AdminUser(user): AdminUser,
    Path(id): Path<i64>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    tracing::info!(target: "security", event = "form_publish", user_id = %user.id, form_id = %id, "Publishing form");

    let form: FormRow = sqlx::query_as(
        r"
        UPDATE forms SET is_published = true, updated_at = NOW()
        WHERE id = $1
        RETURNING id, name, slug, description, fields, settings, is_published, submission_count, created_at, updated_at
        "
    )
    .bind(id)
    .fetch_optional(&state.db.pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()}))))?
    .ok_or_else(|| (StatusCode::NOT_FOUND, Json(json!({"error": "Form not found"}))))?;

    Ok(Json(json!({"data": form, "message": "Form published"})))
}

/// Unpublish form (admin)
pub(super) async fn unpublish_form(
    State(state): State<AppState>,
    AdminUser(user): AdminUser,
    Path(id): Path<i64>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    tracing::info!(target: "security", event = "form_unpublish", user_id = %user.id, form_id = %id, "Unpublishing form");

    let form: FormRow = sqlx::query_as(
        r"
        UPDATE forms SET is_published = false, updated_at = NOW()
        WHERE id = $1
        RETURNING id, name, slug, description, fields, settings, is_published, submission_count, created_at, updated_at
        "
    )
    .bind(id)
    .fetch_optional(&state.db.pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()}))))?
    .ok_or_else(|| (StatusCode::NOT_FOUND, Json(json!({"error": "Form not found"}))))?;

    Ok(Json(json!({"data": form, "message": "Form unpublished"})))
}

/// Duplicate form (admin)
pub(super) async fn duplicate_form(
    State(state): State<AppState>,
    AdminUser(user): AdminUser,
    Path(id): Path<i64>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    tracing::info!(target: "security", event = "form_duplicate", user_id = %user.id, form_id = %id, "Duplicating form");

    let form: FormRow = sqlx::query_as(
        r"
        INSERT INTO forms (name, slug, description, fields, settings, is_published, submission_count, created_at, updated_at)
        SELECT CONCAT(name, ' (Copy)'), CONCAT(slug, '-copy-', EXTRACT(EPOCH FROM NOW())::int),
               description, fields, settings, false, 0, NOW(), NOW()
        FROM forms WHERE id = $1
        RETURNING id, name, slug, description, fields, settings, is_published, submission_count, created_at, updated_at
        "
    )
    .bind(id)
    .fetch_optional(&state.db.pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()}))))?
    .ok_or_else(|| (StatusCode::NOT_FOUND, Json(json!({"error": "Form not found"}))))?;

    Ok(Json(json!({"data": form, "message": "Form duplicated"})))
}

/// Get form stats (admin)
pub(super) async fn form_stats(
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
pub(super) async fn field_types(
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
