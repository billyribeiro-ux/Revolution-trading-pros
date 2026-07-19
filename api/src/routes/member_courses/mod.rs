//! Member Course API Routes
//! Apple Principal Engineer ICT 7 Grade - February 2026
//! Public and authenticated member endpoints for courses.
//!
//! Structural split (R17-B) — sub-modules group handlers by sub-domain so
//! the original 1671-line single file is no longer load-bearing. Public
//! API (`public_router`, `member_router`) is unchanged.

use axum::{
    routing::{get, put},
    Router,
};

use crate::AppState;

mod certificates;
mod entitlement;
mod player;
mod prerequisites;
mod public;
mod quizzes;
mod reviews;

// ═══════════════════════════════════════════════════════════════════════════════════
// ROUTER
// ═══════════════════════════════════════════════════════════════════════════════════

pub fn public_router() -> Router<AppState> {
    Router::new()
        .route("/", get(public::list_published_courses))
        .route("/{slug}", get(public::get_course_detail))
        .route("/{slug}/reviews", get(public::get_course_reviews))
}

pub fn member_router() -> Router<AppState> {
    use axum::routing::{delete, post};

    Router::new()
        .route("/", get(player::get_my_courses))
        .route("/{slug}/player", get(player::get_course_player))
        .route("/{slug}/progress", put(player::update_lesson_progress))
        .route("/{slug}/downloads", get(player::get_course_downloads))
        .route("/{slug}/certificate", get(certificates::get_certificate))
        .route("/{slug}/resume", get(certificates::resume_course))
        // Reviews
        .route(
            "/{slug}/reviews",
            post(reviews::submit_review).delete(reviews::delete_review),
        )
        // Quizzes
        .route("/{slug}/quizzes", get(quizzes::get_course_quizzes))
        .route("/{slug}/quizzes/{quiz_id}/start", post(quizzes::start_quiz))
        .route(
            "/{slug}/quizzes/{quiz_id}/attempts/{attempt_id}/submit",
            post(quizzes::submit_quiz),
        )
        .route(
            "/{slug}/quizzes/{quiz_id}/results",
            get(quizzes::get_quiz_results),
        )
        // Prerequisites
        .route(
            "/{slug}/lessons/{lesson_id}/access",
            get(prerequisites::check_lesson_access),
        )
}
