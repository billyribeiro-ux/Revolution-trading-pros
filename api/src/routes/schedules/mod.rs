// ═══════════════════════════════════════════════════════════════════════════
// TRADING ROOM SCHEDULES API
// ICT 11+ Apple Principal Engineer Grade
// January 2026
//
// Per-room schedule management system with:
// - Public endpoints for fetching schedules
// - Admin endpoints for CRUD operations
// - Comprehensive audit logging
// - Exception handling for holidays/cancellations
//
// ───────────────────────────────────────────────────────────────────────────
// R28-B4 split (2026-05-20): the original 1,134 LOC `schedules.rs` was
// cleaved into this directory module. Pure structural move — every
// handler, every SQL string, every audit-log line, every `AdminUser` gate
// preserved verbatim. The public API surface (this `mod.rs`) is
// unchanged: `router()`, `admin_router()`, and the eight DTOs pinned by
// `tests/schedules_test.rs` (R12-D) continue to bind from
// `revolution_api::routes::schedules::*`.
//
// Sub-files:
//   - `types.rs`                   — DTOs (ScheduleRow, ScheduleExceptionRow,
//                                    TradingRoomPlan, ScheduleEventResponse,
//                                    UpcomingEventResponse, ScheduleQuery,
//                                    CreateScheduleRequest,
//                                    UpdateScheduleRequest,
//                                    CreateExceptionRequest,
//                                    BulkScheduleRequest)
//   - `helpers.rs`                 — day_name, parse_time, parse_date
//   - `public.rs`                  — public handlers (3)
//   - `admin.rs`                   — admin CRUD handlers (5)
//   - `admin_bulk_exceptions.rs`   — admin bulk + exception handlers (3)
// ═══════════════════════════════════════════════════════════════════════════

use axum::{
    routing::{delete, get, post, put},
    Router,
};

use crate::AppState;

mod admin;
mod admin_bulk_exceptions;
mod helpers;
mod public;
mod types;

// Re-export DTOs as the public API surface (pinned by
// `tests/schedules_test.rs` — see R12-D).
pub use types::{
    BulkScheduleRequest, CreateExceptionRequest, CreateScheduleRequest, ScheduleEventResponse,
    ScheduleExceptionRow, ScheduleQuery, ScheduleRow, TradingRoomPlan, UpcomingEventResponse,
    UpdateScheduleRequest,
};

/// Public schedule routes
pub fn router() -> Router<AppState> {
    Router::new()
        .route("/rooms", get(public::get_trading_rooms))
        .route("/{plan_slug}", get(public::get_schedule_by_plan))
        .route("/{plan_slug}/upcoming", get(public::get_upcoming_events))
}

/// Admin schedule routes
pub fn admin_router() -> Router<AppState> {
    Router::new()
        .route(
            "/",
            get(admin::admin_list_schedules).post(admin::admin_create_schedule),
        )
        .route("/plan/{plan_id}", get(admin::admin_get_plan_schedules))
        .route(
            "/{id}",
            put(admin::admin_update_schedule).delete(admin::admin_delete_schedule),
        )
        .route("/bulk", post(admin_bulk_exceptions::admin_bulk_schedules))
        .route(
            "/exceptions",
            post(admin_bulk_exceptions::admin_create_exception),
        )
        .route(
            "/exceptions/{id}",
            delete(admin_bulk_exceptions::admin_delete_exception),
        )
}
