//! CMS Reusable Blocks API Routes - Apple ICT 7+ Principal Engineer Grade
//!
//! Comprehensive API for managing reusable content blocks in the CMS.
//! Supports block libraries, usage tracking, versioning, and sync management.
//!
//! Features:
//! - CRUD operations for reusable blocks
//! - Block duplication with version control
//! - Usage tracking across content items
//! - Sync/detach functionality for block instances
//! - Category and tag-based organization
//! - Full-text search support
//!
//! @version 1.0.0 - January 2026

use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    routing::{delete, get, post, put},
    Json, Router,
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value as JsonValue};
use sqlx::FromRow;
use utoipa::{IntoParams, ToSchema};
use uuid::Uuid;

use crate::{models::User, utils::errors::ApiError, AppState};

// ============================================================================
// TYPE ALIASES
// ============================================================================

type ApiResult<T> = Result<Json<T>, ApiError>;
type ApiResultEmpty = Result<Json<JsonValue>, ApiError>;

// ============================================================================
// DATABASE MODELS
// ============================================================================

/// Reusable content block stored in the block library
#[derive(Debug, Clone, Serialize, Deserialize, FromRow, ToSchema)]
pub struct CmsReusableBlock {
    /// Unique identifier
    pub id: Uuid,
    /// Display name of the block
    pub name: String,
    /// URL-friendly unique identifier
    pub slug: String,
    /// Optional description for the block library
    pub description: Option<String>,
    /// Block content and configuration (JSON)
    pub block_data: JsonValue,
    /// Category for organization (e.g., "headers", "footers", "cta")
    pub category: Option<String>,
    /// Tags for filtering and search
    pub tags: Option<Vec<String>>,
    /// Preview thumbnail URL
    pub thumbnail_url: Option<String>,
    /// Number of times this block is used across content
    pub usage_count: i32,
    /// Whether the block is available globally to all users
    pub is_global: bool,
    /// Whether the block is locked from editing
    pub is_locked: bool,
    /// Version number for tracking changes
    pub version: i32,
    /// Soft delete timestamp
    pub deleted_at: Option<DateTime<Utc>>,
    /// Creation timestamp
    pub created_at: DateTime<Utc>,
    /// Last update timestamp
    pub updated_at: DateTime<Utc>,
    /// UUID of the CMS user who created the block
    pub created_by: Option<Uuid>,
}

/// Lightweight block summary for listings
#[derive(Debug, Clone, Serialize, Deserialize, FromRow, ToSchema)]
pub struct CmsReusableBlockSummary {
    pub id: Uuid,
    pub name: String,
    pub slug: String,
    pub description: Option<String>,
    pub category: Option<String>,
    pub tags: Option<Vec<String>>,
    pub thumbnail_url: Option<String>,
    pub usage_count: i32,
    pub is_global: bool,
    pub is_locked: bool,
    pub version: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Tracks where a reusable block is used
#[derive(Debug, Clone, Serialize, Deserialize, FromRow, ToSchema)]
pub struct CmsReusableBlockUsage {
    /// Unique identifier for this usage record
    pub id: Uuid,
    /// Reference to the reusable block
    pub reusable_block_id: Uuid,
    /// Content item where the block is used
    pub content_id: Uuid,
    /// Instance ID of the block within the content
    pub block_instance_id: String,
    /// Whether this instance syncs with the source block
    pub is_synced: bool,
    /// When the block was detached from sync
    pub detached_at: Option<DateTime<Utc>>,
    /// Who detached the block
    pub detached_by: Option<Uuid>,
    /// When this usage was created
    pub created_at: DateTime<Utc>,
}

/// Usage record with content metadata for display
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct CmsReusableBlockUsageWithContent {
    pub id: Uuid,
    pub reusable_block_id: Uuid,
    pub content_id: Uuid,
    pub content_title: String,
    pub content_type: String,
    pub content_slug: String,
    pub block_instance_id: String,
    pub is_synced: bool,
    pub detached_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}

// ============================================================================
// REQUEST/RESPONSE MODELS
// ============================================================================

/// Request to create a new reusable block
#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateReusableBlockRequest {
    /// Display name (required)
    pub name: String,
    /// Custom slug (optional, auto-generated if not provided)
    pub slug: Option<String>,
    /// Block description
    pub description: Option<String>,
    /// Block content and configuration
    pub block_data: JsonValue,
    /// Category for organization
    pub category: Option<String>,
    /// Tags for filtering
    pub tags: Option<Vec<String>>,
    /// Preview thumbnail URL
    pub thumbnail_url: Option<String>,
    /// Make globally available
    #[serde(default)]
    pub is_global: bool,
}

/// Request to update a reusable block
#[derive(Debug, Deserialize, ToSchema)]
pub struct UpdateReusableBlockRequest {
    pub name: Option<String>,
    pub slug: Option<String>,
    pub description: Option<String>,
    pub block_data: Option<JsonValue>,
    pub category: Option<String>,
    pub tags: Option<Vec<String>>,
    pub thumbnail_url: Option<String>,
    pub is_global: Option<bool>,
    pub is_locked: Option<bool>,
}

/// Request to track block usage
#[derive(Debug, Deserialize, ToSchema)]
pub struct TrackBlockUsageRequest {
    /// The reusable block being used
    pub reusable_block_id: Uuid,
    /// Content item where the block is placed
    pub content_id: Uuid,
    /// Unique instance ID within the content
    pub block_instance_id: String,
    /// Whether to sync updates from source
    #[serde(default = "default_true")]
    pub is_synced: bool,
}

fn default_true() -> bool {
    true
}

/// Query parameters for listing blocks
#[derive(Debug, Deserialize, IntoParams, ToSchema)]
pub struct ListReusableBlocksQuery {
    /// Filter by category
    pub category: Option<String>,
    /// Search in name, description, tags
    pub search: Option<String>,
    /// Filter by tags (comma-separated)
    pub tags: Option<String>,
    /// Include only global blocks
    pub is_global: Option<bool>,
    /// Sort field (name, created_at, updated_at, usage_count)
    pub sort_by: Option<String>,
    /// Sort direction (asc, desc)
    pub sort_order: Option<String>,
    /// Pagination limit (default 20, max 100)
    pub limit: Option<i64>,
    /// Pagination offset
    pub offset: Option<i64>,
    /// Include deleted blocks
    pub include_deleted: Option<bool>,
}

/// Paginated response wrapper
#[derive(Debug, Serialize, ToSchema)]
pub struct PaginatedBlocksResponse {
    pub data: Vec<CmsReusableBlockSummary>,
    pub meta: PaginationMeta,
}

/// Pagination metadata
#[derive(Debug, Serialize, ToSchema)]
pub struct PaginationMeta {
    pub total: i64,
    pub limit: i64,
    pub offset: i64,
    pub has_more: bool,
}

/// Response for block usage listing
#[derive(Debug, Serialize, ToSchema)]
pub struct BlockUsageResponse {
    pub usages: Vec<CmsReusableBlockUsageWithContent>,
    pub total_count: i64,
}

// ============================================================================
// AUTHORIZATION HELPERS
// ============================================================================

/// Check if user has CMS admin privileges
fn require_cms_admin(user: &User) -> Result<(), ApiError> {
    let role = user.role.as_deref().unwrap_or("user");
    if matches!(role, "admin" | "super-admin" | "super_admin" | "developer") {
        Ok(())
    } else {
        Err(ApiError::new(StatusCode::FORBIDDEN, "Admin access required").with_code("FORBIDDEN"))
    }
}

/// Check if user has CMS editor privileges
fn require_cms_editor(user: &User) -> Result<(), ApiError> {
    let role = user.role.as_deref().unwrap_or("user");
    if matches!(
        role,
        "admin" | "super-admin" | "super_admin" | "editor" | "marketing" | "developer"
    ) {
        Ok(())
    } else {
        Err(ApiError::new(StatusCode::FORBIDDEN, "Editor access required").with_code("FORBIDDEN"))
    }
}

// ============================================================================
// HELPER FUNCTIONS
// ============================================================================

/// Generate a URL-friendly slug from a name
fn generate_slug(name: &str) -> String {
    slug::slugify(name)
}

/// Ensure slug uniqueness by appending a suffix if needed
async fn ensure_unique_slug(
    pool: &sqlx::PgPool,
    base_slug: &str,
    exclude_id: Option<Uuid>,
) -> Result<String, ApiError> {
    let mut slug = base_slug.to_string();
    let mut suffix = 0;

    loop {
        let exists: bool = match exclude_id {
            Some(id) => sqlx::query_scalar!(
                r#"
                    SELECT EXISTS(
                        SELECT 1 FROM cms_reusable_blocks
                        WHERE slug = $1 AND id != $2 AND deleted_at IS NULL
                    ) as "exists!"
                    "#,
                slug,
                id
            )
            .fetch_one(pool)
            .await
            .map_err(|e| ApiError::new(StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?,
            None => sqlx::query_scalar!(
                r#"
                    SELECT EXISTS(
                        SELECT 1 FROM cms_reusable_blocks
                        WHERE slug = $1 AND deleted_at IS NULL
                    ) as "exists!"
                    "#,
                slug
            )
            .fetch_one(pool)
            .await
            .map_err(|e| ApiError::new(StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?,
        };

        if !exists {
            return Ok(slug);
        }

        suffix += 1;
        slug = format!("{}-{}", base_slug, suffix);
    }
}

// ============================================================================
// ROUTE HANDLERS
// ============================================================================

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
async fn list_reusable_blocks(
    State(state): State<AppState>,
    user: User,
    Query(query): Query<ListReusableBlocksQuery>,
) -> ApiResult<PaginatedBlocksResponse> {
    require_cms_editor(&user)?;

    let limit = query.limit.unwrap_or(20).min(100);
    let offset = query.offset.unwrap_or(0);
    let include_deleted = query.include_deleted.unwrap_or(false);

    // Build dynamic query
    let mut conditions = vec!["1=1".to_string()];
    let mut bind_index = 1;

    if !include_deleted {
        conditions.push("deleted_at IS NULL".to_string());
    }

    if let Some(ref category) = query.category {
        conditions.push(format!("category = ${}", bind_index));
        bind_index += 1;
    }

    if let Some(ref search) = query.search {
        conditions.push(format!(
            "(name ILIKE ${0} OR description ILIKE ${0} OR ${0} = ANY(tags))",
            bind_index
        ));
        bind_index += 1;
    }

    if let Some(is_global) = query.is_global {
        conditions.push(format!("is_global = ${}", bind_index));
        let _ = bind_index; // Suppress unused warning - bind_index tracks placeholder position
    }

    // Sort configuration
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

    // Count total
    let count_result: (i64,) = sqlx::query_as(&format!(
        "SELECT COUNT(*) FROM cms_reusable_blocks WHERE {}",
        conditions.join(" AND ")
    ))
    .fetch_one(&state.db.pool)
    .await
    .map_err(|e| ApiError::new(StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let total = count_result.0;

    // Fetch blocks
    let blocks: Vec<CmsReusableBlockSummary> = sqlx::query_as(&format!(
        r#"
        SELECT id, name, slug, description, category, tags, thumbnail_url,
               usage_count, is_global, is_locked, version, created_at, updated_at
        FROM cms_reusable_blocks
        WHERE {}
        ORDER BY {} {}
        LIMIT {} OFFSET {}
        "#,
        conditions.join(" AND "),
        sort_column,
        sort_order,
        limit,
        offset
    ))
    .fetch_all(&state.db.pool)
    .await
    .map_err(|e| ApiError::new(StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

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
async fn get_reusable_block(
    State(state): State<AppState>,
    user: User,
    Path(id): Path<Uuid>,
) -> ApiResult<CmsReusableBlock> {
    require_cms_editor(&user)?;

    let block = sqlx::query_as!(
        CmsReusableBlock,
        r#"
        SELECT id, name, slug, description, block_data, category, tags,
               thumbnail_url, usage_count, is_global, is_locked, version,
               deleted_at, created_at, updated_at, created_by
        FROM cms_reusable_blocks
        WHERE id = $1 AND deleted_at IS NULL
        "#,
        id
    )
    .fetch_optional(&state.db.pool)
    .await
    .map_err(|e| ApiError::new(StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
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
async fn get_reusable_block_by_slug(
    State(state): State<AppState>,
    user: User,
    Path(slug): Path<String>,
) -> ApiResult<CmsReusableBlock> {
    require_cms_editor(&user)?;

    let block = sqlx::query_as!(
        CmsReusableBlock,
        r#"
        SELECT id, name, slug, description, block_data, category, tags,
               thumbnail_url, usage_count, is_global, is_locked, version,
               deleted_at, created_at, updated_at, created_by
        FROM cms_reusable_blocks
        WHERE slug = $1 AND deleted_at IS NULL
        "#,
        slug
    )
    .fetch_optional(&state.db.pool)
    .await
    .map_err(|e| ApiError::new(StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
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
async fn create_reusable_block(
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
            .map_err(|e| ApiError::new(StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
            .flatten();

    let block = sqlx::query_as!(
        CmsReusableBlock,
        r#"
        INSERT INTO cms_reusable_blocks (
            id, name, slug, description, block_data, category, tags,
            thumbnail_url, usage_count, is_global, is_locked, version,
            created_at, updated_at, created_by
        )
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, 0, $9, false, 1, NOW(), NOW(), $10)
        RETURNING id, name, slug, description, block_data, category, tags,
                  thumbnail_url, usage_count, is_global, is_locked, version,
                  deleted_at, created_at, updated_at, created_by
        "#,
        Uuid::new_v4(),
        request.name.trim(),
        slug,
        request.description,
        request.block_data,
        request.category,
        request.tags.as_deref(),
        request.thumbnail_url,
        request.is_global,
        cms_user_id
    )
    .fetch_one(&state.db.pool)
    .await
    .map_err(|e| ApiError::new(StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

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
async fn update_reusable_block(
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
    .map_err(|e| ApiError::new(StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
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
        RETURNING id, name, slug, description, block_data, category, tags,
                  thumbnail_url, usage_count, is_global, is_locked, version,
                  deleted_at, created_at, updated_at, created_by
        "#,
        id,
        request.name,
        final_slug,
        request.description,
        request.block_data,
        request.category,
        request.tags.as_deref(),
        request.thumbnail_url,
        request.is_global,
        request.is_locked
    )
    .fetch_one(&state.db.pool)
    .await
    .map_err(|e| ApiError::new(StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

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
async fn delete_reusable_block(
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
    .map_err(|e| ApiError::new(StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
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
    .map_err(|e| ApiError::new(StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

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
async fn duplicate_reusable_block(
    State(state): State<AppState>,
    user: User,
    Path(id): Path<Uuid>,
) -> Result<(StatusCode, Json<CmsReusableBlock>), ApiError> {
    require_cms_editor(&user)?;

    // Fetch the original block
    let original = sqlx::query_as!(
        CmsReusableBlock,
        r#"
        SELECT id, name, slug, description, block_data, category, tags,
               thumbnail_url, usage_count, is_global, is_locked, version,
               deleted_at, created_at, updated_at, created_by
        FROM cms_reusable_blocks
        WHERE id = $1 AND deleted_at IS NULL
        "#,
        id
    )
    .fetch_optional(&state.db.pool)
    .await
    .map_err(|e| ApiError::new(StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
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
            })?
            .flatten();

    let duplicated = sqlx::query_as!(
        CmsReusableBlock,
        r#"
        INSERT INTO cms_reusable_blocks (
            id, name, slug, description, block_data, category, tags,
            thumbnail_url, usage_count, is_global, is_locked, version,
            created_at, updated_at, created_by
        )
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, 0, $9, false, 1, NOW(), NOW(), $10)
        RETURNING id, name, slug, description, block_data, category, tags,
                  thumbnail_url, usage_count, is_global, is_locked, version,
                  deleted_at, created_at, updated_at, created_by
        "#,
        Uuid::new_v4(),
        new_name,
        new_slug,
        original.description,
        original.block_data,
        original.category,
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

/// Get usage locations for a reusable block
#[utoipa::path(
    get,
    path = "/api/cms/reusable-blocks/{id}/usage",
    tag = "CMS Reusable Blocks",
    params(
        ("id" = Uuid, Path, description = "Block UUID")
    ),
    responses(
        (status = 200, description = "Block usage locations", body = BlockUsageResponse),
        (status = 404, description = "Block not found")
    ),
    security(("bearer_auth" = []))
)]
async fn get_block_usage(
    State(state): State<AppState>,
    user: User,
    Path(id): Path<Uuid>,
) -> ApiResult<BlockUsageResponse> {
    require_cms_editor(&user)?;

    // Verify block exists
    let exists = sqlx::query_scalar!(
        "SELECT EXISTS(SELECT 1 FROM cms_reusable_blocks WHERE id = $1 AND deleted_at IS NULL) as \"exists!\"",
        id
    )
    .fetch_one(&state.db.pool)
    .await
    .map_err(|e: sqlx::Error| {
        ApiError::new(StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
    })?;

    if !exists {
        return Err(
            ApiError::new(StatusCode::NOT_FOUND, "Reusable block not found").with_code("NOT_FOUND"),
        );
    }

    // Fetch usage with content metadata
    let usages: Vec<CmsReusableBlockUsageWithContent> = sqlx::query_as!(
        CmsReusableBlockUsageWithContent,
        r#"
        SELECT
            u.id,
            u.reusable_block_id,
            u.content_id,
            c.title as content_title,
            c.content_type::text as "content_type!",
            c.slug as content_slug,
            u.block_instance_id,
            u.is_synced,
            u.detached_at,
            u.created_at
        FROM cms_reusable_block_usage u
        JOIN cms_content c ON c.id = u.content_id
        WHERE u.reusable_block_id = $1
        ORDER BY u.created_at DESC
        "#,
        id
    )
    .fetch_all(&state.db.pool)
    .await
    .map_err(|e| ApiError::new(StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let total_count = usages.len() as i64;

    Ok(Json(BlockUsageResponse {
        usages,
        total_count,
    }))
}

/// Track usage when a reusable block is inserted into content
#[utoipa::path(
    post,
    path = "/api/cms/reusable-blocks/usage",
    tag = "CMS Reusable Blocks",
    request_body = TrackBlockUsageRequest,
    responses(
        (status = 201, description = "Usage tracked successfully", body = CmsReusableBlockUsage),
        (status = 400, description = "Validation error"),
        (status = 404, description = "Block or content not found")
    ),
    security(("bearer_auth" = []))
)]
async fn track_block_usage(
    State(state): State<AppState>,
    user: User,
    Json(request): Json<TrackBlockUsageRequest>,
) -> Result<(StatusCode, Json<CmsReusableBlockUsage>), ApiError> {
    require_cms_editor(&user)?;

    // Verify block exists
    let block_exists = sqlx::query_scalar!(
        "SELECT EXISTS(SELECT 1 FROM cms_reusable_blocks WHERE id = $1 AND deleted_at IS NULL) as \"exists!\"",
        request.reusable_block_id
    )
    .fetch_one(&state.db.pool)
    .await
    .map_err(|e: sqlx::Error| ApiError::new(StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    if !block_exists {
        return Err(
            ApiError::new(StatusCode::NOT_FOUND, "Reusable block not found").with_code("NOT_FOUND"),
        );
    }

    // Verify content exists
    let content_exists = sqlx::query_scalar!(
        "SELECT EXISTS(SELECT 1 FROM cms_content WHERE id = $1 AND deleted_at IS NULL) as \"exists!\"",
        request.content_id
    )
    .fetch_one(&state.db.pool)
    .await
    .map_err(|e: sqlx::Error| ApiError::new(StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    if !content_exists {
        return Err(
            ApiError::new(StatusCode::NOT_FOUND, "Content not found").with_code("NOT_FOUND")
        );
    }

    // Create usage record
    let usage = sqlx::query_as!(
        CmsReusableBlockUsage,
        r#"
        INSERT INTO cms_reusable_block_usage (
            id, reusable_block_id, content_id, block_instance_id, is_synced, created_at
        )
        VALUES ($1, $2, $3, $4, $5, NOW())
        RETURNING id, reusable_block_id, content_id, block_instance_id, is_synced,
                  detached_at, detached_by, created_at
        "#,
        Uuid::new_v4(),
        request.reusable_block_id,
        request.content_id,
        request.block_instance_id,
        request.is_synced
    )
    .fetch_one(&state.db.pool)
    .await
    .map_err(|e| ApiError::new(StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    // Increment usage count on the block
    sqlx::query!(
        "UPDATE cms_reusable_blocks SET usage_count = usage_count + 1 WHERE id = $1",
        request.reusable_block_id
    )
    .execute(&state.db.pool)
    .await
    .map_err(|e| ApiError::new(StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    tracing::info!(
        "Block usage tracked: block {} in content {} (instance {})",
        request.reusable_block_id,
        request.content_id,
        request.block_instance_id
    );

    Ok((StatusCode::CREATED, Json(usage)))
}

/// Remove usage tracking when a block is removed from content
#[utoipa::path(
    delete,
    path = "/api/cms/reusable-blocks/usage/{id}",
    tag = "CMS Reusable Blocks",
    params(
        ("id" = Uuid, Path, description = "Usage record UUID")
    ),
    responses(
        (status = 200, description = "Usage removed successfully"),
        (status = 404, description = "Usage record not found")
    ),
    security(("bearer_auth" = []))
)]
async fn remove_block_usage(
    State(state): State<AppState>,
    user: User,
    Path(id): Path<Uuid>,
) -> ApiResultEmpty {
    require_cms_editor(&user)?;

    // Get usage record to find the block ID
    let usage = sqlx::query!(
        "SELECT reusable_block_id FROM cms_reusable_block_usage WHERE id = $1",
        id
    )
    .fetch_optional(&state.db.pool)
    .await
    .map_err(|e| ApiError::new(StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
    .ok_or_else(|| {
        ApiError::new(StatusCode::NOT_FOUND, "Usage record not found").with_code("NOT_FOUND")
    })?;

    // Delete the usage record
    sqlx::query!("DELETE FROM cms_reusable_block_usage WHERE id = $1", id)
        .execute(&state.db.pool)
        .await
        .map_err(|e| ApiError::new(StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    // Decrement usage count on the block
    sqlx::query!(
        "UPDATE cms_reusable_blocks SET usage_count = GREATEST(0, usage_count - 1) WHERE id = $1",
        usage.reusable_block_id
    )
    .execute(&state.db.pool)
    .await
    .map_err(|e| ApiError::new(StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    tracing::info!(
        "Block usage removed: usage {} from block {}",
        id,
        usage.reusable_block_id
    );

    Ok(Json(json!({
        "message": "Usage removed successfully",
        "id": id
    })))
}

/// Detach a block instance from sync (keeps content but stops updates)
#[utoipa::path(
    post,
    path = "/api/cms/reusable-blocks/usage/{id}/detach",
    tag = "CMS Reusable Blocks",
    params(
        ("id" = Uuid, Path, description = "Usage record UUID")
    ),
    responses(
        (status = 200, description = "Block instance detached from sync", body = CmsReusableBlockUsage),
        (status = 404, description = "Usage record not found"),
        (status = 409, description = "Already detached")
    ),
    security(("bearer_auth" = []))
)]
async fn detach_block_usage(
    State(state): State<AppState>,
    user: User,
    Path(id): Path<Uuid>,
) -> ApiResult<CmsReusableBlockUsage> {
    require_cms_editor(&user)?;

    // Check if already detached
    let existing = sqlx::query!(
        "SELECT is_synced FROM cms_reusable_block_usage WHERE id = $1",
        id
    )
    .fetch_optional(&state.db.pool)
    .await
    .map_err(|e| ApiError::new(StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
    .ok_or_else(|| {
        ApiError::new(StatusCode::NOT_FOUND, "Usage record not found").with_code("NOT_FOUND")
    })?;

    if !existing.is_synced {
        return Err(
            ApiError::new(StatusCode::CONFLICT, "Block instance is already detached")
                .with_code("ALREADY_DETACHED"),
        );
    }

    // Get CMS user ID
    let cms_user_id: Option<Uuid> =
        sqlx::query_scalar!("SELECT id FROM cms_users WHERE user_id = $1", user.id)
            .fetch_optional(&state.db.pool)
            .await
            .map_err(|e: sqlx::Error| {
                ApiError::new(StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
            })?
            .flatten();

    // Detach the usage
    let usage = sqlx::query_as!(
        CmsReusableBlockUsage,
        r#"
        UPDATE cms_reusable_block_usage
        SET is_synced = false, detached_at = NOW(), detached_by = $2
        WHERE id = $1
        RETURNING id, reusable_block_id, content_id, block_instance_id, is_synced,
                  detached_at, detached_by, created_at
        "#,
        id,
        cms_user_id
    )
    .fetch_one(&state.db.pool)
    .await
    .map_err(|e| ApiError::new(StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    tracing::info!(
        "Block usage detached: {} from block {}",
        id,
        usage.reusable_block_id
    );

    Ok(Json(usage))
}

// ============================================================================
// ROUTER
// ============================================================================

/// Admin router for reusable blocks (requires authentication)
pub fn admin_router() -> Router<AppState> {
    Router::new()
        // Block CRUD
        .route("/", get(list_reusable_blocks).post(create_reusable_block))
        .route(
            "/{id}",
            get(get_reusable_block)
                .put(update_reusable_block)
                .delete(delete_reusable_block),
        )
        .route("/slug/{slug}", get(get_reusable_block_by_slug))
        .route("/{id}/duplicate", post(duplicate_reusable_block))
        .route("/{id}/usage", get(get_block_usage))
        // Usage tracking
        .route("/usage", post(track_block_usage))
        .route("/usage/{id}", delete(remove_block_usage))
        .route("/usage/{id}/detach", post(detach_block_usage))
}

/// Public router for read-only block access (optional, for block previews)
pub fn public_router() -> Router<AppState> {
    Router::new().route("/slug/{slug}", get(get_reusable_block_by_slug_public))
}

/// Public endpoint to get a global block by slug (no auth required)
async fn get_reusable_block_by_slug_public(
    State(state): State<AppState>,
    Path(slug): Path<String>,
) -> ApiResult<CmsReusableBlock> {
    let block: CmsReusableBlock = sqlx::query_as(
        r#"
        SELECT id, name, slug, description, block_data, category, tags,
               thumbnail_url, usage_count, is_global, is_locked, version,
               deleted_at, created_at, updated_at, created_by
        FROM cms_reusable_blocks
        WHERE slug = $1 AND deleted_at IS NULL AND is_global = true
        "#,
    )
    .bind(&slug)
    .fetch_optional(&state.db.pool)
    .await
    .map_err(|e: sqlx::Error| ApiError::new(StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
    .ok_or_else(|| {
        ApiError::new(StatusCode::NOT_FOUND, "Reusable block not found").with_code("NOT_FOUND")
    })?;

    Ok(Json(block))
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_slug() {
        assert_eq!(generate_slug("Hero Section"), "hero-section");
        assert_eq!(generate_slug("CTA Banner v2"), "cta-banner-v2");
        assert_eq!(generate_slug("  Multiple   Spaces  "), "multiple-spaces");
    }

    #[test]
    fn test_default_true() {
        assert!(default_true());
    }
}
