//! Trading Rooms routes - Revolution Trading Pros
//! Apple Principal Engineer ICT 7 Grade - January 2026
//!
//! Provides API endpoints for trading rooms and sections management.
//! ICT 7 SECURITY: Admin endpoints require AdminUser authentication
//! All 6 rooms in correct order:
//! 1. Day Trading Room
//! 2. Swing Trading Room
//! 3. Small Account Mentorship
//! 4. Explosive Swings
//! 5. SPX Profit Pulse
//! 6. High Octane Scanner
//!
//! ## R27-B5 maintainability split (2026-05-20)
//!
//! Split from a single 1,201-LOC `trading_rooms.rs` file. Public API
//! unchanged: external callers still see
//! `routes::trading_rooms::router()`,
//! `routes::trading_rooms::admin_router()`,
//! `routes::trading_rooms::admin_rooms_router()`, and every previously-
//! public DTO (`TradingRoom`, `RoomSection`, `TradingRoomsQuery`,
//! `TradersQuery`, `VideosQuery`). Sub-modules group handlers by
//! sub-domain: `public` (anonymous read), `admin_rooms` (admin CRUD-
//! style reads on rooms/sections/traders), `admin_videos` (admin video
//! list per-room), `admin_stats` (admin dashboard counts).

use axum::{routing::get, Router};

use crate::AppState;

mod admin_rooms;
mod admin_stats;
mod admin_videos;
mod fallback;
mod public;
mod types;

// Re-export DTOs so the pre-split public API
// (`trading_rooms::TradingRoom`, `trading_rooms::VideosQuery`, …)
// keeps resolving for `tests/trading_rooms_test.rs` and any other
// downstream consumer.
pub use types::{RoomSection, TradersQuery, TradingRoom, TradingRoomsQuery, VideosQuery};

// ═══════════════════════════════════════════════════════════════════════════
// ROUTERS
// ═══════════════════════════════════════════════════════════════════════════

/// Public router for trading rooms
pub fn router() -> Router<AppState> {
    Router::new()
        .route("/", get(public::list_trading_rooms))
        .route("/sections", get(public::list_sections))
        .route("/{slug}", get(public::get_trading_room))
        .route("/traders", get(public::list_traders))
}

/// Admin router for trading rooms - ICT 7: All endpoints require AdminUser authentication
pub fn admin_router() -> Router<AppState> {
    Router::new()
        .route("/", get(admin_rooms::admin_list_trading_rooms))
        .route("/sections", get(admin_rooms::admin_list_sections))
        .route("/{slug}", get(admin_rooms::admin_get_trading_room))
        .route("/traders", get(admin_rooms::admin_list_traders))
        // ICT 7 FIX: Add videos routes for frontend compatibility
        .route("/videos", get(admin_videos::admin_list_videos))
        .route(
            "/videos/{slug}",
            get(admin_videos::admin_list_videos_by_room),
        )
}

/// Admin "rooms" router — short-prefix companion to `admin_router()` so
/// `/api/admin/rooms/stats` resolves without colliding with
/// `/api/admin/trading-rooms/...`.
///
/// FIX-2026-04-26: added to back the dashboard's
/// `fetch('/api/admin/rooms/stats')` call.
pub fn admin_rooms_router() -> Router<AppState> {
    Router::new().route("/stats", get(admin_stats::admin_get_rooms_stats))
}
