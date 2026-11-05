//! Admin Members - Saved Filters
//!
//! Saved member filter presets (CRUD).

use crate::middleware::admin::{AdminUser, SuperAdminUser};
use axum::{
    Json,
    extract::{Path, Query, State},
};
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::AppState;

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct MemberFilter {
    pub id: i64,
    pub name: String,
    pub description: Option<String>,
    pub filters: serde_json::Value,
    pub is_default: bool,
    pub is_public: bool,
    pub created_by: Option<i64>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Deserialize)]
pub struct MemberFilterQuery {
    pub search: Option<String>,
    pub is_public: Option<bool>,
    pub page: Option<i64>,
    pub per_page: Option<i64>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CreateMemberFilterRequest {
    pub name: String,
    pub description: Option<String>,
    pub filters: serde_json::Value,
    pub is_default: Option<bool>,
    pub is_public: Option<bool>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct UpdateMemberFilterRequest {
    pub name: Option<String>,
    pub description: Option<String>,
    pub filters: Option<serde_json::Value>,
    pub is_default: Option<bool>,
    pub is_public: Option<bool>,
}

use crate::{
    services::member_configuration::{self as validation, storage},
    utils::errors::ApiError,
};
use serde_json::Value;
#[derive(Debug, Serialize, sqlx::FromRow)]
pub(super) struct StoredFilter {
    #[serde(flatten)]
    #[sqlx(flatten)]
    filter: MemberFilter,
    usage_count: Option<i64>,
    last_applied_at: Option<chrono::DateTime<chrono::Utc>>,
}
macro_rules! filter_sql {
    ($prefix:literal,$suffix:literal)=>{concat!($prefix,"id,name,description,filter_config AS filters,COALESCE(is_default,false) AS is_default,COALESCE(is_public,false) AS is_public,created_by,(created_at AT TIME ZONE 'UTC') AS created_at,(updated_at AT TIME ZONE 'UTC') AS updated_at,usage_count,last_applied_at",$suffix)};
}
pub(super) async fn list_member_filters(
    State(state): State<AppState>,
    _admin: AdminUser,
    Query(input): Query<MemberFilterQuery>,
) -> Result<Json<Value>, ApiError> {
    let page = input.page.unwrap_or(1);
    let size = input.per_page.unwrap_or(50);
    if !(1..=10000).contains(&page) || !(1..=100).contains(&size) {
        return Err(ApiError::validation_error(
            "Invalid saved member filter page",
        ));
    }
    if let Some(search) = &input.search {
        validation::text(search, 255, false)?;
    }
    let search = input.search.map(|v| format!("%{v}%"));
    let rows:Vec<StoredFilter>=sqlx::query_as(filter_sql!("SELECT "," FROM member_filters WHERE ($1::text IS NULL OR name ILIKE $1 OR description ILIKE $1) AND ($2::bool IS NULL OR is_public=$2) ORDER BY is_default DESC NULLS LAST,name,id LIMIT $3 OFFSET $4"))
        .bind(&search).bind(input.is_public).bind(size).bind((page-1)*size).fetch_all(&state.db.pool).await.map_err(storage)?;
    let total:i64=sqlx::query_scalar("SELECT COUNT(*) FROM member_filters WHERE ($1::text IS NULL OR name ILIKE $1 OR description ILIKE $1) AND ($2::bool IS NULL OR is_public=$2)").bind(search).bind(input.is_public).fetch_one(&state.db.pool).await.map_err(storage)?;
    Ok(Json(
        json!({"filters":rows,"meta":{"current_page":page,"per_page":size,"total":total,"total_pages":(total+size-1)/size}}),
    ))
}
pub(super) async fn get_member_filter(
    State(state): State<AppState>,
    _admin: AdminUser,
    Path(id): Path<i64>,
) -> Result<Json<StoredFilter>, ApiError> {
    let row = sqlx::query_as(filter_sql!("SELECT ", " FROM member_filters WHERE id=$1"))
        .bind(id)
        .fetch_optional(&state.db.pool)
        .await
        .map_err(storage)?
        .ok_or_else(|| ApiError::not_found("Saved member filter"))?;
    Ok(Json(row))
}
pub(super) async fn create_member_filter(
    State(state): State<AppState>,
    admin: AdminUser,
    Json(input): Json<CreateMemberFilterRequest>,
) -> Result<Json<StoredFilter>, ApiError> {
    validation::text(&input.name, 255, true)?;
    if let Some(description) = &input.description {
        validation::text(description, 4000, false)?;
    }
    let filters = validation::filters(&state.db.pool, &input.filters).await?;
    let slug = validation::generated_slug(&input.name);
    let mut tx = state.db.pool.begin().await.map_err(storage)?;
    let row:StoredFilter=sqlx::query_as(filter_sql!("INSERT INTO member_filters(name,slug,description,filter_config,is_default,is_public,created_by) VALUES($1,$2,$3,$4,$5,$6,$7) RETURNING ",""))
        .bind(input.name).bind(slug).bind(input.description).bind(filters).bind(input.is_default.unwrap_or(false)).bind(input.is_public.unwrap_or(true)).bind(admin.0.id).fetch_one(&mut *tx).await.map_err(storage)?;
    validation::audit(
        &mut tx,
        admin.0.id,
        "Saved member filter created",
        json!({"member_filter_id":row.filter.id}),
    )
    .await?;
    tx.commit().await.map_err(storage)?;
    Ok(Json(row))
}
pub(super) async fn update_member_filter(
    State(state): State<AppState>,
    admin: AdminUser,
    Path(id): Path<i64>,
    Json(input): Json<UpdateMemberFilterRequest>,
) -> Result<Json<StoredFilter>, ApiError> {
    if let Some(name) = &input.name {
        validation::text(name, 255, true)?;
    }
    if let Some(description) = &input.description {
        validation::text(description, 4000, false)?;
    }
    let filters = if let Some(filters) = input.filters {
        Some(validation::filters(&state.db.pool, &filters).await?)
    } else {
        None
    };
    let mut tx = state.db.pool.begin().await.map_err(storage)?;
    let row:StoredFilter=sqlx::query_as(filter_sql!("UPDATE member_filters SET name=COALESCE($2,name),description=COALESCE($3,description),filter_config=COALESCE($4,filter_config),is_default=COALESCE($5,is_default),is_public=COALESCE($6,is_public),updated_at=NOW() WHERE id=$1 RETURNING ",""))
        .bind(id).bind(input.name).bind(input.description).bind(filters).bind(input.is_default).bind(input.is_public).fetch_optional(&mut *tx).await.map_err(storage)?.ok_or_else(||ApiError::not_found("Saved member filter"))?;
    validation::audit(
        &mut tx,
        admin.0.id,
        "Saved member filter updated",
        json!({"member_filter_id":id}),
    )
    .await?;
    tx.commit().await.map_err(storage)?;
    Ok(Json(row))
}
pub(super) async fn delete_member_filter(
    State(state): State<AppState>,
    admin: SuperAdminUser,
    Path(id): Path<i64>,
) -> Result<Json<Value>, ApiError> {
    let mut tx = state.db.pool.begin().await.map_err(storage)?;
    let result = sqlx::query("DELETE FROM member_filters WHERE id=$1")
        .bind(id)
        .execute(&mut *tx)
        .await
        .map_err(storage)?;
    if result.rows_affected() != 1 {
        return Err(ApiError::not_found("Saved member filter"));
    }
    validation::audit(
        &mut tx,
        admin.0.id,
        "Saved member filter deleted",
        json!({"member_filter_id":id}),
    )
    .await?;
    tx.commit().await.map_err(storage)?;
    Ok(Json(json!({"deleted":true,"id":id})))
}
pub(super) async fn apply_member_filter(
    State(state): State<AppState>,
    admin: AdminUser,
    Path(id): Path<i64>,
) -> Result<Json<Value>, ApiError> {
    let mut tx = state.db.pool.begin().await.map_err(storage)?;
    let stored: Value =
        sqlx::query_scalar("SELECT filter_config FROM member_filters WHERE id=$1 FOR UPDATE")
            .bind(id)
            .fetch_optional(&mut *tx)
            .await
            .map_err(storage)?
            .ok_or_else(|| ApiError::not_found("Saved member filter"))?;
    let filters = validation::filters_on(&mut tx, &stored).await?;
    let count:i64=sqlx::query_scalar("UPDATE member_filters SET usage_count=COALESCE(usage_count,0)+1,last_applied_at=NOW() WHERE id=$1 RETURNING usage_count").bind(id).fetch_one(&mut *tx).await.map_err(storage)?;
    validation::audit(
        &mut tx,
        admin.0.id,
        "Saved member filter apply recorded",
        json!({"member_filter_id":id,"recorded_applies":count}),
    )
    .await?;
    tx.commit().await.map_err(storage)?;
    Ok(Json(
        json!({"applied":true,"id":id,"filters":filters,"usage_count":count}),
    ))
}
pub(super) async fn duplicate_member_filter(
    State(state): State<AppState>,
    admin: AdminUser,
    Path(id): Path<i64>,
) -> Result<Json<StoredFilter>, ApiError> {
    let mut tx = state.db.pool.begin().await.map_err(storage)?;
    let original: StoredFilter = sqlx::query_as(filter_sql!(
        "SELECT ",
        " FROM member_filters WHERE id=$1 FOR SHARE"
    ))
    .bind(id)
    .fetch_optional(&mut *tx)
    .await
    .map_err(storage)?
    .ok_or_else(|| ApiError::not_found("Saved member filter"))?;
    let filters = validation::filters_on(&mut tx, &original.filter.filters).await?;
    let name = format!(
        "{} (Copy)",
        original.filter.name.chars().take(248).collect::<String>()
    );
    let slug = format!("filter-copy-{}", uuid::Uuid::new_v4());
    let copy:StoredFilter=sqlx::query_as(filter_sql!("INSERT INTO member_filters(name,slug,description,filter_config,is_default,is_public,created_by) VALUES($1,$2,$3,$4,false,$5,$6) RETURNING ",""))
        .bind(name).bind(slug).bind(original.filter.description).bind(filters).bind(original.filter.is_public).bind(admin.0.id).fetch_one(&mut *tx).await.map_err(storage)?;
    validation::audit(
        &mut tx,
        admin.0.id,
        "Saved member filter duplicated",
        json!({"member_filter_id":copy.filter.id,"original_filter_id":id}),
    )
    .await?;
    tx.commit().await.map_err(storage)?;
    Ok(Json(copy))
}
