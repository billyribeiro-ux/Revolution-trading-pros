//! Bounded contact transfers: durable intent, atomic effects and actual downloads.
use super::contacts::CreateContactInput;
use crate::{
    AppState, routes::crm::manager_permissions::CrmUser as AdminUser, services::crm_logs,
    utils::errors::ApiError,
};
use axum::{
    Json, Router,
    body::to_bytes,
    extract::{DefaultBodyLimit, FromRequest, Multipart, Path, Query, Request, State},
    http::{StatusCode, header},
    response::{IntoResponse, Response},
    routing::{get, post},
};
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value, json};
use sqlx::{PgConnection, Postgres, QueryBuilder, Transaction};
use std::collections::{BTreeMap, BTreeSet};
use uuid::Uuid;

const MAX_BYTES: usize = 2 * 1024 * 1024;
const MAX_ROWS: usize = 1000;
const INPUT_FIELDS: &[&str] = &[
    "email",
    "first_name",
    "last_name",
    "phone",
    "company",
    "job_title",
    "status",
    "source",
    "tags",
    "custom_fields",
];
const OUTPUT_FIELDS: &[&str] = &[
    "id",
    "email",
    "first_name",
    "last_name",
    "phone",
    "company",
    "job_title",
    "status",
    "source",
    "score",
    "tags",
    "custom_fields",
    "last_contacted_at",
    "created_at",
    "updated_at",
];
fn db(error: sqlx::Error) -> ApiError {
    tracing::error!(%error,"CRM transfer persistence failed");
    ApiError::database_error("Unable to persist CRM transfer")
}
fn invalid(message: impl Into<String>) -> ApiError {
    ApiError::validation_error(&message.into())
}
fn conflict(message: &str) -> ApiError {
    ApiError::new(StatusCode::CONFLICT, message)
}
fn supported(kind: &str) -> Result<(), ApiError> {
    if kind != "contacts" {
        return Err(invalid(
            "This transfer endpoint currently supports contacts; no other data was imported or exported",
        ));
    }
    Ok(())
}
fn bounded_json(value: &Value, depth: usize) -> bool {
    if depth > 32 {
        return false;
    }
    match value {
        Value::String(text) => !text.contains('\0') && text.len() <= 65536,
        Value::Array(values) => {
            values.len() <= MAX_ROWS && values.iter().all(|value| bounded_json(value, depth + 1))
        }
        Value::Object(values) => {
            values.len() <= 100
                && values.iter().all(|(key, value)| {
                    !key.contains('\0') && key.len() <= 255 && bounded_json(value, depth + 1)
                })
        }
        _ => true,
    }
}
fn contacts_kind() -> String {
    "contacts".into()
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Upload {
    #[serde(rename = "type", default = "contacts_kind")]
    kind: String,
    file_name: String,
    file_type: String,
    data: Value,
}
struct Parsed {
    headers: Vec<String>,
    rows: Vec<Value>,
}
fn parse(upload: &Upload) -> Result<Parsed, ApiError> {
    supported(&upload.kind)?;
    if upload.file_name.is_empty()
        || upload.file_name.len() > 255
        || upload
            .file_name
            .chars()
            .any(|c| c.is_control() || matches!(c, '/' | '\\'))
    {
        return Err(invalid(
            "Provide a filename up to 255 bytes without paths or control characters",
        ));
    }
    if upload.data.to_string().len() > MAX_BYTES {
        return Err(invalid("Contact uploads must be at most 2 MiB"));
    }
    let (headers, rows) = match upload.file_type.as_str() {
        "csv" => {
            let source = upload
                .data
                .as_str()
                .ok_or_else(|| invalid("CSV data must be UTF-8 text"))?;
            if source.contains('\0') {
                return Err(invalid("CSV cannot contain null characters"));
            }
            let mut reader = csv::ReaderBuilder::new()
                .from_reader(source.trim_start_matches('\u{feff}').as_bytes());
            let headers: Vec<String> = reader
                .headers()
                .map_err(|_| invalid("Invalid CSV header"))?
                .iter()
                .map(|value| value.trim().to_string())
                .collect();
            let mut rows = Vec::new();
            for row in reader.records() {
                let row = row.map_err(|error| invalid(format!("Invalid CSV record: {error}")))?;
                if rows.len() == MAX_ROWS {
                    return Err(invalid("Contact uploads support at most 1000 rows"));
                }
                rows.push(Value::Object(
                    headers
                        .iter()
                        .cloned()
                        .zip(row.iter().map(|value| Value::String(value.into())))
                        .collect(),
                ));
            }
            (headers, rows)
        }
        "json" => {
            let value = if let Some(text) = upload.data.as_str() {
                serde_json::from_str::<Value>(text).map_err(|_| invalid("Invalid JSON file"))?
            } else {
                upload.data.clone()
            };
            let rows = value
                .as_array()
                .ok_or_else(|| invalid("JSON imports require an array of contact objects"))?
                .clone();
            let mut headers = BTreeSet::new();
            for row in &rows {
                headers.extend(
                    row.as_object()
                        .ok_or_else(|| invalid("Every JSON contact row must be an object"))?
                        .keys()
                        .cloned(),
                );
            }
            (headers.into_iter().collect(), rows)
        }
        _ => return Err(invalid("Choose csv or json file_type")),
    };
    if headers.is_empty()
        || headers.len() > 50
        || headers.iter().any(|value| {
            value.is_empty() || value.len() > 100 || value.chars().any(char::is_control)
        })
        || headers.iter().collect::<BTreeSet<_>>().len() != headers.len()
    {
        return Err(invalid(
            "Provide 1–50 unique, nonempty headers up to 100 bytes",
        ));
    }
    if rows.is_empty() || rows.len() > MAX_ROWS || !rows.iter().all(|row| bounded_json(row, 0)) {
        return Err(invalid(
            "Provide 1–1000 bounded contact objects without null characters",
        ));
    }
    Ok(Parsed { headers, rows })
}
async fn read_upload(request: Request, state: &AppState) -> Result<Upload, ApiError> {
    let is_multipart = request
        .headers()
        .get(header::CONTENT_TYPE)
        .and_then(|value| value.to_str().ok())
        .is_some_and(|value| value.starts_with("multipart/form-data"));
    if !is_multipart {
        let bytes = to_bytes(request.into_body(), MAX_BYTES + 65536)
            .await
            .map_err(|_| invalid("Contact upload exceeds the 2 MiB limit"))?;
        return serde_json::from_slice(&bytes)
            .map_err(|_| invalid("Provide a valid JSON contact upload"));
    }
    let mut multipart = Multipart::from_request(request, state)
        .await
        .map_err(|_| invalid("Invalid multipart upload"))?;
    let mut kind = None;
    let mut file = None;
    while let Some(field) = multipart
        .next_field()
        .await
        .map_err(|_| invalid("Invalid or oversized upload"))?
    {
        match field.name() {
            Some("type") if kind.is_none() => {
                kind = Some(
                    field
                        .text()
                        .await
                        .map_err(|_| invalid("Invalid import type"))?,
                );
            }
            Some("file") if file.is_none() => {
                let name = field
                    .file_name()
                    .ok_or_else(|| invalid("File upload requires a filename"))?
                    .to_string();
                let bytes = field
                    .bytes()
                    .await
                    .map_err(|_| invalid("Invalid or oversized upload"))?;
                if bytes.len() > MAX_BYTES {
                    return Err(invalid("Contact uploads must be at most 2 MiB"));
                }
                file = Some((
                    name,
                    String::from_utf8(bytes.to_vec())
                        .map_err(|_| invalid("Upload must use UTF-8 encoding"))?,
                ));
            }
            _ => return Err(invalid("Provide one type and one file field")),
        }
    }
    let (file_name, text) = file.ok_or_else(|| invalid("Choose a CSV or JSON file"))?;
    let file_type = file_name
        .rsplit('.')
        .next()
        .unwrap_or_default()
        .to_ascii_lowercase();
    Ok(Upload {
        kind: kind.unwrap_or_else(contacts_kind),
        file_name,
        file_type,
        data: Value::String(text),
    })
}

const PUBLIC_JOB: &str = "SELECT jsonb_build_object('id',id::text,'type',kind,'file_name',file_name,'file_type',file_type,'status',status,'total_rows',total_rows,'total_records',total_rows,'processed_rows',processed_rows,'success_count',success_count,'created_count',created_count,'updated_count',updated_count,'skipped_count',skipped_count,'error_count',error_count,'errors',errors,'field_mapping',field_mapping,'settings',settings,'filters',filters,'fields',fields,'file_url',CASE WHEN direction='export' AND status='completed' THEN '/api/admin/crm/exports/'||id::text||'/download' ELSE NULL END,'created_by',created_by::text,'started_at',started_at,'completed_at',completed_at,'created_at',created_at,'updated_at',updated_at) FROM crm_transfer_jobs";
async fn public_job(
    connection: &mut PgConnection,
    id: Uuid,
    direction: &str,
) -> Result<Value, ApiError> {
    let mut query = QueryBuilder::<Postgres>::new(PUBLIC_JOB);
    query
        .push(" WHERE id=")
        .push_bind(id)
        .push(" AND direction=")
        .push_bind(direction);
    query
        .build_query_scalar()
        .fetch_optional(connection)
        .await
        .map_err(db)?
        .ok_or_else(|| ApiError::not_found("Transfer job not found"))
}
async fn save_upload(state: &AppState, actor: i64, upload: Upload) -> Result<Value, ApiError> {
    let mut tx = state.db.pool.begin().await.map_err(db)?;
    let saved = save_upload_on(&mut tx, actor, upload).await?;
    tx.commit().await.map_err(db)?;
    Ok(saved)
}
async fn save_upload_on(
    tx: &mut Transaction<'_, Postgres>,
    actor: i64,
    upload: Upload,
) -> Result<Value, ApiError> {
    let parsed = parse(&upload)?;
    let id:Uuid=sqlx::query_scalar("INSERT INTO crm_transfer_jobs(direction,kind,file_name,file_type,status,headers,source_rows,total_rows,created_by) VALUES('import',$1,$2,$3,'pending',$4,$5,$6,$7) RETURNING id")
        .bind(upload.kind).bind(upload.file_name).bind(upload.file_type).bind(json!(parsed.headers)).bind(json!(parsed.rows)).bind(parsed.rows.len() as i32).bind(actor).fetch_one(&mut **tx).await.map_err(db)?;
    crm_logs::record(
        tx,
        "info",
        "import",
        "Contact import file saved",
        json!({"job_id":id,"total_rows":parsed.rows.len()}),
        Some(actor),
    )
    .await
    .map_err(db)?;
    let job = public_job(tx, id, "import").await?;
    Ok(job)
}
async fn create_import(
    State(state): State<AppState>,
    admin: AdminUser,
    request: Request,
) -> Result<Json<Value>, ApiError> {
    let upload = read_upload(request, &state).await?;
    Ok(Json(save_upload(&state, admin.0.id, upload).await?))
}
#[derive(Default, Deserialize)]
#[serde(deny_unknown_fields)]
struct Filters {
    #[serde(rename = "type")]
    kind: Option<String>,
    status: Option<String>,
    page: Option<i64>,
    per_page: Option<i64>,
}
async fn history(state: &AppState, filters: Filters, direction: &str) -> Result<Value, ApiError> {
    let page = filters.page.unwrap_or(1);
    let size = filters.per_page.unwrap_or(20);
    if !(1..=1_000_000).contains(&page) || !(1..=100).contains(&size) {
        return Err(invalid("Use a positive page and page size 1–100"));
    }
    if let Some(kind) = &filters.kind {
        supported(kind)?;
    }
    if filters
        .status
        .as_deref()
        .is_some_and(|status| !["pending", "completed", "failed", "cancelled"].contains(&status))
    {
        return Err(invalid("Invalid transfer status"));
    }
    let total:i64=sqlx::query_scalar("SELECT COUNT(*) FROM crm_transfer_jobs WHERE direction=$1 AND ($2::text IS NULL OR kind=$2) AND ($3::text IS NULL OR status=$3)").bind(direction).bind(&filters.kind).bind(&filters.status).fetch_one(&state.db.pool).await.map_err(db)?;
    let mut query = QueryBuilder::<Postgres>::new(PUBLIC_JOB);
    query.push(" WHERE direction=").push_bind(direction);
    if let Some(kind) = filters.kind {
        query.push(" AND kind=").push_bind(kind);
    }
    if let Some(status) = filters.status {
        query.push(" AND status=").push_bind(status);
    }
    query
        .push(" ORDER BY created_at DESC,id DESC LIMIT ")
        .push_bind(size)
        .push(" OFFSET ")
        .push_bind((page - 1) * size);
    let rows: Vec<Value> = query
        .build_query_scalar()
        .fetch_all(&state.db.pool)
        .await
        .map_err(db)?;
    Ok(
        json!({"data":rows,"meta":{"total":total,"current_page":page,"per_page":size,"total_pages":((total+size-1)/size).max(1)}}),
    )
}
async fn imports(
    State(state): State<AppState>,
    _admin: AdminUser,
    Query(filters): Query<Filters>,
) -> Result<Json<Value>, ApiError> {
    Ok(Json(history(&state, filters, "import").await?))
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct OptionFilters {
    kind: String,
    search: Option<String>,
    page: Option<i64>,
    per_page: Option<i64>,
    selected: Option<String>,
}
async fn options(
    State(state): State<AppState>,
    _admin: AdminUser,
    Query(filters): Query<OptionFilters>,
) -> Result<Json<Value>, ApiError> {
    if !["lists", "tags"].contains(&filters.kind.as_str()) {
        return Err(invalid(
            "Choose lists or tags for contact import references",
        ));
    }
    let page = filters.page.unwrap_or(1);
    let size = filters.per_page.unwrap_or(20);
    if !(1..=1_000_000).contains(&page)
        || !(1..=100).contains(&size)
        || filters
            .search
            .as_ref()
            .is_some_and(|value| value.len() > 500 || value.contains('\0'))
        || filters
            .selected
            .as_ref()
            .is_some_and(|value| value.len() > 2000)
    {
        return Err(invalid(
            "Use positive option pages and bounded search or reference IDs",
        ));
    }
    let selected = ids(&filters
        .selected
        .unwrap_or_default()
        .split(',')
        .filter(|value| !value.is_empty())
        .map(str::to_string)
        .collect::<Vec<_>>())?;
    let base = if filters.kind == "lists" {
        "SELECT jsonb_build_object('id',id::text,'title',COALESCE(title,name)) FROM crm_lists"
    } else {
        "SELECT jsonb_build_object('id',id::text,'title',COALESCE(title,name)) FROM crm_tags"
    };
    let search = filters
        .search
        .map(|value| format!("%{}%", crate::utils::escape_like(&value)));
    let total: i64 = sqlx::query_scalar(if filters.kind == "lists" {
        "SELECT COUNT(*) FROM crm_lists WHERE $1::text IS NULL OR COALESCE(title,name) ILIKE $1"
    } else {
        "SELECT COUNT(*) FROM crm_tags WHERE $1::text IS NULL OR COALESCE(title,name) ILIKE $1"
    })
    .bind(&search)
    .fetch_one(&state.db.pool)
    .await
    .map_err(db)?;
    let mut query = QueryBuilder::<Postgres>::new(base);
    query
        .push(" WHERE ")
        .push_bind(search.clone())
        .push("::text IS NULL OR COALESCE(title,name) ILIKE ")
        .push_bind(search)
        .push(" ORDER BY COALESCE(title,name),id LIMIT ")
        .push_bind(size)
        .push(" OFFSET ")
        .push_bind((page - 1) * size);
    let data: Vec<Value> = query
        .build_query_scalar()
        .fetch_all(&state.db.pool)
        .await
        .map_err(db)?;
    let mut selected_query = QueryBuilder::<Postgres>::new(base);
    selected_query
        .push(" WHERE id=ANY(")
        .push_bind(&selected)
        .push(") ORDER BY id");
    let selected_rows: Vec<Value> = selected_query
        .build_query_scalar()
        .fetch_all(&state.db.pool)
        .await
        .map_err(db)?;
    let present: BTreeSet<_> = selected_rows
        .iter()
        .filter_map(|row| row["id"].as_str())
        .collect();
    let missing: Vec<String> = selected
        .iter()
        .map(ToString::to_string)
        .filter(|id| !present.contains(id.as_str()))
        .collect();
    Ok(Json(
        json!({"data":data,"selected":selected_rows,"missing":missing,"meta":{"total":total,"current_page":page,"per_page":size,"total_pages":((total+size-1)/size).max(1)}}),
    ))
}
async fn exports(
    State(state): State<AppState>,
    _admin: AdminUser,
    Query(filters): Query<Filters>,
) -> Result<Json<Value>, ApiError> {
    Ok(Json(history(&state, filters, "export").await?))
}
async fn get_import(
    State(state): State<AppState>,
    _admin: AdminUser,
    Path(id): Path<Uuid>,
) -> Result<Json<Value>, ApiError> {
    let mut connection = state.db.pool.acquire().await.map_err(db)?;
    Ok(Json(public_job(&mut connection, id, "import").await?))
}
async fn get_export(
    State(state): State<AppState>,
    _admin: AdminUser,
    Path(id): Path<Uuid>,
) -> Result<Json<Value>, ApiError> {
    let mut connection = state.db.pool.acquire().await.map_err(db)?;
    Ok(Json(public_job(&mut connection, id, "export").await?))
}
async fn preview(
    State(state): State<AppState>,
    _admin: AdminUser,
    Path(id): Path<Uuid>,
) -> Result<Json<Value>, ApiError> {
    let (headers, rows): (Value, Value) = sqlx::query_as(
        "SELECT headers,source_rows FROM crm_transfer_jobs WHERE id=$1 AND direction='import'",
    )
    .bind(id)
    .fetch_optional(&state.db.pool)
    .await
    .map_err(db)?
    .ok_or_else(|| ApiError::not_found("Import job not found"))?;
    let rows = rows
        .as_array()
        .ok_or_else(|| ApiError::internal_error("Invalid saved import rows"))?;
    Ok(Json(
        json!({"headers":headers,"rows":rows.iter().take(20).collect::<Vec<_>>(),"total_rows":rows.len()}),
    ))
}

#[derive(Clone, Deserialize, Serialize)]
#[serde(default, deny_unknown_fields)]
struct Settings {
    update_existing: bool,
    skip_duplicates: bool,
    add_to_lists: Vec<String>,
    apply_tags: Vec<String>,
    contact_status: Option<String>,
}
impl Default for Settings {
    fn default() -> Self {
        Self {
            update_existing: false,
            skip_duplicates: true,
            add_to_lists: Vec::new(),
            apply_tags: Vec::new(),
            contact_status: None,
        }
    }
}
#[derive(Default, Deserialize, Serialize)]
#[serde(default, deny_unknown_fields)]
struct Start {
    mapping: BTreeMap<String, String>,
    settings: Settings,
}
fn ids(values: &[String]) -> Result<Vec<i64>, ApiError> {
    if values.len() > 100 {
        return Err(invalid("Apply at most 100 lists or tags"));
    }
    let mut result = values
        .iter()
        .map(|value| {
            value
                .parse::<i64>()
                .ok()
                .filter(|id| *id > 0)
                .ok_or_else(|| invalid("List and tag IDs must be positive integers"))
        })
        .collect::<Result<Vec<_>, _>>()?;
    result.sort_unstable();
    result.dedup();
    Ok(result)
}
fn mapping(
    headers: &Value,
    supplied: &BTreeMap<String, String>,
) -> Result<BTreeMap<String, String>, ApiError> {
    let headers: Vec<String> = serde_json::from_value(headers.clone())
        .map_err(|_| invalid("Invalid saved import headers"))?;
    let result = if supplied.is_empty() {
        headers
            .iter()
            .filter(|field| INPUT_FIELDS.contains(&field.as_str()))
            .map(|field| (field.clone(), field.clone()))
            .collect()
    } else {
        supplied.clone()
    };
    let targets: BTreeSet<_> = result.values().collect();
    if result.len() > 50
        || targets.len() != result.len()
        || !targets.iter().any(|field| field.as_str() == "email")
        || result.iter().any(|(source, target)| {
            !headers.contains(source) || !INPUT_FIELDS.contains(&target.as_str())
        })
    {
        return Err(invalid(
            "Map an existing source header to email; each destination must be supported and mapped once",
        ));
    }
    Ok(result)
}
fn mapped(row: &Value, mapping: &BTreeMap<String, String>) -> Result<Map<String, Value>, String> {
    let mut output = Map::new();
    for (source, target) in mapping {
        let Some(value) = row.get(source) else {
            continue;
        };
        let value = if matches!(target.as_str(), "tags" | "custom_fields") {
            if let Some(text) = value.as_str() {
                if text.trim().is_empty() {
                    if target == "tags" {
                        json!([])
                    } else {
                        json!({})
                    }
                } else {
                    serde_json::from_str(text)
                        .map_err(|_| format!("{target} must contain valid JSON"))?
                }
            } else {
                value.clone()
            }
        } else if let Some(text) = value.as_str() {
            let text = text.trim();
            if text.is_empty() && target != "email" {
                Value::Null
            } else {
                Value::String(text.into())
            }
        } else {
            value.clone()
        };
        output.insert(target.clone(), value);
    }
    if let Some(Value::String(email)) = output.get_mut("email") {
        *email = email.to_lowercase();
    }
    Ok(output)
}
#[derive(sqlx::FromRow)]
struct SavedImport {
    status: String,
    headers: Value,
    source_rows: Value,
    field_mapping: Value,
    settings: Value,
}
struct Change {
    tags_changed: bool,
    id: Option<i64>,
    contact: CreateContactInput,
}

async fn execute_import(
    state: &AppState,
    actor: &AdminUser,
    id: Uuid,
    input: Start,
) -> Result<Value, ApiError> {
    let mut tx = state.db.pool.begin().await.map_err(db)?;
    let saved = execute_import_on(&mut tx, actor, id, input).await?;
    tx.commit().await.map_err(db)?;
    Ok(saved)
}
async fn execute_import_on(
    tx: &mut Transaction<'_, Postgres>,
    actor: &AdminUser,
    id: Uuid,
    mut input: Start,
) -> Result<Value, ApiError> {
    let job:SavedImport=sqlx::query_as("SELECT status,headers,source_rows,field_mapping,settings FROM crm_transfer_jobs WHERE id=$1 AND direction='import' FOR UPDATE").bind(id).fetch_optional(&mut **tx).await.map_err(db)?.ok_or_else(||ApiError::not_found("Import job not found"))?;
    if job.status == "cancelled" {
        return Err(conflict("A cancelled import cannot be started"));
    }
    input.mapping = mapping(&job.headers, &input.mapping)?;
    actor.body(&json!({"mapping":input.mapping,"settings":input.settings}))?;
    let list_ids = ids(&input.settings.add_to_lists)?;
    let tag_ids = ids(&input.settings.apply_tags)?;
    input.settings.add_to_lists = list_ids.iter().map(ToString::to_string).collect();
    input.settings.apply_tags = tag_ids.iter().map(ToString::to_string).collect();
    if input.settings.update_existing && input.settings.skip_duplicates {
        return Err(invalid("Choose either update_existing or skip_duplicates"));
    }
    if input.settings.contact_status.as_deref().is_some_and(|s| {
        ![
            "new",
            "contacted",
            "qualified",
            "proposal",
            "negotiation",
            "won",
            "lost",
            "lead",
            "prospect",
            "customer",
            "churned",
            "unqualified",
        ]
        .contains(&s)
    }) {
        return Err(invalid("Invalid imported contact status"));
    }
    let mapping_value = json!(input.mapping);
    let settings_value = json!(input.settings);
    if job.status != "pending" {
        if job.field_mapping != mapping_value || job.settings != settings_value {
            return Err(conflict(
                "This import already ran with different mapping or settings",
            ));
        }
        return public_job(tx, id, "import").await;
    }
    // Existing addresses are not unique. Keep bounded duplicate decisions
    // stable against all concurrent contact writes, not only other imports.
    sqlx::query("LOCK TABLE contacts IN SHARE ROW EXCLUSIVE MODE")
        .execute(&mut **tx)
        .await
        .map_err(db)?;
    let found_lists: Vec<i64> =
        sqlx::query_scalar("SELECT id FROM crm_lists WHERE id=ANY($1) ORDER BY id FOR UPDATE")
            .bind(&list_ids)
            .fetch_all(&mut **tx)
            .await
            .map_err(db)?;
    if found_lists != list_ids {
        return Err(invalid("One or more selected CRM lists no longer exist"));
    }
    let tags: Vec<(i64, String)> = sqlx::query_as(
        "SELECT id,COALESCE(title,name) FROM crm_tags WHERE id=ANY($1) ORDER BY id FOR UPDATE",
    )
    .bind(&tag_ids)
    .fetch_all(&mut **tx)
    .await
    .map_err(db)?;
    if tags.iter().map(|row| row.0).collect::<Vec<_>>() != tag_ids {
        return Err(invalid("One or more selected CRM tags no longer exist"));
    }
    let rows = job
        .source_rows
        .as_array()
        .ok_or_else(|| ApiError::internal_error("Invalid saved import rows"))?;
    let mut errors = Vec::new();
    let mut changes = Vec::new();
    let mut seen = BTreeSet::new();
    let mut skipped = 0_i32;
    for (index, row) in rows.iter().enumerate() {
        let mut values = match mapped(row, &input.mapping) {
            Ok(v) => v,
            Err(message) => {
                errors.push(json!({"row":index+1,"message":message}));
                continue;
            }
        };
        let Some(email) = values
            .get("email")
            .and_then(Value::as_str)
            .filter(|v| !v.is_empty())
            .map(str::to_string)
        else {
            errors
                .push(json!({"row":index+1,"field":"email","message":"A valid email is required"}));
            continue;
        };
        if !seen.insert(email.clone()) {
            if input.settings.skip_duplicates {
                skipped += 1;
            } else {
                errors.push(json!({"row":index+1,"field":"email","message":"Repeated email in the uploaded file"}));
            }
            continue;
        }
        let existing: Vec<Value> = sqlx::query_scalar(
            "SELECT to_jsonb(c) FROM contacts c WHERE LOWER(BTRIM(email))=$1 ORDER BY id",
        )
        .bind(&email)
        .fetch_all(&mut **tx)
        .await
        .map_err(db)?;
        if !existing.is_empty() && input.settings.skip_duplicates {
            skipped += 1;
            continue;
        }
        if existing.len() > 1 || (!existing.is_empty() && !input.settings.update_existing) {
            errors.push(json!({"row":index+1,"field":"email","message":"Email already exists or identifies multiple CRM contacts"}));
            continue;
        }
        let previous_tags = existing.first().map(|value| value["tags"].clone());
        let contact_id = existing.first().and_then(|v| v["id"].as_i64());
        if let Some(existing) = existing.first().and_then(Value::as_object) {
            let mut merged: Map<String, Value> = existing
                .iter()
                .filter(|(k, _)| INPUT_FIELDS.contains(&k.as_str()))
                .map(|(k, v)| (k.clone(), v.clone()))
                .collect();
            merged.extend(values);
            values = merged;
        } else {
            values.entry("status").or_insert_with(|| {
                json!(input.settings.contact_status.as_deref().unwrap_or("lead"))
            });
            values.entry("source").or_insert_with(|| json!("import"));
        }
        if let Some(status) = &input.settings.contact_status {
            values.insert("status".into(), json!(status));
        }
        let mut contact: CreateContactInput = match serde_json::from_value(Value::Object(values)) {
            Ok(v) => v,
            Err(_) => {
                errors.push(json!({"row":index+1,"message":"Contact fields have invalid types"}));
                continue;
            }
        };
        let contact_tags = contact.tags.get_or_insert_default();
        for (_, title) in &tags {
            if !contact_tags.contains(title) {
                contact_tags.push(title.clone());
            }
        }
        if let Err((_, message)) = contact.validate(false) {
            errors.push(json!({"row":index+1,"message":message.0["error"].as_str().unwrap_or("Invalid contact fields")}));
            continue;
        }
        let tags_changed = previous_tags.map_or_else(
            || contact.tags.as_ref().is_some_and(|tags| !tags.is_empty()),
            |previous| previous != json!(contact.tags),
        );
        changes.push(Change {
            tags_changed,
            id: contact_id,
            contact,
        });
    }
    actor.import_plan(
        changes.iter().any(|change| change.id.is_none()),
        changes.iter().any(|change| change.id.is_some()),
        changes.iter().any(|change| change.tags_changed),
        !changes.is_empty() && !list_ids.is_empty(),
    )?;
    if !errors.is_empty() {
        sqlx::query("UPDATE crm_transfer_jobs SET status='failed',field_mapping=$2,settings=$3,processed_rows=total_rows,error_count=$4,errors=$5,started_at=NOW(),completed_at=NOW(),updated_at=NOW() WHERE id=$1").bind(id).bind(mapping_value).bind(settings_value).bind(errors.len() as i32).bind(json!(errors)).execute(&mut **tx).await.map_err(db)?;
        crm_logs::record(
            tx,
            "warning",
            "import",
            "Contact import validation failed; no contacts changed",
            json!({"job_id":id,"error_count":errors.len()}),
            Some(actor.0.id),
        )
        .await
        .map_err(db)?;
    } else {
        let mut created = 0_i32;
        let mut updated = 0_i32;
        for change in changes {
            let contact = change.contact;
            let tags = json!(contact.tags.unwrap_or_default());
            let saved: i64 = if let Some(contact_id) = change.id {
                updated += 1;
                sqlx::query_scalar("UPDATE contacts SET first_name=$2,last_name=$3,phone=$4,company=$5,job_title=$6,status=COALESCE($7,status),source=$8,tags=$9,custom_fields=$10,updated_at=NOW() AT TIME ZONE 'UTC' WHERE id=$1 RETURNING id").bind(contact_id).bind(contact.first_name).bind(contact.last_name).bind(contact.phone).bind(contact.company).bind(contact.job_title).bind(contact.status).bind(contact.source).bind(tags).bind(contact.custom_fields).fetch_one(&mut **tx).await.map_err(db)?
            } else {
                created += 1;
                sqlx::query_scalar("INSERT INTO contacts(email,first_name,last_name,phone,company,job_title,status,source,tags,custom_fields,created_at,updated_at) VALUES($1,$2,$3,$4,$5,$6,COALESCE($7,'lead'),$8,$9,$10,NOW() AT TIME ZONE 'UTC',NOW() AT TIME ZONE 'UTC') RETURNING id").bind(contact.email).bind(contact.first_name).bind(contact.last_name).bind(contact.phone).bind(contact.company).bind(contact.job_title).bind(contact.status).bind(contact.source).bind(tags).bind(contact.custom_fields).fetch_one(&mut **tx).await.map_err(db)?
            };
            sqlx::query("INSERT INTO crm_list_members(list_id,contact_id) SELECT UNNEST($1::bigint[]),$2 ON CONFLICT DO NOTHING").bind(&list_ids).bind(saved).execute(&mut **tx).await.map_err(db)?;
        }
        sqlx::query("UPDATE crm_lists SET contacts_count=(SELECT COUNT(*) FROM crm_list_members WHERE list_id=crm_lists.id),updated_at=NOW() WHERE id=ANY($1)").bind(&list_ids).execute(&mut **tx).await.map_err(db)?;
        sqlx::query("UPDATE crm_transfer_jobs SET status='completed',field_mapping=$2,settings=$3,processed_rows=total_rows,success_count=$4,created_count=$5,updated_count=$6,skipped_count=$7,started_at=NOW(),completed_at=NOW(),updated_at=NOW() WHERE id=$1").bind(id).bind(mapping_value).bind(settings_value).bind(created+updated).bind(created).bind(updated).bind(skipped).execute(&mut **tx).await.map_err(db)?;
        crm_logs::record(tx,"info","import","Contact import completed",json!({"job_id":id,"created_count":created,"updated_count":updated,"skipped_count":skipped}),Some(actor.0.id)).await.map_err(db)?;
    }
    let job = public_job(tx, id, "import").await?;
    Ok(job)
}
async fn start_import(
    State(state): State<AppState>,
    admin: AdminUser,
    Path(id): Path<Uuid>,
    Json(input): Json<Start>,
) -> Result<Json<Value>, ApiError> {
    Ok(Json(execute_import(&state, &admin, id, input).await?))
}
async fn cancel_import(
    State(state): State<AppState>,
    admin: AdminUser,
    Path(id): Path<Uuid>,
) -> Result<Json<Value>, ApiError> {
    let mut tx = state.db.pool.begin().await.map_err(db)?;
    let status: Option<String> = sqlx::query_scalar(
        "SELECT status FROM crm_transfer_jobs WHERE id=$1 AND direction='import' FOR UPDATE",
    )
    .bind(id)
    .fetch_optional(&mut *tx)
    .await
    .map_err(db)?;
    match status.as_deref() {
        None => return Err(ApiError::not_found("Import job not found")),
        Some("pending") => {
            sqlx::query("UPDATE crm_transfer_jobs SET status='cancelled',completed_at=NOW(),updated_at=NOW() WHERE id=$1").bind(id).execute(&mut *tx).await.map_err(db)?;
            crm_logs::record(
                &mut tx,
                "info",
                "import",
                "Pending contact import cancelled",
                json!({"job_id":id}),
                Some(admin.0.id),
            )
            .await
            .map_err(db)?;
        }
        Some("cancelled") => {}
        _ => return Err(conflict("An executed import cannot be cancelled")),
    }
    let job = public_job(&mut tx, id, "import").await?;
    tx.commit().await.map_err(db)?;
    Ok(Json(job))
}

#[derive(Default, Deserialize, Serialize)]
#[serde(default, deny_unknown_fields)]
struct ExportFilters {
    status: Option<String>,
    search: Option<String>,
    tag: Option<String>,
    list_id: Option<String>,
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Export {
    #[serde(rename = "type", default = "contacts_kind")]
    kind: String,
    #[serde(default)]
    filters: ExportFilters,
    fields: Option<Vec<String>>,
    #[serde(default = "csv_type")]
    file_type: String,
}
fn csv_type() -> String {
    "csv".into()
}
fn csv_cell(value: &Value) -> String {
    match value {
        Value::Null => String::new(),
        Value::String(text) => {
            if text
                .trim_start_matches(|c: char| c.is_whitespace() || c == '\u{feff}')
                .starts_with(['=', '+', '-', '@'])
                || text.starts_with(['\t', '\r', '\n'])
            {
                format!("'{text}")
            } else {
                text.clone()
            }
        }
        _ => value.to_string(),
    }
}
async fn save_export(
    state: &AppState,
    actor: &AdminUser,
    input: Export,
) -> Result<Value, ApiError> {
    actor.body(&json!({"filters":input.filters}))?;
    supported(&input.kind)?;
    if !["csv", "json"].contains(&input.file_type.as_str()) {
        return Err(invalid("Choose csv or json file_type"));
    }
    let fields = input.fields.unwrap_or_else(|| {
        OUTPUT_FIELDS
            .iter()
            .map(|field| (*field).to_string())
            .collect()
    });
    if fields.is_empty()
        || fields.len() > OUTPUT_FIELDS.len()
        || fields.iter().collect::<BTreeSet<_>>().len() != fields.len()
        || fields
            .iter()
            .any(|field| !OUTPUT_FIELDS.contains(&field.as_str()))
    {
        return Err(invalid("Choose unique supported contact export fields"));
    }
    if [
        &input.filters.status,
        &input.filters.search,
        &input.filters.tag,
    ]
    .iter()
    .filter_map(|value| value.as_ref())
    .any(|value| value.len() > 500 || value.contains('\0'))
    {
        return Err(invalid(
            "Export filter values must be at most 500 bytes without null characters",
        ));
    }
    let list_id = input
        .filters
        .list_id
        .as_ref()
        .map(|value| {
            value
                .parse::<i64>()
                .ok()
                .filter(|id| *id > 0)
                .ok_or_else(|| invalid("Invalid list ID"))
        })
        .transpose()?;
    let mut tx = state.db.pool.begin().await.map_err(db)?;
    if let Some(id) = list_id {
        let found: bool = sqlx::query_scalar("SELECT EXISTS(SELECT 1 FROM crm_lists WHERE id=$1)")
            .bind(id)
            .fetch_one(&mut *tx)
            .await
            .map_err(db)?;
        if !found {
            return Err(ApiError::not_found("Export list not found"));
        }
    }
    let mut query = QueryBuilder::<Postgres>::new("SELECT to_jsonb(c) FROM contacts c WHERE TRUE");
    if let Some(status) = &input.filters.status {
        query.push(" AND status=").push_bind(status);
    }
    if let Some(search) = &input.filters.search {
        query
            .push(" AND (email ILIKE ")
            .push_bind(format!("%{}%", crate::utils::escape_like(search)))
            .push(" OR first_name ILIKE ")
            .push_bind(format!("%{}%", crate::utils::escape_like(search)))
            .push(" OR last_name ILIKE ")
            .push_bind(format!("%{}%", crate::utils::escape_like(search)))
            .push(")");
    }
    if let Some(tag) = &input.filters.tag {
        query
            .push(" AND COALESCE(tags,'[]'::jsonb)?")
            .push_bind(tag);
    }
    if let Some(id) = list_id {
        query.push(" AND EXISTS(SELECT 1 FROM crm_list_members m WHERE m.contact_id=c.id AND m.list_id=").push_bind(id).push(")");
    }
    query.push(" ORDER BY c.id LIMIT 1001");
    let rows: Vec<Value> = query
        .build_query_scalar()
        .fetch_all(&mut *tx)
        .await
        .map_err(db)?;
    if rows.len() > MAX_ROWS {
        return Err(invalid(
            "Export exceeds 1000 contacts; narrow the filters before exporting",
        ));
    }
    let rows: Vec<Value> = rows
        .into_iter()
        .map(|row| {
            Value::Object(
                fields
                    .iter()
                    .map(|field| (field.clone(), row[field].clone()))
                    .collect(),
            )
        })
        .collect();
    let bytes = if input.file_type == "json" {
        serde_json::to_vec_pretty(&rows)
            .map_err(|_| ApiError::internal_error("Unable to serialize contact export"))?
    } else {
        let mut writer = csv::Writer::from_writer(Vec::new());
        writer
            .write_record(&fields)
            .map_err(|_| ApiError::internal_error("Unable to write contact export"))?;
        for row in &rows {
            writer
                .write_record(fields.iter().map(|field| csv_cell(&row[field])))
                .map_err(|_| ApiError::internal_error("Unable to write contact export"))?;
        }
        writer
            .into_inner()
            .map_err(|_| ApiError::internal_error("Unable to finish contact export"))?
    };
    if bytes.len() > 8 * 1024 * 1024 {
        return Err(invalid(
            "Export exceeds 8 MiB; narrow the filters or fields",
        ));
    }
    let id = Uuid::new_v4();
    let file_name = format!("crm-contacts-{id}.{}", input.file_type);
    sqlx::query("INSERT INTO crm_transfer_jobs(id,direction,kind,file_name,file_type,status,filters,fields,total_rows,processed_rows,export_bytes,created_by,started_at,completed_at) VALUES($1,'export',$2,$3,$4,'completed',$5,$6,$7,$7,$8,$9,NOW(),NOW())")
        .bind(id).bind(input.kind).bind(file_name).bind(input.file_type).bind(json!(input.filters)).bind(json!(fields)).bind(rows.len() as i32).bind(bytes).bind(actor.0.id).execute(&mut *tx).await.map_err(db)?;
    crm_logs::record(
        &mut tx,
        "info",
        "import",
        "Contact export snapshot created",
        json!({"job_id":id,"total_records":rows.len(),"fields":fields}),
        Some(actor.0.id),
    )
    .await
    .map_err(db)?;
    let job = public_job(&mut tx, id, "export").await?;
    tx.commit().await.map_err(db)?;
    Ok(job)
}
async fn create_export(
    State(state): State<AppState>,
    admin: AdminUser,
    Json(input): Json<Export>,
) -> Result<Json<Value>, ApiError> {
    Ok(Json(save_export(&state, &admin, input).await?))
}
async fn download(
    State(state): State<AppState>,
    admin: AdminUser,
    Path(id): Path<Uuid>,
) -> Result<Response, ApiError> {
    let mut tx = super::manager_sources::read_transaction(&state.db.pool).await?;
    let filters:Value=sqlx::query_scalar("SELECT filters FROM crm_transfer_jobs WHERE id=$1 AND direction='export' AND status='completed'")
        .bind(id).fetch_optional(&mut *tx).await.map_err(db)?.ok_or_else(||ApiError::not_found("Completed export not found"))?;
    admin.export_record(&json!({"filters":filters}))?;
    let (name,kind,bytes):(String,String,Vec<u8>)=sqlx::query_as("SELECT file_name,file_type,export_bytes FROM crm_transfer_jobs WHERE id=$1 AND direction='export' AND status='completed'").bind(id).fetch_optional(&mut *tx).await.map_err(db)?.ok_or_else(||ApiError::not_found("Completed export not found"))?;
    tx.commit().await.map_err(db)?;
    Ok((
        [
            (
                header::CONTENT_TYPE,
                if kind == "csv" {
                    "text/csv; charset=utf-8"
                } else {
                    "application/json; charset=utf-8"
                },
            ),
            (
                header::CONTENT_DISPOSITION,
                &format!("attachment; filename=\"{name}\""),
            ),
            (header::CACHE_CONTROL, "no-store"),
            (header::X_CONTENT_TYPE_OPTIONS, "nosniff"),
        ],
        bytes,
    )
        .into_response())
}
async fn export_fields(
    _admin: AdminUser,
    Path(kind): Path<String>,
) -> Result<Json<Value>, ApiError> {
    supported(&kind)?;
    Ok(Json(json!(
        OUTPUT_FIELDS
            .iter()
            .map(|field| json!({"field":field,"label":field.replace('_'," ")}))
            .collect::<Vec<_>>()
    )))
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct LegacyImport {
    file_type: String,
    data: Value,
    options: Option<Value>,
}
async fn legacy_import(
    State(state): State<AppState>,
    admin: AdminUser,
    Json(input): Json<LegacyImport>,
) -> Result<Json<Value>, ApiError> {
    let start: Start = serde_json::from_value(input.options.unwrap_or_else(|| json!({})))
        .map_err(|_| invalid("Import options must contain mapping and settings"))?;
    admin.body(&json!({"mapping":start.mapping,"settings":start.settings}))?;
    let mut tx = state.db.pool.begin().await.map_err(db)?;
    let job = save_upload_on(
        &mut tx,
        admin.0.id,
        Upload {
            kind: contacts_kind(),
            file_name: format!("contacts.{}", input.file_type),
            file_type: input.file_type,
            data: input.data,
        },
    )
    .await?;
    let id = job["id"]
        .as_str()
        .and_then(|value| value.parse().ok())
        .ok_or_else(|| ApiError::internal_error("Invalid saved import identity"))?;
    let result = execute_import_on(&mut tx, &admin, id, start).await?;
    tx.commit().await.map_err(db)?;
    Ok(Json(result))
}
pub(super) fn router() -> Router<AppState> {
    Router::new()
        .route("/imports", get(imports).post(create_import))
        .route("/imports/options", get(options))
        .route("/imports/{id}", get(get_import))
        .route("/imports/{id}/preview", get(preview))
        .route("/imports/{id}/start", post(start_import))
        .route("/imports/{id}/cancel", post(cancel_import))
        .route("/exports", get(exports).post(create_export))
        .route("/exports/{id}", get(get_export))
        .route("/exports/{id}/download", get(download))
        .route("/exports/fields/{kind}", get(export_fields))
        .route("/import", post(legacy_import))
        .route("/import/history", get(imports))
        .route("/export", post(create_export))
        .layer(DefaultBodyLimit::max(MAX_BYTES + 65536))
        .layer(tower_http::set_header::SetResponseHeaderLayer::overriding(
            header::CACHE_CONTROL,
            axum::http::HeaderValue::from_static("no-store"),
        ))
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn csv_preserves_quoted_values_and_rejects_duplicate_headers() {
        let upload = Upload {
            kind: contacts_kind(),
            file_name: "contacts.csv".into(),
            file_type: "csv".into(),
            data: json!("email,first_name\r\na@example.test,\"Line one\nLine two, quoted\"\r\n"),
        };
        let parsed = parse(&upload).unwrap();
        assert_eq!(parsed.rows[0]["first_name"], "Line one\nLine two, quoted");
        assert!(
            parse(&Upload {
                data: json!("email,email\na@example.test,b@example.test\n"),
                ..upload
            })
            .is_err()
        );
    }
    #[test]
    fn csv_escapes_formulas_without_converting_numeric_values() {
        for text in [
            "=SUM(1,2)",
            " +cmd",
            "\tvalue",
            "\rvalue",
            "\u{feff}@payload",
        ] {
            assert!(csv_cell(&json!(text)).starts_with('\''));
        }
        assert_eq!(csv_cell(&json!(-42)), "-42");
        assert_eq!(csv_cell(&json!("ordinary")), "ordinary");
    }
}
