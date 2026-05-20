//! Admin application settings — extracted from the original
//! `routes/admin.rs` as part of the R6-B split (2026-05-20).
//!
//! Handler bodies are byte-identical to the pre-split source.

use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use serde::Deserialize;
use serde_json::json;

use crate::{middleware::admin::AdminUser, AppState};

// ═══════════════════════════════════════════════════════════════════════════
// SETTINGS MANAGEMENT
// ═══════════════════════════════════════════════════════════════════════════

#[derive(Debug, serde::Serialize, sqlx::FromRow)]
pub struct SettingRow {
    pub id: i64,
    pub key: String,
    pub value: Option<serde_json::Value>,
    pub group_name: Option<String>,
    pub description: Option<String>,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

/// Get all settings (admin)
/// ICT 7 SECURITY FIX: Added require_admin check
pub(super) async fn get_settings(
    State(state): State<AppState>,
    AdminUser(user): AdminUser,
) -> Result<Json<Vec<SettingRow>>, (StatusCode, Json<serde_json::Value>)> {
    let settings: Vec<SettingRow> =
        sqlx::query_as("SELECT * FROM application_settings ORDER BY group_name, key")
            .fetch_all(&state.db.pool)
            .await
            .map_err(|e| {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({"error": e.to_string()})),
                )
            })?;

    Ok(Json(settings))
}

/// Get setting by key (admin)
/// ICT 7 SECURITY FIX: Added require_admin check
pub(super) async fn get_setting(
    State(state): State<AppState>,
    AdminUser(user): AdminUser,
    Path(key): Path<String>,
) -> Result<Json<SettingRow>, (StatusCode, Json<serde_json::Value>)> {
    let setting: SettingRow = sqlx::query_as("SELECT * FROM application_settings WHERE key = $1")
        .bind(&key)
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
                Json(json!({"error": "Setting not found"})),
            )
        })?;

    Ok(Json(setting))
}

#[derive(Debug, Deserialize)]
pub struct UpdateSettingRequest {
    pub value: serde_json::Value,
}

/// Update setting (admin)
/// ICT 7 SECURITY FIX: Added require_admin check
pub(super) async fn update_setting(
    State(state): State<AppState>,
    AdminUser(user): AdminUser,
    Path(key): Path<String>,
    Json(input): Json<UpdateSettingRequest>,
) -> Result<Json<SettingRow>, (StatusCode, Json<serde_json::Value>)> {
    let setting: SettingRow = sqlx::query_as(
        "UPDATE application_settings SET value = $1, updated_at = NOW() WHERE key = $2 RETURNING *",
    )
    .bind(&input.value)
    .bind(&key)
    .fetch_one(&state.db.pool)
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": e.to_string()})),
        )
    })?;

    Ok(Json(setting))
}
