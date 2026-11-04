//! Media helpers — magic-byte validation and human-readable byte formatting.
//!
//! R27-B split: these are private utilities shared by the upload and
//! statistics handlers. They have no router-facing surface, so they
//! stay `pub(super)`.

pub(super) async fn delete_media(
    state: &crate::AppState,
    id: i64,
) -> Result<super::dto::Media, crate::utils::errors::ApiError> {
    use crate::utils::errors::ApiError;
    let db = |error: sqlx::Error| ApiError::database_error(&error.to_string());
    let mut tx = state.db.pool().begin().await.map_err(db)?;
    let media: super::dto::Media = sqlx::query_as("SELECT id,filename,original_filename,mime_type,size,path,url,title,alt_text,caption,description,collection,is_optimized,width,height,scan_status,scan_result,created_at,updated_at FROM media WHERE id=$1 FOR UPDATE")
        .bind(id).fetch_optional(&mut *tx).await.map_err(db)?.ok_or_else(||ApiError::not_found("Media not found"))?;
    let deletion = async {
        crate::services::media_jobs::delete_variants(state, &mut tx, id).await?;
        if let Some(path) = &media.path {
            state.services.storage.delete(path).await?;
        }
        Ok::<(), anyhow::Error>(())
    }
    .await;
    if let Err(error) = deletion {
        tracing::warn!(media_id=id,%error,"Storage deletion failed; media retained");
        return Err(ApiError::new(
            axum::http::StatusCode::SERVICE_UNAVAILABLE,
            "Storage deletion failed; media record retained so deletion can be retried",
        ));
    }
    sqlx::query("DELETE FROM media WHERE id=$1")
        .bind(id)
        .execute(&mut *tx)
        .await
        .map_err(db)?;
    tx.commit().await.map_err(db)?;
    Ok(media)
}

/// ICT 7 SECURITY: Validate file content matches declared MIME type
/// Checks magic bytes/file signatures to prevent content-type spoofing
pub(super) fn validate_file_signature(data: &[u8], content_type: &str) -> bool {
    if data.is_empty() {
        return false;
    }

    match content_type {
        // Images
        "image/jpeg" => data.len() >= 2 && data[0] == 0xFF && data[1] == 0xD8,
        "image/png" => {
            data.len() >= 8 && data[..8] == [0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A]
        }
        "image/gif" => data.len() >= 6 && (data[..6] == *b"GIF87a" || data[..6] == *b"GIF89a"),
        "image/webp" => data.len() >= 12 && data[..4] == *b"RIFF" && data[8..12] == *b"WEBP",
        "image/svg+xml" => {
            // SVG is text-based, check for XML or SVG tag
            let text = String::from_utf8_lossy(&data[..data.len().min(1000)]);
            text.contains("<?xml") || text.contains("<svg") || text.contains("<SVG")
        }

        // Videos
        "video/mp4" => {
            // MP4/M4V: ftyp atom
            data.len() >= 12
                && (data[4..8] == *b"ftyp" || data[4..8] == *b"moov" || data[4..8] == *b"mdat")
        }
        "video/webm" => {
            // WebM: EBML header
            data.len() >= 4 && data[..4] == [0x1A, 0x45, 0xDF, 0xA3]
        }
        "video/quicktime" => {
            // QuickTime: ftyp or moov atom
            data.len() >= 8
                && (data[4..8] == *b"ftyp"
                    || data[4..8] == *b"moov"
                    || data[4..8] == *b"wide"
                    || data[4..8] == *b"free")
        }

        // Documents
        "application/pdf" => data.len() >= 5 && data[..5] == *b"%PDF-",
        "application/msword" => {
            // DOC: OLE Compound Document
            data.len() >= 8 && data[..8] == [0xD0, 0xCF, 0x11, 0xE0, 0xA1, 0xB1, 0x1A, 0xE1]
        }
        "application/vnd.openxmlformats-officedocument.wordprocessingml.document" => {
            // DOCX: ZIP-based (PK signature)
            data.len() >= 4 && data[..4] == *b"PK\x03\x04"
        }
        "application/vnd.ms-excel" => {
            // XLS: OLE Compound Document
            data.len() >= 8 && data[..8] == [0xD0, 0xCF, 0x11, 0xE0, 0xA1, 0xB1, 0x1A, 0xE1]
        }
        "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet" => {
            // XLSX: ZIP-based
            data.len() >= 4 && data[..4] == *b"PK\x03\x04"
        }

        // Text
        "text/plain" | "text/csv" => {
            // Basic text validation: mostly printable ASCII or valid UTF-8
            String::from_utf8(data[..data.len().min(1000)].to_vec()).is_ok()
        }

        // Unknown type - reject by default
        _ => false,
    }
}

/// Format bytes to human readable
pub(super) fn format_bytes(bytes: i64) -> String {
    const KB: i64 = 1024;
    const MB: i64 = KB * 1024;
    const GB: i64 = MB * 1024;

    if bytes >= GB {
        format!("{:.2} GB", bytes as f64 / GB as f64)
    } else if bytes >= MB {
        format!("{:.2} MB", bytes as f64 / MB as f64)
    } else if bytes >= KB {
        format!("{:.2} KB", bytes as f64 / KB as f64)
    } else {
        format!("{bytes} B")
    }
}
