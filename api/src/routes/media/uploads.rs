//! Media uploads — presigned R2 URLs, confirm upload, multipart direct upload.
//!
//! R27-B split: every endpoint that materially writes a NEW row into
//! `media` lives here. The CRUD module only mutates existing rows.

// The pre-split file had `#![allow(clippy::double_ended_iterator_last)]`
// at module scope; the `.split('/').last()` filename extractions below
// rely on it. Preserve verbatim to keep the split structural-only.
#![allow(clippy::double_ended_iterator_last)]

use axum::{
    extract::{Multipart, State},
    response::Json,
};
use uuid::Uuid;

use super::dto::{ConfirmUploadRequest, Media, PresignedUploadRequest};
use super::helpers::validate_file_signature;
use crate::{AppState, middleware::admin::AdminUser, utils::errors::ApiError};

/// POST /admin/media/presigned-upload - Get presigned URL for direct upload
/// Client uploads directly to R2, then confirms with /admin/media/confirm-upload
/// ICT 7 SECURITY: AdminUser authentication required
#[tracing::instrument(skip(state, _admin))]
pub(super) async fn presigned_upload(
    State(state): State<AppState>,
    _admin: AdminUser,
    Json(payload): Json<PresignedUploadRequest>,
) -> Result<Json<serde_json::Value>, ApiError> {
    if !super::helpers::allowed_upload_type(&payload.content_type) {
        return Err(ApiError::validation_error("Unsupported content type"));
    }
    if payload
        .size
        .is_some_and(|size| !(1..=50 * 1024 * 1024).contains(&size))
    {
        return Err(ApiError::validation_error(
            "Declared upload size must be 1–50 MiB",
        ));
    }
    if payload.filename.is_empty()
        || payload.filename.len() > 255
        || payload.filename.contains(['/', '\\', '\0'])
    {
        return Err(ApiError::validation_error("Invalid filename"));
    }

    // Generate file key
    let extension = match payload.content_type.as_str() {
        "image/jpeg" => "jpg",
        "image/png" => "png",
        "image/webp" => "webp",
        "image/gif" => "gif",
        "image/avif" => "avif",
        "video/mp4" => "mp4",
        "video/webm" => "webm",
        "video/quicktime" => "mov",
        "application/pdf" => "pdf",
        _ => "bin",
    };

    let folder = payload.collection.unwrap_or_else(|| "uploads".to_string());
    if folder.contains("..")
        || folder.contains('/')
        || folder.contains('\\')
        || folder.contains('\0')
    {
        return Err(ApiError::validation_error("Invalid collection name"));
    }
    let file_key = format!("{}/{}.{}", folder, Uuid::new_v4(), extension);

    // Get presigned URL from storage service
    let storage = &state.services.storage;

    let expires_in = 3600u64; // 1 hour
    let upload_url = storage
        .presigned_upload_url(&file_key, &payload.content_type, expires_in)
        .await
        .map_err(|e| ApiError::internal_error(&format!("Failed to generate upload URL: {e}")))?;

    // Get public URL
    let public_url = format!("{}/{}", state.config.r2_public_url, file_key);

    sqlx::query("INSERT INTO media_upload_tickets(file_key,user_id,content_type,expected_size,expires_at) VALUES($1,$2,$3,$4,NOW()+INTERVAL '1 hour')")
        .bind(&file_key).bind(_admin.0.id).bind(&payload.content_type).bind(payload.size).execute(state.db.pool()).await.map_err(|e|ApiError::database_error(&e.to_string()))?;

    Ok(Json(serde_json::json!({
        "success": true,
        "data": {
            "upload_url": upload_url,
            "file_key": file_key,
            "public_url": public_url,
            "expires_in": expires_in
        }
    })))
}

/// POST /admin/media/confirm-upload - Confirm upload and save to database
/// ICT 7 SECURITY: AdminUser authentication required, path traversal protection
#[tracing::instrument(skip(state, _admin))]
pub(super) async fn confirm_upload(
    State(state): State<AppState>,
    _admin: AdminUser,
    Json(payload): Json<ConfirmUploadRequest>,
) -> Result<Json<serde_json::Value>, ApiError> {
    let metadata = super::dto::UpdateMedia {
        title: payload.title.clone(),
        alt_text: payload.alt_text.clone(),
        caption: payload.caption.clone(),
        description: payload.description.clone(),
        collection: payload.collection.clone(),
        tags: None,
    };
    super::helpers::validate_metadata(&metadata)?;
    if payload.original_filename.is_empty()
        || payload.original_filename.len() > 255
        || payload.original_filename.contains(['/', '\\', '\0'])
    {
        return Err(ApiError::validation_error("Invalid original filename"));
    }
    let mut tx = state
        .db
        .pool()
        .begin()
        .await
        .map_err(|e| ApiError::database_error(&e.to_string()))?;
    let ticket: Option<(String,Option<i64>,Option<i64>)> = sqlx::query_as("SELECT content_type,expected_size,confirmed_media_id FROM media_upload_tickets WHERE file_key=$1 AND user_id=$2 AND expires_at>NOW() FOR UPDATE")
        .bind(&payload.file_key).bind(_admin.0.id).fetch_optional(&mut *tx).await.map_err(|e|ApiError::database_error(&e.to_string()))?;
    let (mime, expected_size, confirmed) = ticket.ok_or_else(|| {
        ApiError::validation_error("Upload ticket is missing, expired, or belongs to another user")
    })?;
    if let Some(id) = confirmed {
        tx.rollback()
            .await
            .map_err(|e| ApiError::database_error(&e.to_string()))?;
        let result =
            super::crud::show(State(state.clone()), _admin, axum::extract::Path(id)).await?;
        if result.0["data"]["scan_status"] == "infected" {
            state
                .services
                .storage
                .delete(&payload.file_key)
                .await
                .map_err(|_| {
                    ApiError::new(
                        axum::http::StatusCode::SERVICE_UNAVAILABLE,
                        "Threat detected; stored object removal failed",
                    )
                })?;
            return Err(ApiError::validation_error(
                "Malware detected; stored object removed",
            ));
        }
        return Ok(result);
    }
    if payload.content_type != mime {
        return Err(ApiError::validation_error(
            "Content type differs from the issued upload ticket",
        ));
    }
    let data = state
        .services
        .storage
        .download_for_scan(&payload.file_key)
        .await
        .map_err(|error| {
            tracing::warn!(%error,"Cannot confirm stored upload");
            ApiError::new(
                axum::http::StatusCode::SERVICE_UNAVAILABLE,
                "Cannot retrieve uploaded object; no media record created",
            )
        })?;
    let data = axum::body::Bytes::from(data);
    if data.is_empty()
        || data.len() > 50 * 1024 * 1024
        || payload.size != data.len() as i64
        || expected_size.is_some_and(|size| size != data.len() as i64)
    {
        return Err(ApiError::validation_error(
            "Stored object size differs from the declared upload",
        ));
    }
    if !validate_file_signature(&data, &mime) {
        return Err(ApiError::validation_error(
            "Stored bytes do not match the declared content type",
        ));
    }
    let scan = super::scanning::scan_for_malware(&data).await;
    let info = if mime.starts_with("image/") {
        if let Some(processor) = &state.services.media_processor {
            match processor.inspect(data.clone()).await {
                Ok(info) => Some(info),
                Err(error) => {
                    if error
                        .downcast_ref::<crate::services::media_processor::ProcessorResponseError>()
                        .is_some_and(|error| (400..500).contains(&error.status))
                    {
                        return Err(ApiError::validation_error(&format!(
                            "Uploaded image cannot be decoded: {error}"
                        )));
                    }
                    tracing::warn!(%error,"Confirmed image dimensions unavailable");
                    None
                }
            }
        } else {
            None
        }
    } else {
        None
    };
    // A still-valid presigned PUT must never overwrite a scanned published
    // original. Publish the bytes under a new server-only key.
    let final_key = if scan.status == "infected" {
        payload.file_key.clone()
    } else {
        format!(
            "uploads/{}.{}",
            Uuid::new_v4(),
            payload.file_key.rsplit('.').next().unwrap_or("bin")
        )
    };
    let public_url = if scan.status == "infected" {
        None
    } else {
        Some(
            state
                .services
                .storage
                .upload_at_key(&final_key, data.to_vec(), &mime)
                .await
                .map_err(|error| {
                    tracing::warn!(%error,"Cannot publish confirmed upload");
                    ApiError::new(
                        axum::http::StatusCode::SERVICE_UNAVAILABLE,
                        "Cannot publish uploaded object; confirmation can be retried",
                    )
                })?,
        )
    };
    let saved = async {
    let filename = final_key
        .rsplit('/')
        .next()
        .ok_or_else(|| ApiError::validation_error("Invalid file key"))?;
    let id: i64 = sqlx::query_scalar("INSERT INTO media(filename,original_filename,mime_type,size,path,url,title,alt_text,caption,description,collection,width,height,created_by,scan_status,scan_result,is_optimized,created_at,updated_at) VALUES($1,$2,$3,$4,$5,$6,$7,$8,$9,$10,$11,$12,$13,$14,$15,$16,FALSE,NOW(),NOW()) RETURNING id")
        .bind(filename).bind(payload.original_filename).bind(mime).bind(payload.size).bind(&final_key).bind(public_url).bind(metadata.title).bind(metadata.alt_text).bind(metadata.caption).bind(metadata.description).bind(metadata.collection)
        .bind(info.as_ref().map(|i|i.width as i32)).bind(info.as_ref().map(|i|i.height as i32)).bind(_admin.0.id).bind(&scan.status).bind(serde_json::to_value(&scan).map_err(|_|ApiError::internal_error("Cannot serialize scan"))?)
        .fetch_one(&mut *tx).await.map_err(|e|ApiError::database_error(&e.to_string()))?;
    super::versions::record_current(&mut tx,id,Some(_admin.0.id),"Uploaded").await?;
    sqlx::query("UPDATE media_upload_tickets SET confirmed_media_id=$2,cleanup_pending=TRUE WHERE file_key=$1")
        .bind(&payload.file_key)
        .bind(id)
        .execute(&mut *tx)
        .await
        .map_err(|e| ApiError::database_error(&e.to_string()))?;
    tx.commit()
        .await
        .map_err(|e| ApiError::database_error(&e.to_string()))?;
    Ok::<i64,ApiError>(id)
    }.await;
    let id = match saved {
        Ok(id) => id,
        Err(error) => {
            if final_key != payload.file_key
                && let Err(cleanup) = state.services.storage.delete(&final_key).await
            {
                tracing::error!(%cleanup,%final_key,"Failed to clean an unpublished confirmed upload");
            }
            return Err(error);
        }
    };
    // Durable ticket cleanup remains pending until deletion is acknowledged.
    if let Err(error) = state.services.storage.delete(&payload.file_key).await {
        tracing::warn!(%error,"Pending upload key retained for worker cleanup");
    } else {
        sqlx::query("UPDATE media_upload_tickets SET cleanup_pending=FALSE,cleanup_error=NULL WHERE file_key=$1").bind(&payload.file_key).execute(state.db.pool()).await.map_err(|e|ApiError::database_error(&e.to_string()))?;
    }
    if scan.status == "infected" {
        state
            .services
            .storage
            .delete(&payload.file_key)
            .await
            .map_err(|error| {
                tracing::error!(%error,"Infected confirmed upload removal failed");
                ApiError::new(
                    axum::http::StatusCode::SERVICE_UNAVAILABLE,
                    "Threat detected; stored object removal failed",
                )
            })?;
        return Err(ApiError::validation_error(
            "Malware detected; stored object removed",
        ));
    }
    super::crud::show(State(state), _admin, axum::extract::Path(id)).await
}

/// POST /admin/media/upload - Direct multipart file upload
/// For smaller files or when presigned URLs aren't suitable
/// ICT 7 SECURITY: AdminUser authentication, file type whitelist, size limits
#[tracing::instrument(skip(state, _admin, multipart))]
pub(super) async fn direct_upload(
    State(state): State<AppState>,
    _admin: AdminUser,
    mut multipart: Multipart,
) -> Result<Json<serde_json::Value>, ApiError> {
    let mut fields = serde_json::Map::new();
    let mut files = Vec::new();
    let mut optimize = false;
    let mut total_bytes = 0;
    while let Some(field) = multipart
        .next_field()
        .await
        .map_err(|e| ApiError::validation_error(&format!("Invalid multipart upload: {e}")))?
    {
        let name = field.name().unwrap_or("file").to_owned();
        if field.file_name().is_none() {
            let text = field
                .text()
                .await
                .map_err(|_| ApiError::validation_error("Invalid metadata field"))?;
            if matches!(name.as_str(), "optimize" | "generate_webp") {
                match text.as_str() {
                    "true" => optimize = true,
                    "false" => {}
                    _ => {
                        return Err(ApiError::validation_error(
                            "Upload processing flags must be true or false",
                        ));
                    }
                }
            } else if name == "tags" {
                let tags: Vec<String> = serde_json::from_str(&text)
                    .map_err(|_| ApiError::validation_error("Tags must be a JSON string array"))?;
                fields.insert(name, serde_json::json!(tags));
            } else {
                fields.insert(name, serde_json::Value::String(text));
            }
            continue;
        }
        let filename = field.file_name().unwrap_or_default().to_owned();
        let mime = field
            .content_type()
            .unwrap_or("application/octet-stream")
            .to_owned();
        if filename.is_empty()
            || filename.len() > 255
            || filename.contains("..")
            || filename.contains(['/', '\\', '\0'])
        {
            return Err(ApiError::validation_error("Invalid filename"));
        }
        let data = field
            .bytes()
            .await
            .map_err(|_| ApiError::validation_error("Cannot read upload body"))?;
        total_bytes += data.len();
        if files.len() >= 5 || data.is_empty() || total_bytes > 50 * 1024 * 1024 {
            return Err(ApiError::validation_error(
                "Upload 1–5 files with at most 50 MiB in total",
            ));
        }
        if !super::helpers::allowed_upload_type(&mime) || !validate_file_signature(&data, &mime) {
            return Err(ApiError::validation_error(
                "File content does not match a supported declared type",
            ));
        }
        files.push((filename, mime, data));
    }
    if files.is_empty() {
        return Err(ApiError::validation_error("No files uploaded"));
    }
    let metadata: super::dto::UpdateMedia =
        serde_json::from_value(serde_json::Value::Object(fields))
            .map_err(|_| ApiError::validation_error("Unknown or invalid media metadata field"))?;
    super::helpers::validate_metadata(&metadata)?;
    let preset = if optimize {
        if state.services.media_processor.is_none() {
            return Err(ApiError::new(
                axum::http::StatusCode::SERVICE_UNAVAILABLE,
                "Image processing is not configured",
            ));
        }
        Some(super::processing::preset(&state, None).await?)
    } else {
        None
    };
    let mut scanned = Vec::new();
    for (filename, mime, data) in files {
        let scan = super::scanning::scan_for_malware(&data).await;
        if scan.status == "infected" {
            return Err(ApiError::validation_error(
                "Malware detected; file was not uploaded",
            ));
        }
        if optimize
            && !matches!(
                mime.as_str(),
                "image/jpeg" | "image/png" | "image/webp" | "image/gif" | "image/avif"
            )
        {
            return Err(ApiError::validation_error(
                "Processing requires a supported raster image",
            ));
        }
        scanned.push((filename, mime, data, scan));
    }
    let mut keys = Vec::new();
    let outcome = async {
        let mut tx = state.db.pool().begin().await.map_err(|e|ApiError::database_error(&e.to_string()))?;
        let mut uploaded = Vec::new();
        for (filename,mime,data,scan) in scanned {
            let info = if matches!(mime.as_str(),"image/jpeg"|"image/png"|"image/webp"|"image/gif"|"image/avif") {
                if let Some(processor) = &state.services.media_processor {
                    // Inspection is optional for uploads, strict inside processing jobs.
                    match processor.inspect(data.clone()).await { Ok(info)=>Some(info),Err(error)=>{if error.downcast_ref::<crate::services::media_processor::ProcessorResponseError>().is_some_and(|error|(400..500).contains(&error.status)){return Err(ApiError::validation_error(&format!("Uploaded image cannot be decoded: {error}")));}tracing::warn!(%error,"Uploaded image dimensions unavailable");None} }
                } else { None }
            } else { None };
            let url = state.services.storage.upload(data.to_vec(),&mime,"uploads").await.map_err(|e|ApiError::new(axum::http::StatusCode::SERVICE_UNAVAILABLE,format!("Upload failed: {e}")))?;
            let key = url.strip_prefix(&format!("{}/",state.config.r2_public_url.trim_end_matches('/'))).ok_or_else(||ApiError::internal_error("Invalid storage URL"))?.to_owned();
            keys.push(key.clone());
            let stored_filename = key.rsplit('/').next().unwrap_or(&key);
            let media: Media = sqlx::query_as("INSERT INTO media(filename,original_filename,mime_type,size,path,url,title,alt_text,caption,description,collection,tags,created_by,width,height,scan_status,scan_result,is_optimized,created_at,updated_at) VALUES($1,$2,$3,$4,$5,$6,$7,$8,$9,$10,$11,$12,$13,$14,$15,$16,$17,FALSE,NOW(),NOW()) RETURNING id,filename,original_filename,mime_type,size,path,url,title,alt_text,caption,description,collection,is_optimized,width,height,scan_status,scan_result,created_at,updated_at")
                .bind(stored_filename).bind(filename).bind(mime).bind(data.len() as i64).bind(&key).bind(url)
                .bind(&metadata.title).bind(&metadata.alt_text).bind(&metadata.caption).bind(&metadata.description).bind(&metadata.collection).bind(serde_json::json!(metadata.tags.as_deref().unwrap_or_default())).bind(_admin.0.id)
                .bind(info.as_ref().map(|i|i.width as i32)).bind(info.as_ref().map(|i|i.height as i32)).bind(&scan.status).bind(serde_json::to_value(&scan).map_err(|_|ApiError::internal_error("Cannot serialize scan"))?)
                .fetch_one(&mut *tx).await.map_err(|e|ApiError::database_error(&e.to_string()))?;
            super::versions::record_current(&mut tx,media.id,Some(_admin.0.id),"Uploaded").await?;
            if let Some(preset) = &preset {
                sqlx::query("INSERT INTO media_optimization_jobs(media_id,preset) VALUES($1,$2)").bind(media.id).bind(serde_json::json!(preset)).execute(&mut *tx).await.map_err(|e|ApiError::database_error(&e.to_string()))?;
            }
            uploaded.push(media);
        }
        tx.commit().await.map_err(|e|ApiError::database_error(&e.to_string()))?;
        Ok::<_,ApiError>(uploaded)
    }.await;
    let uploaded = match outcome {
        Ok(uploaded) => uploaded,
        Err(error) => {
            for key in keys {
                if let Err(cleanup) = state.services.storage.delete(&key).await {
                    tracing::error!(%key,%cleanup,"Failed to clean an unrecorded upload");
                }
            }
            return Err(error);
        }
    };
    Ok(Json(
        serde_json::json!({"success":true,"data":super::helpers::present(&state,uploaded).await?,"message":"Files uploaded"}),
    ))
}
