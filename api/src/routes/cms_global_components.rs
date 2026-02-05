//! CMS Global Components Library API Routes - Storyblok-Style
//!
//! Comprehensive API for managing global components in the CMS.
//! Enables managing headers, footers, CTAs, and other repeated elements from one place.
//!
//! Features:
//! - CRUD operations for global components
//! - Categories: headers, footers, CTAs, forms, navigation
//! - Usage tracking (which pages use each component)
//! - Sync/detach functionality for component instances
//! - Version history for all changes
//! - Search and filter capabilities
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

/// Global component category enum
#[derive(
    Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize, sqlx::Type, ToSchema,
)]
#[sqlx(type_name = "cms_global_component_category", rename_all = "lowercase")]
#[serde(rename_all = "lowercase")]
pub enum GlobalComponentCategory {
    #[default]
    Header,
    Footer,
    Cta,
    Form,
    Navigation,
    Sidebar,
    Banner,
    Modal,
    Card,
    Section,
}

impl std::fmt::Display for GlobalComponentCategory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GlobalComponentCategory::Header => write!(f, "header"),
            GlobalComponentCategory::Footer => write!(f, "footer"),
            GlobalComponentCategory::Cta => write!(f, "cta"),
            GlobalComponentCategory::Form => write!(f, "form"),
            GlobalComponentCategory::Navigation => write!(f, "navigation"),
            GlobalComponentCategory::Sidebar => write!(f, "sidebar"),
            GlobalComponentCategory::Banner => write!(f, "banner"),
            GlobalComponentCategory::Modal => write!(f, "modal"),
            GlobalComponentCategory::Card => write!(f, "card"),
            GlobalComponentCategory::Section => write!(f, "section"),
        }
    }
}

/// Global component stored in the component library
#[derive(Debug, Clone, Serialize, Deserialize, FromRow, ToSchema)]
pub struct GlobalComponent {
    /// Unique identifier
    pub id: Uuid,
    /// Display name of the component
    pub name: String,
    /// URL-friendly unique identifier
    pub slug: String,
    /// Optional description for the component library
    pub description: Option<String>,
    /// Component content and configuration (JSON array of blocks)
    pub component_data: JsonValue,
    /// Category for organization
    pub category: GlobalComponentCategory,
    /// Tags for filtering and search
    pub tags: Option<Vec<String>>,
    /// Preview thumbnail URL
    pub thumbnail_url: Option<String>,
    /// Number of times this component is used across content
    pub usage_count: i32,
    /// Whether the component is available globally to all users
    pub is_global: bool,
    /// Whether the component is locked from editing
    pub is_locked: bool,
    /// Current version number
    pub version: i32,
    /// Soft delete timestamp
    pub deleted_at: Option<DateTime<Utc>>,
    /// Creation timestamp
    pub created_at: DateTime<Utc>,
    /// Last update timestamp
    pub updated_at: DateTime<Utc>,
    /// UUID of the CMS user who created the component
    pub created_by: Option<Uuid>,
    /// UUID of the CMS user who last updated the component
    pub updated_by: Option<Uuid>,
}

/// Lightweight component summary for listings
#[derive(Debug, Clone, Serialize, Deserialize, FromRow, ToSchema)]
pub struct GlobalComponentSummary {
    pub id: Uuid,
    pub name: String,
    pub slug: String,
    pub description: Option<String>,
    pub category: String,
    pub tags: Option<Vec<String>>,
    pub thumbnail_url: Option<String>,
    pub usage_count: i32,
    pub is_global: bool,
    pub is_locked: bool,
    pub version: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Version history record for a global component
#[derive(Debug, Clone, Serialize, Deserialize, FromRow, ToSchema)]
pub struct GlobalComponentVersion {
    /// Unique identifier for this version
    pub id: Uuid,
    /// Reference to the global component
    pub component_id: Uuid,
    /// Version number
    pub version: i32,
    /// Component data at this version
    pub component_data: JsonValue,
    /// Change description/commit message
    pub change_message: Option<String>,
    /// Who made this change
    pub created_by: Option<Uuid>,
    /// Creator's display name
    pub created_by_name: Option<String>,
    /// When this version was created
    pub created_at: DateTime<Utc>,
}

/// Tracks where a global component is used
#[derive(Debug, Clone, Serialize, Deserialize, FromRow, ToSchema)]
pub struct GlobalComponentUsage {
    /// Unique identifier for this usage record
    pub id: Uuid,
    /// Reference to the global component
    pub component_id: Uuid,
    /// Content item where the component is used
    pub content_id: Uuid,
    /// Instance ID of the component within the content
    pub instance_id: String,
    /// Whether this instance syncs with the source component
    pub is_synced: bool,
    /// Version at which this instance was last synced
    pub synced_version: i32,
    /// When the component was detached from sync
    pub detached_at: Option<DateTime<Utc>>,
    /// Who detached the component
    pub detached_by: Option<Uuid>,
    /// When this usage was created
    pub created_at: DateTime<Utc>,
}

/// Usage record with content metadata for display
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct GlobalComponentUsageWithContent {
    pub id: Uuid,
    pub component_id: Uuid,
    pub content_id: Uuid,
    pub content_title: String,
    pub content_type: String,
    pub content_slug: String,
    pub instance_id: String,
    pub is_synced: bool,
    pub synced_version: i32,
    pub needs_update: bool,
    pub detached_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}

// ============================================================================
// REQUEST/RESPONSE MODELS
// ============================================================================

/// Request to create a new global component
#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateGlobalComponentRequest {
    /// Display name (required)
    pub name: String,
    /// Custom slug (optional, auto-generated if not provided)
    pub slug: Option<String>,
    /// Component description
    pub description: Option<String>,
    /// Component content and configuration (array of blocks)
    pub component_data: JsonValue,
    /// Category for organization
    pub category: GlobalComponentCategory,
    /// Tags for filtering
    pub tags: Option<Vec<String>>,
    /// Preview thumbnail URL
    pub thumbnail_url: Option<String>,
    /// Make globally available
    #[serde(default)]
    pub is_global: bool,
}

/// Request to update a global component
#[derive(Debug, Deserialize, ToSchema)]
pub struct UpdateGlobalComponentRequest {
    pub name: Option<String>,
    pub slug: Option<String>,
    pub description: Option<String>,
    pub component_data: Option<JsonValue>,
    pub category: Option<GlobalComponentCategory>,
    pub tags: Option<Vec<String>>,
    pub thumbnail_url: Option<String>,
    pub is_global: Option<bool>,
    pub is_locked: Option<bool>,
    /// Optional change message for version history
    pub change_message: Option<String>,
}

/// Request to track component usage
#[derive(Debug, Deserialize, ToSchema)]
pub struct TrackComponentUsageRequest {
    /// The global component being used
    pub component_id: Uuid,
    /// Content item where the component is placed
    pub content_id: Uuid,
    /// Unique instance ID within the content
    pub instance_id: String,
    /// Whether to sync updates from source
    #[serde(default = "default_true")]
    pub is_synced: bool,
}

fn default_true() -> bool {
    true
}

/// Request to sync a component instance to latest version
#[derive(Debug, Deserialize, ToSchema)]
pub struct SyncComponentRequest {
    /// Whether to force sync even if there are local changes
    #[serde(default)]
    pub force: bool,
}

/// Query parameters for listing components
#[derive(Debug, Deserialize, IntoParams, ToSchema)]
pub struct ListGlobalComponentsQuery {
    /// Filter by category
    pub category: Option<String>,
    /// Search in name, description, tags
    pub search: Option<String>,
    /// Filter by tags (comma-separated)
    pub tags: Option<String>,
    /// Include only global components
    pub is_global: Option<bool>,
    /// Sort field (name, created_at, updated_at, usage_count)
    pub sort_by: Option<String>,
    /// Sort direction (asc, desc)
    pub sort_order: Option<String>,
    /// Pagination limit (default 20, max 100)
    pub limit: Option<i64>,
    /// Pagination offset
    pub offset: Option<i64>,
    /// Include deleted components
    pub include_deleted: Option<bool>,
}

/// Paginated response wrapper
#[derive(Debug, Serialize, ToSchema)]
pub struct PaginatedComponentsResponse {
    pub data: Vec<GlobalComponentSummary>,
    pub meta: PaginationMeta,
    pub categories: Vec<CategoryCount>,
}

/// Pagination metadata
#[derive(Debug, Serialize, ToSchema)]
pub struct PaginationMeta {
    pub total: i64,
    pub limit: i64,
    pub offset: i64,
    pub has_more: bool,
}

/// Category count for sidebar filters
#[derive(Debug, Serialize, Deserialize, FromRow, ToSchema)]
pub struct CategoryCount {
    pub category: String,
    pub count: i64,
}

/// Response for component usage listing
#[derive(Debug, Serialize, ToSchema)]
pub struct ComponentUsageResponse {
    pub usages: Vec<GlobalComponentUsageWithContent>,
    pub total_count: i64,
    pub synced_count: i64,
    pub outdated_count: i64,
}

/// Response for version history
#[derive(Debug, Serialize, ToSchema)]
pub struct VersionHistoryResponse {
    pub versions: Vec<GlobalComponentVersion>,
    pub total_count: i64,
    pub current_version: i32,
}

/// Response for component with full details
#[derive(Debug, Serialize, ToSchema)]
pub struct GlobalComponentWithMeta {
    #[serde(flatten)]
    pub component: GlobalComponent,
    pub created_by_name: Option<String>,
    pub updated_by_name: Option<String>,
    pub recent_versions: Vec<GlobalComponentVersion>,
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
                        SELECT 1 FROM cms_global_components
                        WHERE slug = $1 AND id != $2 AND deleted_at IS NULL
                    )
                    "#,
                )
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
                        SELECT 1 FROM cms_global_components
                        WHERE slug = $1 AND deleted_at IS NULL
                    )
                    "#,
                )
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

/// Get CMS user ID from platform user
async fn get_cms_user_id(pool: &sqlx::PgPool, user_id: i64) -> Option<Uuid> {
    sqlx::query_scalar("SELECT id FROM cms_users WHERE user_id = $1")
        .bind(user_id)
        .fetch_optional(pool)
        .await
        .ok()?
}

/// Create a version history record
async fn create_version_record(
    pool: &sqlx::PgPool,
    component_id: Uuid,
    version: i32,
    component_data: &JsonValue,
    change_message: Option<&str>,
    created_by: Option<Uuid>,
) -> Result<(), ApiError> {
    sqlx::query(
        r#"
        INSERT INTO cms_global_component_versions (
            id, component_id, version, component_data, change_message, created_by, created_at
        )
        VALUES ($1, $2, $3, $4, $5, $6, NOW())
        "#,
    )
    .bind(Uuid::new_v4())
    .bind(component_id)
    .bind(version)
    .bind(component_data)
    .bind(change_message)
    .bind(created_by)
    .execute(pool)
    .await
    .map_err(|e: sqlx::Error| ApiError::new(StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(())
}

// ============================================================================
// ROUTE HANDLERS
// ============================================================================

/// List global components with pagination and filtering
#[utoipa::path(
    get,
    path = "/api/cms/global-components",
    tag = "CMS Global Components",
    params(ListGlobalComponentsQuery),
    responses(
        (status = 200, description = "List of global components", body = PaginatedComponentsResponse),
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Forbidden - editor access required"),
        (status = 500, description = "Internal server error")
    ),
    security(("bearer_auth" = []))
)]
async fn list_global_components(
    State(state): State<AppState>,
    user: User,
    Query(query): Query<ListGlobalComponentsQuery>,
) -> ApiResult<PaginatedComponentsResponse> {
    require_cms_editor(&user)?;

    let limit = query.limit.unwrap_or(20).min(100);
    let offset = query.offset.unwrap_or(0);
    let include_deleted = query.include_deleted.unwrap_or(false);

    // Sort configuration - validated against whitelist
    let sort_column = match query.sort_by.as_deref() {
        Some("name") => "name",
        Some("usage_count") => "usage_count",
        Some("updated_at") => "updated_at",
        Some("category") => "category",
        _ => "created_at",
    };
    let sort_order = match query.sort_order.as_deref() {
        Some("asc") => "ASC",
        _ => "DESC",
    };

    // Prepare search pattern if provided
    let search_pattern = query.search.as_ref().map(|s| format!("%{}%", s));

    // Count total
    let count_result: (i64,) = sqlx::query_as(
        r#"
        SELECT COUNT(*)
        FROM cms_global_components
        WHERE ($1::boolean OR deleted_at IS NULL)
          AND ($2::text IS NULL OR category::text = $2)
          AND ($3::text IS NULL OR name ILIKE $3 OR description ILIKE $3)
          AND ($4::boolean IS NULL OR is_global = $4)
        "#,
    )
    .bind(include_deleted)
    .bind(&query.category)
    .bind(&search_pattern)
    .bind(query.is_global)
    .fetch_one(&state.db.pool)
    .await
    .map_err(|e: sqlx::Error| ApiError::new(StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let total = count_result.0;

    // Fetch components with safe sort columns
    let components: Vec<GlobalComponentSummary> = sqlx::query_as(&format!(
        r#"
        SELECT id, name, slug, description, category::text, tags, thumbnail_url,
               usage_count, is_global, is_locked, version, created_at, updated_at
        FROM cms_global_components
        WHERE ($1::boolean OR deleted_at IS NULL)
          AND ($2::text IS NULL OR category::text = $2)
          AND ($3::text IS NULL OR name ILIKE $3 OR description ILIKE $3)
          AND ($4::boolean IS NULL OR is_global = $4)
        ORDER BY {} {}
        LIMIT $5 OFFSET $6
        "#,
        sort_column, sort_order
    ))
    .bind(include_deleted)
    .bind(&query.category)
    .bind(&search_pattern)
    .bind(query.is_global)
    .bind(limit)
    .bind(offset)
    .fetch_all(&state.db.pool)
    .await
    .map_err(|e: sqlx::Error| ApiError::new(StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    // Get category counts for sidebar filters
    let categories: Vec<CategoryCount> = sqlx::query_as(
        r#"
        SELECT category::text as category, COUNT(*) as count
        FROM cms_global_components
        WHERE deleted_at IS NULL
        GROUP BY category
        ORDER BY count DESC
        "#,
    )
    .fetch_all(&state.db.pool)
    .await
    .map_err(|e: sqlx::Error| ApiError::new(StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(PaginatedComponentsResponse {
        data: components,
        meta: PaginationMeta {
            total,
            limit,
            offset,
            has_more: offset + limit < total,
        },
        categories,
    }))
}

/// Get a global component by ID with metadata
#[utoipa::path(
    get,
    path = "/api/cms/global-components/{id}",
    tag = "CMS Global Components",
    params(("id" = Uuid, Path, description = "Component UUID")),
    responses(
        (status = 200, description = "Global component details", body = GlobalComponentWithMeta),
        (status = 404, description = "Component not found"),
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Forbidden")
    ),
    security(("bearer_auth" = []))
)]
async fn get_global_component(
    State(state): State<AppState>,
    user: User,
    Path(id): Path<Uuid>,
) -> ApiResult<GlobalComponentWithMeta> {
    require_cms_editor(&user)?;

    let component: GlobalComponent = sqlx::query_as(
        r#"
        SELECT id, name, slug, description, component_data, category,
               tags, thumbnail_url, usage_count, is_global, is_locked, version,
               deleted_at, created_at, updated_at, created_by, updated_by
        FROM cms_global_components
        WHERE id = $1 AND deleted_at IS NULL
        "#,
    )
    .bind(id)
    .fetch_optional(&state.db.pool)
    .await
    .map_err(|e: sqlx::Error| ApiError::new(StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
    .ok_or_else(|| {
        ApiError::new(StatusCode::NOT_FOUND, "Global component not found").with_code("NOT_FOUND")
    })?;

    // Get creator name
    let created_by_name: Option<String> = if let Some(creator_id) = component.created_by {
        sqlx::query_scalar("SELECT name FROM cms_users WHERE id = $1")
            .bind(creator_id)
            .fetch_optional(&state.db.pool)
            .await
            .map_err(|e: sqlx::Error| ApiError::new(StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
            .flatten()
    } else {
        None
    };

    // Get updater name
    let updated_by_name: Option<String> = if let Some(updater_id) = component.updated_by {
        sqlx::query_scalar("SELECT name FROM cms_users WHERE id = $1")
            .bind(updater_id)
            .fetch_optional(&state.db.pool)
            .await
            .map_err(|e: sqlx::Error| ApiError::new(StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
            .flatten()
    } else {
        None
    };

    // Get recent versions (last 5)
    let recent_versions: Vec<GlobalComponentVersion> = sqlx::query_as(
        r#"
        SELECT v.id, v.component_id, v.version, v.component_data,
               v.change_message, v.created_by, u.name as created_by_name, v.created_at
        FROM cms_global_component_versions v
        LEFT JOIN cms_users u ON u.id = v.created_by
        WHERE v.component_id = $1
        ORDER BY v.version DESC
        LIMIT 5
        "#,
    )
    .bind(id)
    .fetch_all(&state.db.pool)
    .await
    .map_err(|e: sqlx::Error| ApiError::new(StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(GlobalComponentWithMeta {
        component,
        created_by_name,
        updated_by_name,
        recent_versions,
    }))
}

/// Get a global component by slug
#[utoipa::path(
    get,
    path = "/api/cms/global-components/slug/{slug}",
    tag = "CMS Global Components",
    params(("slug" = String, Path, description = "Component slug")),
    responses(
        (status = 200, description = "Global component details", body = GlobalComponent),
        (status = 404, description = "Component not found"),
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Forbidden")
    ),
    security(("bearer_auth" = []))
)]
async fn get_global_component_by_slug(
    State(state): State<AppState>,
    user: User,
    Path(slug): Path<String>,
) -> ApiResult<GlobalComponent> {
    require_cms_editor(&user)?;

    let component: GlobalComponent = sqlx::query_as(
        r#"
        SELECT id, name, slug, description, component_data, category,
               tags, thumbnail_url, usage_count, is_global, is_locked, version,
               deleted_at, created_at, updated_at, created_by, updated_by
        FROM cms_global_components
        WHERE slug = $1 AND deleted_at IS NULL
        "#,
    )
    .bind(&slug)
    .fetch_optional(&state.db.pool)
    .await
    .map_err(|e: sqlx::Error| ApiError::new(StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
    .ok_or_else(|| {
        ApiError::new(StatusCode::NOT_FOUND, "Global component not found").with_code("NOT_FOUND")
    })?;

    Ok(Json(component))
}

/// Create a new global component
#[utoipa::path(
    post,
    path = "/api/cms/global-components",
    tag = "CMS Global Components",
    request_body = CreateGlobalComponentRequest,
    responses(
        (status = 201, description = "Component created successfully", body = GlobalComponent),
        (status = 400, description = "Validation error"),
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Forbidden"),
        (status = 409, description = "Slug already exists")
    ),
    security(("bearer_auth" = []))
)]
async fn create_global_component(
    State(state): State<AppState>,
    user: User,
    Json(request): Json<CreateGlobalComponentRequest>,
) -> Result<(StatusCode, Json<GlobalComponent>), ApiError> {
    require_cms_editor(&user)?;

    // Validate required fields
    if request.name.trim().is_empty() {
        return Err(
            ApiError::new(StatusCode::BAD_REQUEST, "Name is required").with_code("VALIDATION_ERROR")
        );
    }

    // Generate or validate slug
    let base_slug = request
        .slug
        .clone()
        .unwrap_or_else(|| generate_slug(&request.name));
    let slug = ensure_unique_slug(&state.db.pool, &base_slug, None).await?;

    // Get CMS user ID from platform user
    let cms_user_id = get_cms_user_id(&state.db.pool, user.id).await;

    let new_id = Uuid::new_v4();
    let component: GlobalComponent = sqlx::query_as(
        r#"
        INSERT INTO cms_global_components (
            id, name, slug, description, component_data, category, tags,
            thumbnail_url, usage_count, is_global, is_locked, version,
            created_at, updated_at, created_by, updated_by
        )
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, 0, $9, false, 1, NOW(), NOW(), $10, $10)
        RETURNING id, name, slug, description, component_data, category,
                  tags, thumbnail_url, usage_count, is_global, is_locked, version,
                  deleted_at, created_at, updated_at, created_by, updated_by
        "#,
    )
    .bind(new_id)
    .bind(request.name.trim())
    .bind(&slug)
    .bind(&request.description)
    .bind(&request.component_data)
    .bind(request.category as GlobalComponentCategory)
    .bind(&request.tags)
    .bind(&request.thumbnail_url)
    .bind(request.is_global)
    .bind(cms_user_id)
    .fetch_one(&state.db.pool)
    .await
    .map_err(|e: sqlx::Error| ApiError::new(StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    // Create initial version record
    create_version_record(
        &state.db.pool,
        new_id,
        1,
        &request.component_data,
        Some("Initial version"),
        cms_user_id,
    )
    .await?;

    tracing::info!(
        "Global component created: {} ({}) by user {}",
        component.name,
        component.id,
        user.id
    );

    Ok((StatusCode::CREATED, Json(component)))
}

/// Update a global component
#[utoipa::path(
    put,
    path = "/api/cms/global-components/{id}",
    tag = "CMS Global Components",
    params(("id" = Uuid, Path, description = "Component UUID")),
    request_body = UpdateGlobalComponentRequest,
    responses(
        (status = 200, description = "Component updated successfully", body = GlobalComponent),
        (status = 400, description = "Validation error"),
        (status = 404, description = "Component not found"),
        (status = 409, description = "Slug already exists"),
        (status = 423, description = "Component is locked")
    ),
    security(("bearer_auth" = []))
)]
async fn update_global_component(
    State(state): State<AppState>,
    user: User,
    Path(id): Path<Uuid>,
    Json(request): Json<UpdateGlobalComponentRequest>,
) -> ApiResult<GlobalComponent> {
    require_cms_editor(&user)?;

    // Check if component exists and get current state
    #[derive(Debug, sqlx::FromRow)]
    struct ExistingComponent {
        is_locked: bool,
        version: i32,
        component_data: JsonValue,
    }
    let existing: ExistingComponent = sqlx::query_as(
        "SELECT is_locked, version, component_data FROM cms_global_components WHERE id = $1 AND deleted_at IS NULL",
    )
    .bind(id)
    .fetch_optional(&state.db.pool)
    .await
    .map_err(|e: sqlx::Error| ApiError::new(StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
    .ok_or_else(|| {
        ApiError::new(StatusCode::NOT_FOUND, "Global component not found").with_code("NOT_FOUND")
    })?;

    // Only admins can update locked components
    if existing.is_locked && request.is_locked != Some(false) {
        require_cms_admin(&user)?;
    }

    // Handle slug update
    let final_slug = if let Some(ref new_slug) = request.slug {
        Some(ensure_unique_slug(&state.db.pool, new_slug, Some(id)).await?)
    } else {
        None
    };

    // Get CMS user ID
    let cms_user_id = get_cms_user_id(&state.db.pool, user.id).await;

    // Determine if we need to increment version (only if component_data changed)
    let increment_version = request.component_data.is_some();
    let new_version = if increment_version {
        existing.version + 1
    } else {
        existing.version
    };

    let component: GlobalComponent = sqlx::query_as(
        r#"
        UPDATE cms_global_components
        SET name = COALESCE($2, name),
            slug = COALESCE($3, slug),
            description = COALESCE($4, description),
            component_data = COALESCE($5, component_data),
            category = COALESCE($6, category),
            tags = COALESCE($7, tags),
            thumbnail_url = COALESCE($8, thumbnail_url),
            is_global = COALESCE($9, is_global),
            is_locked = COALESCE($10, is_locked),
            version = $11,
            updated_at = NOW(),
            updated_by = $12
        WHERE id = $1 AND deleted_at IS NULL
        RETURNING id, name, slug, description, component_data, category,
                  tags, thumbnail_url, usage_count, is_global, is_locked, version,
                  deleted_at, created_at, updated_at, created_by, updated_by
        "#,
    )
    .bind(id)
    .bind(&request.name)
    .bind(&final_slug)
    .bind(&request.description)
    .bind(&request.component_data)
    .bind(request.category.map(|c| c as GlobalComponentCategory))
    .bind(&request.tags)
    .bind(&request.thumbnail_url)
    .bind(request.is_global)
    .bind(request.is_locked)
    .bind(new_version)
    .bind(cms_user_id)
    .fetch_one(&state.db.pool)
    .await
    .map_err(|e: sqlx::Error| ApiError::new(StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    // Create version record if component_data changed
    if let Some(ref new_data) = request.component_data {
        create_version_record(
            &state.db.pool,
            id,
            new_version,
            new_data,
            request.change_message.as_deref(),
            cms_user_id,
        )
        .await?;
    }

    tracing::info!(
        "Global component updated: {} ({}) v{} by user {}",
        component.name,
        component.id,
        component.version,
        user.id
    );

    Ok(Json(component))
}

/// Delete a global component (soft delete)
#[utoipa::path(
    delete,
    path = "/api/cms/global-components/{id}",
    tag = "CMS Global Components",
    params(("id" = Uuid, Path, description = "Component UUID")),
    responses(
        (status = 200, description = "Component deleted successfully"),
        (status = 404, description = "Component not found"),
        (status = 423, description = "Component is locked")
    ),
    security(("bearer_auth" = []))
)]
async fn delete_global_component(
    State(state): State<AppState>,
    user: User,
    Path(id): Path<Uuid>,
) -> ApiResultEmpty {
    require_cms_admin(&user)?;

    // Check if component exists and is not locked
    #[derive(Debug, sqlx::FromRow)]
    struct ExistingComponent {
        is_locked: bool,
        name: String,
        usage_count: i32,
    }
    let existing: ExistingComponent = sqlx::query_as(
        "SELECT is_locked, name, usage_count FROM cms_global_components WHERE id = $1 AND deleted_at IS NULL",
    )
    .bind(id)
    .fetch_optional(&state.db.pool)
    .await
    .map_err(|e: sqlx::Error| ApiError::new(StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
    .ok_or_else(|| {
        ApiError::new(StatusCode::NOT_FOUND, "Global component not found").with_code("NOT_FOUND")
    })?;

    if existing.is_locked {
        return Err(ApiError::new(
            StatusCode::LOCKED,
            "Cannot delete a locked component. Unlock it first.",
        )
        .with_code("COMPONENT_LOCKED"));
    }

    // Soft delete
    sqlx::query("UPDATE cms_global_components SET deleted_at = NOW() WHERE id = $1")
        .bind(id)
        .execute(&state.db.pool)
        .await
        .map_err(|e: sqlx::Error| ApiError::new(StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    tracing::info!(
        "Global component deleted: {} ({}) by user {}",
        existing.name,
        id,
        user.id
    );

    Ok(Json(json!({
        "message": "Component deleted successfully",
        "id": id,
        "usage_count": existing.usage_count
    })))
}

/// Duplicate a global component
#[utoipa::path(
    post,
    path = "/api/cms/global-components/{id}/duplicate",
    tag = "CMS Global Components",
    params(("id" = Uuid, Path, description = "Component UUID to duplicate")),
    responses(
        (status = 201, description = "Component duplicated successfully", body = GlobalComponent),
        (status = 404, description = "Component not found")
    ),
    security(("bearer_auth" = []))
)]
async fn duplicate_global_component(
    State(state): State<AppState>,
    user: User,
    Path(id): Path<Uuid>,
) -> Result<(StatusCode, Json<GlobalComponent>), ApiError> {
    require_cms_editor(&user)?;

    // Fetch the original component
    let original: GlobalComponent = sqlx::query_as(
        r#"
        SELECT id, name, slug, description, component_data, category,
               tags, thumbnail_url, usage_count, is_global, is_locked, version,
               deleted_at, created_at, updated_at, created_by, updated_by
        FROM cms_global_components
        WHERE id = $1 AND deleted_at IS NULL
        "#,
    )
    .bind(id)
    .fetch_optional(&state.db.pool)
    .await
    .map_err(|e: sqlx::Error| ApiError::new(StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
    .ok_or_else(|| {
        ApiError::new(StatusCode::NOT_FOUND, "Global component not found").with_code("NOT_FOUND")
    })?;

    // Generate new name and slug
    let new_name = format!("{} (Copy)", original.name);
    let base_slug = generate_slug(&new_name);
    let new_slug = ensure_unique_slug(&state.db.pool, &base_slug, None).await?;

    // Get CMS user ID
    let cms_user_id = get_cms_user_id(&state.db.pool, user.id).await;

    let dup_id = Uuid::new_v4();
    let duplicated: GlobalComponent = sqlx::query_as(
        r#"
        INSERT INTO cms_global_components (
            id, name, slug, description, component_data, category, tags,
            thumbnail_url, usage_count, is_global, is_locked, version,
            created_at, updated_at, created_by, updated_by
        )
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, 0, $9, false, 1, NOW(), NOW(), $10, $10)
        RETURNING id, name, slug, description, component_data, category,
                  tags, thumbnail_url, usage_count, is_global, is_locked, version,
                  deleted_at, created_at, updated_at, created_by, updated_by
        "#,
    )
    .bind(dup_id)
    .bind(&new_name)
    .bind(&new_slug)
    .bind(&original.description)
    .bind(&original.component_data)
    .bind(original.category as GlobalComponentCategory)
    .bind(&original.tags)
    .bind(&original.thumbnail_url)
    .bind(original.is_global)
    .bind(cms_user_id)
    .fetch_one(&state.db.pool)
    .await
    .map_err(|e: sqlx::Error| ApiError::new(StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    // Create initial version record for duplicate
    create_version_record(
        &state.db.pool,
        dup_id,
        1,
        &original.component_data,
        Some(&format!("Duplicated from {}", original.name)),
        cms_user_id,
    )
    .await?;

    tracing::info!(
        "Global component duplicated: {} -> {} ({}) by user {}",
        original.name,
        duplicated.name,
        duplicated.id,
        user.id
    );

    Ok((StatusCode::CREATED, Json(duplicated)))
}

/// Get version history for a global component
#[utoipa::path(
    get,
    path = "/api/cms/global-components/{id}/versions",
    tag = "CMS Global Components",
    params(("id" = Uuid, Path, description = "Component UUID")),
    responses(
        (status = 200, description = "Version history", body = VersionHistoryResponse),
        (status = 404, description = "Component not found")
    ),
    security(("bearer_auth" = []))
)]
async fn get_version_history(
    State(state): State<AppState>,
    user: User,
    Path(id): Path<Uuid>,
) -> ApiResult<VersionHistoryResponse> {
    require_cms_editor(&user)?;

    // Get current version
    #[derive(Debug, sqlx::FromRow)]
    struct CurrentVersion {
        version: i32,
    }
    let current: CurrentVersion = sqlx::query_as(
        "SELECT version FROM cms_global_components WHERE id = $1 AND deleted_at IS NULL",
    )
    .bind(id)
    .fetch_optional(&state.db.pool)
    .await
    .map_err(|e: sqlx::Error| ApiError::new(StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
    .ok_or_else(|| {
        ApiError::new(StatusCode::NOT_FOUND, "Global component not found").with_code("NOT_FOUND")
    })?;

    // Get all versions
    let versions: Vec<GlobalComponentVersion> = sqlx::query_as(
        r#"
        SELECT v.id, v.component_id, v.version, v.component_data,
               v.change_message, v.created_by, u.name as created_by_name, v.created_at
        FROM cms_global_component_versions v
        LEFT JOIN cms_users u ON u.id = v.created_by
        WHERE v.component_id = $1
        ORDER BY v.version DESC
        "#,
    )
    .bind(id)
    .fetch_all(&state.db.pool)
    .await
    .map_err(|e: sqlx::Error| ApiError::new(StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(VersionHistoryResponse {
        total_count: versions.len() as i64,
        versions,
        current_version: current.version,
    }))
}

/// Restore a specific version of a global component
#[utoipa::path(
    post,
    path = "/api/cms/global-components/{id}/versions/{version}/restore",
    tag = "CMS Global Components",
    params(
        ("id" = Uuid, Path, description = "Component UUID"),
        ("version" = i32, Path, description = "Version number to restore")
    ),
    responses(
        (status = 200, description = "Version restored successfully", body = GlobalComponent),
        (status = 404, description = "Component or version not found"),
        (status = 423, description = "Component is locked")
    ),
    security(("bearer_auth" = []))
)]
async fn restore_version(
    State(state): State<AppState>,
    user: User,
    Path((id, version)): Path<(Uuid, i32)>,
) -> ApiResult<GlobalComponent> {
    require_cms_editor(&user)?;

    // Check if component exists and is not locked
    #[derive(Debug, sqlx::FromRow)]
    struct ExistingComponent {
        is_locked: bool,
        current_version: i32,
    }
    let existing: ExistingComponent = sqlx::query_as(
        "SELECT is_locked, version as current_version FROM cms_global_components WHERE id = $1 AND deleted_at IS NULL",
    )
    .bind(id)
    .fetch_optional(&state.db.pool)
    .await
    .map_err(|e: sqlx::Error| ApiError::new(StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
    .ok_or_else(|| {
        ApiError::new(StatusCode::NOT_FOUND, "Global component not found").with_code("NOT_FOUND")
    })?;

    if existing.is_locked {
        require_cms_admin(&user)?;
    }

    // Get the version to restore
    #[derive(Debug, sqlx::FromRow)]
    struct VersionRecord {
        component_data: JsonValue,
    }
    let version_record: VersionRecord = sqlx::query_as(
        "SELECT component_data FROM cms_global_component_versions WHERE component_id = $1 AND version = $2",
    )
    .bind(id)
    .bind(version)
    .fetch_optional(&state.db.pool)
    .await
    .map_err(|e: sqlx::Error| ApiError::new(StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
    .ok_or_else(|| {
        ApiError::new(StatusCode::NOT_FOUND, "Version not found").with_code("VERSION_NOT_FOUND")
    })?;

    // Get CMS user ID
    let cms_user_id = get_cms_user_id(&state.db.pool, user.id).await;
    let new_version = existing.current_version + 1;

    // Update component with restored data
    let component: GlobalComponent = sqlx::query_as(
        r#"
        UPDATE cms_global_components
        SET component_data = $2,
            version = $3,
            updated_at = NOW(),
            updated_by = $4
        WHERE id = $1 AND deleted_at IS NULL
        RETURNING id, name, slug, description, component_data, category,
                  tags, thumbnail_url, usage_count, is_global, is_locked, version,
                  deleted_at, created_at, updated_at, created_by, updated_by
        "#,
    )
    .bind(id)
    .bind(&version_record.component_data)
    .bind(new_version)
    .bind(cms_user_id)
    .fetch_one(&state.db.pool)
    .await
    .map_err(|e: sqlx::Error| ApiError::new(StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    // Create version record for restoration
    create_version_record(
        &state.db.pool,
        id,
        new_version,
        &version_record.component_data,
        Some(&format!("Restored from version {}", version)),
        cms_user_id,
    )
    .await?;

    tracing::info!(
        "Global component version restored: {} ({}) v{} -> v{} by user {}",
        component.name,
        id,
        version,
        new_version,
        user.id
    );

    Ok(Json(component))
}

/// Get usage locations for a global component
#[utoipa::path(
    get,
    path = "/api/cms/global-components/{id}/usage",
    tag = "CMS Global Components",
    params(("id" = Uuid, Path, description = "Component UUID")),
    responses(
        (status = 200, description = "Component usage locations", body = ComponentUsageResponse),
        (status = 404, description = "Component not found")
    ),
    security(("bearer_auth" = []))
)]
async fn get_component_usage(
    State(state): State<AppState>,
    user: User,
    Path(id): Path<Uuid>,
) -> ApiResult<ComponentUsageResponse> {
    require_cms_editor(&user)?;

    // Get current version
    #[derive(Debug, sqlx::FromRow)]
    struct CurrentVersion {
        version: i32,
    }
    let current: CurrentVersion = sqlx::query_as(
        "SELECT version FROM cms_global_components WHERE id = $1 AND deleted_at IS NULL",
    )
    .bind(id)
    .fetch_optional(&state.db.pool)
    .await
    .map_err(|e: sqlx::Error| ApiError::new(StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
    .ok_or_else(|| {
        ApiError::new(StatusCode::NOT_FOUND, "Global component not found").with_code("NOT_FOUND")
    })?;

    // Fetch usage with content metadata
    #[derive(Debug, sqlx::FromRow)]
    struct UsageRow {
        id: Uuid,
        component_id: Uuid,
        content_id: Uuid,
        content_title: String,
        content_type: String,
        content_slug: String,
        instance_id: String,
        is_synced: bool,
        synced_version: i32,
        needs_update: bool,
        detached_at: Option<DateTime<Utc>>,
        created_at: DateTime<Utc>,
    }
    let usage_rows: Vec<UsageRow> = sqlx::query_as(
        r#"
        SELECT
            u.id,
            u.component_id,
            u.content_id,
            c.title as content_title,
            c.content_type::text as content_type,
            c.slug as content_slug,
            u.instance_id,
            u.is_synced,
            u.synced_version,
            (u.is_synced AND u.synced_version < $2) as needs_update,
            u.detached_at,
            u.created_at
        FROM cms_global_component_usage u
        JOIN cms_content c ON c.id = u.content_id
        WHERE u.component_id = $1
        ORDER BY u.created_at DESC
        "#,
    )
    .bind(id)
    .bind(current.version)
    .fetch_all(&state.db.pool)
    .await
    .map_err(|e: sqlx::Error| ApiError::new(StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let usages: Vec<GlobalComponentUsageWithContent> = usage_rows
        .into_iter()
        .map(|r| GlobalComponentUsageWithContent {
            id: r.id,
            component_id: r.component_id,
            content_id: r.content_id,
            content_title: r.content_title,
            content_type: r.content_type,
            content_slug: r.content_slug,
            instance_id: r.instance_id,
            is_synced: r.is_synced,
            synced_version: r.synced_version,
            needs_update: r.needs_update,
            detached_at: r.detached_at,
            created_at: r.created_at,
        })
        .collect();

    let total_count = usages.len() as i64;
    let synced_count = usages.iter().filter(|u| u.is_synced).count() as i64;
    let outdated_count = usages.iter().filter(|u| u.needs_update).count() as i64;

    Ok(Json(ComponentUsageResponse {
        usages,
        total_count,
        synced_count,
        outdated_count,
    }))
}

/// Track usage when a global component is inserted into content
#[utoipa::path(
    post,
    path = "/api/cms/global-components/usage",
    tag = "CMS Global Components",
    request_body = TrackComponentUsageRequest,
    responses(
        (status = 201, description = "Usage tracked successfully", body = GlobalComponentUsage),
        (status = 400, description = "Validation error"),
        (status = 404, description = "Component or content not found")
    ),
    security(("bearer_auth" = []))
)]
async fn track_component_usage(
    State(state): State<AppState>,
    user: User,
    Json(request): Json<TrackComponentUsageRequest>,
) -> Result<(StatusCode, Json<GlobalComponentUsage>), ApiError> {
    require_cms_editor(&user)?;

    // Get component and its current version
    #[derive(Debug, sqlx::FromRow)]
    struct ComponentVersion {
        version: i32,
    }
    let component: ComponentVersion = sqlx::query_as(
        "SELECT version FROM cms_global_components WHERE id = $1 AND deleted_at IS NULL",
    )
    .bind(request.component_id)
    .fetch_optional(&state.db.pool)
    .await
    .map_err(|e: sqlx::Error| ApiError::new(StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
    .ok_or_else(|| {
        ApiError::new(StatusCode::NOT_FOUND, "Global component not found").with_code("NOT_FOUND")
    })?;

    // Verify content exists
    let content_exists: (bool,) = sqlx::query_as(
        "SELECT EXISTS(SELECT 1 FROM cms_content WHERE id = $1 AND deleted_at IS NULL)",
    )
    .bind(request.content_id)
    .fetch_one(&state.db.pool)
    .await
    .map_err(|e: sqlx::Error| ApiError::new(StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    if !content_exists.0 {
        return Err(
            ApiError::new(StatusCode::NOT_FOUND, "Content not found").with_code("NOT_FOUND")
        );
    }

    // Create usage record
    let usage: GlobalComponentUsage = sqlx::query_as(
        r#"
        INSERT INTO cms_global_component_usage (
            id, component_id, content_id, instance_id, is_synced, synced_version, created_at
        )
        VALUES ($1, $2, $3, $4, $5, $6, NOW())
        RETURNING id, component_id, content_id, instance_id, is_synced, synced_version,
                  detached_at, detached_by, created_at
        "#,
    )
    .bind(Uuid::new_v4())
    .bind(request.component_id)
    .bind(request.content_id)
    .bind(&request.instance_id)
    .bind(request.is_synced)
    .bind(component.version)
    .fetch_one(&state.db.pool)
    .await
    .map_err(|e: sqlx::Error| ApiError::new(StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    // Increment usage count on the component
    sqlx::query("UPDATE cms_global_components SET usage_count = usage_count + 1 WHERE id = $1")
        .bind(request.component_id)
        .execute(&state.db.pool)
        .await
        .map_err(|e: sqlx::Error| ApiError::new(StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    tracing::info!(
        "Component usage tracked: component {} in content {} (instance {}) by user {}",
        request.component_id,
        request.content_id,
        request.instance_id,
        user.id
    );

    Ok((StatusCode::CREATED, Json(usage)))
}

/// Remove usage tracking when a component is removed from content
#[utoipa::path(
    delete,
    path = "/api/cms/global-components/usage/{id}",
    tag = "CMS Global Components",
    params(("id" = Uuid, Path, description = "Usage record UUID")),
    responses(
        (status = 200, description = "Usage removed successfully"),
        (status = 404, description = "Usage record not found")
    ),
    security(("bearer_auth" = []))
)]
async fn remove_component_usage(
    State(state): State<AppState>,
    user: User,
    Path(id): Path<Uuid>,
) -> ApiResultEmpty {
    require_cms_editor(&user)?;

    // Get usage record to find the component ID
    #[derive(Debug, sqlx::FromRow)]
    struct UsageRecord {
        component_id: Uuid,
    }
    let usage: UsageRecord = sqlx::query_as(
        "SELECT component_id FROM cms_global_component_usage WHERE id = $1",
    )
    .bind(id)
    .fetch_optional(&state.db.pool)
    .await
    .map_err(|e: sqlx::Error| ApiError::new(StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
    .ok_or_else(|| {
        ApiError::new(StatusCode::NOT_FOUND, "Usage record not found").with_code("NOT_FOUND")
    })?;

    // Delete the usage record
    sqlx::query("DELETE FROM cms_global_component_usage WHERE id = $1")
        .bind(id)
        .execute(&state.db.pool)
        .await
        .map_err(|e: sqlx::Error| ApiError::new(StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    // Decrement usage count on the component
    sqlx::query(
        "UPDATE cms_global_components SET usage_count = GREATEST(0, usage_count - 1) WHERE id = $1",
    )
    .bind(usage.component_id)
    .execute(&state.db.pool)
    .await
    .map_err(|e: sqlx::Error| ApiError::new(StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    tracing::info!(
        "Component usage removed: usage {} from component {} by user {}",
        id,
        usage.component_id,
        user.id
    );

    Ok(Json(json!({
        "message": "Usage removed successfully",
        "id": id
    })))
}

/// Sync a component instance to the latest version
#[utoipa::path(
    post,
    path = "/api/cms/global-components/usage/{id}/sync",
    tag = "CMS Global Components",
    params(("id" = Uuid, Path, description = "Usage record UUID")),
    request_body = SyncComponentRequest,
    responses(
        (status = 200, description = "Component synced to latest version", body = GlobalComponentUsage),
        (status = 404, description = "Usage record not found"),
        (status = 409, description = "Component is detached")
    ),
    security(("bearer_auth" = []))
)]
async fn sync_component_usage(
    State(state): State<AppState>,
    user: User,
    Path(id): Path<Uuid>,
    Json(_request): Json<SyncComponentRequest>,
) -> ApiResult<GlobalComponentUsage> {
    require_cms_editor(&user)?;

    // Check if usage exists and is synced
    #[derive(Debug, sqlx::FromRow)]
    struct ExistingUsage {
        is_synced: bool,
        component_id: Uuid,
        current_version: i32,
    }
    let existing: ExistingUsage = sqlx::query_as(
        r#"
        SELECT u.is_synced, u.component_id, c.version as current_version
        FROM cms_global_component_usage u
        JOIN cms_global_components c ON c.id = u.component_id
        WHERE u.id = $1
        "#,
    )
    .bind(id)
    .fetch_optional(&state.db.pool)
    .await
    .map_err(|e: sqlx::Error| ApiError::new(StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
    .ok_or_else(|| {
        ApiError::new(StatusCode::NOT_FOUND, "Usage record not found").with_code("NOT_FOUND")
    })?;

    if !existing.is_synced {
        return Err(ApiError::new(
            StatusCode::CONFLICT,
            "Component instance is detached. Re-attach it first to sync.",
        )
        .with_code("COMPONENT_DETACHED"));
    }

    // Update synced version
    let usage: GlobalComponentUsage = sqlx::query_as(
        r#"
        UPDATE cms_global_component_usage
        SET synced_version = $2
        WHERE id = $1
        RETURNING id, component_id, content_id, instance_id, is_synced, synced_version,
                  detached_at, detached_by, created_at
        "#,
    )
    .bind(id)
    .bind(existing.current_version)
    .fetch_one(&state.db.pool)
    .await
    .map_err(|e: sqlx::Error| ApiError::new(StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    tracing::info!(
        "Component usage synced: {} to v{} by user {}",
        id,
        existing.current_version,
        user.id
    );

    Ok(Json(usage))
}

/// Detach a component instance from sync (copy as local)
#[utoipa::path(
    post,
    path = "/api/cms/global-components/usage/{id}/detach",
    tag = "CMS Global Components",
    params(("id" = Uuid, Path, description = "Usage record UUID")),
    responses(
        (status = 200, description = "Component instance detached from sync", body = GlobalComponentUsage),
        (status = 404, description = "Usage record not found"),
        (status = 409, description = "Already detached")
    ),
    security(("bearer_auth" = []))
)]
async fn detach_component_usage(
    State(state): State<AppState>,
    user: User,
    Path(id): Path<Uuid>,
) -> ApiResult<GlobalComponentUsage> {
    require_cms_editor(&user)?;

    // Check if already detached
    #[derive(Debug, sqlx::FromRow)]
    struct ExistingUsage {
        is_synced: bool,
    }
    let existing: ExistingUsage = sqlx::query_as(
        "SELECT is_synced FROM cms_global_component_usage WHERE id = $1",
    )
    .bind(id)
    .fetch_optional(&state.db.pool)
    .await
    .map_err(|e: sqlx::Error| ApiError::new(StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
    .ok_or_else(|| {
        ApiError::new(StatusCode::NOT_FOUND, "Usage record not found").with_code("NOT_FOUND")
    })?;

    if !existing.is_synced {
        return Err(
            ApiError::new(StatusCode::CONFLICT, "Component instance is already detached")
                .with_code("ALREADY_DETACHED"),
        );
    }

    // Get CMS user ID
    let cms_user_id = get_cms_user_id(&state.db.pool, user.id).await;

    // Detach the usage
    let usage: GlobalComponentUsage = sqlx::query_as(
        r#"
        UPDATE cms_global_component_usage
        SET is_synced = false, detached_at = NOW(), detached_by = $2
        WHERE id = $1
        RETURNING id, component_id, content_id, instance_id, is_synced, synced_version,
                  detached_at, detached_by, created_at
        "#,
    )
    .bind(id)
    .bind(cms_user_id)
    .fetch_one(&state.db.pool)
    .await
    .map_err(|e: sqlx::Error| ApiError::new(StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    tracing::info!(
        "Component usage detached: {} by user {}",
        id,
        user.id
    );

    Ok(Json(usage))
}

/// Re-attach a detached component instance to sync
#[utoipa::path(
    post,
    path = "/api/cms/global-components/usage/{id}/reattach",
    tag = "CMS Global Components",
    params(("id" = Uuid, Path, description = "Usage record UUID")),
    responses(
        (status = 200, description = "Component instance re-attached to sync", body = GlobalComponentUsage),
        (status = 404, description = "Usage record not found"),
        (status = 409, description = "Already synced")
    ),
    security(("bearer_auth" = []))
)]
async fn reattach_component_usage(
    State(state): State<AppState>,
    user: User,
    Path(id): Path<Uuid>,
) -> ApiResult<GlobalComponentUsage> {
    require_cms_editor(&user)?;

    // Check if already synced
    #[derive(Debug, sqlx::FromRow)]
    struct ExistingUsage {
        is_synced: bool,
        current_version: i32,
    }
    let existing: ExistingUsage = sqlx::query_as(
        r#"
        SELECT u.is_synced, c.version as current_version
        FROM cms_global_component_usage u
        JOIN cms_global_components c ON c.id = u.component_id
        WHERE u.id = $1
        "#,
    )
    .bind(id)
    .fetch_optional(&state.db.pool)
    .await
    .map_err(|e: sqlx::Error| ApiError::new(StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
    .ok_or_else(|| {
        ApiError::new(StatusCode::NOT_FOUND, "Usage record not found").with_code("NOT_FOUND")
    })?;

    if existing.is_synced {
        return Err(
            ApiError::new(StatusCode::CONFLICT, "Component instance is already synced")
                .with_code("ALREADY_SYNCED"),
        );
    }

    // Re-attach and sync to latest version
    let usage: GlobalComponentUsage = sqlx::query_as(
        r#"
        UPDATE cms_global_component_usage
        SET is_synced = true, synced_version = $2, detached_at = NULL, detached_by = NULL
        WHERE id = $1
        RETURNING id, component_id, content_id, instance_id, is_synced, synced_version,
                  detached_at, detached_by, created_at
        "#,
    )
    .bind(id)
    .bind(existing.current_version)
    .fetch_one(&state.db.pool)
    .await
    .map_err(|e: sqlx::Error| ApiError::new(StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    tracing::info!(
        "Component usage re-attached: {} to v{} by user {}",
        id,
        existing.current_version,
        user.id
    );

    Ok(Json(usage))
}

// ============================================================================
// PUBLIC ENDPOINTS
// ============================================================================

/// Public endpoint to get a global component by slug (no auth required)
async fn get_global_component_public(
    State(state): State<AppState>,
    Path(slug): Path<String>,
) -> ApiResult<GlobalComponent> {
    let component: GlobalComponent = sqlx::query_as(
        r#"
        SELECT id, name, slug, description, component_data, category, tags,
               thumbnail_url, usage_count, is_global, is_locked, version,
               deleted_at, created_at, updated_at, created_by, updated_by
        FROM cms_global_components
        WHERE slug = $1 AND deleted_at IS NULL AND is_global = true
        "#,
    )
    .bind(&slug)
    .fetch_optional(&state.db.pool)
    .await
    .map_err(|e: sqlx::Error| ApiError::new(StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
    .ok_or_else(|| {
        ApiError::new(StatusCode::NOT_FOUND, "Global component not found").with_code("NOT_FOUND")
    })?;

    Ok(Json(component))
}

/// Public endpoint to get a global component by ID (no auth required)
async fn get_global_component_by_id_public(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> ApiResult<GlobalComponent> {
    let component: GlobalComponent = sqlx::query_as(
        r#"
        SELECT id, name, slug, description, component_data, category, tags,
               thumbnail_url, usage_count, is_global, is_locked, version,
               deleted_at, created_at, updated_at, created_by, updated_by
        FROM cms_global_components
        WHERE id = $1 AND deleted_at IS NULL AND is_global = true
        "#,
    )
    .bind(id)
    .fetch_optional(&state.db.pool)
    .await
    .map_err(|e: sqlx::Error| ApiError::new(StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
    .ok_or_else(|| {
        ApiError::new(StatusCode::NOT_FOUND, "Global component not found").with_code("NOT_FOUND")
    })?;

    Ok(Json(component))
}

// ============================================================================
// ROUTER
// ============================================================================

/// Admin router for global components (requires authentication)
pub fn admin_router() -> Router<AppState> {
    Router::new()
        // Component CRUD
        .route("/", get(list_global_components).post(create_global_component))
        .route(
            "/{id}",
            get(get_global_component)
                .put(update_global_component)
                .delete(delete_global_component),
        )
        .route("/slug/{slug}", get(get_global_component_by_slug))
        .route("/{id}/duplicate", post(duplicate_global_component))
        // Version history
        .route("/{id}/versions", get(get_version_history))
        .route("/{id}/versions/{version}/restore", post(restore_version))
        // Usage tracking
        .route("/{id}/usage", get(get_component_usage))
        .route("/usage", post(track_component_usage))
        .route("/usage/{id}", delete(remove_component_usage))
        .route("/usage/{id}/sync", post(sync_component_usage))
        .route("/usage/{id}/detach", post(detach_component_usage))
        .route("/usage/{id}/reattach", post(reattach_component_usage))
}

/// Public router for read-only component access
pub fn public_router() -> Router<AppState> {
    Router::new()
        .route("/slug/{slug}", get(get_global_component_public))
        .route("/{id}", get(get_global_component_by_id_public))
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_slug() {
        assert_eq!(generate_slug("Main Header"), "main-header");
        assert_eq!(generate_slug("Footer v2"), "footer-v2");
        assert_eq!(generate_slug("  Multiple   Spaces  "), "multiple-spaces");
    }

    #[test]
    fn test_category_display() {
        assert_eq!(format!("{}", GlobalComponentCategory::Header), "header");
        assert_eq!(format!("{}", GlobalComponentCategory::Footer), "footer");
        assert_eq!(format!("{}", GlobalComponentCategory::Cta), "cta");
        assert_eq!(format!("{}", GlobalComponentCategory::Form), "form");
        assert_eq!(format!("{}", GlobalComponentCategory::Navigation), "navigation");
    }

    #[test]
    fn test_default_true() {
        assert!(default_true());
    }
}
