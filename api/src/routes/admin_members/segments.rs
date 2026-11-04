//! Admin Members - Segments
//!
//! Member segmentation for targeted marketing.

use crate::middleware::admin::{AdminUser, SuperAdminUser};
use axum::{
    Json,
    extract::{Path, Query, State},
};
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

use crate::AppState;

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct MemberSegment {
    pub id: i64,
    pub name: String,
    pub slug: String,
    pub description: Option<String>,
    pub rules: Option<serde_json::Value>,
    pub member_count: i32,
    pub is_active: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Deserialize)]
pub struct SegmentQuery {
    pub search: Option<String>,
    pub is_active: Option<bool>,
    pub page: Option<i64>,
    pub per_page: Option<i64>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CreateSegmentRequest {
    pub name: String,
    pub slug: Option<String>,
    pub description: Option<String>,
    pub rules: Option<serde_json::Value>,
    pub is_active: Option<bool>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct UpdateSegmentRequest {
    pub name: Option<String>,
    pub slug: Option<String>,
    pub description: Option<String>,
    pub rules: Option<serde_json::Value>,
    pub is_active: Option<bool>,
}

use crate::services::member_segmentation::{self as evaluation, storage};
use crate::utils::errors::{self, ApiError};
use serde_json::{Value, json as value};
use sqlx::PgPool;
macro_rules! segment_sql {
    ($prefix:literal,$suffix:literal)=>{concat!($prefix,"id,name,slug,description,filter_criteria AS rules,COALESCE(member_count,0) AS member_count,COALESCE(is_active,false) AS is_active,(created_at AT TIME ZONE 'UTC') AS created_at,(updated_at AT TIME ZONE 'UTC') AS updated_at",$suffix)};
}
fn validate_text(value: &str, max: usize, required: bool) -> Result<(), ApiError> {
    if value.contains('\0') || value.chars().count() > max || required && value.trim().is_empty() {
        return Err(ApiError::validation_error(
            "Provide bounded nonempty text without null characters",
        ));
    }
    Ok(())
}
fn slug(value: &str) -> Result<String, ApiError> {
    validate_text(value, 255, true)?;
    if !value
        .chars()
        .all(|v| v.is_ascii_lowercase() || v.is_ascii_digit() || v == '-')
    {
        return Err(ApiError::validation_error(
            "Segment slug must contain lowercase letters, numbers or hyphens",
        ));
    }
    Ok(value.into())
}
async fn record(
    tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    actor: i64,
    id: i64,
    action: &str,
) -> Result<(), ApiError> {
    crate::services::crm_logs::record(
        tx,
        "info",
        "system",
        action,
        value!({"member_segment_id":id}),
        Some(actor),
    )
    .await
    .map_err(storage)
}
async fn enriched(pool: &PgPool, row: MemberSegment) -> Result<Value, ApiError> {
    let definition = evaluation::load(pool, row.id, false).await?;
    let count: i64 = evaluation::segment_query("SELECT COUNT(*)", &definition)?
        .build_query_scalar()
        .fetch_one(pool)
        .await
        .map_err(storage)?;
    let mut data = serde_json::to_value(&row)
        .map_err(|_| errors::internal_error("Unable to encode member segment"))?;
    data["member_count"] = value!(count);
    data["memberCount"] = value!(count);
    data["conditions"] = value!(definition.conditions);
    data["type"] = value!(if definition.static_id.is_some() {
        "static"
    } else {
        "smart"
    });
    data["lastUpdated"] = value!(row.updated_at);
    data["isSystem"] = value!(definition.is_system);
    Ok(data)
}
pub(super) async fn list_segments(
    State(state): State<AppState>,
    _admin: AdminUser,
    Query(query): Query<SegmentQuery>,
) -> Result<Json<Value>, ApiError> {
    let page = query.page.unwrap_or(1);
    let size = query.per_page.unwrap_or(50);
    if !(1..=10_000).contains(&page) || !(1..=100).contains(&size) {
        return Err(ApiError::validation_error(
            "Invalid segment page or page size",
        ));
    }
    if let Some(search) = &query.search {
        validate_text(search, 255, false)?;
    }
    let search = query.search.map(|value| format!("%{value}%"));
    let rows:Vec<MemberSegment>=sqlx::query_as(segment_sql!("SELECT "," FROM member_segments WHERE ($1::text IS NULL OR name ILIKE $1 OR description ILIKE $1) AND ($2::bool IS NULL OR is_active=$2) ORDER BY created_at DESC,id DESC LIMIT $3 OFFSET $4"))
    .bind(&search).bind(query.is_active).bind(size).bind((page-1)*size).fetch_all(&state.db.pool).await.map_err(storage)?;
    let total:i64=sqlx::query_scalar("SELECT COUNT(*) FROM member_segments WHERE ($1::text IS NULL OR name ILIKE $1 OR description ILIKE $1) AND ($2::bool IS NULL OR is_active=$2)").bind(search).bind(query.is_active).fetch_one(&state.db.pool).await.map_err(storage)?;
    let mut segments = Vec::with_capacity(rows.len());
    for row in rows {
        segments.push(enriched(&state.db.pool, row).await?);
    }
    Ok(Json(
        value!({"segments":segments,"meta":{"current_page":page,"per_page":size,"total":total,"total_pages":(total+size-1)/size}}),
    ))
}
pub(super) async fn get_segment(
    State(state): State<AppState>,
    _admin: AdminUser,
    Path(id): Path<i64>,
) -> Result<Json<Value>, ApiError> {
    let row: MemberSegment =
        sqlx::query_as(segment_sql!("SELECT ", " FROM member_segments WHERE id=$1"))
            .bind(id)
            .fetch_optional(&state.db.pool)
            .await
            .map_err(storage)?
            .ok_or_else(|| ApiError::not_found("Segment"))?;
    Ok(Json(enriched(&state.db.pool, row).await?))
}
pub(super) async fn create_segment(
    State(state): State<AppState>,
    admin: AdminUser,
    Json(input): Json<CreateSegmentRequest>,
) -> Result<Json<Value>, ApiError> {
    validate_text(&input.name, 255, true)?;
    if let Some(description) = &input.description {
        validate_text(description, 4000, false)?;
    }
    let conditions = evaluation::rules(input.rules)?;
    if conditions.is_empty() {
        return Err(ApiError::validation_error(
            "Add at least one segment condition",
        ));
    }
    let generated = input
        .name
        .to_lowercase()
        .replace(' ', "-")
        .chars()
        .filter(|c| c.is_ascii_alphanumeric() || *c == '-')
        .collect::<String>();
    let selected = slug(&input.slug.unwrap_or_else(|| {
        if generated.is_empty() {
            uuid::Uuid::new_v4().to_string()
        } else {
            generated
        }
    }))?;
    let mut tx = state.db.pool.begin().await.map_err(storage)?;
    let row:MemberSegment=sqlx::query_as(segment_sql!("INSERT INTO member_segments(name,slug,description,filter_criteria,is_active,is_dynamic,created_by) VALUES($1,$2,$3,$4,$5,true,$6) RETURNING ",""))
    .bind(input.name).bind(selected).bind(input.description).bind(value!(conditions)).bind(input.is_active.unwrap_or(true)).bind(admin.0.id).fetch_one(&mut *tx).await.map_err(storage)?;
    record(&mut tx, admin.0.id, row.id, "Member segment created").await?;
    tx.commit().await.map_err(storage)?;
    Ok(Json(enriched(&state.db.pool, row).await?))
}
pub(super) async fn update_segment(
    State(state): State<AppState>,
    admin: AdminUser,
    Path(id): Path<i64>,
    Json(input): Json<UpdateSegmentRequest>,
) -> Result<Json<Value>, ApiError> {
    if let Some(name) = &input.name {
        validate_text(name, 255, true)?;
    }
    if let Some(description) = &input.description {
        validate_text(description, 4000, false)?;
    }
    let selected = input.slug.as_deref().map(slug).transpose()?;
    let conditions = input
        .rules
        .map(|value| evaluation::rules(Some(value)))
        .transpose()?;
    if conditions.as_ref().is_some_and(Vec::is_empty) {
        return Err(ApiError::validation_error(
            "Add at least one segment condition",
        ));
    }
    let mut tx = state.db.pool.begin().await.map_err(storage)?;
    let system: bool =
        sqlx::query_scalar("SELECT is_system FROM member_segments WHERE id=$1 FOR UPDATE")
            .bind(id)
            .fetch_optional(&mut *tx)
            .await
            .map_err(storage)?
            .ok_or_else(|| ApiError::not_found("Segment"))?;
    if system {
        return Err(errors::conflict(
            "System segment definitions cannot be changed; create a custom segment instead",
        ));
    }
    let row:MemberSegment=sqlx::query_as(segment_sql!("UPDATE member_segments SET name=COALESCE($2,name),slug=COALESCE($3,slug),description=COALESCE($4,description),filter_criteria=COALESCE($5,filter_criteria),is_dynamic=CASE WHEN $5::jsonb IS NOT NULL THEN true ELSE is_dynamic END,is_active=COALESCE($6,is_active),updated_at=NOW() WHERE id=$1 RETURNING ",""))
    .bind(id).bind(input.name).bind(selected).bind(input.description).bind(conditions.map(|v|value!(v))).bind(input.is_active).fetch_one(&mut *tx).await.map_err(storage)?;
    record(&mut tx, admin.0.id, id, "Member segment updated").await?;
    tx.commit().await.map_err(storage)?;
    Ok(Json(enriched(&state.db.pool, row).await?))
}
pub(super) async fn delete_segment(
    State(state): State<AppState>,
    admin: SuperAdminUser,
    Path(id): Path<i64>,
) -> Result<Json<Value>, ApiError> {
    let mut tx = state.db.pool.begin().await.map_err(storage)?;
    let system: bool =
        sqlx::query_scalar("SELECT is_system FROM member_segments WHERE id=$1 FOR UPDATE")
            .bind(id)
            .fetch_optional(&mut *tx)
            .await
            .map_err(storage)?
            .ok_or_else(|| ApiError::not_found("Segment"))?;
    if system {
        return Err(errors::conflict("System segments cannot be deleted"));
    }
    sqlx::query("DELETE FROM member_segments WHERE id=$1")
        .bind(id)
        .execute(&mut *tx)
        .await
        .map_err(storage)?;
    record(&mut tx, admin.0.id, id, "Member segment deleted").await?;
    tx.commit().await.map_err(storage)?;
    Ok(Json(value!({"deleted":true,"id":id})))
}
