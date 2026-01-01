//! Indicator routes - Revolution Trading Pros
//! Apple ICT 11+ Principal Engineer Grade - December 2025

use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    routing::{get, post, put, delete},
    Json, Router,
};
use serde::Deserialize;
use serde_json::json;

use crate::{
    models::User,
    AppState,
};

/// Indicator list query
#[derive(Debug, Deserialize)]
pub struct IndicatorListQuery {
    pub page: Option<i64>,
    pub per_page: Option<i64>,
    pub platform: Option<String>,
    pub is_active: Option<bool>,
}

/// Indicator row from database
#[derive(Debug, serde::Serialize, sqlx::FromRow)]
pub struct IndicatorRow {
    pub id: i64,
    pub name: String,
    pub slug: String,
    pub description: Option<String>,
    pub long_description: Option<String>,
    pub price: f64,
    pub is_active: bool,
    pub platform: String,
    pub version: String,
    pub download_url: Option<String>,
    pub documentation_url: Option<String>,
    pub thumbnail: Option<String>,
    pub screenshots: Option<serde_json::Value>,
    pub features: Option<serde_json::Value>,
    pub requirements: Option<serde_json::Value>,
    pub meta_title: Option<String>,
    pub meta_description: Option<String>,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

/// Create indicator request
#[derive(Debug, Deserialize)]
pub struct CreateIndicatorRequest {
    pub name: String,
    pub description: Option<String>,
    pub long_description: Option<String>,
    pub price: f64,
    pub is_active: Option<bool>,
    pub platform: String,
    pub version: Option<String>,
    pub download_url: Option<String>,
    pub documentation_url: Option<String>,
    pub thumbnail: Option<String>,
    pub screenshots: Option<serde_json::Value>,
    pub features: Option<serde_json::Value>,
    pub requirements: Option<serde_json::Value>,
    pub meta_title: Option<String>,
    pub meta_description: Option<String>,
}

/// List all indicators (public)
async fn list_indicators(
    State(state): State<AppState>,
    Query(query): Query<IndicatorListQuery>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let page = query.page.unwrap_or(1).max(1);
    let per_page = query.per_page.unwrap_or(20).min(100);
    let offset = (page - 1) * per_page;

    let mut conditions = vec!["1=1".to_string()];

    if query.is_active.unwrap_or(true) {
        conditions.push("is_active = true".to_string());
    }

    if let Some(ref platform) = query.platform {
        conditions.push(format!("platform = '{}'", platform));
    }

    let where_clause = conditions.join(" AND ");

    let sql = format!(
        "SELECT * FROM indicators WHERE {} ORDER BY created_at DESC LIMIT {} OFFSET {}",
        where_clause, per_page, offset
    );
    let count_sql = format!("SELECT COUNT(*) FROM indicators WHERE {}", where_clause);

    let indicators: Vec<IndicatorRow> = sqlx::query_as(&sql)
        .fetch_all(&state.db.pool)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()}))))?;

    let total: (i64,) = sqlx::query_as(&count_sql)
        .fetch_one(&state.db.pool)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()}))))?;

    Ok(Json(json!({
        "data": indicators,
        "meta": {
            "current_page": page,
            "per_page": per_page,
            "total": total.0,
            "total_pages": (total.0 as f64 / per_page as f64).ceil() as i64
        }
    })))
}

/// Get indicator by slug (public)
async fn get_indicator(
    State(state): State<AppState>,
    Path(slug): Path<String>,
) -> Result<Json<IndicatorRow>, (StatusCode, Json<serde_json::Value>)> {
    let indicator: IndicatorRow = sqlx::query_as("SELECT * FROM indicators WHERE slug = $1")
        .bind(&slug)
        .fetch_optional(&state.db.pool)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()}))))?
        .ok_or_else(|| (StatusCode::NOT_FOUND, Json(json!({"error": "Indicator not found"}))))?;

    Ok(Json(indicator))
}

/// Create indicator (admin only)
async fn create_indicator(
    State(state): State<AppState>,
    user: User,
    Json(input): Json<CreateIndicatorRequest>,
) -> Result<Json<IndicatorRow>, (StatusCode, Json<serde_json::Value>)> {
    // Role check - only admins and super_admins can create indicators
    if user.role.as_deref() != Some("admin") && user.role.as_deref() != Some("super_admin") {
        return Err((
            StatusCode::FORBIDDEN,
            Json(serde_json::json!({"error": "Insufficient permissions to create indicators"}))
        ));
    }

    let slug = slug::slugify(&input.name);
    let version = input.version.unwrap_or_else(|| "1.0.0".to_string());

    let indicator: IndicatorRow = sqlx::query_as(
        r#"
        INSERT INTO indicators (name, slug, description, long_description, price, is_active, platform, version, download_url, documentation_url, thumbnail, screenshots, features, requirements, meta_title, meta_description, created_at, updated_at)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, NOW(), NOW())
        RETURNING *
        "#,
    )
    .bind(&input.name)
    .bind(&slug)
    .bind(&input.description)
    .bind(&input.long_description)
    .bind(input.price)
    .bind(input.is_active.unwrap_or(true))
    .bind(&input.platform)
    .bind(&version)
    .bind(&input.download_url)
    .bind(&input.documentation_url)
    .bind(&input.thumbnail)
    .bind(&input.screenshots)
    .bind(&input.features)
    .bind(&input.requirements)
    .bind(&input.meta_title)
    .bind(&input.meta_description)
    .fetch_one(&state.db.pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()}))))?;

    Ok(Json(indicator))
}

/// Get user's purchased indicators
async fn get_user_indicators(
    State(state): State<AppState>,
    user: User,
) -> Result<Json<Vec<IndicatorRow>>, (StatusCode, Json<serde_json::Value>)> {
    let indicators: Vec<IndicatorRow> = sqlx::query_as(
        r#"
        SELECT i.* FROM indicators i
        INNER JOIN user_indicators ui ON i.id = ui.indicator_id
        WHERE ui.user_id = $1
        ORDER BY ui.purchased_at DESC
        "#
    )
    .bind(user.id)
    .fetch_all(&state.db.pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()}))))?;

    Ok(Json(indicators))
}

/// Download indicator (authenticated, must own)
async fn download_indicator(
    State(state): State<AppState>,
    user: User,
    Path(id): Path<i64>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    // Check ownership
    let ownership: Option<(i64,)> = sqlx::query_as(
        "SELECT id FROM user_indicators WHERE user_id = $1 AND indicator_id = $2"
    )
    .bind(user.id)
    .bind(id)
    .fetch_optional(&state.db.pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()}))))?;

    if ownership.is_none() {
        return Err((StatusCode::FORBIDDEN, Json(json!({"error": "You do not own this indicator"}))));
    }

    // Get download URL
    let indicator: IndicatorRow = sqlx::query_as("SELECT * FROM indicators WHERE id = $1")
        .bind(id)
        .fetch_one(&state.db.pool)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()}))))?;

    Ok(Json(json!({
        "download_url": indicator.download_url,
        "documentation_url": indicator.documentation_url,
        "version": indicator.version
    })))
}

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/", get(list_indicators).post(create_indicator))
        .route("/my", get(get_user_indicators))
        .route("/:slug", get(get_indicator))
        .route("/:id/download", get(download_indicator))
}
