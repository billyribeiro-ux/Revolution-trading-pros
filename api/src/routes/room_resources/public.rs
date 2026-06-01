//! Room resources public routes (split from `room_resources.rs`
//! lines 379-623, 1656-1700, 1758-1898 + 2024-2040, 2042-2320).
//!
//! Public-facing handlers: resource listing/detail, download tracking,
//! course/lesson resource queries, engagement (recently-accessed,
//! favorites), and stock-list public reads.

use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Json,
};
use chrono::NaiveDate;
use serde::Serialize;
use serde_json::json;
use sqlx::FromRow;

use crate::AppState;

use super::helpers::resource_to_response;
use super::{
    PaginationMeta, ResourceListQuery, ResourceResponse, RoomResource, StockList, StockListQuery,
};

// ═══════════════════════════════════════════════════════════════════════════
// PUBLIC ROUTES
// ═══════════════════════════════════════════════════════════════════════════

/// GET /api/room-resources - List resources (public)
pub(super) async fn list_resources(
    State(state): State<AppState>,
    Query(query): Query<ResourceListQuery>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let page = query.page.unwrap_or(1).max(1);
    let per_page = query.per_page.unwrap_or(20).min(100);
    let offset = (page - 1) * per_page;

    // Build parameterized query to prevent SQL injection
    let mut conditions = Vec::new();
    let mut param_idx = 1usize;

    if query.room_id.is_some() {
        conditions.push(format!("trading_room_id = ${param_idx}"));
        param_idx += 1;
    }
    if query.resource_type.is_some() {
        conditions.push(format!("resource_type = ${param_idx}"));
        param_idx += 1;
    }
    if query.content_type.is_some() {
        conditions.push(format!("content_type = ${param_idx}"));
        param_idx += 1;
    }
    if query.is_featured.is_some() {
        conditions.push(format!("is_featured = ${param_idx}"));
        param_idx += 1;
    }
    if query.difficulty_level.is_some() {
        conditions.push(format!("difficulty_level = ${param_idx}"));
        param_idx += 1;
    }
    if query.section.is_some() {
        conditions.push(format!("section = ${param_idx}"));
        param_idx += 1;
    }
    // Tags: each tag gets its own JSONB containment check
    let tag_list: Vec<String> = query
        .tags
        .as_ref()
        .map(|t| t.split(',').map(|s| s.trim().to_string()).collect())
        .unwrap_or_default();
    for _ in &tag_list {
        conditions.push(format!("tags @> ${param_idx}"));
        param_idx += 1;
    }
    if query.search.is_some() {
        conditions.push(format!(
            "(title ILIKE ${param_idx} OR description ILIKE ${param_idx})"
        ));
        param_idx += 1;
    }
    if query.access_level.is_some() {
        conditions.push(format!("access_level = ${param_idx}"));
        param_idx += 1;
    }
    if query.course_id.is_some() {
        conditions.push(format!("course_id = ${param_idx}"));
        param_idx += 1;
    }
    if query.lesson_id.is_some() {
        conditions.push(format!("lesson_id = ${param_idx}"));
        param_idx += 1;
    }

    // ICT 7 NEW: Only show latest versions by default
    let show_latest_only = query.latest_only.unwrap_or(true);
    if show_latest_only {
        conditions.push("(is_latest_version = true OR is_latest_version IS NULL)".to_string());
    }

    let where_clause = if conditions.is_empty() {
        String::new()
    } else {
        format!(" AND {}", conditions.join(" AND "))
    };

    let sql = format!(
        "SELECT * FROM room_resources WHERE is_published = true AND deleted_at IS NULL{} ORDER BY is_pinned DESC, is_featured DESC, resource_date DESC, created_at DESC LIMIT ${} OFFSET ${}",
        where_clause, param_idx, param_idx + 1
    );
    let count_sql = format!(
        "SELECT COUNT(*) FROM room_resources WHERE is_published = true AND deleted_at IS NULL{where_clause}"
    );

    // Helper macro-like closure to bind params to a query
    // Bind parameters for the main query
    let mut q = sqlx::query_as::<_, RoomResource>(&sql);
    if let Some(room_id) = query.room_id {
        q = q.bind(room_id);
    }
    if let Some(ref resource_type) = query.resource_type {
        q = q.bind(resource_type);
    }
    if let Some(ref content_type) = query.content_type {
        q = q.bind(content_type);
    }
    if let Some(is_featured) = query.is_featured {
        q = q.bind(is_featured);
    }
    if let Some(ref difficulty) = query.difficulty_level {
        q = q.bind(difficulty);
    }
    if let Some(ref section) = query.section {
        q = q.bind(section);
    }
    for tag in &tag_list {
        let tag_json = serde_json::json!([tag]);
        q = q.bind(tag_json);
    }
    if let Some(ref search) = query.search {
        let search_pattern = format!("%{search}%");
        q = q.bind(search_pattern);
    }
    if let Some(ref access_level) = query.access_level {
        q = q.bind(access_level);
    }
    if let Some(course_id) = query.course_id {
        q = q.bind(course_id);
    }
    if let Some(lesson_id) = query.lesson_id {
        q = q.bind(lesson_id);
    }
    q = q.bind(per_page);
    q = q.bind(offset);

    let resources: Vec<RoomResource> = q.fetch_all(&state.db.pool).await.map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": e.to_string()})),
        )
    })?;

    // Bind parameters for the count query (same filters, no LIMIT/OFFSET)
    let mut cq = sqlx::query_as::<_, (i64,)>(&count_sql);
    if let Some(room_id) = query.room_id {
        cq = cq.bind(room_id);
    }
    if let Some(ref resource_type) = query.resource_type {
        cq = cq.bind(resource_type);
    }
    if let Some(ref content_type) = query.content_type {
        cq = cq.bind(content_type);
    }
    if let Some(is_featured) = query.is_featured {
        cq = cq.bind(is_featured);
    }
    if let Some(ref difficulty) = query.difficulty_level {
        cq = cq.bind(difficulty);
    }
    if let Some(ref section) = query.section {
        cq = cq.bind(section);
    }
    for tag in &tag_list {
        let tag_json = serde_json::json!([tag]);
        cq = cq.bind(tag_json);
    }
    if let Some(ref search) = query.search {
        let search_pattern = format!("%{search}%");
        cq = cq.bind(search_pattern);
    }
    if let Some(ref access_level) = query.access_level {
        cq = cq.bind(access_level);
    }
    if let Some(course_id) = query.course_id {
        cq = cq.bind(course_id);
    }
    if let Some(lesson_id) = query.lesson_id {
        cq = cq.bind(lesson_id);
    }

    let total: (i64,) = cq.fetch_one(&state.db.pool).await.unwrap_or((0,));

    let responses: Vec<ResourceResponse> =
        resources.into_iter().map(resource_to_response).collect();
    let last_page = ((total.0 as f64) / (per_page as f64)).ceil() as i64;

    Ok(Json(json!({
        "success": true,
        "data": responses,
        "meta": PaginationMeta {
            current_page: page,
            per_page,
            total: total.0,
            last_page,
        }
    })))
}

/// GET /api/room-resources/:id - Get single resource
pub(super) async fn get_resource(
    State(state): State<AppState>,
    Path(id_or_slug): Path<String>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let resource: RoomResource = if let Ok(id) = id_or_slug.parse::<i64>() {
        sqlx::query_as(
            "SELECT * FROM room_resources WHERE id = $1 AND is_published = true AND deleted_at IS NULL"
        )
        .bind(id)
        .fetch_optional(&state.db.pool)
        .await
    } else {
        sqlx::query_as(
            "SELECT * FROM room_resources WHERE slug = $1 AND is_published = true AND deleted_at IS NULL"
        )
        .bind(&id_or_slug)
        .fetch_optional(&state.db.pool)
        .await
    }
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()}))))?
    .ok_or_else(|| (StatusCode::NOT_FOUND, Json(json!({"error": "Resource not found"}))))?;

    // Increment view count
    let _ = sqlx::query("UPDATE room_resources SET views_count = views_count + 1 WHERE id = $1")
        .bind(resource.id)
        .execute(&state.db.pool)
        .await;

    Ok(Json(json!({
        "success": true,
        "data": resource_to_response(resource)
    })))
}

/// POST /api/room-resources/:id/download - Track download
pub(super) async fn track_download(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let _ = sqlx::query(
        "UPDATE room_resources SET downloads_count = downloads_count + 1 WHERE id = $1",
    )
    .bind(id)
    .execute(&state.db.pool)
    .await;

    Ok(Json(json!({"success": true, "status": "ok"})))
}

// ═══════════════════════════════════════════════════════════════════════════
// ICT 7 NEW: COURSE RESOURCES
// ═══════════════════════════════════════════════════════════════════════════

/// GET /api/room-resources/by-course/:course_id - Get resources for a course
pub(super) async fn get_course_resources(
    State(state): State<AppState>,
    Path(course_id): Path<i64>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let resources: Vec<RoomResource> = sqlx::query_as(
        "SELECT * FROM room_resources WHERE course_id = $1 AND is_published = true AND deleted_at IS NULL ORDER BY course_order ASC, created_at ASC"
    )
    .bind(course_id)
    .fetch_all(&state.db.pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()}))))?;

    let responses: Vec<ResourceResponse> =
        resources.into_iter().map(resource_to_response).collect();

    Ok(Json(json!({
        "success": true,
        "data": responses,
        "total": responses.len()
    })))
}

/// GET /api/room-resources/by-lesson/:lesson_id - Get resources for a lesson
pub(super) async fn get_lesson_resources(
    State(state): State<AppState>,
    Path(lesson_id): Path<i64>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let resources: Vec<RoomResource> = sqlx::query_as(
        "SELECT * FROM room_resources WHERE lesson_id = $1 AND is_published = true AND deleted_at IS NULL ORDER BY course_order ASC, created_at ASC"
    )
    .bind(lesson_id)
    .fetch_all(&state.db.pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()}))))?;

    let responses: Vec<ResourceResponse> =
        resources.into_iter().map(resource_to_response).collect();

    Ok(Json(json!({
        "success": true,
        "data": responses,
        "total": responses.len()
    })))
}

// ═══════════════════════════════════════════════════════════════════════════
// ICT 7 NEW: ETF/STOCK LISTS (PUBLIC READS)
// ═══════════════════════════════════════════════════════════════════════════

/// GET /api/room-resources/stock-lists - List stock/ETF lists
pub(super) async fn list_stock_lists(
    State(state): State<AppState>,
    Query(query): Query<StockListQuery>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let page = query.page.unwrap_or(1).max(1);
    let per_page = query.per_page.unwrap_or(20).min(100);
    let offset = (page - 1) * per_page;

    // Build parameterized query to prevent SQL injection
    let mut conditions = Vec::new();
    let mut param_idx = 1usize;

    if query.room_id.is_some() {
        conditions.push(format!("trading_room_id = ${param_idx}"));
        param_idx += 1;
    }
    if query.list_type.is_some() {
        conditions.push(format!("list_type = ${param_idx}"));
        param_idx += 1;
    }
    if query.is_active.is_some() {
        conditions.push(format!("is_active = ${param_idx}"));
        param_idx += 1;
    }
    if query.week_of.is_some() {
        conditions.push(format!("week_of = ${param_idx}"));
        param_idx += 1;
    }

    let where_clause = if conditions.is_empty() {
        String::new()
    } else {
        format!(" AND {}", conditions.join(" AND "))
    };

    // SAFETY: `where_clause` is composed entirely of `${idx}` bind
    // placeholders produced above (room_id / list_type / is_active /
    // week_of) — no user input is formatted into the SQL text. ORDER BY
    // uses literal column names. LIMIT and OFFSET are bound via
    // `param_idx` placeholders below.
    let sql = format!(
        "SELECT * FROM stock_lists WHERE 1=1{} ORDER BY is_featured DESC, week_of DESC NULLS LAST, created_at DESC LIMIT ${} OFFSET ${}",
        where_clause, param_idx, param_idx + 1
    );
    // SAFETY: see comment on `sql` above — `where_clause` carries only
    // bind-parameter placeholders.
    let count_sql = format!("SELECT COUNT(*) FROM stock_lists WHERE 1=1{where_clause}");

    // Bind parameters for the main query
    let mut q = sqlx::query_as::<_, StockList>(&sql);
    if let Some(room_id) = query.room_id {
        q = q.bind(room_id);
    }
    if let Some(ref list_type) = query.list_type {
        q = q.bind(list_type);
    }
    if let Some(is_active) = query.is_active {
        q = q.bind(is_active);
    }
    if let Some(ref week_of) = query.week_of {
        let week_date = NaiveDate::parse_from_str(week_of, "%Y-%m-%d")
            .unwrap_or_else(|_| chrono::Utc::now().date_naive());
        q = q.bind(week_date);
    }
    q = q.bind(per_page);
    q = q.bind(offset);

    // P2-D: previously `.unwrap_or_default()` turned a DB fault into an empty
    // list with a 200 + success:true — masking an outage. Propagate via the
    // same house 500 shape `get_stock_list` (below) uses.
    let lists: Vec<StockList> = q.fetch_all(&state.db.pool).await.map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": e.to_string()})),
        )
    })?;

    // Bind parameters for the count query
    let mut cq = sqlx::query_as::<_, (i64,)>(&count_sql);
    if let Some(room_id) = query.room_id {
        cq = cq.bind(room_id);
    }
    if let Some(ref list_type) = query.list_type {
        cq = cq.bind(list_type);
    }
    if let Some(is_active) = query.is_active {
        cq = cq.bind(is_active);
    }
    if let Some(ref week_of) = query.week_of {
        let week_date = NaiveDate::parse_from_str(week_of, "%Y-%m-%d")
            .unwrap_or_else(|_| chrono::Utc::now().date_naive());
        cq = cq.bind(week_date);
    }

    // P2-D: same fault-masking issue — a failed COUNT must not silently
    // report total:0 alongside an otherwise-successful page.
    let total: (i64,) = cq.fetch_one(&state.db.pool).await.map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": e.to_string()})),
        )
    })?;

    Ok(Json(json!({
        "success": true,
        "data": lists,
        "meta": {
            "current_page": page,
            "per_page": per_page,
            "total": total.0,
            "last_page": ((total.0 as f64) / (per_page as f64)).ceil() as i64
        }
    })))
}

/// GET /api/room-resources/stock-lists/:id - Get single stock list
pub(super) async fn get_stock_list(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let list: StockList = sqlx::query_as("SELECT * FROM stock_lists WHERE id = $1")
        .bind(id)
        .fetch_optional(&state.db.pool)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": e.to_string()})),
            )
        })?
        .ok_or_else(|| {
            (
                StatusCode::NOT_FOUND,
                Json(json!({"error": "Stock list not found"})),
            )
        })?;

    Ok(Json(json!({
        "success": true,
        "data": list
    })))
}

/// GET /api/room-resources/stock-lists/latest/:room_id - Get latest watchlist for room
pub(super) async fn get_latest_watchlist(
    State(state): State<AppState>,
    Path(room_id): Path<i64>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let list: Option<StockList> = sqlx::query_as(
        "SELECT * FROM stock_lists WHERE trading_room_id = $1 AND list_type = 'watchlist' AND is_active = true ORDER BY week_of DESC NULLS LAST, created_at DESC LIMIT 1"
    )
    .bind(room_id)
    .fetch_optional(&state.db.pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()}))))?;

    Ok(Json(json!({
        "success": true,
        "data": list
    })))
}

// ═══════════════════════════════════════════════════════════════════════════
// ICT 7 NEW: RECENTLY ACCESSED TRACKING
// ═══════════════════════════════════════════════════════════════════════════

#[derive(Debug, Serialize, FromRow)]
pub struct RecentlyAccessed {
    pub id: i64,
    pub user_id: i64,
    pub resource_id: i64,
    pub resource_type: String,
    pub resource_title: String,
    pub resource_thumbnail: Option<String>,
    pub accessed_at: chrono::NaiveDateTime,
}

/// POST /api/room-resources/:id/track-access - Track resource access (requires auth)
pub(super) async fn track_resource_access(
    State(state): State<AppState>,
    Path(id): Path<i64>,
    headers: axum::http::HeaderMap,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    // Extract user ID from auth header/session (simplified - in production use proper auth)
    let user_id: Option<i64> = headers
        .get("x-user-id")
        .and_then(|v| v.to_str().ok())
        .and_then(|s| s.parse().ok());

    if let Some(uid) = user_id {
        // Get resource info
        let resource: Option<(String, String, Option<String>)> = sqlx::query_as(
            "SELECT resource_type, title, thumbnail_url FROM room_resources WHERE id = $1",
        )
        .bind(id)
        .fetch_optional(&state.db.pool)
        .await
        .unwrap_or(None);

        if let Some((resource_type, title, thumbnail)) = resource {
            // Upsert into recently_accessed
            let _ = sqlx::query(
                r"
                INSERT INTO resource_access_log (user_id, resource_id, resource_type, resource_title, resource_thumbnail)
                VALUES ($1, $2, $3, $4, $5)
                ON CONFLICT (user_id, resource_id) DO UPDATE SET accessed_at = NOW()
                "
            )
            .bind(uid)
            .bind(id)
            .bind(&resource_type)
            .bind(&title)
            .bind(&thumbnail)
            .execute(&state.db.pool)
            .await;

            // Increment view count
            let _ = sqlx::query(
                "UPDATE room_resources SET views_count = views_count + 1 WHERE id = $1",
            )
            .bind(id)
            .execute(&state.db.pool)
            .await;
        }
    }

    Ok(Json(json!({"success": true})))
}

/// GET /api/room-resources/recently-accessed - Get user's recently accessed resources
pub(super) async fn get_recently_accessed(
    State(state): State<AppState>,
    headers: axum::http::HeaderMap,
    Query(params): Query<std::collections::HashMap<String, String>>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let user_id: Option<i64> = headers
        .get("x-user-id")
        .and_then(|v| v.to_str().ok())
        .and_then(|s| s.parse().ok());

    let limit: i64 = params
        .get("limit")
        .and_then(|s| s.parse().ok())
        .unwrap_or(10)
        .min(50);

    if let Some(uid) = user_id {
        let recent: Vec<RecentlyAccessed> = sqlx::query_as(
            "SELECT * FROM resource_access_log WHERE user_id = $1 ORDER BY accessed_at DESC LIMIT $2"
        )
        .bind(uid)
        .bind(limit)
        .fetch_all(&state.db.pool)
        .await
        .unwrap_or_default();

        Ok(Json(json!({
            "success": true,
            "data": recent
        })))
    } else {
        Ok(Json(json!({
            "success": true,
            "data": []
        })))
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// ICT 7 NEW: RESOURCE FAVORITES
// ═══════════════════════════════════════════════════════════════════════════

/// POST /api/room-resources/:id/favorite - Add to favorites
pub(super) async fn add_resource_favorite(
    State(state): State<AppState>,
    Path(id): Path<i64>,
    headers: axum::http::HeaderMap,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let user_id: i64 = headers
        .get("x-user-id")
        .and_then(|v| v.to_str().ok())
        .and_then(|s| s.parse().ok())
        .ok_or_else(|| {
            (
                StatusCode::UNAUTHORIZED,
                Json(json!({"error": "Authentication required"})),
            )
        })?;

    let result = sqlx::query(
        "INSERT INTO resource_favorites (user_id, resource_id) VALUES ($1, $2) ON CONFLICT DO NOTHING"
    )
    .bind(user_id)
    .bind(id)
    .execute(&state.db.pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()}))))?;

    Ok(Json(json!({
        "success": true,
        "added": result.rows_affected() > 0,
        "message": if result.rows_affected() > 0 { "Added to favorites" } else { "Already in favorites" }
    })))
}

/// DELETE /api/room-resources/:id/favorite - Remove from favorites
pub(super) async fn remove_resource_favorite(
    State(state): State<AppState>,
    Path(id): Path<i64>,
    headers: axum::http::HeaderMap,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let user_id: i64 = headers
        .get("x-user-id")
        .and_then(|v| v.to_str().ok())
        .and_then(|s| s.parse().ok())
        .ok_or_else(|| {
            (
                StatusCode::UNAUTHORIZED,
                Json(json!({"error": "Authentication required"})),
            )
        })?;

    let result =
        sqlx::query("DELETE FROM resource_favorites WHERE user_id = $1 AND resource_id = $2")
            .bind(user_id)
            .bind(id)
            .execute(&state.db.pool)
            .await
            .map_err(|e| {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({"error": e.to_string()})),
                )
            })?;

    Ok(Json(json!({
        "success": true,
        "removed": result.rows_affected() > 0,
        "message": if result.rows_affected() > 0 { "Removed from favorites" } else { "Not in favorites" }
    })))
}

/// GET /api/room-resources/:id/favorite - Check if favorited
pub(super) async fn check_resource_favorite(
    State(state): State<AppState>,
    Path(id): Path<i64>,
    headers: axum::http::HeaderMap,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let user_id: Option<i64> = headers
        .get("x-user-id")
        .and_then(|v| v.to_str().ok())
        .and_then(|s| s.parse().ok());

    if let Some(uid) = user_id {
        let exists: bool = sqlx::query_scalar(
            "SELECT EXISTS(SELECT 1 FROM resource_favorites WHERE user_id = $1 AND resource_id = $2)"
        )
        .bind(uid)
        .bind(id)
        .fetch_one(&state.db.pool)
        .await
        .unwrap_or(false);

        Ok(Json(json!({
            "success": true,
            "is_favorited": exists
        })))
    } else {
        Ok(Json(json!({
            "success": true,
            "is_favorited": false
        })))
    }
}

/// GET /api/room-resources/favorites - Get user's favorite resources
pub(super) async fn get_favorite_resources(
    State(state): State<AppState>,
    headers: axum::http::HeaderMap,
    Query(params): Query<std::collections::HashMap<String, String>>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let user_id: i64 = headers
        .get("x-user-id")
        .and_then(|v| v.to_str().ok())
        .and_then(|s| s.parse().ok())
        .ok_or_else(|| {
            (
                StatusCode::UNAUTHORIZED,
                Json(json!({"error": "Authentication required"})),
            )
        })?;

    let page: i64 = params
        .get("page")
        .and_then(|s| s.parse().ok())
        .unwrap_or(1)
        .max(1);
    let per_page: i64 = params
        .get("per_page")
        .and_then(|s| s.parse().ok())
        .unwrap_or(20)
        .min(50);
    let offset = (page - 1) * per_page;

    let resources: Vec<RoomResource> = sqlx::query_as(
        r"
        SELECT r.* FROM room_resources r
        INNER JOIN resource_favorites f ON r.id = f.resource_id
        WHERE f.user_id = $1 AND r.deleted_at IS NULL
        ORDER BY f.created_at DESC
        LIMIT $2 OFFSET $3
        ",
    )
    .bind(user_id)
    .bind(per_page)
    .bind(offset)
    .fetch_all(&state.db.pool)
    .await
    .unwrap_or_default();

    let total: (i64,) =
        sqlx::query_as("SELECT COUNT(*) FROM resource_favorites WHERE user_id = $1")
            .bind(user_id)
            .fetch_one(&state.db.pool)
            .await
            .unwrap_or((0,));

    let responses: Vec<ResourceResponse> =
        resources.into_iter().map(resource_to_response).collect();

    Ok(Json(json!({
        "success": true,
        "data": responses,
        "meta": {
            "current_page": page,
            "per_page": per_page,
            "total": total.0,
            "last_page": ((total.0 as f64) / (per_page as f64)).ceil() as i64
        }
    })))
}
