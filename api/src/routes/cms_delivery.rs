//! CMS Delivery API - Apple ICT 7+ Principal Engineer Grade
//!
//! Public API routes for content delivery to the frontend.
//! Optimized for performance with <10ms response targets.
//!
//! Features:
//! - Full-text search with ranking and highlighting
//! - Content retrieval with related data
//! - Sitemap generation support
//! - CDN-friendly response headers
//!
//! @version 2.0.0 - January 2026

use axum::{
    extract::{Path, Query, State},
    http::{header, StatusCode},
    response::{IntoResponse, Response},
    routing::get,
    Json, Router,
};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value as JsonValue};
use uuid::Uuid;

use crate::{
    models::cms::{CmsContentStatus, CmsContentType},
    AppState,
};

// ═══════════════════════════════════════════════════════════════════════════════════════
// TYPES
// ═══════════════════════════════════════════════════════════════════════════════════════

type ApiResult<T> = Result<Json<T>, (StatusCode, Json<JsonValue>)>;

fn api_error(status: StatusCode, message: &str) -> (StatusCode, Json<JsonValue>) {
    (status, Json(json!({ "error": message })))
}

/// Search query parameters
#[derive(Debug, Deserialize)]
pub struct SearchQuery {
    /// Search query string
    pub q: String,
    /// Filter by content types (comma-separated)
    pub content_types: Option<String>,
    /// Filter by tag IDs (comma-separated UUIDs)
    pub tags: Option<String>,
    /// Locale (default: en)
    pub locale: Option<String>,
    /// Number of results (default: 20, max: 100)
    pub limit: Option<i32>,
    /// Offset for pagination
    pub offset: Option<i32>,
}

/// Search result item
#[derive(Debug, Serialize)]
pub struct SearchResultItem {
    pub id: Uuid,
    pub content_type: String,
    pub slug: String,
    pub title: String,
    pub excerpt: Option<String>,
    pub featured_image_id: Option<Uuid>,
    pub author_id: Option<Uuid>,
    pub published_at: Option<chrono::DateTime<chrono::Utc>>,
    pub rank: f32,
    pub headline: Option<String>,
}

/// Search response
#[derive(Debug, Serialize)]
pub struct SearchResponse {
    pub data: Vec<SearchResultItem>,
    pub meta: SearchMeta,
}

#[derive(Debug, Serialize)]
pub struct SearchMeta {
    pub query: String,
    pub total: i64,
    pub limit: i32,
    pub offset: i32,
    pub took_ms: u64,
}

/// Content list query parameters
#[derive(Debug, Deserialize)]
pub struct ContentListQuery {
    pub limit: Option<i64>,
    pub offset: Option<i64>,
    pub locale: Option<String>,
    pub tag: Option<Uuid>,
    pub category: Option<Uuid>,
}

/// Sitemap entry
#[derive(Debug, Serialize)]
pub struct SitemapEntry {
    pub content_type: String,
    pub slug: String,
    pub locale: String,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub priority: f32,
    pub changefreq: String,
}

// ═══════════════════════════════════════════════════════════════════════════════════════
// SEARCH ENDPOINT
// ═══════════════════════════════════════════════════════════════════════════════════════

/// Full-text search across all content
async fn search_content(
    State(state): State<AppState>,
    Query(query): Query<SearchQuery>,
) -> ApiResult<SearchResponse> {
    let start = std::time::Instant::now();

    if query.q.trim().is_empty() {
        return Err(api_error(StatusCode::BAD_REQUEST, "Search query is required"));
    }

    let limit = query.limit.unwrap_or(20).min(100);
    let offset = query.offset.unwrap_or(0);
    let locale = query.locale.as_deref().unwrap_or("en");

    // Parse content types
    let content_types: Option<Vec<String>> = query
        .content_types
        .as_ref()
        .map(|s| s.split(',').map(|t| t.trim().to_string()).collect());

    // Parse tag IDs
    let tag_ids: Option<Vec<Uuid>> = query.tags.as_ref().and_then(|s| {
        s.split(',')
            .filter_map(|t| Uuid::parse_str(t.trim()).ok())
            .collect::<Vec<_>>()
            .into()
    });

    // Execute search query using the database function
    let results: Vec<SearchResultItem> = sqlx::query_as!(
        SearchResultItem,
        r#"
        SELECT
            id,
            content_type::text as "content_type!",
            slug::varchar as "slug!",
            title::varchar as "title!",
            excerpt::text as "excerpt",
            featured_image_id,
            author_id,
            published_at,
            rank::real as "rank!",
            headline::text as "headline"
        FROM cms_search_content(
            $1::text,
            $2::cms_content_type[],
            $3::uuid[],
            $4::text,
            $5::integer,
            $6::integer,
            true
        )
        "#,
        query.q,
        content_types.as_deref() as Option<&[String]>,
        tag_ids.as_deref() as Option<&[Uuid]>,
        locale,
        limit,
        offset
    )
    .fetch_all(&state.db.pool)
    .await
    .map_err(|e| api_error(StatusCode::INTERNAL_SERVER_ERROR, &e.to_string()))?;

    // Get total count for pagination
    let total: (i64,) = sqlx::query_as(
        r#"
        SELECT COUNT(*)
        FROM cms_content
        WHERE deleted_at IS NULL
        AND status = 'published'
        AND search_vector @@ websearch_to_tsquery('english', $1)
        "#,
    )
    .bind(&query.q)
    .fetch_one(&state.db.pool)
    .await
    .map_err(|e| api_error(StatusCode::INTERNAL_SERVER_ERROR, &e.to_string()))?;

    let took_ms = start.elapsed().as_millis() as u64;

    Ok(Json(SearchResponse {
        data: results,
        meta: SearchMeta {
            query: query.q,
            total: total.0,
            limit,
            offset,
            took_ms,
        },
    }))
}

// ═══════════════════════════════════════════════════════════════════════════════════════
// CONTENT DELIVERY ENDPOINTS
// ═══════════════════════════════════════════════════════════════════════════════════════

/// Get full content by type and slug
async fn get_content_full(
    State(state): State<AppState>,
    Path((content_type, slug)): Path<(String, String)>,
    Query(query): Query<ContentListQuery>,
) -> ApiResult<JsonValue> {
    let locale = query.locale.as_deref().unwrap_or("en");

    let content: Option<JsonValue> = sqlx::query_scalar(
        "SELECT cms_get_content_full($1::cms_content_type, $2, $3, false)",
    )
    .bind(&content_type)
    .bind(&slug)
    .bind(locale)
    .fetch_optional(&state.db.pool)
    .await
    .map_err(|e| api_error(StatusCode::INTERNAL_SERVER_ERROR, &e.to_string()))?;

    content
        .flatten()
        .map(Json)
        .ok_or_else(|| api_error(StatusCode::NOT_FOUND, "Content not found"))
}

/// Get recent content by type
async fn get_recent_content(
    State(state): State<AppState>,
    Path(content_type): Path<String>,
    Query(query): Query<ContentListQuery>,
) -> ApiResult<JsonValue> {
    let limit = query.limit.unwrap_or(10).min(50) as i32;
    let offset = query.offset.unwrap_or(0) as i32;
    let locale = query.locale.as_deref().unwrap_or("en");

    let result: JsonValue = sqlx::query_scalar(
        "SELECT cms_get_recent_content($1::cms_content_type, $2, $3, $4)",
    )
    .bind(&content_type)
    .bind(limit)
    .bind(offset)
    .bind(locale)
    .fetch_one(&state.db.pool)
    .await
    .map_err(|e| api_error(StatusCode::INTERNAL_SERVER_ERROR, &e.to_string()))?;

    Ok(Json(result))
}

/// Get content stats for public display
async fn get_public_stats(State(state): State<AppState>) -> ApiResult<JsonValue> {
    let stats: JsonValue = sqlx::query_scalar("SELECT cms_get_content_stats()")
        .fetch_one(&state.db.pool)
        .await
        .map_err(|e| api_error(StatusCode::INTERNAL_SERVER_ERROR, &e.to_string()))?;

    // Return only public-safe stats
    let public_stats = json!({
        "content": {
            "blogPosts": stats.get("content").and_then(|c| c.get("byType")).and_then(|t| t.get("blog_post")),
            "courses": stats.get("content").and_then(|c| c.get("byType")).and_then(|t| t.get("course")),
            "tradingRooms": stats.get("content").and_then(|c| c.get("byType")).and_then(|t| t.get("trading_room")),
        }
    });

    Ok(Json(public_stats))
}

// ═══════════════════════════════════════════════════════════════════════════════════════
// SITEMAP SUPPORT
// ═══════════════════════════════════════════════════════════════════════════════════════

/// Get all content slugs for sitemap generation
async fn get_sitemap_entries(
    State(state): State<AppState>,
    Query(query): Query<ContentListQuery>,
) -> ApiResult<Vec<SitemapEntry>> {
    let locale = query.locale.as_deref();

    let entries: Vec<SitemapEntry> = sqlx::query_as!(
        SitemapEntry,
        r#"
        SELECT
            content_type::text as "content_type!",
            slug::varchar as "slug!",
            locale::varchar as "locale!",
            updated_at as "updated_at!",
            CASE content_type
                WHEN 'page' THEN 1.0
                WHEN 'blog_post' THEN 0.8
                WHEN 'course' THEN 0.9
                WHEN 'trading_room' THEN 0.9
                WHEN 'alert_service' THEN 0.9
                ELSE 0.5
            END::real as "priority!",
            CASE content_type
                WHEN 'blog_post' THEN 'weekly'
                WHEN 'weekly_watchlist' THEN 'weekly'
                ELSE 'monthly'
            END::varchar as "changefreq!"
        FROM cms_get_all_slugs($1::cms_content_type, $2)
        "#,
        None::<String>,
        locale
    )
    .fetch_all(&state.db.pool)
    .await
    .map_err(|e| api_error(StatusCode::INTERNAL_SERVER_ERROR, &e.to_string()))?;

    Ok(Json(entries))
}

// ═══════════════════════════════════════════════════════════════════════════════════════
// NAVIGATION & MENUS
// ═══════════════════════════════════════════════════════════════════════════════════════

/// Get navigation menu by location
async fn get_menu_by_location(
    State(state): State<AppState>,
    Path(location): Path<String>,
) -> ApiResult<JsonValue> {
    let menu: Option<JsonValue> = sqlx::query_scalar(
        r#"
        SELECT jsonb_build_object(
            'id', id,
            'name', name,
            'slug', slug,
            'location', location,
            'items', items,
            'isActive', is_active
        )
        FROM cms_navigation_menus
        WHERE (location = $1 OR slug = $1) AND is_active = true
        "#,
    )
    .bind(&location)
    .fetch_optional(&state.db.pool)
    .await
    .map_err(|e| api_error(StatusCode::INTERNAL_SERVER_ERROR, &e.to_string()))?;

    menu.flatten()
        .map(Json)
        .ok_or_else(|| api_error(StatusCode::NOT_FOUND, "Menu not found"))
}

/// Get all active menus
async fn get_all_menus(State(state): State<AppState>) -> ApiResult<Vec<JsonValue>> {
    let menus: Vec<JsonValue> = sqlx::query_scalar(
        r#"
        SELECT jsonb_build_object(
            'id', id,
            'name', name,
            'slug', slug,
            'location', location,
            'items', items
        )
        FROM cms_navigation_menus
        WHERE is_active = true
        ORDER BY name
        "#,
    )
    .fetch_all(&state.db.pool)
    .await
    .map_err(|e| api_error(StatusCode::INTERNAL_SERVER_ERROR, &e.to_string()))?;

    Ok(Json(menus))
}

// ═══════════════════════════════════════════════════════════════════════════════════════
// REDIRECTS
// ═══════════════════════════════════════════════════════════════════════════════════════

/// Check for redirect and return target if found
async fn check_redirect(
    State(state): State<AppState>,
    Path(path): Path<String>,
) -> Result<Response, (StatusCode, Json<JsonValue>)> {
    let normalized_path = if path.starts_with('/') {
        path.clone()
    } else {
        format!("/{}", path)
    };

    let redirect: Option<(String, i32)> = sqlx::query_as(
        r#"
        SELECT target_path, status_code
        FROM cms_redirects
        WHERE source_path = $1 AND is_active = true
        "#,
    )
    .bind(&normalized_path)
    .fetch_optional(&state.db.pool)
    .await
    .map_err(|e| api_error(StatusCode::INTERNAL_SERVER_ERROR, &e.to_string()))?;

    match redirect {
        Some((target, status_code)) => {
            // Increment hit count
            let _ = sqlx::query(
                "UPDATE cms_redirects SET hit_count = hit_count + 1, last_hit_at = NOW() WHERE source_path = $1",
            )
            .bind(&normalized_path)
            .execute(&state.db.pool)
            .await;

            let status = match status_code {
                301 => StatusCode::MOVED_PERMANENTLY,
                302 => StatusCode::FOUND,
                307 => StatusCode::TEMPORARY_REDIRECT,
                308 => StatusCode::PERMANENT_REDIRECT,
                _ => StatusCode::MOVED_PERMANENTLY,
            };

            Ok((
                status,
                [(header::LOCATION, target.clone())],
                Json(json!({ "redirect": target, "statusCode": status_code })),
            )
                .into_response())
        }
        None => Ok((
            StatusCode::NOT_FOUND,
            Json(json!({ "redirect": null })),
        )
            .into_response()),
    }
}

// ═══════════════════════════════════════════════════════════════════════════════════════
// TAGS
// ═══════════════════════════════════════════════════════════════════════════════════════

/// Get all tags with usage counts
async fn get_all_tags(State(state): State<AppState>) -> ApiResult<Vec<JsonValue>> {
    let tags: Vec<JsonValue> = sqlx::query_scalar(
        r#"
        SELECT jsonb_build_object(
            'id', id,
            'name', name,
            'slug', slug,
            'color', color,
            'icon', icon,
            'usageCount', usage_count
        )
        FROM cms_tags
        WHERE usage_count > 0
        ORDER BY usage_count DESC, name
        "#,
    )
    .fetch_all(&state.db.pool)
    .await
    .map_err(|e| api_error(StatusCode::INTERNAL_SERVER_ERROR, &e.to_string()))?;

    Ok(Json(tags))
}

/// Get content by tag
async fn get_content_by_tag(
    State(state): State<AppState>,
    Path(tag_slug): Path<String>,
    Query(query): Query<ContentListQuery>,
) -> ApiResult<JsonValue> {
    let limit = query.limit.unwrap_or(20).min(50) as i32;
    let offset = query.offset.unwrap_or(0) as i32;

    let result: JsonValue = sqlx::query_scalar(
        r#"
        SELECT jsonb_build_object(
            'tag', (
                SELECT jsonb_build_object(
                    'id', id,
                    'name', name,
                    'slug', slug,
                    'description', description,
                    'color', color
                )
                FROM cms_tags WHERE slug = $1
            ),
            'data', COALESCE((
                SELECT jsonb_agg(jsonb_build_object(
                    'id', c.id,
                    'contentType', c.content_type,
                    'slug', c.slug,
                    'title', c.title,
                    'excerpt', c.excerpt,
                    'featuredImage', CASE WHEN a.id IS NOT NULL THEN jsonb_build_object(
                        'url', a.cdn_url,
                        'width', a.width,
                        'height', a.height,
                        'blurhash', a.blurhash
                    ) END,
                    'publishedAt', c.published_at
                ) ORDER BY c.published_at DESC)
                FROM cms_content c
                JOIN cms_content_tags ct ON c.id = ct.content_id
                JOIN cms_tags t ON ct.tag_id = t.id
                LEFT JOIN cms_assets a ON c.featured_image_id = a.id
                WHERE t.slug = $1
                AND c.status = 'published'
                AND c.deleted_at IS NULL
                LIMIT $2 OFFSET $3
            ), '[]'::jsonb),
            'meta', jsonb_build_object(
                'total', (
                    SELECT COUNT(*)
                    FROM cms_content c
                    JOIN cms_content_tags ct ON c.id = ct.content_id
                    JOIN cms_tags t ON ct.tag_id = t.id
                    WHERE t.slug = $1
                    AND c.status = 'published'
                    AND c.deleted_at IS NULL
                ),
                'limit', $2,
                'offset', $3
            )
        )
        "#,
    )
    .bind(&tag_slug)
    .bind(limit)
    .bind(offset)
    .fetch_one(&state.db.pool)
    .await
    .map_err(|e| api_error(StatusCode::INTERNAL_SERVER_ERROR, &e.to_string()))?;

    Ok(Json(result))
}

// ═══════════════════════════════════════════════════════════════════════════════════════
// ROUTER
// ═══════════════════════════════════════════════════════════════════════════════════════

/// Public CMS delivery routes (no auth required)
pub fn delivery_router() -> Router<AppState> {
    Router::new()
        // Search
        .route("/search", get(search_content))
        // Content delivery
        .route("/content/:content_type/:slug", get(get_content_full))
        .route("/content/:content_type", get(get_recent_content))
        // Stats (public-safe)
        .route("/stats", get(get_public_stats))
        // Sitemap support
        .route("/sitemap", get(get_sitemap_entries))
        // Navigation
        .route("/menus", get(get_all_menus))
        .route("/menus/:location", get(get_menu_by_location))
        // Redirects
        .route("/redirect/*path", get(check_redirect))
        // Tags
        .route("/tags", get(get_all_tags))
        .route("/tags/:tag_slug", get(get_content_by_tag))
}
