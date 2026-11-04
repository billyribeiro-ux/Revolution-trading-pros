use crate::{
    AppState, middleware::admin::AdminUser, services::media_jobs::Preset, utils::errors::ApiError,
};
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::Json,
};
use serde::Deserialize;
use serde_json::{Value, json};

fn db(error: sqlx::Error) -> ApiError {
    ApiError::database_error(&error.to_string())
}
fn unavailable() -> ApiError {
    ApiError::new(
        StatusCode::SERVICE_UNAVAILABLE,
        "Image processing is not configured; set MEDIA_PROCESSOR_URL and MEDIA_PROCESSOR_TOKEN",
    )
}

#[derive(Default, Deserialize)]
pub(super) struct Options {
    preset: Option<String>,
    priority: Option<i32>,
    limit: Option<i64>,
}
#[derive(Deserialize)]
pub(super) struct BulkOptions {
    ids: Vec<i64>,
    #[serde(flatten)]
    options: Options,
}

pub(super) async fn health(
    State(state): State<AppState>,
    _admin: AdminUser,
) -> Result<Json<Value>, ApiError> {
    let processor = state
        .services
        .media_processor
        .as_ref()
        .ok_or_else(unavailable)?;
    let health = processor.health().await.map_err(|error| {
        tracing::warn!(%error,"Processor health check failed");
        ApiError::new(
            StatusCode::SERVICE_UNAVAILABLE,
            "Image processor is unavailable",
        )
    })?;
    Ok(Json(
        json!({"sharp_service":{"available":health.available,"engine":health.engine,"version":health.version},"success":true}),
    ))
}
pub(super) async fn presets(
    State(state): State<AppState>,
    _admin: AdminUser,
) -> Result<Json<Value>, ApiError> {
    let rows: Vec<Preset> =
        sqlx::query_as("SELECT * FROM media_optimization_presets ORDER BY is_default DESC,id")
            .fetch_all(state.db.pool())
            .await
            .map_err(db)?;
    Ok(Json(json!({"success":true,"data":rows})))
}
pub(super) async fn preset(state: &AppState, slug: Option<&str>) -> Result<Preset, ApiError> {
    let row: Preset = sqlx::query_as("SELECT * FROM media_optimization_presets WHERE ($1::text IS NULL AND is_default) OR slug=$1 LIMIT 1")
        .bind(slug).fetch_optional(state.db.pool()).await.map_err(db)?.ok_or_else(||ApiError::validation_error("Unknown optimization preset"))?;
    row.validate()
        .map_err(|e| ApiError::validation_error(&e.to_string()))?;
    Ok(row)
}
async fn enqueue(
    state: &AppState,
    media_id: i64,
    preset: &Preset,
    priority: i32,
) -> Result<i64, ApiError> {
    if state.services.media_processor.is_none() {
        return Err(unavailable());
    }
    if !(0..=10).contains(&priority) {
        return Err(ApiError::validation_error("Priority must be from 0 to 10"));
    }
    let mut tx = state.db.pool().begin().await.map_err(db)?;
    let media: Option<(Option<String>, Option<String>, String, i64)> = sqlx::query_as(
        "SELECT mime_type,path,scan_status,content_version FROM media WHERE id=$1 FOR UPDATE",
    )
    .bind(media_id)
    .fetch_optional(&mut *tx)
    .await
    .map_err(db)?;
    let (mime, path, scan, source_version) =
        media.ok_or_else(|| ApiError::not_found("Media not found"))?;
    if !matches!(
        mime.as_deref(),
        Some("image/jpeg" | "image/png" | "image/webp" | "image/gif" | "image/avif")
    ) || path.is_none()
    {
        return Err(ApiError::validation_error(
            "Only stored JPEG, PNG, GIF, WebP and AVIF images can be optimized",
        ));
    }
    if scan == "infected" {
        return Err(ApiError::validation_error(
            "Infected media cannot be processed",
        ));
    }
    let id: Option<i64> = sqlx::query_scalar("SELECT id FROM media_optimization_jobs WHERE media_id=$1 AND status IN ('pending','processing')")
        .bind(media_id).fetch_optional(&mut *tx).await.map_err(db)?;
    let id = if let Some(id) = id {
        id
    } else {
        sqlx::query_scalar("INSERT INTO media_optimization_jobs(media_id,preset,priority,source_version) VALUES($1,$2,$3,$4) RETURNING id")
            .bind(media_id).bind(json!(preset)).bind(priority).bind(source_version).fetch_one(&mut *tx).await.map_err(db)?
    };
    tx.commit().await.map_err(db)?;
    Ok(id)
}
async fn job_data(state: &AppState, id: i64) -> Result<Value, ApiError> {
    sqlx::query_scalar("SELECT to_jsonb(j) || jsonb_build_object('progress', CASE WHEN status='completed' THEN 100 ELSE 0 END,'current_operation',CASE WHEN status='processing' THEN 'Processing scanned image' ELSE NULL END,'savings_bytes',COALESCE(original_size-optimized_size,0),'savings_percent',CASE WHEN original_size>0 AND optimized_size IS NOT NULL THEN 100.0*(original_size-optimized_size)/original_size ELSE NULL END,'processing_time',processing_time_ms) FROM media_optimization_jobs j WHERE id=$1")
        .bind(id).fetch_optional(state.db.pool()).await.map_err(db)?.ok_or_else(||ApiError::not_found("Optimization job not found"))
}
pub(super) async fn optimize(
    State(state): State<AppState>,
    _admin: AdminUser,
    Path(id): Path<i64>,
    Json(options): Json<Options>,
) -> Result<(StatusCode, Json<Value>), ApiError> {
    let preset = preset(&state, options.preset.as_deref()).await?;
    let job = enqueue(&state, id, &preset, options.priority.unwrap_or(0)).await?;
    Ok((
        StatusCode::ACCEPTED,
        Json(
            json!({"success":true,"data":job_data(&state,job).await?,"message":"Image processing queued; original retained until completion"}),
        ),
    ))
}
pub(super) async fn job(
    State(state): State<AppState>,
    _admin: AdminUser,
    Path(id): Path<i64>,
) -> Result<Json<Value>, ApiError> {
    Ok(Json(
        json!({"success":true,"data":job_data(&state,id).await?}),
    ))
}
pub(super) async fn queue(
    State(state): State<AppState>,
    _admin: AdminUser,
) -> Result<Json<Value>, ApiError> {
    let data: Value = sqlx::query_scalar("SELECT jsonb_build_object('pending',COUNT(*) FILTER(WHERE status='pending'),'processing',COUNT(*) FILTER(WHERE status='processing'),'completed',COUNT(*) FILTER(WHERE status='completed'),'failed',COUNT(*) FILTER(WHERE status='failed'),'total_savings_bytes',COALESCE(SUM(original_size-optimized_size) FILTER(WHERE status='completed'),0),'avg_processing_time_ms',COALESCE(AVG(processing_time_ms) FILTER(WHERE status='completed'),0)) FROM media_optimization_jobs")
        .fetch_one(state.db.pool()).await.map_err(db)?;
    Ok(Json(json!({"success":true,"data":data})))
}
pub(super) async fn variants(
    State(state): State<AppState>,
    _admin: AdminUser,
    Path(id): Path<i64>,
) -> Result<Json<Value>, ApiError> {
    let scan: Option<String> = sqlx::query_scalar("SELECT scan_status FROM media WHERE id=$1")
        .bind(id)
        .fetch_optional(state.db.pool())
        .await
        .map_err(db)?;
    if scan.is_none() {
        return Err(ApiError::not_found("Media not found"));
    }
    if scan.as_deref() == Some("infected") {
        return Err(ApiError::validation_error("Infected media is quarantined"));
    }
    let rows: Vec<Value> = sqlx::query_scalar("SELECT to_jsonb(v)-'path' || jsonb_build_object('lqip',NULL,'blurhash',NULL) FROM media_variants v WHERE media_id=$1 AND source_version=(SELECT content_version FROM media WHERE id=$1) AND job_id=(SELECT MAX(job_id) FROM media_variants WHERE media_id=$1 AND source_version=(SELECT content_version FROM media WHERE id=$1)) ORDER BY width DESC,id")
        .bind(id).fetch_all(state.db.pool()).await.map_err(db)?;
    Ok(Json(json!({"success":true,"data":rows})))
}
pub(super) async fn bulk_optimize(
    State(state): State<AppState>,
    _admin: AdminUser,
    Json(options): Json<BulkOptions>,
) -> Result<Json<Value>, ApiError> {
    if options.ids.is_empty() || options.ids.len() > 100 {
        return Err(ApiError::validation_error("Choose 1 to 100 images"));
    }
    let preset = preset(&state, options.options.preset.as_deref()).await?;
    let mut queued = Vec::new();
    let mut errors = Vec::new();
    let mut seen = std::collections::HashSet::new();
    for id in options.ids {
        if !seen.insert(id) {
            continue;
        }
        match enqueue(&state, id, &preset, options.options.priority.unwrap_or(0)).await {
            Ok(job_id) => queued.push(json!({"media_id":id,"job_id":job_id})),
            Err(error) => errors.push(json!({"media_id":id,"message":error.message})),
        }
    }
    Ok(Json(
        json!({"success":queued.len(),"failed":errors.len(),"jobs":queued,"errors":errors}),
    ))
}
pub(super) async fn optimize_all(
    State(state): State<AppState>,
    _admin: AdminUser,
    Json(options): Json<Options>,
) -> Result<Json<Value>, ApiError> {
    if state.services.media_processor.is_none() {
        return Err(unavailable());
    }
    let preset = preset(&state, options.preset.as_deref()).await?;
    let limit = options.limit.unwrap_or(100);
    if !(1..=100).contains(&limit) {
        return Err(ApiError::validation_error("Limit must be from 1 to 100"));
    }
    let ids: Vec<i64> = sqlx::query_scalar("SELECT id FROM media WHERE mime_type IN ('image/jpeg','image/png','image/webp','image/gif','image/avif') AND NOT COALESCE(is_optimized,FALSE) AND scan_status<>'infected' AND path IS NOT NULL ORDER BY id LIMIT $1")
        .bind(limit).fetch_all(state.db.pool()).await.map_err(db)?;
    let mut jobs = Vec::new();
    for id in &ids {
        jobs.push(enqueue(&state, *id, &preset, options.priority.unwrap_or(0)).await?);
    }
    Ok(Json(
        json!({"success":true,"total_pending":ids.len(),"jobs_queued":jobs.len(),"job_ids":jobs,"message":"Image processing queued"}),
    ))
}
