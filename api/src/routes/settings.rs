//! Settings Controller
//! ICT 11+ Principal Engineer - Application settings management
//!
//! Manages key-value application settings with grouping support.

use axum::{
    extract::{Path, State},
    response::Json,
    routing::{get, put},
    Router,
};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use std::collections::HashMap;

use crate::{
    utils::errors::ApiError,
    AppState,
};

#[derive(Debug, Serialize, FromRow)]
pub struct Setting {
    pub id: i64,
    pub key: String,
    pub value: String,
    pub group: Option<String>,
    pub description: Option<String>,
    pub is_public: bool,
}

#[derive(Debug, Deserialize)]
pub struct UpdateSettings {
    pub settings: HashMap<String, String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateSingleSetting {
    pub value: String,
}

/// GET /admin/settings - List all settings grouped
#[tracing::instrument(skip(state))]
pub async fn index(
    State(state): State<AppState>,
) -> Result<Json<serde_json::Value>, ApiError> {
    let settings: Vec<Setting> = sqlx::query_as(
        "SELECT id, key, value, \"group\", description, is_public 
         FROM settings 
         ORDER BY \"group\", key"
    )
    .fetch_all(state.db.pool())
    .await
    .map_err(|e| ApiError::database_error(&e.to_string()))?;

    // Group settings by group field
    let mut grouped: HashMap<String, Vec<Setting>> = HashMap::new();
    for setting in settings {
        let group_key = setting.group.clone().unwrap_or_else(|| "general".to_string());
        grouped.entry(group_key).or_insert_with(Vec::new).push(setting);
    }

    Ok(Json(serde_json::json!(grouped)))
}

/// GET /admin/settings/:key - Get single setting
#[tracing::instrument(skip(state))]
pub async fn show(
    State(state): State<AppState>,
    Path(key): Path<String>,
) -> Result<Json<serde_json::Value>, ApiError> {
    let setting: Setting = sqlx::query_as(
        "SELECT id, key, value, \"group\", description, is_public 
         FROM settings 
         WHERE key = $1"
    )
    .bind(&key)
    .fetch_one(state.db.pool())
    .await
    .map_err(|_| ApiError::not_found("Setting not found"))?;

    Ok(Json(serde_json::json!({ "data": setting })))
}

/// PUT /admin/settings - Bulk update settings
#[tracing::instrument(skip(state))]
pub async fn update(
    State(state): State<AppState>,
    Json(payload): Json<UpdateSettings>,
) -> Result<Json<serde_json::Value>, ApiError> {
    if payload.settings.is_empty() {
        return Err(ApiError::validation_error("Settings object cannot be empty"));
    }

    // Use transaction for bulk updates
    let mut tx = state.db.pool()
        .begin()
        .await
        .map_err(|e| ApiError::database_error(&e.to_string()))?;

    for (key, value) in payload.settings.iter() {
        sqlx::query(
            "INSERT INTO settings (key, value, created_at, updated_at)
             VALUES ($1, $2, NOW(), NOW())
             ON CONFLICT (key) 
             DO UPDATE SET value = $2, updated_at = NOW()"
        )
        .bind(key)
        .bind(value)
        .execute(&mut *tx)
        .await
        .map_err(|e| ApiError::database_error(&e.to_string()))?;
    }

    tx.commit()
        .await
        .map_err(|e| ApiError::database_error(&e.to_string()))?;

    Ok(Json(serde_json::json!({ 
        "message": "Settings updated successfully",
        "updated_count": payload.settings.len()
    })))
}

/// PUT /admin/settings/:key - Update single setting
#[tracing::instrument(skip(state))]
pub async fn update_single(
    State(state): State<AppState>,
    Path(key): Path<String>,
    Json(payload): Json<UpdateSingleSetting>,
) -> Result<Json<serde_json::Value>, ApiError> {
    // Check if setting exists
    let exists: bool = sqlx::query_scalar(
        "SELECT EXISTS(SELECT 1 FROM settings WHERE key = $1)"
    )
    .bind(&key)
    .fetch_one(state.db.pool())
    .await
    .map_err(|e| ApiError::database_error(&e.to_string()))?;

    if !exists {
        return Err(ApiError::not_found("Setting not found"));
    }

    let setting: Setting = sqlx::query_as(
        "UPDATE settings 
         SET value = $1, updated_at = NOW() 
         WHERE key = $2 
         RETURNING id, key, value, \"group\", description, is_public"
    )
    .bind(&payload.value)
    .bind(&key)
    .fetch_one(state.db.pool())
    .await
    .map_err(|e| ApiError::database_error(&e.to_string()))?;

    Ok(Json(serde_json::json!({ 
        "message": "Setting updated successfully",
        "setting": setting
    })))
}

/// Build the settings router
pub fn router() -> Router<AppState> {
    Router::new()
        .route("/admin/settings", get(index).put(update))
        .route("/admin/settings/:key", get(show).put(update_single))
}
