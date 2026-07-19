//! Member Indicator API Routes - Secure Downloads
//! Apple Principal Engineer ICT 7 Grade - January 2026
//!
//! - Public indicator listing
//! - Ownership verification
//! - Secure hash-based download URLs (WordPress-compatible)
//! - Download tracking
//!
//! ## Module layout (R28-B3 structural split)
//!
//! This module was split out of a single 1,142-LOC `member_indicators.rs`
//! into a directory module. Public API is unchanged — `public_router()`,
//! `member_router()`, and `download_router()` are the only exported items
//! that the rest of the crate / tests consume.
//!
//! - `public`     — unauth catalog (`list_public_indicators`, `get_public_indicator`)
//! - `ownership`  — paywall-gated ownership listing
//!   (`get_my_indicators`, `get_indicator_downloads`)
//! - `downloads`  — secure download URL generation + token-gated streaming
//!   + per-user download history
//! - `license`    — license-key generation + validation
//! - `guide`      — update notifications + installation guides

use axum::{routing::get, Router};

use crate::AppState;

mod downloads;
mod guide;
mod license;
mod ownership;
mod public;

// ═══════════════════════════════════════════════════════════════════════════════════
// ROUTERS
// ═══════════════════════════════════════════════════════════════════════════════════

pub fn public_router() -> Router<AppState> {
    Router::new()
        .route("/", get(public::list_public_indicators))
        .route("/{slug}", get(public::get_public_indicator))
}

pub fn member_router() -> Router<AppState> {
    Router::new()
        .route("/", get(ownership::get_my_indicators))
        .route("/history", get(downloads::get_download_history))
        .route("/updates", get(guide::check_updates))
        .route("/{slug}", get(ownership::get_indicator_downloads))
        .route(
            "/{slug}/download/{file_id}",
            get(downloads::generate_download_url),
        )
        .route("/{slug}/license", get(license::get_license_key))
        .route("/{slug}/validate", get(license::validate_license_key))
        .route("/{slug}/guide/{platform}", get(guide::get_installation_guide))
}

pub fn download_router() -> Router<AppState> {
    Router::new().route("/indicator/{slug}/{file_id}", get(downloads::download_file))
}
