//! Scan actual stored objects through ClamAV and persist truthful verdicts.

use axum::{
    extract::{Path, State},
    response::Json,
};

use super::dto::{MalwareScanResult, Media};
use crate::{AppState, middleware::admin::AdminUser, services::malware, utils::errors::ApiError};

pub(super) async fn scan_for_malware(data: &[u8]) -> MalwareScanResult {
    let address = std::env::var("CLAMD_ADDRESS").ok();
    malware::scan(address.as_deref(), data).await
}

pub(super) async fn persist_result(
    state: &AppState,
    id: i64,
    result: &MalwareScanResult,
) -> Result<(), ApiError> {
    sqlx::query(
        "UPDATE media SET scan_status = $2, scan_result = $3, updated_at = NOW() WHERE id = $1",
    )
    .bind(id)
    .bind(&result.status)
    .bind(
        serde_json::to_value(result)
            .map_err(|_| ApiError::internal_error("Cannot serialize scan result"))?,
    )
    .execute(state.db.pool())
    .await
    .map_err(|e| ApiError::database_error(&e.to_string()))?;
    Ok(())
}

pub(super) async fn scan_stored(
    state: &AppState,
    media: &Media,
) -> Result<MalwareScanResult, ApiError> {
    // Claim the scan, preventing overlapping results from replacing a newer
    // verdict. A pending row with no result is an upload awaiting its first scan.
    let claimed = sqlx::query("UPDATE media SET scan_status = 'pending', scan_result = '{\"status\":\"pending\",\"is_clean\":false}'::jsonb, updated_at = NOW() WHERE id = $1 AND NOT (scan_status = 'pending' AND scan_result IS NOT NULL AND updated_at > NOW() - INTERVAL '2 minutes')")
        .bind(media.id).execute(state.db.pool()).await
        .map_err(|e| ApiError::database_error(&e.to_string()))?;
    if claimed.rows_affected() == 0 {
        return Err(ApiError::new(
            axum::http::StatusCode::CONFLICT,
            "A scan is already pending",
        ));
    }
    let address = std::env::var("CLAMD_ADDRESS")
        .ok()
        .filter(|s| !s.trim().is_empty());
    let result = if address.is_none() {
        malware::unsuccessful("unavailable", "CLAMD_ADDRESS is not configured")
    } else if let Some(path) = &media.path {
        match state.services.storage.download_for_scan(path).await {
            Ok(data) => malware::scan(address.as_deref(), &data).await,
            Err(error) => {
                tracing::warn!(media_id = media.id, %error, "Cannot retrieve object for scanning");
                malware::unsuccessful("failed", "Cannot retrieve stored object for scanning")
            }
        }
    } else {
        malware::unsuccessful("failed", "Media has no storage object key")
    };
    persist_result(state, media.id, &result).await?;
    // Direct uploads reject infected bytes before storage. For a previously
    // uploaded object, remove its publicly accessible copy on detection.
    if result.status == "infected" {
        if let Some(path) = &media.path {
            state.services.storage.delete(path).await.map_err(|error| {
                tracing::error!(media_id = media.id, %error, "Infected object removal failed");
                ApiError::internal_error("Threat detected; object removal failed")
            })?;
        }
        sqlx::query("UPDATE media SET url = NULL WHERE id = $1")
            .bind(media.id)
            .execute(state.db.pool())
            .await
            .map_err(|e| ApiError::database_error(&e.to_string()))?;
    }
    Ok(result)
}

/// POST /admin/media/scan/{id}; GET /admin/media/{id} exposes persisted status.
pub(super) async fn scan_media_file(
    State(state): State<AppState>,
    _admin: AdminUser,
    Path(id): Path<i64>,
) -> Result<Json<serde_json::Value>, ApiError> {
    let media: Media = sqlx::query_as(
        "SELECT id, filename, original_filename, mime_type, size, path, url,
                title, alt_text, caption, description, collection, is_optimized,
                width, height, scan_status, scan_result, created_at, updated_at
         FROM media WHERE id = $1",
    )
    .bind(id)
    .fetch_optional(state.db.pool())
    .await
    .map_err(|e| ApiError::database_error(&e.to_string()))?
    .ok_or_else(|| ApiError::not_found("Media not found"))?;
    let scan_result = scan_stored(&state, &media).await?;
    Ok(Json(serde_json::json!({
        "success": matches!(scan_result.status.as_str(), "clean" | "infected"),
        "data": { "media_id": id, "filename": media.filename, "scan_result": scan_result },
        "message": match scan_result.status.as_str() {
            "clean" => "File scanned; no threats detected",
            "infected" => "Threat detected; stored object removed",
            "unavailable" => "Scanner unavailable; file has not been scanned",
            _ => "Scan failed; file has no clean verdict"
        }
    })))
}
