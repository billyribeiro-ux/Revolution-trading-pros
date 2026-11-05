//! Dynamic segments evaluate validated rules against current contact records.
use super::{
    contacts::CrmContact,
    segment_rules::{self, Condition},
};
use crate::{
    AppState, routes::crm::manager_permissions::CrmUser as AdminUser, utils::errors::ApiError,
};
use axum::{
    Json, Router,
    extract::{Path, Query, State},
    routing::{get, post},
};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use sqlx::{FromRow, PgConnection};

macro_rules! fields {()=>{"id,COALESCE(title,name) AS title,COALESCE(slug,'') AS slug,description,conditions,COALESCE(match_type,'all') AS match_type,COALESCE(contacts_count,0) AS contacts_count,COALESCE(is_active,true) AS is_active,last_synced_at,created_at,updated_at"};}
#[derive(Debug, Clone, Serialize, FromRow)]
pub struct ContactSegment {
    pub id: i64,
    pub title: String,
    pub slug: String,
    pub description: Option<String>,
    pub conditions: Option<Value>,
    pub match_type: String,
    pub contacts_count: Option<i32>,
    #[sqlx(skip)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validation_error: Option<String>,
    pub is_active: bool,
    pub last_synced_at: Option<chrono::DateTime<chrono::Utc>>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}
#[derive(Deserialize)]
struct SegmentInput {
    #[serde(alias = "name")]
    title: String,
    description: Option<String>,
    #[serde(default)]
    conditions: Vec<Condition>,
    #[serde(default = "all")]
    match_type: String,
    #[serde(default = "active")]
    is_active: bool,
}
fn all() -> String {
    "all".into()
}
fn active() -> bool {
    true
}
#[derive(Deserialize, Default)]
struct Filters {
    search: Option<String>,
    is_active: Option<bool>,
    page: Option<i64>,
    per_page: Option<i64>,
    #[serde(default)]
    definition: bool,
}
fn db_error(error: sqlx::Error) -> ApiError {
    ApiError::database_error(&error.to_string())
}
fn invalid(message: &str) -> ApiError {
    ApiError::validation_error(message)
}
impl SegmentInput {
    fn validate(&mut self) -> Result<(), ApiError> {
        self.title = self.title.trim().to_owned();
        if self.title.is_empty()
            || self.title.chars().count() > 255
            || self.title.contains('\0')
            || self
                .description
                .as_ref()
                .is_some_and(|s| s.len() > 50000 || s.contains('\0'))
        {
            return Err(invalid(
                "Provide a title up to 255 characters and description up to 50 KiB",
            ));
        }
        let _ = segment_rules::query(
            "SELECT COUNT(*) FROM contacts c",
            &self.conditions,
            &self.match_type,
        )?;
        Ok(())
    }
}
async fn load(
    connection: &mut PgConnection,
    id: i64,
    lock: bool,
) -> Result<ContactSegment, ApiError> {
    sqlx::query_as(if lock {
        concat!(
            "SELECT ",
            fields!(),
            " FROM crm_segments WHERE id=$1 FOR UPDATE"
        )
    } else {
        concat!("SELECT ", fields!(), " FROM crm_segments WHERE id=$1")
    })
    .bind(id)
    .fetch_optional(connection)
    .await
    .map_err(db_error)?
    .ok_or_else(|| ApiError::not_found("Segment not found"))
}
fn rules(segment: &ContactSegment) -> Result<Vec<Condition>, ApiError> {
    serde_json::from_value(segment.conditions.clone().unwrap_or_else(|| json!([]))).map_err(|_| {
        invalid("Stored segment rules have an unsupported format; update the segment rules")
    })
}
async fn refresh(
    connection: &mut PgConnection,
    mut segment: ContactSegment,
    persist: bool,
) -> Result<ContactSegment, ApiError> {
    let rules = rules(&segment)?;
    let mut query = segment_rules::query(
        "SELECT COUNT(*) FROM contacts c",
        &rules,
        &segment.match_type,
    )?;
    let count: i64 = query
        .build_query_scalar()
        .fetch_one(&mut *connection)
        .await
        .map_err(db_error)?;
    segment.contacts_count = Some(
        count
            .try_into()
            .map_err(|_| invalid("Segment count exceeds the supported range"))?,
    );
    if persist {
        segment=sqlx::query_as(concat!("UPDATE crm_segments SET contacts_count=$2,member_count=$2,last_synced_at=NOW(),updated_at=NOW() WHERE id=$1 RETURNING ",fields!()))
            .bind(segment.id).bind(segment.contacts_count).fetch_one(&mut *connection).await.map_err(db_error)?;
    }
    segment.conditions =
        Some(serde_json::to_value(&rules).map_err(|_| invalid("Invalid segment rules"))?);
    Ok(segment)
}
async fn read_current(
    connection: &mut PgConnection,
    segment: ContactSegment,
) -> Result<ContactSegment, ApiError> {
    match refresh(connection, segment.clone(), false).await {
        Ok(segment) => Ok(segment),
        Err(error) if error.status == "400" => {
            let mut segment = segment;
            segment.contacts_count = None;
            segment.validation_error = Some(error.message);
            Ok(segment)
        }
        Err(error) => Err(error),
    }
}
async fn save(
    connection: &mut PgConnection,
    id: Option<i64>,
    mut input: SegmentInput,
) -> Result<ContactSegment, ApiError> {
    input.validate()?;
    let conditions =
        serde_json::to_value(&input.conditions).map_err(|_| invalid("Invalid segment rules"))?;
    let segment = if let Some(id) = id {
        sqlx::query_as(concat!("UPDATE crm_segments SET name=$2,title=$2,description=$3,conditions=$4,match_type=$5,is_active=$6,updated_at=NOW() WHERE id=$1 RETURNING ",fields!()))
            .bind(id).bind(&input.title).bind(&input.description).bind(&conditions).bind(&input.match_type).bind(input.is_active).fetch_one(&mut *connection).await.map_err(db_error)?
    } else {
        let slug = format!("segment-{}", uuid::Uuid::new_v4());
        sqlx::query_as(concat!("INSERT INTO crm_segments(name,title,slug,description,conditions,match_type,is_active) VALUES($1,$1,$2,$3,$4,$5,$6) RETURNING ",fields!()))
            .bind(&input.title).bind(slug).bind(&input.description).bind(&conditions).bind(&input.match_type).bind(input.is_active).fetch_one(&mut *connection).await.map_err(db_error)?
    };
    refresh(connection, segment, true).await
}
async fn list_segments(
    State(state): State<AppState>,
    _admin: AdminUser,
    Query(filters): Query<Filters>,
) -> Result<Json<Value>, ApiError> {
    let page = filters.page.unwrap_or(1).max(1);
    let per_page = filters.per_page.unwrap_or(25).clamp(1, 100);
    let search = filters.search.map(|s| format!("%{}%", s.replace('\0', "")));
    let records:Vec<ContactSegment>=sqlx::query_as(concat!("SELECT ",fields!()," FROM crm_segments WHERE ($1::text IS NULL OR COALESCE(title,name) ILIKE $1 OR description ILIKE $1) AND ($2::bool IS NULL OR COALESCE(is_active,true)=$2) ORDER BY created_at DESC,id DESC LIMIT $3 OFFSET $4"))
        .bind(&search).bind(filters.is_active).bind(per_page).bind(page.saturating_sub(1).saturating_mul(per_page)).fetch_all(&state.db.pool).await.map_err(db_error)?;
    let (total,active):(i64,i64)=sqlx::query_as("SELECT COUNT(*),COUNT(*) FILTER(WHERE COALESCE(is_active,true)) FROM crm_segments WHERE ($1::text IS NULL OR COALESCE(title,name) ILIKE $1 OR description ILIKE $1) AND ($2::bool IS NULL OR COALESCE(is_active,true)=$2)")
        .bind(&search).bind(filters.is_active).fetch_one(&state.db.pool).await.map_err(db_error)?;
    let mut connection = state.db.pool.acquire().await.map_err(db_error)?;
    let mut result = Vec::new();
    for record in records {
        result.push(read_current(&mut connection, record).await?);
    }
    Ok(Json(
        json!({"data":result,"meta":{"total":total,"active_total":active,"current_page":page,"per_page":per_page,"total_pages":((total+per_page-1)/per_page).max(1)}}),
    ))
}
async fn create_segment(
    State(state): State<AppState>,
    _admin: AdminUser,
    Json(input): Json<SegmentInput>,
) -> Result<Json<ContactSegment>, ApiError> {
    let mut tx = state.db.pool.begin().await.map_err(db_error)?;
    let segment = save(&mut tx, None, input).await?;
    tx.commit().await.map_err(db_error)?;
    Ok(Json(segment))
}
async fn update_segment(
    State(state): State<AppState>,
    _admin: AdminUser,
    Path(id): Path<i64>,
    Json(patch): Json<Value>,
) -> Result<Json<ContactSegment>, ApiError> {
    let mut tx = state.db.pool.begin().await.map_err(db_error)?;
    let segment = load(&mut tx, id, true).await?;
    let mut merged = json!({"title":segment.title,"description":segment.description,"conditions":segment.conditions.unwrap_or_else(||json!([])),"match_type":segment.match_type,"is_active":segment.is_active});
    let patch = patch
        .as_object()
        .filter(|m| !m.is_empty())
        .ok_or_else(|| invalid("Provide segment fields to update"))?;
    for (key, value) in patch {
        let key = if key == "name" { "title" } else { key.as_str() };
        if !matches!(
            key,
            "title" | "description" | "conditions" | "match_type" | "is_active"
        ) {
            return Err(invalid("Unsupported segment update field"));
        }
        merged[key] = value.clone();
    }
    let input =
        serde_json::from_value(merged).map_err(|_| invalid("Invalid segment field types"))?;
    let segment = save(&mut tx, Some(id), input).await?;
    tx.commit().await.map_err(db_error)?;
    Ok(Json(segment))
}
async fn get_segment(
    State(state): State<AppState>,
    _admin: AdminUser,
    Path(id): Path<i64>,
    Query(filters): Query<Filters>,
) -> Result<Json<ContactSegment>, ApiError> {
    let mut connection = state.db.pool.acquire().await.map_err(db_error)?;
    let segment = load(&mut connection, id, false).await?;
    if filters.definition {
        return Ok(Json(segment));
    }
    Ok(Json(read_current(&mut connection, segment).await?))
}
async fn sync_segment(
    State(state): State<AppState>,
    _admin: AdminUser,
    Path(id): Path<i64>,
) -> Result<Json<ContactSegment>, ApiError> {
    let mut tx = state.db.pool.begin().await.map_err(db_error)?;
    let segment = load(&mut tx, id, true).await?;
    let segment = refresh(&mut tx, segment, true).await?;
    tx.commit().await.map_err(db_error)?;
    Ok(Json(segment))
}
async fn duplicate_segment(
    State(state): State<AppState>,
    _admin: AdminUser,
    Path(id): Path<i64>,
) -> Result<Json<ContactSegment>, ApiError> {
    let mut tx = state.db.pool.begin().await.map_err(db_error)?;
    let segment = load(&mut tx, id, false).await?;
    let conditions = rules(&segment)?;
    let input = SegmentInput {
        title: format!(
            "{} (Copy)",
            segment.title.chars().take(248).collect::<String>()
        ),
        description: segment.description,
        conditions,
        match_type: segment.match_type,
        is_active: false,
    };
    let copy = save(&mut tx, None, input).await?;
    tx.commit().await.map_err(db_error)?;
    Ok(Json(copy))
}
async fn delete_segment(
    State(state): State<AppState>,
    _admin: AdminUser,
    Path(id): Path<i64>,
) -> Result<Json<Value>, ApiError> {
    if sqlx::query("DELETE FROM crm_segments WHERE id=$1")
        .bind(id)
        .execute(&state.db.pool)
        .await
        .map_err(db_error)?
        .rows_affected()
        == 0
    {
        return Err(ApiError::not_found("Segment not found"));
    }
    Ok(Json(json!({"message":"Segment deleted"})))
}
async fn contacts(
    State(state): State<AppState>,
    _admin: AdminUser,
    Path(id): Path<i64>,
    Query(filters): Query<Filters>,
) -> Result<Json<Value>, ApiError> {
    let mut connection = state.db.pool.acquire().await.map_err(db_error)?;
    let segment = load(&mut connection, id, false).await?;
    let rules = rules(&segment)?;
    let mut count_query = segment_rules::query(
        "SELECT COUNT(*) FROM contacts c",
        &rules,
        &segment.match_type,
    )?;
    let total: i64 = count_query
        .build_query_scalar()
        .fetch_one(&mut *connection)
        .await
        .map_err(db_error)?;
    let page = filters.page.unwrap_or(1).max(1);
    let per_page = filters.per_page.unwrap_or(25).clamp(1, 100);
    let mut query = segment_rules::query(
        "SELECT c.id,c.email,c.first_name,c.last_name,c.phone,c.company,c.job_title,c.status,c.source,c.score,c.tags,c.custom_fields,c.last_contacted_at,c.created_at,c.updated_at FROM contacts c",
        &rules,
        &segment.match_type,
    )?;
    query
        .push(" ORDER BY c.created_at DESC,c.id DESC LIMIT ")
        .push_bind(per_page)
        .push(" OFFSET ")
        .push_bind(page.saturating_sub(1).saturating_mul(per_page));
    let contacts: Vec<CrmContact> = query
        .build_query_as()
        .fetch_all(&mut *connection)
        .await
        .map_err(db_error)?;
    Ok(Json(
        json!({"data":contacts,"meta":{"total":total,"current_page":page,"per_page":per_page,"total_pages":((total+per_page-1)/per_page).max(1)}}),
    ))
}
pub(super) fn router() -> Router<AppState> {
    Router::new()
        .route("/segments", get(list_segments).post(create_segment))
        .route(
            "/segments/{id}",
            get(get_segment).put(update_segment).delete(delete_segment),
        )
        .route("/segments/{id}/contacts", get(contacts))
        .route("/segments/{id}/sync", post(sync_segment))
        .route("/segments/{id}/duplicate", post(duplicate_segment))
}
