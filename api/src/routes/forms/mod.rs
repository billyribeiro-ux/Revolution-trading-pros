//! Forms Admin Routes - Revolution Trading Pros
//! Apple ICT 7+ Principal Engineer Grade - January 2026
//!
//! Full CRUD for forms and form submissions with admin protection.
//!
//! This module was split (R11-B) from a single 1,660-LOC `forms.rs`
//! into a directory with one file per sub-domain. The public API
//! (`public_router()`, `admin_router()`, and all previously-`pub`
//! DTOs) is preserved exactly — external callers in `routes/mod.rs`
//! and `tests/forms_test.rs` import unchanged.
//!
//! Layout:
//! - `types.rs` — DTOs (FormRow, FormSubmissionRow, request/query structs)
//! - `crud.rs` — admin CRUD + stats + field_types
//! - `submissions.rs` — admin submission management + CSV export
//! - `public.rs` — no-auth list / get / submit / view-track
//! - `analytics.rs` — admin analytics (views, conversion, trends)

use axum::{
    routing::{get, post, put},
    Router,
};

use crate::AppState;

mod analytics;
mod crud;
mod public;
mod submissions;
mod types;

// Re-export the public DTO surface so external consumers (tests,
// `routes::forms::FormRow`, etc.) keep the same import path.
pub use types::{
    BulkDeleteRequest, BulkUpdateStatusRequest, CreateFormRequest, ExportQuery, FormListQuery,
    FormRow, FormSubmissionRow, SubmissionListQuery, UpdateFormRequest,
    UpdateSubmissionStatusRequest,
};

/// Public forms router - /api/forms (frontend compatibility)
pub fn public_router() -> Router<AppState> {
    Router::new()
        .route("/", get(public::list_public_forms))
        .route("/{id}", get(public::get_public_form))
        .route("/{slug}/submit", post(public::submit_form))
        .route("/{slug}/view", post(public::track_form_view))
}

pub fn admin_router() -> Router<AppState> {
    Router::new()
        // Form CRUD
        .route("/", get(crud::list_forms).post(crud::create_form))
        .route("/stats", get(crud::form_stats))
        .route("/field-types", get(crud::field_types))
        .route(
            "/{id}",
            get(crud::get_form)
                .put(crud::update_form)
                .delete(crud::delete_form),
        )
        .route("/{id}/publish", post(crud::publish_form))
        .route("/{id}/unpublish", post(crud::unpublish_form))
        .route("/{id}/duplicate", post(crud::duplicate_form))
        .route("/{id}/analytics", get(analytics::get_form_analytics))
        // Submissions
        .route("/{form_id}/submissions", get(submissions::list_submissions))
        .route(
            "/{form_id}/submissions/export",
            get(submissions::export_submissions),
        )
        .route(
            "/{form_id}/submissions/bulk-update-status",
            post(submissions::bulk_update_status),
        )
        .route(
            "/{form_id}/submissions/bulk-delete",
            post(submissions::bulk_delete_submissions),
        )
        .route(
            "/{form_id}/submissions/{submission_id}",
            get(submissions::get_submission).delete(submissions::delete_submission),
        )
        .route(
            "/{form_id}/submissions/{submission_id}/status",
            put(submissions::update_submission_status),
        )
}
