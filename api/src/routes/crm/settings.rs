//! CRM settings, managers, and system-log handlers
//! (split from `crm.rs` lines 1646-1709).

use axum::{
    Json, Router,
    extract::{Query, State},
    http::StatusCode,
    routing::get,
};
use serde::Deserialize;
use serde_json::json;

use super::ListFilters;
use crate::{AppState, middleware::admin::AdminUser, utils::errors::ApiError};

// ═══════════════════════════════════════════════════════════════════════════
// TYPES
// ═══════════════════════════════════════════════════════════════════════════

#[derive(Debug, Deserialize)]
pub struct UpdateSettingsInput {
    pub settings: serde_json::Value,
}

// ═══════════════════════════════════════════════════════════════════════════
// HANDLERS
// ═══════════════════════════════════════════════════════════════════════════

async fn get_crm_settings(
    State(state): State<AppState>,
    _admin: AdminUser,
) -> Result<Json<serde_json::Value>, ApiError> {
    let defaults = json!({
        "email_settings": {
            "from_name": "Revolution Trading Pros",
            "from_email": "noreply@revolutiontradingpros.com",
            "reply_to": "support@revolutiontradingpros.com"
        },
        "scoring_settings": {
            "enabled": true,
            "max_score": 100
        },
        "general_settings": {
            "double_opt_in": false,
            "unsubscribe_footer": true
        }
    });
    Ok(Json(read_settings(&state, "crm.settings", defaults).await?))
}

async fn update_crm_settings(
    State(state): State<AppState>,
    _admin: AdminUser,
    Json(input): Json<UpdateSettingsInput>,
) -> Result<Json<serde_json::Value>, ApiError> {
    validate_object(&input.settings)?;
    write_settings(&state, "crm.settings", &input.settings).await?;
    Ok(Json(json!({
        "success": true,
        "message": "Settings updated",
        "settings": input.settings
    })))
}

async fn read_settings(
    state: &AppState,
    key: &str,
    defaults: serde_json::Value,
) -> Result<serde_json::Value, ApiError> {
    let stored: Option<serde_json::Value> =
        sqlx::query_scalar("SELECT value FROM settings WHERE key = $1")
            .bind(key)
            .fetch_optional(&state.db.pool)
            .await
            .map_err(|e| ApiError::database_error(&e.to_string()))?;
    Ok(stored.unwrap_or(defaults))
}

async fn write_settings(
    state: &AppState,
    key: &str,
    value: &serde_json::Value,
) -> Result<(), ApiError> {
    sqlx::query(
        "INSERT INTO settings (key, value, created_at, updated_at) VALUES ($1, $2, NOW(), NOW())
         ON CONFLICT (key) DO UPDATE SET value = EXCLUDED.value, updated_at = NOW()",
    )
    .bind(key)
    .bind(value)
    .execute(&state.db.pool)
    .await
    .map_err(|e| ApiError::database_error(&e.to_string()))?;
    Ok(())
}

fn validate_object(value: &serde_json::Value) -> Result<(), ApiError> {
    if !value.is_object() || value.to_string().len() > 100_000 {
        return Err(ApiError::validation_error(
            "Settings must be an object under 100 KB",
        ));
    }
    Ok(())
}

fn double_optin_defaults() -> serde_json::Value {
    json!({
        "enabled": false,
        "email_subject": "Please confirm your subscription",
        "email_body": "<p>Please confirm your email: <a href=\"{{confirmation_link}}\">Confirm Email</a></p>",
        "confirmation_page_url": "", "redirect_url": "",
        "after_confirmation_status": "subscribed",
        "apply_tags_on_confirm": [], "add_to_lists_on_confirm": []
    })
}

fn email_preferences_defaults() -> serde_json::Value {
    json!({
        "enabled": false, "title": "Email Preferences",
        "intro_text": "Manage your email subscription preferences below.",
        "show_lists": true, "show_tags": false,
        "show_communication_types": true, "show_unsubscribe_all": true,
        "custom_css": "", "redirect_after_update": ""
    })
}

fn validate_section(
    input: &serde_json::Value,
    defaults: &serde_json::Value,
) -> Result<(), ApiError> {
    validate_object(input)?;
    for (key, value) in input.as_object().expect("validated object") {
        let expected = defaults
            .get(key)
            .ok_or_else(|| ApiError::validation_error(&format!("Unknown setting: {key}")))?;
        let valid = match expected {
            serde_json::Value::Bool(_) => value.is_boolean(),
            serde_json::Value::String(_) => value.is_string(),
            serde_json::Value::Array(_) => value.as_array().is_some_and(|items| {
                items.len() <= 1000 && items.iter().all(serde_json::Value::is_string)
            }),
            _ => false,
        };
        if !valid {
            return Err(ApiError::validation_error(&format!(
                "Invalid value for {key}"
            )));
        }
        if matches!(
            key.as_str(),
            "confirmation_page_url" | "redirect_url" | "redirect_after_update"
        ) {
            let url = value.as_str().unwrap_or_default();
            if !url.is_empty()
                && !reqwest::Url::parse(url).is_ok_and(|u| {
                    matches!(u.scheme(), "http" | "https")
                        && u.host_str().is_some()
                        && u.username().is_empty()
                        && u.password().is_none()
                })
            {
                return Err(ApiError::validation_error("Redirects must be HTTP(S) URLs"));
            }
        }
        if key == "after_confirmation_status" && value != "subscribed" && value != "pending" {
            return Err(ApiError::validation_error("Invalid confirmation status"));
        }
    }
    Ok(())
}

async fn get_double_optin(
    State(state): State<AppState>,
    _admin: AdminUser,
) -> Result<Json<serde_json::Value>, ApiError> {
    Ok(Json(
        read_settings(&state, "crm.double_optin", double_optin_defaults()).await?,
    ))
}

async fn get_email_preferences(
    State(state): State<AppState>,
    _admin: AdminUser,
) -> Result<Json<serde_json::Value>, ApiError> {
    Ok(Json(
        read_settings(
            &state,
            "crm.email_preferences",
            email_preferences_defaults(),
        )
        .await?,
    ))
}

// Lock the setting's row while merging a partial update so concurrent saves
// cannot silently discard independent fields. The transaction also makes the
// first save atomic when two administrators create the settings together.
async fn save_section(
    state: &AppState,
    key: &str,
    defaults: serde_json::Value,
    input: serde_json::Value,
) -> Result<Json<serde_json::Value>, ApiError> {
    validate_section(&input, &defaults)?;
    let mut tx = state
        .db
        .pool
        .begin()
        .await
        .map_err(|e| ApiError::database_error(&e.to_string()))?;
    sqlx::query("INSERT INTO settings (key, value) VALUES ($1, $2) ON CONFLICT (key) DO NOTHING")
        .bind(key)
        .bind(&defaults)
        .execute(&mut *tx)
        .await
        .map_err(|e| ApiError::database_error(&e.to_string()))?;
    let mut value: serde_json::Value =
        sqlx::query_scalar("SELECT value FROM settings WHERE key = $1 FOR UPDATE")
            .bind(key)
            .fetch_one(&mut *tx)
            .await
            .map_err(|e| ApiError::database_error(&e.to_string()))?;
    let object = value
        .as_object_mut()
        .ok_or_else(|| ApiError::internal_error("Invalid stored settings"))?;
    for (field, setting) in input.as_object().expect("validated object") {
        object.insert(field.clone(), setting.clone());
    }
    sqlx::query("UPDATE settings SET value = $2, updated_at = NOW() WHERE key = $1")
        .bind(key)
        .bind(&value)
        .execute(&mut *tx)
        .await
        .map_err(|e| ApiError::database_error(&e.to_string()))?;
    tx.commit()
        .await
        .map_err(|e| ApiError::database_error(&e.to_string()))?;
    Ok(Json(
        json!({"success": true, "message": "Settings saved", "settings": value}),
    ))
}

async fn save_double_optin(
    State(state): State<AppState>,
    _admin: AdminUser,
    Json(input): Json<serde_json::Value>,
) -> Result<Json<serde_json::Value>, ApiError> {
    save_section(&state, "crm.double_optin", double_optin_defaults(), input).await
}

async fn save_email_preferences(
    State(state): State<AppState>,
    _admin: AdminUser,
    Json(input): Json<serde_json::Value>,
) -> Result<Json<serde_json::Value>, ApiError> {
    save_section(
        &state,
        "crm.email_preferences",
        email_preferences_defaults(),
        input,
    )
    .await
}

async fn list_managers(
    State(_state): State<AppState>,
    _admin: AdminUser,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    Ok(Json(json!({
        "data": [],
        "meta": { "total": 0 }
    })))
}

async fn get_system_logs(
    State(_state): State<AppState>,
    _admin: AdminUser,
    Query(filters): Query<ListFilters>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let per_page = filters.per_page.unwrap_or(50);
    Ok(Json(json!({
        "data": [],
        "meta": { "total": 0, "per_page": per_page }
    })))
}

// ═══════════════════════════════════════════════════════════════════════════
// ROUTER
// ═══════════════════════════════════════════════════════════════════════════

pub(super) fn router() -> Router<AppState> {
    Router::new()
        .route("/settings", get(get_crm_settings).put(update_crm_settings))
        .route(
            "/settings/double-optin",
            get(get_double_optin).post(save_double_optin),
        )
        .route(
            "/settings/email-preferences",
            get(get_email_preferences).post(save_email_preferences),
        )
        .route("/managers", get(list_managers))
        .route("/system-logs", get(get_system_logs))
}
