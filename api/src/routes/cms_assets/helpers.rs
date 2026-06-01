//! CMS Assets — shared helpers
//!
//! Free functions used across handler files. Visibility is
//! `pub(super)` so the sibling submodules (`folders`, `crud`, …)
//! can reach them without re-exporting them through the public
//! API.

use crate::middleware::admin::AdminUser;
use crate::services::cms_content;
use uuid::Uuid;

pub(super) fn slugify(text: &str) -> String {
    text.to_lowercase()
        .chars()
        .map(|c| if c.is_alphanumeric() { c } else { '-' })
        .collect::<String>()
        .split('-')
        .filter(|s| !s.is_empty())
        .collect::<Vec<_>>()
        .join("-")
}

pub(super) fn format_bytes(bytes: i64) -> String {
    const KB: i64 = 1024;
    const MB: i64 = KB * 1024;
    const GB: i64 = MB * 1024;

    if bytes >= GB {
        format!("{:.2} GB", bytes as f64 / GB as f64)
    } else if bytes >= MB {
        format!("{:.2} MB", bytes as f64 / MB as f64)
    } else if bytes >= KB {
        format!("{:.2} KB", bytes as f64 / KB as f64)
    } else {
        format!("{bytes} B")
    }
}

/// Get CMS user UUID from admin user
pub(super) async fn get_cms_user_id(pool: &sqlx::PgPool, admin: &AdminUser) -> Option<Uuid> {
    let user_id = admin.0.id;
    cms_content::get_cms_user_by_user_id(pool, user_id)
        .await
        .ok()
        .flatten()
        .map(|u| u.id)
}
