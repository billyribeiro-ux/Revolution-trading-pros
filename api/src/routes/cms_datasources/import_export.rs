//! CSV import + export handlers for datasource entries.

use axum::{
    Json,
    extract::{Multipart, Path, State},
    http::StatusCode,
};
use serde_json::json;
use uuid::Uuid;

use crate::{AppState, middleware::admin::CmsEditorUser, utils::errors::ApiError};

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
    CmsEditorUser(_user): CmsEditorUser,
    Path(datasource_id): Path<Uuid>,
) -> Result<
    (
        StatusCode,
        [(axum::http::header::HeaderName, String); 2],
        String,
    ),
    ApiError,
> {
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
            .map_err(ApiError::from)?
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
    .map_err(ApiError::from)?;

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
    CmsEditorUser(user): CmsEditorUser,
    Path(datasource_id): Path<Uuid>,
    mut multipart: Multipart,
) -> ApiResultEmpty {
    super::helpers::require_writable(&state.db.pool, datasource_id, &user).await?;
    let mut contents = None;
    while let Some(field) = multipart
        .next_field()
        .await
        .map_err(|_| ApiError::validation_error("Unable to read CSV upload"))?
    {
        if field.name() != Some("file") {
            continue;
        }
        if contents.is_some() {
            return Err(ApiError::validation_error("Upload one CSV file"));
        }
        let bytes = field
            .bytes()
            .await
            .map_err(|_| ApiError::validation_error("Unable to read CSV upload"))?;
        if bytes.len() > 2 * 1024 * 1024 {
            return Err(ApiError::validation_error("CSV file must be at most 2 MiB"));
        }
        contents = Some(
            String::from_utf8(bytes.to_vec())
                .map_err(|_| ApiError::validation_error("CSV must use UTF-8 encoding"))?,
        );
    }
    let contents = contents
        .filter(|value| !value.is_empty())
        .ok_or_else(|| ApiError::validation_error("Upload a nonempty CSV file"))?;
    let mut reader = csv::ReaderBuilder::new()
        .has_headers(true)
        .flexible(true)
        .from_reader(contents.as_bytes());
    let headers: Vec<String> = reader
        .headers()
        .map_err(|_| ApiError::validation_error("Invalid CSV header"))?
        .iter()
        .map(|header| header.trim().trim_start_matches('\u{feff}').to_lowercase())
        .collect();
    let unique: std::collections::HashSet<&str> = headers.iter().map(String::as_str).collect();
    if unique.len() != headers.len()
        || !unique.contains("name")
        || !unique.contains("value")
        || unique
            .iter()
            .any(|header| !["name", "value", "dimension"].contains(header))
    {
        return Err(ApiError::validation_error(
            "CSV needs name and value headers and an optional dimension header, without duplicate or unknown columns",
        ));
    }
    let name_index = headers
        .iter()
        .position(|header| header == "name")
        .ok_or_else(|| ApiError::validation_error("Missing name header"))?;
    let value_index = headers
        .iter()
        .position(|header| header == "value")
        .ok_or_else(|| ApiError::validation_error("Missing value header"))?;
    let dimension_index = headers.iter().position(|header| header == "dimension");
    let mut tx = state.db.pool.begin().await?;
    let flags:Option<(bool,bool)>=sqlx::query_as("SELECT is_locked,is_system FROM cms_datasources WHERE id=$1 AND deleted_at IS NULL FOR UPDATE").bind(datasource_id).fetch_optional(&mut *tx).await?;
    let (locked, system) = flags.ok_or_else(|| ApiError::not_found("Datasource not found"))?;
    if locked || system {
        super::helpers::require_cms_admin(&user)?;
    }
    let mut next_sort: i32 = sqlx::query_scalar(
        "SELECT COALESCE(MAX(sort_order),-1)+1 FROM cms_datasource_entries WHERE datasource_id=$1",
    )
    .bind(datasource_id)
    .fetch_one(&mut *tx)
    .await?;
    let mut created = 0;
    let mut updated = 0;
    let mut skipped = 0;
    let mut errors = 0;
    let mut row_errors = Vec::new();
    for (index, row) in reader.records().enumerate() {
        if index >= 20_000 {
            return Err(ApiError::validation_error(
                "CSV may contain at most 20,000 entries",
            ));
        }
        let row = match row {
            Ok(row) => row,
            Err(_) => {
                errors += 1;
                if row_errors.len() < 20 {
                    row_errors.push(json!({"row":index+2,"message":"Malformed CSV row"}));
                }
                continue;
            }
        };
        let name = row.get(name_index).unwrap_or("").trim();
        let value = row.get(value_index).unwrap_or("").trim();
        let dimension = dimension_index
            .and_then(|index| row.get(index))
            .map(str::trim)
            .filter(|value| !value.is_empty())
            .unwrap_or("default");
        if name.is_empty() || value.is_empty() {
            skipped += 1;
            continue;
        }
        let validation = super::helpers::text(name, "Name", 255, true)
            .and_then(|()| super::helpers::text(value, "Value", 500, true))
            .and_then(|()| super::helpers::text(dimension, "Dimension", 50, true));
        if let Err(error) = validation {
            errors += 1;
            if row_errors.len() < 20 {
                row_errors.push(json!({"row":index+2,"message":error.message}));
            }
            continue;
        }
        sqlx::query("SAVEPOINT cms_csv_row")
            .execute(&mut *tx)
            .await?;
        let inserted=sqlx::query("INSERT INTO cms_datasource_entries(id,datasource_id,name,value,dimension,sort_order) VALUES($1,$2,$3,$4,$5,$6) ON CONFLICT(datasource_id,value,dimension) DO NOTHING").bind(Uuid::new_v4()).bind(datasource_id).bind(name).bind(value).bind(dimension).bind(next_sort).execute(&mut *tx).await;
        let outcome=match inserted {
            Ok(result) if result.rows_affected()==1=>Ok(true),
            Ok(_)=>sqlx::query("UPDATE cms_datasource_entries SET name=$4,updated_at=NOW() WHERE datasource_id=$1 AND value=$2 AND dimension=$3").bind(datasource_id).bind(value).bind(dimension).bind(name).execute(&mut *tx).await.and_then(|result|if result.rows_affected()==1{Ok(false)}else{Err(sqlx::Error::RowNotFound)}),
            Err(error)=>Err(error),
        };
        match outcome {
            Ok(inserted) => {
                sqlx::query("RELEASE SAVEPOINT cms_csv_row")
                    .execute(&mut *tx)
                    .await?;
                if inserted {
                    created += 1;
                    next_sort = next_sort
                        .checked_add(1)
                        .ok_or_else(|| ApiError::validation_error("Too many datasource entries"))?;
                } else {
                    updated += 1;
                }
            }
            Err(error) => {
                sqlx::query("ROLLBACK TO SAVEPOINT cms_csv_row")
                    .execute(&mut *tx)
                    .await?;
                sqlx::query("RELEASE SAVEPOINT cms_csv_row")
                    .execute(&mut *tx)
                    .await?;
                tracing::warn!(%error,row=index+2,"CSV entry could not be saved");
                errors += 1;
                if row_errors.len() < 20 {
                    row_errors.push(json!({"row":index+2,"message":"Entry could not be saved"}));
                }
            }
        }
    }
    tx.commit().await?;
    Ok(Json(
        json!({"message":if errors>0{"Import completed with row errors"}else{"Import completed"},"created":created,"updated":updated,"skipped":skipped,"errors":errors,"row_errors":row_errors}),
    ))
}
