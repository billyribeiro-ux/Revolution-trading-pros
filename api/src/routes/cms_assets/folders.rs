//! CMS Assets — folder hierarchy CRUD
//!
//! Endpoints (mounted under `/cms/assets` in `mod.rs::router()`):
//!   GET    /folders        — list folders with asset counts
//!   POST   /folders        — create folder
//!   PUT    /folders/:id    — update folder
//!   DELETE /folders/:id    — delete folder (move children to parent or :move_to)

use axum::{
    extract::{Path, Query, State},
    Json,
};
use serde_json::{json, Value as JsonValue};
use uuid::Uuid;

use crate::{middleware::admin::AdminUser, utils::errors::ApiError, AppState};

use super::dto::{AssetFolder, CreateFolderRequest, UpdateFolderRequest};
use super::helpers::{get_cms_user_id, slugify};

/// GET /cms/assets/folders - List all folders with hierarchy
#[tracing::instrument(skip(state, _admin))]
pub(super) async fn list_folders(
    State(state): State<AppState>,
    _admin: AdminUser,
) -> Result<Json<Vec<AssetFolder>>, ApiError> {
    let folders: Vec<AssetFolder> = sqlx::query_as(
        r"
        WITH folder_counts AS (
            SELECT folder_id, COUNT(*) as cnt
            FROM cms_assets
            WHERE deleted_at IS NULL
            GROUP BY folder_id
        )
        SELECT
            f.id, f.name, f.slug, f.parent_id, f.path, f.depth,
            f.color, f.icon,
            COALESCE(fc.cnt, 0)::int as asset_count,
            f.created_at, f.updated_at, f.created_by
        FROM cms_asset_folders f
        LEFT JOIN folder_counts fc ON fc.folder_id = f.id
        ORDER BY f.path, f.name
        ",
    )
    .fetch_all(state.db.pool())
    .await
    .map_err(|e| ApiError::database_error(&e.to_string()))?;

    Ok(Json(folders))
}

/// POST /cms/assets/folders - Create a new folder
#[tracing::instrument(skip(state, admin))]
pub(super) async fn create_folder(
    State(state): State<AppState>,
    admin: AdminUser,
    Json(payload): Json<CreateFolderRequest>,
) -> Result<Json<AssetFolder>, ApiError> {
    // Generate slug from name
    let slug = slugify(&payload.name);

    // Calculate path and depth
    let (path, depth) = if let Some(parent_id) = payload.parent_id {
        let parent: Option<(String, i32)> =
            sqlx::query_as("SELECT path, depth FROM cms_asset_folders WHERE id = $1")
                .bind(parent_id)
                .fetch_optional(state.db.pool())
                .await
                .map_err(|e| ApiError::database_error(&e.to_string()))?;

        match parent {
            Some((parent_path, parent_depth)) => {
                (format!("{parent_path}/{slug}"), parent_depth + 1)
            }
            None => return Err(ApiError::not_found("Parent folder not found")),
        }
    } else {
        (format!("/{slug}"), 0)
    };

    let cms_user_id = get_cms_user_id(state.db.pool(), &admin).await;

    let folder: AssetFolder = sqlx::query_as(
        r"
        INSERT INTO cms_asset_folders (
            id, name, slug, parent_id, path, depth, color, icon,
            created_at, updated_at, created_by
        ) VALUES (
            $1, $2, $3, $4, $5, $6, $7, $8,
            NOW(), NOW(), $9
        )
        RETURNING id, name, slug, parent_id, path, depth, color, icon,
                  0 as asset_count, created_at, updated_at, created_by
        ",
    )
    .bind(Uuid::new_v4())
    .bind(&payload.name)
    .bind(&slug)
    .bind(payload.parent_id)
    .bind(&path)
    .bind(depth)
    .bind(&payload.color)
    .bind(&payload.icon)
    .bind(cms_user_id)
    .fetch_one(state.db.pool())
    .await
    .map_err(|e| ApiError::database_error(&e.to_string()))?;

    Ok(Json(folder))
}

/// PUT /cms/assets/folders/:id - Update folder
#[tracing::instrument(skip(state, admin))]
pub(super) async fn update_folder(
    State(state): State<AppState>,
    admin: AdminUser,
    Path(id): Path<Uuid>,
    Json(payload): Json<UpdateFolderRequest>,
) -> Result<Json<AssetFolder>, ApiError> {
    // Build dynamic update query
    let mut updates = Vec::new();
    let mut param_idx = 1;

    if payload.name.is_some() {
        updates.push(format!("name = ${param_idx}"));
        param_idx += 1;
        updates.push(format!("slug = ${param_idx}"));
        param_idx += 1;
    }
    if payload.color.is_some() {
        updates.push(format!("color = ${param_idx}"));
        param_idx += 1;
    }
    if payload.icon.is_some() {
        updates.push(format!("icon = ${param_idx}"));
        param_idx += 1;
    }
    updates.push("updated_at = NOW()".to_string());
    updates.push(format!("updated_by = ${param_idx}"));
    param_idx += 1;

    if updates.len() <= 2 {
        return Err(ApiError::validation_error("No fields to update"));
    }

    let query = format!(
        r"
        UPDATE cms_asset_folders
        SET {}
        WHERE id = ${}
        RETURNING id, name, slug, parent_id, path, depth, color, icon,
                  0 as asset_count, created_at, updated_at, created_by
        ",
        updates.join(", "),
        param_idx
    );

    let cms_user_id = get_cms_user_id(state.db.pool(), &admin).await;
    let mut q = sqlx::query_as::<_, AssetFolder>(sqlx::AssertSqlSafe(query.as_str()));

    if let Some(ref name) = payload.name {
        q = q.bind(name);
        q = q.bind(slugify(name));
    }
    if let Some(ref color) = payload.color {
        q = q.bind(color);
    }
    if let Some(ref icon) = payload.icon {
        q = q.bind(icon);
    }
    q = q.bind(cms_user_id);
    q = q.bind(id);

    let folder = q
        .fetch_one(state.db.pool())
        .await
        .map_err(|e| ApiError::database_error(&e.to_string()))?;

    Ok(Json(folder))
}

/// DELETE /cms/assets/folders/:id - Delete folder (moves assets to parent)
#[tracing::instrument(skip(state, _admin))]
pub(super) async fn delete_folder(
    State(state): State<AppState>,
    _admin: AdminUser,
    Path(id): Path<Uuid>,
    Query(params): Query<std::collections::HashMap<String, String>>,
) -> Result<Json<JsonValue>, ApiError> {
    let move_to: Option<Uuid> = params.get("move_to").and_then(|s| Uuid::parse_str(s).ok());

    // Move assets to target folder or root
    sqlx::query("UPDATE cms_assets SET folder_id = $1 WHERE folder_id = $2")
        .bind(move_to)
        .bind(id)
        .execute(state.db.pool())
        .await
        .map_err(|e| ApiError::database_error(&e.to_string()))?;

    // Move child folders to target or root
    sqlx::query("UPDATE cms_asset_folders SET parent_id = $1 WHERE parent_id = $2")
        .bind(move_to)
        .bind(id)
        .execute(state.db.pool())
        .await
        .map_err(|e| ApiError::database_error(&e.to_string()))?;

    // Delete the folder
    sqlx::query("DELETE FROM cms_asset_folders WHERE id = $1")
        .bind(id)
        .execute(state.db.pool())
        .await
        .map_err(|e| ApiError::database_error(&e.to_string()))?;

    Ok(Json(json!({
        "success": true,
        "message": "Folder deleted successfully"
    })))
}
