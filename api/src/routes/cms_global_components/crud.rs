//! CRUD handlers for global components themselves
//! (list / get / get-by-slug / create / update / delete / duplicate).

use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Json,
};
use serde_json::{json, Value as JsonValue};
use uuid::Uuid;

use crate::{models::User, utils::errors::ApiError, AppState};

use super::helpers::{
    create_version_record, ensure_unique_slug, generate_slug, get_cms_user_id, require_cms_admin,
    require_cms_editor,
};
use super::types::{
    not_found_component, sqlx_err, ApiResult, ApiResultEmpty, CategoryCount,
    CreateGlobalComponentRequest, GlobalComponent, GlobalComponentCategory, GlobalComponentSummary,
    GlobalComponentVersion, GlobalComponentWithMeta, ListGlobalComponentsQuery,
    PaginatedComponentsResponse, PaginationMeta, UpdateGlobalComponentRequest,
};

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
pub(super) async fn list_global_components(
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
    let search_pattern = query.search.as_ref().map(|s| format!("%{s}%"));

    // Count total
    let count_result: (i64,) = sqlx::query_as(
        r"
        SELECT COUNT(*)
        FROM cms_global_components
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
    .map_err(sqlx_err)?;

    let total = count_result.0;

    // Fetch components with safe sort columns
    let components: Vec<GlobalComponentSummary> = sqlx::query_as(sqlx::AssertSqlSafe(format!(
        r"
        SELECT id, name, slug, description, category::text, tags, thumbnail_url,
               usage_count, is_global, is_locked, version, created_at, updated_at
        FROM cms_global_components
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
    .map_err(sqlx_err)?;

    // Get category counts for sidebar filters
    let categories: Vec<CategoryCount> = sqlx::query_as(
        r"
        SELECT category::text as category, COUNT(*) as count
        FROM cms_global_components
        WHERE deleted_at IS NULL
        GROUP BY category
        ORDER BY count DESC
        ",
    )
    .fetch_all(&state.db.pool)
    .await
    .map_err(sqlx_err)?;

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
pub(super) async fn get_global_component(
    State(state): State<AppState>,
    user: User,
    Path(id): Path<Uuid>,
) -> ApiResult<GlobalComponentWithMeta> {
    require_cms_editor(&user)?;

    let component: GlobalComponent = sqlx::query_as(
        r"
        SELECT id, name, slug, description, component_data, category,
               tags, thumbnail_url, usage_count, is_global, is_locked, version,
               deleted_at, created_at, updated_at, created_by, updated_by
        FROM cms_global_components
        WHERE id = $1 AND deleted_at IS NULL
        ",
    )
    .bind(id)
    .fetch_optional(&state.db.pool)
    .await
    .map_err(sqlx_err)?
    .ok_or_else(not_found_component)?;

    // Get creator name
    let created_by_name: Option<String> = if let Some(creator_id) = component.created_by {
        sqlx::query_scalar("SELECT name FROM cms_users WHERE id = $1")
            .bind(creator_id)
            .fetch_optional(&state.db.pool)
            .await
            .map_err(sqlx_err)?
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
            .map_err(sqlx_err)?
            .flatten()
    } else {
        None
    };

    // Get recent versions (last 5)
    let recent_versions: Vec<GlobalComponentVersion> = sqlx::query_as(
        r"
        SELECT v.id, v.component_id, v.version, v.component_data,
               v.change_message, v.created_by, u.name as created_by_name, v.created_at
        FROM cms_global_component_versions v
        LEFT JOIN cms_users u ON u.id = v.created_by
        WHERE v.component_id = $1
        ORDER BY v.version DESC
        LIMIT 5
        ",
    )
    .bind(id)
    .fetch_all(&state.db.pool)
    .await
    .map_err(sqlx_err)?;

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
pub(super) async fn get_global_component_by_slug(
    State(state): State<AppState>,
    user: User,
    Path(slug): Path<String>,
) -> ApiResult<GlobalComponent> {
    require_cms_editor(&user)?;

    let component: GlobalComponent = sqlx::query_as(
        r"
        SELECT id, name, slug, description, component_data, category,
               tags, thumbnail_url, usage_count, is_global, is_locked, version,
               deleted_at, created_at, updated_at, created_by, updated_by
        FROM cms_global_components
        WHERE slug = $1 AND deleted_at IS NULL
        ",
    )
    .bind(&slug)
    .fetch_optional(&state.db.pool)
    .await
    .map_err(sqlx_err)?
    .ok_or_else(not_found_component)?;

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
pub(super) async fn create_global_component(
    State(state): State<AppState>,
    user: User,
    Json(request): Json<CreateGlobalComponentRequest>,
) -> Result<(StatusCode, Json<GlobalComponent>), ApiError> {
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
    let cms_user_id = get_cms_user_id(&state.db.pool, user.id).await;

    let new_id = Uuid::new_v4();
    let component: GlobalComponent = sqlx::query_as(
        r"
        INSERT INTO cms_global_components (
            id, name, slug, description, component_data, category, tags,
            thumbnail_url, usage_count, is_global, is_locked, version,
            created_at, updated_at, created_by, updated_by
        )
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, 0, $9, false, 1, NOW(), NOW(), $10, $10)
        RETURNING id, name, slug, description, component_data, category,
                  tags, thumbnail_url, usage_count, is_global, is_locked, version,
                  deleted_at, created_at, updated_at, created_by, updated_by
        ",
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
    .map_err(sqlx_err)?;

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
pub(super) async fn update_global_component(
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
    .map_err(sqlx_err)?
    .ok_or_else(not_found_component)?;

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
        r"
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
        ",
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
    .map_err(sqlx_err)?;

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
pub(super) async fn delete_global_component(
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
    .map_err(sqlx_err)?
    .ok_or_else(not_found_component)?;

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
        .map_err(sqlx_err)?;

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
pub(super) async fn duplicate_global_component(
    State(state): State<AppState>,
    user: User,
    Path(id): Path<Uuid>,
) -> Result<(StatusCode, Json<GlobalComponent>), ApiError> {
    require_cms_editor(&user)?;

    // Fetch the original component
    let original: GlobalComponent = sqlx::query_as(
        r"
        SELECT id, name, slug, description, component_data, category,
               tags, thumbnail_url, usage_count, is_global, is_locked, version,
               deleted_at, created_at, updated_at, created_by, updated_by
        FROM cms_global_components
        WHERE id = $1 AND deleted_at IS NULL
        ",
    )
    .bind(id)
    .fetch_optional(&state.db.pool)
    .await
    .map_err(sqlx_err)?
    .ok_or_else(not_found_component)?;

    // Generate new name and slug
    let new_name = format!("{} (Copy)", original.name);
    let base_slug = generate_slug(&new_name);
    let new_slug = ensure_unique_slug(&state.db.pool, &base_slug, None).await?;

    // Get CMS user ID
    let cms_user_id = get_cms_user_id(&state.db.pool, user.id).await;

    let dup_id = Uuid::new_v4();
    let duplicated: GlobalComponent = sqlx::query_as(
        r"
        INSERT INTO cms_global_components (
            id, name, slug, description, component_data, category, tags,
            thumbnail_url, usage_count, is_global, is_locked, version,
            created_at, updated_at, created_by, updated_by
        )
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, 0, $9, false, 1, NOW(), NOW(), $10, $10)
        RETURNING id, name, slug, description, component_data, category,
                  tags, thumbnail_url, usage_count, is_global, is_locked, version,
                  deleted_at, created_at, updated_at, created_by, updated_by
        ",
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
    .map_err(sqlx_err)?;

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
