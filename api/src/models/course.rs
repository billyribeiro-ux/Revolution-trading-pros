//! Course Management System Models
//! Apple Principal Engineer ICT 7 Grade - January 2026

use chrono::{DateTime, NaiveDateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

// ═══════════════════════════════════════════════════════════════════════════════════
// COURSE MODEL
// ═══════════════════════════════════════════════════════════════════════════════════

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Course {
    pub id: Uuid,
    pub title: String,
    pub slug: String,
    pub description: Option<String>,
    pub price_cents: i32,
    pub instructor_id: Option<i64>,
    pub is_published: bool,
    pub thumbnail: Option<String>,
    pub preview_video_url: Option<String>,
    pub duration_minutes: Option<i32>,
    pub level: Option<String>,
    pub metadata: Option<serde_json::Value>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,

    // Card Display
    pub card_image_url: Option<String>,
    pub card_description: Option<String>,
    pub card_badge: Option<String>,
    pub card_badge_color: Option<String>,

    // Course Details
    pub what_you_learn: Option<serde_json::Value>,
    pub requirements: Option<serde_json::Value>,
    pub target_audience: Option<serde_json::Value>,

    // Instructor
    pub instructor_name: Option<String>,
    pub instructor_title: Option<String>,
    pub instructor_avatar_url: Option<String>,
    pub instructor_bio: Option<String>,

    // Pricing & Access
    pub is_free: Option<bool>,
    pub is_premium: Option<bool>,
    pub required_membership_ids: Option<serde_json::Value>,

    // Video Integration
    pub bunny_library_id: Option<i64>,

    // SEO
    pub meta_title: Option<String>,
    pub meta_description: Option<String>,
    pub og_image_url: Option<String>,

    // Status
    pub status: Option<String>,
    pub published_at: Option<NaiveDateTime>,

    // Analytics
    pub enrollment_count: Option<i32>,
    pub completion_rate: Option<f64>,
    pub avg_rating: Option<f64>,
    pub review_count: Option<i32>,

    // Counts
    pub module_count: Option<i32>,
    pub lesson_count: Option<i32>,
    pub total_duration_minutes: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct CourseListItem {
    pub id: Uuid,
    pub title: String,
    pub slug: String,
    pub description: Option<String>,
    pub card_image_url: Option<String>,
    pub card_description: Option<String>,
    pub card_badge: Option<String>,
    pub card_badge_color: Option<String>,
    pub instructor_name: Option<String>,
    pub instructor_avatar_url: Option<String>,
    pub level: Option<String>,
    pub price_cents: i32,
    pub is_free: Option<bool>,
    pub is_published: bool,
    pub status: Option<String>,
    pub module_count: Option<i32>,
    pub lesson_count: Option<i32>,
    pub total_duration_minutes: Option<i32>,
    pub enrollment_count: Option<i32>,
    pub avg_rating: Option<f64>,
    pub review_count: Option<i32>,
    pub created_at: NaiveDateTime,
}

// ═══════════════════════════════════════════════════════════════════════════════════
// MODULE MODEL
// ═══════════════════════════════════════════════════════════════════════════════════

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct CourseModule {
    pub id: i64,
    pub course_id: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub sort_order: i32,
    pub is_published: bool,
    pub drip_enabled: Option<bool>,
    pub drip_days: Option<i32>,
    pub drip_date: Option<NaiveDateTime>,
    pub lesson_count: Option<i32>,
    pub total_duration_minutes: Option<i32>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

// ═══════════════════════════════════════════════════════════════════════════════════
// LESSON MODEL
// ═══════════════════════════════════════════════════════════════════════════════════

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Lesson {
    pub id: Uuid,
    pub course_id: Uuid,
    pub module_id: Option<i64>,
    pub title: String,
    pub slug: String,
    pub description: Option<String>,
    pub video_url: Option<String>,
    pub duration_minutes: Option<i32>,
    pub position: i32,
    pub is_free: bool,
    pub content: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,

    // Video Integration
    pub video_id: Option<i64>,
    pub bunny_video_guid: Option<String>,
    pub thumbnail_url: Option<String>,

    // Content
    pub content_html: Option<String>,
    pub resources: Option<serde_json::Value>,

    // Access Control
    pub is_preview: Option<bool>,
    pub is_published: Option<bool>,
    pub drip_days: Option<i32>,

    // Prerequisites
    pub prerequisite_lesson_ids: Option<serde_json::Value>,

    // Sort
    pub sort_order: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct LessonListItem {
    pub id: Uuid,
    pub course_id: Uuid,
    pub module_id: Option<i64>,
    pub title: String,
    pub slug: String,
    pub description: Option<String>,
    pub duration_minutes: Option<i32>,
    pub position: i32,
    pub sort_order: Option<i32>,
    pub is_free: bool,
    pub is_preview: Option<bool>,
    pub is_published: Option<bool>,
    pub bunny_video_guid: Option<String>,
    pub thumbnail_url: Option<String>,
}

// ═══════════════════════════════════════════════════════════════════════════════════
// DOWNLOAD MODEL
// ═══════════════════════════════════════════════════════════════════════════════════

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct CourseDownload {
    pub id: i64,
    pub course_id: Option<Uuid>,
    pub module_id: Option<i64>,
    pub lesson_id: Option<Uuid>,
    pub title: String,
    pub description: Option<String>,
    pub file_name: String,
    pub file_path: String,
    pub file_size_bytes: Option<i64>,
    pub file_type: Option<String>,
    pub mime_type: Option<String>,
    pub bunny_file_id: Option<String>,
    pub storage_zone: Option<String>,
    pub download_url: Option<String>,
    pub preview_url: Option<String>,
    pub category: Option<String>,
    pub sort_order: Option<i32>,
    pub is_public: Option<bool>,
    pub require_enrollment: Option<bool>,
    pub require_lesson_complete: Option<bool>,
    pub download_count: Option<i32>,
    pub uploaded_by: Option<i64>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

// ═══════════════════════════════════════════════════════════════════════════════════
// ENROLLMENT MODEL
// ═══════════════════════════════════════════════════════════════════════════════════

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct UserCourseEnrollment {
    pub id: i64,
    pub user_id: i64,
    pub course_id: Uuid,
    pub current_module_id: Option<i64>,
    pub current_lesson_id: Option<Uuid>,
    pub completed_lesson_ids: Option<serde_json::Value>,
    pub progress_percent: Option<i16>,
    pub status: Option<String>,
    pub enrolled_at: NaiveDateTime,
    pub started_at: Option<NaiveDateTime>,
    pub completed_at: Option<NaiveDateTime>,
    pub last_accessed_at: Option<NaiveDateTime>,
    pub access_expires_at: Option<NaiveDateTime>,
    pub is_lifetime_access: Option<bool>,
    pub order_id: Option<i64>,
    pub price_paid_cents: Option<i32>,
    pub certificate_issued: Option<bool>,
    pub certificate_url: Option<String>,
    pub certificate_issued_at: Option<NaiveDateTime>,
    pub notes: Option<serde_json::Value>,
    pub bookmarks: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnrollmentWithCourse {
    pub enrollment: UserCourseEnrollment,
    pub course: CourseListItem,
}

// ═══════════════════════════════════════════════════════════════════════════════════
// LESSON PROGRESS MODEL
// ═══════════════════════════════════════════════════════════════════════════════════

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct UserLessonProgress {
    pub id: i64,
    pub user_id: i64,
    pub lesson_id: Uuid,
    pub enrollment_id: i64,
    pub video_position_seconds: Option<i32>,
    pub video_duration_seconds: Option<i32>,
    pub video_watch_percent: Option<i16>,
    pub is_completed: bool,
    pub completed_at: Option<NaiveDateTime>,
    pub time_spent_seconds: Option<i32>,
    pub view_count: Option<i32>,
    pub first_accessed_at: NaiveDateTime,
    pub last_accessed_at: NaiveDateTime,
}

// ═══════════════════════════════════════════════════════════════════════════════════
// REVIEW MODEL
// ═══════════════════════════════════════════════════════════════════════════════════

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct CourseReview {
    pub id: i64,
    pub course_id: Uuid,
    pub user_id: i64,
    pub enrollment_id: Option<i64>,
    pub rating: i16,
    pub title: Option<String>,
    pub content: Option<String>,
    pub is_verified_purchase: Option<bool>,
    pub is_featured: Option<bool>,
    pub is_approved: Option<bool>,
    pub helpful_count: Option<i32>,
    pub not_helpful_count: Option<i32>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReviewWithUser {
    pub review: CourseReview,
    pub user_name: String,
    pub user_avatar: Option<String>,
}

// ═══════════════════════════════════════════════════════════════════════════════════
// REQUEST/RESPONSE DTOs
// ═══════════════════════════════════════════════════════════════════════════════════

#[derive(Debug, Deserialize)]
pub struct CreateCourseRequest {
    pub title: String,
    pub slug: Option<String>,
    pub description: Option<String>,
    pub card_description: Option<String>,
    pub card_image_url: Option<String>,
    pub card_badge: Option<String>,
    pub card_badge_color: Option<String>,
    pub price_cents: Option<i32>,
    pub is_free: Option<bool>,
    pub level: Option<String>,
    pub instructor_name: Option<String>,
    pub instructor_title: Option<String>,
    pub instructor_avatar_url: Option<String>,
    pub instructor_bio: Option<String>,
    pub what_you_learn: Option<Vec<String>>,
    pub requirements: Option<Vec<String>>,
    pub target_audience: Option<Vec<String>>,
    pub meta_title: Option<String>,
    pub meta_description: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateCourseRequest {
    pub title: Option<String>,
    pub slug: Option<String>,
    pub description: Option<String>,
    pub card_description: Option<String>,
    pub card_image_url: Option<String>,
    pub card_badge: Option<String>,
    pub card_badge_color: Option<String>,
    pub price_cents: Option<i32>,
    pub is_free: Option<bool>,
    pub is_published: Option<bool>,
    pub status: Option<String>,
    pub level: Option<String>,
    pub instructor_name: Option<String>,
    pub instructor_title: Option<String>,
    pub instructor_avatar_url: Option<String>,
    pub instructor_bio: Option<String>,
    pub what_you_learn: Option<Vec<String>>,
    pub requirements: Option<Vec<String>>,
    pub target_audience: Option<Vec<String>>,
    pub meta_title: Option<String>,
    pub meta_description: Option<String>,
    pub bunny_library_id: Option<i64>,
}

#[derive(Debug, Deserialize)]
pub struct CreateModuleRequest {
    pub title: String,
    pub description: Option<String>,
    pub sort_order: Option<i32>,
    pub is_published: Option<bool>,
    pub drip_enabled: Option<bool>,
    pub drip_days: Option<i32>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateModuleRequest {
    pub title: Option<String>,
    pub description: Option<String>,
    pub sort_order: Option<i32>,
    pub is_published: Option<bool>,
    pub drip_enabled: Option<bool>,
    pub drip_days: Option<i32>,
}

#[derive(Debug, Deserialize)]
pub struct CreateLessonRequest {
    pub title: String,
    pub slug: Option<String>,
    pub description: Option<String>,
    pub module_id: Option<i64>,
    pub video_url: Option<String>,
    pub bunny_video_guid: Option<String>,
    pub thumbnail_url: Option<String>,
    pub duration_minutes: Option<i32>,
    pub content_html: Option<String>,
    pub is_free: Option<bool>,
    pub is_preview: Option<bool>,
    pub is_published: Option<bool>,
    pub sort_order: Option<i32>,
    pub drip_days: Option<i32>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateLessonRequest {
    pub title: Option<String>,
    pub slug: Option<String>,
    pub description: Option<String>,
    pub module_id: Option<i64>,
    pub video_url: Option<String>,
    pub bunny_video_guid: Option<String>,
    pub thumbnail_url: Option<String>,
    pub duration_minutes: Option<i32>,
    pub content_html: Option<String>,
    pub is_free: Option<bool>,
    pub is_preview: Option<bool>,
    pub is_published: Option<bool>,
    pub sort_order: Option<i32>,
    pub drip_days: Option<i32>,
}

#[derive(Debug, Deserialize)]
pub struct CreateDownloadRequest {
    pub title: String,
    pub description: Option<String>,
    pub file_name: String,
    pub file_path: String,
    pub file_size_bytes: Option<i64>,
    pub file_type: Option<String>,
    pub mime_type: Option<String>,
    pub download_url: Option<String>,
    pub category: Option<String>,
    pub sort_order: Option<i32>,
    pub is_public: Option<bool>,
    pub course_id: Option<Uuid>,
    pub module_id: Option<i64>,
    pub lesson_id: Option<Uuid>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateDownloadRequest {
    pub title: Option<String>,
    pub description: Option<String>,
    pub category: Option<String>,
    pub sort_order: Option<i32>,
    pub is_public: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct ReorderRequest {
    pub items: Vec<ReorderItem>,
}

#[derive(Debug, Deserialize)]
pub struct ReorderItem {
    pub id: String,
    pub sort_order: i32,
}

#[derive(Debug, Deserialize)]
pub struct UpdateProgressRequest {
    pub lesson_id: Uuid,
    pub video_position_seconds: Option<i32>,
    pub video_duration_seconds: Option<i32>,
    pub is_completed: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct CreateReviewRequest {
    pub rating: i16,
    pub title: Option<String>,
    pub content: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct CourseQueryParams {
    pub status: Option<String>,
    pub level: Option<String>,
    pub is_free: Option<bool>,
    pub search: Option<String>,
    pub page: Option<i32>,
    pub per_page: Option<i32>,
    pub sort_by: Option<String>,
    pub sort_order: Option<String>,
}

// ═══════════════════════════════════════════════════════════════════════════════════
// RESPONSE STRUCTURES
// ═══════════════════════════════════════════════════════════════════════════════════

#[derive(Debug, Serialize)]
pub struct CourseWithContent {
    #[serde(flatten)]
    pub course: Course,
    pub modules: Vec<ModuleWithLessons>,
    pub downloads: Vec<CourseDownload>,
}

#[derive(Debug, Serialize)]
pub struct ModuleWithLessons {
    #[serde(flatten)]
    pub module: CourseModule,
    pub lessons: Vec<LessonListItem>,
}

#[derive(Debug, Serialize)]
pub struct CoursePlayerData {
    pub course: Course,
    pub modules: Vec<ModuleWithLessons>,
    pub downloads: Vec<CourseDownload>,
    pub enrollment: Option<UserCourseEnrollment>,
    pub progress: Vec<UserLessonProgress>,
    pub current_lesson: Option<Lesson>,
}

#[derive(Debug, Serialize)]
pub struct PaginatedCourses {
    pub courses: Vec<CourseListItem>,
    pub total: i64,
    pub page: i32,
    pub per_page: i32,
    pub total_pages: i32,
}

// ═══════════════════════════════════════════════════════════════════════════════════
// QUIZ/ASSESSMENT MODELS - ICT 7 Grade
// ═══════════════════════════════════════════════════════════════════════════════════

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Quiz {
    pub id: i64,
    pub course_id: Uuid,
    pub lesson_id: Option<Uuid>,
    pub module_id: Option<i64>,
    pub title: String,
    pub description: Option<String>,
    pub quiz_type: String, // "graded", "practice", "survey"
    pub passing_score: Option<i32>,
    pub time_limit_minutes: Option<i32>,
    pub max_attempts: Option<i32>,
    pub shuffle_questions: Option<bool>,
    pub shuffle_answers: Option<bool>,
    pub show_correct_answers: Option<bool>,
    pub is_required: Option<bool>,
    pub is_published: bool,
    pub sort_order: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct QuizQuestion {
    pub id: i64,
    pub quiz_id: i64,
    pub question_type: String, // "multiple_choice", "true_false", "short_answer", "essay"
    pub question_text: String,
    pub question_html: Option<String>,
    pub explanation: Option<String>,
    pub points: i32,
    pub sort_order: i32,
    pub is_required: Option<bool>,
    pub media_url: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct QuizAnswer {
    pub id: i64,
    pub question_id: i64,
    pub answer_text: String,
    pub is_correct: bool,
    pub sort_order: i32,
    pub feedback: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct UserQuizAttempt {
    pub id: i64,
    pub user_id: i64,
    pub quiz_id: i64,
    pub enrollment_id: i64,
    pub score: Option<i32>,
    pub max_score: i32,
    pub score_percent: Option<f64>,
    pub passed: Option<bool>,
    pub started_at: NaiveDateTime,
    pub completed_at: Option<NaiveDateTime>,
    pub time_spent_seconds: Option<i32>,
    pub attempt_number: i32,
    pub answers: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuizWithQuestions {
    #[serde(flatten)]
    pub quiz: Quiz,
    pub questions: Vec<QuestionWithAnswers>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuestionWithAnswers {
    #[serde(flatten)]
    pub question: QuizQuestion,
    pub answers: Vec<QuizAnswer>,
}

#[derive(Debug, Deserialize)]
pub struct CreateQuizRequest {
    pub title: String,
    pub description: Option<String>,
    pub lesson_id: Option<Uuid>,
    pub module_id: Option<i64>,
    pub quiz_type: Option<String>,
    pub passing_score: Option<i32>,
    pub time_limit_minutes: Option<i32>,
    pub max_attempts: Option<i32>,
    pub shuffle_questions: Option<bool>,
    pub shuffle_answers: Option<bool>,
    pub show_correct_answers: Option<bool>,
    pub is_required: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateQuizRequest {
    pub title: Option<String>,
    pub description: Option<String>,
    pub lesson_id: Option<Uuid>,
    pub module_id: Option<i64>,
    pub quiz_type: Option<String>,
    pub passing_score: Option<i32>,
    pub time_limit_minutes: Option<i32>,
    pub max_attempts: Option<i32>,
    pub shuffle_questions: Option<bool>,
    pub shuffle_answers: Option<bool>,
    pub show_correct_answers: Option<bool>,
    pub is_required: Option<bool>,
    pub is_published: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct CreateQuestionRequest {
    pub question_type: String,
    pub question_text: String,
    pub question_html: Option<String>,
    pub explanation: Option<String>,
    pub points: Option<i32>,
    pub media_url: Option<String>,
    pub answers: Option<Vec<CreateAnswerRequest>>,
}

#[derive(Debug, Deserialize)]
pub struct CreateAnswerRequest {
    pub answer_text: String,
    pub is_correct: bool,
    pub feedback: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct SubmitQuizRequest {
    pub answers: Vec<SubmitAnswerRequest>,
}

#[derive(Debug, Deserialize)]
pub struct SubmitAnswerRequest {
    pub question_id: i64,
    pub answer_id: Option<i64>,
    pub answer_text: Option<String>,
}

// ═══════════════════════════════════════════════════════════════════════════════════
// COURSE CATEGORY/TAG MODELS - ICT 7 Grade
// ═══════════════════════════════════════════════════════════════════════════════════

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct CourseCategory {
    pub id: i64,
    pub name: String,
    pub slug: String,
    pub description: Option<String>,
    pub icon: Option<String>,
    pub color: Option<String>,
    pub parent_id: Option<i64>,
    pub sort_order: i32,
    pub is_featured: Option<bool>,
    pub course_count: Option<i32>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct CourseTag {
    pub id: i64,
    pub name: String,
    pub slug: String,
    pub color: Option<String>,
    pub course_count: Option<i32>,
    pub created_at: NaiveDateTime,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct CourseCategoryMapping {
    pub course_id: Uuid,
    pub category_id: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct CourseTagMapping {
    pub course_id: Uuid,
    pub tag_id: i64,
}

#[derive(Debug, Deserialize)]
pub struct CreateCategoryRequest {
    pub name: String,
    pub slug: Option<String>,
    pub description: Option<String>,
    pub icon: Option<String>,
    pub color: Option<String>,
    pub parent_id: Option<i64>,
    pub is_featured: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct CreateCourseTagRequest {
    pub name: String,
    pub slug: Option<String>,
    pub color: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateCategoriesRequest {
    pub category_ids: Vec<i64>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateTagsRequest {
    pub tag_ids: Vec<i64>,
}

// ═══════════════════════════════════════════════════════════════════════════════════
// PREREQUISITES REQUEST DTOs - ICT 7 Grade
// ═══════════════════════════════════════════════════════════════════════════════════

#[derive(Debug, Deserialize)]
pub struct UpdatePrerequisitesRequest {
    pub prerequisite_lesson_ids: Vec<Uuid>,
}

#[derive(Debug, Deserialize)]
pub struct CheckPrerequisitesRequest {
    pub lesson_id: Uuid,
}

#[derive(Debug, Serialize)]
pub struct PrerequisitesStatus {
    pub lesson_id: Uuid,
    pub can_access: bool,
    pub missing_prerequisites: Vec<PrerequisiteInfo>,
}

#[derive(Debug, Serialize)]
pub struct PrerequisiteInfo {
    pub lesson_id: Uuid,
    pub lesson_title: String,
    pub is_completed: bool,
}

// ═══════════════════════════════════════════════════════════════════════════════════
// REVIEW REQUEST DTOs - ICT 7 Grade
// ═══════════════════════════════════════════════════════════════════════════════════

#[derive(Debug, Deserialize)]
pub struct UpdateReviewRequest {
    pub rating: Option<i16>,
    pub title: Option<String>,
    pub content: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct ReviewHelpfulRequest {
    pub helpful: bool,
}

#[derive(Debug, Serialize)]
pub struct CourseReviewSummary {
    pub course_id: Uuid,
    pub avg_rating: f64,
    pub total_reviews: i32,
    pub rating_distribution: RatingDistribution,
    pub recent_reviews: Vec<ReviewWithUser>,
}

#[derive(Debug, Serialize)]
pub struct RatingDistribution {
    pub five_star: i32,
    pub four_star: i32,
    pub three_star: i32,
    pub two_star: i32,
    pub one_star: i32,
}

// ═══════════════════════════════════════════════════════════════════════════════════
// ENROLLMENT STATS - ICT 7 Grade
// ═══════════════════════════════════════════════════════════════════════════════════

#[derive(Debug, Serialize)]
pub struct CourseEnrollmentStats {
    pub course_id: Uuid,
    pub total_enrollments: i64,
    pub active_enrollments: i64,
    pub completed_enrollments: i64,
    pub avg_progress: f64,
    pub avg_completion_time_days: Option<f64>,
    pub certificates_issued: i64,
}
