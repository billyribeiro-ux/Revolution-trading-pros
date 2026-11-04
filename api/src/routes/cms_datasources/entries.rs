//! Datasource entry handlers (key-value rows nested under a datasource).
//!
//! Endpoints: list, create, bulk-create, update, delete, reorder.

use axum::{
    Json,
    extract::{Path, Query, State},
    http::StatusCode,
};
use serde_json::json;
use uuid::Uuid;

use crate::{AppState, middleware::admin::CmsEditorUser, utils::errors::ApiError};

use super::types::{
    ApiResult, ApiResultEmpty, BulkCreateEntriesRequest, CmsDatasourceEntry,
    CmsDatasourceEntrySummary, CreateEntryRequest, ListEntriesQuery, PaginatedEntriesResponse,
    PaginationMeta, ReorderEntriesRequest, UpdateEntryRequest,
};

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
pub(super) async fn list_entries(
    State(state): State<AppState>,
    CmsEditorUser(_user): CmsEditorUser,
    Path(datasource_id): Path<Uuid>,
    Query(query): Query<ListEntriesQuery>,
) -> ApiResult<PaginatedEntriesResponse> {
    let (limit, offset) = super::helpers::pagination(query.limit, query.offset, 100, 1000)?;
    if let Some(search) = &query.search {
        super::helpers::text(search, "Search", 500, false)?;
    }
    if let Some(dimension) = &query.dimension {
        super::helpers::text(dimension, "Dimension", 50, false)?;
    }
    let dimension = query
        .dimension
        .clone()
        .unwrap_or_else(|| "default".to_string());
    let search_pattern = query.search.as_ref().map(|s| format!("%{s}%"));

    // Verify datasource exists
    let exists: (bool,) = sqlx::query_as(
        "SELECT EXISTS(SELECT 1 FROM cms_datasources WHERE id = $1 AND deleted_at IS NULL)",
    )
    .bind(datasource_id)
    .fetch_one(&state.db.pool)
    .await
    .map_err(ApiError::from)?;

    if !exists.0 {
        return Err(
            ApiError::new(StatusCode::NOT_FOUND, "Datasource not found").with_code("NOT_FOUND")
        );
    }

    // Count total
    let count_result: (i64,) = sqlx::query_as(
        r"
        SELECT COUNT(*)
        FROM cms_datasource_entries
        WHERE datasource_id = $1
          AND dimension = $2
          AND ($3::text IS NULL OR name ILIKE $3 OR value ILIKE $3)
        ",
    )
    .bind(datasource_id)
    .bind(&dimension)
    .bind(&search_pattern)
    .fetch_one(&state.db.pool)
    .await
    .map_err(ApiError::from)?;

    let total = count_result.0;

    // Fetch entries
    let entries: Vec<CmsDatasourceEntrySummary> = sqlx::query_as(
        r"
        SELECT id, name, value, dimension, sort_order, metadata
        FROM cms_datasource_entries
        WHERE datasource_id = $1
          AND dimension = $2
          AND ($3::text IS NULL OR name ILIKE $3 OR value ILIKE $3)
        ORDER BY sort_order, name
        LIMIT $4 OFFSET $5
        ",
    )
    .bind(datasource_id)
    .bind(&dimension)
    .bind(&search_pattern)
    .bind(limit)
    .bind(offset)
    .fetch_all(&state.db.pool)
    .await
    .map_err(ApiError::from)?;

    // Get available dimensions
    let dimensions: Vec<String> = sqlx::query_scalar(
        r"
        SELECT DISTINCT dimension
        FROM cms_datasource_entries
        WHERE datasource_id = $1
        ORDER BY dimension
        ",
    )
    .bind(datasource_id)
    .fetch_all(&state.db.pool)
    .await
    .map_err(ApiError::from)?;

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
pub(super) async fn create_entry(
    State(state): State<AppState>,
    CmsEditorUser(user): CmsEditorUser,
    Path(datasource_id): Path<Uuid>,
    Json(request): Json<CreateEntryRequest>,
) -> Result<(StatusCode, Json<CmsDatasourceEntry>), ApiError> {
    request.validate()?;
    super::helpers::require_writable(&state.db.pool, datasource_id, &user).await?;
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
    .map_err(ApiError::from)?;

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
    .map_err(ApiError::from)?;

    let new_id = Uuid::new_v4();
    let entry: CmsDatasourceEntry = sqlx::query_as(
        r"
        INSERT INTO cms_datasource_entries (id, datasource_id, name, value, dimension, sort_order, metadata)
        VALUES ($1, $2, $3, $4, $5, $6, $7)
        RETURNING id, datasource_id, name, value, dimension, sort_order, metadata, created_at, updated_at
        ",
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
pub(super) async fn bulk_create_entries(
    State(state): State<AppState>,
    CmsEditorUser(user): CmsEditorUser,
    Path(datasource_id): Path<Uuid>,
    Json(request): Json<BulkCreateEntriesRequest>,
) -> ApiResultEmpty {
    super::helpers::require_writable(&state.db.pool, datasource_id, &user).await?;
    // Verify datasource exists
    let exists: (bool,) = sqlx::query_as(
        "SELECT EXISTS(SELECT 1 FROM cms_datasources WHERE id = $1 AND deleted_at IS NULL)",
    )
    .bind(datasource_id)
    .fetch_one(&state.db.pool)
    .await
    .map_err(ApiError::from)?;

    if !exists.0 {
        return Err(
            ApiError::new(StatusCode::NOT_FOUND, "Datasource not found").with_code("NOT_FOUND")
        );
    }

    if request.entries.len() > 1000 {
        return Err(ApiError::validation_error(
            "At most 1000 entries can be added in one batch",
        ));
    }
    for entry in &request.entries {
        entry.validate()?;
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
            r"
            INSERT INTO cms_datasource_entries (id, datasource_id, name, value, dimension, sort_order, metadata)
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            ON CONFLICT (datasource_id, value, dimension) DO NOTHING
            ",
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
pub(super) async fn update_entry(
    State(state): State<AppState>,
    CmsEditorUser(user): CmsEditorUser,
    Path((datasource_id, entry_id)): Path<(Uuid, Uuid)>,
    Json(request): Json<UpdateEntryRequest>,
) -> ApiResult<CmsDatasourceEntry> {
    request.validate()?;
    super::helpers::require_writable(&state.db.pool, datasource_id, &user).await?;
    let entry: CmsDatasourceEntry = sqlx::query_as(
        r"
        UPDATE cms_datasource_entries
        SET name = COALESCE($3, name),
            value = COALESCE($4, value),
            dimension = COALESCE($5, dimension),
            sort_order = COALESCE($6, sort_order),
            metadata = COALESCE($7, metadata),
            updated_at = NOW()
        WHERE id = $1 AND datasource_id = $2
        RETURNING id, datasource_id, name, value, dimension, sort_order, metadata, created_at, updated_at
        ",
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
    .map_err(ApiError::from)?
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
pub(super) async fn delete_entry(
    State(state): State<AppState>,
    CmsEditorUser(user): CmsEditorUser,
    Path((datasource_id, entry_id)): Path<(Uuid, Uuid)>,
) -> ApiResultEmpty {
    super::helpers::require_writable(&state.db.pool, datasource_id, &user).await?;
    let result =
        sqlx::query("DELETE FROM cms_datasource_entries WHERE id = $1 AND datasource_id = $2")
            .bind(entry_id)
            .bind(datasource_id)
            .execute(&state.db.pool)
            .await
            .map_err(ApiError::from)?;

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
pub(super) async fn reorder_entries(
    State(state): State<AppState>,
    CmsEditorUser(user): CmsEditorUser,
    Path(datasource_id): Path<Uuid>,
    Json(request): Json<ReorderEntriesRequest>,
) -> ApiResultEmpty {
    super::helpers::require_writable(&state.db.pool, datasource_id, &user).await?;
    let selected: std::collections::HashSet<Uuid> = request.entry_ids.iter().copied().collect();
    if selected.len() != request.entry_ids.len() || selected.len() > 2000 {
        return Err(ApiError::validation_error(
            "Provide at most 2000 distinct entry IDs",
        ));
    }
    if selected.is_empty() {
        return Ok(Json(
            json!({"message":"No entries reordered","reordered_count":0}),
        ));
    }
    let mut tx = state.db.pool.begin().await.map_err(|error| {
        tracing::error!(%error,"Unable to reorder entries");
        ApiError::database_error("Unable to reorder entries")
    })?;
    let flags:Option<(bool,bool)>=sqlx::query_as("SELECT is_locked,is_system FROM cms_datasources WHERE id=$1 AND deleted_at IS NULL FOR UPDATE").bind(datasource_id).fetch_optional(&mut *tx).await?;
    let (locked, system) = flags.ok_or_else(|| ApiError::not_found("Datasource not found"))?;
    if locked || system {
        super::helpers::require_cms_admin(&user)?;
    }
    let selected_rows:Vec<(Uuid,String)>=sqlx::query_as("SELECT id,COALESCE(dimension,'default') FROM cms_datasource_entries WHERE datasource_id=$1 AND id=ANY($2)").bind(datasource_id).bind(&request.entry_ids).fetch_all(&mut *tx).await?;
    if selected_rows.len() != selected.len() {
        return Err(ApiError::validation_error(
            "Every reordered entry must belong to this datasource",
        ));
    }
    let dimension = &selected_rows[0].1;
    if selected_rows.iter().any(|row| &row.1 != dimension) {
        return Err(ApiError::validation_error(
            "Reorder entries within one dimension at a time",
        ));
    }
    let mut order:Vec<Uuid>=sqlx::query_scalar("SELECT id FROM cms_datasource_entries WHERE datasource_id=$1 AND COALESCE(dimension,'default')=$2 ORDER BY sort_order,name,id FOR UPDATE").bind(datasource_id).bind(dimension).fetch_all(&mut *tx).await?;
    let mut next = request.entry_ids.iter();
    for id in &mut order {
        if selected.contains(id) {
            *id = *next.next().ok_or_else(|| {
                ApiError::validation_error("Entry order changed; reload before reordering")
            })?;
        }
    }
    if next.next().is_some() {
        return Err(ApiError::validation_error(
            "Entry order changed; reload before reordering",
        ));
    }
    sqlx::query("WITH ordered AS(SELECT id,(ordinality-1)::int position FROM unnest($2::uuid[]) WITH ORDINALITY input(id,ordinality)) UPDATE cms_datasource_entries entry SET sort_order=ordered.position,updated_at=NOW() FROM ordered WHERE entry.datasource_id=$1 AND entry.id=ordered.id").bind(datasource_id).bind(&order).execute(&mut *tx).await?;
    tx.commit().await?;
    Ok(Json(
        json!({"message":"Entries reordered successfully","reordered_count":selected.len()}),
    ))
}
