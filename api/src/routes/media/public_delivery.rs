//! Public, opaque image selector. The source object remains the fallback while
//! a scanned derivative is pending or when the browser lacks format support.
use axum::{
    extract::{Path, Query, State},
    http::{HeaderMap, HeaderValue, StatusCode, header},
};
use serde::Deserialize;
use uuid::Uuid;

use crate::{AppState, utils::errors::ApiError};

#[derive(Deserialize)]
pub(super) struct ImageQuery {
    width: Option<i32>,
}

pub(super) async fn image(
    State(state): State<AppState>,
    Path(token): Path<Uuid>,
    Query(query): Query<ImageQuery>,
    request_headers: HeaderMap,
) -> Result<(StatusCode, HeaderMap), ApiError> {
    let requested_width = match query.width {
        Some(width) if (1..=4096).contains(&width) => Some(width),
        None => None,
        _ => return Err(ApiError::validation_error("Width must be from 1 to 4096")),
    };
    let row: Option<(i64, String, i64, String, i64)> = sqlx::query_as(
        "SELECT id,path,size,mime_type,content_version FROM media WHERE public_token=$1 AND scan_status='clean' AND mime_type IN ('image/jpeg','image/png','image/gif','image/webp','image/avif') AND path IS NOT NULL AND size IS NOT NULL",
    )
    .bind(token)
    .fetch_optional(state.db.pool())
    .await
    .map_err(|error| ApiError::database_error(&error.to_string()))?;
    let (media_id, original_path, original_size, _mime, source_version) =
        row.ok_or_else(|| ApiError::not_found("Image not found"))?;
    let accept = request_headers
        .get(header::ACCEPT)
        .and_then(|value| value.to_str().ok())
        .unwrap_or_default();
    let accepts_webp = accepts_format(accept, "image/webp");
    let accepts_avif = accepts_format(accept, "image/avif");
    let chosen: Option<String> = if accepts_webp || accepts_avif {
        sqlx::query_scalar(
            "SELECT path FROM media_variants WHERE media_id=$1 AND source_version=$2 AND file_size<$3 AND ((format='webp' AND $4) OR (format='avif' AND $5)) AND ($6::int IS NOT NULL OR size='full') ORDER BY CASE WHEN $6::int IS NULL OR width >= $6 THEN 0 ELSE 1 END, CASE WHEN $6::int IS NULL OR width >= $6 THEN width ELSE -width END, file_size ASC LIMIT 1",
        )
        .bind(media_id)
        .bind(source_version)
        .bind(original_size)
        .bind(accepts_webp)
        .bind(accepts_avif)
        .bind(requested_width)
        .fetch_optional(state.db.pool())
        .await
        .map_err(|error| ApiError::database_error(&error.to_string()))?
    } else {
        None
    };
    let path = chosen.unwrap_or(original_path);
    if !valid_object_key(&path) || state.config.r2_public_url.trim().is_empty() {
        return Err(ApiError::new(
            StatusCode::SERVICE_UNAVAILABLE,
            "Image storage is unavailable",
        ));
    }
    let mut public_url = reqwest::Url::parse(&state.config.r2_public_url)
        .map_err(|_| ApiError::internal_error("Invalid image storage URL"))?;
    if !matches!(public_url.scheme(), "http" | "https")
        || public_url.host_str().is_none()
        || public_url.query().is_some()
        || public_url.fragment().is_some()
        || !public_url.username().is_empty()
        || public_url.password().is_some()
    {
        return Err(ApiError::internal_error("Invalid image storage URL"));
    }
    public_url
        .path_segments_mut()
        .map_err(|_| ApiError::internal_error("Invalid image storage URL"))?
        .pop_if_empty()
        .extend(path.split('/'));
    let location = HeaderValue::from_str(public_url.as_str())
        .map_err(|_| ApiError::internal_error("Invalid image location"))?;
    let mut headers = HeaderMap::new();
    headers.insert(header::LOCATION, location);
    headers.insert(
        header::CACHE_CONTROL,
        HeaderValue::from_static("public, max-age=30, s-maxage=30"),
    );
    headers.insert(header::VARY, HeaderValue::from_static("Accept"));
    headers.insert(
        "x-content-type-options",
        HeaderValue::from_static("nosniff"),
    );
    Ok((StatusCode::FOUND, headers))
}

fn valid_object_key(path: &str) -> bool {
    path.contains('/')
        && !path.chars().any(char::is_control)
        && !path.contains('\\')
        && path
            .split('/')
            .all(|part| !part.is_empty() && part != "." && part != "..")
}

fn accepts_format(accept: &str, format: &str) -> bool {
    accept.split(',').any(|entry| {
        let mut parts = entry.trim().split(';');
        if parts
            .next()
            .is_none_or(|media_type| media_type.trim() != format)
        {
            return false;
        }
        parts.all(|parameter| {
            let parameter = parameter.trim();
            !parameter.starts_with("q=")
                || parameter[2..]
                    .parse::<f32>()
                    .is_ok_and(|quality| quality > 0.0 && quality <= 1.0)
        })
    })
}

#[cfg(test)]
mod tests {
    use super::{accepts_format, valid_object_key};

    #[test]
    fn rejects_unacceptable_formats_and_unsafe_storage_keys() {
        assert!(accepts_format("image/avif,image/webp;q=0.8", "image/webp"));
        assert!(!accepts_format("image/webp;q=0", "image/webp"));
        assert!(!accepts_format("image/png,*/*", "image/webp"));
        assert!(valid_object_key(
            "uploads/01234567-89ab-cdef-0123-456789abcdef.png"
        ));
        assert!(valid_object_key("blog/fixture.png"));
        assert!(!valid_object_key("uploads/../secret"));
        assert!(!valid_object_key("https://evil.example/x"));
    }
}
