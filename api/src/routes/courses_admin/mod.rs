//! Enhanced Course Admin Routes - Revolution Trading Pros
//! Apple Principal Engineer ICT 7 Grade - January 2026
//!
//! R14-B SPLIT (2026-05-20): Originally a 1,775 LOC monolith
//! `routes/courses_admin.rs`. Split into 7 sub-domain files preserving
//! the exact public API (`courses_admin::router()`) byte-for-byte.
//!
//! - `crud`          — Course CRUD (list / get / create / update / delete)
//! - `sections`      — Section endpoints (create / update / delete / reorder)
//! - `lessons`       — Lesson endpoints (create / update / delete / reorder)
//! - `resources`     — Course resource endpoints (create / delete)
//! - `live_sessions` — Live session endpoints (create / bulk / delete)
//! - `enrollments`   — Enrollment + per-lesson progress endpoints
//! - `utility`       — Categories, stats, clone-course
//!
//! SECURITY: All endpoints require AdminUser authentication. SQL is
//! parameterized via sqlx — preserved verbatim from the monolith.

use axum::{
    routing::{delete, get, post, put},
    Router,
};

use crate::AppState;

mod crud;
mod enrollments;
mod lessons;
mod live_sessions;
mod resources;
mod sections;
mod utility;

// ═══════════════════════════════════════════════════════════════════════════════════
// ROUTER SETUP
// ═══════════════════════════════════════════════════════════════════════════════════

pub fn router() -> Router<AppState> {
    Router::new()
        // Courses
        .route("/", get(crud::list_courses).post(crud::create_course))
        .route("/stats", get(utility::get_course_stats))
        .route("/categories", get(utility::get_categories))
        .route(
            "/{course_id}",
            get(crud::get_course)
                .put(crud::update_course)
                .delete(crud::delete_course),
        )
        .route("/{course_id}/clone", post(utility::clone_course))
        // Sections
        .route("/{course_id}/sections", post(sections::create_section))
        .route(
            "/{course_id}/sections/reorder",
            put(sections::reorder_sections),
        )
        .route(
            "/{course_id}/sections/{section_id}",
            put(sections::update_section).delete(sections::delete_section),
        )
        // Lessons
        .route("/{course_id}/lessons", post(lessons::create_lesson))
        .route(
            "/{course_id}/lessons/{lesson_id}",
            put(lessons::update_lesson).delete(lessons::delete_lesson),
        )
        .route(
            "/{course_id}/sections/{section_id}/lessons/reorder",
            put(lessons::reorder_lessons),
        )
        // Resources
        .route("/{course_id}/resources", post(resources::create_resource))
        .route(
            "/{course_id}/resources/{resource_id}",
            delete(resources::delete_resource),
        )
        // Live Sessions
        .route(
            "/{course_id}/live-sessions",
            post(live_sessions::create_live_session),
        )
        .route(
            "/{course_id}/live-sessions/bulk",
            post(live_sessions::bulk_create_live_sessions),
        )
        .route(
            "/{course_id}/live-sessions/{session_id}",
            delete(live_sessions::delete_live_session),
        )
        // Enrollments & Progress
        .route("/{course_id}/enroll", post(enrollments::enroll_user))
        .route("/{course_id}/enrollments", get(enrollments::get_enrollments))
        .route(
            "/{course_id}/lessons/{lesson_id}/progress",
            post(enrollments::update_lesson_progress),
        )
        .route(
            "/{course_id}/progress/{user_id}",
            get(enrollments::get_user_progress),
        )
}
