//! Quiz + question CRUD handlers
//! Extracted from `admin_courses.rs` in R13-B split (2026-05-20).

use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use chrono::NaiveDateTime;
use serde_json::json;
use uuid::Uuid;

use crate::middleware::admin::AdminUser;
use crate::AppState;

// ═══════════════════════════════════════════════════════════════════════════════════
// QUIZ CRUD - ICT 7 Grade
// ═══════════════════════════════════════════════════════════════════════════════════

/// List all quizzes for a course
pub(super) async fn list_quizzes(
    _admin: AdminUser,
    State(state): State<AppState>,
    Path(course_id): Path<Uuid>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let quizzes: Vec<serde_json::Value> = sqlx::query_as::<_, (i64, String, Option<String>, Option<Uuid>, Option<i64>, String, Option<i32>, bool, i32, NaiveDateTime)>(
        r"
        SELECT id, title, description, lesson_id, module_id, quiz_type, passing_score, is_published, sort_order, created_at
        FROM course_quizzes WHERE course_id = $1 ORDER BY sort_order
        ",
    )
    .bind(course_id)
    .fetch_all(&state.db.pool)
    .await
    .unwrap_or_default()
    .into_iter()
    .map(|q| json!({
        "id": q.0,
        "title": q.1,
        "description": q.2,
        "lesson_id": q.3,
        "module_id": q.4,
        "quiz_type": q.5,
        "passing_score": q.6,
        "is_published": q.7,
        "sort_order": q.8,
        "created_at": q.9.format("%Y-%m-%dT%H:%M:%S").to_string()
    }))
    .collect();

    Ok(Json(json!({
        "success": true,
        "data": quizzes
    })))
}

/// Create a new quiz
pub(super) async fn create_quiz(
    _admin: AdminUser,
    State(state): State<AppState>,
    Path(course_id): Path<Uuid>,
    Json(input): Json<serde_json::Value>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let title = input["title"].as_str().unwrap_or("Untitled Quiz");
    let description = input["description"].as_str();
    let lesson_id = input["lesson_id"]
        .as_str()
        .and_then(|s| Uuid::parse_str(s).ok());
    let module_id = input["module_id"].as_i64();
    let quiz_type = input["quiz_type"].as_str().unwrap_or("graded");
    let passing_score = input["passing_score"]
        .as_i64()
        .map(|v| v as i32)
        .unwrap_or(70);
    let time_limit = input["time_limit_minutes"].as_i64().map(|v| v as i32);
    let max_attempts = input["max_attempts"].as_i64().map(|v| v as i32);
    let shuffle_questions = input["shuffle_questions"].as_bool().unwrap_or(false);
    let shuffle_answers = input["shuffle_answers"].as_bool().unwrap_or(false);
    let show_correct = input["show_correct_answers"].as_bool().unwrap_or(true);
    let is_required = input["is_required"].as_bool().unwrap_or(false);

    let max_order: (Option<i32>,) =
        sqlx::query_as("SELECT MAX(sort_order) FROM course_quizzes WHERE course_id = $1")
            .bind(course_id)
            .fetch_one(&state.db.pool)
            .await
            .unwrap_or((None,));

    let quiz = sqlx::query_as::<_, (i64, String, String, bool, NaiveDateTime)>(
        r"
        INSERT INTO course_quizzes (
            course_id, lesson_id, module_id, title, description, quiz_type,
            passing_score, time_limit_minutes, max_attempts, shuffle_questions,
            shuffle_answers, show_correct_answers, is_required, is_published, sort_order
        ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, false, $14)
        RETURNING id, title, quiz_type, is_published, created_at
        ",
    )
    .bind(course_id)
    .bind(lesson_id)
    .bind(module_id)
    .bind(title)
    .bind(description)
    .bind(quiz_type)
    .bind(passing_score)
    .bind(time_limit)
    .bind(max_attempts)
    .bind(shuffle_questions)
    .bind(shuffle_answers)
    .bind(show_correct)
    .bind(is_required)
    .bind(max_order.0.unwrap_or(0) + 1)
    .fetch_one(&state.db.pool)
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": format!("Failed to create quiz: {}", e)})),
        )
    })?;

    Ok(Json(json!({
        "success": true,
        "message": "Quiz created successfully",
        "data": {
            "id": quiz.0,
            "title": quiz.1,
            "quiz_type": quiz.2,
            "is_published": quiz.3,
            "created_at": quiz.4.format("%Y-%m-%dT%H:%M:%S").to_string()
        }
    })))
}

/// Get quiz with questions
pub(super) async fn get_quiz(
    _admin: AdminUser,
    State(state): State<AppState>,
    Path((course_id, quiz_id)): Path<(Uuid, i64)>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let quiz: serde_json::Value = sqlx::query_as::<
        _,
        (
            i64,
            Uuid,
            String,
            Option<String>,
            String,
            Option<i32>,
            Option<i32>,
            Option<i32>,
            bool,
            bool,
            bool,
            bool,
            bool,
            i32,
        ),
    >(
        r"
        SELECT id, course_id, title, description, quiz_type, passing_score, time_limit_minutes,
               max_attempts, shuffle_questions, shuffle_answers, show_correct_answers, is_required,
               is_published, sort_order
        FROM course_quizzes WHERE id = $1 AND course_id = $2
        ",
    )
    .bind(quiz_id)
    .bind(course_id)
    .fetch_optional(&state.db.pool)
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": format!("Database error: {}", e)})),
        )
    })?
    .map(|q| {
        json!({
            "id": q.0, "course_id": q.1, "title": q.2, "description": q.3, "quiz_type": q.4,
            "passing_score": q.5, "time_limit_minutes": q.6, "max_attempts": q.7,
            "shuffle_questions": q.8, "shuffle_answers": q.9, "show_correct_answers": q.10,
            "is_required": q.11, "is_published": q.12, "sort_order": q.13
        })
    })
    .ok_or_else(|| {
        (
            StatusCode::NOT_FOUND,
            Json(json!({"error": "Quiz not found"})),
        )
    })?;

    let questions: Vec<serde_json::Value> = sqlx::query_as::<_, (i64, String, String, Option<String>, i32, i32)>(
        r"SELECT id, question_type, question_text, explanation, points, sort_order
           FROM quiz_questions WHERE quiz_id = $1 ORDER BY sort_order",
    )
    .bind(quiz_id)
    .fetch_all(&state.db.pool)
    .await
    .unwrap_or_default()
    .into_iter()
    .map(|q| json!({"id": q.0, "question_type": q.1, "question_text": q.2, "explanation": q.3, "points": q.4, "sort_order": q.5}))
    .collect();

    Ok(Json(json!({
        "success": true,
        "data": {
            "quiz": quiz,
            "questions": questions
        }
    })))
}

/// Update quiz
pub(super) async fn update_quiz(
    _admin: AdminUser,
    State(state): State<AppState>,
    Path((course_id, quiz_id)): Path<(Uuid, i64)>,
    Json(input): Json<serde_json::Value>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    sqlx::query(
        r"
        UPDATE course_quizzes SET
            title = COALESCE($1, title),
            description = COALESCE($2, description),
            quiz_type = COALESCE($3, quiz_type),
            passing_score = COALESCE($4, passing_score),
            time_limit_minutes = COALESCE($5, time_limit_minutes),
            max_attempts = COALESCE($6, max_attempts),
            shuffle_questions = COALESCE($7, shuffle_questions),
            shuffle_answers = COALESCE($8, shuffle_answers),
            show_correct_answers = COALESCE($9, show_correct_answers),
            is_required = COALESCE($10, is_required),
            is_published = COALESCE($11, is_published),
            updated_at = NOW()
        WHERE id = $12 AND course_id = $13
        ",
    )
    .bind(input["title"].as_str())
    .bind(input["description"].as_str())
    .bind(input["quiz_type"].as_str())
    .bind(input["passing_score"].as_i64().map(|v| v as i32))
    .bind(input["time_limit_minutes"].as_i64().map(|v| v as i32))
    .bind(input["max_attempts"].as_i64().map(|v| v as i32))
    .bind(input["shuffle_questions"].as_bool())
    .bind(input["shuffle_answers"].as_bool())
    .bind(input["show_correct_answers"].as_bool())
    .bind(input["is_required"].as_bool())
    .bind(input["is_published"].as_bool())
    .bind(quiz_id)
    .bind(course_id)
    .execute(&state.db.pool)
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": format!("Failed to update quiz: {}", e)})),
        )
    })?;

    Ok(Json(
        json!({"success": true, "message": "Quiz updated successfully"}),
    ))
}

/// Delete quiz
pub(super) async fn delete_quiz(
    _admin: AdminUser,
    State(state): State<AppState>,
    Path((course_id, quiz_id)): Path<(Uuid, i64)>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    sqlx::query("DELETE FROM course_quizzes WHERE id = $1 AND course_id = $2")
        .bind(quiz_id)
        .bind(course_id)
        .execute(&state.db.pool)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": format!("Failed to delete quiz: {}", e)})),
            )
        })?;

    Ok(Json(
        json!({"success": true, "message": "Quiz deleted successfully"}),
    ))
}

/// Add question to quiz
pub(super) async fn add_quiz_question(
    _admin: AdminUser,
    State(state): State<AppState>,
    Path((course_id, quiz_id)): Path<(Uuid, i64)>,
    Json(input): Json<serde_json::Value>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let question_type = input["question_type"].as_str().unwrap_or("multiple_choice");
    let question_text = input["question_text"].as_str().unwrap_or("");
    let explanation = input["explanation"].as_str();
    let points = input["points"].as_i64().unwrap_or(1) as i32;

    let max_order: (Option<i32>,) =
        sqlx::query_as("SELECT MAX(sort_order) FROM quiz_questions WHERE quiz_id = $1")
            .bind(quiz_id)
            .fetch_one(&state.db.pool)
            .await
            .unwrap_or((None,));

    let question = sqlx::query_as::<_, (i64, String, String, i32)>(
        r"
        INSERT INTO quiz_questions (quiz_id, question_type, question_text, explanation, points, sort_order)
        VALUES ($1, $2, $3, $4, $5, $6)
        RETURNING id, question_type, question_text, points
        ",
    )
    .bind(quiz_id)
    .bind(question_type)
    .bind(question_text)
    .bind(explanation)
    .bind(points)
    .bind(max_order.0.unwrap_or(0) + 1)
    .fetch_one(&state.db.pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": format!("Failed to add question: {}", e)}))))?;

    // Add answers if provided
    if let Some(answers) = input["answers"].as_array() {
        for (i, answer) in answers.iter().enumerate() {
            let _ = sqlx::query(
                "INSERT INTO quiz_answers (question_id, answer_text, is_correct, sort_order, feedback) VALUES ($1, $2, $3, $4, $5)"
            )
            .bind(question.0)
            .bind(answer["answer_text"].as_str().unwrap_or(""))
            .bind(answer["is_correct"].as_bool().unwrap_or(false))
            .bind(i as i32)
            .bind(answer["feedback"].as_str())
            .execute(&state.db.pool)
            .await;
        }
    }

    Ok(Json(json!({
        "success": true,
        "message": "Question added successfully",
        "data": {"id": question.0, "question_type": question.1, "question_text": question.2, "points": question.3}
    })))
}

/// Delete question from quiz
pub(super) async fn delete_quiz_question(
    _admin: AdminUser,
    State(state): State<AppState>,
    Path((_course_id, _quiz_id, question_id)): Path<(Uuid, i64, i64)>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    sqlx::query("DELETE FROM quiz_questions WHERE id = $1")
        .bind(question_id)
        .execute(&state.db.pool)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": format!("Failed to delete question: {}", e)})),
            )
        })?;

    Ok(Json(
        json!({"success": true, "message": "Question deleted successfully"}),
    ))
}
