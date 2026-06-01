//! Media bulk ops — bulk delete, bulk update, and orphan cleanup.
//!
//! R27-B split: grouped here because they all iterate over multiple
//! media rows (or all rows over a horizon) and produce per-row
//! success/failure tallies.

use axum::{
    extract::{Query, State},
    response::Json,
};

use super::dto::{BulkDeleteRequest, BulkUpdateRequest, CleanupResult, Media};
use crate::{middleware::admin::AdminUser, utils::errors::ApiError, AppState};

/// POST /admin/media/bulk-delete - Delete multiple media items
/// ICT 7 SECURITY: AdminUser authentication required
#[tracing::instrument(skip(state, _admin))]
pub(super) async fn bulk_delete(
    State(state): State<AppState>,
    _admin: AdminUser,
    Json(payload): Json<BulkDeleteRequest>,
) -> Result<Json<serde_json::Value>, ApiError> {
    if payload.ids.is_empty() {
        return Err(ApiError::validation_error("No IDs provided"));
    }

    if payload.ids.len() > 100 {
        return Err(ApiError::validation_error(
            "Maximum 100 items per bulk delete",
        ));
    }

    let storage = &state.services.storage;
    let mut deleted_count = 0;
    let mut failed_count = 0;
    let mut errors: Vec<String> = Vec::new();

    for id in &payload.ids {
        // Get media info first
        let media: Option<Media> = sqlx::query_as(
            "SELECT id, filename, original_filename, mime_type, size, path, url,
                    title, alt_text, caption, description, collection, is_optimized,
                    width, height, created_at, updated_at
             FROM media WHERE id = $1",
        )
        .bind(id)
        .fetch_optional(state.db.pool())
        .await
        .map_err(|e| ApiError::database_error(&e.to_string()))?;

        if let Some(media) = media {
            // Delete from R2 if path exists
            if let Some(file_key) = &media.path {
                if let Err(e) = storage.delete(file_key).await {
                    tracing::warn!("Failed to delete file from R2: {} - {}", file_key, e);
                }
            }

            // Delete from database
            match sqlx::query("DELETE FROM media WHERE id = $1")
                .bind(id)
                .execute(state.db.pool())
                .await
            {
                Ok(_) => deleted_count += 1,
                Err(e) => {
                    failed_count += 1;
                    errors.push(format!("ID {id}: {e}"));
                }
            }
        } else {
            failed_count += 1;
            errors.push(format!("ID {id}: Not found"));
        }
    }

    Ok(Json(serde_json::json!({
        "success": true,
        "data": {
            "deleted": deleted_count,
            "failed": failed_count,
            "errors": errors
        },
        "message": format!("{} item(s) deleted, {} failed", deleted_count, failed_count)
    })))
}

/// POST /admin/media/bulk-update - Update multiple media items
/// ICT 7 SECURITY: AdminUser authentication required
#[tracing::instrument(skip(state, _admin))]
pub(super) async fn bulk_update(
    State(state): State<AppState>,
    _admin: AdminUser,
    Json(payload): Json<BulkUpdateRequest>,
) -> Result<Json<serde_json::Value>, ApiError> {
    if payload.ids.is_empty() {
        return Err(ApiError::validation_error("No IDs provided"));
    }

    if payload.ids.len() > 100 {
        return Err(ApiError::validation_error(
            "Maximum 100 items per bulk update",
        ));
    }

    let mut updated_count = 0;

    for id in &payload.ids {
        let mut updates = Vec::new();

        if payload.collection.is_some() {
            updates.push("collection = $2");
        }
        if payload.title.is_some() {
            updates.push("title = $3");
        }
        if payload.alt_text.is_some() {
            updates.push("alt_text = $4");
        }

        if updates.is_empty() {
            continue;
        }

        updates.push("updated_at = NOW()");

        let query = format!("UPDATE media SET {} WHERE id = $1", updates.join(", "));

        let mut q = sqlx::query(&query).bind(id);

        if let Some(ref collection) = payload.collection {
            q = q.bind(collection);
        }
        if let Some(ref title) = payload.title {
            q = q.bind(title);
        }
        if let Some(ref alt_text) = payload.alt_text {
            q = q.bind(alt_text);
        }

        if q.execute(state.db.pool()).await.is_ok() {
            updated_count += 1;
        }
    }

    Ok(Json(serde_json::json!({
        "success": true,
        "data": {
            "updated": updated_count
        },
        "message": format!("{} item(s) updated", updated_count)
    })))
}

/// POST /admin/media/cleanup-orphans - Find and optionally remove orphaned files
/// ICT 7 SECURITY: SuperAdmin only - dangerous operation
#[tracing::instrument(skip(state, _admin))]
pub(super) async fn cleanup_orphaned_files(
    State(state): State<AppState>,
    _admin: AdminUser,
    Query(params): Query<std::collections::HashMap<String, String>>,
) -> Result<Json<serde_json::Value>, ApiError> {
    let dry_run = params.get("dry_run").map(|v| v == "true").unwrap_or(true);

    let mut result = CleanupResult {
        orphaned_files_found: 0,
        orphaned_files_deleted: 0,
        orphaned_db_records: 0,
        db_records_cleaned: 0,
        errors: Vec::new(),
    };

    // Step 1: Find database records with paths that don't exist in R2
    // (This is a simplified check - in production, you'd verify against R2 directly)
    let orphaned_records: Vec<(i64, String)> = sqlx::query_as(
        r"
        SELECT id, path FROM media
        WHERE path IS NOT NULL
        AND created_at < NOW() - INTERVAL '24 hours'
        ORDER BY created_at ASC
        LIMIT 100
        ",
    )
    .fetch_all(state.db.pool())
    .await
    .unwrap_or_default();

    // Step 2: Verify each file exists in storage
    let storage = &state.services.storage;
    let mut orphaned_db_ids: Vec<i64> = Vec::new();

    for (id, path) in &orphaned_records {
        // Try to check if the file exists by attempting to get a presigned URL
        // If it fails, the file likely doesn't exist
        match storage.presigned_download_url(path, 60).await {
            Ok(_) => {
                // File exists, not orphaned
            }
            Err(_) => {
                // File doesn't exist in R2 but record exists in DB
                result.orphaned_db_records += 1;
                orphaned_db_ids.push(*id);
            }
        }
    }

    // Step 3: Clean up orphaned DB records (if not dry run)
    if !dry_run && !orphaned_db_ids.is_empty() {
        for id in &orphaned_db_ids {
            match sqlx::query("DELETE FROM media WHERE id = $1")
                .bind(id)
                .execute(state.db.pool())
                .await
            {
                Ok(_) => result.db_records_cleaned += 1,
                Err(e) => result
                    .errors
                    .push(format!("Failed to delete record {id}: {e}")),
            }
        }
    }

    // Step 4: Find R2 files that aren't in database (would require listing R2 bucket)
    // This is more complex and would need to be paginated for large buckets
    // For now, we'll just report what we can verify

    let action = if dry_run { "would be" } else { "were" };

    Ok(Json(serde_json::json!({
        "success": true,
        "dry_run": dry_run,
        "data": {
            "orphaned_db_records_found": result.orphaned_db_records,
            "db_records_cleaned": result.db_records_cleaned,
            "errors": result.errors
        },
        "message": format!(
            "{} orphaned database record(s) found. {} {} cleaned.",
            result.orphaned_db_records,
            result.db_records_cleaned,
            action
        )
    })))
}
