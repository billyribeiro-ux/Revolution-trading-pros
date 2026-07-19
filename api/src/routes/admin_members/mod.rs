//! Admin Members Controller - ICT 11+ Principal Engineer
//! Apple ICT 7 Grade - January 2026
//!
//! Member segments, tags, and saved filters management for CRM.
//! All routes require admin privileges.
//!
//! R24-B split: this module was a single 1374-line file; it has been split
//! by sub-domain (segments / tags / filters / analytics / notes_emails) for
//! navigability. Public API is unchanged — only `router()` is exported,
//! mounted at `/admin/members` from `routes::mod.rs`.

use axum::{
    routing::{delete, get, post},
    Router,
};

use crate::AppState;

mod analytics;
mod filters;
mod notes_emails;
mod segments;
mod tags;

// Re-export shared DTOs at the module root so external callers
// (integration tests, etc.) keep the same `routes::admin_members::Foo`
// paths they used before the R24-B split.
pub use analytics::AnalyticsRangeQuery;
pub use filters::{
    CreateMemberFilterRequest, MemberFilter, MemberFilterQuery, UpdateMemberFilterRequest,
};
pub use notes_emails::{CreateNoteRequest, MemberEmail, MemberNote};
pub use segments::{CreateSegmentRequest, MemberSegment, SegmentQuery, UpdateSegmentRequest};
pub use tags::{
    AssignTagRequest, BulkAssignTagsRequest, CreateMemberTagRequest, MemberTag, MemberTagQuery,
    UpdateMemberTagRequest,
};

// ===============================================================================
// AUTHORIZATION
// ===============================================================================
//
// Batch 5b: local `require_admin` and `require_superadmin` helpers
// removed. Read-only admin endpoints now use the `AdminUser` extractor;
// destructive endpoints (delete segment, delete tag, delete filter,
// bulk-assign tags — per audit 02 §P1-10) use `SuperAdminUser`. Both
// extractors live in `crate::middleware::admin`.

// ===============================================================================
// ROUTER
// ===============================================================================

/// Build the admin members router for segments, tags, and filters
pub fn router() -> Router<AppState> {
    Router::new()
        // Segments
        .route(
            "/segments",
            get(segments::list_segments).post(segments::create_segment),
        )
        .route(
            "/segments/{id}",
            get(segments::get_segment)
                .put(segments::update_segment)
                .delete(segments::delete_segment),
        )
        // Tags
        .route(
            "/tags",
            get(tags::list_member_tags).post(tags::create_member_tag),
        )
        .route(
            "/tags/{id}",
            get(tags::get_member_tag)
                .put(tags::update_member_tag)
                .delete(tags::delete_member_tag),
        )
        .route("/tags/assign", post(tags::assign_tag_to_user))
        .route("/tags/unassign", delete(tags::unassign_tag_from_user))
        .route("/tags/bulk-assign", post(tags::bulk_assign_tags))
        // Filters
        .route(
            "/filters",
            get(filters::list_member_filters).post(filters::create_member_filter),
        )
        .route(
            "/filters/{id}",
            get(filters::get_member_filter)
                .put(filters::update_member_filter)
                .delete(filters::delete_member_filter),
        )
        // Analytics (ICT 7 FIX: Missing endpoints)
        .route("/analytics/metrics", get(analytics::analytics_metrics))
        .route("/analytics/growth", get(analytics::analytics_growth))
        .route("/analytics/cohorts", get(analytics::analytics_cohorts))
        .route("/analytics/revenue", get(analytics::analytics_revenue))
        .route(
            "/analytics/churn-reasons",
            get(analytics::analytics_churn_reasons),
        )
        .route("/analytics/segments", get(analytics::analytics_segments))
        // Member Notes & Emails (ICT 7 FIX: Missing endpoints)
        .route(
            "/{id}/notes",
            get(notes_emails::get_member_notes).post(notes_emails::create_member_note),
        )
        .route("/{id}/emails", get(notes_emails::get_member_emails))
}
