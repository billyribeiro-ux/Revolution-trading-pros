//! CMS Datasources API Routes - Reusable Option Lists
//!
//! Apple ICT 7+ Principal Engineer Grade - February 2026
//!
//! Comprehensive API for managing datasources (reusable option lists) in the CMS.
//! Provides key-value pairs for dropdowns, selects, and other form elements.
//!
//! Features:
//! - CRUD operations for datasources
//! - Entry management with drag-to-reorder
//! - CSV import/export
//! - Dimension support for translations
//! - Pagination for large datasources
//! - Search and filtering
//!
//! @version 1.0.0 - February 2026

use axum::{
    extract::{Multipart, Path, Query, State},
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

/// Datasource container for reusable option lists
#[derive(Debug, Clone, Serialize, Deserialize, FromRow, ToSchema)]
pub struct CmsDatasource {
    pub id: Uuid,
    pub name: String,
    pub slug: String,
    pub description: Option<String>,
    pub icon: Option<String>,
    pub color: Option<String>,
    pub entry_count: i32,
    pub is_system: bool,
    pub is_locked: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub created_by: Option<Uuid>,
    pub updated_by: Option<Uuid>,
    pub deleted_at: Option<DateTime<Utc>>,
}

/// Lightweight datasource summary for listings
#[derive(Debug, Clone, Serialize, Deserialize, FromRow, ToSchema)]
pub struct CmsDatasourceSummary {
    pub id: Uuid,
    pub name: String,
    pub slug: String,
    pub description: Option<String>,
    pub icon: Option<String>,
    pub color: Option<String>,
    pub entry_count: i32,
    pub is_system: bool,
    pub is_locked: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Datasource entry (key-value pair)
#[derive(Debug, Clone, Serialize, Deserialize, FromRow, ToSchema)]
pub struct CmsDatasourceEntry {
    pub id: Uuid,
    pub datasource_id: Uuid,
    pub name: String,
    pub value: String,
    pub dimension: String,
    pub sort_order: i32,
    pub metadata: Option<JsonValue>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Lightweight entry for listings
#[derive(Debug, Clone, Serialize, Deserialize, FromRow, ToSchema)]
pub struct CmsDatasourceEntrySummary {
    pub id: Uuid,
    pub name: String,
    pub value: String,
    pub dimension: String,
    pub sort_order: i32,
    pub metadata: Option<JsonValue>,
}

// ============================================================================
// REQUEST/RESPONSE MODELS
// ============================================================================

/// Request to create a new datasource
#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateDatasourceRequest {
    pub name: String,
    pub slug: Option<String>,
    pub description: Option<String>,
    pub icon: Option<String>,
    pub color: Option<String>,
}

/// Request to update a datasource
#[derive(Debug, Deserialize, ToSchema)]
pub struct UpdateDatasourceRequest {
    pub name: Option<String>,
    pub slug: Option<String>,
    pub description: Option<String>,
    pub icon: Option<String>,
    pub color: Option<String>,
    pub is_locked: Option<bool>,
}

/// Request to create a new entry
#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateEntryRequest {
    pub name: String,
    pub value: String,
    pub dimension: Option<String>,
    pub metadata: Option<JsonValue>,
}

/// Request to update an entry
#[derive(Debug, Deserialize, ToSchema)]
pub struct UpdateEntryRequest {
    pub name: Option<String>,
    pub value: Option<String>,
    pub dimension: Option<String>,
    pub sort_order: Option<i32>,
    pub metadata: Option<JsonValue>,
}

/// Request to bulk create entries
#[derive(Debug, Deserialize, ToSchema)]
pub struct BulkCreateEntriesRequest {
    pub entries: Vec<CreateEntryRequest>,
}

/// Request to reorder entries
#[derive(Debug, Deserialize, ToSchema)]
pub struct ReorderEntriesRequest {
    pub entry_ids: Vec<Uuid>,
}

/// Query parameters for listing datasources
#[derive(Debug, Deserialize, IntoParams, ToSchema)]
pub struct ListDatasourcesQuery {
    pub search: Option<String>,
    pub is_system: Option<bool>,
    pub sort_by: Option<String>,
    pub sort_order: Option<String>,
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

/// Query parameters for listing entries
#[derive(Debug, Deserialize, IntoParams, ToSchema)]
pub struct ListEntriesQuery {
    pub dimension: Option<String>,
    pub search: Option<String>,
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

/// Paginated response wrapper for datasources
#[derive(Debug, Serialize, ToSchema)]
pub struct PaginatedDatasourcesResponse {
    pub data: Vec<CmsDatasourceSummary>,
    pub meta: PaginationMeta,
}

/// Paginated response wrapper for entries
#[derive(Debug, Serialize, ToSchema)]
pub struct PaginatedEntriesResponse {
    pub data: Vec<CmsDatasourceEntrySummary>,
    pub meta: PaginationMeta,
    pub dimensions: Vec<String>,
}

/// Pagination metadata
#[derive(Debug, Serialize, ToSchema)]
pub struct PaginationMeta {
    pub total: i64,
    pub limit: i64,
    pub offset: i64,
    pub has_more: bool,
}

/// CSV export row
#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct CsvEntryRow {
    pub name: String,
    pub value: String,
    pub dimension: String,
}

/// Response for public datasource access
#[derive(Debug, Serialize, ToSchema)]
pub struct PublicDatasourceResponse {
    pub slug: String,
    pub name: String,
    pub entries: Vec<PublicEntryResponse>,
}

/// Public entry response (minimal)
#[derive(Debug, Serialize, FromRow, ToSchema)]
pub struct PublicEntryResponse {
    pub name: String,
    pub value: String,
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

/// Ensure slug uniqueness
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
                        SELECT 1 FROM cms_datasources
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
                        SELECT 1 FROM cms_datasources
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

// ============================================================================
// DATASOURCE ROUTE HANDLERS
// ============================================================================

/// List all datasources with pagination
#[utoipa::path(
    get,
    path = "/api/cms/datasources",
    tag = "CMS Datasources",
    params(ListDatasourcesQuery),
    responses(
        (status = 200, description = "List of datasources", body = PaginatedDatasourcesResponse),
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Forbidden")
    ),
    security(("bearer_auth" = []))
)]
async fn list_datasources(
    State(state): State<AppState>,
    user: User,
    Query(query): Query<ListDatasourcesQuery>,
) -> ApiResult<PaginatedDatasourcesResponse> {
    require_cms_editor(&user)?;

    let limit = query.limit.unwrap_or(50).min(200);
    let offset = query.offset.unwrap_or(0);

    let sort_column = match query.sort_by.as_deref() {
        Some("name") => "name",
        Some("entry_count") => "entry_count",
        Some("updated_at") => "updated_at",
        _ => "created_at",
    };
    let sort_order = match query.sort_order.as_deref() {
        Some("asc") => "ASC",
        _ => "DESC",
    };

    let search_pattern = query.search.as_ref().map(|s| format!("%{}%", s));

    // Count total
    let count_result: (i64,) = sqlx::query_as(
        r#"
        SELECT COUNT(*)
        FROM cms_datasources
        WHERE deleted_at IS NULL
          AND ($1::text IS NULL OR name ILIKE $1 OR description ILIKE $1)
          AND ($2::boolean IS NULL OR is_system = $2)
        "#,
    )
    .bind(&search_pattern)
    .bind(query.is_system)
    .fetch_one(&state.db.pool)
    .await
    .map_err(|e: sqlx::Error| ApiError::new(StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let total = count_result.0;

    // Fetch datasources
    let datasources: Vec<CmsDatasourceSummary> = sqlx::query_as(&format!(
        r#"
        SELECT id, name, slug, description, icon, color, entry_count,
               is_system, is_locked, created_at, updated_at
        FROM cms_datasources
        WHERE deleted_at IS NULL
          AND ($1::text IS NULL OR name ILIKE $1 OR description ILIKE $1)
          AND ($2::boolean IS NULL OR is_system = $2)
        ORDER BY is_system DESC, {} {}
        LIMIT $3 OFFSET $4
        "#,
        sort_column, sort_order
    ))
    .bind(&search_pattern)
    .bind(query.is_system)
    .bind(limit)
    .bind(offset)
    .fetch_all(&state.db.pool)
    .await
    .map_err(|e: sqlx::Error| ApiError::new(StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(PaginatedDatasourcesResponse {
        data: datasources,
        meta: PaginationMeta {
            total,
            limit,
            offset,
            has_more: offset + limit < total,
        },
    }))
}

/// Get a datasource by ID
#[utoipa::path(
    get,
    path = "/api/cms/datasources/{id}",
    tag = "CMS Datasources",
    params(
        ("id" = Uuid, Path, description = "Datasource UUID")
    ),
    responses(
        (status = 200, description = "Datasource details", body = CmsDatasource),
        (status = 404, description = "Datasource not found")
    ),
    security(("bearer_auth" = []))
)]
async fn get_datasource(
    State(state): State<AppState>,
    user: User,
    Path(id): Path<Uuid>,
) -> ApiResult<CmsDatasource> {
    require_cms_editor(&user)?;

    let datasource: CmsDatasource = sqlx::query_as(
        r#"
        SELECT id, name, slug, description, icon, color, entry_count,
               is_system, is_locked, created_at, updated_at,
               created_by, updated_by, deleted_at
        FROM cms_datasources
        WHERE id = $1 AND deleted_at IS NULL
        "#,
    )
    .bind(id)
    .fetch_optional(&state.db.pool)
    .await
    .map_err(|e: sqlx::Error| ApiError::new(StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
    .ok_or_else(|| {
        ApiError::new(StatusCode::NOT_FOUND, "Datasource not found").with_code("NOT_FOUND")
    })?;

    Ok(Json(datasource))
}

/// Get a datasource by slug
#[utoipa::path(
    get,
    path = "/api/cms/datasources/slug/{slug}",
    tag = "CMS Datasources",
    params(
        ("slug" = String, Path, description = "Datasource slug")
    ),
    responses(
        (status = 200, description = "Datasource details", body = CmsDatasource),
        (status = 404, description = "Datasource not found")
    ),
    security(("bearer_auth" = []))
)]
async fn get_datasource_by_slug(
    State(state): State<AppState>,
    user: User,
    Path(slug): Path<String>,
) -> ApiResult<CmsDatasource> {
    require_cms_editor(&user)?;

    let datasource: CmsDatasource = sqlx::query_as(
        r#"
        SELECT id, name, slug, description, icon, color, entry_count,
               is_system, is_locked, created_at, updated_at,
               created_by, updated_by, deleted_at
        FROM cms_datasources
        WHERE slug = $1 AND deleted_at IS NULL
        "#,
    )
    .bind(&slug)
    .fetch_optional(&state.db.pool)
    .await
    .map_err(|e: sqlx::Error| ApiError::new(StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
    .ok_or_else(|| {
        ApiError::new(StatusCode::NOT_FOUND, "Datasource not found").with_code("NOT_FOUND")
    })?;

    Ok(Json(datasource))
}

/// Create a new datasource
#[utoipa::path(
    post,
    path = "/api/cms/datasources",
    tag = "CMS Datasources",
    request_body = CreateDatasourceRequest,
    responses(
        (status = 201, description = "Datasource created", body = CmsDatasource),
        (status = 400, description = "Validation error"),
        (status = 409, description = "Slug already exists")
    ),
    security(("bearer_auth" = []))
)]
async fn create_datasource(
    State(state): State<AppState>,
    user: User,
    Json(request): Json<CreateDatasourceRequest>,
) -> Result<(StatusCode, Json<CmsDatasource>), ApiError> {
    require_cms_editor(&user)?;

    // Validate
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
    let datasource: CmsDatasource = sqlx::query_as(
        r#"
        INSERT INTO cms_datasources (id, name, slug, description, icon, color, created_by)
        VALUES ($1, $2, $3, $4, $5, $6, $7)
        RETURNING id, name, slug, description, icon, color, entry_count,
                  is_system, is_locked, created_at, updated_at,
                  created_by, updated_by, deleted_at
        "#,
    )
    .bind(new_id)
    .bind(request.name.trim())
    .bind(&slug)
    .bind(&request.description)
    .bind(&request.icon)
    .bind(&request.color)
    .bind(cms_user_id)
    .fetch_one(&state.db.pool)
    .await
    .map_err(|e: sqlx::Error| ApiError::new(StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    tracing::info!(
        "Datasource created: {} ({})",
        datasource.name,
        datasource.id
    );

    Ok((StatusCode::CREATED, Json(datasource)))
}

/// Update a datasource
#[utoipa::path(
    put,
    path = "/api/cms/datasources/{id}",
    tag = "CMS Datasources",
    params(
        ("id" = Uuid, Path, description = "Datasource UUID")
    ),
    request_body = UpdateDatasourceRequest,
    responses(
        (status = 200, description = "Datasource updated", body = CmsDatasource),
        (status = 404, description = "Datasource not found"),
        (status = 423, description = "Datasource is locked")
    ),
    security(("bearer_auth" = []))
)]
async fn update_datasource(
    State(state): State<AppState>,
    user: User,
    Path(id): Path<Uuid>,
    Json(request): Json<UpdateDatasourceRequest>,
) -> ApiResult<CmsDatasource> {
    require_cms_editor(&user)?;

    // Check if exists and is not locked/system
    #[derive(Debug, sqlx::FromRow)]
    struct ExistingDatasource {
        is_locked: bool,
        is_system: bool,
    }
    let existing: ExistingDatasource = sqlx::query_as(
        "SELECT is_locked, is_system FROM cms_datasources WHERE id = $1 AND deleted_at IS NULL",
    )
    .bind(id)
    .fetch_optional(&state.db.pool)
    .await
    .map_err(|e: sqlx::Error| ApiError::new(StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
    .ok_or_else(|| {
        ApiError::new(StatusCode::NOT_FOUND, "Datasource not found").with_code("NOT_FOUND")
    })?;

    // Only admins can update locked or system datasources
    if existing.is_locked || existing.is_system {
        require_cms_admin(&user)?;
    }

    // Handle slug update
    let final_slug = if let Some(ref new_slug) = request.slug {
        Some(ensure_unique_slug(&state.db.pool, new_slug, Some(id)).await?)
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

    let datasource: CmsDatasource = sqlx::query_as(
        r#"
        UPDATE cms_datasources
        SET name = COALESCE($2, name),
            slug = COALESCE($3, slug),
            description = COALESCE($4, description),
            icon = COALESCE($5, icon),
            color = COALESCE($6, color),
            is_locked = COALESCE($7, is_locked),
            updated_at = NOW(),
            updated_by = $8
        WHERE id = $1 AND deleted_at IS NULL
        RETURNING id, name, slug, description, icon, color, entry_count,
                  is_system, is_locked, created_at, updated_at,
                  created_by, updated_by, deleted_at
        "#,
    )
    .bind(id)
    .bind(&request.name)
    .bind(&final_slug)
    .bind(&request.description)
    .bind(&request.icon)
    .bind(&request.color)
    .bind(request.is_locked)
    .bind(cms_user_id)
    .fetch_one(&state.db.pool)
    .await
    .map_err(|e: sqlx::Error| ApiError::new(StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    tracing::info!(
        "Datasource updated: {} ({})",
        datasource.name,
        datasource.id
    );

    Ok(Json(datasource))
}

/// Delete a datasource (soft delete)
#[utoipa::path(
    delete,
    path = "/api/cms/datasources/{id}",
    tag = "CMS Datasources",
    params(
        ("id" = Uuid, Path, description = "Datasource UUID")
    ),
    responses(
        (status = 200, description = "Datasource deleted"),
        (status = 404, description = "Datasource not found"),
        (status = 423, description = "Cannot delete system datasource")
    ),
    security(("bearer_auth" = []))
)]
async fn delete_datasource(
    State(state): State<AppState>,
    user: User,
    Path(id): Path<Uuid>,
) -> ApiResultEmpty {
    require_cms_admin(&user)?;

    // Check if exists and is not system
    #[derive(Debug, sqlx::FromRow)]
    struct DatasourceCheck {
        is_system: bool,
        name: String,
    }
    let existing: DatasourceCheck = sqlx::query_as(
        "SELECT is_system, name FROM cms_datasources WHERE id = $1 AND deleted_at IS NULL",
    )
    .bind(id)
    .fetch_optional(&state.db.pool)
    .await
    .map_err(|e: sqlx::Error| ApiError::new(StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
    .ok_or_else(|| {
        ApiError::new(StatusCode::NOT_FOUND, "Datasource not found").with_code("NOT_FOUND")
    })?;

    if existing.is_system {
        return Err(
            ApiError::new(StatusCode::LOCKED, "Cannot delete a system datasource")
                .with_code("SYSTEM_DATASOURCE"),
        );
    }

    // Soft delete
    sqlx::query("UPDATE cms_datasources SET deleted_at = NOW() WHERE id = $1")
        .bind(id)
        .execute(&state.db.pool)
        .await
        .map_err(|e: sqlx::Error| {
            ApiError::new(StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
        })?;

    tracing::info!("Datasource deleted: {} ({})", existing.name, id);

    Ok(Json(json!({
        "message": "Datasource deleted successfully",
        "id": id
    })))
}

/// Duplicate a datasource
#[utoipa::path(
    post,
    path = "/api/cms/datasources/{id}/duplicate",
    tag = "CMS Datasources",
    params(
        ("id" = Uuid, Path, description = "Datasource UUID to duplicate")
    ),
    responses(
        (status = 201, description = "Datasource duplicated", body = CmsDatasource),
        (status = 404, description = "Datasource not found")
    ),
    security(("bearer_auth" = []))
)]
async fn duplicate_datasource(
    State(state): State<AppState>,
    user: User,
    Path(id): Path<Uuid>,
) -> Result<(StatusCode, Json<CmsDatasource>), ApiError> {
    require_cms_editor(&user)?;

    // Fetch original
    let original: CmsDatasource = sqlx::query_as(
        r#"
        SELECT id, name, slug, description, icon, color, entry_count,
               is_system, is_locked, created_at, updated_at,
               created_by, updated_by, deleted_at
        FROM cms_datasources
        WHERE id = $1 AND deleted_at IS NULL
        "#,
    )
    .bind(id)
    .fetch_optional(&state.db.pool)
    .await
    .map_err(|e: sqlx::Error| ApiError::new(StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
    .ok_or_else(|| {
        ApiError::new(StatusCode::NOT_FOUND, "Datasource not found").with_code("NOT_FOUND")
    })?;

    // Generate new name and slug
    let new_name = format!("{} (Copy)", original.name);
    let base_slug = generate_slug(&new_name);
    let new_slug = ensure_unique_slug(&state.db.pool, &base_slug, None).await?;

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

    // Create duplicated datasource
    let duplicated: CmsDatasource = sqlx::query_as(
        r#"
        INSERT INTO cms_datasources (id, name, slug, description, icon, color, is_system, is_locked, created_by)
        VALUES ($1, $2, $3, $4, $5, $6, false, false, $7)
        RETURNING id, name, slug, description, icon, color, entry_count,
                  is_system, is_locked, created_at, updated_at,
                  created_by, updated_by, deleted_at
        "#,
    )
    .bind(dup_id)
    .bind(&new_name)
    .bind(&new_slug)
    .bind(&original.description)
    .bind(&original.icon)
    .bind(&original.color)
    .bind(cms_user_id)
    .fetch_one(&state.db.pool)
    .await
    .map_err(|e: sqlx::Error| ApiError::new(StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    // Copy all entries
    sqlx::query(
        r#"
        INSERT INTO cms_datasource_entries (datasource_id, name, value, dimension, sort_order, metadata)
        SELECT $2, name, value, dimension, sort_order, metadata
        FROM cms_datasource_entries
        WHERE datasource_id = $1
        "#,
    )
    .bind(original.id)
    .bind(dup_id)
    .execute(&state.db.pool)
    .await
    .map_err(|e: sqlx::Error| ApiError::new(StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    tracing::info!(
        "Datasource duplicated: {} -> {} ({})",
        original.name,
        duplicated.name,
        duplicated.id
    );

    Ok((StatusCode::CREATED, Json(duplicated)))
}

// ============================================================================
// ENTRY ROUTE HANDLERS
// ============================================================================

/// List entries for a datasource
#[utoipa::path(
    get,
    path = "/api/cms/datasources/{datasource_id}/entries",
    tag = "CMS Datasources",
    params(
        ("datasource_id" = Uuid, Path, description = "Datasource UUID"),
        ListEntriesQuery
    ),
    responses(
        (status = 200, description = "List of entries", body = PaginatedEntriesResponse),
        (status = 404, description = "Datasource not found")
    ),
    security(("bearer_auth" = []))
)]
async fn list_entries(
    State(state): State<AppState>,
    user: User,
    Path(datasource_id): Path<Uuid>,
    Query(query): Query<ListEntriesQuery>,
) -> ApiResult<PaginatedEntriesResponse> {
    require_cms_editor(&user)?;

    let limit = query.limit.unwrap_or(100).min(1000);
    let offset = query.offset.unwrap_or(0);
    let dimension = query
        .dimension
        .clone()
        .unwrap_or_else(|| "default".to_string());
    let search_pattern = query.search.as_ref().map(|s| format!("%{}%", s));

    // Verify datasource exists
    let exists: (bool,) = sqlx::query_as(
        "SELECT EXISTS(SELECT 1 FROM cms_datasources WHERE id = $1 AND deleted_at IS NULL)",
    )
    .bind(datasource_id)
    .fetch_one(&state.db.pool)
    .await
    .map_err(|e: sqlx::Error| ApiError::new(StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    if !exists.0 {
        return Err(
            ApiError::new(StatusCode::NOT_FOUND, "Datasource not found").with_code("NOT_FOUND")
        );
    }

    // Count total
    let count_result: (i64,) = sqlx::query_as(
        r#"
        SELECT COUNT(*)
        FROM cms_datasource_entries
        WHERE datasource_id = $1
          AND dimension = $2
          AND ($3::text IS NULL OR name ILIKE $3 OR value ILIKE $3)
        "#,
    )
    .bind(datasource_id)
    .bind(&dimension)
    .bind(&search_pattern)
    .fetch_one(&state.db.pool)
    .await
    .map_err(|e: sqlx::Error| ApiError::new(StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let total = count_result.0;

    // Fetch entries
    let entries: Vec<CmsDatasourceEntrySummary> = sqlx::query_as(
        r#"
        SELECT id, name, value, dimension, sort_order, metadata
        FROM cms_datasource_entries
        WHERE datasource_id = $1
          AND dimension = $2
          AND ($3::text IS NULL OR name ILIKE $3 OR value ILIKE $3)
        ORDER BY sort_order, name
        LIMIT $4 OFFSET $5
        "#,
    )
    .bind(datasource_id)
    .bind(&dimension)
    .bind(&search_pattern)
    .bind(limit)
    .bind(offset)
    .fetch_all(&state.db.pool)
    .await
    .map_err(|e: sqlx::Error| ApiError::new(StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    // Get available dimensions
    let dimensions: Vec<String> = sqlx::query_scalar(
        r#"
        SELECT DISTINCT dimension
        FROM cms_datasource_entries
        WHERE datasource_id = $1
        ORDER BY dimension
        "#,
    )
    .bind(datasource_id)
    .fetch_all(&state.db.pool)
    .await
    .map_err(|e: sqlx::Error| ApiError::new(StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(PaginatedEntriesResponse {
        data: entries,
        meta: PaginationMeta {
            total,
            limit,
            offset,
            has_more: offset + limit < total,
        },
        dimensions,
    }))
}

/// Create a new entry
#[utoipa::path(
    post,
    path = "/api/cms/datasources/{datasource_id}/entries",
    tag = "CMS Datasources",
    params(
        ("datasource_id" = Uuid, Path, description = "Datasource UUID")
    ),
    request_body = CreateEntryRequest,
    responses(
        (status = 201, description = "Entry created", body = CmsDatasourceEntry),
        (status = 400, description = "Validation error"),
        (status = 404, description = "Datasource not found")
    ),
    security(("bearer_auth" = []))
)]
async fn create_entry(
    State(state): State<AppState>,
    user: User,
    Path(datasource_id): Path<Uuid>,
    Json(request): Json<CreateEntryRequest>,
) -> Result<(StatusCode, Json<CmsDatasourceEntry>), ApiError> {
    require_cms_editor(&user)?;

    // Validate
    if request.name.trim().is_empty() {
        return Err(ApiError::new(StatusCode::BAD_REQUEST, "Name is required")
            .with_code("VALIDATION_ERROR"));
    }
    if request.value.trim().is_empty() {
        return Err(ApiError::new(StatusCode::BAD_REQUEST, "Value is required")
            .with_code("VALIDATION_ERROR"));
    }

    // Verify datasource exists
    let exists: (bool,) = sqlx::query_as(
        "SELECT EXISTS(SELECT 1 FROM cms_datasources WHERE id = $1 AND deleted_at IS NULL)",
    )
    .bind(datasource_id)
    .fetch_one(&state.db.pool)
    .await
    .map_err(|e: sqlx::Error| ApiError::new(StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    if !exists.0 {
        return Err(
            ApiError::new(StatusCode::NOT_FOUND, "Datasource not found").with_code("NOT_FOUND")
        );
    }

    let dimension = request
        .dimension
        .clone()
        .unwrap_or_else(|| "default".to_string());

    // Get next sort order
    let next_sort: Option<i32> = sqlx::query_scalar(
        "SELECT COALESCE(MAX(sort_order), -1) + 1 FROM cms_datasource_entries WHERE datasource_id = $1 AND dimension = $2",
    )
    .bind(datasource_id)
    .bind(&dimension)
    .fetch_one(&state.db.pool)
    .await
    .map_err(|e: sqlx::Error| ApiError::new(StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let new_id = Uuid::new_v4();
    let entry: CmsDatasourceEntry = sqlx::query_as(
        r#"
        INSERT INTO cms_datasource_entries (id, datasource_id, name, value, dimension, sort_order, metadata)
        VALUES ($1, $2, $3, $4, $5, $6, $7)
        RETURNING id, datasource_id, name, value, dimension, sort_order, metadata, created_at, updated_at
        "#,
    )
    .bind(new_id)
    .bind(datasource_id)
    .bind(request.name.trim())
    .bind(request.value.trim())
    .bind(&dimension)
    .bind(next_sort.unwrap_or(0))
    .bind(&request.metadata)
    .fetch_one(&state.db.pool)
    .await
    .map_err(|e: sqlx::Error| {
        if e.to_string().contains("cms_datasource_entries_unique") {
            ApiError::new(StatusCode::CONFLICT, "Entry with this value already exists in this dimension")
                .with_code("DUPLICATE_ENTRY")
        } else {
            ApiError::new(StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
        }
    })?;

    tracing::info!(
        "Entry created: {} = {} in datasource {}",
        entry.name,
        entry.value,
        datasource_id
    );

    Ok((StatusCode::CREATED, Json(entry)))
}

/// Bulk create entries
#[utoipa::path(
    post,
    path = "/api/cms/datasources/{datasource_id}/entries/bulk",
    tag = "CMS Datasources",
    params(
        ("datasource_id" = Uuid, Path, description = "Datasource UUID")
    ),
    request_body = BulkCreateEntriesRequest,
    responses(
        (status = 201, description = "Entries created"),
        (status = 404, description = "Datasource not found")
    ),
    security(("bearer_auth" = []))
)]
async fn bulk_create_entries(
    State(state): State<AppState>,
    user: User,
    Path(datasource_id): Path<Uuid>,
    Json(request): Json<BulkCreateEntriesRequest>,
) -> ApiResultEmpty {
    require_cms_editor(&user)?;

    // Verify datasource exists
    let exists: (bool,) = sqlx::query_as(
        "SELECT EXISTS(SELECT 1 FROM cms_datasources WHERE id = $1 AND deleted_at IS NULL)",
    )
    .bind(datasource_id)
    .fetch_one(&state.db.pool)
    .await
    .map_err(|e: sqlx::Error| ApiError::new(StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    if !exists.0 {
        return Err(
            ApiError::new(StatusCode::NOT_FOUND, "Datasource not found").with_code("NOT_FOUND")
        );
    }

    let mut created_count = 0;
    let mut skipped_count = 0;

    for (i, entry) in request.entries.iter().enumerate() {
        if entry.name.trim().is_empty() || entry.value.trim().is_empty() {
            skipped_count += 1;
            continue;
        }

        let dimension = entry
            .dimension
            .clone()
            .unwrap_or_else(|| "default".to_string());

        let result = sqlx::query(
            r#"
            INSERT INTO cms_datasource_entries (id, datasource_id, name, value, dimension, sort_order, metadata)
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            ON CONFLICT (datasource_id, value, dimension) DO NOTHING
            "#,
        )
        .bind(Uuid::new_v4())
        .bind(datasource_id)
        .bind(entry.name.trim())
        .bind(entry.value.trim())
        .bind(&dimension)
        .bind(i as i32)
        .bind(&entry.metadata)
        .execute(&state.db.pool)
        .await;

        match result {
            Ok(r) if r.rows_affected() > 0 => created_count += 1,
            Ok(_) => skipped_count += 1,
            Err(_) => skipped_count += 1,
        }
    }

    tracing::info!(
        "Bulk created {} entries in datasource {} ({} skipped)",
        created_count,
        datasource_id,
        skipped_count
    );

    Ok(Json(json!({
        "message": "Entries created successfully",
        "created": created_count,
        "skipped": skipped_count
    })))
}

/// Update an entry
#[utoipa::path(
    put,
    path = "/api/cms/datasources/{datasource_id}/entries/{entry_id}",
    tag = "CMS Datasources",
    params(
        ("datasource_id" = Uuid, Path, description = "Datasource UUID"),
        ("entry_id" = Uuid, Path, description = "Entry UUID")
    ),
    request_body = UpdateEntryRequest,
    responses(
        (status = 200, description = "Entry updated", body = CmsDatasourceEntry),
        (status = 404, description = "Entry not found")
    ),
    security(("bearer_auth" = []))
)]
async fn update_entry(
    State(state): State<AppState>,
    user: User,
    Path((datasource_id, entry_id)): Path<(Uuid, Uuid)>,
    Json(request): Json<UpdateEntryRequest>,
) -> ApiResult<CmsDatasourceEntry> {
    require_cms_editor(&user)?;

    let entry: CmsDatasourceEntry = sqlx::query_as(
        r#"
        UPDATE cms_datasource_entries
        SET name = COALESCE($3, name),
            value = COALESCE($4, value),
            dimension = COALESCE($5, dimension),
            sort_order = COALESCE($6, sort_order),
            metadata = COALESCE($7, metadata),
            updated_at = NOW()
        WHERE id = $1 AND datasource_id = $2
        RETURNING id, datasource_id, name, value, dimension, sort_order, metadata, created_at, updated_at
        "#,
    )
    .bind(entry_id)
    .bind(datasource_id)
    .bind(&request.name)
    .bind(&request.value)
    .bind(&request.dimension)
    .bind(request.sort_order)
    .bind(&request.metadata)
    .fetch_optional(&state.db.pool)
    .await
    .map_err(|e: sqlx::Error| ApiError::new(StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
    .ok_or_else(|| {
        ApiError::new(StatusCode::NOT_FOUND, "Entry not found").with_code("NOT_FOUND")
    })?;

    tracing::info!("Entry updated: {} ({})", entry.name, entry.id);

    Ok(Json(entry))
}

/// Delete an entry
#[utoipa::path(
    delete,
    path = "/api/cms/datasources/{datasource_id}/entries/{entry_id}",
    tag = "CMS Datasources",
    params(
        ("datasource_id" = Uuid, Path, description = "Datasource UUID"),
        ("entry_id" = Uuid, Path, description = "Entry UUID")
    ),
    responses(
        (status = 200, description = "Entry deleted"),
        (status = 404, description = "Entry not found")
    ),
    security(("bearer_auth" = []))
)]
async fn delete_entry(
    State(state): State<AppState>,
    user: User,
    Path((datasource_id, entry_id)): Path<(Uuid, Uuid)>,
) -> ApiResultEmpty {
    require_cms_editor(&user)?;

    let result =
        sqlx::query("DELETE FROM cms_datasource_entries WHERE id = $1 AND datasource_id = $2")
            .bind(entry_id)
            .bind(datasource_id)
            .execute(&state.db.pool)
            .await
            .map_err(|e: sqlx::Error| {
                ApiError::new(StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
            })?;

    if result.rows_affected() == 0 {
        return Err(ApiError::new(StatusCode::NOT_FOUND, "Entry not found").with_code("NOT_FOUND"));
    }

    tracing::info!(
        "Entry deleted: {} from datasource {}",
        entry_id,
        datasource_id
    );

    Ok(Json(json!({
        "message": "Entry deleted successfully",
        "id": entry_id
    })))
}

/// Reorder entries
#[utoipa::path(
    put,
    path = "/api/cms/datasources/{datasource_id}/entries/reorder",
    tag = "CMS Datasources",
    params(
        ("datasource_id" = Uuid, Path, description = "Datasource UUID")
    ),
    request_body = ReorderEntriesRequest,
    responses(
        (status = 200, description = "Entries reordered"),
        (status = 404, description = "Datasource not found")
    ),
    security(("bearer_auth" = []))
)]
async fn reorder_entries(
    State(state): State<AppState>,
    user: User,
    Path(datasource_id): Path<Uuid>,
    Json(request): Json<ReorderEntriesRequest>,
) -> ApiResultEmpty {
    require_cms_editor(&user)?;

    // Verify datasource exists
    let exists: (bool,) = sqlx::query_as(
        "SELECT EXISTS(SELECT 1 FROM cms_datasources WHERE id = $1 AND deleted_at IS NULL)",
    )
    .bind(datasource_id)
    .fetch_one(&state.db.pool)
    .await
    .map_err(|e: sqlx::Error| ApiError::new(StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    if !exists.0 {
        return Err(
            ApiError::new(StatusCode::NOT_FOUND, "Datasource not found").with_code("NOT_FOUND")
        );
    }

    // Update sort orders
    for (i, entry_id) in request.entry_ids.iter().enumerate() {
        sqlx::query(
            "UPDATE cms_datasource_entries SET sort_order = $1 WHERE id = $2 AND datasource_id = $3",
        )
        .bind(i as i32)
        .bind(entry_id)
        .bind(datasource_id)
        .execute(&state.db.pool)
        .await
        .map_err(|e: sqlx::Error| ApiError::new(StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    }

    tracing::info!("Entries reordered in datasource {}", datasource_id);

    Ok(Json(json!({
        "message": "Entries reordered successfully"
    })))
}

// ============================================================================
// IMPORT/EXPORT HANDLERS
// ============================================================================

/// Export entries as CSV
#[utoipa::path(
    get,
    path = "/api/cms/datasources/{datasource_id}/export",
    tag = "CMS Datasources",
    params(
        ("datasource_id" = Uuid, Path, description = "Datasource UUID")
    ),
    responses(
        (status = 200, description = "CSV file"),
        (status = 404, description = "Datasource not found")
    ),
    security(("bearer_auth" = []))
)]
async fn export_entries_csv(
    State(state): State<AppState>,
    user: User,
    Path(datasource_id): Path<Uuid>,
) -> Result<
    (
        StatusCode,
        [(axum::http::header::HeaderName, String); 2],
        String,
    ),
    ApiError,
> {
    require_cms_editor(&user)?;

    // Get datasource info for filename
    #[derive(Debug, sqlx::FromRow)]
    struct DatasourceSlug {
        slug: String,
    }
    let datasource: DatasourceSlug =
        sqlx::query_as("SELECT slug FROM cms_datasources WHERE id = $1 AND deleted_at IS NULL")
            .bind(datasource_id)
            .fetch_optional(&state.db.pool)
            .await
            .map_err(|e: sqlx::Error| {
                ApiError::new(StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
            })?
            .ok_or_else(|| {
                ApiError::new(StatusCode::NOT_FOUND, "Datasource not found").with_code("NOT_FOUND")
            })?;

    // Fetch all entries
    let entries: Vec<CsvEntryRow> = sqlx::query_as(
        r#"
        SELECT name, value, dimension
        FROM cms_datasource_entries
        WHERE datasource_id = $1
        ORDER BY dimension, sort_order, name
        "#,
    )
    .bind(datasource_id)
    .fetch_all(&state.db.pool)
    .await
    .map_err(|e: sqlx::Error| ApiError::new(StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    // Generate CSV
    let mut csv_content = String::from("name,value,dimension\n");
    for entry in entries {
        csv_content.push_str(&format!(
            "\"{}\",\"{}\",\"{}\"\n",
            entry.name.replace('"', "\"\""),
            entry.value.replace('"', "\"\""),
            entry.dimension.replace('"', "\"\"")
        ));
    }

    let filename = format!("{}-entries.csv", datasource.slug);

    Ok((
        StatusCode::OK,
        [
            (axum::http::header::CONTENT_TYPE, "text/csv".to_string()),
            (
                axum::http::header::CONTENT_DISPOSITION,
                format!("attachment; filename=\"{}\"", filename),
            ),
        ],
        csv_content,
    ))
}

/// Import entries from CSV
#[utoipa::path(
    post,
    path = "/api/cms/datasources/{datasource_id}/import",
    tag = "CMS Datasources",
    params(
        ("datasource_id" = Uuid, Path, description = "Datasource UUID")
    ),
    responses(
        (status = 200, description = "Import results"),
        (status = 400, description = "Invalid CSV"),
        (status = 404, description = "Datasource not found")
    ),
    security(("bearer_auth" = []))
)]
async fn import_entries_csv(
    State(state): State<AppState>,
    user: User,
    Path(datasource_id): Path<Uuid>,
    mut multipart: Multipart,
) -> ApiResultEmpty {
    require_cms_editor(&user)?;

    // Verify datasource exists
    let exists: (bool,) = sqlx::query_as(
        "SELECT EXISTS(SELECT 1 FROM cms_datasources WHERE id = $1 AND deleted_at IS NULL)",
    )
    .bind(datasource_id)
    .fetch_one(&state.db.pool)
    .await
    .map_err(|e: sqlx::Error| ApiError::new(StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    if !exists.0 {
        return Err(
            ApiError::new(StatusCode::NOT_FOUND, "Datasource not found").with_code("NOT_FOUND")
        );
    }

    // Get the file from multipart
    let mut csv_content = String::new();
    while let Some(field) = multipart.next_field().await.map_err(|e| {
        ApiError::new(
            StatusCode::BAD_REQUEST,
            format!("Failed to read file: {}", e),
        )
    })? {
        if field.name() == Some("file") {
            let bytes = field.bytes().await.map_err(|e| {
                ApiError::new(
                    StatusCode::BAD_REQUEST,
                    format!("Failed to read file: {}", e),
                )
            })?;
            csv_content = String::from_utf8(bytes.to_vec()).map_err(|e| {
                ApiError::new(StatusCode::BAD_REQUEST, format!("Invalid UTF-8: {}", e))
            })?;
        }
    }

    if csv_content.is_empty() {
        return Err(ApiError::new(StatusCode::BAD_REQUEST, "No file uploaded").with_code("NO_FILE"));
    }

    // Parse CSV
    let mut reader = csv::ReaderBuilder::new()
        .has_headers(true)
        .flexible(true)
        .from_reader(csv_content.as_bytes());

    let mut created_count = 0;
    let mut skipped_count = 0;
    let mut error_count = 0;

    // Get current max sort order
    let base_sort: Option<i32> = sqlx::query_scalar(
        "SELECT COALESCE(MAX(sort_order), -1) + 1 FROM cms_datasource_entries WHERE datasource_id = $1",
    )
    .bind(datasource_id)
    .fetch_one(&state.db.pool)
    .await
    .map_err(|e: sqlx::Error| ApiError::new(StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    let base_sort = base_sort.unwrap_or(0);

    for (i, result) in reader.records().enumerate() {
        match result {
            Ok(record) => {
                let name = record.get(0).unwrap_or("").trim();
                let value = record.get(1).unwrap_or("").trim();
                let dimension = record.get(2).unwrap_or("default").trim();

                if name.is_empty() || value.is_empty() {
                    skipped_count += 1;
                    continue;
                }

                let result = sqlx::query(
                    r#"
                    INSERT INTO cms_datasource_entries (id, datasource_id, name, value, dimension, sort_order)
                    VALUES ($1, $2, $3, $4, $5, $6)
                    ON CONFLICT (datasource_id, value, dimension) DO UPDATE SET name = EXCLUDED.name
                    "#,
                )
                .bind(Uuid::new_v4())
                .bind(datasource_id)
                .bind(name)
                .bind(value)
                .bind(if dimension.is_empty() { "default" } else { dimension })
                .bind(base_sort + i as i32)
                .execute(&state.db.pool)
                .await;

                match result {
                    Ok(_) => created_count += 1,
                    Err(_) => error_count += 1,
                }
            }
            Err(_) => {
                error_count += 1;
            }
        }
    }

    tracing::info!(
        "CSV import completed for datasource {}: {} created, {} skipped, {} errors",
        datasource_id,
        created_count,
        skipped_count,
        error_count
    );

    Ok(Json(json!({
        "message": "Import completed",
        "created": created_count,
        "skipped": skipped_count,
        "errors": error_count
    })))
}

// ============================================================================
// PUBLIC API HANDLERS
// ============================================================================

/// Public endpoint to get entries by datasource slug
async fn public_get_entries(
    State(state): State<AppState>,
    Path(slug): Path<String>,
    Query(query): Query<ListEntriesQuery>,
) -> ApiResult<PublicDatasourceResponse> {
    let dimension = query
        .dimension
        .clone()
        .unwrap_or_else(|| "default".to_string());

    // Get datasource name
    #[derive(Debug, sqlx::FromRow)]
    struct DatasourceName {
        name: String,
    }
    let datasource: DatasourceName =
        sqlx::query_as("SELECT name FROM cms_datasources WHERE slug = $1 AND deleted_at IS NULL")
            .bind(&slug)
            .fetch_optional(&state.db.pool)
            .await
            .map_err(|e: sqlx::Error| {
                ApiError::new(StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
            })?
            .ok_or_else(|| {
                ApiError::new(StatusCode::NOT_FOUND, "Datasource not found").with_code("NOT_FOUND")
            })?;

    // Fetch entries
    let entries: Vec<PublicEntryResponse> = sqlx::query_as(
        r#"
        SELECT e.name, e.value
        FROM cms_datasource_entries e
        JOIN cms_datasources d ON e.datasource_id = d.id
        WHERE d.slug = $1 AND d.deleted_at IS NULL AND e.dimension = $2
        ORDER BY e.sort_order, e.name
        "#,
    )
    .bind(&slug)
    .bind(&dimension)
    .fetch_all(&state.db.pool)
    .await
    .map_err(|e: sqlx::Error| ApiError::new(StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(PublicDatasourceResponse {
        slug,
        name: datasource.name,
        entries,
    }))
}

// ============================================================================
// ROUTER
// ============================================================================

/// Admin router for datasources (requires authentication)
pub fn admin_router() -> Router<AppState> {
    Router::new()
        // Datasource CRUD
        .route("/", get(list_datasources).post(create_datasource))
        .route("/slug/{slug}", get(get_datasource_by_slug))
        .route(
            "/{id}",
            get(get_datasource)
                .put(update_datasource)
                .delete(delete_datasource),
        )
        .route("/{id}/duplicate", post(duplicate_datasource))
        // Entry management
        .route(
            "/{datasource_id}/entries",
            get(list_entries).post(create_entry),
        )
        .route("/{datasource_id}/entries/bulk", post(bulk_create_entries))
        .route("/{datasource_id}/entries/reorder", put(reorder_entries))
        .route(
            "/{datasource_id}/entries/{entry_id}",
            put(update_entry).delete(delete_entry),
        )
        // Import/Export
        .route("/{datasource_id}/export", get(export_entries_csv))
        .route("/{datasource_id}/import", post(import_entries_csv))
}

/// Public router for read-only datasource access (no auth required)
pub fn public_router() -> Router<AppState> {
    Router::new().route("/{slug}", get(public_get_entries))
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_slug() {
        assert_eq!(generate_slug("US States"), "us-states");
        assert_eq!(generate_slug("Color Palettes"), "color-palettes");
        assert_eq!(generate_slug("  Multiple   Spaces  "), "multiple-spaces");
    }
}
