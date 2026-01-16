//! Forms Admin Routes - Revolution Trading Pros
//! Apple ICT 7+ Principal Engineer Grade - January 2026
//!
//! Full CRUD for forms and form submissions with admin protection.

use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    routing::{get, post, put, delete},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::{
    middleware::admin::AdminUser,
    AppState,
};

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
        "#
    )
    .bind(query.is_published)
    .bind(search_pattern.as_deref())
    .bind(per_page)
    .bind(offset)
    .fetch_all(&state.db.pool)
    .await
    .map_err(|e| {
        tracing::error!("Database error in list_forms: {}", e);
        (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": "Database error"})))
    })?;

    let total: (i64,) = sqlx::query_as(
        r#"
        SELECT COUNT(*) FROM forms
        WHERE ($1::boolean IS NULL OR is_published = $1)
          AND ($2::text IS NULL OR name ILIKE $2 OR description ILIKE $2)
        "#
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
        "#
    )
    .bind(id)
    .fetch_optional(&state.db.pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()}))))?
    .ok_or_else(|| (StatusCode::NOT_FOUND, Json(json!({"error": "Form not found"}))))?;

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

    Ok(Json(json!({"data": form, "message": "Form created successfully"})))
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

    Ok(Json(json!({"data": form, "message": "Form updated successfully"})))
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
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()}))))?;

    if result.rows_affected() == 0 {
        return Err((StatusCode::NOT_FOUND, Json(json!({"error": "Form not found"}))));
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

/// Get available field types
async fn field_types(
    AdminUser(_user): AdminUser,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    Ok(Json(json!({
        "data": {
            "types": [
                {"id": "text", "label": "Text Input", "icon": "type"},
                {"id": "email", "label": "Email", "icon": "mail"},
                {"id": "phone", "label": "Phone", "icon": "phone"},
                {"id": "number", "label": "Number", "icon": "hash"},
                {"id": "textarea", "label": "Text Area", "icon": "align-left"},
                {"id": "select", "label": "Dropdown", "icon": "chevron-down"},
                {"id": "checkbox", "label": "Checkbox", "icon": "check-square"},
                {"id": "radio", "label": "Radio Buttons", "icon": "circle"},
                {"id": "date", "label": "Date", "icon": "calendar"},
                {"id": "file", "label": "File Upload", "icon": "upload"},
                {"id": "hidden", "label": "Hidden Field", "icon": "eye-off"}
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
        SELECT id, form_id, data, ip_address, user_agent, created_at
        FROM form_submissions
        WHERE form_id = $1
        ORDER BY created_at DESC
        LIMIT $2 OFFSET $3
        "#
    )
    .bind(form_id)
    .bind(per_page)
    .bind(offset)
    .fetch_all(&state.db.pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()}))))?;

    let total: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM form_submissions WHERE form_id = $1")
        .bind(form_id)
        .fetch_one(&state.db.pool)
        .await
        .unwrap_or((0,));

    Ok(Json(json!({
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
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()}))))?;

    if result.rows_affected() == 0 {
        return Err((StatusCode::NOT_FOUND, Json(json!({"error": "Submission not found"}))));
    }

    // Update submission count
    sqlx::query("UPDATE forms SET submission_count = submission_count - 1 WHERE id = $1")
        .bind(form_id)
        .execute(&state.db.pool)
        .await
        .ok();

    Ok(Json(json!({"message": "Submission deleted successfully"})))
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
        (StatusCode::NOT_FOUND, Json(json!({"error": "Form not found"})))
    })
}

/// Submit a form (public) - POST /forms/:slug/submit
async fn submit_form(
    State(state): State<AppState>,
    Path(slug): Path<String>,
    Json(data): Json<serde_json::Value>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    // Find form by slug
    let form: Option<FormRow> = sqlx::query_as(
        "SELECT * FROM forms WHERE slug = $1 AND is_published = true"
    )
    .bind(&slug)
    .fetch_optional(&state.db.pool)
    .await
    .ok()
    .flatten();

    let form = form.ok_or_else(|| {
        (StatusCode::NOT_FOUND, Json(json!({"error": "Form not found"})))
    })?;

    // Insert submission
    sqlx::query(
        "INSERT INTO form_submissions (form_id, data, created_at) VALUES ($1, $2, NOW())"
    )
    .bind(form.id)
    .bind(&data)
    .execute(&state.db.pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()}))))?;

    // Update submission count
    sqlx::query("UPDATE forms SET submission_count = submission_count + 1 WHERE id = $1")
        .bind(form.id)
        .execute(&state.db.pool)
        .await
        .ok();

    Ok(Json(json!({
        "success": true,
        "message": "Form submitted successfully"
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
        // Submissions
        .route("/:form_id/submissions", get(list_submissions))
        .route("/:form_id/submissions/:submission_id", get(get_submission).delete(delete_submission))
}
