//! Organization Routes - ICT Level 7 Principal Engineer Grade
//! Apple ICT 7 Principal Engineer Grade - January 2026
//!
//! SECURITY: All endpoints require AdminUser authentication
//! Features:
//! - Organization profile management (company info, branding)
//! - Teams CRUD with member counts
//! - Departments CRUD with hierarchy
//! - Audit logging for all changes
//!
//! R27-B3 (2026-05-20): the original 1,233-LOC `organization.rs` was
//! split into this `organization/` directory as a PURE STRUCTURAL MOVE.
//! Every handler, SQL statement (including the singleton-row UPSERT
//! into `organization_profile WHERE id = 1`, the
//! `team_members`/`department_members` count subselects, the slug
//! uniqueness guard, the circular-parent guard, the refusal-to-delete-
//! with-children / -with-members invariants), audit-log call site,
//! tracing field, and JSON error envelope is preserved byte-for-byte.
//! No behavioural change.
//!
//! Public API contract (re-exported below — consumed by
//! `tests/organization_test.rs`):
//!   - DTOs: `OrganizationProfile`, `UpdateOrganizationRequest`,
//!     `Team`, `CreateTeamRequest`, `UpdateTeamRequest`,
//!     `Department`, `CreateDepartmentRequest`,
//!     `UpdateDepartmentRequest`, `ListQuery`
//!   - Routers: `teams_router`, `departments_router`,
//!     `profile_router` (ORPHAN — see note below), `admin_router`

use axum::{routing::get, Router};

use crate::AppState;

mod departments;
mod dtos;
mod helpers;
mod profile;
mod teams;

// Re-export the DTOs the integration tests import directly. Keeping
// `routes::organization::{...}` resolvable preserves the module's
// public API across the R27-B3 split.
pub use dtos::{
    CreateDepartmentRequest, CreateTeamRequest, Department, ListQuery, OrganizationProfile, Team,
    UpdateDepartmentRequest, UpdateOrganizationRequest, UpdateTeamRequest,
};

// ═══════════════════════════════════════════════════════════════════════════
// ROUTERS
// ═══════════════════════════════════════════════════════════════════════════

/// Teams admin router
pub fn teams_router() -> Router<AppState> {
    Router::new()
        .route("/", get(teams::list_teams).post(teams::create_team))
        .route(
            "/{id}",
            get(teams::get_team)
                .put(teams::update_team)
                .delete(teams::delete_team),
        )
}

/// Departments admin router
pub fn departments_router() -> Router<AppState> {
    Router::new()
        .route(
            "/",
            get(departments::list_departments).post(departments::create_department),
        )
        .route(
            "/{id}",
            get(departments::get_department)
                .put(departments::update_department)
                .delete(departments::delete_department),
        )
}

// FIX-2026-05-20: ORPHAN — defined but not mounted in routes/mod.rs.
// Carried over from FIX-2026-04-26 and confirmed by R26-B's user.rs
// split audit + the orphan inventory at
// /tmp/maint-reports/ORPHAN_INVENTORY_2026-05-20.md. Kept as `pub fn`
// (not `pub(crate)`) per the CREATE-not-DELETE hard rule so that when
// someone wires it up via `routes/mod.rs` / `main.rs` (or deletes it),
// the change is intentional and discoverable rather than an accidental
// break. `tests/organization_test.rs::organization_all_routers_build_with_app_state`
// keeps it compile-pinned in the meantime.
/// Organization profile router
pub fn profile_router() -> Router<AppState> {
    Router::new().route(
        "/",
        get(profile::get_organization_profile).put(profile::update_organization_profile),
    )
}

/// Full organization admin router (combined)
pub fn admin_router() -> Router<AppState> {
    Router::new()
        .route(
            "/profile",
            get(profile::get_organization_profile).put(profile::update_organization_profile),
        )
        .nest("/teams", teams_router())
        .nest("/departments", departments_router())
}
