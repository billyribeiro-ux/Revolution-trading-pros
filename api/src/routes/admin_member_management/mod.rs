//! Admin Member Management Controller - ICT 11+ Principal Engineer
//! Apple ICT 7 Grade - January 2026
//!
//! Enterprise-grade member management: CRUD, ban/suspend, activity log,
//! notes, Excel/PDF export. All routes require admin privileges.
//!
//! SECURITY: Uses AdminUser extractor for consistent authorization across all routes.
//!
//! Module split (R21-B, 2026-05-20):
//! - `crud`       — get_member_full / create / update / delete + engagement & timeline helpers
//! - `moderation` — ban / suspend / unban
//! - `notes`      — get / create / delete member notes
//! - `activity`   — get member activity log (paginated)
//! - `export`     — CSV / XLSX / PDF export + format helpers

use axum::{
    routing::{delete, get, post},
    Router,
};
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

use crate::AppState;

mod activity;
mod crud;
mod export;
mod moderation;
mod notes;

// ===============================================================================
// SHARED TYPES
// ===============================================================================

#[derive(Debug, Serialize, FromRow)]
pub struct MemberDetail {
    pub id: i64,
    pub name: Option<String>,
    pub email: String,
    pub role: Option<String>,
    pub avatar_url: Option<String>,
    pub email_verified_at: Option<NaiveDateTime>,
    pub mfa_enabled: Option<bool>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Serialize, FromRow)]
pub struct MemberSubscription {
    pub id: i64,
    pub product_name: Option<String>,
    pub status: String,
    pub price_cents: Option<i64>,
    pub billing_period: Option<String>,
    pub started_at: Option<NaiveDateTime>,
    pub expires_at: Option<NaiveDateTime>,
    pub cancelled_at: Option<NaiveDateTime>,
    pub created_at: NaiveDateTime,
}

#[derive(Debug, Serialize, FromRow)]
pub struct MemberOrder {
    pub id: i64,
    pub order_number: Option<String>,
    pub total_cents: Option<i64>,
    pub status: Option<String>,
    pub created_at: NaiveDateTime,
}

#[derive(Debug, Serialize, FromRow)]
pub struct MemberActivity {
    pub id: i64,
    pub user_id: i64,
    pub action: String,
    pub description: Option<String>,
    pub metadata: Option<serde_json::Value>,
    pub ip_address: Option<String>,
    pub user_agent: Option<String>,
    pub created_at: NaiveDateTime,
}

#[derive(Debug, Serialize, FromRow)]
pub struct MemberNote {
    pub id: i64,
    pub user_id: i64,
    pub content: String,
    pub created_by: Option<i64>,
    pub created_by_name: Option<String>,
    pub created_at: NaiveDateTime,
}

// ===============================================================================
// REQUEST TYPES
// ===============================================================================

#[derive(Debug, Deserialize)]
pub struct CreateMemberRequest {
    pub name: String,
    pub email: String,
    pub password: Option<String>,
    pub role: Option<String>,
    pub send_welcome_email: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateMemberRequest {
    pub name: Option<String>,
    pub email: Option<String>,
    pub role: Option<String>,
    pub avatar_url: Option<String>,
    pub password: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct BanMemberRequest {
    pub reason: Option<String>,
    pub duration_days: Option<i32>,
}

#[derive(Debug, Deserialize)]
pub struct CreateNoteRequest {
    pub content: String,
}

#[derive(Debug, Deserialize)]
pub struct ExportQuery {
    pub format: Option<String>, // csv, xlsx, pdf
    pub status: Option<String>,
    pub date_from: Option<String>,
    pub date_to: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct ActivityQuery {
    pub page: Option<i64>,
    pub per_page: Option<i64>,
    pub action: Option<String>,
}

// ===============================================================================
// ROUTER
// ===============================================================================

pub fn router() -> Router<AppState> {
    Router::new()
        // CRUD
        .route("/", post(crud::create_member))
        .route("/export", get(export::export_members))
        .route(
            "/{id}",
            get(crud::get_member_full)
                .put(crud::update_member)
                .delete(crud::delete_member),
        )
        // Ban/Suspend/Unban
        .route("/{id}/ban", post(moderation::ban_member))
        .route("/{id}/suspend", post(moderation::suspend_member))
        .route("/{id}/unban", post(moderation::unban_member))
        // Notes
        .route("/{id}/notes", get(notes::get_notes).post(notes::create_note))
        .route("/{id}/notes/{note_id}", delete(notes::delete_note))
        // Activity
        .route("/{id}/activity", get(activity::get_activity))
}
