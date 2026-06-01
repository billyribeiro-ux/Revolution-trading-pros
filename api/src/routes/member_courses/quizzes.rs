//! Quiz endpoints — listing, starting, submitting, and reviewing past results.
//!
//! All quiz routes are gated behind course enrollment. The `start_quiz`
//! handler hands back questions+answers (without `is_correct` for
//! students); `submit_quiz` scores against `quiz_answers.is_correct` and
//! marks the attempt complete.

use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use chrono::NaiveDateTime;
use serde_json::json;
use uuid::Uuid;

use crate::models::User;
use crate::AppState;

/// Get available quizzes for a course
pub(super) async fn get_course_quizzes(
    user: User,
    State(state): State<AppState>,
    Path(slug): Path<String>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let user_id = user.id;

    let course: (Uuid,) = sqlx::query_as("SELECT id FROM courses WHERE slug = $1")
        .bind(&slug)
        .fetch_optional(&state.db.pool)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": format!("Database error: {}", e)})),
            )
        })?
        .ok_or_else(|| {
            (
                StatusCode::NOT_FOUND,
                Json(json!({"error": "Course not found"})),
            )
        })?;

    // Check enrollment
    let enrollment: Option<(i64,)> = sqlx::query_as(
        "SELECT id FROM user_course_enrollments WHERE user_id = $1 AND course_id = $2",
    )
    .bind(user_id)
    .bind(course.0)
    .fetch_optional(&state.db.pool)
    .await
    .ok()
    .flatten();

    if enrollment.is_none() {
        return Err((
            StatusCode::FORBIDDEN,
            Json(json!({"error": "Not enrolled in this course"})),
        ));
    }

    let quizzes: Vec<serde_json::Value> = sqlx::query_as::<_, (i64, String, Option<String>, String, Option<i32>, Option<i32>, Option<i32>, bool)>(
        r"SELECT id, title, description, quiz_type, passing_score, time_limit_minutes, max_attempts, is_required
           FROM course_quizzes WHERE course_id = $1 AND is_published = true ORDER BY sort_order",
    )
    .bind(course.0)
    .fetch_all(&state.db.pool)
    .await
    .unwrap_or_default()
    .into_iter()
    .map(|q| json!({"id": q.0, "title": q.1, "description": q.2, "quiz_type": q.3, "passing_score": q.4, "time_limit_minutes": q.5, "max_attempts": q.6, "is_required": q.7}))
    .collect();

    Ok(Json(json!({"success": true, "data": quizzes})))
}

/// Start a quiz attempt
pub(super) async fn start_quiz(
    user: User,
    State(state): State<AppState>,
    Path((slug, quiz_id)): Path<(String, i64)>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let user_id = user.id;

    let course: (Uuid,) = sqlx::query_as("SELECT id FROM courses WHERE slug = $1")
        .bind(&slug)
        .fetch_optional(&state.db.pool)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": format!("Database error: {}", e)})),
            )
        })?
        .ok_or_else(|| {
            (
                StatusCode::NOT_FOUND,
                Json(json!({"error": "Course not found"})),
            )
        })?;

    // Get enrollment
    let enrollment: (i64,) = sqlx::query_as(
        "SELECT id FROM user_course_enrollments WHERE user_id = $1 AND course_id = $2",
    )
    .bind(user_id)
    .bind(course.0)
    .fetch_optional(&state.db.pool)
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": format!("Database error: {}", e)})),
        )
    })?
    .ok_or_else(|| {
        (
            StatusCode::FORBIDDEN,
            Json(json!({"error": "Not enrolled in this course"})),
        )
    })?;

    // Get quiz details
    let quiz: (i64, String, Option<i32>, Option<i32>, bool, bool) = sqlx::query_as(
        "SELECT id, title, max_attempts, time_limit_minutes, shuffle_questions, shuffle_answers FROM course_quizzes WHERE id = $1 AND course_id = $2 AND is_published = true"
    )
    .bind(quiz_id)
    .bind(course.0)
    .fetch_optional(&state.db.pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": format!("Database error: {}", e)}))))?
    .ok_or_else(|| (StatusCode::NOT_FOUND, Json(json!({"error": "Quiz not found"}))))?;

    // Check attempt count
    let attempt_count: (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM user_quiz_attempts WHERE user_id = $1 AND quiz_id = $2",
    )
    .bind(user_id)
    .bind(quiz_id)
    .fetch_one(&state.db.pool)
    .await
    .unwrap_or((0,));

    if let Some(max) = quiz.2 {
        if attempt_count.0 >= max as i64 {
            return Err((
                StatusCode::FORBIDDEN,
                Json(json!({"error": "Maximum attempts reached", "max_attempts": max})),
            ));
        }
    }

    // Get questions.
    //
    // SAFETY: `order_clause` is a binary choice between two literal SQL
    // fragments (`"RANDOM()"` and `"sort_order"`) selected by the
    // `shuffle_questions` boolean column from the DB — no user input
    // reaches this format string. `quiz_id` is bound below as `$1`.
    let order_clause = if quiz.4 { "RANDOM()" } else { "sort_order" };
    let questions: Vec<serde_json::Value> = sqlx::query_as::<_, (i64, String, String, i32)>(
        &format!("SELECT id, question_type, question_text, points FROM quiz_questions WHERE quiz_id = $1 ORDER BY {order_clause}")
    )
    .bind(quiz_id)
    .fetch_all(&state.db.pool)
    .await
    .unwrap_or_default()
    .into_iter()
    .map(|q| {
        let question_id = q.0;
        json!({"id": question_id, "question_type": q.1, "question_text": q.2, "points": q.3})
    })
    .collect();

    // Get answers for each question (without is_correct for students)
    let mut questions_with_answers = Vec::new();
    for mut question in questions {
        let question_id = question["id"].as_i64().unwrap_or(0);
        // SAFETY: `answer_order` is a binary choice between two literal SQL
        // fragments (`"RANDOM()"` and `"sort_order"`) selected by the
        // `shuffle_answers` boolean column from the DB — no user input
        // reaches this format string. `question_id` is bound below as `$1`.
        let answer_order = if quiz.5 { "RANDOM()" } else { "sort_order" };
        let answers: Vec<serde_json::Value> = sqlx::query_as::<_, (i64, String)>(&format!(
            "SELECT id, answer_text FROM quiz_answers WHERE question_id = $1 ORDER BY {answer_order}"
        ))
        .bind(question_id)
        .fetch_all(&state.db.pool)
        .await
        .unwrap_or_default()
        .into_iter()
        .map(|a| json!({"id": a.0, "answer_text": a.1}))
        .collect();
        question["answers"] = json!(answers);
        questions_with_answers.push(question);
    }

    // Calculate max score
    let max_score: (Option<i64>,) =
        sqlx::query_as("SELECT SUM(points) FROM quiz_questions WHERE quiz_id = $1")
            .bind(quiz_id)
            .fetch_one(&state.db.pool)
            .await
            .unwrap_or((None,));

    // Create attempt
    let attempt = sqlx::query_as::<_, (i64, i32, NaiveDateTime)>(
        r"INSERT INTO user_quiz_attempts (user_id, quiz_id, enrollment_id, max_score, attempt_number, started_at)
           VALUES ($1, $2, $3, $4, $5, NOW()) RETURNING id, attempt_number, started_at"
    )
    .bind(user_id)
    .bind(quiz_id)
    .bind(enrollment.0)
    .bind(max_score.0.unwrap_or(0) as i32)
    .bind(attempt_count.0 as i32 + 1)
    .fetch_one(&state.db.pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": format!("Failed to start quiz: {}", e)}))))?;

    Ok(Json(json!({
        "success": true,
        "data": {
            "attempt_id": attempt.0,
            "attempt_number": attempt.1,
            "started_at": attempt.2.format("%Y-%m-%dT%H:%M:%S").to_string(),
            "quiz_title": quiz.1,
            "time_limit_minutes": quiz.3,
            "max_score": max_score.0.unwrap_or(0),
            "questions": questions_with_answers
        }
    })))
}

/// Submit quiz answers
pub(super) async fn submit_quiz(
    user: User,
    State(state): State<AppState>,
    Path((_slug, quiz_id, attempt_id)): Path<(String, i64, i64)>,
    Json(input): Json<serde_json::Value>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let user_id = user.id;

    // Verify attempt belongs to user
    let attempt: (i64, i32, NaiveDateTime) = sqlx::query_as(
        "SELECT id, max_score, started_at FROM user_quiz_attempts WHERE id = $1 AND user_id = $2 AND quiz_id = $3 AND completed_at IS NULL"
    )
    .bind(attempt_id)
    .bind(user_id)
    .bind(quiz_id)
    .fetch_optional(&state.db.pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": format!("Database error: {}", e)}))))?
    .ok_or_else(|| (StatusCode::NOT_FOUND, Json(json!({"error": "Quiz attempt not found or already submitted"}))))?;

    // Get quiz passing score
    let quiz: (Option<i32>, Option<bool>) = sqlx::query_as(
        "SELECT passing_score, show_correct_answers FROM course_quizzes WHERE id = $1",
    )
    .bind(quiz_id)
    .fetch_one(&state.db.pool)
    .await
    .unwrap_or((Some(70), Some(true)));

    // Calculate score
    let mut total_score = 0i32;
    let empty_vec = Vec::new();
    let answers = input["answers"].as_array().unwrap_or(&empty_vec);
    let mut results = Vec::new();

    for answer in answers {
        let question_id = answer["question_id"].as_i64().unwrap_or(0);
        let answer_id = answer["answer_id"].as_i64();

        // Get correct answer and points
        let question: Option<(i32,)> =
            sqlx::query_as("SELECT points FROM quiz_questions WHERE id = $1")
                .bind(question_id)
                .fetch_optional(&state.db.pool)
                .await
                .ok()
                .flatten();

        if let Some(q) = question {
            if let Some(aid) = answer_id {
                let is_correct: Option<(bool,)> = sqlx::query_as(
                    "SELECT is_correct FROM quiz_answers WHERE id = $1 AND question_id = $2",
                )
                .bind(aid)
                .bind(question_id)
                .fetch_optional(&state.db.pool)
                .await
                .ok()
                .flatten();

                if is_correct.map(|c| c.0).unwrap_or(false) {
                    total_score += q.0;
                    results
                        .push(json!({"question_id": question_id, "correct": true, "points": q.0}));
                } else {
                    results
                        .push(json!({"question_id": question_id, "correct": false, "points": 0}));
                }
            }
        }
    }

    let score_percent = if attempt.1 > 0 {
        total_score as f64 / attempt.1 as f64 * 100.0
    } else {
        0.0
    };
    let passed = score_percent >= quiz.0.unwrap_or(70) as f64;
    let time_spent = (chrono::Utc::now().naive_utc() - attempt.2).num_seconds() as i32;

    // Update attempt
    sqlx::query(
        r"UPDATE user_quiz_attempts SET score = $1, score_percent = $2, passed = $3, completed_at = NOW(), time_spent_seconds = $4, answers = $5 WHERE id = $6"
    )
    .bind(total_score)
    .bind(score_percent)
    .bind(passed)
    .bind(time_spent)
    .bind(json!(answers))
    .bind(attempt_id)
    .execute(&state.db.pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": format!("Failed to submit quiz: {}", e)}))))?;

    Ok(Json(json!({
        "success": true,
        "data": {
            "score": total_score,
            "max_score": attempt.1,
            "score_percent": score_percent,
            "passed": passed,
            "passing_score": quiz.0.unwrap_or(70),
            "time_spent_seconds": time_spent,
            "results": if quiz.1.unwrap_or(true) { json!(results) } else { json!(null) }
        }
    })))
}

/// Get quiz results history
pub(super) async fn get_quiz_results(
    user: User,
    State(state): State<AppState>,
    Path((_slug, quiz_id)): Path<(String, i64)>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let user_id = user.id;

    let attempts: Vec<serde_json::Value> = sqlx::query_as::<_, (i64, Option<i32>, Option<f64>, Option<bool>, i32, NaiveDateTime, Option<NaiveDateTime>)>(
        "SELECT id, score, score_percent, passed, attempt_number, started_at, completed_at FROM user_quiz_attempts WHERE user_id = $1 AND quiz_id = $2 ORDER BY attempt_number DESC"
    )
    .bind(user_id)
    .bind(quiz_id)
    .fetch_all(&state.db.pool)
    .await
    .unwrap_or_default()
    .into_iter()
    .map(|a| json!({
        "attempt_id": a.0, "score": a.1, "score_percent": a.2, "passed": a.3,
        "attempt_number": a.4, "started_at": a.5.format("%Y-%m-%dT%H:%M:%S").to_string(),
        "completed_at": a.6.map(|t| t.format("%Y-%m-%dT%H:%M:%S").to_string())
    }))
    .collect();

    Ok(Json(json!({"success": true, "data": attempts})))
}
