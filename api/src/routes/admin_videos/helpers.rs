//! Helper functions for video transformations and formatting.
//!
//! Pure functions — no DB / no I/O. Shared by `crud` (the main router's
//! responses) and any future sub-module that needs to build a
//! `VideoResponse` from a `UnifiedVideoRow`.

use crate::models::video::{get_all_tags, RoomInfo, TagDetail, TraderInfo, VideoResponse};

use super::UnifiedVideoRow;

pub(super) fn slugify(s: &str) -> String {
    s.to_lowercase()
        .chars()
        .map(|c| if c.is_alphanumeric() { c } else { '-' })
        .collect::<String>()
        .split('-')
        .filter(|s| !s.is_empty())
        .collect::<Vec<_>>()
        .join("-")
}

pub(super) fn get_embed_url(video: &UnifiedVideoRow) -> String {
    match video.video_platform.as_str() {
        "bunny" => {
            if let (Some(guid), Some(lib_id)) = (&video.bunny_video_guid, video.bunny_library_id) {
                format!("https://iframe.mediadelivery.net/embed/{lib_id}/{guid}")
            } else {
                video.video_url.clone()
            }
        }
        "vimeo" => {
            if let Some(id) = &video.video_id {
                format!("https://player.vimeo.com/video/{id}")
            } else {
                video.video_url.clone()
            }
        }
        "youtube" => {
            if let Some(id) = &video.video_id {
                format!("https://www.youtube.com/embed/{id}")
            } else {
                video.video_url.clone()
            }
        }
        _ => video.video_url.clone(),
    }
}

pub(super) fn format_duration(seconds: Option<i32>) -> String {
    match seconds {
        Some(d) if d > 0 => {
            let hours = d / 3600;
            let minutes = (d % 3600) / 60;
            let secs = d % 60;
            if hours > 0 {
                format!("{hours}:{minutes:02}:{secs:02}")
            } else {
                format!("{minutes}:{secs:02}")
            }
        }
        _ => String::new(),
    }
}

pub(super) fn get_tags_vec(tags: &Option<serde_json::Value>) -> Vec<String> {
    tags.as_ref()
        .and_then(|t| serde_json::from_value::<Vec<String>>(t.clone()).ok())
        .unwrap_or_default()
}

pub(super) fn get_tag_details(tags: &Option<serde_json::Value>) -> Vec<TagDetail> {
    get_tags_vec(tags)
        .iter()
        .filter_map(|slug| get_all_tags().iter().find(|t| &t.slug == slug).cloned())
        .collect()
}

pub(super) fn video_to_response(
    video: UnifiedVideoRow,
    trader: Option<TraderInfo>,
    rooms: Vec<RoomInfo>,
) -> VideoResponse {
    VideoResponse {
        id: video.id,
        title: video.title.clone(),
        slug: video.slug.clone(),
        description: video.description.clone(),
        video_url: video.video_url.clone(),
        embed_url: get_embed_url(&video),
        video_platform: video.video_platform.clone(),
        thumbnail_url: video.thumbnail_url.clone(),
        duration: video.duration,
        formatted_duration: format_duration(video.duration),
        content_type: video.content_type.clone(),
        video_date: video.video_date.to_string(),
        formatted_date: video.video_date.format("%B %d, %Y").to_string(),
        is_published: video.is_published,
        is_featured: video.is_featured,
        tags: get_tags_vec(&video.tags),
        tag_details: get_tag_details(&video.tags),
        views_count: video.views_count,
        trader,
        rooms,
        created_at: video.created_at.format("%Y-%m-%dT%H:%M:%S").to_string(),
    }
}
