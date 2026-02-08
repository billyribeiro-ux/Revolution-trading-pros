//! CMS Presets API Routes - Component Presets/Templates
//!
//! Apple ICT 7+ Principal Engineer Grade - February 2026
//!
//! Comprehensive API for managing block presets/templates in the CMS.
//! Provides pre-designed block configurations for quick content creation.
//!
//! Features:
//! - CRUD operations for presets
//! - Block type filtering
//! - Category organization (default, brand, seasonal, etc.)
//! - Thumbnail preview support
//! - Usage tracking
//! - Search and filtering
//!
//! @version 1.0.0 - February 2026

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

/// Preset category enum - maps directly to PostgreSQL cms_preset_category
#[derive(
    Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize, sqlx::Type, ToSchema,
)]
#[sqlx(type_name = "cms_preset_category", rename_all = "lowercase")]
#[serde(rename_all = "lowercase")]
pub enum CmsPresetCategory {
    #[default]
    Default,
    Brand,
    Seasonal,
    Marketing,
    Trading,
    Custom,
}

/// Component preset stored in the preset library
#[derive(Debug, Clone, Serialize, Deserialize, FromRow, ToSchema)]
pub struct CmsPreset {
    /// Unique identifier
    pub id: Uuid,
    /// Block type this preset applies to (e.g., button, heading)
    pub block_type: String,
    /// Display name of the preset
    pub name: String,
    /// URL-friendly unique identifier
    pub slug: String,
    /// Optional description
    pub description: Option<String>,
    /// Preset content and settings (JSON)
    pub preset_data: JsonValue,
    /// Preview thumbnail URL
    pub thumbnail_url: Option<String>,
    /// Thumbnail blurhash for loading
    pub thumbnail_blurhash: Option<String>,
    /// Category for organization
    pub category: CmsPresetCategory,
    /// Tags for filtering and search
    pub tags: Option<Vec<String>>,
    /// Whether this is the default preset for the block type
    pub is_default: bool,
    /// Whether the preset is locked from editing
    pub is_locked: bool,
    /// Whether available globally to all users
    pub is_global: bool,
    /// Number of times this preset has been used
    pub usage_count: i32,
    /// Version number for tracking changes
    pub version: i32,
    /// Creation timestamp
    pub created_at: DateTime<Utc>,
    /// Last update timestamp
    pub updated_at: DateTime<Utc>,
    /// UUID of the CMS user who created the preset
    pub created_by: Option<Uuid>,
    /// UUID of the CMS user who last updated the preset
    pub updated_by: Option<Uuid>,
    /// Soft delete timestamp
    pub deleted_at: Option<DateTime<Utc>>,
}

/// Lightweight preset summary for listings
#[derive(Debug, Clone, Serialize, Deserialize, FromRow, ToSchema)]
pub struct CmsPresetSummary {
    pub id: Uuid,
    pub block_type: String,
    pub name: String,
    pub slug: String,
    pub description: Option<String>,
    pub thumbnail_url: Option<String>,
    pub thumbnail_blurhash: Option<String>,
    pub category: String,
    pub tags: Option<Vec<String>>,
    pub is_default: bool,
    pub is_locked: bool,
    pub is_global: bool,
    pub usage_count: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Block type with preset count
#[derive(Debug, Clone, Serialize, Deserialize, FromRow, ToSchema)]
pub struct BlockTypeWithPresets {
    pub block_type: String,
    pub preset_count: i64,
    pub has_default: bool,
}

// ============================================================================
// REQUEST/RESPONSE MODELS
// ============================================================================

/// Request to create a new preset
#[derive(Debug, Deserialize, ToSchema)]
pub struct CreatePresetRequest {
    /// Block type this preset applies to (required)
    pub block_type: String,
    /// Display name (required)
    pub name: String,
    /// Custom slug (optional, auto-generated if not provided)
    pub slug: Option<String>,
    /// Description
    pub description: Option<String>,
    /// Preset content and settings
    pub preset_data: JsonValue,
    /// Preview thumbnail URL
    pub thumbnail_url: Option<String>,
    /// Thumbnail blurhash
    pub thumbnail_blurhash: Option<String>,
    /// Category for organization
    pub category: Option<CmsPresetCategory>,
    /// Tags for filtering
    pub tags: Option<Vec<String>>,
    /// Mark as default for this block type
    #[serde(default)]
    pub is_default: bool,
    /// Make globally available
    #[serde(default = "default_true")]
    pub is_global: bool,
}

/// Request to update a preset
#[derive(Debug, Deserialize, ToSchema)]
pub struct UpdatePresetRequest {
    pub name: Option<String>,
    pub slug: Option<String>,
    pub description: Option<String>,
    pub preset_data: Option<JsonValue>,
    pub thumbnail_url: Option<String>,
    pub thumbnail_blurhash: Option<String>,
    pub category: Option<CmsPresetCategory>,
    pub tags: Option<Vec<String>>,
    pub is_default: Option<bool>,
    pub is_global: Option<bool>,
    pub is_locked: Option<bool>,
}

fn default_true() -> bool {
    true
}

/// Query parameters for listing presets
#[derive(Debug, Deserialize, IntoParams, ToSchema)]
pub struct ListPresetsQuery {
    /// Filter by block type
    pub block_type: Option<String>,
    /// Filter by category
    pub category: Option<String>,
    /// Search in name, description, tags
    pub search: Option<String>,
    /// Filter by tags (comma-separated)
    pub tags: Option<String>,
    /// Include only global presets
    pub is_global: Option<bool>,
    /// Include only default presets
    pub is_default: Option<bool>,
    /// Sort field (name, created_at, updated_at, usage_count)
    pub sort_by: Option<String>,
    /// Sort direction (asc, desc)
    pub sort_order: Option<String>,
    /// Pagination limit (default 50, max 200)
    pub limit: Option<i64>,
    /// Pagination offset
    pub offset: Option<i64>,
}

/// Paginated response wrapper
#[derive(Debug, Serialize, ToSchema)]
pub struct PaginatedPresetsResponse {
    pub data: Vec<CmsPresetSummary>,
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

/// Response for block types with presets
#[derive(Debug, Serialize, ToSchema)]
pub struct BlockTypesWithPresetsResponse {
    pub data: Vec<BlockTypeWithPresets>,
}

/// Response for presets grouped by category
#[derive(Debug, Serialize, ToSchema)]
pub struct PresetsByCategory {
    pub category: String,
    pub presets: Vec<CmsPresetSummary>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct GroupedPresetsResponse {
    pub block_type: String,
    pub categories: Vec<PresetsByCategory>,
    pub total_count: i64,
}

// ============================================================================
// AUTHORIZATION HELPERS
// ============================================================================

/// Check if user has CMS admin privileges
#[allow(clippy::result_large_err)]
fn require_cms_admin(user: &User) -> Result<(), ApiError> {
    let role = user.role.as_deref().unwrap_or("user");
    if matches!(role, "admin" | "super-admin" | "super_admin" | "developer") {
        Ok(())
    } else {
        Err(ApiError::new(StatusCode::FORBIDDEN, "Admin access required").with_code("FORBIDDEN"))
    }
}

/// Check if user has CMS editor privileges
#[allow(clippy::result_large_err)]
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
    block_type: &str,
    base_slug: &str,
    exclude_id: Option<Uuid>,
) -> Result<String, ApiError> {
    let mut slug = base_slug.to_string();
    let mut suffix = 0;

    loop {
        let exists: bool = match exclude_id {
            Some(id) => {
                let result: (bool,) = sqlx::query_as(
                    r#"
                    SELECT EXISTS(
                        SELECT 1 FROM cms_presets
                        WHERE block_type = $1 AND slug = $2 AND id != $3 AND deleted_at IS NULL
                    )
                    "#,
                )
                .bind(block_type)
                .bind(&slug)
                .bind(id)
                .fetch_one(pool)
                .await
                .map_err(|e: sqlx::Error| {
                    ApiError::new(StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
                })?;
                result.0
            }
            None => {
                let result: (bool,) = sqlx::query_as(
                    r#"
                    SELECT EXISTS(
                        SELECT 1 FROM cms_presets
                        WHERE block_type = $1 AND slug = $2 AND deleted_at IS NULL
                    )
                    "#,
                )
                .bind(block_type)
                .bind(&slug)
                .fetch_one(pool)
                .await
                .map_err(|e: sqlx::Error| {
                    ApiError::new(StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
                })?;
                result.0
            }
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

/// List presets with pagination and filtering
///
/// Supports filtering by block type, category, tags, and search term.
/// Results are paginated with customizable sort order.
#[utoipa::path(
    get,
    path = "/api/cms/presets",
    tag = "CMS Presets",
    params(ListPresetsQuery),
    responses(
        (status = 200, description = "List of presets", body = PaginatedPresetsResponse),
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Forbidden - editor access required"),
        (status = 500, description = "Internal server error")
    ),
    security(("bearer_auth" = []))
)]
async fn list_presets(
    State(state): State<AppState>,
    user: User,
    Query(query): Query<ListPresetsQuery>,
) -> ApiResult<PaginatedPresetsResponse> {
    require_cms_editor(&user)?;

    let limit = query.limit.unwrap_or(50).min(200);
    let offset = query.offset.unwrap_or(0);

    // Sort configuration - validated against whitelist to prevent SQL injection
    let sort_column = match query.sort_by.as_deref() {
        Some("name") => "name",
        Some("usage_count") => "usage_count",
        Some("updated_at") => "updated_at",
        Some("block_type") => "block_type",
        _ => "created_at",
    };
    let sort_order = match query.sort_order.as_deref() {
        Some("asc") => "ASC",
        _ => "DESC",
    };

    // Prepare search pattern if provided
    let search_pattern = query.search.as_ref().map(|s| format!("%{}%", s));

    // Count total using parameterized query
    let count_result: (i64,) = sqlx::query_as(
        r#"
        SELECT COUNT(*)
        FROM cms_presets
        WHERE deleted_at IS NULL
          AND ($1::text IS NULL OR block_type = $1)
          AND ($2::text IS NULL OR category::text = $2)
          AND ($3::text IS NULL OR name ILIKE $3 OR description ILIKE $3)
          AND ($4::boolean IS NULL OR is_global = $4)
          AND ($5::boolean IS NULL OR is_default = $5)
        "#,
    )
    .bind(&query.block_type)
    .bind(&query.category)
    .bind(&search_pattern)
    .bind(query.is_global)
    .bind(query.is_default)
    .fetch_one(&state.db.pool)
    .await
    .map_err(|e: sqlx::Error| ApiError::new(StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let total = count_result.0;

    // Fetch presets using parameterized query with safe sort columns
    let presets: Vec<CmsPresetSummary> = sqlx::query_as(&format!(
        r#"
        SELECT id, block_type, name, slug, description, thumbnail_url, thumbnail_blurhash,
               category::text as category, tags, is_default, is_locked, is_global,
               usage_count, created_at, updated_at
        FROM cms_presets
        WHERE deleted_at IS NULL
          AND ($1::text IS NULL OR block_type = $1)
          AND ($2::text IS NULL OR category::text = $2)
          AND ($3::text IS NULL OR name ILIKE $3 OR description ILIKE $3)
          AND ($4::boolean IS NULL OR is_global = $4)
          AND ($5::boolean IS NULL OR is_default = $5)
        ORDER BY is_default DESC, {} {}
        LIMIT $6 OFFSET $7
        "#,
        sort_column, sort_order
    ))
    .bind(&query.block_type)
    .bind(&query.category)
    .bind(&search_pattern)
    .bind(query.is_global)
    .bind(query.is_default)
    .bind(limit)
    .bind(offset)
    .fetch_all(&state.db.pool)
    .await
    .map_err(|e: sqlx::Error| ApiError::new(StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(PaginatedPresetsResponse {
        data: presets,
        meta: PaginationMeta {
            total,
            limit,
            offset,
            has_more: offset + limit < total,
        },
    }))
}

/// Get presets for a specific block type, grouped by category
///
/// Returns presets organized by category for the block inserter UI.
#[utoipa::path(
    get,
    path = "/api/cms/presets/block/{block_type}",
    tag = "CMS Presets",
    params(
        ("block_type" = String, Path, description = "Block type (e.g., button, heading)")
    ),
    responses(
        (status = 200, description = "Presets grouped by category", body = GroupedPresetsResponse),
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Forbidden")
    ),
    security(("bearer_auth" = []))
)]
async fn get_presets_by_block_type(
    State(state): State<AppState>,
    user: User,
    Path(block_type): Path<String>,
) -> ApiResult<GroupedPresetsResponse> {
    require_cms_editor(&user)?;

    // Fetch all presets for this block type
    let presets: Vec<CmsPresetSummary> = sqlx::query_as(
        r#"
        SELECT id, block_type, name, slug, description, thumbnail_url, thumbnail_blurhash,
               category::text as category, tags, is_default, is_locked, is_global,
               usage_count, created_at, updated_at
        FROM cms_presets
        WHERE block_type = $1 AND deleted_at IS NULL AND is_global = true
        ORDER BY category, is_default DESC, usage_count DESC, name
        "#,
    )
    .bind(&block_type)
    .fetch_all(&state.db.pool)
    .await
    .map_err(|e: sqlx::Error| ApiError::new(StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let total_count = presets.len() as i64;

    // Group by category
    let mut category_map: std::collections::HashMap<String, Vec<CmsPresetSummary>> =
        std::collections::HashMap::new();

    for preset in presets {
        category_map
            .entry(preset.category.clone())
            .or_default()
            .push(preset);
    }

    // Convert to response format with specific ordering
    let category_order = [
        "default",
        "brand",
        "trading",
        "marketing",
        "seasonal",
        "custom",
    ];
    let mut categories: Vec<PresetsByCategory> = Vec::new();

    for cat in category_order {
        if let Some(preset_list) = category_map.remove(cat) {
            categories.push(PresetsByCategory {
                category: cat.to_string(),
                presets: preset_list,
            });
        }
    }

    // Add any remaining categories
    for (cat, preset_list) in category_map {
        categories.push(PresetsByCategory {
            category: cat,
            presets: preset_list,
        });
    }

    Ok(Json(GroupedPresetsResponse {
        block_type,
        categories,
        total_count,
    }))
}

/// Get all block types that have presets
#[utoipa::path(
    get,
    path = "/api/cms/presets/block-types",
    tag = "CMS Presets",
    responses(
        (status = 200, description = "Block types with preset counts", body = BlockTypesWithPresetsResponse),
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Forbidden")
    ),
    security(("bearer_auth" = []))
)]
async fn get_block_types_with_presets(
    State(state): State<AppState>,
    user: User,
) -> ApiResult<BlockTypesWithPresetsResponse> {
    require_cms_editor(&user)?;

    let block_types: Vec<BlockTypeWithPresets> = sqlx::query_as(
        r#"
        SELECT
            block_type,
            COUNT(*)::BIGINT AS preset_count,
            BOOL_OR(is_default) AS has_default
        FROM cms_presets
        WHERE deleted_at IS NULL AND is_global = true
        GROUP BY block_type
        ORDER BY block_type
        "#,
    )
    .fetch_all(&state.db.pool)
    .await
    .map_err(|e: sqlx::Error| ApiError::new(StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(BlockTypesWithPresetsResponse { data: block_types }))
}

/// Get a preset by ID
#[utoipa::path(
    get,
    path = "/api/cms/presets/{id}",
    tag = "CMS Presets",
    params(
        ("id" = Uuid, Path, description = "Preset UUID")
    ),
    responses(
        (status = 200, description = "Preset details", body = CmsPreset),
        (status = 404, description = "Preset not found"),
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Forbidden")
    ),
    security(("bearer_auth" = []))
)]
async fn get_preset(
    State(state): State<AppState>,
    user: User,
    Path(id): Path<Uuid>,
) -> ApiResult<CmsPreset> {
    require_cms_editor(&user)?;

    let preset: CmsPreset = sqlx::query_as(
        r#"
        SELECT id, block_type, name, slug, description, preset_data,
               thumbnail_url, thumbnail_blurhash, category,
               tags, is_default, is_locked, is_global, usage_count, version,
               created_at, updated_at, created_by, updated_by, deleted_at
        FROM cms_presets
        WHERE id = $1 AND deleted_at IS NULL
        "#,
    )
    .bind(id)
    .fetch_optional(&state.db.pool)
    .await
    .map_err(|e: sqlx::Error| ApiError::new(StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
    .ok_or_else(|| {
        ApiError::new(StatusCode::NOT_FOUND, "Preset not found").with_code("NOT_FOUND")
    })?;

    Ok(Json(preset))
}

/// Get a preset by block type and slug
#[utoipa::path(
    get,
    path = "/api/cms/presets/block/{block_type}/slug/{slug}",
    tag = "CMS Presets",
    params(
        ("block_type" = String, Path, description = "Block type"),
        ("slug" = String, Path, description = "Preset slug")
    ),
    responses(
        (status = 200, description = "Preset details", body = CmsPreset),
        (status = 404, description = "Preset not found"),
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Forbidden")
    ),
    security(("bearer_auth" = []))
)]
async fn get_preset_by_slug(
    State(state): State<AppState>,
    user: User,
    Path((block_type, slug)): Path<(String, String)>,
) -> ApiResult<CmsPreset> {
    require_cms_editor(&user)?;

    let preset: CmsPreset = sqlx::query_as(
        r#"
        SELECT id, block_type, name, slug, description, preset_data,
               thumbnail_url, thumbnail_blurhash, category,
               tags, is_default, is_locked, is_global, usage_count, version,
               created_at, updated_at, created_by, updated_by, deleted_at
        FROM cms_presets
        WHERE block_type = $1 AND slug = $2 AND deleted_at IS NULL
        "#,
    )
    .bind(&block_type)
    .bind(&slug)
    .fetch_optional(&state.db.pool)
    .await
    .map_err(|e: sqlx::Error| ApiError::new(StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
    .ok_or_else(|| {
        ApiError::new(StatusCode::NOT_FOUND, "Preset not found").with_code("NOT_FOUND")
    })?;

    Ok(Json(preset))
}

/// Create a new preset
#[utoipa::path(
    post,
    path = "/api/cms/presets",
    tag = "CMS Presets",
    request_body = CreatePresetRequest,
    responses(
        (status = 201, description = "Preset created successfully", body = CmsPreset),
        (status = 400, description = "Validation error"),
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Forbidden"),
        (status = 409, description = "Slug already exists")
    ),
    security(("bearer_auth" = []))
)]
async fn create_preset(
    State(state): State<AppState>,
    user: User,
    Json(request): Json<CreatePresetRequest>,
) -> Result<(StatusCode, Json<CmsPreset>), ApiError> {
    require_cms_editor(&user)?;

    // Validate required fields
    if request.name.trim().is_empty() {
        return Err(ApiError::new(StatusCode::BAD_REQUEST, "Name is required")
            .with_code("VALIDATION_ERROR"));
    }

    if request.block_type.trim().is_empty() {
        return Err(
            ApiError::new(StatusCode::BAD_REQUEST, "Block type is required")
                .with_code("VALIDATION_ERROR"),
        );
    }

    // Generate or validate slug
    let base_slug = request
        .slug
        .clone()
        .unwrap_or_else(|| generate_slug(&request.name));
    let slug = ensure_unique_slug(&state.db.pool, &request.block_type, &base_slug, None).await?;

    // Get CMS user ID from platform user
    let cms_user_id: Option<Uuid> =
        sqlx::query_scalar("SELECT id FROM cms_users WHERE user_id = $1")
            .bind(user.id)
            .fetch_optional(&state.db.pool)
            .await
            .map_err(|e: sqlx::Error| {
                ApiError::new(StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
            })?;

    // If setting as default, clear other defaults for this block type
    if request.is_default {
        sqlx::query(
            "UPDATE cms_presets SET is_default = false WHERE block_type = $1 AND is_default = true",
        )
        .bind(&request.block_type)
        .execute(&state.db.pool)
        .await
        .map_err(|e: sqlx::Error| {
            ApiError::new(StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
        })?;
    }

    let new_id = Uuid::new_v4();
    let category = request.category.unwrap_or(CmsPresetCategory::Custom);

    let preset: CmsPreset = sqlx::query_as(
        r#"
        INSERT INTO cms_presets (
            id, block_type, name, slug, description, preset_data,
            thumbnail_url, thumbnail_blurhash, category, tags,
            is_default, is_global, is_locked, usage_count, version,
            created_at, updated_at, created_by
        )
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, false, 0, 1, NOW(), NOW(), $13)
        RETURNING id, block_type, name, slug, description, preset_data,
                  thumbnail_url, thumbnail_blurhash, category,
                  tags, is_default, is_locked, is_global, usage_count, version,
                  created_at, updated_at, created_by, updated_by, deleted_at
        "#,
    )
    .bind(new_id)
    .bind(request.block_type.trim())
    .bind(request.name.trim())
    .bind(&slug)
    .bind(&request.description)
    .bind(&request.preset_data)
    .bind(&request.thumbnail_url)
    .bind(&request.thumbnail_blurhash)
    .bind(category)
    .bind(&request.tags)
    .bind(request.is_default)
    .bind(request.is_global)
    .bind(cms_user_id)
    .fetch_one(&state.db.pool)
    .await
    .map_err(|e: sqlx::Error| ApiError::new(StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    tracing::info!(
        "Preset created: {} ({}) for block type {}",
        preset.name,
        preset.id,
        preset.block_type
    );

    Ok((StatusCode::CREATED, Json(preset)))
}

/// Update a preset
#[utoipa::path(
    put,
    path = "/api/cms/presets/{id}",
    tag = "CMS Presets",
    params(
        ("id" = Uuid, Path, description = "Preset UUID")
    ),
    request_body = UpdatePresetRequest,
    responses(
        (status = 200, description = "Preset updated successfully", body = CmsPreset),
        (status = 400, description = "Validation error"),
        (status = 404, description = "Preset not found"),
        (status = 409, description = "Slug already exists"),
        (status = 423, description = "Preset is locked")
    ),
    security(("bearer_auth" = []))
)]
async fn update_preset(
    State(state): State<AppState>,
    user: User,
    Path(id): Path<Uuid>,
    Json(request): Json<UpdatePresetRequest>,
) -> ApiResult<CmsPreset> {
    require_cms_editor(&user)?;

    // Check if preset exists and is not locked
    #[derive(Debug, sqlx::FromRow)]
    struct ExistingPreset {
        is_locked: bool,
        block_type: String,
    }

    let existing: ExistingPreset = sqlx::query_as(
        "SELECT is_locked, block_type FROM cms_presets WHERE id = $1 AND deleted_at IS NULL",
    )
    .bind(id)
    .fetch_optional(&state.db.pool)
    .await
    .map_err(|e: sqlx::Error| ApiError::new(StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
    .ok_or_else(|| {
        ApiError::new(StatusCode::NOT_FOUND, "Preset not found").with_code("NOT_FOUND")
    })?;

    // Only admins can update locked presets
    if existing.is_locked && request.is_locked != Some(false) {
        require_cms_admin(&user)?;
    }

    // Handle slug update
    let final_slug = if let Some(ref new_slug) = request.slug {
        Some(ensure_unique_slug(&state.db.pool, &existing.block_type, new_slug, Some(id)).await?)
    } else {
        None
    };

    // Get CMS user ID
    let cms_user_id: Option<Uuid> =
        sqlx::query_scalar("SELECT id FROM cms_users WHERE user_id = $1")
            .bind(user.id)
            .fetch_optional(&state.db.pool)
            .await
            .map_err(|e: sqlx::Error| {
                ApiError::new(StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
            })?;

    // If setting as default, clear other defaults for this block type
    if request.is_default == Some(true) {
        sqlx::query(
            "UPDATE cms_presets SET is_default = false WHERE block_type = $1 AND is_default = true AND id != $2"
        )
        .bind(&existing.block_type)
        .bind(id)
        .execute(&state.db.pool)
        .await
        .map_err(|e: sqlx::Error| {
            ApiError::new(StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
        })?;
    }

    let preset: CmsPreset = sqlx::query_as(
        r#"
        UPDATE cms_presets
        SET name = COALESCE($2, name),
            slug = COALESCE($3, slug),
            description = COALESCE($4, description),
            preset_data = COALESCE($5, preset_data),
            thumbnail_url = COALESCE($6, thumbnail_url),
            thumbnail_blurhash = COALESCE($7, thumbnail_blurhash),
            category = COALESCE($8, category),
            tags = COALESCE($9, tags),
            is_default = COALESCE($10, is_default),
            is_global = COALESCE($11, is_global),
            is_locked = COALESCE($12, is_locked),
            version = version + 1,
            updated_at = NOW(),
            updated_by = $13
        WHERE id = $1 AND deleted_at IS NULL
        RETURNING id, block_type, name, slug, description, preset_data,
                  thumbnail_url, thumbnail_blurhash, category,
                  tags, is_default, is_locked, is_global, usage_count, version,
                  created_at, updated_at, created_by, updated_by, deleted_at
        "#,
    )
    .bind(id)
    .bind(&request.name)
    .bind(&final_slug)
    .bind(&request.description)
    .bind(&request.preset_data)
    .bind(&request.thumbnail_url)
    .bind(&request.thumbnail_blurhash)
    .bind(request.category)
    .bind(&request.tags)
    .bind(request.is_default)
    .bind(request.is_global)
    .bind(request.is_locked)
    .bind(cms_user_id)
    .fetch_one(&state.db.pool)
    .await
    .map_err(|e: sqlx::Error| ApiError::new(StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    tracing::info!("Preset updated: {} ({})", preset.name, preset.id);

    Ok(Json(preset))
}

/// Delete a preset (soft delete)
#[utoipa::path(
    delete,
    path = "/api/cms/presets/{id}",
    tag = "CMS Presets",
    params(
        ("id" = Uuid, Path, description = "Preset UUID")
    ),
    responses(
        (status = 200, description = "Preset deleted successfully"),
        (status = 404, description = "Preset not found"),
        (status = 423, description = "Preset is locked")
    ),
    security(("bearer_auth" = []))
)]
async fn delete_preset(
    State(state): State<AppState>,
    user: User,
    Path(id): Path<Uuid>,
) -> ApiResultEmpty {
    require_cms_admin(&user)?;

    // Check if preset exists and is not locked
    #[derive(Debug, sqlx::FromRow)]
    struct DeletePreset {
        is_locked: bool,
        name: String,
    }

    let existing: DeletePreset = sqlx::query_as(
        "SELECT is_locked, name FROM cms_presets WHERE id = $1 AND deleted_at IS NULL",
    )
    .bind(id)
    .fetch_optional(&state.db.pool)
    .await
    .map_err(|e: sqlx::Error| ApiError::new(StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
    .ok_or_else(|| {
        ApiError::new(StatusCode::NOT_FOUND, "Preset not found").with_code("NOT_FOUND")
    })?;

    if existing.is_locked {
        return Err(ApiError::new(
            StatusCode::LOCKED,
            "Cannot delete a locked preset. Unlock it first.",
        )
        .with_code("PRESET_LOCKED"));
    }

    // Soft delete
    sqlx::query("UPDATE cms_presets SET deleted_at = NOW() WHERE id = $1")
        .bind(id)
        .execute(&state.db.pool)
        .await
        .map_err(|e: sqlx::Error| {
            ApiError::new(StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
        })?;

    tracing::info!("Preset deleted: {} ({})", existing.name, id);

    Ok(Json(json!({
        "message": "Preset deleted successfully",
        "id": id
    })))
}

/// Duplicate a preset
#[utoipa::path(
    post,
    path = "/api/cms/presets/{id}/duplicate",
    tag = "CMS Presets",
    params(
        ("id" = Uuid, Path, description = "Preset UUID to duplicate")
    ),
    responses(
        (status = 201, description = "Preset duplicated successfully", body = CmsPreset),
        (status = 404, description = "Preset not found")
    ),
    security(("bearer_auth" = []))
)]
async fn duplicate_preset(
    State(state): State<AppState>,
    user: User,
    Path(id): Path<Uuid>,
) -> Result<(StatusCode, Json<CmsPreset>), ApiError> {
    require_cms_editor(&user)?;

    // Fetch the original preset
    let original: CmsPreset = sqlx::query_as(
        r#"
        SELECT id, block_type, name, slug, description, preset_data,
               thumbnail_url, thumbnail_blurhash, category,
               tags, is_default, is_locked, is_global, usage_count, version,
               created_at, updated_at, created_by, updated_by, deleted_at
        FROM cms_presets
        WHERE id = $1 AND deleted_at IS NULL
        "#,
    )
    .bind(id)
    .fetch_optional(&state.db.pool)
    .await
    .map_err(|e: sqlx::Error| ApiError::new(StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
    .ok_or_else(|| {
        ApiError::new(StatusCode::NOT_FOUND, "Preset not found").with_code("NOT_FOUND")
    })?;

    // Generate new name and slug
    let new_name = format!("{} (Copy)", original.name);
    let base_slug = generate_slug(&new_name);
    let new_slug =
        ensure_unique_slug(&state.db.pool, &original.block_type, &base_slug, None).await?;

    // Get CMS user ID
    let cms_user_id: Option<Uuid> =
        sqlx::query_scalar("SELECT id FROM cms_users WHERE user_id = $1")
            .bind(user.id)
            .fetch_optional(&state.db.pool)
            .await
            .map_err(|e: sqlx::Error| {
                ApiError::new(StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
            })?;

    let dup_id = Uuid::new_v4();
    let duplicated: CmsPreset = sqlx::query_as(
        r#"
        INSERT INTO cms_presets (
            id, block_type, name, slug, description, preset_data,
            thumbnail_url, thumbnail_blurhash, category, tags,
            is_default, is_global, is_locked, usage_count, version,
            created_at, updated_at, created_by
        )
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, false, $11, false, 0, 1, NOW(), NOW(), $12)
        RETURNING id, block_type, name, slug, description, preset_data,
                  thumbnail_url, thumbnail_blurhash, category,
                  tags, is_default, is_locked, is_global, usage_count, version,
                  created_at, updated_at, created_by, updated_by, deleted_at
        "#,
    )
    .bind(dup_id)
    .bind(&original.block_type)
    .bind(&new_name)
    .bind(&new_slug)
    .bind(&original.description)
    .bind(&original.preset_data)
    .bind(&original.thumbnail_url)
    .bind(&original.thumbnail_blurhash)
    .bind(original.category)
    .bind(&original.tags)
    .bind(original.is_global)
    .bind(cms_user_id)
    .fetch_one(&state.db.pool)
    .await
    .map_err(|e: sqlx::Error| ApiError::new(StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    tracing::info!(
        "Preset duplicated: {} -> {} ({})",
        original.name,
        duplicated.name,
        duplicated.id
    );

    Ok((StatusCode::CREATED, Json(duplicated)))
}

/// Apply a preset (increment usage count)
#[utoipa::path(
    post,
    path = "/api/cms/presets/{id}/apply",
    tag = "CMS Presets",
    params(
        ("id" = Uuid, Path, description = "Preset UUID")
    ),
    responses(
        (status = 200, description = "Preset applied successfully", body = CmsPreset),
        (status = 404, description = "Preset not found")
    ),
    security(("bearer_auth" = []))
)]
async fn apply_preset(
    State(state): State<AppState>,
    user: User,
    Path(id): Path<Uuid>,
) -> ApiResult<CmsPreset> {
    require_cms_editor(&user)?;

    // Increment usage count and return updated preset
    let preset: CmsPreset = sqlx::query_as(
        r#"
        UPDATE cms_presets
        SET usage_count = usage_count + 1
        WHERE id = $1 AND deleted_at IS NULL
        RETURNING id, block_type, name, slug, description, preset_data,
                  thumbnail_url, thumbnail_blurhash, category,
                  tags, is_default, is_locked, is_global, usage_count, version,
                  created_at, updated_at, created_by, updated_by, deleted_at
        "#,
    )
    .bind(id)
    .fetch_optional(&state.db.pool)
    .await
    .map_err(|e: sqlx::Error| ApiError::new(StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
    .ok_or_else(|| {
        ApiError::new(StatusCode::NOT_FOUND, "Preset not found").with_code("NOT_FOUND")
    })?;

    tracing::debug!(
        "Preset applied: {} ({}) - usage count: {}",
        preset.name,
        preset.id,
        preset.usage_count
    );

    Ok(Json(preset))
}

/// Save current block as a new preset
#[derive(Debug, Deserialize, ToSchema)]
pub struct SaveBlockAsPresetRequest {
    /// Block type
    pub block_type: String,
    /// Preset name
    pub name: String,
    /// Optional description
    pub description: Option<String>,
    /// Block content
    pub content: JsonValue,
    /// Block settings
    pub settings: JsonValue,
    /// Category
    pub category: Option<CmsPresetCategory>,
    /// Tags
    pub tags: Option<Vec<String>>,
    /// Thumbnail URL (optional)
    pub thumbnail_url: Option<String>,
}

#[utoipa::path(
    post,
    path = "/api/cms/presets/save-from-block",
    tag = "CMS Presets",
    request_body = SaveBlockAsPresetRequest,
    responses(
        (status = 201, description = "Preset created from block", body = CmsPreset),
        (status = 400, description = "Validation error"),
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Forbidden")
    ),
    security(("bearer_auth" = []))
)]
async fn save_block_as_preset(
    State(state): State<AppState>,
    user: User,
    Json(request): Json<SaveBlockAsPresetRequest>,
) -> Result<(StatusCode, Json<CmsPreset>), ApiError> {
    require_cms_editor(&user)?;

    // Validate required fields
    if request.name.trim().is_empty() {
        return Err(ApiError::new(StatusCode::BAD_REQUEST, "Name is required")
            .with_code("VALIDATION_ERROR"));
    }

    if request.block_type.trim().is_empty() {
        return Err(
            ApiError::new(StatusCode::BAD_REQUEST, "Block type is required")
                .with_code("VALIDATION_ERROR"),
        );
    }

    // Combine content and settings into preset_data
    let preset_data = json!({
        "content": request.content,
        "settings": request.settings
    });

    // Generate slug
    let base_slug = generate_slug(&request.name);
    let slug = ensure_unique_slug(&state.db.pool, &request.block_type, &base_slug, None).await?;

    // Get CMS user ID
    let cms_user_id: Option<Uuid> =
        sqlx::query_scalar("SELECT id FROM cms_users WHERE user_id = $1")
            .bind(user.id)
            .fetch_optional(&state.db.pool)
            .await
            .map_err(|e: sqlx::Error| {
                ApiError::new(StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
            })?;

    let new_id = Uuid::new_v4();
    let category = request.category.unwrap_or(CmsPresetCategory::Custom);

    let preset: CmsPreset = sqlx::query_as(
        r#"
        INSERT INTO cms_presets (
            id, block_type, name, slug, description, preset_data,
            thumbnail_url, category, tags,
            is_default, is_global, is_locked, usage_count, version,
            created_at, updated_at, created_by
        )
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, false, true, false, 0, 1, NOW(), NOW(), $10)
        RETURNING id, block_type, name, slug, description, preset_data,
                  thumbnail_url, thumbnail_blurhash, category,
                  tags, is_default, is_locked, is_global, usage_count, version,
                  created_at, updated_at, created_by, updated_by, deleted_at
        "#,
    )
    .bind(new_id)
    .bind(request.block_type.trim())
    .bind(request.name.trim())
    .bind(&slug)
    .bind(&request.description)
    .bind(&preset_data)
    .bind(&request.thumbnail_url)
    .bind(category)
    .bind(&request.tags)
    .bind(cms_user_id)
    .fetch_one(&state.db.pool)
    .await
    .map_err(|e: sqlx::Error| ApiError::new(StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    tracing::info!(
        "Preset saved from block: {} ({}) for block type {}",
        preset.name,
        preset.id,
        preset.block_type
    );

    Ok((StatusCode::CREATED, Json(preset)))
}

// ============================================================================
// ROUTER
// ============================================================================

/// Admin router for presets (requires authentication)
pub fn admin_router() -> Router<AppState> {
    Router::new()
        // Listing and retrieval
        .route("/", get(list_presets).post(create_preset))
        .route("/block-types", get(get_block_types_with_presets))
        .route("/block/{block_type}", get(get_presets_by_block_type))
        .route("/block/{block_type}/slug/{slug}", get(get_preset_by_slug))
        // CRUD by ID
        .route(
            "/{id}",
            get(get_preset).put(update_preset).delete(delete_preset),
        )
        // Actions
        .route("/{id}/duplicate", post(duplicate_preset))
        .route("/{id}/apply", post(apply_preset))
        // Save from block
        .route("/save-from-block", post(save_block_as_preset))
}

/// Public router for read-only preset access
pub fn public_router() -> Router<AppState> {
    Router::new().route("/block/{block_type}", get(get_presets_by_block_type_public))
}

/// Public endpoint to get presets by block type (global only, no auth)
async fn get_presets_by_block_type_public(
    State(state): State<AppState>,
    Path(block_type): Path<String>,
) -> ApiResult<GroupedPresetsResponse> {
    // Fetch all global presets for this block type
    let presets: Vec<CmsPresetSummary> = sqlx::query_as(
        r#"
        SELECT id, block_type, name, slug, description, thumbnail_url, thumbnail_blurhash,
               category::text as category, tags, is_default, is_locked, is_global,
               usage_count, created_at, updated_at
        FROM cms_presets
        WHERE block_type = $1 AND deleted_at IS NULL AND is_global = true
        ORDER BY category, is_default DESC, usage_count DESC, name
        "#,
    )
    .bind(&block_type)
    .fetch_all(&state.db.pool)
    .await
    .map_err(|e: sqlx::Error| ApiError::new(StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let total_count = presets.len() as i64;

    // Group by category
    let mut category_map: std::collections::HashMap<String, Vec<CmsPresetSummary>> =
        std::collections::HashMap::new();

    for preset in presets {
        category_map
            .entry(preset.category.clone())
            .or_default()
            .push(preset);
    }

    // Convert to response format
    let category_order = [
        "default",
        "brand",
        "trading",
        "marketing",
        "seasonal",
        "custom",
    ];
    let mut categories: Vec<PresetsByCategory> = Vec::new();

    for cat in category_order {
        if let Some(preset_list) = category_map.remove(cat) {
            categories.push(PresetsByCategory {
                category: cat.to_string(),
                presets: preset_list,
            });
        }
    }

    for (cat, preset_list) in category_map {
        categories.push(PresetsByCategory {
            category: cat,
            presets: preset_list,
        });
    }

    Ok(Json(GroupedPresetsResponse {
        block_type,
        categories,
        total_count,
    }))
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_slug() {
        assert_eq!(generate_slug("Primary CTA"), "primary-cta");
        assert_eq!(generate_slug("Hero Title v2"), "hero-title-v2");
        assert_eq!(generate_slug("  Multiple   Spaces  "), "multiple-spaces");
    }

    #[test]
    fn test_default_true() {
        assert!(default_true());
    }

    #[test]
    fn test_preset_category_serialization() {
        let category = CmsPresetCategory::Default;
        let json = serde_json::to_string(&category).unwrap();
        assert_eq!(json, "\"default\"");

        let category = CmsPresetCategory::Trading;
        let json = serde_json::to_string(&category).unwrap();
        assert_eq!(json, "\"trading\"");
    }
}
