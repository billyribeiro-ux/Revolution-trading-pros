//! Room resources helper functions (split from `room_resources.rs` lines 238-377).
//!
//! Pure formatting / response-shaping helpers shared by all sub-modules.

use super::{ResourceResponse, RoomResource};

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

pub(super) fn get_embed_url(resource: &RoomResource) -> String {
    if resource.resource_type != "video" {
        return resource.file_url.clone();
    }

    match resource.video_platform.as_deref() {
        Some("bunny") => {
            if let (Some(guid), Some(lib_id)) =
                (&resource.bunny_video_guid, resource.bunny_library_id)
            {
                format!("https://iframe.mediadelivery.net/embed/{lib_id}/{guid}")
            } else {
                resource.file_url.clone()
            }
        }
        Some("vimeo") => {
            if let Some(id) = &resource.video_id {
                format!("https://player.vimeo.com/video/{id}")
            } else {
                resource.file_url.clone()
            }
        }
        Some("youtube") => {
            if let Some(id) = &resource.video_id {
                format!("https://www.youtube.com/embed/{id}")
            } else {
                resource.file_url.clone()
            }
        }
        _ => resource.file_url.clone(),
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

pub(super) fn format_file_size(bytes: Option<i64>) -> String {
    match bytes {
        Some(b) if b > 0 => {
            if b >= 1_073_741_824 {
                format!("{:.2} GB", b as f64 / 1_073_741_824.0)
            } else if b >= 1_048_576 {
                format!("{:.2} MB", b as f64 / 1_048_576.0)
            } else if b >= 1024 {
                format!("{:.2} KB", b as f64 / 1024.0)
            } else {
                format!("{b} bytes")
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

pub(super) fn resource_to_response(resource: RoomResource) -> ResourceResponse {
    let access_level = resource
        .access_level
        .clone()
        .unwrap_or_else(|| "premium".to_string());
    let requires_premium = access_level != "free";

    ResourceResponse {
        id: resource.id,
        title: resource.title.clone(),
        slug: resource.slug.clone(),
        description: resource.description.clone(),
        resource_type: resource.resource_type.clone(),
        content_type: resource.content_type.clone(),
        file_url: resource.file_url.clone(),
        embed_url: get_embed_url(&resource),
        secure_download_url: None, // Generated on-demand via /download endpoint
        mime_type: resource.mime_type.clone(),
        file_size: resource.file_size,
        formatted_size: format_file_size(resource.file_size),
        video_platform: resource.video_platform.clone(),
        duration: resource.duration,
        formatted_duration: format_duration(resource.duration),
        thumbnail_url: resource.thumbnail_url.clone(),
        width: resource.width,
        height: resource.height,
        trading_room_id: resource.trading_room_id,
        trader_id: resource.trader_id,
        resource_date: resource.resource_date.to_string(),
        formatted_date: resource.resource_date.format("%B %d, %Y").to_string(),
        is_published: resource.is_published,
        is_featured: resource.is_featured,
        is_pinned: resource.is_pinned,
        category: resource.category.clone(),
        tags: get_tags_vec(&resource.tags),
        difficulty_level: resource.difficulty_level.clone(),
        views_count: resource.views_count,
        downloads_count: resource.downloads_count,
        section: resource.section.clone(),
        created_at: resource.created_at.format("%Y-%m-%dT%H:%M:%S").to_string(),
        // ICT 7 NEW: Access control
        access_level,
        requires_premium,
        // ICT 7 NEW: Versioning
        version: resource.version.unwrap_or(1),
        has_previous_version: resource.previous_version_id.is_some(),
        is_latest_version: resource.is_latest_version.unwrap_or(true),
        // ICT 7 NEW: Course/Lesson linking
        course_id: resource.course_id,
        lesson_id: resource.lesson_id,
        course_order: resource.course_order,
    }
}
