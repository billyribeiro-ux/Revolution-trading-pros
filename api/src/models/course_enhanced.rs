//! Enhanced Course Models - Revolution Trading Pros
//! Apple Principal Engineer ICT 7 Grade - January 2026
//!
//! Complete course management: sections, lessons, resources, live sessions, progress

use chrono::{DateTime, NaiveDate, NaiveTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

// ═══════════════════════════════════════════════════════════════════════════════════
// COURSE MODELS
// ═══════════════════════════════════════════════════════════════════════════════════

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct CourseEnhanced {
    pub id: i64,
    pub title: String,
    pub slug: String,
    pub subtitle: Option<String>,
    pub description: Option<String>,
    pub description_html: Option<String>,
    pub thumbnail_url: Option<String>,
    pub trailer_video_url: Option<String>,
    pub trailer_bunny_guid: Option<String>,
    pub difficulty_level: String,
    pub category: Option<String>,
    pub tags: Option<serde_json::Value>,
    pub instructor_id: Option<i64>,
    pub estimated_duration_minutes: i32,
    pub total_lessons: i32,
    pub total_sections: i32,
    pub is_published: bool,
    pub is_featured: bool,
    pub is_free: bool,
    pub required_plan_id: Option<i64>,
    pub price_cents: Option<i32>,
    pub prerequisite_course_ids: Option<serde_json::Value>,
    pub certificate_enabled: bool,
    pub certificate_template: String,
    pub completion_threshold_percent: i32,
    pub meta_title: Option<String>,
    pub meta_description: Option<String>,
    pub created_by: Option<i64>,
    pub updated_by: Option<i64>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub published_at: Option<DateTime<Utc>>,
    pub deleted_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct CourseSection {
    pub id: i64,
    pub course_id: i64,
    pub title: String,
    pub description: Option<String>,
    pub sort_order: i32,
    pub section_type: String,
    pub unlock_type: String,
    pub unlock_after_section_id: Option<i64>,
    pub unlock_date: Option<DateTime<Utc>>,
    pub is_published: bool,
    pub estimated_duration_minutes: i32,
    pub lesson_count: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct CourseLesson {
    pub id: i64,
    pub course_id: i64,
    pub section_id: i64,
    pub title: String,
    pub description: Option<String>,
    pub content_html: Option<String>,
    pub video_url: Option<String>,
    pub video_platform: String,
    pub bunny_video_guid: Option<String>,
    pub bunny_library_id: Option<i64>,
    pub duration_seconds: Option<i32>,
    pub thumbnail_url: Option<String>,
    pub sort_order: i32,
    pub lesson_type: String,
    pub is_preview: bool,
    pub is_published: bool,
    pub completion_type: String,
    pub required_watch_percent: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct CourseResource {
    pub id: i64,
    pub course_id: i64,
    pub section_id: Option<i64>,
    pub lesson_id: Option<i64>,
    pub title: String,
    pub description: Option<String>,
    pub file_url: String,
    pub file_name: String,
    pub file_type: Option<String>,
    pub file_size_bytes: Option<i64>,
    pub sort_order: i32,
    pub version: String,
    pub download_count: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct CourseLiveSession {
    pub id: i64,
    pub course_id: i64,
    pub section_id: Option<i64>,
    pub title: String,
    pub description: Option<String>,
    pub session_date: NaiveDate,
    pub session_time: Option<NaiveTime>,
    pub video_url: Option<String>,
    pub bunny_video_guid: Option<String>,
    pub bunny_library_id: Option<i64>,
    pub duration_seconds: Option<i32>,
    pub thumbnail_url: Option<String>,
    pub replay_available_until: Option<DateTime<Utc>>,
    pub is_published: bool,
    pub sort_order: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct UserCourseEnrollment {
    pub id: i64,
    pub user_id: i64,
    pub course_id: i64,
    pub enrolled_at: DateTime<Utc>,
    pub enrollment_source: Option<String>,
    pub access_expires_at: Option<DateTime<Utc>>,
    pub is_active: bool,
    pub completed_lessons: i32,
    pub total_lessons: i32,
    pub progress_percent: i32,
    pub is_completed: bool,
    pub completed_at: Option<DateTime<Utc>>,
    pub certificate_issued: bool,
    pub certificate_url: Option<String>,
    pub last_lesson_id: Option<i64>,
    pub last_position_seconds: i32,
    pub last_activity_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct UserLessonProgress {
    pub id: i64,
    pub user_id: i64,
    pub course_id: i64,
    pub lesson_id: i64,
    pub watch_position_seconds: i32,
    pub watch_time_total_seconds: i32,
    pub watch_percent: i32,
    pub is_completed: bool,
    pub completed_at: Option<DateTime<Utc>>,
    pub notes: Option<String>,
    pub bookmarked: bool,
    pub first_watched_at: DateTime<Utc>,
    pub last_watched_at: DateTime<Utc>,
}

// ═══════════════════════════════════════════════════════════════════════════════════
// REQUEST DTOs
// ═══════════════════════════════════════════════════════════════════════════════════

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateCourseRequest {
    pub title: String,
    pub subtitle: Option<String>,
    pub description: Option<String>,
    pub description_html: Option<String>,
    pub thumbnail_url: Option<String>,
    pub trailer_video_url: Option<String>,
    pub difficulty_level: Option<String>,
    pub category: Option<String>,
    pub tags: Option<Vec<String>>,
    pub instructor_id: Option<i64>,
    pub is_published: Option<bool>,
    pub is_featured: Option<bool>,
    pub is_free: Option<bool>,
    pub required_plan_id: Option<i64>,
    pub price_cents: Option<i32>,
    pub prerequisite_course_ids: Option<Vec<i64>>,
    pub certificate_enabled: Option<bool>,
    pub completion_threshold_percent: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateCourseRequest {
    pub title: Option<String>,
    pub subtitle: Option<String>,
    pub description: Option<String>,
    pub description_html: Option<String>,
    pub thumbnail_url: Option<String>,
    pub trailer_video_url: Option<String>,
    pub difficulty_level: Option<String>,
    pub category: Option<String>,
    pub tags: Option<Vec<String>>,
    pub instructor_id: Option<i64>,
    pub is_published: Option<bool>,
    pub is_featured: Option<bool>,
    pub is_free: Option<bool>,
    pub required_plan_id: Option<i64>,
    pub price_cents: Option<i32>,
    pub prerequisite_course_ids: Option<Vec<i64>>,
    pub certificate_enabled: Option<bool>,
    pub completion_threshold_percent: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateSectionRequest {
    pub title: String,
    pub description: Option<String>,
    pub section_type: Option<String>,
    pub unlock_type: Option<String>,
    pub unlock_after_section_id: Option<i64>,
    pub unlock_date: Option<String>,
    pub is_published: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateSectionRequest {
    pub title: Option<String>,
    pub description: Option<String>,
    pub section_type: Option<String>,
    pub unlock_type: Option<String>,
    pub unlock_after_section_id: Option<i64>,
    pub unlock_date: Option<String>,
    pub is_published: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateLessonRequest {
    pub section_id: i64,
    pub title: String,
    pub description: Option<String>,
    pub content_html: Option<String>,
    pub video_url: Option<String>,
    pub bunny_video_guid: Option<String>,
    pub thumbnail_url: Option<String>,
    pub lesson_type: Option<String>,
    pub is_preview: Option<bool>,
    pub is_published: Option<bool>,
    pub completion_type: Option<String>,
    pub required_watch_percent: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateLessonRequest {
    pub section_id: Option<i64>,
    pub title: Option<String>,
    pub description: Option<String>,
    pub content_html: Option<String>,
    pub video_url: Option<String>,
    pub bunny_video_guid: Option<String>,
    pub thumbnail_url: Option<String>,
    pub duration_seconds: Option<i32>,
    pub lesson_type: Option<String>,
    pub is_preview: Option<bool>,
    pub is_published: Option<bool>,
    pub completion_type: Option<String>,
    pub required_watch_percent: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateResourceRequest {
    pub section_id: Option<i64>,
    pub lesson_id: Option<i64>,
    pub title: String,
    pub description: Option<String>,
    pub file_url: String,
    pub file_name: String,
    pub file_type: Option<String>,
    pub file_size_bytes: Option<i64>,
    pub version: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateLiveSessionRequest {
    pub section_id: Option<i64>,
    pub title: String,
    pub description: Option<String>,
    pub session_date: String,
    pub session_time: Option<String>,
    pub video_url: Option<String>,
    pub bunny_video_guid: Option<String>,
    pub thumbnail_url: Option<String>,
    pub replay_available_until: Option<String>,
    pub is_published: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BulkLiveSessionsRequest {
    pub section_id: Option<i64>,
    pub sessions: Vec<LiveSessionInput>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LiveSessionInput {
    pub title: String,
    pub session_date: String,
    pub video_url: Option<String>,
    pub bunny_video_guid: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReorderItemsRequest {
    pub items: Vec<ReorderItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReorderItem {
    pub id: i64,
    pub sort_order: i32,
}

// ═══════════════════════════════════════════════════════════════════════════════════
// RESPONSE DTOs
// ═══════════════════════════════════════════════════════════════════════════════════

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CourseResponse {
    pub id: i64,
    pub title: String,
    pub slug: String,
    pub subtitle: Option<String>,
    pub description: Option<String>,
    pub description_html: Option<String>,
    pub thumbnail_url: Option<String>,
    pub trailer_video_url: Option<String>,
    pub difficulty_level: String,
    pub category: Option<String>,
    pub tags: Vec<String>,
    pub instructor: Option<InstructorInfo>,
    pub estimated_duration_minutes: i32,
    pub formatted_duration: String,
    pub total_lessons: i32,
    pub total_sections: i32,
    pub is_published: bool,
    pub is_featured: bool,
    pub is_free: bool,
    pub price_cents: Option<i32>,
    pub certificate_enabled: bool,
    pub sections: Option<Vec<SectionResponse>>,
    pub created_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InstructorInfo {
    pub id: i64,
    pub name: String,
    pub slug: String,
    pub avatar_url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SectionResponse {
    pub id: i64,
    pub title: String,
    pub description: Option<String>,
    pub sort_order: i32,
    pub section_type: String,
    pub unlock_type: String,
    pub is_published: bool,
    pub lesson_count: i32,
    pub estimated_duration_minutes: i32,
    pub lessons: Option<Vec<LessonResponse>>,
    pub resources: Option<Vec<ResourceResponse>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LessonResponse {
    pub id: i64,
    pub title: String,
    pub description: Option<String>,
    pub video_url: Option<String>,
    pub embed_url: Option<String>,
    pub thumbnail_url: Option<String>,
    pub duration_seconds: Option<i32>,
    pub formatted_duration: Option<String>,
    pub sort_order: i32,
    pub lesson_type: String,
    pub is_preview: bool,
    pub is_published: bool,
    pub resources: Option<Vec<ResourceResponse>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceResponse {
    pub id: i64,
    pub title: String,
    pub description: Option<String>,
    pub file_url: String,
    pub file_name: String,
    pub file_type: Option<String>,
    pub file_size_bytes: Option<i64>,
    pub formatted_size: String,
    pub version: String,
    pub download_count: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LiveSessionResponse {
    pub id: i64,
    pub title: String,
    pub description: Option<String>,
    pub session_date: String,
    pub session_time: Option<String>,
    pub formatted_date: String,
    pub video_url: Option<String>,
    pub embed_url: Option<String>,
    pub thumbnail_url: Option<String>,
    pub duration_seconds: Option<i32>,
    pub formatted_duration: Option<String>,
    pub is_published: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CourseProgressResponse {
    pub course_id: i64,
    pub course_title: String,
    pub enrolled_at: String,
    pub completed_lessons: i32,
    pub total_lessons: i32,
    pub progress_percent: i32,
    pub is_completed: bool,
    pub completed_at: Option<String>,
    pub certificate_url: Option<String>,
    pub last_lesson: Option<LessonProgressInfo>,
    pub lesson_progress: Vec<LessonProgressInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LessonProgressInfo {
    pub lesson_id: i64,
    pub lesson_title: String,
    pub watch_percent: i32,
    pub is_completed: bool,
    pub last_position_seconds: i32,
}

#[derive(Debug, Deserialize)]
pub struct CourseListQuery {
    pub page: Option<i64>,
    pub per_page: Option<i64>,
    pub difficulty_level: Option<String>,
    pub category: Option<String>,
    pub instructor_id: Option<i64>,
    pub is_published: Option<bool>,
    pub is_featured: Option<bool>,
    pub search: Option<String>,
}

// ═══════════════════════════════════════════════════════════════════════════════════
// HELPER FUNCTIONS
// ═══════════════════════════════════════════════════════════════════════════════════

pub fn format_duration_minutes(minutes: i32) -> String {
    let hours = minutes / 60;
    let mins = minutes % 60;
    if hours > 0 {
        format!("{}h {}m", hours, mins)
    } else {
        format!("{}m", mins)
    }
}

pub fn format_duration_seconds(seconds: i32) -> String {
    let hours = seconds / 3600;
    let minutes = (seconds % 3600) / 60;
    let secs = seconds % 60;
    if hours > 0 {
        format!("{}:{:02}:{:02}", hours, minutes, secs)
    } else {
        format!("{}:{:02}", minutes, secs)
    }
}

pub fn format_file_size(bytes: i64) -> String {
    if bytes < 1024 {
        format!("{} B", bytes)
    } else if bytes < 1024 * 1024 {
        format!("{:.1} KB", bytes as f64 / 1024.0)
    } else if bytes < 1024 * 1024 * 1024 {
        format!("{:.1} MB", bytes as f64 / (1024.0 * 1024.0))
    } else {
        format!("{:.2} GB", bytes as f64 / (1024.0 * 1024.0 * 1024.0))
    }
}

pub fn get_embed_url(
    video_url: &Option<String>,
    bunny_guid: &Option<String>,
    bunny_library: Option<i64>,
) -> Option<String> {
    if let (Some(guid), Some(lib_id)) = (bunny_guid, bunny_library) {
        return Some(format!(
            "https://iframe.mediadelivery.net/embed/{}/{}",
            lib_id, guid
        ));
    }
    video_url.clone()
}
