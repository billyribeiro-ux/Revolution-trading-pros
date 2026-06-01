//! Room resources admin routes (split from `room_resources.rs`
//! lines 625-1073, 1546-1650, 1901-2021, 2322-2560).
//!
//! Covers admin CRUD on `room_resources` rows plus the admin-only
//! companion endpoints: upload-limits / validate-upload, stock-list
//! mutations, analytics, and download logs.

use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Json,
};
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use serde_json::json;
use sqlx::FromRow;

use crate::middleware::admin::AdminUser;
use crate::AppState;

use super::helpers::{resource_to_response, slugify};
use super::{
    CreateResourceRequest, CreateStockListRequest, PaginationMeta, ResourceListQuery,
    ResourceResponse, RoomResource, StockList, UpdateResourceRequest,
};

// ═══════════════════════════════════════════════════════════════════════════
// ADMIN ROUTES
// ═══════════════════════════════════════════════════════════════════════════

/// GET /api/admin/room-resources - List all resources (admin)
pub(super) async fn admin_list_resources(
    _admin: AdminUser,
    State(state): State<AppState>,
    Query(query): Query<ResourceListQuery>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let page = query.page.unwrap_or(1).max(1);
    let per_page = query.per_page.unwrap_or(50).min(100);
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
    if query.is_published.is_some() {
        conditions.push(format!("is_published = ${param_idx}"));
        param_idx += 1;
    }
    if query.section.is_some() {
        conditions.push(format!("section = ${param_idx}"));
        param_idx += 1;
    }
    if query.search.is_some() {
        conditions.push(format!(
            "(title ILIKE ${param_idx} OR description ILIKE ${param_idx})"
        ));
        param_idx += 1;
    }

    let where_clause = if conditions.is_empty() {
        String::new()
    } else {
        format!(" AND {}", conditions.join(" AND "))
    };

    let sql = format!(
        "SELECT * FROM room_resources WHERE deleted_at IS NULL{} ORDER BY created_at DESC LIMIT ${} OFFSET ${}",
        where_clause, param_idx, param_idx + 1
    );
    let count_sql =
        format!("SELECT COUNT(*) FROM room_resources WHERE deleted_at IS NULL{where_clause}");

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
    if let Some(is_published) = query.is_published {
        q = q.bind(is_published);
    }
    if let Some(ref section) = query.section {
        q = q.bind(section);
    }
    if let Some(ref search) = query.search {
        let search_pattern = format!("%{search}%");
        q = q.bind(search_pattern);
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
    if let Some(is_published) = query.is_published {
        cq = cq.bind(is_published);
    }
    if let Some(ref section) = query.section {
        cq = cq.bind(section);
    }
    if let Some(ref search) = query.search {
        let search_pattern = format!("%{search}%");
        cq = cq.bind(search_pattern);
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

/// POST /api/admin/room-resources - Create resource
pub(super) async fn create_resource(
    _admin: AdminUser,
    State(state): State<AppState>,
    Json(input): Json<CreateResourceRequest>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let slug = slugify(&input.title);
    let resource_date = input
        .resource_date
        .as_ref()
        .and_then(|d| NaiveDate::parse_from_str(d, "%Y-%m-%d").ok())
        .unwrap_or_else(|| chrono::Utc::now().date_naive());

    let tags_json = input
        .tags
        .as_ref()
        .map(|t| serde_json::to_value(t).unwrap_or(json!([])))
        .unwrap_or(json!([]));

    let resource: RoomResource = sqlx::query_as(
        r"
        INSERT INTO room_resources (
            title, slug, description, resource_type, content_type,
            file_url, mime_type, file_size, video_platform, bunny_video_guid,
            bunny_library_id, duration, thumbnail_url, width, height,
            trading_room_id, trader_id, resource_date, is_published, is_featured,
            is_pinned, category, tags, difficulty_level
        ) VALUES (
            $1, $2, $3, $4, $5, $6, $7, $8, $9, $10,
            $11, $12, $13, $14, $15, $16, $17, $18, $19, $20,
            $21, $22, $23, $24
        )
        RETURNING *
        ",
    )
    .bind(&input.title)
    .bind(&slug)
    .bind(&input.description)
    .bind(&input.resource_type)
    .bind(&input.content_type)
    .bind(&input.file_url)
    .bind(&input.mime_type)
    .bind(input.file_size)
    .bind(&input.video_platform)
    .bind(&input.bunny_video_guid)
    .bind(input.bunny_library_id)
    .bind(input.duration)
    .bind(&input.thumbnail_url)
    .bind(input.width)
    .bind(input.height)
    .bind(input.trading_room_id)
    .bind(input.trader_id)
    .bind(resource_date)
    .bind(input.is_published.unwrap_or(false))
    .bind(input.is_featured.unwrap_or(false))
    .bind(input.is_pinned.unwrap_or(false))
    .bind(&input.category)
    .bind(tags_json)
    .bind(&input.difficulty_level)
    .fetch_one(&state.db.pool)
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": e.to_string()})),
        )
    })?;

    Ok(Json(json!({
        "success": true,
        "message": "Resource created successfully",
        "data": resource_to_response(resource)
    })))
}

/// PUT /api/admin/room-resources/:id - Update resource
pub(super) async fn update_resource(
    _admin: AdminUser,
    State(state): State<AppState>,
    Path(id): Path<i64>,
    Json(input): Json<UpdateResourceRequest>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    // Build dynamic update query
    let mut updates = Vec::new();
    let mut param_idx = 1;

    if input.title.is_some() {
        updates.push(format!("title = ${param_idx}"));
        param_idx += 1;
    }
    if input.description.is_some() {
        updates.push(format!("description = ${param_idx}"));
        param_idx += 1;
    }
    if input.resource_type.is_some() {
        updates.push(format!("resource_type = ${param_idx}"));
        param_idx += 1;
    }
    if input.content_type.is_some() {
        updates.push(format!("content_type = ${param_idx}"));
        param_idx += 1;
    }
    if input.file_url.is_some() {
        updates.push(format!("file_url = ${param_idx}"));
        param_idx += 1;
    }
    if input.mime_type.is_some() {
        updates.push(format!("mime_type = ${param_idx}"));
        param_idx += 1;
    }
    if input.file_size.is_some() {
        updates.push(format!("file_size = ${param_idx}"));
        param_idx += 1;
    }
    if input.video_platform.is_some() {
        updates.push(format!("video_platform = ${param_idx}"));
        param_idx += 1;
    }
    if input.bunny_video_guid.is_some() {
        updates.push(format!("bunny_video_guid = ${param_idx}"));
        param_idx += 1;
    }
    if input.bunny_library_id.is_some() {
        updates.push(format!("bunny_library_id = ${param_idx}"));
        param_idx += 1;
    }
    if input.duration.is_some() {
        updates.push(format!("duration = ${param_idx}"));
        param_idx += 1;
    }
    if input.thumbnail_url.is_some() {
        updates.push(format!("thumbnail_url = ${param_idx}"));
        param_idx += 1;
    }
    if input.width.is_some() {
        updates.push(format!("width = ${param_idx}"));
        param_idx += 1;
    }
    if input.height.is_some() {
        updates.push(format!("height = ${param_idx}"));
        param_idx += 1;
    }
    if input.trader_id.is_some() {
        updates.push(format!("trader_id = ${param_idx}"));
        param_idx += 1;
    }
    if input.resource_date.is_some() {
        updates.push(format!("resource_date = ${param_idx}"));
        param_idx += 1;
    }
    if input.is_published.is_some() {
        updates.push(format!("is_published = ${param_idx}"));
        param_idx += 1;
    }
    if input.is_featured.is_some() {
        updates.push(format!("is_featured = ${param_idx}"));
        param_idx += 1;
    }
    if input.is_pinned.is_some() {
        updates.push(format!("is_pinned = ${param_idx}"));
        param_idx += 1;
    }
    if input.category.is_some() {
        updates.push(format!("category = ${param_idx}"));
        param_idx += 1;
    }
    if input.tags.is_some() {
        updates.push(format!("tags = ${param_idx}"));
        param_idx += 1;
    }
    if input.difficulty_level.is_some() {
        updates.push(format!("difficulty_level = ${param_idx}"));
        param_idx += 1;
    }

    if updates.is_empty() {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(json!({"error": "No fields to update"})),
        ));
    }

    let query_str = format!(
        "UPDATE room_resources SET {}, updated_at = NOW() WHERE id = ${} AND deleted_at IS NULL RETURNING *",
        updates.join(", "),
        param_idx
    );

    let mut query = sqlx::query_as::<_, RoomResource>(&query_str);

    // Bind parameters in order
    if let Some(ref v) = input.title {
        query = query.bind(v);
    }
    if let Some(ref v) = input.description {
        query = query.bind(v);
    }
    if let Some(ref v) = input.resource_type {
        query = query.bind(v);
    }
    if let Some(ref v) = input.content_type {
        query = query.bind(v);
    }
    if let Some(ref v) = input.file_url {
        query = query.bind(v);
    }
    if let Some(ref v) = input.mime_type {
        query = query.bind(v);
    }
    if let Some(v) = input.file_size {
        query = query.bind(v);
    }
    if let Some(ref v) = input.video_platform {
        query = query.bind(v);
    }
    if let Some(ref v) = input.bunny_video_guid {
        query = query.bind(v);
    }
    if let Some(v) = input.bunny_library_id {
        query = query.bind(v);
    }
    if let Some(v) = input.duration {
        query = query.bind(v);
    }
    if let Some(ref v) = input.thumbnail_url {
        query = query.bind(v);
    }
    if let Some(v) = input.width {
        query = query.bind(v);
    }
    if let Some(v) = input.height {
        query = query.bind(v);
    }
    if let Some(v) = input.trader_id {
        query = query.bind(v);
    }
    if let Some(ref v) = input.resource_date {
        let date = NaiveDate::parse_from_str(v, "%Y-%m-%d")
            .unwrap_or_else(|_| chrono::Utc::now().date_naive());
        query = query.bind(date);
    }
    if let Some(v) = input.is_published {
        query = query.bind(v);
    }
    if let Some(v) = input.is_featured {
        query = query.bind(v);
    }
    if let Some(v) = input.is_pinned {
        query = query.bind(v);
    }
    if let Some(ref v) = input.category {
        query = query.bind(v);
    }
    if let Some(ref v) = input.tags {
        let tags_json = serde_json::to_value(v).unwrap_or(json!([]));
        query = query.bind(tags_json);
    }
    if let Some(ref v) = input.difficulty_level {
        query = query.bind(v);
    }

    query = query.bind(id);

    let resource = query
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
                Json(json!({"error": "Resource not found"})),
            )
        })?;

    Ok(Json(json!({
        "success": true,
        "message": "Resource updated successfully",
        "data": resource_to_response(resource)
    })))
}

/// DELETE /api/admin/room-resources/:id - Soft delete resource
pub(super) async fn delete_resource(
    _admin: AdminUser,
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let result = sqlx::query(
        "UPDATE room_resources SET deleted_at = NOW() WHERE id = $1 AND deleted_at IS NULL",
    )
    .bind(id)
    .execute(&state.db.pool)
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": e.to_string()})),
        )
    })?;

    if result.rows_affected() == 0 {
        return Err((
            StatusCode::NOT_FOUND,
            Json(json!({"error": "Resource not found"})),
        ));
    }

    Ok(Json(json!({
        "success": true,
        "message": "Resource deleted successfully"
    })))
}

// ═══════════════════════════════════════════════════════════════════════════
// ICT 7 NEW: FILE UPLOAD LIMITS
// ═══════════════════════════════════════════════════════════════════════════

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct UploadLimit {
    pub resource_type: String,
    pub max_file_size_bytes: i64,
    pub allowed_mime_types: Vec<String>,
    pub allowed_extensions: Vec<String>,
    pub requires_premium: bool,
}

/// GET /api/admin/room-resources/upload-limits - Get file upload limits
pub(super) async fn get_upload_limits(
    _admin: AdminUser,
    State(state): State<AppState>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let limits: Vec<UploadLimit> = sqlx::query_as(
        "SELECT resource_type, max_file_size_bytes, allowed_mime_types, allowed_extensions, requires_premium FROM resource_upload_limits ORDER BY resource_type"
    )
    .fetch_all(&state.db.pool)
    .await
    .unwrap_or_default();

    Ok(Json(json!({
        "success": true,
        "data": limits
    })))
}

/// POST /api/admin/room-resources/validate-upload - Validate file before upload
pub(super) async fn validate_upload(
    _admin: AdminUser,
    State(state): State<AppState>,
    Json(input): Json<ValidateUploadRequest>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    // Get limits for this resource type
    let limit: Option<UploadLimit> = sqlx::query_as(
        "SELECT resource_type, max_file_size_bytes, allowed_mime_types, allowed_extensions, requires_premium FROM resource_upload_limits WHERE resource_type = $1"
    )
    .bind(&input.resource_type)
    .fetch_optional(&state.db.pool)
    .await
    .unwrap_or(None);

    let limit = match limit {
        Some(l) => l,
        None => {
            return Ok(Json(json!({
                "success": true,
                "valid": true,
                "message": "No restrictions for this resource type"
            })));
        }
    };

    // Validate file size
    if input.file_size > limit.max_file_size_bytes {
        let max_mb = limit.max_file_size_bytes / 1_048_576;
        return Ok(Json(json!({
            "success": true,
            "valid": false,
            "error": format!("File size exceeds maximum of {} MB", max_mb)
        })));
    }

    // Validate MIME type
    if let Some(ref mime) = input.mime_type {
        if !limit.allowed_mime_types.contains(mime) {
            return Ok(Json(json!({
                "success": true,
                "valid": false,
                "error": format!("MIME type '{}' is not allowed. Allowed: {:?}", mime, limit.allowed_mime_types)
            })));
        }
    }

    // Validate extension
    if let Some(ref ext) = input.extension {
        let ext_lower = ext.to_lowercase();
        if !limit.allowed_extensions.contains(&ext_lower) {
            return Ok(Json(json!({
                "success": true,
                "valid": false,
                "error": format!("Extension '.{}' is not allowed. Allowed: {:?}", ext, limit.allowed_extensions)
            })));
        }
    }

    Ok(Json(json!({
        "success": true,
        "valid": true,
        "message": "File is valid for upload",
        "requires_premium": limit.requires_premium
    })))
}

#[derive(Debug, Deserialize)]
pub struct ValidateUploadRequest {
    pub resource_type: String,
    pub file_size: i64,
    pub mime_type: Option<String>,
    pub extension: Option<String>,
}

// ═══════════════════════════════════════════════════════════════════════════
// ICT 7 NEW: STOCK LISTS (ADMIN MUTATIONS)
// ═══════════════════════════════════════════════════════════════════════════

/// POST /api/admin/room-resources/stock-lists - Create stock list
pub(super) async fn create_stock_list(
    _admin: AdminUser,
    State(state): State<AppState>,
    Json(input): Json<CreateStockListRequest>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let slug = slugify(&input.name);
    let symbols_json = serde_json::to_value(&input.symbols).unwrap_or(json!([]));
    let week_of = input
        .week_of
        .as_ref()
        .and_then(|d| chrono::NaiveDate::parse_from_str(d, "%Y-%m-%d").ok());

    let list: StockList = sqlx::query_as(
        r"
        INSERT INTO stock_lists (name, slug, description, list_type, trading_room_id, symbols, is_active, is_featured, week_of)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
        RETURNING *
        "
    )
    .bind(&input.name)
    .bind(&slug)
    .bind(&input.description)
    .bind(&input.list_type)
    .bind(input.trading_room_id)
    .bind(symbols_json)
    .bind(input.is_active.unwrap_or(true))
    .bind(input.is_featured.unwrap_or(false))
    .bind(week_of)
    .fetch_one(&state.db.pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()}))))?;

    Ok(Json(json!({
        "success": true,
        "message": "Stock list created successfully",
        "data": list
    })))
}

/// PUT /api/admin/room-resources/stock-lists/:id - Update stock list
pub(super) async fn update_stock_list(
    _admin: AdminUser,
    State(state): State<AppState>,
    Path(id): Path<i64>,
    Json(input): Json<CreateStockListRequest>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let symbols_json = serde_json::to_value(&input.symbols).unwrap_or(json!([]));
    let week_of = input
        .week_of
        .as_ref()
        .and_then(|d| chrono::NaiveDate::parse_from_str(d, "%Y-%m-%d").ok());

    let list: StockList = sqlx::query_as(
        r"
        UPDATE stock_lists SET
            name = $1, description = $2, list_type = $3, symbols = $4,
            is_active = $5, is_featured = $6, week_of = $7, updated_at = NOW()
        WHERE id = $8
        RETURNING *
        ",
    )
    .bind(&input.name)
    .bind(&input.description)
    .bind(&input.list_type)
    .bind(symbols_json)
    .bind(input.is_active.unwrap_or(true))
    .bind(input.is_featured.unwrap_or(false))
    .bind(week_of)
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
        "message": "Stock list updated successfully",
        "data": list
    })))
}

/// DELETE /api/admin/room-resources/stock-lists/:id - Delete stock list
pub(super) async fn delete_stock_list(
    _admin: AdminUser,
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let result = sqlx::query("DELETE FROM stock_lists WHERE id = $1")
        .bind(id)
        .execute(&state.db.pool)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": e.to_string()})),
            )
        })?;

    if result.rows_affected() == 0 {
        return Err((
            StatusCode::NOT_FOUND,
            Json(json!({"error": "Stock list not found"})),
        ));
    }

    Ok(Json(json!({
        "success": true,
        "message": "Stock list deleted successfully"
    })))
}

// ═══════════════════════════════════════════════════════════════════════════
// ICT 7 NEW: RESOURCE ANALYTICS
// ═══════════════════════════════════════════════════════════════════════════

#[derive(Debug, Serialize)]
pub struct ResourceAnalytics {
    pub total_resources: i64,
    pub total_views: i64,
    pub total_downloads: i64,
    pub total_favorites: i64,
    pub by_type: Vec<TypeStats>,
    pub by_access_level: Vec<AccessStats>,
    pub top_viewed: Vec<ResourceStats>,
    pub top_downloaded: Vec<ResourceStats>,
    pub recent_uploads: Vec<ResourceStats>,
}

#[derive(Debug, Serialize, FromRow)]
pub struct TypeStats {
    pub resource_type: String,
    pub count: i64,
    pub total_views: i64,
    pub total_downloads: i64,
}

#[derive(Debug, Serialize, FromRow)]
pub struct AccessStats {
    pub access_level: String,
    pub count: i64,
}

#[derive(Debug, Serialize, FromRow)]
pub struct ResourceStats {
    pub id: i64,
    pub title: String,
    pub resource_type: String,
    pub views_count: i32,
    pub downloads_count: i32,
    pub created_at: chrono::NaiveDateTime,
}

/// GET /api/admin/room-resources/analytics - Get resource analytics
pub(super) async fn get_resource_analytics(
    _admin: AdminUser,
    State(state): State<AppState>,
    Query(params): Query<std::collections::HashMap<String, String>>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let room_id: Option<i64> = params.get("room_id").and_then(|s| s.parse().ok());

    // Use parameterized queries to prevent SQL injection
    let room_filter = if room_id.is_some() {
        " AND trading_room_id = $1"
    } else {
        ""
    };

    // Total counts
    let totals_sql = format!(
        "SELECT COUNT(*), COALESCE(SUM(views_count), 0), COALESCE(SUM(downloads_count), 0) FROM room_resources WHERE deleted_at IS NULL{room_filter}"
    );
    let mut totals_q = sqlx::query_as::<_, (i64, i64, i64)>(&totals_sql);
    if let Some(rid) = room_id {
        totals_q = totals_q.bind(rid);
    }
    let totals: (i64, i64, i64) = totals_q
        .fetch_one(&state.db.pool)
        .await
        .unwrap_or((0, 0, 0));

    let total_favorites: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM resource_favorites")
        .fetch_one(&state.db.pool)
        .await
        .unwrap_or((0,));

    // By type
    let by_type_sql = format!(
        "SELECT resource_type, COUNT(*) as count, COALESCE(SUM(views_count), 0) as total_views, COALESCE(SUM(downloads_count), 0) as total_downloads FROM room_resources WHERE deleted_at IS NULL{room_filter} GROUP BY resource_type ORDER BY count DESC"
    );
    let mut by_type_q = sqlx::query_as::<_, TypeStats>(&by_type_sql);
    if let Some(rid) = room_id {
        by_type_q = by_type_q.bind(rid);
    }
    let by_type: Vec<TypeStats> = by_type_q
        .fetch_all(&state.db.pool)
        .await
        .unwrap_or_default();

    // By access level
    let by_access_sql = format!(
        "SELECT COALESCE(access_level, 'premium') as access_level, COUNT(*) as count FROM room_resources WHERE deleted_at IS NULL{room_filter} GROUP BY access_level ORDER BY count DESC"
    );
    let mut by_access_q = sqlx::query_as::<_, AccessStats>(&by_access_sql);
    if let Some(rid) = room_id {
        by_access_q = by_access_q.bind(rid);
    }
    let by_access_level: Vec<AccessStats> = by_access_q
        .fetch_all(&state.db.pool)
        .await
        .unwrap_or_default();

    // Top viewed
    let top_viewed_sql = format!(
        "SELECT id, title, resource_type, views_count, downloads_count, created_at FROM room_resources WHERE deleted_at IS NULL{room_filter} ORDER BY views_count DESC LIMIT 10"
    );
    let mut top_viewed_q = sqlx::query_as::<_, ResourceStats>(&top_viewed_sql);
    if let Some(rid) = room_id {
        top_viewed_q = top_viewed_q.bind(rid);
    }
    let top_viewed: Vec<ResourceStats> = top_viewed_q
        .fetch_all(&state.db.pool)
        .await
        .unwrap_or_default();

    // Top downloaded
    let top_downloaded_sql = format!(
        "SELECT id, title, resource_type, views_count, downloads_count, created_at FROM room_resources WHERE deleted_at IS NULL{room_filter} ORDER BY downloads_count DESC LIMIT 10"
    );
    let mut top_downloaded_q = sqlx::query_as::<_, ResourceStats>(&top_downloaded_sql);
    if let Some(rid) = room_id {
        top_downloaded_q = top_downloaded_q.bind(rid);
    }
    let top_downloaded: Vec<ResourceStats> = top_downloaded_q
        .fetch_all(&state.db.pool)
        .await
        .unwrap_or_default();

    // Recent uploads
    let recent_uploads_sql = format!(
        "SELECT id, title, resource_type, views_count, downloads_count, created_at FROM room_resources WHERE deleted_at IS NULL{room_filter} ORDER BY created_at DESC LIMIT 10"
    );
    let mut recent_uploads_q = sqlx::query_as::<_, ResourceStats>(&recent_uploads_sql);
    if let Some(rid) = room_id {
        recent_uploads_q = recent_uploads_q.bind(rid);
    }
    let recent_uploads: Vec<ResourceStats> = recent_uploads_q
        .fetch_all(&state.db.pool)
        .await
        .unwrap_or_default();

    let analytics = ResourceAnalytics {
        total_resources: totals.0,
        total_views: totals.1,
        total_downloads: totals.2,
        total_favorites: total_favorites.0,
        by_type,
        by_access_level,
        top_viewed,
        top_downloaded,
        recent_uploads,
    };

    Ok(Json(json!({
        "success": true,
        "data": analytics
    })))
}

/// GET /api/admin/room-resources/download-logs - Get download logs
pub(super) async fn get_download_logs(
    _admin: AdminUser,
    State(state): State<AppState>,
    Query(params): Query<std::collections::HashMap<String, String>>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let resource_id: Option<i64> = params.get("resource_id").and_then(|s| s.parse().ok());
    let page: i64 = params
        .get("page")
        .and_then(|s| s.parse().ok())
        .unwrap_or(1)
        .max(1);
    let per_page: i64 = params
        .get("per_page")
        .and_then(|s| s.parse().ok())
        .unwrap_or(50)
        .min(100);
    let offset = (page - 1) * per_page;

    let (logs, total): (Vec<serde_json::Value>, (i64,)) = if let Some(rid) = resource_id {
        #[allow(clippy::type_complexity)]
        let logs: Vec<(i64, i64, Option<i64>, Option<String>, chrono::DateTime<chrono::Utc>)> = sqlx::query_as(
            "SELECT id, resource_id, user_id, ip_address::TEXT, downloaded_at FROM resource_download_logs WHERE resource_id = $1 ORDER BY downloaded_at DESC LIMIT $2 OFFSET $3"
        )
        .bind(rid)
        .bind(per_page)
        .bind(offset)
        .fetch_all(&state.db.pool)
        .await
        .unwrap_or_default();

        let total: (i64,) =
            sqlx::query_as("SELECT COUNT(*) FROM resource_download_logs WHERE resource_id = $1")
                .bind(rid)
                .fetch_one(&state.db.pool)
                .await
                .unwrap_or((0,));

        let json_logs: Vec<serde_json::Value> = logs.into_iter().map(|(id, rid, uid, ip, ts)| {
            json!({"id": id, "resource_id": rid, "user_id": uid, "ip_address": ip, "downloaded_at": ts})
        }).collect();

        (json_logs, total)
    } else {
        #[allow(clippy::type_complexity)]
        let logs: Vec<(i64, i64, Option<i64>, Option<String>, chrono::DateTime<chrono::Utc>)> = sqlx::query_as(
            "SELECT id, resource_id, user_id, ip_address::TEXT, downloaded_at FROM resource_download_logs ORDER BY downloaded_at DESC LIMIT $1 OFFSET $2"
        )
        .bind(per_page)
        .bind(offset)
        .fetch_all(&state.db.pool)
        .await
        .unwrap_or_default();

        let total: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM resource_download_logs")
            .fetch_one(&state.db.pool)
            .await
            .unwrap_or((0,));

        let json_logs: Vec<serde_json::Value> = logs.into_iter().map(|(id, rid, uid, ip, ts)| {
            json!({"id": id, "resource_id": rid, "user_id": uid, "ip_address": ip, "downloaded_at": ts})
        }).collect();

        (json_logs, total)
    };

    Ok(Json(json!({
        "success": true,
        "data": logs,
        "meta": {
            "current_page": page,
            "per_page": per_page,
            "total": total.0
        }
    })))
}
