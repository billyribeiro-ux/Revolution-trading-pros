//! CSV import + export handlers for datasource entries.

use axum::{
    extract::{Multipart, Path, State},
    http::StatusCode,
    Json,
};
use serde_json::json;
use uuid::Uuid;

use crate::{models::User, utils::errors::ApiError, AppState};

use super::helpers::require_cms_editor;
use super::types::{ApiResultEmpty, CsvEntryRow};

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
pub(super) async fn export_entries_csv(
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
        r"
        SELECT name, value, dimension
        FROM cms_datasource_entries
        WHERE datasource_id = $1
        ORDER BY dimension, sort_order, name
        ",
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
                format!("attachment; filename=\"{filename}\""),
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
pub(super) async fn import_entries_csv(
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
    while let Some(field) = multipart
        .next_field()
        .await
        .map_err(|e| ApiError::new(StatusCode::BAD_REQUEST, format!("Failed to read file: {e}")))?
    {
        if field.name() == Some("file") {
            let bytes = field.bytes().await.map_err(|e| {
                ApiError::new(StatusCode::BAD_REQUEST, format!("Failed to read file: {e}"))
            })?;
            csv_content = String::from_utf8(bytes.to_vec()).map_err(|e| {
                ApiError::new(StatusCode::BAD_REQUEST, format!("Invalid UTF-8: {e}"))
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
                    r"
                    INSERT INTO cms_datasource_entries (id, datasource_id, name, value, dimension, sort_order)
                    VALUES ($1, $2, $3, $4, $5, $6)
                    ON CONFLICT (datasource_id, value, dimension) DO UPDATE SET name = EXCLUDED.name
                    ",
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
