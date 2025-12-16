//! Admin media routes

use worker::{Request, Response, RouteContext};
use crate::AppState;
use crate::error::ApiError;
use crate::models::post::{Media, PaginatedResponse, PaginationMeta};
use crate::middleware::auth::require_admin;
use crate::services::r2::{R2Service, get_content_type};

/// GET /api/admin/media - List all media files
pub async fn list(req: Request, ctx: RouteContext<AppState>) -> worker::Result<Response> {
    require_admin(&req, &ctx).await?;

    let url = req.url()?;
    let query: MediaListQuery = serde_urlencoded::from_str(url.query().unwrap_or(""))
        .unwrap_or_default();

    let page = query.page.unwrap_or(1).max(1);
    let per_page = query.per_page.unwrap_or(24).clamp(1, 100);
    let offset = (page - 1) * per_page;

    let mut sql = String::from("SELECT * FROM media WHERE 1=1");
    let mut params: Vec<serde_json::Value> = vec![];
    let mut param_idx = 1;

    if let Some(mime_type) = &query.mime_type {
        sql.push_str(&format!(" AND mime_type LIKE ${}", param_idx));
        params.push(serde_json::json!(format!("{}%", mime_type)));
        param_idx += 1;
    }

    if let Some(folder) = &query.folder {
        sql.push_str(&format!(" AND folder = ${}", param_idx));
        params.push(serde_json::json!(folder));
        param_idx += 1;
    }

    if let Some(search) = &query.search {
        sql.push_str(&format!(" AND (filename ILIKE ${} OR original_filename ILIKE ${})", param_idx, param_idx));
        params.push(serde_json::json!(format!("%{}%", search)));
        param_idx += 1;
    }

    // Count total
    let count_sql = sql.replace("SELECT *", "SELECT COUNT(*) as count");
    let total: i64 = ctx.data.db.query_one::<CountResult>(&count_sql, params.clone())
        .await?
        .map(|r| r.count)
        .unwrap_or(0);

    sql.push_str(" ORDER BY created_at DESC");
    sql.push_str(&format!(" LIMIT {} OFFSET {}", per_page, offset));

    let media: Vec<Media> = ctx.data.db.query(&sql, params).await?;

    let response = PaginatedResponse {
        data: media,
        meta: PaginationMeta::new(page, per_page, total),
    };

    Response::from_json(&response)
}

/// POST /api/admin/media/upload - Upload a media file
pub async fn upload(mut req: Request, ctx: RouteContext<AppState>) -> worker::Result<Response> {
    let user = require_admin(&req, &ctx).await?;

    // Parse multipart form data
    let form_data = req.form_data().await
        .map_err(|e| ApiError::BadRequest(format!("Invalid form data: {}", e)))?;

    let file = form_data.get("file")
        .ok_or_else(|| ApiError::BadRequest("No file provided".to_string()))?;

    let (filename, data) = match file {
        worker::FormEntry::File(f) => {
            let name = f.name();
            let bytes = f.bytes().await
                .map_err(|e| ApiError::BadRequest(format!("Failed to read file: {}", e)))?;
            (name, bytes)
        }
        _ => return Err(ApiError::BadRequest("Expected file upload".to_string()).into()),
    };

    let folder = form_data.get("folder")
        .and_then(|f| match f {
            worker::FormEntry::Field(s) => Some(s),
            _ => None,
        })
        .unwrap_or_else(|| "uploads".to_string());

    let content_type = get_content_type(&filename);
    let key = R2Service::generate_key(&folder, &filename);

    // Upload to R2
    let r2 = &ctx.data.services.r2;
    let upload_result = r2.upload(&key, data.clone(), content_type).await?;

    // Get image dimensions if applicable
    let (width, height): (Option<i32>, Option<i32>) = if content_type.starts_with("image/") {
        // In production, use an image processing library
        (None, None)
    } else {
        (None, None)
    };

    // Save to database
    let media_id = uuid::Uuid::new_v4();
    let now = crate::utils::now();

    ctx.data.db.execute(
        r#"
        INSERT INTO media (
            id, filename, original_filename, mime_type, size, width, height,
            url, folder, uploaded_by, created_at, updated_at
        ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $11)
        "#,
        vec![
            serde_json::json!(media_id.to_string()),
            serde_json::json!(key),
            serde_json::json!(filename),
            serde_json::json!(content_type),
            serde_json::json!(data.len() as i64),
            serde_json::json!(width),
            serde_json::json!(height),
            serde_json::json!(upload_result.url),
            serde_json::json!(folder),
            serde_json::json!(user.id.to_string()),
            serde_json::json!(now.to_rfc3339()),
        ]
    ).await?;

    let media: Media = ctx.data.db.query_one(
        "SELECT * FROM media WHERE id = $1",
        vec![serde_json::json!(media_id.to_string())]
    ).await?
    .ok_or_else(|| ApiError::Internal("Failed to retrieve uploaded media".to_string()))?;

    Response::from_json(&media).map(|r| r.with_status(201))
}

/// DELETE /api/admin/media/:id - Delete a media file
pub async fn delete(req: Request, ctx: RouteContext<AppState>) -> worker::Result<Response> {
    require_admin(&req, &ctx).await?;

    let id = ctx.param("id")
        .ok_or_else(|| ApiError::BadRequest("Missing media id".to_string()))?;

    // Get media record
    let media: Option<Media> = ctx.data.db.query_one(
        "SELECT * FROM media WHERE id = $1",
        vec![serde_json::json!(id)]
    ).await?;

    let media = media.ok_or_else(|| ApiError::NotFound("Media not found".to_string()))?;

    // Delete from R2
    let r2 = &ctx.data.services.r2;
    r2.delete(&media.filename).await?;

    // Delete from database
    ctx.data.db.execute(
        "DELETE FROM media WHERE id = $1",
        vec![serde_json::json!(id)]
    ).await?;

    Response::from_json(&serde_json::json!({
        "message": "Media deleted successfully"
    }))
}

#[derive(serde::Deserialize, Default)]
struct MediaListQuery {
    page: Option<i64>,
    per_page: Option<i64>,
    mime_type: Option<String>,
    folder: Option<String>,
    search: Option<String>,
}

#[derive(serde::Deserialize)]
struct CountResult {
    count: i64,
}
