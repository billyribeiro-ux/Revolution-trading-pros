//! Shared error helpers and auth gates for the CMS revisions sub-module.
//!
//! R27-B4 split (2026-05-20) — extracted verbatim from the original
//! `routes/cms_revisions.rs` (lines 32-69). No behavioural change.

use axum::{Json, http::StatusCode};
use chrono::Utc;
use serde_json::{Value as JsonValue, json};

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
