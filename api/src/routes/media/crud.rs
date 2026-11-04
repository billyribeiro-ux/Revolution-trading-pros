//! Media CRUD — list, show, update, destroy, and library statistics.
//!
//! R27-B split: index/show/update/destroy mirror the standard REST shape;
//! `statistics` lives here because it's a read-only summary over the
//! same table and uses the shared `format_bytes` helper.

#![allow(clippy::useless_format)]
#![allow(clippy::double_ended_iterator_last)]

use axum::{
    extract::{Path, Query, State},
    response::Json,
};

use super::dto::{Media, MediaQuery, UpdateMedia};
use crate::{AppState, middleware::admin::AdminUser, utils::errors::ApiError};

/// GET /admin/media - List all media with filtering and pagination
/// ICT 7 SECURITY: AdminUser authentication required, parameterized queries
#[tracing::instrument(skip(state, _admin))]
pub(super) async fn index(
    State(state): State<AppState>,
    _admin: AdminUser,
    Query(params): Query<MediaQuery>,
) -> Result<Json<serde_json::Value>, ApiError> {
    // ICT 7 SECURITY FIX: Use parameterized queries to prevent SQL injection
    let mut conditions = Vec::new();
    let mut bind_values: Vec<String> = Vec::new();
    let mut param_idx = 1;

    // Search filter - parameterized
    if let Some(search) = &params.search {
        // Sanitize search input - only allow alphanumeric, spaces, and common chars
        let sanitized: String = search
            .chars()
            .filter(|c| c.is_alphanumeric() || *c == ' ' || *c == '-' || *c == '_' || *c == '.')
            .take(100) // Limit length
            .collect();
        if !sanitized.is_empty() {
            conditions.push(format!(
                "(original_filename ILIKE ${0} OR filename ILIKE ${0} OR title ILIKE ${1} OR alt_text ILIKE ${2})",
                param_idx,
                param_idx + 1,
                param_idx + 2
            ));
            let pattern = format!("%{sanitized}%");
            bind_values.push(pattern.clone());
            bind_values.push(pattern.clone());
            bind_values.push(pattern);
            param_idx += 3;
        }
    }

    // Type filter - whitelist validation
    if let Some(media_type) = &params.r#type {
        if media_type == "document" {
            conditions.push("(mime_type LIKE 'application/%' OR mime_type LIKE 'text/%')".into());
        }
        let allowed_types = ["image", "video", "audio", "application", "text"];
        if allowed_types.contains(&media_type.as_str()) {
            conditions.push(format!("mime_type LIKE ${param_idx}"));
            bind_values.push(format!("{media_type}%"));
            param_idx += 1;
        }
    }

    // Collection filter - parameterized
    if let Some(collection) = &params.collection {
        // Sanitize collection name
        let sanitized: String = collection
            .chars()
            .filter(|c| c.is_alphanumeric() || *c == '-' || *c == '_')
            .take(50)
            .collect();
        if !sanitized.is_empty() {
            conditions.push(format!("collection = ${param_idx}"));
            bind_values.push(sanitized);
            #[allow(unused_assignments)]
            {
                param_idx += 1;
            } // Keep for extensibility
        }
    }

    // Images only filter - no user input, safe
    if params.images_only.unwrap_or(false) {
        conditions.push("mime_type LIKE 'image/%'".to_string());
    }

    // Optimization filter - boolean, safe
    if let Some(is_optimized) = params.is_optimized {
        conditions.push(format!("COALESCE(is_optimized,FALSE) = {is_optimized}"));
    }
    if params.needs_optimization == Some(true) {
        conditions.push("mime_type LIKE 'image/%' AND NOT COALESCE(is_optimized,FALSE)".into());
    }
    if let Some(status) = &params.processing_status {
        if !matches!(
            status.as_str(),
            "pending" | "processing" | "completed" | "failed" | "cancelled"
        ) {
            return Err(ApiError::validation_error("Invalid processing status"));
        }
        // Value is an exhaustive status whitelist; latest attempt controls the filter.
        conditions.push(format!("(SELECT status FROM media_optimization_jobs WHERE media_id=media.id ORDER BY id DESC LIMIT 1) = '{status}'"));
    }

    // Sorting - whitelist validation (no user input in query)
    let allowed_columns = [
        "filename",
        "title",
        "size",
        "created_at",
        "updated_at",
        "id",
    ];
    let sort_by = params.sort_by.as_deref().unwrap_or("created_at");
    let sort_dir = params.sort_dir.as_deref().unwrap_or("desc");

    let sort_column = if allowed_columns.contains(&sort_by) {
        sort_by
    } else {
        "created_at"
    };
    let sort_direction = if sort_dir.eq_ignore_ascii_case("asc") {
        "ASC"
    } else {
        "DESC"
    };

    // Pagination - validated integers
    let per_page = params.per_page.unwrap_or(24).clamp(1, 100);
    let page = params.page.unwrap_or(1);
    if !(1..=1_000_000).contains(&page) {
        return Err(ApiError::validation_error("Page must be from 1 to 1000000"));
    }
    let offset = (page - 1) * per_page;

    // Build query with conditions
    let where_clause = if conditions.is_empty() {
        String::new()
    } else {
        format!(" AND {}", conditions.join(" AND "))
    };

    // Get total count with parameterized query
    let count_query = format!("SELECT COUNT(*) FROM media WHERE 1=1{where_clause}");

    let mut count_q = sqlx::query_scalar::<_, i64>(sqlx::AssertSqlSafe(count_query.as_str()));
    for val in &bind_values {
        count_q = count_q.bind(val);
    }

    let total: i64 = count_q
        .fetch_one(state.db.pool())
        .await
        .map_err(|e| ApiError::database_error(&e.to_string()))?;

    // Build main query.
    //
    // SAFETY: `sort_column` and `sort_direction` are checked against the
    // `allowed_columns` allow-list and a binary ASC/DESC choice above.
    // Format-string injection through them is therefore impossible. LIMIT
    // and OFFSET are bound as parameters below — never formatted in.
    let query = format!(
        "SELECT id, filename, original_filename, mime_type, size, path, url,
                title, alt_text, caption, description, collection, is_optimized,
                width, height, scan_status, scan_result, created_at, updated_at
         FROM media WHERE 1=1{} ORDER BY {} {} LIMIT ${} OFFSET ${}",
        where_clause,
        sort_column,
        sort_direction,
        bind_values.len() + 1,
        bind_values.len() + 2,
    );

    let mut main_q = sqlx::query_as::<_, Media>(sqlx::AssertSqlSafe(query.as_str()));
    for val in &bind_values {
        main_q = main_q.bind(val);
    }
    main_q = main_q.bind(per_page).bind(offset);

    let media: Vec<Media> = main_q
        .fetch_all(state.db.pool())
        .await
        .map_err(|e| ApiError::database_error(&e.to_string()))?;

    let last_page = ((total as f64 / per_page as f64).ceil() as i64).max(1);

    Ok(Json(serde_json::json!({
        "success": true,
        "data": super::helpers::present(&state, media).await?,
        "meta": {
            "current_page": page,
            "last_page": last_page,
            "per_page": per_page,
            "total": total
        }
    })))
}

/// GET /admin/media/:id - Get single media item
/// ICT 7 SECURITY: AdminUser authentication required
#[tracing::instrument(skip(state, _admin))]
pub(super) async fn show(
    State(state): State<AppState>,
    _admin: AdminUser,
    Path(id): Path<i64>,
) -> Result<Json<serde_json::Value>, ApiError> {
    let media: Option<Media> = sqlx::query_as(
        "SELECT id, filename, original_filename, mime_type, size, path, url,
                title, alt_text, caption, description, collection, is_optimized,
                width, height, scan_status, scan_result, created_at, updated_at
         FROM media WHERE id = $1",
    )
    .bind(id)
    .fetch_optional(state.db.pool())
    .await
    .map_err(|e| ApiError::database_error(&e.to_string()))?;

    match media {
        Some(m) => Ok(Json(
            serde_json::json!({ "success": true, "data": super::helpers::present(&state, vec![m]).await?.pop() }),
        )),
        None => Err(ApiError::not_found("Media not found")),
    }
}

/// PUT /admin/media/:id - Update media metadata (SEO fields)
/// ICT 7 SECURITY: AdminUser authentication required
#[tracing::instrument(skip(state, _admin))]
pub(super) async fn update(
    State(state): State<AppState>,
    _admin: AdminUser,
    Path(id): Path<i64>,
    Json(payload): Json<UpdateMedia>,
) -> Result<Json<serde_json::Value>, ApiError> {
    super::helpers::validate_metadata(&payload)?;
    let media: Media = sqlx::query_as("UPDATE media SET title=COALESCE($2,title),alt_text=COALESCE($3,alt_text),caption=COALESCE($4,caption),description=COALESCE($5,description),collection=COALESCE($6,collection),tags=COALESCE($7,tags),updated_at=NOW() WHERE id=$1 RETURNING id,filename,original_filename,mime_type,size,path,url,title,alt_text,caption,description,collection,is_optimized,width,height,scan_status,scan_result,created_at,updated_at")
        .bind(id).bind(payload.title).bind(payload.alt_text).bind(payload.caption).bind(payload.description).bind(payload.collection).bind(payload.tags.map(|tags|serde_json::json!(tags)))
        .fetch_optional(state.db.pool()).await.map_err(|e|ApiError::database_error(&e.to_string()))?.ok_or_else(||ApiError::not_found("Media not found"))?;
    Ok(Json(
        serde_json::json!({"success":true,"data":super::helpers::present(&state,vec![media]).await?.pop(),"message":"Media updated"}),
    ))
}

/// DELETE /admin/media/:id - Delete media item
/// ICT 7+ ENHANCEMENT: Complete file deletion from R2 storage
/// ICT 7 SECURITY: AdminUser authentication required
#[tracing::instrument(skip(state, _admin))]
pub(super) async fn destroy(
    State(state): State<AppState>,
    _admin: AdminUser,
    Path(id): Path<i64>,
) -> Result<Json<serde_json::Value>, ApiError> {
    let media = super::helpers::delete_media(&state, id).await?;

    Ok(Json(serde_json::json!({
        "success": true,
        "message": "Media deleted successfully",
        "deleted": {
            "id": media.id,
            "filename": media.filename,
            "path": media.path
        }
    })))
}

/// GET /admin/media/statistics - Get media library statistics
/// ICT 7 SECURITY: AdminUser authentication required
#[tracing::instrument(skip(state, _admin))]
pub(super) async fn statistics(
    State(state): State<AppState>,
    _admin: AdminUser,
) -> Result<Json<serde_json::Value>, ApiError> {
    let data: serde_json::Value = sqlx::query_scalar(r"
        WITH media_totals AS (
            SELECT COUNT(*) total, COALESCE(SUM(size),0) bytes,
                COUNT(*) FILTER(WHERE mime_type LIKE 'image/%') images,
                COUNT(*) FILTER(WHERE mime_type LIKE 'video/%') videos,
                COUNT(*) FILTER(WHERE mime_type LIKE 'application/%' OR mime_type LIKE 'text/%') documents,
                COUNT(*) FILTER(WHERE mime_type LIKE 'image/%' AND is_optimized) optimized
            FROM media
        ), variants AS (SELECT COUNT(*) count,COALESCE(SUM(file_size),0) bytes FROM media_variants), physical AS (SELECT COALESCE(SUM(bytes),0) bytes FROM (SELECT path,MAX(bytes) bytes FROM (SELECT path,size bytes FROM media UNION ALL SELECT snapshot->>'path',(snapshot->>'size')::bigint FROM media_versions UNION ALL SELECT path,file_size FROM media_variants) objects WHERE path IS NOT NULL GROUP BY path) stored),
        latest AS (SELECT DISTINCT ON(j.media_id) original_size,optimized_size FROM media_optimization_jobs j JOIN media m ON m.id=j.media_id AND m.content_version=j.source_version WHERE j.status='completed' ORDER BY j.media_id,j.id DESC),
        jobs AS (SELECT jsonb_build_object('pending',COUNT(*) FILTER(WHERE status='pending'),'processing',COUNT(*) FILTER(WHERE status='processing'),'completed',COUNT(*) FILTER(WHERE status='completed'),'failed',COUNT(*) FILTER(WHERE status='failed'),'total_savings_bytes',COALESCE(SUM(original_size-optimized_size) FILTER(WHERE status='completed'),0),'avg_processing_time_ms',COALESCE(AVG(processing_time_ms) FILTER(WHERE status='completed'),0)) data FROM media_optimization_jobs),
        types AS (SELECT split_part(COALESCE(mime_type,'other'),'/',1) type,COUNT(*) count,COALESCE(SUM(size),0) bytes FROM media GROUP BY 1)
        SELECT jsonb_build_object(
            'total_files',m.total,'total_count',m.total,'total_size',m.bytes,'total_storage',(SELECT bytes FROM physical),
            'total_images',m.images,'total_videos',m.videos,'total_documents',m.documents,
            'optimized_images',m.optimized,'needs_optimization',m.images-m.optimized,'pending_optimization',m.images-m.optimized,
            'total_variants',v.count,'total_savings_bytes',(SELECT COALESCE(SUM(original_size-optimized_size),0) FROM latest),
            'total_downloads',NULL,'total_views',NULL,'unused_files',NULL,
            'storage_by_type',(SELECT COALESCE(jsonb_object_agg(type,jsonb_build_object('count',count,'size',bytes,'human_size',pg_size_pretty(bytes))), '{}') FROM types),
            'job_stats',jobs.data,
            'by_type',jsonb_build_object('images',m.images,'videos',m.videos,'documents',m.documents),
            'optimization',jsonb_build_object('optimized',m.optimized,'unoptimized',m.images-m.optimized,'percentage',CASE WHEN m.images>0 THEN m.optimized*100/m.images ELSE 0 END)
        ) FROM media_totals m CROSS JOIN variants v CROSS JOIN jobs
    ").fetch_one(state.db.pool()).await.map_err(|e|ApiError::database_error(&e.to_string()))?;
    Ok(Json(serde_json::json!({"success":true,"data":data})))
}
