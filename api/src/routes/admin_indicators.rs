//! Indicator Management Admin API Routes
//! Apple Principal Engineer ICT 7 Grade - January 2026
//!
//! Full-service indicator management matching existing database schema.
//! Database uses BIGINT id, not UUID.

use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    routing::{delete, get, post, put},
    Json, Router,
};
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use serde_json::json;
use sqlx::FromRow;

use crate::middleware::admin::AdminUser;
use crate::AppState;

// ═══════════════════════════════════════════════════════════════════════════════════
// TYPES - Match actual database schema (BIGINT id)
// ═══════════════════════════════════════════════════════════════════════════════════

#[derive(Debug, Deserialize)]
pub struct IndicatorQueryParams {
    pub page: Option<i32>,
    pub per_page: Option<i32>,
    pub search: Option<String>,
    pub is_active: Option<bool>,
    pub platform: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct IndicatorRow {
    pub id: i64,
    pub name: String,
    pub slug: String,
    pub description: Option<String>,
    pub long_description: Option<String>,
    pub price: Option<f64>,
    pub is_active: Option<bool>,
    pub platform: Option<String>,
    pub version: Option<String>,
    pub download_url: Option<String>,
    pub documentation_url: Option<String>,
    pub thumbnail: Option<String>,
    pub screenshots: Option<serde_json::Value>,
    pub features: Option<serde_json::Value>,
    pub requirements: Option<serde_json::Value>,
    pub meta_title: Option<String>,
    pub meta_description: Option<String>,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Debug, Deserialize)]
pub struct CreateIndicatorRequest {
    pub name: String,
    pub slug: Option<String>,
    pub description: Option<String>,
    pub long_description: Option<String>,
    pub price: Option<f64>,
    pub platform: Option<String>,
    pub version: Option<String>,
    pub download_url: Option<String>,
    pub documentation_url: Option<String>,
    pub thumbnail: Option<String>,
    pub features: Option<serde_json::Value>,
    pub requirements: Option<serde_json::Value>,
    pub meta_title: Option<String>,
    pub meta_description: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateIndicatorRequest {
    pub name: Option<String>,
    pub slug: Option<String>,
    pub description: Option<String>,
    pub long_description: Option<String>,
    pub price: Option<f64>,
    pub is_active: Option<bool>,
    pub platform: Option<String>,
    pub version: Option<String>,
    pub download_url: Option<String>,
    pub documentation_url: Option<String>,
    pub thumbnail: Option<String>,
    pub features: Option<serde_json::Value>,
    pub requirements: Option<serde_json::Value>,
    pub meta_title: Option<String>,
    pub meta_description: Option<String>,
}

// ═══════════════════════════════════════════════════════════════════════════════════
// HELPER FUNCTIONS
// ═══════════════════════════════════════════════════════════════════════════════════

fn slugify(s: &str) -> String {
    s.to_lowercase()
        .chars()
        .map(|c| if c.is_alphanumeric() { c } else { '-' })
        .collect::<String>()
        .split('-')
        .filter(|s| !s.is_empty())
        .collect::<Vec<_>>()
        .join("-")
}

// ═══════════════════════════════════════════════════════════════════════════════════
// HANDLERS
// ═══════════════════════════════════════════════════════════════════════════════════

async fn list_indicators(
    _admin: AdminUser,
    State(state): State<AppState>,
    Query(params): Query<IndicatorQueryParams>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let page = params.page.unwrap_or(1);
    let per_page = params.per_page.unwrap_or(20).min(100);
    let offset = (page - 1) * per_page;

    let mut query = String::from("SELECT * FROM indicators WHERE 1=1");

    if let Some(is_active) = params.is_active {
        query.push_str(&format!(" AND is_active = {}", is_active));
    }

    if let Some(ref platform) = params.platform {
        let platform = platform.replace('\'', "''");
        query.push_str(&format!(" AND platform = '{}'", platform));
    }

    if let Some(ref search) = params.search {
        let search = search.replace('\'', "''");
        query.push_str(&format!(
            " AND (name ILIKE '%{}%' OR description ILIKE '%{}%')",
            search, search
        ));
    }

    query.push_str(" ORDER BY created_at DESC");
    query.push_str(&format!(" LIMIT {} OFFSET {}", per_page, offset));

    let indicators: Vec<IndicatorRow> = sqlx::query_as(&query)
        .fetch_all(&state.db.pool)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": format!("Database error: {}", e)})),
            )
        })?;

    let total: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM indicators")
        .fetch_one(&state.db.pool)
        .await
        .unwrap_or((0,));

    let total_pages = ((total.0 as f64) / (per_page as f64)).ceil() as i32;

    Ok(Json(json!({
        "success": true,
        "data": {
            "indicators": indicators,
            "total": total.0,
            "page": page,
            "per_page": per_page,
            "total_pages": total_pages
        }
    })))
}

async fn get_indicator(
    _admin: AdminUser,
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let indicator: IndicatorRow = if let Ok(numeric_id) = id.parse::<i64>() {
        sqlx::query_as("SELECT * FROM indicators WHERE id = $1")
            .bind(numeric_id)
            .fetch_optional(&state.db.pool)
            .await
    } else {
        sqlx::query_as("SELECT * FROM indicators WHERE slug = $1")
            .bind(&id)
            .fetch_optional(&state.db.pool)
            .await
    }
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": format!("Database error: {}", e)})),
        )
    })?
    .ok_or_else(|| {
        (
            StatusCode::NOT_FOUND,
            Json(json!({"error": "Indicator not found"})),
        )
    })?;

    Ok(Json(json!({
        "success": true,
        "data": indicator
    })))
}

async fn create_indicator(
    _admin: AdminUser,
    State(state): State<AppState>,
    Json(input): Json<CreateIndicatorRequest>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let slug = input.slug.clone().unwrap_or_else(|| slugify(&input.name));

    let existing: Option<(i64,)> = sqlx::query_as("SELECT id FROM indicators WHERE slug = $1")
        .bind(&slug)
        .fetch_optional(&state.db.pool)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": format!("Database error: {}", e)})),
            )
        })?;

    if existing.is_some() {
        return Err((
            StatusCode::CONFLICT,
            Json(json!({"error": "An indicator with this slug already exists"})),
        ));
    }

    let indicator: IndicatorRow = sqlx::query_as(
        r#"INSERT INTO indicators (
            name, slug, description, long_description, price, platform, version,
            download_url, documentation_url, thumbnail, features, requirements,
            meta_title, meta_description, is_active
        )
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, true)
        RETURNING *"#,
    )
    .bind(&input.name)
    .bind(&slug)
    .bind(&input.description)
    .bind(&input.long_description)
    .bind(input.price.unwrap_or(0.0))
    .bind(&input.platform)
    .bind(input.version.as_deref().unwrap_or("1.0"))
    .bind(&input.download_url)
    .bind(&input.documentation_url)
    .bind(&input.thumbnail)
    .bind(&input.features)
    .bind(&input.requirements)
    .bind(&input.meta_title)
    .bind(&input.meta_description)
    .fetch_one(&state.db.pool)
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": format!("Failed to create indicator: {}", e)})),
        )
    })?;

    Ok(Json(json!({
        "success": true,
        "message": "Indicator created successfully",
        "data": indicator
    })))
}

async fn update_indicator(
    _admin: AdminUser,
    State(state): State<AppState>,
    Path(id): Path<i64>,
    Json(input): Json<UpdateIndicatorRequest>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let indicator: IndicatorRow = sqlx::query_as(
        r#"UPDATE indicators SET
            name = COALESCE($1, name),
            slug = COALESCE($2, slug),
            description = COALESCE($3, description),
            long_description = COALESCE($4, long_description),
            price = COALESCE($5, price),
            is_active = COALESCE($6, is_active),
            platform = COALESCE($7, platform),
            version = COALESCE($8, version),
            download_url = COALESCE($9, download_url),
            documentation_url = COALESCE($10, documentation_url),
            thumbnail = COALESCE($11, thumbnail),
            features = COALESCE($12, features),
            requirements = COALESCE($13, requirements),
            meta_title = COALESCE($14, meta_title),
            meta_description = COALESCE($15, meta_description),
            updated_at = NOW()
        WHERE id = $16
        RETURNING *"#,
    )
    .bind(&input.name)
    .bind(&input.slug)
    .bind(&input.description)
    .bind(&input.long_description)
    .bind(input.price)
    .bind(input.is_active)
    .bind(&input.platform)
    .bind(&input.version)
    .bind(&input.download_url)
    .bind(&input.documentation_url)
    .bind(&input.thumbnail)
    .bind(&input.features)
    .bind(&input.requirements)
    .bind(&input.meta_title)
    .bind(&input.meta_description)
    .bind(id)
    .fetch_one(&state.db.pool)
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": format!("Failed to update indicator: {}", e)})),
        )
    })?;

    Ok(Json(json!({
        "success": true,
        "message": "Indicator updated successfully",
        "data": indicator
    })))
}

async fn delete_indicator(
    _admin: AdminUser,
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    sqlx::query("DELETE FROM indicators WHERE id = $1")
        .bind(id)
        .execute(&state.db.pool)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": format!("Failed to delete indicator: {}", e)})),
            )
        })?;

    Ok(Json(json!({
        "success": true,
        "message": "Indicator deleted successfully"
    })))
}

async fn toggle_indicator(
    _admin: AdminUser,
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let indicator: IndicatorRow = sqlx::query_as(
        r#"UPDATE indicators 
           SET is_active = NOT COALESCE(is_active, false), updated_at = NOW()
           WHERE id = $1
           RETURNING *"#,
    )
    .bind(id)
    .fetch_one(&state.db.pool)
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": format!("Failed to toggle indicator: {}", e)})),
        )
    })?;

    let status = if indicator.is_active.unwrap_or(false) { "activated" } else { "deactivated" };

    Ok(Json(json!({
        "success": true,
        "message": format!("Indicator {}", status),
        "data": indicator
    })))
}

// ═══════════════════════════════════════════════════════════════════════════════════
// ROUTER
// ═══════════════════════════════════════════════════════════════════════════════════

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/", get(list_indicators).post(create_indicator))
        .route("/:id", get(get_indicator).put(update_indicator).delete(delete_indicator))
        .route("/:id/toggle", post(toggle_indicator))
}
