//! Videos Handlers
//!
//! ICT 11+ Principal Engineer Grade
//! Complete video CRUD with Previous/Next navigation support

use axum::{
    extract::{Path, Query, State},
    Json,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    errors::AppError,
    models::{VideoListQuery, VideoWithNavigation, VideoWithTrader},
    repositories::VideoRepository,
    responses::ApiResponse,
    AppState,
};

/// Video list response matching frontend expectations
#[derive(Debug, Serialize)]
pub struct VideoListResponse {
    pub videos: Vec<VideoWithTrader>,
    pub pagination: PaginationInfo,
}

#[derive(Debug, Serialize)]
pub struct PaginationInfo {
    pub page: i64,
    pub per_page: i64,
    pub total: i64,
    pub total_pages: i64,
}

/// GET /api/videos - List all videos with pagination
pub async fn index(
    State(state): State<AppState>,
    Query(query): Query<VideoListQuery>,
) -> Result<Json<ApiResponse<VideoListResponse>>, AppError> {
    let repo = VideoRepository::new(&state.db);

    let videos = repo.list(&query).await?;
    let total = repo.count(&query).await?;
    let per_page = query.per_page();
    let total_pages = (total as f64 / per_page as f64).ceil() as i64;

    Ok(Json(ApiResponse::success(VideoListResponse {
        videos,
        pagination: PaginationInfo {
            page: query.page(),
            per_page,
            total,
            total_pages,
        },
    })))
}

/// GET /api/videos/:id - Get single video by UUID
pub async fn show(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<ApiResponse<VideoWithTrader>>, AppError> {
    let repo = VideoRepository::new(&state.db);

    let video = repo
        .find(id)
        .await?
        .ok_or_else(|| AppError::NotFound("Video not found".to_string()))?;

    // Get trader info if available
    let trader = if let Some(trader_id) = video.trader_id {
        sqlx::query_as::<_, (Uuid, String, String, Option<String>)>(
            "SELECT id, name, slug, avatar_url FROM room_traders WHERE id = $1",
        )
        .bind(trader_id)
        .fetch_optional(&state.db)
        .await?
        .map(|(id, name, slug, avatar_url)| crate::models::TraderInfo {
            id,
            name,
            slug,
            avatar_url,
        })
    } else {
        None
    };

    let response = VideoWithTrader {
        id: video.id,
        trading_room_id: video.trading_room_id,
        title: video.title,
        slug: video.slug,
        description: video.description,
        video_url: video.video_url,
        thumbnail_url: video.thumbnail_url,
        duration: video.duration,
        is_published: video.is_published,
        is_featured: video.is_featured,
        views_count: video.views_count,
        published_at: video.published_at,
        created_at: video.created_at,
        trader,
    };

    Ok(Json(ApiResponse::success(response)))
}

/// Path params for room + video slug lookup
#[derive(Debug, Deserialize)]
pub struct RoomVideoPath {
    pub room_slug: String,
    pub video_slug: String,
}

/// GET /api/trading-rooms/:room_slug/videos/:video_slug
/// Returns video with Previous/Next navigation
pub async fn show_by_slug(
    State(state): State<AppState>,
    Path(path): Path<RoomVideoPath>,
) -> Result<Json<ApiResponse<VideoWithNavigation>>, AppError> {
    let repo = VideoRepository::new(&state.db);

    let video_nav = repo
        .get_with_navigation(&path.room_slug, &path.video_slug)
        .await?
        .ok_or_else(|| AppError::NotFound("Video not found".to_string()))?;

    // Increment view count (fire and forget)
    let _ = repo.increment_views(video_nav.video.id).await;

    Ok(Json(ApiResponse::success(video_nav)))
}

/// GET /api/videos/by-slug/:slug - Get video by slug (global)
pub async fn show_by_global_slug(
    State(state): State<AppState>,
    Path(slug): Path<String>,
) -> Result<Json<ApiResponse<VideoWithTrader>>, AppError> {
    let repo = VideoRepository::new(&state.db);

    let video = repo
        .find_by_slug(&slug)
        .await?
        .ok_or_else(|| AppError::NotFound("Video not found".to_string()))?;

    // Get trader info
    let trader = if let Some(trader_id) = video.trader_id {
        sqlx::query_as::<_, (Uuid, String, String, Option<String>)>(
            "SELECT id, name, slug, avatar_url FROM room_traders WHERE id = $1",
        )
        .bind(trader_id)
        .fetch_optional(&state.db)
        .await?
        .map(|(id, name, slug, avatar_url)| crate::models::TraderInfo {
            id,
            name,
            slug,
            avatar_url,
        })
    } else {
        None
    };

    let response = VideoWithTrader {
        id: video.id,
        trading_room_id: video.trading_room_id,
        title: video.title,
        slug: video.slug,
        description: video.description,
        video_url: video.video_url,
        thumbnail_url: video.thumbnail_url,
        duration: video.duration,
        is_published: video.is_published,
        is_featured: video.is_featured,
        views_count: video.views_count,
        published_at: video.published_at,
        created_at: video.created_at,
        trader,
    };

    // Increment view count
    let _ = repo.increment_views(video.id).await;

    Ok(Json(ApiResponse::success(response)))
}
