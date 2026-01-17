//! Advanced Video Models - Revolution Trading Pros
//! Apple Principal Engineer ICT 7 Grade - January 2026
//!
//! Models for: Analytics, Series, Chapters, Transcriptions, Scheduled Publishing

use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

// ═══════════════════════════════════════════════════════════════════════════════════
// VIDEO ANALYTICS
// ═══════════════════════════════════════════════════════════════════════════════════

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalyticsEventRequest {
    pub video_id: i64,
    pub session_id: Option<String>,
    pub event_type: String,
    pub event_data: Option<serde_json::Value>,
    pub watch_time_seconds: Option<i32>,
    pub progress_percent: Option<i16>,
    pub buffer_count: Option<i32>,
    pub quality_level: Option<String>,
    pub playback_speed: Option<f64>,
    pub device_type: Option<String>,
    pub browser: Option<String>,
    pub os: Option<String>,
    pub screen_width: Option<i32>,
    pub screen_height: Option<i32>,
    pub connection_type: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchAnalyticsRequest {
    pub events: Vec<AnalyticsEventRequest>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct VideoAnalyticsEvent {
    pub id: i64,
    pub video_id: i64,
    pub user_id: Option<i64>,
    pub session_id: Option<String>,
    pub event_type: String,
    pub event_data: Option<serde_json::Value>,
    pub watch_time_seconds: i32,
    pub progress_percent: Option<i16>,
    pub buffer_count: i32,
    pub quality_level: Option<String>,
    pub playback_speed: Option<f64>,
    pub device_type: Option<String>,
    pub browser: Option<String>,
    pub os: Option<String>,
    pub screen_width: Option<i32>,
    pub screen_height: Option<i32>,
    pub connection_type: Option<String>,
    pub country_code: Option<String>,
    pub region_code: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct VideoAnalyticsDaily {
    pub id: i64,
    pub video_id: i64,
    pub analytics_date: NaiveDate,
    pub unique_viewers: i32,
    pub total_views: i32,
    pub total_watch_time_seconds: i64,
    pub avg_watch_time_seconds: i32,
    pub avg_completion_percent: i16,
    pub completed_count: i32,
    pub avg_buffer_count: Option<f64>,
    pub device_breakdown: Option<serde_json::Value>,
    pub quality_breakdown: Option<serde_json::Value>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct VideoWatchHistory {
    pub id: i64,
    pub user_id: i64,
    pub video_id: i64,
    pub last_position_seconds: i32,
    pub total_watch_time_seconds: i32,
    pub completion_percent: i16,
    pub is_completed: bool,
    pub completed_at: Option<DateTime<Utc>>,
    pub rating: Option<i16>,
    pub is_bookmarked: bool,
    pub notes: Option<String>,
    pub first_watched_at: DateTime<Utc>,
    pub last_watched_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VideoAnalyticsSummary {
    pub video_id: i64,
    pub title: String,
    pub total_views: i64,
    pub unique_viewers: i64,
    pub total_watch_time_hours: f64,
    pub avg_watch_time_minutes: f64,
    pub avg_completion_percent: i16,
    pub completion_rate: f64,
    pub peak_concurrent_viewers: i32,
    pub device_breakdown: serde_json::Value,
    pub quality_breakdown: serde_json::Value,
    pub daily_views: Vec<DailyViewCount>,
    pub top_drop_off_points: Vec<DropOffPoint>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DailyViewCount {
    pub date: String,
    pub views: i64,
    pub unique_viewers: i64,
    pub watch_time_minutes: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DropOffPoint {
    pub timestamp_seconds: i32,
    pub drop_off_count: i32,
    pub drop_off_percent: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalyticsDashboard {
    pub period: String,
    pub total_videos: i64,
    pub total_views: i64,
    pub total_watch_time_hours: f64,
    pub avg_completion_rate: f64,
    pub new_viewers: i64,
    pub returning_viewers: i64,
    pub top_videos: Vec<TopVideoStats>,
    pub views_by_day: Vec<DailyViewCount>,
    pub views_by_content_type: serde_json::Value,
    pub device_breakdown: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TopVideoStats {
    pub video_id: i64,
    pub title: String,
    pub thumbnail_url: Option<String>,
    pub views: i64,
    pub watch_time_hours: f64,
    pub completion_rate: f64,
}

#[derive(Debug, Deserialize)]
pub struct AnalyticsDashboardQuery {
    pub period: Option<String>, // 7d, 30d, 90d, custom
    pub start_date: Option<String>,
    pub end_date: Option<String>,
    pub content_type: Option<String>,
    pub room_id: Option<i64>,
}

#[derive(Debug, Deserialize)]
pub struct VideoAnalyticsQuery {
    pub period: Option<String>,
    pub start_date: Option<String>,
    pub end_date: Option<String>,
}

// ═══════════════════════════════════════════════════════════════════════════════════
// VIDEO SERIES / PLAYLISTS
// ═══════════════════════════════════════════════════════════════════════════════════

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct VideoSeries {
    pub id: i64,
    pub title: String,
    pub slug: String,
    pub description: Option<String>,
    pub thumbnail_url: Option<String>,
    pub content_type: String,
    pub difficulty_level: Option<String>,
    pub category: Option<String>,
    pub is_published: bool,
    pub is_premium: bool,
    pub required_plan_id: Option<i64>,
    pub sort_order: i32,
    pub estimated_duration_minutes: Option<i32>,
    pub video_count: i32,
    pub tags: Option<serde_json::Value>,
    pub metadata: Option<serde_json::Value>,
    pub created_by: Option<i64>,
    pub updated_by: Option<i64>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct VideoSeriesItem {
    pub id: i64,
    pub series_id: i64,
    pub video_id: i64,
    pub sort_order: i32,
    pub section_title: Option<String>,
    pub section_order: i32,
    pub custom_title: Option<String>,
    pub is_preview: bool,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateSeriesRequest {
    pub title: String,
    pub description: Option<String>,
    pub thumbnail_url: Option<String>,
    pub content_type: String,
    pub difficulty_level: Option<String>,
    pub category: Option<String>,
    pub is_published: Option<bool>,
    pub is_premium: Option<bool>,
    pub required_plan_id: Option<i64>,
    pub tags: Option<Vec<String>>,
    pub video_ids: Option<Vec<i64>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateSeriesRequest {
    pub title: Option<String>,
    pub description: Option<String>,
    pub thumbnail_url: Option<String>,
    pub content_type: Option<String>,
    pub difficulty_level: Option<String>,
    pub category: Option<String>,
    pub is_published: Option<bool>,
    pub is_premium: Option<bool>,
    pub required_plan_id: Option<i64>,
    pub tags: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddVideosToSeriesRequest {
    pub video_ids: Vec<i64>,
    pub section_title: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReorderSeriesVideosRequest {
    pub video_orders: Vec<VideoOrderItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VideoOrderItem {
    pub video_id: i64,
    pub sort_order: i32,
    pub section_title: Option<String>,
    pub section_order: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SeriesResponse {
    pub id: i64,
    pub title: String,
    pub slug: String,
    pub description: Option<String>,
    pub thumbnail_url: Option<String>,
    pub content_type: String,
    pub difficulty_level: Option<String>,
    pub is_published: bool,
    pub is_premium: bool,
    pub video_count: i32,
    pub estimated_duration_minutes: Option<i32>,
    pub tags: Vec<String>,
    pub videos: Option<Vec<SeriesVideoItem>>,
    pub progress: Option<UserSeriesProgressInfo>,
    pub created_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SeriesVideoItem {
    pub video_id: i64,
    pub title: String,
    pub thumbnail_url: Option<String>,
    pub duration: Option<i32>,
    pub sort_order: i32,
    pub section_title: Option<String>,
    pub is_preview: bool,
    pub is_completed: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserSeriesProgressInfo {
    pub completed_videos: i32,
    pub total_videos: i32,
    pub completion_percent: i16,
    pub current_video_id: Option<i64>,
    pub last_position_seconds: i32,
    pub is_completed: bool,
}

#[derive(Debug, Deserialize)]
pub struct SeriesListQuery {
    pub page: Option<i64>,
    pub per_page: Option<i64>,
    pub content_type: Option<String>,
    pub difficulty_level: Option<String>,
    pub is_published: Option<bool>,
    pub search: Option<String>,
}

// ═══════════════════════════════════════════════════════════════════════════════════
// VIDEO CHAPTERS
// ═══════════════════════════════════════════════════════════════════════════════════

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct VideoChapter {
    pub id: i64,
    pub video_id: i64,
    pub title: String,
    pub description: Option<String>,
    pub start_time_seconds: i32,
    pub end_time_seconds: Option<i32>,
    pub thumbnail_url: Option<String>,
    pub thumbnail_time_seconds: Option<i32>,
    pub chapter_number: i32,
    pub tags: Option<serde_json::Value>,
    pub metadata: Option<serde_json::Value>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateChapterRequest {
    pub title: String,
    pub description: Option<String>,
    pub start_time_seconds: i32,
    pub end_time_seconds: Option<i32>,
    pub thumbnail_url: Option<String>,
    pub thumbnail_time_seconds: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateChapterRequest {
    pub title: Option<String>,
    pub description: Option<String>,
    pub start_time_seconds: Option<i32>,
    pub end_time_seconds: Option<i32>,
    pub thumbnail_url: Option<String>,
    pub thumbnail_time_seconds: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BulkChaptersRequest {
    pub chapters: Vec<ChapterInput>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChapterInput {
    pub title: String,
    pub description: Option<String>,
    pub start_time_seconds: i32,
    pub end_time_seconds: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChapterResponse {
    pub id: i64,
    pub title: String,
    pub description: Option<String>,
    pub start_time_seconds: i32,
    pub end_time_seconds: Option<i32>,
    pub formatted_start_time: String,
    pub formatted_end_time: Option<String>,
    pub thumbnail_url: Option<String>,
    pub chapter_number: i32,
}

// ═══════════════════════════════════════════════════════════════════════════════════
// VIDEO TRANSCRIPTIONS
// ═══════════════════════════════════════════════════════════════════════════════════

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct VideoTranscription {
    pub id: i64,
    pub video_id: i64,
    pub language_code: String,
    pub language_name: String,
    pub transcription_type: String,
    pub full_text: Option<String>,
    pub segments: Option<serde_json::Value>,
    pub vtt_content: Option<String>,
    pub srt_content: Option<String>,
    pub summary: Option<String>,
    pub key_topics: Option<serde_json::Value>,
    pub sentiment_analysis: Option<serde_json::Value>,
    pub status: String,
    pub error_message: Option<String>,
    pub provider: Option<String>,
    pub provider_job_id: Option<String>,
    pub confidence_score: Option<f64>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub completed_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateTranscriptionRequest {
    pub language_code: Option<String>,
    pub provider: Option<String>, // openai, assemblyai, deepgram
    pub auto_generate_captions: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TranscriptionSegment {
    pub start: f64,
    pub end: f64,
    pub text: String,
    pub confidence: Option<f64>,
    pub speaker: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TranscriptionResponse {
    pub id: i64,
    pub video_id: i64,
    pub language_code: String,
    pub language_name: String,
    pub transcription_type: String,
    pub status: String,
    pub full_text: Option<String>,
    pub segments: Option<Vec<TranscriptionSegment>>,
    pub summary: Option<String>,
    pub key_topics: Vec<String>,
    pub vtt_url: Option<String>,
    pub srt_url: Option<String>,
    pub confidence_score: Option<f64>,
    pub created_at: String,
    pub completed_at: Option<String>,
}

// ═══════════════════════════════════════════════════════════════════════════════════
// SCHEDULED PUBLISHING
// ═══════════════════════════════════════════════════════════════════════════════════

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct ScheduledPublishJob {
    pub id: i64,
    pub resource_type: String,
    pub resource_id: i64,
    pub scheduled_at: DateTime<Utc>,
    pub timezone: String,
    pub action: String,
    pub action_data: Option<serde_json::Value>,
    pub status: String,
    pub error_message: Option<String>,
    pub processed_at: Option<DateTime<Utc>>,
    pub notify_on_publish: bool,
    pub notification_recipients: Option<serde_json::Value>,
    pub created_by: Option<i64>,
    pub cancelled_by: Option<i64>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateScheduledJobRequest {
    pub resource_type: String, // video, series
    pub resource_id: i64,
    pub scheduled_at: String, // ISO 8601 datetime
    pub timezone: Option<String>,
    pub action: String, // publish, unpublish, feature, unfeature
    pub notify_on_publish: Option<bool>,
    pub notification_recipients: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateScheduledJobRequest {
    pub scheduled_at: Option<String>,
    pub timezone: Option<String>,
    pub action: Option<String>,
    pub notify_on_publish: Option<bool>,
    pub notification_recipients: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScheduledJobResponse {
    pub id: i64,
    pub resource_type: String,
    pub resource_id: i64,
    pub resource_title: Option<String>,
    pub scheduled_at: String,
    pub timezone: String,
    pub action: String,
    pub status: String,
    pub notify_on_publish: bool,
    pub created_at: String,
}

#[derive(Debug, Deserialize)]
pub struct ScheduledJobsQuery {
    pub status: Option<String>,
    pub resource_type: Option<String>,
    pub page: Option<i64>,
    pub per_page: Option<i64>,
}

// ═══════════════════════════════════════════════════════════════════════════════════
// BUNNY.NET WEBHOOK
// ═══════════════════════════════════════════════════════════════════════════════════

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct BunnyWebhookEvent {
    pub id: i64,
    pub event_id: Option<String>,
    pub event_type: String,
    pub video_guid: String,
    pub library_id: Option<i64>,
    pub payload: serde_json::Value,
    pub status: String,
    pub processed_at: Option<DateTime<Utc>>,
    pub error_message: Option<String>,
    pub received_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BunnyWebhookPayload {
    #[serde(rename = "VideoGuid")]
    pub video_guid: String,
    #[serde(rename = "VideoLibraryId")]
    pub video_library_id: i64,
    #[serde(rename = "Status")]
    pub status: Option<i32>,
    #[serde(rename = "EncodeProgress")]
    pub encode_progress: Option<i32>,
    #[serde(rename = "Length")]
    pub length: Option<i32>,
    #[serde(rename = "Width")]
    pub width: Option<i32>,
    #[serde(rename = "Height")]
    pub height: Option<i32>,
    #[serde(rename = "ThumbnailUrl")]
    pub thumbnail_url: Option<String>,
    #[serde(rename = "Title")]
    pub title: Option<String>,
}

// ═══════════════════════════════════════════════════════════════════════════════════
// VIDEO UPLOAD QUEUE (Bulk Upload)
// ═══════════════════════════════════════════════════════════════════════════════════

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct VideoUploadQueue {
    pub id: i64,
    pub batch_id: Uuid,
    pub batch_order: i32,
    pub original_filename: String,
    pub file_size_bytes: Option<i64>,
    pub content_type: Option<String>,
    pub status: String,
    pub progress_percent: i16,
    pub error_message: Option<String>,
    pub bunny_video_guid: Option<String>,
    pub bunny_upload_url: Option<String>,
    pub video_metadata: serde_json::Value,
    pub created_video_id: Option<i64>,
    pub created_by: Option<i64>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub completed_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BulkUploadRequest {
    pub files: Vec<BulkUploadFileInfo>,
    pub default_metadata: BulkUploadDefaultMetadata,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BulkUploadFileInfo {
    pub filename: String,
    pub file_size_bytes: Option<i64>,
    pub content_type: Option<String>,
    pub title: Option<String>,
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BulkUploadDefaultMetadata {
    pub content_type: String,
    pub video_date: Option<String>,
    pub trader_id: Option<i64>,
    pub room_ids: Option<Vec<i64>>,
    pub upload_to_all: Option<bool>,
    pub is_published: Option<bool>,
    pub tags: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BulkUploadResponse {
    pub batch_id: String,
    pub uploads: Vec<UploadQueueItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UploadQueueItem {
    pub id: i64,
    pub filename: String,
    pub upload_url: String,
    pub video_guid: String,
    pub status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchStatusResponse {
    pub batch_id: String,
    pub total_files: i32,
    pub completed: i32,
    pub failed: i32,
    pub in_progress: i32,
    pub pending: i32,
    pub uploads: Vec<UploadStatusItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UploadStatusItem {
    pub id: i64,
    pub filename: String,
    pub status: String,
    pub progress_percent: i16,
    pub error_message: Option<String>,
    pub created_video_id: Option<i64>,
}

// ═══════════════════════════════════════════════════════════════════════════════════
// VIDEO CLONING
// ═══════════════════════════════════════════════════════════════════════════════════

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CloneVideoRequest {
    pub new_title: Option<String>,
    pub new_video_date: Option<String>,
    pub content_type: Option<String>,
    pub room_ids: Option<Vec<i64>>,
    pub upload_to_all: Option<bool>,
    pub include_chapters: Option<bool>,
    pub is_published: Option<bool>,
}

// ═══════════════════════════════════════════════════════════════════════════════════
// CSV EXPORT
// ═══════════════════════════════════════════════════════════════════════════════════

#[derive(Debug, Deserialize)]
pub struct ExportVideosQuery {
    pub content_type: Option<String>,
    pub room_id: Option<i64>,
    pub trader_id: Option<i64>,
    pub is_published: Option<bool>,
    pub start_date: Option<String>,
    pub end_date: Option<String>,
    pub format: Option<String>, // csv, xlsx
}

// ═══════════════════════════════════════════════════════════════════════════════════
// BULK EDIT
// ═══════════════════════════════════════════════════════════════════════════════════

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BulkEditRequest {
    pub video_ids: Vec<i64>,
    pub updates: BulkEditUpdates,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BulkEditUpdates {
    pub content_type: Option<String>,
    pub trader_id: Option<i64>,
    pub is_published: Option<bool>,
    pub is_featured: Option<bool>,
    pub difficulty_level: Option<String>,
    pub category: Option<String>,
    pub add_tags: Option<Vec<String>>,
    pub remove_tags: Option<Vec<String>>,
    pub add_room_ids: Option<Vec<i64>>,
    pub remove_room_ids: Option<Vec<i64>>,
}

// ═══════════════════════════════════════════════════════════════════════════════════
// DRAG & DROP REORDER
// ═══════════════════════════════════════════════════════════════════════════════════

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReorderVideosRequest {
    pub video_orders: Vec<VideoRoomOrder>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VideoRoomOrder {
    pub video_id: i64,
    pub sort_order: i32,
    pub is_pinned: Option<bool>,
}

// ═══════════════════════════════════════════════════════════════════════════════════
// HELPER FUNCTIONS
// ═══════════════════════════════════════════════════════════════════════════════════

pub fn format_duration(seconds: i32) -> String {
    let hours = seconds / 3600;
    let minutes = (seconds % 3600) / 60;
    let secs = seconds % 60;
    if hours > 0 {
        format!("{}:{:02}:{:02}", hours, minutes, secs)
    } else {
        format!("{}:{:02}", minutes, secs)
    }
}

pub fn parse_time_to_seconds(time_str: &str) -> Option<i32> {
    let parts: Vec<&str> = time_str.split(':').collect();
    match parts.len() {
        2 => {
            let mins: i32 = parts[0].parse().ok()?;
            let secs: i32 = parts[1].parse().ok()?;
            Some(mins * 60 + secs)
        }
        3 => {
            let hours: i32 = parts[0].parse().ok()?;
            let mins: i32 = parts[1].parse().ok()?;
            let secs: i32 = parts[2].parse().ok()?;
            Some(hours * 3600 + mins * 60 + secs)
        }
        _ => None,
    }
}

/// Generate WebVTT from segments
pub fn generate_vtt(segments: &[TranscriptionSegment]) -> String {
    let mut vtt = String::from("WEBVTT\n\n");
    for (i, segment) in segments.iter().enumerate() {
        let start = format_vtt_time(segment.start);
        let end = format_vtt_time(segment.end);
        vtt.push_str(&format!(
            "{}\n{} --> {}\n{}\n\n",
            i + 1,
            start,
            end,
            segment.text
        ));
    }
    vtt
}

/// Generate SRT from segments
pub fn generate_srt(segments: &[TranscriptionSegment]) -> String {
    let mut srt = String::new();
    for (i, segment) in segments.iter().enumerate() {
        let start = format_srt_time(segment.start);
        let end = format_srt_time(segment.end);
        srt.push_str(&format!(
            "{}\n{} --> {}\n{}\n\n",
            i + 1,
            start,
            end,
            segment.text
        ));
    }
    srt
}

fn format_vtt_time(seconds: f64) -> String {
    let total_secs = seconds as i32;
    let millis = ((seconds - total_secs as f64) * 1000.0) as i32;
    let hours = total_secs / 3600;
    let mins = (total_secs % 3600) / 60;
    let secs = total_secs % 60;
    format!("{:02}:{:02}:{:02}.{:03}", hours, mins, secs, millis)
}

fn format_srt_time(seconds: f64) -> String {
    let total_secs = seconds as i32;
    let millis = ((seconds - total_secs as f64) * 1000.0) as i32;
    let hours = total_secs / 3600;
    let mins = (total_secs % 3600) / 60;
    let secs = total_secs % 60;
    format!("{:02}:{:02}:{:02},{:03}", hours, mins, secs, millis)
}
