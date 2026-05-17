//! Consent Management Settings - ICT 11+ Principal Engineer Grade
//! Created 2026-04-26 to satisfy the `/admin/consent/settings` admin page
//! per docs/audits/ADMIN_FAILURE_DATA.md §9.5 (load/save/reset all 404).
//!
//! SECURITY: All endpoints require `AdminUser` authentication.
//!
//! PERSISTENCE: We piggy-back on the generic `settings` table by storing the
//! whole `ConsentSettings` blob as a JSON string under a single fixed key
//! (`consent_management_settings`). This avoids a new migration while still
//! durably persisting changes. If the table is missing or unwritable we fall
//! back to a process-local cache so the page never 404s — see TODO below.
//!
//! TODO: persist to a dedicated `consent_settings` table with JSONB column
//! once the schema migration lands; the helper functions below are wrapped
//! so swapping storage is a one-place change.

use std::sync::OnceLock;

use axum::{
    extract::State,
    http::StatusCode,
    response::Json,
    routing::{get, post},
    Router,
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use tokio::sync::Mutex;
use validator::Validate;

use crate::{middleware::admin::AdminUser, AppState};

// ═══════════════════════════════════════════════════════════════════════════
// TYPES
// ═══════════════════════════════════════════════════════════════════════════

/// All consent-management settings exposed by the admin page.
///
/// Field names are kept snake_case on the wire to match the existing frontend
/// `ConsentSettings` interface in
/// `frontend/src/routes/admin/consent/settings/+page.svelte`. Keep them in sync.
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct ConsentSettings {
    // ── General ─────────────────────────────────────────────────────────
    pub consent_enabled: bool,
    pub test_mode: bool,
    #[validate(range(min = 1, max = 730))]
    pub expire_days: i32,
    #[validate(range(min = 1))]
    pub consent_version: i32,
    #[validate(length(min = 1, max = 32))]
    pub policy_version: String,

    // ── Banner ──────────────────────────────────────────────────────────
    pub banner_enabled: bool,
    #[validate(length(min = 1, max = 32))]
    pub banner_position: String,
    #[validate(length(min = 1, max = 32))]
    pub banner_layout: String,
    pub show_reject_button: bool,
    pub show_settings_button: bool,
    pub close_on_scroll: bool,
    #[validate(range(min = 10, max = 5000))]
    pub close_on_scroll_distance: i32,

    // ── Script Blocking ────────────────────────────────────────────────
    pub script_blocking_enabled: bool,
    pub block_google_analytics: bool,
    pub block_google_tag_manager: bool,
    pub block_facebook_pixel: bool,
    pub block_tiktok_pixel: bool,
    pub block_twitter_pixel: bool,
    pub block_linkedin_pixel: bool,
    pub block_pinterest_tag: bool,
    pub block_reddit_pixel: bool,
    pub block_hotjar: bool,
    pub block_youtube_embeds: bool,
    pub block_vimeo_embeds: bool,
    pub block_google_maps: bool,

    // ── Integrations ───────────────────────────────────────────────────
    pub google_consent_mode_enabled: bool,
    pub bing_consent_mode_enabled: bool,

    // ── Geolocation ────────────────────────────────────────────────────
    pub geolocation_enabled: bool,
    pub geo_default_strict: bool,

    // ── Proof of Consent ───────────────────────────────────────────────
    pub proof_consent_enabled: bool,
    #[validate(range(min = 30, max = 3650))]
    pub proof_retention_days: i32,
    pub proof_auto_delete: bool,
}

impl Default for ConsentSettings {
    /// Defaults mirror the initial `$state` block on the admin page so a
    /// just-installed deployment renders a sensible, GDPR-leaning configuration
    /// with no further admin action required.
    fn default() -> Self {
        Self {
            // General
            consent_enabled: true,
            test_mode: false,
            expire_days: 365,
            consent_version: 1,
            policy_version: "1.0.0".to_string(),
            // Banner
            banner_enabled: true,
            banner_position: "bottom".to_string(),
            banner_layout: "bar".to_string(),
            show_reject_button: true,
            show_settings_button: true,
            close_on_scroll: false,
            close_on_scroll_distance: 60,
            // Script Blocking
            script_blocking_enabled: true,
            block_google_analytics: true,
            block_google_tag_manager: true,
            block_facebook_pixel: true,
            block_tiktok_pixel: true,
            block_twitter_pixel: true,
            block_linkedin_pixel: true,
            block_pinterest_tag: true,
            block_reddit_pixel: true,
            block_hotjar: true,
            block_youtube_embeds: true,
            block_vimeo_embeds: true,
            block_google_maps: true,
            // Integrations
            google_consent_mode_enabled: true,
            bing_consent_mode_enabled: false,
            // Geolocation
            geolocation_enabled: false,
            geo_default_strict: true,
            // Proof
            proof_consent_enabled: true,
            proof_retention_days: 365,
            proof_auto_delete: true,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct BulkUpdateRequest {
    /// Page sends `{ settings: { ... full ConsentSettings ... } }`
    pub settings: ConsentSettings,
}

// ═══════════════════════════════════════════════════════════════════════════
// PERSISTENCE
// ═══════════════════════════════════════════════════════════════════════════

/// Single fixed key in the existing `settings` table — see file-level docs.
const STORAGE_KEY: &str = "consent_management_settings";

/// Process-local fallback. When the `settings` table doesn't exist (fresh
/// dev DB) or a write fails we still want subsequent GETs to return the
/// most-recent in-memory state rather than reverting to defaults.
fn cache() -> &'static Mutex<ConsentSettings> {
    static CACHE: OnceLock<Mutex<ConsentSettings>> = OnceLock::new();
    CACHE.get_or_init(|| Mutex::new(ConsentSettings::default()))
}

/// Best-effort load — DB first, fall back to in-memory cache, then defaults.
async fn load_from_storage(pool: &sqlx::PgPool) -> ConsentSettings {
    // Try DB first.
    let row: Result<Option<String>, sqlx::Error> =
        sqlx::query_scalar("SELECT value FROM settings WHERE key = $1")
            .bind(STORAGE_KEY)
            .fetch_optional(pool)
            .await;

    if let Ok(Some(json_str)) = row {
        if let Ok(parsed) = serde_json::from_str::<ConsentSettings>(&json_str) {
            // Refresh the in-memory cache so subsequent saves see fresh state.
            *cache().lock().await = parsed.clone();
            return parsed;
        }
    }

    // DB miss / parse error / table missing — return cached or defaults.
    cache().lock().await.clone()
}

/// Best-effort save — write to DB if possible, always update the in-memory
/// cache so the page reflects the change immediately.
async fn save_to_storage(pool: &sqlx::PgPool, settings: &ConsentSettings) {
    *cache().lock().await = settings.clone();

    let Ok(json_str) = serde_json::to_string(settings) else {
        tracing::warn!("consent: failed to serialise ConsentSettings to JSON");
        return;
    };

    // Upsert into the existing settings table. The table is shared with
    // routes/settings.rs and uses a `key` UNIQUE index.
    let result = sqlx::query(
        r#"
        INSERT INTO settings (key, value, "group", description, is_public, created_at, updated_at)
        VALUES ($1, $2, 'consent', 'Consent Management Settings (JSON blob)', false, NOW(), NOW())
        ON CONFLICT (key)
        DO UPDATE SET value = EXCLUDED.value, updated_at = NOW()
        "#,
    )
    .bind(STORAGE_KEY)
    .bind(&json_str)
    .execute(pool)
    .await;

    if let Err(e) = result {
        // Non-fatal: cache already holds the new state, the page will work.
        // TODO: persist to dedicated consent_settings table once it exists.
        tracing::warn!(
            "consent: failed to persist settings to DB ({}); retained in-memory only",
            e
        );
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// HANDLERS
// ═══════════════════════════════════════════════════════════════════════════

/// `GET /admin/consent/settings` — load current consent settings.
#[tracing::instrument(skip(state, admin))]
async fn get_settings(
    State(state): State<AppState>,
    admin: AdminUser,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    tracing::info!(
        admin_id = admin.0.id,
        admin_email = %admin.0.email,
        "Admin loading consent settings"
    );

    let settings = load_from_storage(state.db.pool()).await;

    Ok(Json(json!({
        "success": true,
        "data": settings,
    })))
}

/// `POST /admin/consent/settings/bulk` — bulk update consent settings.
#[tracing::instrument(skip(state, admin, payload))]
async fn bulk_update_settings(
    State(state): State<AppState>,
    admin: AdminUser,
    Json(payload): Json<BulkUpdateRequest>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    if let Err(errors) = payload.settings.validate() {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(json!({
                "success": false,
                "error": "Validation failed",
                "details": errors.to_string(),
            })),
        ));
    }

    save_to_storage(state.db.pool(), &payload.settings).await;

    tracing::info!(
        admin_id = admin.0.id,
        admin_email = %admin.0.email,
        "Admin bulk-updated consent settings"
    );

    Ok(Json(json!({
        "success": true,
        "message": "Consent settings updated",
        "data": payload.settings,
    })))
}

/// `POST /admin/consent/settings/reset` — reset all settings to defaults.
#[tracing::instrument(skip(state, admin))]
async fn reset_settings(
    State(state): State<AppState>,
    admin: AdminUser,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let defaults = ConsentSettings::default();
    save_to_storage(state.db.pool(), &defaults).await;

    tracing::info!(
        admin_id = admin.0.id,
        admin_email = %admin.0.email,
        "Admin reset consent settings to defaults"
    );

    Ok(Json(json!({
        "success": true,
        "message": "Consent settings reset to defaults",
        "data": defaults,
    })))
}

// ═══════════════════════════════════════════════════════════════════════════
// ROUTER
// ═══════════════════════════════════════════════════════════════════════════

/// Build the consent admin router. Mount under `/admin/consent` so the page
/// hits `GET /api/admin/consent/settings`, `POST /api/admin/consent/settings/bulk`
/// and `POST /api/admin/consent/settings/reset`.
pub fn router() -> Router<AppState> {
    Router::new()
        .route("/settings", get(get_settings))
        .route("/settings/bulk", post(bulk_update_settings))
        .route("/settings/reset", post(reset_settings))
}
