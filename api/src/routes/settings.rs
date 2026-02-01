//! Settings Controller - ICT Level 7 Principal Engineer Grade
//! Apple ICT 11+ Principal Engineer - Application settings management
//!
//! SECURITY: All admin endpoints require AdminUser authentication
//! Features:
//! - Site settings CRUD with grouping
//! - Feature flags management
//! - Sensitive settings protection (API keys masked)
//! - Audit logging for all changes
//! - Public settings endpoint for non-sensitive data

use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::Json,
    routing::{delete, get, post, put},
    Router,
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::json;
use sqlx::FromRow;
use std::collections::HashMap;

use crate::{
    middleware::admin::AdminUser,
    utils::errors::ApiError,
    AppState,
};

// ═══════════════════════════════════════════════════════════════════════════════
// TYPES
// ═══════════════════════════════════════════════════════════════════════════════

#[derive(Debug, Serialize, FromRow)]
pub struct Setting {
    pub id: i64,
    pub key: String,
    pub value: String,
    #[sqlx(rename = "group")]
    pub setting_group: Option<String>,
    pub description: Option<String>,
    pub is_public: bool,
    pub is_sensitive: bool,
    pub value_type: Option<String>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize)]
pub struct SettingResponse {
    pub id: i64,
    pub key: String,
    pub value: String,
    pub group: Option<String>,
    pub description: Option<String>,
    pub is_public: bool,
    pub is_sensitive: bool,
    pub value_type: Option<String>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

impl From<Setting> for SettingResponse {
    fn from(s: Setting) -> Self {
        SettingResponse {
            id: s.id,
            key: s.key,
            value: s.value,
            group: s.setting_group,
            description: s.description,
            is_public: s.is_public,
            is_sensitive: s.is_sensitive,
            value_type: s.value_type,
            created_at: s.created_at,
            updated_at: s.updated_at,
        }
    }
}

#[derive(Debug, Serialize, FromRow)]
pub struct FeatureFlag {
    pub id: i64,
    pub key: String,
    pub enabled: bool,
    pub description: Option<String>,
    pub conditions: Option<serde_json::Value>,
    pub rollout_percentage: Option<i32>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateSettings {
    pub settings: HashMap<String, String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateSingleSetting {
    pub value: String,
}

#[derive(Debug, Deserialize)]
pub struct CreateSettingRequest {
    pub key: String,
    pub value: String,
    pub group: Option<String>,
    pub description: Option<String>,
    pub is_public: Option<bool>,
    pub is_sensitive: Option<bool>,
    pub value_type: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct CreateFeatureFlagRequest {
    pub key: String,
    pub enabled: Option<bool>,
    pub description: Option<String>,
    pub conditions: Option<serde_json::Value>,
    pub rollout_percentage: Option<i32>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateFeatureFlagRequest {
    pub enabled: Option<bool>,
    pub description: Option<String>,
    pub conditions: Option<serde_json::Value>,
    pub rollout_percentage: Option<i32>,
}

#[derive(Debug, Deserialize)]
pub struct SettingsQuery {
    pub group: Option<String>,
    pub search: Option<String>,
}

// ═══════════════════════════════════════════════════════════════════════════════
// HELPER FUNCTIONS
// ═══════════════════════════════════════════════════════════════════════════════

/// Mask sensitive values (show only last 4 chars for API keys)
fn mask_sensitive_value(value: &str) -> String {
    if value.len() <= 8 {
        "*".repeat(value.len())
    } else {
        format!("{}...{}", "*".repeat(8), &value[value.len() - 4..])
    }
}

/// Log audit event for settings changes
async fn log_settings_audit(
    pool: &sqlx::PgPool,
    admin_id: i64,
    admin_email: &str,
    action: &str,
    entity_type: &str,
    entity_id: Option<i64>,
    old_value: Option<&str>,
    new_value: Option<&str>,
    metadata: Option<serde_json::Value>,
) {
    let _ = sqlx::query(
        r#"
        INSERT INTO admin_audit_logs (
            admin_id, admin_email, action, entity_type, entity_id,
            old_value, new_value, metadata, created_at
        ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, NOW())
        "#,
    )
    .bind(admin_id)
    .bind(admin_email)
    .bind(action)
    .bind(entity_type)
    .bind(entity_id)
    .bind(old_value)
    .bind(new_value)
    .bind(metadata)
    .execute(pool)
    .await;
}

// ═══════════════════════════════════════════════════════════════════════════════
// ADMIN SETTINGS ENDPOINTS (require AdminUser)
// ═══════════════════════════════════════════════════════════════════════════════

/// GET /admin/settings - List all settings grouped (admin only)
#[tracing::instrument(skip(state, admin))]
pub async fn index(
    State(state): State<AppState>,
    admin: AdminUser,
    Query(query): Query<SettingsQuery>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    tracing::info!(
        admin_id = admin.0.id,
        admin_email = %admin.0.email,
        "Admin fetching all settings"
    );

    let mut sql = String::from(
        r#"
        SELECT id, key, value, "group" as setting_group, description,
               is_public, COALESCE(is_sensitive, false) as is_sensitive,
               value_type, created_at, updated_at
        FROM settings
        WHERE 1=1
        "#,
    );

    let mut bind_count = 0;

    if query.group.is_some() {
        bind_count += 1;
        sql.push_str(&format!(" AND \"group\" = ${}", bind_count));
    }

    if query.search.is_some() {
        bind_count += 1;
        sql.push_str(&format!(
            " AND (key ILIKE '%' || ${} || '%' OR description ILIKE '%' || ${} || '%')",
            bind_count, bind_count
        ));
    }

    sql.push_str(" ORDER BY \"group\" NULLS LAST, key");

    let mut query_builder = sqlx::query_as::<_, Setting>(&sql);

    if let Some(ref group) = query.group {
        query_builder = query_builder.bind(group);
    }
    if let Some(ref search) = query.search {
        query_builder = query_builder.bind(search);
    }

    let settings: Vec<Setting> = query_builder
        .fetch_all(state.db.pool())
        .await
        .map_err(|e| {
            tracing::error!("Database error fetching settings: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": "Failed to fetch settings"})),
            )
        })?;

    // Process settings - mask sensitive values
    let processed: Vec<SettingResponse> = settings
        .into_iter()
        .map(|mut s| {
            if s.is_sensitive {
                s.value = mask_sensitive_value(&s.value);
            }
            s.into()
        })
        .collect();

    // Group settings by group field
    let mut grouped: HashMap<String, Vec<SettingResponse>> = HashMap::new();
    for setting in processed {
        let group_key = setting.group.clone().unwrap_or_else(|| "general".to_string());
        grouped.entry(group_key).or_default().push(setting);
    }

    Ok(Json(json!({
        "success": true,
        "data": grouped
    })))
}

/// GET /admin/settings/:key - Get single setting (admin only)
#[tracing::instrument(skip(state, admin))]
pub async fn show(
    State(state): State<AppState>,
    admin: AdminUser,
    Path(key): Path<String>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    // Validate key - prevent SQL injection via path
    if key.contains(';') || key.contains("--") || key.len() > 255 {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(json!({"error": "Invalid setting key"})),
        ));
    }

    tracing::info!(
        admin_id = admin.0.id,
        key = %key,
        "Admin fetching setting"
    );

    let setting: Option<Setting> = sqlx::query_as(
        r#"
        SELECT id, key, value, "group" as setting_group, description,
               is_public, COALESCE(is_sensitive, false) as is_sensitive,
               value_type, created_at, updated_at
        FROM settings
        WHERE key = $1
        "#,
    )
    .bind(&key)
    .fetch_optional(state.db.pool())
    .await
    .map_err(|e| {
        tracing::error!("Database error: {}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": "Database error"})),
        )
    })?;

    match setting {
        Some(mut s) => {
            // Mask sensitive value for display
            if s.is_sensitive {
                s.value = mask_sensitive_value(&s.value);
            }
            Ok(Json(json!({
                "success": true,
                "data": SettingResponse::from(s)
            })))
        }
        None => Err((
            StatusCode::NOT_FOUND,
            Json(json!({"error": "Setting not found"})),
        )),
    }
}

/// POST /admin/settings - Create new setting (admin only)
#[tracing::instrument(skip(state, admin, payload))]
pub async fn create(
    State(state): State<AppState>,
    admin: AdminUser,
    Json(payload): Json<CreateSettingRequest>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    // Validate key
    if payload.key.is_empty() || payload.key.len() > 255 {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(json!({"error": "Invalid setting key"})),
        ));
    }

    // Check if key already exists
    let exists: bool = sqlx::query_scalar("SELECT EXISTS(SELECT 1 FROM settings WHERE key = $1)")
        .bind(&payload.key)
        .fetch_one(state.db.pool())
        .await
        .unwrap_or(false);

    if exists {
        return Err((
            StatusCode::CONFLICT,
            Json(json!({"error": "Setting with this key already exists"})),
        ));
    }

    let setting: Setting = sqlx::query_as(
        r#"
        INSERT INTO settings (key, value, "group", description, is_public, is_sensitive, value_type, created_at, updated_at)
        VALUES ($1, $2, $3, $4, $5, $6, $7, NOW(), NOW())
        RETURNING id, key, value, "group" as setting_group, description, is_public,
                  COALESCE(is_sensitive, false) as is_sensitive, value_type, created_at, updated_at
        "#,
    )
    .bind(&payload.key)
    .bind(&payload.value)
    .bind(&payload.group)
    .bind(&payload.description)
    .bind(payload.is_public.unwrap_or(false))
    .bind(payload.is_sensitive.unwrap_or(false))
    .bind(&payload.value_type)
    .fetch_one(state.db.pool())
    .await
    .map_err(|e| {
        tracing::error!("Failed to create setting: {}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": "Failed to create setting"})),
        )
    })?;

    // Audit log
    log_settings_audit(
        state.db.pool(),
        admin.0.id,
        &admin.0.email,
        "setting.created",
        "setting",
        Some(setting.id),
        None,
        Some(&payload.value),
        Some(json!({"key": payload.key})),
    )
    .await;

    tracing::info!(
        admin_id = admin.0.id,
        setting_id = setting.id,
        key = %payload.key,
        "Setting created"
    );

    Ok(Json(json!({
        "success": true,
        "message": "Setting created successfully",
        "data": SettingResponse::from(setting)
    })))
}

/// PUT /admin/settings - Bulk update settings (admin only)
#[tracing::instrument(skip(state, admin, payload))]
pub async fn update(
    State(state): State<AppState>,
    admin: AdminUser,
    Json(payload): Json<UpdateSettings>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    if payload.settings.is_empty() {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(json!({"error": "Settings object cannot be empty"})),
        ));
    }

    // Validate all keys
    for key in payload.settings.keys() {
        if key.contains(';') || key.contains("--") || key.len() > 255 {
            return Err((
                StatusCode::BAD_REQUEST,
                Json(json!({"error": format!("Invalid setting key: {}", key)})),
            ));
        }
    }

    // Use transaction for bulk updates
    let mut tx = state
        .db
        .pool()
        .begin()
        .await
        .map_err(|e| {
            tracing::error!("Transaction start failed: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": "Database error"})),
            )
        })?;

    let mut updated_count = 0;

    for (key, value) in payload.settings.iter() {
        // Get old value for audit
        let old_value: Option<String> = sqlx::query_scalar(
            "SELECT value FROM settings WHERE key = $1"
        )
        .bind(key)
        .fetch_optional(&mut *tx)
        .await
        .ok()
        .flatten();

        let result = sqlx::query(
            r#"
            INSERT INTO settings (key, value, created_at, updated_at)
            VALUES ($1, $2, NOW(), NOW())
            ON CONFLICT (key)
            DO UPDATE SET value = $2, updated_at = NOW()
            "#,
        )
        .bind(key)
        .bind(value)
        .execute(&mut *tx)
        .await
        .map_err(|e| {
            tracing::error!("Failed to update setting {}: {}", key, e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": format!("Failed to update setting: {}", key)})),
            )
        })?;

        if result.rows_affected() > 0 {
            updated_count += 1;

            // Audit log for each change
            let _ = sqlx::query(
                r#"
                INSERT INTO admin_audit_logs (
                    admin_id, admin_email, action, entity_type,
                    old_value, new_value, metadata, created_at
                ) VALUES ($1, $2, 'setting.updated', 'setting', $3, $4, $5, NOW())
                "#,
            )
            .bind(admin.0.id)
            .bind(&admin.0.email)
            .bind(&old_value)
            .bind(value)
            .bind(json!({"key": key}))
            .execute(&mut *tx)
            .await;
        }
    }

    tx.commit().await.map_err(|e| {
        tracing::error!("Transaction commit failed: {}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": "Failed to commit changes"})),
        )
    })?;

    tracing::info!(
        admin_id = admin.0.id,
        updated_count = updated_count,
        "Bulk settings update completed"
    );

    Ok(Json(json!({
        "success": true,
        "message": "Settings updated successfully",
        "updated_count": updated_count
    })))
}

/// PUT /admin/settings/:key - Update single setting (admin only)
#[tracing::instrument(skip(state, admin, payload))]
pub async fn update_single(
    State(state): State<AppState>,
    admin: AdminUser,
    Path(key): Path<String>,
    Json(payload): Json<UpdateSingleSetting>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    // Validate key
    if key.contains(';') || key.contains("--") || key.len() > 255 {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(json!({"error": "Invalid setting key"})),
        ));
    }

    // Get existing setting with old value
    let old_setting: Option<Setting> = sqlx::query_as(
        r#"
        SELECT id, key, value, "group" as setting_group, description,
               is_public, COALESCE(is_sensitive, false) as is_sensitive,
               value_type, created_at, updated_at
        FROM settings WHERE key = $1
        "#,
    )
    .bind(&key)
    .fetch_optional(state.db.pool())
    .await
    .map_err(|e| {
        tracing::error!("Database error: {}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": "Database error"})),
        )
    })?;

    let old_setting = old_setting.ok_or((
        StatusCode::NOT_FOUND,
        Json(json!({"error": "Setting not found"})),
    ))?;

    let old_value = old_setting.value.clone();

    let setting: Setting = sqlx::query_as(
        r#"
        UPDATE settings
        SET value = $1, updated_at = NOW()
        WHERE key = $2
        RETURNING id, key, value, "group" as setting_group, description,
                  is_public, COALESCE(is_sensitive, false) as is_sensitive,
                  value_type, created_at, updated_at
        "#,
    )
    .bind(&payload.value)
    .bind(&key)
    .fetch_one(state.db.pool())
    .await
    .map_err(|e| {
        tracing::error!("Failed to update setting: {}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": "Failed to update setting"})),
        )
    })?;

    // Audit log
    log_settings_audit(
        state.db.pool(),
        admin.0.id,
        &admin.0.email,
        "setting.updated",
        "setting",
        Some(setting.id),
        Some(&old_value),
        Some(&payload.value),
        Some(json!({"key": key})),
    )
    .await;

    tracing::info!(
        admin_id = admin.0.id,
        setting_id = setting.id,
        key = %key,
        "Setting updated"
    );

    Ok(Json(json!({
        "success": true,
        "message": "Setting updated successfully",
        "data": SettingResponse::from(setting)
    })))
}

/// DELETE /admin/settings/:key - Delete setting (admin only)
#[tracing::instrument(skip(state, admin))]
pub async fn delete_setting(
    State(state): State<AppState>,
    admin: AdminUser,
    Path(key): Path<String>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    // Validate key
    if key.contains(';') || key.contains("--") || key.len() > 255 {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(json!({"error": "Invalid setting key"})),
        ));
    }

    // Get setting for audit
    let old_setting: Option<Setting> = sqlx::query_as(
        r#"
        SELECT id, key, value, "group" as setting_group, description,
               is_public, COALESCE(is_sensitive, false) as is_sensitive,
               value_type, created_at, updated_at
        FROM settings WHERE key = $1
        "#,
    )
    .bind(&key)
    .fetch_optional(state.db.pool())
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": e.to_string()})),
        )
    })?;

    let old_setting = old_setting.ok_or((
        StatusCode::NOT_FOUND,
        Json(json!({"error": "Setting not found"})),
    ))?;

    sqlx::query("DELETE FROM settings WHERE key = $1")
        .bind(&key)
        .execute(state.db.pool())
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": e.to_string()})),
            )
        })?;

    // Audit log
    log_settings_audit(
        state.db.pool(),
        admin.0.id,
        &admin.0.email,
        "setting.deleted",
        "setting",
        Some(old_setting.id),
        Some(&old_setting.value),
        None,
        Some(json!({"key": key})),
    )
    .await;

    tracing::info!(
        admin_id = admin.0.id,
        key = %key,
        "Setting deleted"
    );

    Ok(Json(json!({
        "success": true,
        "message": "Setting deleted successfully"
    })))
}

// ═══════════════════════════════════════════════════════════════════════════════
// FEATURE FLAGS ENDPOINTS (require AdminUser)
// ═══════════════════════════════════════════════════════════════════════════════

/// GET /admin/settings/feature-flags - List all feature flags
#[tracing::instrument(skip(state, _admin))]
pub async fn list_feature_flags(
    State(state): State<AppState>,
    _admin: AdminUser,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let flags: Vec<FeatureFlag> = sqlx::query_as(
        r#"
        SELECT id, key, enabled, description, conditions,
               rollout_percentage, created_at, updated_at
        FROM feature_flags
        ORDER BY key
        "#,
    )
    .fetch_all(state.db.pool())
    .await
    .unwrap_or_default();

    Ok(Json(json!({
        "success": true,
        "data": flags
    })))
}

/// GET /admin/settings/feature-flags/:key - Get single feature flag
#[tracing::instrument(skip(state, _admin))]
pub async fn get_feature_flag(
    State(state): State<AppState>,
    _admin: AdminUser,
    Path(key): Path<String>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let flag: Option<FeatureFlag> = sqlx::query_as(
        r#"
        SELECT id, key, enabled, description, conditions,
               rollout_percentage, created_at, updated_at
        FROM feature_flags
        WHERE key = $1
        "#,
    )
    .bind(&key)
    .fetch_optional(state.db.pool())
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": e.to_string()})),
        )
    })?;

    match flag {
        Some(f) => Ok(Json(json!({
            "success": true,
            "data": f
        }))),
        None => Err((
            StatusCode::NOT_FOUND,
            Json(json!({"error": "Feature flag not found"})),
        )),
    }
}

/// POST /admin/settings/feature-flags - Create feature flag
#[tracing::instrument(skip(state, admin, payload))]
pub async fn create_feature_flag(
    State(state): State<AppState>,
    admin: AdminUser,
    Json(payload): Json<CreateFeatureFlagRequest>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    // Validate key format (alphanumeric, underscores, hyphens only)
    if !payload
        .key
        .chars()
        .all(|c| c.is_alphanumeric() || c == '_' || c == '-')
    {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(json!({"error": "Invalid feature flag key format"})),
        ));
    }

    let flag: FeatureFlag = sqlx::query_as(
        r#"
        INSERT INTO feature_flags (key, enabled, description, conditions, rollout_percentage, created_at, updated_at)
        VALUES ($1, $2, $3, $4, $5, NOW(), NOW())
        RETURNING id, key, enabled, description, conditions, rollout_percentage, created_at, updated_at
        "#,
    )
    .bind(&payload.key)
    .bind(payload.enabled.unwrap_or(false))
    .bind(&payload.description)
    .bind(&payload.conditions)
    .bind(payload.rollout_percentage)
    .fetch_one(state.db.pool())
    .await
    .map_err(|e| {
        if e.to_string().contains("duplicate") {
            (
                StatusCode::CONFLICT,
                Json(json!({"error": "Feature flag with this key already exists"})),
            )
        } else {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": e.to_string()})),
            )
        }
    })?;

    // Audit log
    log_settings_audit(
        state.db.pool(),
        admin.0.id,
        &admin.0.email,
        "feature_flag.created",
        "feature_flag",
        Some(flag.id),
        None,
        None,
        Some(json!({"key": payload.key, "enabled": payload.enabled})),
    )
    .await;

    Ok(Json(json!({
        "success": true,
        "message": "Feature flag created",
        "data": flag
    })))
}

/// PUT /admin/settings/feature-flags/:key - Update feature flag
#[tracing::instrument(skip(state, admin, payload))]
pub async fn update_feature_flag(
    State(state): State<AppState>,
    admin: AdminUser,
    Path(key): Path<String>,
    Json(payload): Json<UpdateFeatureFlagRequest>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    // Get old state for audit
    let old_flag: Option<FeatureFlag> = sqlx::query_as(
        "SELECT id, key, enabled, description, conditions, rollout_percentage, created_at, updated_at FROM feature_flags WHERE key = $1"
    )
    .bind(&key)
    .fetch_optional(state.db.pool())
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": e.to_string()})),
        )
    })?;

    let old_flag = old_flag.ok_or((
        StatusCode::NOT_FOUND,
        Json(json!({"error": "Feature flag not found"})),
    ))?;

    let flag: FeatureFlag = sqlx::query_as(
        r#"
        UPDATE feature_flags
        SET enabled = COALESCE($2, enabled),
            description = COALESCE($3, description),
            conditions = COALESCE($4, conditions),
            rollout_percentage = COALESCE($5, rollout_percentage),
            updated_at = NOW()
        WHERE key = $1
        RETURNING id, key, enabled, description, conditions, rollout_percentage, created_at, updated_at
        "#,
    )
    .bind(&key)
    .bind(payload.enabled)
    .bind(&payload.description)
    .bind(&payload.conditions)
    .bind(payload.rollout_percentage)
    .fetch_one(state.db.pool())
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": e.to_string()})),
        )
    })?;

    // Audit log
    log_settings_audit(
        state.db.pool(),
        admin.0.id,
        &admin.0.email,
        "feature_flag.updated",
        "feature_flag",
        Some(flag.id),
        None,
        None,
        Some(json!({
            "key": key,
            "old_enabled": old_flag.enabled,
            "new_enabled": flag.enabled
        })),
    )
    .await;

    Ok(Json(json!({
        "success": true,
        "message": "Feature flag updated",
        "data": flag
    })))
}

/// DELETE /admin/settings/feature-flags/:key - Delete feature flag
#[tracing::instrument(skip(state, admin))]
pub async fn delete_feature_flag(
    State(state): State<AppState>,
    admin: AdminUser,
    Path(key): Path<String>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let result = sqlx::query("DELETE FROM feature_flags WHERE key = $1")
        .bind(&key)
        .execute(state.db.pool())
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
            Json(json!({"error": "Feature flag not found"})),
        ));
    }

    // Audit log
    log_settings_audit(
        state.db.pool(),
        admin.0.id,
        &admin.0.email,
        "feature_flag.deleted",
        "feature_flag",
        None,
        None,
        None,
        Some(json!({"key": key})),
    )
    .await;

    Ok(Json(json!({
        "success": true,
        "message": "Feature flag deleted"
    })))
}

// ═══════════════════════════════════════════════════════════════════════════════
// PUBLIC SETTINGS ENDPOINT (no auth required)
// ═══════════════════════════════════════════════════════════════════════════════

/// GET /settings/public - Get public settings only (no auth)
#[tracing::instrument(skip(state))]
pub async fn public_settings(
    State(state): State<AppState>,
) -> Result<Json<serde_json::Value>, ApiError> {
    let settings: Vec<Setting> = sqlx::query_as(
        r#"
        SELECT id, key, value, "group" as setting_group, description,
               is_public, false as is_sensitive, value_type, created_at, updated_at
        FROM settings
        WHERE is_public = true AND (is_sensitive = false OR is_sensitive IS NULL)
        ORDER BY "group", key
        "#,
    )
    .fetch_all(state.db.pool())
    .await
    .map_err(|e| ApiError::database_error(&e.to_string()))?;

    // Convert to simple key-value map
    let settings_map: HashMap<String, String> = settings
        .into_iter()
        .map(|s| (s.key, s.value))
        .collect();

    Ok(Json(json!({
        "success": true,
        "data": settings_map
    })))
}

/// GET /settings/feature-flags/check/:key - Check if feature flag is enabled (public)
#[tracing::instrument(skip(state))]
pub async fn check_feature_flag(
    State(state): State<AppState>,
    Path(key): Path<String>,
) -> Result<Json<serde_json::Value>, ApiError> {
    let enabled: Option<bool> = sqlx::query_scalar(
        "SELECT enabled FROM feature_flags WHERE key = $1",
    )
    .bind(&key)
    .fetch_optional(state.db.pool())
    .await
    .map_err(|e| ApiError::database_error(&e.to_string()))?;

    Ok(Json(json!({
        "key": key,
        "enabled": enabled.unwrap_or(false)
    })))
}

// ═══════════════════════════════════════════════════════════════════════════════
// AUDIT LOG ENDPOINT (admin only)
// ═══════════════════════════════════════════════════════════════════════════════

/// GET /admin/settings/audit-logs - Get settings audit logs
#[tracing::instrument(skip(state, _admin))]
pub async fn get_audit_logs(
    State(state): State<AppState>,
    _admin: AdminUser,
    Query(params): Query<HashMap<String, String>>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let limit: i64 = params
        .get("limit")
        .and_then(|s| s.parse().ok())
        .unwrap_or(50)
        .min(500);
    let offset: i64 = params
        .get("offset")
        .and_then(|s| s.parse().ok())
        .unwrap_or(0);

    #[derive(Debug, Serialize, FromRow)]
    struct AuditLogEntry {
        id: i64,
        admin_id: i64,
        admin_email: Option<String>,
        action: String,
        entity_type: String,
        entity_id: Option<i64>,
        old_value: Option<String>,
        new_value: Option<String>,
        metadata: Option<serde_json::Value>,
        created_at: DateTime<Utc>,
    }

    let logs: Vec<AuditLogEntry> = sqlx::query_as(
        r#"
        SELECT id, admin_id, admin_email, action, entity_type, entity_id,
               old_value, new_value, metadata, created_at
        FROM admin_audit_logs
        WHERE entity_type IN ('setting', 'feature_flag')
        ORDER BY created_at DESC
        LIMIT $1 OFFSET $2
        "#,
    )
    .bind(limit)
    .bind(offset)
    .fetch_all(state.db.pool())
    .await
    .unwrap_or_default();

    let total: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM admin_audit_logs WHERE entity_type IN ('setting', 'feature_flag')",
    )
    .fetch_one(state.db.pool())
    .await
    .unwrap_or(0);

    Ok(Json(json!({
        "success": true,
        "data": logs,
        "pagination": {
            "total": total,
            "limit": limit,
            "offset": offset
        }
    })))
}

// ═══════════════════════════════════════════════════════════════════════════════
// ROUTERS
// ═══════════════════════════════════════════════════════════════════════════════

/// Build the admin settings router (requires AdminUser)
pub fn admin_router() -> Router<AppState> {
    Router::new()
        .route("/", get(index).post(create).put(update))
        .route("/:key", get(show).put(update_single).delete(delete_setting))
        .route("/feature-flags", get(list_feature_flags).post(create_feature_flag))
        .route(
            "/feature-flags/:key",
            get(get_feature_flag)
                .put(update_feature_flag)
                .delete(delete_feature_flag),
        )
        .route("/audit-logs", get(get_audit_logs))
}

/// Build the public settings router (no auth required)
pub fn public_router() -> Router<AppState> {
    Router::new()
        .route("/public", get(public_settings))
        .route("/feature-flags/check/:key", get(check_feature_flag))
}

/// Legacy router for backward compatibility
pub fn router() -> Router<AppState> {
    Router::new()
        .route("/admin/settings", get(index).put(update))
        .route("/admin/settings/:key", get(show).put(update_single))
}
