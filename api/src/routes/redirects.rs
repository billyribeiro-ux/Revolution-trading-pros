//! Redirects Controller
//! ICT Level 7 Principal Engineer - URL redirect management
//!
//! Manages URL redirects for SEO and site maintenance.
//! All admin endpoints require AdminUser authentication.

use axum::{
    extract::{Path, Query, State},
    response::Json,
    routing::{delete, get, post, put},
    Router,
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

use crate::{middleware::admin::AdminUser, utils::errors::ApiError, AppState};

#[derive(Debug, Serialize, FromRow)]
pub struct Redirect {
    pub id: i64,
    pub from_path: String,
    pub to_path: String,
    pub status_code: i32,
    pub redirect_type: String,
    pub is_active: bool,
    pub hit_count: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct RedirectQuery {
    pub search: Option<String>,
    pub is_active: Option<bool>,
    pub per_page: Option<i64>,
    pub page: Option<i64>,
}

#[derive(Debug, Deserialize)]
pub struct CreateRedirect {
    pub from_path: String,
    pub to_path: String,
    pub status_code: Option<i32>,
    pub redirect_type: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateRedirect {
    pub from_path: Option<String>,
    pub to_path: Option<String>,
    pub status_code: Option<i32>,
    pub is_active: Option<bool>,
}

/// GET /admin/redirects - List all redirects (ICT 7: SQL injection safe)
#[tracing::instrument(skip(state, _admin))]
pub async fn index(
    _admin: AdminUser,
    State(state): State<AppState>,
    Query(params): Query<RedirectQuery>,
) -> Result<Json<serde_json::Value>, ApiError> {
    // ICT Level 7: Parameterized query to prevent SQL injection
    let search_pattern = params
        .search
        .as_ref()
        .map(|s| format!("%{}%", s.replace('%', "\\%").replace('_', "\\_")));

    // Pagination
    let per_page = params.per_page.unwrap_or(50).min(500);
    let page = params.page.unwrap_or(1).max(1);
    let offset = (page - 1) * per_page;

    // Execute with proper parameter binding
    let redirects: Vec<Redirect> = match (&search_pattern, params.is_active) {
        (Some(pattern), Some(is_active)) => {
            sqlx::query_as(
                "SELECT * FROM redirects WHERE (from_path ILIKE $1 OR to_path ILIKE $1) AND is_active = $2 ORDER BY created_at DESC LIMIT $3 OFFSET $4"
            )
            .bind(pattern)
            .bind(is_active)
            .bind(per_page)
            .bind(offset)
            .fetch_all(state.db.pool())
            .await
        }
        (Some(pattern), None) => {
            sqlx::query_as(
                "SELECT * FROM redirects WHERE (from_path ILIKE $1 OR to_path ILIKE $1) ORDER BY created_at DESC LIMIT $2 OFFSET $3"
            )
            .bind(pattern)
            .bind(per_page)
            .bind(offset)
            .fetch_all(state.db.pool())
            .await
        }
        (None, Some(is_active)) => {
            sqlx::query_as(
                "SELECT * FROM redirects WHERE is_active = $1 ORDER BY created_at DESC LIMIT $2 OFFSET $3"
            )
            .bind(is_active)
            .bind(per_page)
            .bind(offset)
            .fetch_all(state.db.pool())
            .await
        }
        (None, None) => {
            sqlx::query_as(
                "SELECT * FROM redirects ORDER BY created_at DESC LIMIT $1 OFFSET $2"
            )
            .bind(per_page)
            .bind(offset)
            .fetch_all(state.db.pool())
            .await
        }
    }.map_err(|e| ApiError::database_error(&e.to_string()))?;

    Ok(Json(serde_json::json!({ "data": redirects })))
}

/// GET /admin/redirects/:id - Get a single redirect
#[tracing::instrument(skip(state, _admin))]
pub async fn show(
    _admin: AdminUser,
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> Result<Json<serde_json::Value>, ApiError> {
    let redirect: Redirect = sqlx::query_as("SELECT * FROM redirects WHERE id = $1")
        .bind(id)
        .fetch_one(state.db.pool())
        .await
        .map_err(|_| ApiError::not_found("Redirect not found"))?;

    Ok(Json(serde_json::json!({ "data": redirect })))
}

/// POST /admin/redirects - Create a new redirect
#[tracing::instrument(skip(state, _admin))]
pub async fn store(
    _admin: AdminUser,
    State(state): State<AppState>,
    Json(payload): Json<CreateRedirect>,
) -> Result<Json<serde_json::Value>, ApiError> {
    // Validate status code
    let status_code = payload.status_code.unwrap_or(301);
    if ![301, 302, 307, 308].contains(&status_code) {
        return Err(ApiError::validation_error(
            "Status code must be 301, 302, 307, or 308",
        ));
    }

    // ICT 7: Validate path format (must start with /)
    if !payload.from_path.starts_with('/') {
        return Err(ApiError::validation_error("from_path must start with /"));
    }

    // Check if from_path already exists
    let exists: bool =
        sqlx::query_scalar("SELECT EXISTS(SELECT 1 FROM redirects WHERE from_path = $1)")
            .bind(&payload.from_path)
            .fetch_one(state.db.pool())
            .await
            .map_err(|e| ApiError::database_error(&e.to_string()))?;

    if exists {
        return Err(ApiError::validation_error(
            "Redirect from this path already exists",
        ));
    }

    // ICT 7: Prevent redirect loops
    if payload.from_path == payload.to_path {
        return Err(ApiError::validation_error(
            "Cannot redirect a path to itself",
        ));
    }

    let redirect_type = payload
        .redirect_type
        .unwrap_or_else(|| "manual".to_string());

    let redirect: Redirect = sqlx::query_as(
        "INSERT INTO redirects (from_path, to_path, status_code, redirect_type, is_active, hit_count, created_at, updated_at)
         VALUES ($1, $2, $3, $4, true, 0, NOW(), NOW())
         RETURNING *"
    )
    .bind(&payload.from_path)
    .bind(&payload.to_path)
    .bind(status_code)
    .bind(&redirect_type)
    .fetch_one(state.db.pool())
    .await
    .map_err(|e| ApiError::database_error(&e.to_string()))?;

    Ok(Json(serde_json::json!({ "data": redirect })))
}

/// PUT /admin/redirects/:id - Update a redirect
#[tracing::instrument(skip(state, _admin))]
pub async fn update(
    _admin: AdminUser,
    State(state): State<AppState>,
    Path(id): Path<i64>,
    Json(payload): Json<UpdateRedirect>,
) -> Result<Json<serde_json::Value>, ApiError> {
    // Check if redirect exists and get current data
    let current: Redirect = sqlx::query_as("SELECT * FROM redirects WHERE id = $1")
        .bind(id)
        .fetch_optional(state.db.pool())
        .await
        .map_err(|e| ApiError::database_error(&e.to_string()))?
        .ok_or_else(|| ApiError::not_found("Redirect not found"))?;

    // Validate status code if provided
    if let Some(status_code) = payload.status_code {
        if ![301, 302, 307, 308].contains(&status_code) {
            return Err(ApiError::validation_error(
                "Status code must be 301, 302, 307, or 308",
            ));
        }
    }

    // ICT 7: Validate path format if provided
    if let Some(ref from_path) = payload.from_path {
        if !from_path.starts_with('/') {
            return Err(ApiError::validation_error("from_path must start with /"));
        }

        // Check if from_path already exists for another redirect
        let path_exists: bool = sqlx::query_scalar(
            "SELECT EXISTS(SELECT 1 FROM redirects WHERE from_path = $1 AND id != $2)",
        )
        .bind(from_path)
        .bind(id)
        .fetch_one(state.db.pool())
        .await
        .map_err(|e| ApiError::database_error(&e.to_string()))?;

        if path_exists {
            return Err(ApiError::validation_error(
                "Redirect from this path already exists",
            ));
        }
    }

    // ICT 7: Prevent redirect loops
    let final_from = payload.from_path.as_ref().unwrap_or(&current.from_path);
    let final_to = payload.to_path.as_ref().unwrap_or(&current.to_path);
    if final_from == final_to {
        return Err(ApiError::validation_error(
            "Cannot redirect a path to itself",
        ));
    }

    // Build dynamic update query
    let mut updates = Vec::new();
    let mut param_count = 1;

    if payload.from_path.is_some() {
        updates.push(format!("from_path = ${}", param_count));
        param_count += 1;
    }
    if payload.to_path.is_some() {
        updates.push(format!("to_path = ${}", param_count));
        param_count += 1;
    }
    if payload.status_code.is_some() {
        updates.push(format!("status_code = ${}", param_count));
        param_count += 1;
    }
    if payload.is_active.is_some() {
        updates.push(format!("is_active = ${}", param_count));
        param_count += 1;
    }

    updates.push(format!("updated_at = ${}", param_count));

    let query_str = format!(
        "UPDATE redirects SET {} WHERE id = ${} RETURNING *",
        updates.join(", "),
        param_count + 1
    );

    let mut query = sqlx::query_as::<_, Redirect>(&query_str);

    if let Some(from_path) = payload.from_path {
        query = query.bind(from_path);
    }
    if let Some(to_path) = payload.to_path {
        query = query.bind(to_path);
    }
    if let Some(status_code) = payload.status_code {
        query = query.bind(status_code);
    }
    if let Some(is_active) = payload.is_active {
        query = query.bind(is_active);
    }

    query = query.bind(Utc::now());
    query = query.bind(id);

    let redirect = query
        .fetch_one(state.db.pool())
        .await
        .map_err(|e| ApiError::database_error(&e.to_string()))?;

    Ok(Json(serde_json::json!({ "data": redirect })))
}

/// DELETE /admin/redirects/:id - Delete a redirect
#[tracing::instrument(skip(state, _admin))]
pub async fn destroy(
    _admin: AdminUser,
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> Result<Json<serde_json::Value>, ApiError> {
    let result = sqlx::query("DELETE FROM redirects WHERE id = $1")
        .bind(id)
        .execute(state.db.pool())
        .await
        .map_err(|e| ApiError::database_error(&e.to_string()))?;

    if result.rows_affected() == 0 {
        return Err(ApiError::not_found("Redirect not found"));
    }

    Ok(Json(
        serde_json::json!({ "message": "Redirect deleted successfully" }),
    ))
}

/// POST /admin/redirects/:id/hit - Increment hit count (for redirect tracking)
#[tracing::instrument(skip(state))]
pub async fn increment_hit(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> Result<Json<serde_json::Value>, ApiError> {
    sqlx::query("UPDATE redirects SET hit_count = hit_count + 1 WHERE id = $1")
        .bind(id)
        .execute(state.db.pool())
        .await
        .map_err(|e| ApiError::database_error(&e.to_string()))?;

    Ok(Json(serde_json::json!({ "success": true })))
}

/// GET /redirect/resolve - Public endpoint to resolve a redirect (for middleware use)
#[tracing::instrument(skip(state))]
pub async fn resolve_redirect(
    State(state): State<AppState>,
    Query(params): Query<std::collections::HashMap<String, String>>,
) -> Result<Json<serde_json::Value>, ApiError> {
    let path = params
        .get("path")
        .ok_or_else(|| ApiError::validation_error("path parameter required"))?;

    let redirect: Option<Redirect> =
        sqlx::query_as("SELECT * FROM redirects WHERE from_path = $1 AND is_active = true")
            .bind(path)
            .fetch_optional(state.db.pool())
            .await
            .map_err(|e| ApiError::database_error(&e.to_string()))?;

    match redirect {
        Some(r) => {
            // Increment hit count asynchronously
            let pool = state.db.pool().clone();
            let rid = r.id;
            tokio::spawn(async move {
                let _ = sqlx::query("UPDATE redirects SET hit_count = hit_count + 1 WHERE id = $1")
                    .bind(rid)
                    .execute(&pool)
                    .await;
            });

            Ok(Json(serde_json::json!({
                "redirect": true,
                "to_path": r.to_path,
                "status_code": r.status_code
            })))
        }
        None => Ok(Json(serde_json::json!({ "redirect": false }))),
    }
}

/// Build the redirects router
pub fn router() -> Router<AppState> {
    Router::new()
        // Admin routes (require auth)
        .route("/admin/redirects", get(index).post(store))
        .route(
            "/admin/redirects/:id",
            get(show).put(update).delete(destroy),
        )
        .route("/admin/redirects/:id/hit", post(increment_hit))
        // Public route for redirect resolution
        .route("/redirect/resolve", get(resolve_redirect))
}
