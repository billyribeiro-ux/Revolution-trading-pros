//! Shared error helpers and auth gates for the CMS revisions sub-module.
//!
//! R27-B4 split (2026-05-20) — extracted verbatim from the original
//! `routes/cms_revisions.rs` (lines 32-69). No behavioural change.

use axum::{http::StatusCode, Json};
use chrono::Utc;
use serde_json::{json, Value as JsonValue};

use crate::models::User;

// ═══════════════════════════════════════════════════════════════════════════════════════
// TYPE ALIASES AND ERROR HANDLING
// ═══════════════════════════════════════════════════════════════════════════════════════

pub(super) type ApiResult<T> = Result<Json<T>, (StatusCode, Json<JsonValue>)>;
#[allow(dead_code)]
pub(super) type ApiResultEmpty = Result<Json<JsonValue>, (StatusCode, Json<JsonValue>)>;

pub(super) fn api_error(status: StatusCode, message: &str) -> (StatusCode, Json<JsonValue>) {
    (
        status,
        Json(json!({
            "error": message,
            "status": status.as_u16(),
            "timestamp": Utc::now().to_rfc3339()
        })),
    )
}

pub(super) fn require_cms_editor(user: &User) -> Result<(), (StatusCode, Json<JsonValue>)> {
    let role = user.role.as_deref().unwrap_or("user");
    if matches!(
        role,
        "admin" | "super-admin" | "super_admin" | "editor" | "marketing"
    ) {
        Ok(())
    } else {
        Err(api_error(StatusCode::FORBIDDEN, "Editor access required"))
    }
}

pub(super) fn require_cms_admin(user: &User) -> Result<(), (StatusCode, Json<JsonValue>)> {
    let role = user.role.as_deref().unwrap_or("user");
    if matches!(role, "admin" | "super-admin" | "super_admin") {
        Ok(())
    } else {
        Err(api_error(StatusCode::FORBIDDEN, "Admin access required"))
    }
}
