//! Authenticated, bounded byte responses for previews and downloads. No remote
//! URL supplied by a caller is fetched; every key comes from a media record.
use crate::{AppState, middleware::admin::AdminUser, utils::errors::ApiError};
use axum::{
    body::Bytes,
    extract::{Path, Query, State},
    http::{HeaderMap, HeaderValue, StatusCode},
};
use serde::Deserialize;

#[derive(Deserialize)]
pub(super) struct ContentQuery {
    variant_id: Option<i64>,
    version_id: Option<i64>,
}
pub(super) async fn content(
    State(state): State<AppState>,
    _admin: AdminUser,
    Path(id): Path<i64>,
    Query(query): Query<ContentQuery>,
) -> Result<(HeaderMap, Bytes), ApiError> {
    let db = |error: sqlx::Error| ApiError::database_error(&error.to_string());
    let row: Option<(Option<String>, Option<String>, String)> =
        sqlx::query_as("SELECT path,mime_type,scan_status FROM media WHERE id=$1")
            .bind(id)
            .fetch_optional(state.db.pool())
            .await
            .map_err(db)?;
    let (mut path, mut mime, mut scan) =
        row.ok_or_else(|| ApiError::not_found("Media not found"))?;
    if query.variant_id.is_some() && query.version_id.is_some() {
        return Err(ApiError::validation_error("Choose a variant or a version"));
    }
    if let Some(version_id) = query.version_id {
        let version:Option<(Option<String>,Option<String>,Option<String>)>=sqlx::query_as("SELECT snapshot->>'path',snapshot->>'mime_type',CASE WHEN v.version_number=m.content_version THEN m.scan_status ELSE snapshot->>'scan_status' END FROM media_versions v JOIN media m ON m.id=v.media_id WHERE v.id=$1 AND v.media_id=$2")
            .bind(version_id).bind(id).fetch_optional(state.db.pool()).await.map_err(db)?;
        let (stored_path, stored_mime, stored_scan) =
            version.ok_or_else(|| ApiError::not_found("Media version not found"))?;
        path = stored_path;
        mime = stored_mime;
        scan = stored_scan.unwrap_or_else(|| "unknown".into());
    } else if let Some(variant_id) = query.variant_id {
        let row:Option<(String,String)>=sqlx::query_as("SELECT path,format FROM media_variants WHERE id=$1 AND media_id=$2 AND source_version=(SELECT content_version FROM media WHERE id=$2)").bind(variant_id).bind(id).fetch_optional(state.db.pool()).await.map_err(db)?;
        let (stored_path, format) =
            row.ok_or_else(|| ApiError::not_found("Media variant not found"))?;
        path = Some(stored_path);
        mime = Some(format!("image/{format}"));
    }
    if scan == "infected" {
        return Err(ApiError::new(
            StatusCode::FORBIDDEN,
            "Infected media is quarantined",
        ));
    }
    let path = path.ok_or_else(|| ApiError::not_found("Media has no stored object"))?;
    let mime = mime.unwrap_or_else(|| "application/octet-stream".into());
    let bytes = state
        .services
        .storage
        .download_for_scan(&path)
        .await
        .map_err(|error| {
            tracing::warn!(media_id=id,%error,"Cannot read media bytes");
            ApiError::new(
                StatusCode::SERVICE_UNAVAILABLE,
                "Stored media is unavailable",
            )
        })?;
    // Active document formats are always attachments, including unknown legacy
    // MIME values. No script-capable MIME can execute on the application origin.
    let inline = matches!(
        mime.as_str(),
        "image/png"
            | "image/jpeg"
            | "image/webp"
            | "image/gif"
            | "image/avif"
            | "video/mp4"
            | "video/webm"
            | "video/quicktime"
            | "audio/mpeg"
            | "audio/ogg"
            | "audio/wav"
    );
    let mut headers = HeaderMap::new();
    headers.insert(
        "content-type",
        HeaderValue::from_str(&mime)
            .map_err(|_| ApiError::internal_error("Invalid stored media type"))?,
    );
    headers.insert(
        "cache-control",
        HeaderValue::from_static("private, no-store"),
    );
    headers.insert(
        "x-content-type-options",
        HeaderValue::from_static("nosniff"),
    );
    headers.insert(
        "content-disposition",
        HeaderValue::from_static(if inline { "inline" } else { "attachment" }),
    );
    headers.insert(
        "x-media-scan-status",
        HeaderValue::from_str(&scan)
            .map_err(|_| ApiError::internal_error("Invalid stored scan status"))?,
    );
    Ok((headers, Bytes::from(bytes)))
}
