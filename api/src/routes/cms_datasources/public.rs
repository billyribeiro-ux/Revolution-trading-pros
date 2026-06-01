//! No-auth public datasource read endpoint(s).

use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Json,
};

use crate::{utils::errors::ApiError, AppState};

use super::types::{ApiResult, ListEntriesQuery, PublicDatasourceResponse, PublicEntryResponse};

/// Public endpoint to get entries by datasource slug
pub(super) async fn public_get_entries(
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
        r"
        SELECT e.name, e.value
        FROM cms_datasource_entries e
        JOIN cms_datasources d ON e.datasource_id = d.id
        WHERE d.slug = $1 AND d.deleted_at IS NULL AND e.dimension = $2
        ORDER BY e.sort_order, e.name
        ",
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
