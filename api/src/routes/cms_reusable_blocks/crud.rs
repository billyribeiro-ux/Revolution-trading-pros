//! Block CRUD handlers: list, get-by-id, get-by-slug, create, update,
//! delete (soft), and duplicate.

use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Json,
};
use uuid::Uuid;

use crate::{models::User, utils::errors::ApiError, AppState};

use super::helpers::{ensure_unique_slug, generate_slug, require_cms_admin, require_cms_editor};
use super::types::{
    ApiResult, ApiResultEmpty, CmsReusableBlock, CmsReusableBlockCategory, CmsReusableBlockSummary,
    CreateReusableBlockRequest, ListReusableBlocksQuery, PaginatedBlocksResponse, PaginationMeta,
    UpdateReusableBlockRequest,
};
use serde_json::json;

/// List reusable blocks with pagination and filtering
///
/// Supports filtering by category, tags, search term, and global status.
/// Results are paginated with customizable sort order.
#[utoipa::path(
    get,
    path = "/api/cms/reusable-blocks",
    tag = "CMS Reusable Blocks",
    params(ListReusableBlocksQuery),
    responses(
        (status = 200, description = "List of reusable blocks", body = PaginatedBlocksResponse),
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Forbidden - editor access required"),
        (status = 500, description = "Internal server error")
    ),
    security(("bearer_auth" = []))
)]
pub(super) async fn list_reusable_blocks(
    State(state): State<AppState>,
    user: User,
    Query(query): Query<ListReusableBlocksQuery>,
) -> ApiResult<PaginatedBlocksResponse> {
    require_cms_editor(&user)?;

    let limit = query.limit.unwrap_or(20).min(100);
    let offset = query.offset.unwrap_or(0);
    let include_deleted = query.include_deleted.unwrap_or(false);

    // Sort configuration - validated against whitelist to prevent SQL injection
    let sort_column = match query.sort_by.as_deref() {
        Some("name") => "name",
        Some("usage_count") => "usage_count",
        Some("updated_at") => "updated_at",
        _ => "created_at",
    };
    let sort_order = match query.sort_order.as_deref() {
        Some("asc") => "ASC",
        _ => "DESC",
    };

    // Prepare search pattern if provided
    let search_pattern = query.search.as_ref().map(|s| format!("%{s}%"));

    // Count total using parameterized query
    let count_result: (i64,) = sqlx::query_as(
        r"
        SELECT COUNT(*)
        FROM cms_reusable_blocks
        WHERE ($1::boolean OR deleted_at IS NULL)
          AND ($2::text IS NULL OR category::text = $2)
          AND ($3::text IS NULL OR name ILIKE $3 OR description ILIKE $3)
          AND ($4::boolean IS NULL OR is_global = $4)
        ",
    )
    .bind(include_deleted)
    .bind(&query.category)
    .bind(&search_pattern)
    .bind(query.is_global)
    .fetch_one(&state.db.pool)
    .await
    .map_err(|e: sqlx::Error| ApiError::new(StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let total = count_result.0;

    // Fetch blocks using parameterized query with safe sort columns
    // Note: sort_column and sort_order are validated against whitelists above
    let blocks: Vec<CmsReusableBlockSummary> = sqlx::query_as(sqlx::AssertSqlSafe(format!(
        r"
        SELECT id, name, slug, description, category, tags, thumbnail_url,
               usage_count, is_global, is_locked, version, created_at, updated_at
        FROM cms_reusable_blocks
        WHERE ($1::boolean OR deleted_at IS NULL)
          AND ($2::text IS NULL OR category::text = $2)
          AND ($3::text IS NULL OR name ILIKE $3 OR description ILIKE $3)
          AND ($4::boolean IS NULL OR is_global = $4)
        ORDER BY {sort_column} {sort_order}
        LIMIT $5 OFFSET $6
        "
    )))
    .bind(include_deleted)
    .bind(&query.category)
    .bind(&search_pattern)
    .bind(query.is_global)
    .bind(limit)
    .bind(offset)
    .fetch_all(&state.db.pool)
    .await
    .map_err(|e: sqlx::Error| ApiError::new(StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(PaginatedBlocksResponse {
        data: blocks,
        meta: PaginationMeta {
            total,
            limit,
            offset,
            has_more: offset + limit < total,
        },
    }))
}

/// Get a reusable block by ID
#[utoipa::path(
    get,
    path = "/api/cms/reusable-blocks/{id}",
    tag = "CMS Reusable Blocks",
    params(
        ("id" = Uuid, Path, description = "Block UUID")
    ),
    responses(
        (status = 200, description = "Reusable block details", body = CmsReusableBlock),
        (status = 404, description = "Block not found"),
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Forbidden")
    ),
    security(("bearer_auth" = []))
)]
pub(super) async fn get_reusable_block(
    State(state): State<AppState>,
    user: User,
    Path(id): Path<Uuid>,
) -> ApiResult<CmsReusableBlock> {
    require_cms_editor(&user)?;

    let block = sqlx::query_as!(
        CmsReusableBlock,
        r#"
        SELECT id, name, slug, description, block_data,
               category as "category: CmsReusableBlockCategory",
               tags, thumbnail_url, usage_count, is_global, is_locked, version,
               deleted_at, created_at, updated_at, created_by
        FROM cms_reusable_blocks
        WHERE id = $1 AND deleted_at IS NULL
        "#,
        id
    )
    .fetch_optional(&state.db.pool)
    .await
    .map_err(|e: sqlx::Error| ApiError::new(StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
    .ok_or_else(|| {
        ApiError::new(StatusCode::NOT_FOUND, "Reusable block not found").with_code("NOT_FOUND")
    })?;

    Ok(Json(block))
}

/// Get a reusable block by slug
#[utoipa::path(
    get,
    path = "/api/cms/reusable-blocks/slug/{slug}",
    tag = "CMS Reusable Blocks",
    params(
        ("slug" = String, Path, description = "Block slug")
    ),
    responses(
        (status = 200, description = "Reusable block details", body = CmsReusableBlock),
        (status = 404, description = "Block not found"),
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Forbidden")
    ),
    security(("bearer_auth" = []))
)]
pub(super) async fn get_reusable_block_by_slug(
    State(state): State<AppState>,
    user: User,
    Path(slug): Path<String>,
) -> ApiResult<CmsReusableBlock> {
    require_cms_editor(&user)?;

    let block = sqlx::query_as!(
        CmsReusableBlock,
        r#"
        SELECT id, name, slug, description, block_data,
               category as "category: CmsReusableBlockCategory",
               tags, thumbnail_url, usage_count, is_global, is_locked, version,
               deleted_at, created_at, updated_at, created_by
        FROM cms_reusable_blocks
        WHERE slug = $1 AND deleted_at IS NULL
        "#,
        slug
    )
    .fetch_optional(&state.db.pool)
    .await
    .map_err(|e: sqlx::Error| ApiError::new(StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
    .ok_or_else(|| {
        ApiError::new(StatusCode::NOT_FOUND, "Reusable block not found").with_code("NOT_FOUND")
    })?;

    Ok(Json(block))
}

/// Create a new reusable block
#[utoipa::path(
    post,
    path = "/api/cms/reusable-blocks",
    tag = "CMS Reusable Blocks",
    request_body = CreateReusableBlockRequest,
    responses(
        (status = 201, description = "Block created successfully", body = CmsReusableBlock),
        (status = 400, description = "Validation error"),
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Forbidden"),
        (status = 409, description = "Slug already exists")
    ),
    security(("bearer_auth" = []))
)]
pub(super) async fn create_reusable_block(
    State(state): State<AppState>,
    user: User,
    Json(request): Json<CreateReusableBlockRequest>,
) -> Result<(StatusCode, Json<CmsReusableBlock>), ApiError> {
    require_cms_editor(&user)?;

    // Validate required fields
    if request.name.trim().is_empty() {
        return Err(ApiError::new(StatusCode::BAD_REQUEST, "Name is required")
            .with_code("VALIDATION_ERROR"));
    }

    // Generate or validate slug
    let base_slug = request
        .slug
        .clone()
        .unwrap_or_else(|| generate_slug(&request.name));
    let slug = ensure_unique_slug(&state.db.pool, &base_slug, None).await?;

    // Get CMS user ID from platform user
    let cms_user_id: Option<Uuid> =
        sqlx::query_scalar!("SELECT id FROM cms_users WHERE user_id = $1", user.id)
            .fetch_optional(&state.db.pool)
            .await
            .map_err(|e: sqlx::Error| {
                ApiError::new(StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
            })?;

    let new_id = Uuid::new_v4();
    let block = sqlx::query_as!(
        CmsReusableBlock,
        r#"
        INSERT INTO cms_reusable_blocks (
            id, name, slug, description, block_data, category, tags,
            thumbnail_url, usage_count, is_global, is_locked, version,
            created_at, updated_at, created_by
        )
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, 0, $9, false, 1, NOW(), NOW(), $10)
        RETURNING id, name, slug, description, block_data,
                  category as "category: CmsReusableBlockCategory",
                  tags, thumbnail_url, usage_count, is_global, is_locked, version,
                  deleted_at, created_at, updated_at, created_by
        "#,
        new_id,
        request.name.trim(),
        slug,
        request.description,
        request.block_data,
        request.category as Option<CmsReusableBlockCategory>,
        request.tags.as_deref(),
        request.thumbnail_url,
        request.is_global,
        cms_user_id
    )
    .fetch_one(&state.db.pool)
    .await
    .map_err(|e: sqlx::Error| ApiError::new(StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    tracing::info!("Reusable block created: {} ({})", block.name, block.id);

    Ok((StatusCode::CREATED, Json(block)))
}

/// Update a reusable block
#[utoipa::path(
    put,
    path = "/api/cms/reusable-blocks/{id}",
    tag = "CMS Reusable Blocks",
    params(
        ("id" = Uuid, Path, description = "Block UUID")
    ),
    request_body = UpdateReusableBlockRequest,
    responses(
        (status = 200, description = "Block updated successfully", body = CmsReusableBlock),
        (status = 400, description = "Validation error"),
        (status = 404, description = "Block not found"),
        (status = 409, description = "Slug already exists"),
        (status = 423, description = "Block is locked")
    ),
    security(("bearer_auth" = []))
)]
pub(super) async fn update_reusable_block(
    State(state): State<AppState>,
    user: User,
    Path(id): Path<Uuid>,
    Json(request): Json<UpdateReusableBlockRequest>,
) -> ApiResult<CmsReusableBlock> {
    require_cms_editor(&user)?;

    // Check if block exists and is not locked
    let existing = sqlx::query!(
        "SELECT is_locked FROM cms_reusable_blocks WHERE id = $1 AND deleted_at IS NULL",
        id
    )
    .fetch_optional(&state.db.pool)
    .await
    .map_err(|e: sqlx::Error| ApiError::new(StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
    .ok_or_else(|| {
        ApiError::new(StatusCode::NOT_FOUND, "Reusable block not found").with_code("NOT_FOUND")
    })?;

    // Only admins can update locked blocks
    if existing.is_locked && request.is_locked != Some(false) {
        require_cms_admin(&user)?;
    }

    // Handle slug update
    let final_slug = if let Some(ref new_slug) = request.slug {
        Some(ensure_unique_slug(&state.db.pool, new_slug, Some(id)).await?)
    } else {
        None
    };

    let block = sqlx::query_as!(
        CmsReusableBlock,
        r#"
        UPDATE cms_reusable_blocks
        SET name = COALESCE($2, name),
            slug = COALESCE($3, slug),
            description = COALESCE($4, description),
            block_data = COALESCE($5, block_data),
            category = COALESCE($6, category),
            tags = COALESCE($7, tags),
            thumbnail_url = COALESCE($8, thumbnail_url),
            is_global = COALESCE($9, is_global),
            is_locked = COALESCE($10, is_locked),
            version = version + 1,
            updated_at = NOW()
        WHERE id = $1 AND deleted_at IS NULL
        RETURNING id, name, slug, description, block_data,
                  category as "category: CmsReusableBlockCategory",
                  tags, thumbnail_url, usage_count, is_global, is_locked, version,
                  deleted_at, created_at, updated_at, created_by
        "#,
        id,
        request.name,
        final_slug,
        request.description,
        request.block_data,
        request.category as Option<CmsReusableBlockCategory>,
        request.tags.as_deref(),
        request.thumbnail_url,
        request.is_global,
        request.is_locked
    )
    .fetch_one(&state.db.pool)
    .await
    .map_err(|e: sqlx::Error| ApiError::new(StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    tracing::info!("Reusable block updated: {} ({})", block.name, block.id);

    Ok(Json(block))
}

/// Delete a reusable block (soft delete)
#[utoipa::path(
    delete,
    path = "/api/cms/reusable-blocks/{id}",
    tag = "CMS Reusable Blocks",
    params(
        ("id" = Uuid, Path, description = "Block UUID")
    ),
    responses(
        (status = 200, description = "Block deleted successfully"),
        (status = 404, description = "Block not found"),
        (status = 423, description = "Block is locked")
    ),
    security(("bearer_auth" = []))
)]
pub(super) async fn delete_reusable_block(
    State(state): State<AppState>,
    user: User,
    Path(id): Path<Uuid>,
) -> ApiResultEmpty {
    require_cms_admin(&user)?;

    // Check if block exists and is not locked
    let existing = sqlx::query!(
        "SELECT is_locked, name FROM cms_reusable_blocks WHERE id = $1 AND deleted_at IS NULL",
        id
    )
    .fetch_optional(&state.db.pool)
    .await
    .map_err(|e: sqlx::Error| ApiError::new(StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
    .ok_or_else(|| {
        ApiError::new(StatusCode::NOT_FOUND, "Reusable block not found").with_code("NOT_FOUND")
    })?;

    if existing.is_locked {
        return Err(ApiError::new(
            StatusCode::LOCKED,
            "Cannot delete a locked block. Unlock it first.",
        )
        .with_code("BLOCK_LOCKED"));
    }

    // Soft delete
    sqlx::query!(
        "UPDATE cms_reusable_blocks SET deleted_at = NOW() WHERE id = $1",
        id
    )
    .execute(&state.db.pool)
    .await
    .map_err(|e: sqlx::Error| ApiError::new(StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    tracing::info!("Reusable block deleted: {} ({})", existing.name, id);

    Ok(Json(json!({
        "message": "Block deleted successfully",
        "id": id
    })))
}

/// Duplicate a reusable block
#[utoipa::path(
    post,
    path = "/api/cms/reusable-blocks/{id}/duplicate",
    tag = "CMS Reusable Blocks",
    params(
        ("id" = Uuid, Path, description = "Block UUID to duplicate")
    ),
    responses(
        (status = 201, description = "Block duplicated successfully", body = CmsReusableBlock),
        (status = 404, description = "Block not found")
    ),
    security(("bearer_auth" = []))
)]
pub(super) async fn duplicate_reusable_block(
    State(state): State<AppState>,
    user: User,
    Path(id): Path<Uuid>,
) -> Result<(StatusCode, Json<CmsReusableBlock>), ApiError> {
    require_cms_editor(&user)?;

    // Fetch the original block
    let original = sqlx::query_as!(
        CmsReusableBlock,
        r#"
        SELECT id, name, slug, description, block_data,
               category as "category: CmsReusableBlockCategory",
               tags, thumbnail_url, usage_count, is_global, is_locked, version,
               deleted_at, created_at, updated_at, created_by
        FROM cms_reusable_blocks
        WHERE id = $1 AND deleted_at IS NULL
        "#,
        id
    )
    .fetch_optional(&state.db.pool)
    .await
    .map_err(|e: sqlx::Error| ApiError::new(StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
    .ok_or_else(|| {
        ApiError::new(StatusCode::NOT_FOUND, "Reusable block not found").with_code("NOT_FOUND")
    })?;

    // Generate new name and slug
    let new_name = format!("{} (Copy)", original.name);
    let base_slug = generate_slug(&new_name);
    let new_slug = ensure_unique_slug(&state.db.pool, &base_slug, None).await?;

    // Get CMS user ID
    let cms_user_id: Option<Uuid> =
        sqlx::query_scalar!("SELECT id FROM cms_users WHERE user_id = $1", user.id)
            .fetch_optional(&state.db.pool)
            .await
            .map_err(|e: sqlx::Error| {
                ApiError::new(StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
            })?;

    let dup_id = Uuid::new_v4();
    let duplicated = sqlx::query_as!(
        CmsReusableBlock,
        r#"
        INSERT INTO cms_reusable_blocks (
            id, name, slug, description, block_data, category, tags,
            thumbnail_url, usage_count, is_global, is_locked, version,
            created_at, updated_at, created_by
        )
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, 0, $9, false, 1, NOW(), NOW(), $10)
        RETURNING id, name, slug, description, block_data,
                  category as "category: CmsReusableBlockCategory",
                  tags, thumbnail_url, usage_count, is_global, is_locked, version,
                  deleted_at, created_at, updated_at, created_by
        "#,
        dup_id,
        new_name,
        new_slug,
        original.description,
        original.block_data,
        original.category as Option<CmsReusableBlockCategory>,
        original.tags.as_deref(),
        original.thumbnail_url,
        original.is_global,
        cms_user_id
    )
    .fetch_one(&state.db.pool)
    .await
    .map_err(|e: sqlx::Error| ApiError::new(StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    tracing::info!(
        "Reusable block duplicated: {} -> {} ({})",
        original.name,
        duplicated.name,
        duplicated.id
    );

    Ok((StatusCode::CREATED, Json(duplicated)))
}
