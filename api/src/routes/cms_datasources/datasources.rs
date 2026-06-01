//! Datasource container CRUD handlers.
//!
//! Endpoints: list, get-by-id, get-by-slug, create, update, delete, duplicate.

use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Json,
};
use serde_json::json;
use uuid::Uuid;

use crate::{models::User, utils::errors::ApiError, AppState};

use super::helpers::{ensure_unique_slug, generate_slug, require_cms_admin, require_cms_editor};
use super::types::{
    ApiResult, ApiResultEmpty, CmsDatasource, CmsDatasourceSummary, CreateDatasourceRequest,
    ListDatasourcesQuery, PaginatedDatasourcesResponse, PaginationMeta, UpdateDatasourceRequest,
};

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
pub(super) async fn list_datasources(
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

    let search_pattern = query.search.as_ref().map(|s| format!("%{s}%"));

    // Count total
    let count_result: (i64,) = sqlx::query_as(
        r"
        SELECT COUNT(*)
        FROM cms_datasources
        WHERE deleted_at IS NULL
          AND ($1::text IS NULL OR name ILIKE $1 OR description ILIKE $1)
          AND ($2::boolean IS NULL OR is_system = $2)
        ",
    )
    .bind(&search_pattern)
    .bind(query.is_system)
    .fetch_one(&state.db.pool)
    .await
    .map_err(|e: sqlx::Error| ApiError::new(StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let total = count_result.0;

    // Fetch datasources
    let datasources: Vec<CmsDatasourceSummary> = sqlx::query_as(&format!(
        r"
        SELECT id, name, slug, description, icon, color, entry_count,
               is_system, is_locked, created_at, updated_at
        FROM cms_datasources
        WHERE deleted_at IS NULL
          AND ($1::text IS NULL OR name ILIKE $1 OR description ILIKE $1)
          AND ($2::boolean IS NULL OR is_system = $2)
        ORDER BY is_system DESC, {sort_column} {sort_order}
        LIMIT $3 OFFSET $4
        "
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
pub(super) async fn get_datasource(
    State(state): State<AppState>,
    user: User,
    Path(id): Path<Uuid>,
) -> ApiResult<CmsDatasource> {
    require_cms_editor(&user)?;

    let datasource: CmsDatasource = sqlx::query_as(
        r"
        SELECT id, name, slug, description, icon, color, entry_count,
               is_system, is_locked, created_at, updated_at,
               created_by, updated_by, deleted_at
        FROM cms_datasources
        WHERE id = $1 AND deleted_at IS NULL
        ",
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
pub(super) async fn get_datasource_by_slug(
    State(state): State<AppState>,
    user: User,
    Path(slug): Path<String>,
) -> ApiResult<CmsDatasource> {
    require_cms_editor(&user)?;

    let datasource: CmsDatasource = sqlx::query_as(
        r"
        SELECT id, name, slug, description, icon, color, entry_count,
               is_system, is_locked, created_at, updated_at,
               created_by, updated_by, deleted_at
        FROM cms_datasources
        WHERE slug = $1 AND deleted_at IS NULL
        ",
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
pub(super) async fn create_datasource(
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
        r"
        INSERT INTO cms_datasources (id, name, slug, description, icon, color, created_by)
        VALUES ($1, $2, $3, $4, $5, $6, $7)
        RETURNING id, name, slug, description, icon, color, entry_count,
                  is_system, is_locked, created_at, updated_at,
                  created_by, updated_by, deleted_at
        ",
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
pub(super) async fn update_datasource(
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
        r"
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
        ",
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
pub(super) async fn delete_datasource(
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
pub(super) async fn duplicate_datasource(
    State(state): State<AppState>,
    user: User,
    Path(id): Path<Uuid>,
) -> Result<(StatusCode, Json<CmsDatasource>), ApiError> {
    require_cms_editor(&user)?;

    // Fetch original
    let original: CmsDatasource = sqlx::query_as(
        r"
        SELECT id, name, slug, description, icon, color, entry_count,
               is_system, is_locked, created_at, updated_at,
               created_by, updated_by, deleted_at
        FROM cms_datasources
        WHERE id = $1 AND deleted_at IS NULL
        ",
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
        r"
        INSERT INTO cms_datasources (id, name, slug, description, icon, color, is_system, is_locked, created_by)
        VALUES ($1, $2, $3, $4, $5, $6, false, false, $7)
        RETURNING id, name, slug, description, icon, color, entry_count,
                  is_system, is_locked, created_at, updated_at,
                  created_by, updated_by, deleted_at
        ",
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
        r"
        INSERT INTO cms_datasource_entries (datasource_id, name, value, dimension, sort_order, metadata)
        SELECT $2, name, value, dimension, sort_order, metadata
        FROM cms_datasource_entries
        WHERE datasource_id = $1
        ",
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
