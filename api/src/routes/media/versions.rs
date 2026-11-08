//! Immutable content revisions. Replacements preserve old objects; restoring
//! creates another revision, retaining original links and version history.
use crate::{AppState, middleware::admin::AdminUser, utils::errors::ApiError};
use axum::{
    body::Bytes,
    extract::{Multipart, Path, Query, State},
    http::StatusCode,
    response::Json,
};
use serde::Deserialize;
use serde_json::{Value, json};
use sqlx::{Postgres, Transaction};
fn db(error: sqlx::Error) -> ApiError {
    ApiError::database_error(&error.to_string())
}

pub(super) async fn record_current(
    tx: &mut Transaction<'_, Postgres>,
    media_id: i64,
    actor: Option<i64>,
    changes: &str,
) -> Result<i64, ApiError> {
    sqlx::query("INSERT INTO media_versions(media_id,version_number,snapshot,changes,created_by) SELECT id,content_version,to_jsonb(media),$2,$3 FROM media WHERE id=$1 ON CONFLICT(media_id,version_number) DO NOTHING")
        .bind(media_id).bind(changes).bind(actor).execute(&mut **tx).await.map_err(db)?;
    sqlx::query_scalar("SELECT v.id FROM media_versions v JOIN media m ON m.id=v.media_id AND m.content_version=v.version_number WHERE m.id=$1").bind(media_id).fetch_one(&mut **tx).await.map_err(db)
}
async fn version(state: &AppState, id: i64) -> Result<Value, ApiError> {
    sqlx::query_scalar("SELECT jsonb_build_object('id',id::text,'version_number',version_number,'file_path',snapshot->'path','url',snapshot->'url','file_size',snapshot->'size','mime_type',snapshot->'mime_type','width',snapshot->'width','height',snapshot->'height','scan_status',snapshot->'scan_status','changes',changes,'created_by',created_by,'created_at',created_at) FROM media_versions WHERE id=$1")
        .bind(id).fetch_optional(state.db.pool()).await.map_err(db)?.ok_or_else(||ApiError::not_found("Media version not found"))
}
#[derive(Deserialize)]
pub(super) struct VersionsQuery {
    page: Option<i64>,
    per_page: Option<i64>,
}
pub(super) async fn index(
    State(state): State<AppState>,
    _admin: AdminUser,
    Path(id): Path<i64>,
    Query(query): Query<VersionsQuery>,
) -> Result<Json<Value>, ApiError> {
    let exists: bool = sqlx::query_scalar("SELECT EXISTS(SELECT 1 FROM media WHERE id=$1)")
        .bind(id)
        .fetch_one(state.db.pool())
        .await
        .map_err(db)?;
    if !exists {
        return Err(ApiError::not_found("Media not found"));
    }
    let page = query.page.unwrap_or(1);
    let per_page = query.per_page.unwrap_or(20);
    if !(1..=100_000).contains(&page) || !(1..=100).contains(&per_page) {
        return Err(ApiError::validation_error("Invalid version pagination"));
    }
    let total: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM media_versions WHERE media_id=$1")
        .bind(id)
        .fetch_one(state.db.pool())
        .await
        .map_err(db)?;
    let rows:Vec<Value>=sqlx::query_scalar("SELECT to_jsonb(v)-'snapshot' || jsonb_build_object('id',id::text,'file_path',snapshot->'path','url',snapshot->'url','file_size',snapshot->'size','mime_type',snapshot->'mime_type','width',snapshot->'width','height',snapshot->'height','scan_status',snapshot->'scan_status') FROM media_versions v WHERE media_id=$1 ORDER BY version_number DESC LIMIT $2 OFFSET $3")
        .bind(id).bind(per_page).bind((page-1)*per_page).fetch_all(state.db.pool()).await.map_err(db)?;
    Ok(Json(
        json!({"versions":rows,"meta":{"current_page":page,"last_page":((total+per_page-1)/per_page).max(1),"total":total}}),
    ))
}
struct File {
    name: String,
    mime: String,
    bytes: Bytes,
}
async fn replace_file(
    state: &AppState,
    media_id: i64,
    actor: i64,
    file: File,
    changes: &str,
    existing_key: Option<String>,
) -> Result<i64, ApiError> {
    if changes.chars().count() > 2000 || changes.contains('\0') {
        return Err(ApiError::validation_error(
            "Changes must contain at most 2000 characters",
        ));
    }
    if file.name.is_empty() || file.name.len() > 255 || file.name.contains(['/', '\\', '\0']) {
        return Err(ApiError::validation_error("Invalid filename"));
    }
    if file.bytes.is_empty()
        || file.bytes.len() > 50 * 1024 * 1024
        || !super::helpers::allowed_upload_type(&file.mime)
        || !super::helpers::validate_file_signature(&file.bytes, &file.mime)
    {
        return Err(ApiError::validation_error(
            "Invalid replacement file type, bytes or size",
        ));
    }
    let scan = super::scanning::scan_for_malware(&file.bytes).await;
    if scan.status == "infected" {
        return Err(ApiError::validation_error(
            "Malware detected; current media and versions retained",
        ));
    }
    let info = if file.mime.starts_with("image/") {
        if let Some(processor) = &state.services.media_processor {
            Some(
                processor
                    .inspect(file.bytes.clone())
                    .await
                    .map_err(|error| {
                        ApiError::new(
                            if error.downcast_ref::<crate::services::media_processor::ProcessorResponseError>().is_some_and(|error|(400..500).contains(&error.status)) {StatusCode::BAD_REQUEST}else{StatusCode::SERVICE_UNAVAILABLE},
                            format!("Replacement image could not be inspected: {error}"),
                        )
                    })?,
            )
        } else {
            None
        }
    } else {
        None
    };
    let mut uploaded = None;
    let outcome=async {
        let mut tx=state.db.pool().begin().await.map_err(db)?;
        let current:Option<i64>=sqlx::query_scalar("SELECT content_version FROM media WHERE id=$1 FOR UPDATE").bind(media_id).fetch_optional(&mut *tx).await.map_err(db)?;
        let current=current.ok_or_else(||ApiError::not_found("Media not found"))?;
        let next=current.checked_add(1).ok_or_else(||ApiError::internal_error("Media version limit reached"))?;
        record_current(&mut tx,media_id,None,"Original retained before replacement").await?;
        let key=if let Some(key)=existing_key {key}else{
            let extension=match file.mime.as_str(){"image/jpeg"=>"jpg","image/png"=>"png","image/webp"=>"webp","image/gif"=>"gif","image/avif"=>"avif","application/pdf"=>"pdf","text/plain"=>"txt",_=>"bin"};
            let key=format!("uploads/{}.{}",uuid::Uuid::new_v4(),extension);
            uploaded=Some(key.clone());
            state.services.storage.upload_at_key(&key,file.bytes.to_vec(),&file.mime).await.map_err(|error|{tracing::warn!(%error,"Replacement storage failed");ApiError::new(StatusCode::SERVICE_UNAVAILABLE,"Replacement storage failed; current file retained")})?;
            key
        };
        // Lock order matches the worker, including cancellation of older input.
        sqlx::query("UPDATE media_optimization_jobs SET status='cancelled',error='Original replaced',lease_until=NULL,completed_at=NOW() WHERE media_id=$1 AND status IN ('pending','processing')").bind(media_id).execute(&mut *tx).await.map_err(db)?;
        let url=format!("{}/{}",state.config.r2_public_url.trim_end_matches('/'),key);
        let filename=key.rsplit('/').next().unwrap_or(&key);
        sqlx::query("UPDATE media SET filename=$2,original_filename=$3,mime_type=$4,size=$5,path=$6,url=$7,width=$8,height=$9,is_optimized=FALSE,scan_status=$10,scan_result=$11,content_version=$12,updated_at=NOW() WHERE id=$1")
            .bind(media_id).bind(filename).bind(file.name).bind(&file.mime).bind(file.bytes.len() as i64).bind(&key).bind(url).bind(info.as_ref().map(|i|i.width as i32)).bind(info.as_ref().map(|i|i.height as i32)).bind(&scan.status).bind(json!(scan)).bind(next).execute(&mut *tx).await.map_err(db)?;
        let version_id=record_current(&mut tx,media_id,Some(actor),changes).await?;
        crate::services::media_jobs::enqueue_default_if_image(
            state, &mut tx, media_id, &file.mime, &scan.status, next,
        ).await.map_err(db)?;
        tx.commit().await.map_err(db)?;
        Ok::<i64,ApiError>(version_id)
    }.await;
    if outcome.is_err()
        && let Some(key) = uploaded
        && let Err(error) = state.services.storage.delete(&key).await
    {
        tracing::error!(%key,%error,"Cannot clean unpublished replacement");
    }
    outcome
}
pub(super) async fn replace(
    State(state): State<AppState>,
    admin: AdminUser,
    Path(id): Path<i64>,
    mut multipart: Multipart,
) -> Result<Json<Value>, ApiError> {
    let mut file = None;
    let mut changes = "File replaced".to_owned();
    while let Some(field) = multipart
        .next_field()
        .await
        .map_err(|_| ApiError::validation_error("Invalid multipart body"))?
    {
        if field.name() == Some("changes") && field.file_name().is_none() {
            changes = field
                .text()
                .await
                .map_err(|_| ApiError::validation_error("Invalid changes field"))?;
            continue;
        }
        if field.name() != Some("file") || file.is_some() {
            return Err(ApiError::validation_error(
                "Supply exactly one replacement file and optional changes",
            ));
        }
        let name = field.file_name().unwrap_or_default().to_owned();
        let mime = field.content_type().unwrap_or_default().to_owned();
        let bytes = field
            .bytes()
            .await
            .map_err(|_| ApiError::validation_error("Cannot read replacement file"))?;
        file = Some(File { name, mime, bytes });
    }
    let file = file.ok_or_else(|| ApiError::validation_error("A replacement file is required"))?;
    let version_id = replace_file(&state, id, admin.0.id, file, &changes, None).await?;
    response(state, admin, id, version_id).await
}
#[derive(Deserialize)]
struct Snapshot {
    original_filename: Option<String>,
    filename: String,
    mime_type: Option<String>,
    path: Option<String>,
}
pub(super) async fn restore(
    State(state): State<AppState>,
    admin: AdminUser,
    Path((id, version_id)): Path<(i64, i64)>,
) -> Result<Json<Value>, ApiError> {
    let snapshot: Value =
        sqlx::query_scalar("SELECT snapshot FROM media_versions WHERE id=$1 AND media_id=$2")
            .bind(version_id)
            .bind(id)
            .fetch_optional(state.db.pool())
            .await
            .map_err(db)?
            .ok_or_else(|| ApiError::not_found("Media version not found"))?;
    let snapshot: Snapshot = serde_json::from_value(snapshot)
        .map_err(|_| ApiError::internal_error("Invalid stored version snapshot"))?;
    let key = snapshot
        .path
        .ok_or_else(|| ApiError::validation_error("Version has no stored object"))?;
    let bytes = state
        .services
        .storage
        .download_for_scan(&key)
        .await
        .map_err(|error| {
            tracing::warn!(%error,"Version object unavailable");
            ApiError::new(
                StatusCode::SERVICE_UNAVAILABLE,
                "Version object is unavailable; current media retained",
            )
        })?;
    let file = File {
        name: snapshot.original_filename.unwrap_or(snapshot.filename),
        mime: snapshot
            .mime_type
            .ok_or_else(|| ApiError::validation_error("Version has no known content type"))?,
        bytes: bytes.into(),
    };
    let restored = replace_file(
        &state,
        id,
        admin.0.id,
        file,
        &format!("Restored version record {version_id}"),
        Some(key),
    )
    .await?;
    response(state, admin, id, restored).await
}
async fn response(
    state: AppState,
    admin: AdminUser,
    id: i64,
    version_id: i64,
) -> Result<Json<Value>, ApiError> {
    let version = version(&state, version_id).await?;
    let mut response = super::crud::show(State(state), admin, Path(id)).await?;
    response.0["version"] = version;
    Ok(response)
}
