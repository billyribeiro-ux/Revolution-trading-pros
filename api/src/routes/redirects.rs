//! Redirects Controller
//! ICT 11+ Principal Engineer - URL redirect management
//!
//! Manages URL redirects for SEO and site maintenance.

use axum::{
    extract::{Path, Query, State},
    response::Json,
    routing::{delete, get, post, put},
    Router,
};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use chrono::{DateTime, Utc};

use crate::{
    utils::errors::ApiError,
    AppState,
};

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

/// GET /redirects - List all redirects
#[tracing::instrument(skip(state))]
pub async fn index(
    State(state): State<AppState>,
    Query(params): Query<RedirectQuery>,
) -> Result<Json<serde_json::Value>, ApiError> {
    let mut query = String::from("SELECT * FROM redirects WHERE 1=1");
    let mut conditions = Vec::new();

    // Search
    if let Some(search) = &params.search {
        conditions.push(format!(
            "(from_path ILIKE '%{}%' OR to_path ILIKE '%{}%')",
            search.replace('\'', "''"),
            search.replace('\'', "''")
        ));
    }

    // Filter by active status
    if let Some(is_active) = params.is_active {
        conditions.push(format!("is_active = {}", is_active));
    }

    if !conditions.is_empty() {
        query.push_str(&format!(" AND {}", conditions.join(" AND ")));
    }

    query.push_str(" ORDER BY created_at DESC");

    // Pagination
    let per_page = params.per_page.unwrap_or(50);
    let page = params.page.unwrap_or(1);
    let offset = (page - 1) * per_page;

    query.push_str(&format!(" LIMIT {} OFFSET {}", per_page, offset));

    let redirects: Vec<Redirect> = sqlx::query_as(&query)
        .fetch_all(state.db.pool())
        .await
        .map_err(|e| ApiError::database_error(&e.to_string()))?;

    Ok(Json(serde_json::json!({ "data": redirects })))
}

/// GET /redirects/:id - Get a single redirect
#[tracing::instrument(skip(state))]
pub async fn show(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> Result<Json<serde_json::Value>, ApiError> {
    let redirect: Redirect = sqlx::query_as(
        "SELECT * FROM redirects WHERE id = $1"
    )
    .bind(id)
    .fetch_one(state.db.pool())
    .await
    .map_err(|_| ApiError::not_found("Redirect not found"))?;

    Ok(Json(serde_json::json!({ "data": redirect })))
}

/// POST /redirects - Create a new redirect
#[tracing::instrument(skip(state))]
pub async fn store(
    State(state): State<AppState>,
    Json(payload): Json<CreateRedirect>,
) -> Result<Json<serde_json::Value>, ApiError> {
    // Validate status code
    let status_code = payload.status_code.unwrap_or(301);
    if ![301, 302, 307, 308].contains(&status_code) {
        return Err(ApiError::validation_error("Status code must be 301, 302, 307, or 308"));
    }

    // Check if from_path already exists
    let exists: bool = sqlx::query_scalar(
        "SELECT EXISTS(SELECT 1 FROM redirects WHERE from_path = $1)"
    )
    .bind(&payload.from_path)
    .fetch_one(state.db.pool())
    .await
    .map_err(|e| ApiError::database_error(&e.to_string()))?;

    if exists {
        return Err(ApiError::validation_error("Redirect from this path already exists"));
    }

    let redirect_type = payload.redirect_type.unwrap_or_else(|| "manual".to_string());

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

/// PUT /redirects/:id - Update a redirect
#[tracing::instrument(skip(state))]
pub async fn update(
    State(state): State<AppState>,
    Path(id): Path<i64>,
    Json(payload): Json<UpdateRedirect>,
) -> Result<Json<serde_json::Value>, ApiError> {
    // Check if redirect exists
    let exists: bool = sqlx::query_scalar(
        "SELECT EXISTS(SELECT 1 FROM redirects WHERE id = $1)"
    )
    .bind(id)
    .fetch_one(state.db.pool())
    .await
    .map_err(|e| ApiError::database_error(&e.to_string()))?;

    if !exists {
        return Err(ApiError::not_found("Redirect not found"));
    }

    // Validate status code if provided
    if let Some(status_code) = payload.status_code {
        if ![301, 302, 307, 308].contains(&status_code) {
            return Err(ApiError::validation_error("Status code must be 301, 302, 307, or 308"));
        }
    }

    // Check if from_path already exists for another redirect
    if let Some(from_path) = &payload.from_path {
        let path_exists: bool = sqlx::query_scalar(
            "SELECT EXISTS(SELECT 1 FROM redirects WHERE from_path = $1 AND id != $2)"
        )
        .bind(from_path)
        .bind(id)
        .fetch_one(state.db.pool())
        .await
        .map_err(|e| ApiError::database_error(&e.to_string()))?;

        if path_exists {
            return Err(ApiError::validation_error("Redirect from this path already exists"));
        }
    }

    // Build dynamic update query
    let mut updates = Vec::new();
    let mut param_count = 1;

    if payload.from_path.is_some() { updates.push(format!("from_path = ${}", param_count)); param_count += 1; }
    if payload.to_path.is_some() { updates.push(format!("to_path = ${}", param_count)); param_count += 1; }
    if payload.status_code.is_some() { updates.push(format!("status_code = ${}", param_count)); param_count += 1; }
    if payload.is_active.is_some() { updates.push(format!("is_active = ${}", param_count)); param_count += 1; }

    updates.push(format!("updated_at = ${}", param_count));

    let query_str = format!(
        "UPDATE redirects SET {} WHERE id = ${} RETURNING *",
        updates.join(", "),
        param_count + 1
    );

    let mut query = sqlx::query_as::<_, Redirect>(&query_str);

    if let Some(from_path) = payload.from_path { query = query.bind(from_path); }
    if let Some(to_path) = payload.to_path { query = query.bind(to_path); }
    if let Some(status_code) = payload.status_code { query = query.bind(status_code); }
    if let Some(is_active) = payload.is_active { query = query.bind(is_active); }

    query = query.bind(Utc::now());
    query = query.bind(id);

    let redirect = query
        .fetch_one(state.db.pool())
        .await
        .map_err(|e| ApiError::database_error(&e.to_string()))?;

    Ok(Json(serde_json::json!({ "data": redirect })))
}

/// DELETE /redirects/:id - Delete a redirect
#[tracing::instrument(skip(state))]
pub async fn destroy(
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

    Ok(Json(serde_json::json!({ "message": "Redirect deleted successfully" })))
}

/// Build the redirects router
pub fn router() -> Router<AppState> {
    Router::new()
        .route("/redirects", get(index).post(store))
        .route("/redirects/:id", get(show).put(update).delete(destroy))
}
