//! Course Management Admin API Routes
//! Apple Principal Engineer ICT 7 Grade - January 2026
//!
//! R13-B SPLIT (2026-05-20): Originally a 2,637 LOC monolith
//! `routes/admin_courses.rs`. Split into 7 sub-domain files preserving
//! the exact public API (`admin_courses::router()` and
//! `admin_courses::taxonomy_router()`) byte-for-byte.
//!
//! - `crud`              — Course CRUD lifecycle (list / get / create /
//!   update / delete / publish / unpublish / archive / restore)
//! - `enrollments`       — Enrollment CRUD + enrollment statistics
//! - `structure`         — Modules + lessons + prerequisites
//! - `media`             — Downloads CRUD + video / file upload URLs
//! - `quizzes`           — Quiz + question CRUD
//! - `taxonomy`          — Categories, tags, and course mappings
//! - `analytics_pricing` — Course analytics + Stripe price change

use axum::{
    routing::{get, post, put},
    Router,
};

use crate::AppState;

mod analytics_pricing;
mod crud;
mod enrollments;
mod media;
mod quizzes;
mod structure;
mod taxonomy;

// ═══════════════════════════════════════════════════════════════════════════════════
// SHARED HELPERS
// ═══════════════════════════════════════════════════════════════════════════════════

pub(super) fn slugify(text: &str) -> String {
    text.to_lowercase()
        .chars()
        .map(|c| {
            if c.is_alphanumeric() || c == ' ' {
                c
            } else {
                ' '
            }
        })
        .collect::<String>()
        .split_whitespace()
        .collect::<Vec<&str>>()
        .join("-")
}

// ═══════════════════════════════════════════════════════════════════════════════════
// ROUTER
// ═══════════════════════════════════════════════════════════════════════════════════

pub fn router() -> Router<AppState> {
    Router::new()
        // Courses
        .route("/", get(crud::list_courses).post(crud::create_course))
        .route(
            "/{id}",
            get(crud::get_course)
                .put(crud::update_course)
                .delete(crud::delete_course),
        )
        .route("/{id}/publish", post(crud::publish_course))
        .route("/{id}/unpublish", post(crud::unpublish_course))
        .route("/{id}/archive", post(crud::archive_course))
        .route("/{id}/restore", post(crud::restore_course))
        .route(
            "/{id}/change-price",
            post(analytics_pricing::change_course_price),
        )
        // Enrollments
        .route(
            "/{course_id}/enrollments",
            get(enrollments::list_enrollments).post(enrollments::enroll_user),
        )
        .route(
            "/{course_id}/enrollments/{enrollment_id}",
            axum::routing::delete(enrollments::remove_enrollment),
        )
        .route("/{course_id}/stats", get(enrollments::get_enrollment_stats))
        // FIX-2026-04-26: New endpoint backing CourseDetailDrawer Analytics tab.
        .route(
            "/{course_id}/analytics",
            get(analytics_pricing::get_course_analytics),
        )
        // Modules
        .route(
            "/{course_id}/modules",
            get(structure::list_modules).post(structure::create_module),
        )
        .route(
            "/{course_id}/modules/{module_id}",
            put(structure::update_module).delete(structure::delete_module),
        )
        .route(
            "/{course_id}/modules/reorder",
            put(structure::reorder_modules),
        )
        // Lessons
        .route(
            "/{course_id}/lessons",
            get(structure::list_lessons).post(structure::create_lesson),
        )
        .route(
            "/{course_id}/lessons/{lesson_id}",
            get(structure::get_lesson)
                .put(structure::update_lesson)
                .delete(structure::delete_lesson),
        )
        .route(
            "/{course_id}/lessons/reorder",
            put(structure::reorder_lessons),
        )
        .route(
            "/{course_id}/lessons/{lesson_id}/prerequisites",
            get(structure::get_lesson_prerequisites).put(structure::update_lesson_prerequisites),
        )
        // Quizzes - ICT 7 Grade
        .route(
            "/{course_id}/quizzes",
            get(quizzes::list_quizzes).post(quizzes::create_quiz),
        )
        .route(
            "/{course_id}/quizzes/{quiz_id}",
            get(quizzes::get_quiz)
                .put(quizzes::update_quiz)
                .delete(quizzes::delete_quiz),
        )
        .route(
            "/{course_id}/quizzes/{quiz_id}/questions",
            post(quizzes::add_quiz_question),
        )
        .route(
            "/{course_id}/quizzes/{quiz_id}/questions/{question_id}",
            axum::routing::delete(quizzes::delete_quiz_question),
        )
        // Categories & Tags - ICT 7 Grade
        .route(
            "/{course_id}/categories",
            put(taxonomy::update_course_categories),
        )
        .route("/{course_id}/tags", put(taxonomy::update_course_tags))
        // Downloads
        .route(
            "/{course_id}/downloads",
            get(media::list_downloads).post(media::create_download),
        )
        .route(
            "/{course_id}/downloads/{download_id}",
            put(media::update_download).delete(media::delete_download),
        )
        .route("/{course_id}/upload-url", post(media::get_upload_url))
        // Video Upload (TUS)
        .route("/{course_id}/video-upload", post(media::create_video_upload))
}

// FIX-2026-04-26: ORPHAN — defined but never registered/called. Either wire up via routes/mod.rs / main.rs or delete in follow-up.
/// Separate router for global category/tag management
pub fn taxonomy_router() -> Router<AppState> {
    Router::new()
        .route(
            "/categories",
            get(taxonomy::list_categories).post(taxonomy::create_category),
        )
        .route(
            "/categories/{id}",
            axum::routing::delete(taxonomy::delete_category),
        )
        .route("/tags", get(taxonomy::list_tags).post(taxonomy::create_tag))
        .route("/tags/{id}", axum::routing::delete(taxonomy::delete_tag))
}
