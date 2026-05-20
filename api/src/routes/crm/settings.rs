//! CRM settings, managers, and system-log handlers
//! (split from `crm.rs` lines 1646-1709).

use axum::{
    extract::{Query, State},
    http::StatusCode,
    routing::get,
    Json, Router,
};
use serde::Deserialize;
use serde_json::json;

use super::ListFilters;
use crate::{middleware::admin::AdminUser, AppState};

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
    State(_state): State<AppState>,
    _admin: AdminUser,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    Ok(Json(json!({
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
    })))
}

async fn update_crm_settings(
    State(_state): State<AppState>,
    _admin: AdminUser,
    Json(input): Json<UpdateSettingsInput>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    Ok(Json(json!({
        "success": true,
        "message": "Settings updated",
        "settings": input.settings
    })))
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
        .route("/managers", get(list_managers))
        .route("/system-logs", get(get_system_logs))
}
