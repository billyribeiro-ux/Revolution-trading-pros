//! Admin Members - Notes & Email History
//!
//! Member notes (CRUD) and email history (read-only) per-member endpoints.

use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, FromRow)]
pub struct MemberNote {
    pub id: i64,
    pub user_id: i64,
    pub content: String,
    pub created_by: Option<i64>,
    pub created_at: chrono::NaiveDateTime,
}

#[derive(Debug, Deserialize)]
pub struct CreateNoteRequest {
    pub content: String,
}

#[derive(Debug, Serialize, FromRow)]
pub struct MemberEmail {
    pub id: i64,
    pub user_id: i64,
    pub subject: String,
    pub status: String,
    pub sent_at: Option<chrono::NaiveDateTime>,
    pub created_at: chrono::NaiveDateTime,
}

pub(super) use crate::routes::admin_member_management::read_handlers::{
    add_note as create_member_note, emails as get_member_emails, notes as get_member_notes,
};
