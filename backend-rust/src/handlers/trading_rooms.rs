//! Trading Rooms Handlers
//!
//! ICT 11+ Principal Engineer Grade
//! Complete trading room management with video listing

use axum::{
    extract::{Path, Query, State},
    Json,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    errors::AppError,
    extractors::AuthUser,
    models::{VideoListQuery, VideoWithTrader},
    repositories::VideoRepository,
    responses::ApiResponse,
    AppState,
};

#[derive(Debug, Serialize)]
pub struct TradingRoomResponse {
    pub id: Uuid,
    pub name: String,
    pub slug: String,
    pub description: Option<String>,
    pub is_active: bool,
}

#[derive(Debug, Serialize)]
pub struct TradingRoomVideosResponse {
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

/// GET /api/trading-rooms - List all trading rooms
pub async fn index(
    State(state): State<AppState>,
    _auth: AuthUser,
) -> Result<Json<ApiResponse<Vec<TradingRoomResponse>>>, AppError> {
    let rooms = sqlx::query_as::<_, (Uuid, String, String, Option<String>, bool)>(
        "SELECT id, name, slug, description, is_active FROM trading_rooms WHERE is_active = true ORDER BY name"
    )
    .fetch_all(&state.db)
    .await?
    .into_iter()
    .map(|(id, name, slug, description, is_active)| TradingRoomResponse {
        id,
        name,
        slug,
        description,
        is_active,
    })
    .collect();

    Ok(Json(ApiResponse::success(rooms)))
}

/// GET /api/trading-rooms/:slug - Get single trading room
pub async fn show(
    State(state): State<AppState>,
    _auth: AuthUser,
    Path(slug): Path<String>,
) -> Result<Json<ApiResponse<TradingRoomResponse>>, AppError> {
    let room = sqlx::query_as::<_, (Uuid, String, String, Option<String>, bool)>(
        "SELECT id, name, slug, description, is_active FROM trading_rooms WHERE slug = $1 AND is_active = true"
    )
    .bind(&slug)
    .fetch_optional(&state.db)
    .await?
    .map(|(id, name, slug, description, is_active)| TradingRoomResponse {
        id,
        name,
        slug,
        description,
        is_active,
    })
    .ok_or_else(|| AppError::NotFound("Trading room not found".to_string()))?;

    Ok(Json(ApiResponse::success(room)))
}

/// Query params for video listing
#[derive(Debug, Deserialize, Default)]
pub struct VideoQueryParams {
    pub page: Option<i64>,
    pub per_page: Option<i64>,
    pub search: Option<String>,
}

/// GET /api/trading-rooms/:slug/videos - List videos for a trading room
pub async fn videos(
    State(state): State<AppState>,
    _auth: AuthUser,
    Path(slug): Path<String>,
    Query(params): Query<VideoQueryParams>,
) -> Result<Json<ApiResponse<TradingRoomVideosResponse>>, AppError> {
    let repo = VideoRepository::new(&state.db);

    let query = VideoListQuery {
        page: params.page,
        per_page: params.per_page,
        room_slug: Some(slug),
        search: params.search,
        is_published: Some(true),
        ..Default::default()
    };

    let videos = repo.list(&query).await?;
    let total = repo.count(&query).await?;
    let per_page = query.per_page();
    let total_pages = (total as f64 / per_page as f64).ceil() as i64;

    Ok(Json(ApiResponse::success(TradingRoomVideosResponse {
        videos,
        pagination: PaginationInfo {
            page: query.page(),
            per_page,
            total,
            total_pages,
        },
    })))
}

/// POST /api/trading-rooms/:slug/sso - Generate SSO token for external room
pub async fn generate_sso(
    State(_state): State<AppState>,
    _auth: AuthUser,
    Path(_slug): Path<String>,
) -> Result<Json<serde_json::Value>, AppError> {
    // SSO implementation would go here
    Err(AppError::Internal("SSO not yet implemented".to_string()))
}
