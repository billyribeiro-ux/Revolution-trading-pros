//! Admin Members - Tags
//!
//! Member tagging system: CRUD + per-user assign/unassign + bulk-assign.

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
pub struct MemberTag {
    pub id: i64,
    pub name: String,
    pub slug: String,
    pub color: Option<String>,
    pub description: Option<String>,
    pub member_count: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Deserialize)]
pub struct MemberTagQuery {
    pub search: Option<String>,
    pub page: Option<i64>,
    pub per_page: Option<i64>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CreateMemberTagRequest {
    pub name: String,
    pub slug: Option<String>,
    pub color: Option<String>,
    pub description: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct UpdateMemberTagRequest {
    pub name: Option<String>,
    pub slug: Option<String>,
    pub color: Option<String>,
    pub description: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AssignTagRequest {
    pub user_id: i64,
    pub tag_id: i64,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct BulkAssignTagsRequest {
    pub user_ids: Vec<i64>,
    pub tag_ids: Vec<i64>,
}

use crate::{
    services::member_configuration::{self as validation, storage},
    utils::errors::ApiError,
};
use serde_json::Value;
use sqlx::PgPool;
macro_rules! tag_sql {
    ($prefix:literal,$suffix:literal)=>{concat!($prefix,"id,name,slug,color,description,(SELECT COUNT(*)::integer FROM user_member_tags assignment JOIN users member ON member.id=assignment.user_id WHERE assignment.tag_id=member_tags.id AND member.deleted_at IS NULL AND COALESCE(LOWER(member.role),'')<>'deleted') AS member_count,(created_at AT TIME ZONE 'UTC') AS created_at,(updated_at AT TIME ZONE 'UTC') AS updated_at",$suffix)};
}
fn validate(
    name: Option<&str>,
    slug: Option<&str>,
    color: Option<&str>,
    description: Option<&str>,
) -> Result<(), ApiError> {
    if let Some(name) = name {
        validation::text(name, 255, true)?;
    }
    if let Some(slug) = slug {
        validation::slug(slug)?;
    }
    if let Some(color) = color {
        validation::color(color)?;
    }
    if let Some(description) = description {
        validation::text(description, 4000, false)?;
    }
    Ok(())
}
pub(super) async fn list_member_tags(
    State(state): State<AppState>,
    _admin: AdminUser,
    Query(input): Query<MemberTagQuery>,
) -> Result<Json<Value>, ApiError> {
    let page = input.page.unwrap_or(1);
    let size = input.per_page.unwrap_or(50);
    if !(1..=10000).contains(&page) || !(1..=100).contains(&size) {
        return Err(ApiError::validation_error("Invalid member tag page"));
    }
    if let Some(search) = &input.search {
        validation::text(search, 255, false)?;
    }
    let search = input.search.map(|v| format!("%{v}%"));
    let rows:Vec<MemberTag>=sqlx::query_as(tag_sql!("SELECT "," FROM member_tags WHERE ($1::text IS NULL OR name ILIKE $1 OR description ILIKE $1) ORDER BY name,id LIMIT $2 OFFSET $3"))
        .bind(&search).bind(size).bind((page-1)*size).fetch_all(&state.db.pool).await.map_err(storage)?;
    let total:i64=sqlx::query_scalar("SELECT COUNT(*) FROM member_tags WHERE ($1::text IS NULL OR name ILIKE $1 OR description ILIKE $1)").bind(search).fetch_one(&state.db.pool).await.map_err(storage)?;
    Ok(Json(
        json!({"tags":rows,"meta":{"current_page":page,"per_page":size,"total":total,"total_pages":(total+size-1)/size}}),
    ))
}
pub(super) async fn get_member_tag(
    State(state): State<AppState>,
    _admin: AdminUser,
    Path(id): Path<i64>,
) -> Result<Json<MemberTag>, ApiError> {
    let row = sqlx::query_as(tag_sql!("SELECT ", " FROM member_tags WHERE id=$1"))
        .bind(id)
        .fetch_optional(&state.db.pool)
        .await
        .map_err(storage)?
        .ok_or_else(|| ApiError::not_found("Member tag"))?;
    Ok(Json(row))
}
pub(super) async fn create_member_tag(
    State(state): State<AppState>,
    admin: AdminUser,
    Json(input): Json<CreateMemberTagRequest>,
) -> Result<Json<MemberTag>, ApiError> {
    validate(
        Some(&input.name),
        input.slug.as_deref(),
        input.color.as_deref(),
        input.description.as_deref(),
    )?;
    let slug = input
        .slug
        .unwrap_or_else(|| validation::generated_slug(&input.name));
    let mut tx = state.db.pool.begin().await.map_err(storage)?;
    let row:MemberTag=sqlx::query_as(tag_sql!("INSERT INTO member_tags(name,slug,color,description) VALUES($1,$2,COALESCE($3,'#10b981'),$4) RETURNING ",""))
        .bind(input.name).bind(slug).bind(input.color).bind(input.description).fetch_one(&mut *tx).await.map_err(storage)?;
    validation::audit(
        &mut tx,
        admin.0.id,
        "Member tag created",
        json!({"member_tag_id":row.id}),
    )
    .await?;
    tx.commit().await.map_err(storage)?;
    Ok(Json(row))
}
pub(super) async fn update_member_tag(
    State(state): State<AppState>,
    admin: AdminUser,
    Path(id): Path<i64>,
    Json(input): Json<UpdateMemberTagRequest>,
) -> Result<Json<MemberTag>, ApiError> {
    validate(
        input.name.as_deref(),
        input.slug.as_deref(),
        input.color.as_deref(),
        input.description.as_deref(),
    )?;
    let mut tx = state.db.pool.begin().await.map_err(storage)?;
    let row:MemberTag=sqlx::query_as(tag_sql!("UPDATE member_tags SET name=COALESCE($2,name),slug=COALESCE($3,slug),color=COALESCE($4,color),description=COALESCE($5,description),updated_at=NOW() WHERE id=$1 RETURNING ",""))
        .bind(id).bind(input.name).bind(input.slug).bind(input.color).bind(input.description).fetch_optional(&mut *tx).await.map_err(storage)?.ok_or_else(||ApiError::not_found("Member tag"))?;
    validation::audit(
        &mut tx,
        admin.0.id,
        "Member tag updated",
        json!({"member_tag_id":id}),
    )
    .await?;
    tx.commit().await.map_err(storage)?;
    Ok(Json(row))
}
pub(super) async fn delete_member_tag(
    State(state): State<AppState>,
    admin: SuperAdminUser,
    Path(id): Path<i64>,
) -> Result<Json<Value>, ApiError> {
    let mut tx = state.db.pool.begin().await.map_err(storage)?;
    sqlx::query("SELECT id FROM member_tags WHERE id=$1 FOR UPDATE")
        .bind(id)
        .fetch_optional(&mut *tx)
        .await
        .map_err(storage)?
        .ok_or_else(|| ApiError::not_found("Member tag"))?;
    let removed = sqlx::query("DELETE FROM user_member_tags WHERE tag_id=$1")
        .bind(id)
        .execute(&mut *tx)
        .await
        .map_err(storage)?
        .rows_affected();
    sqlx::query("DELETE FROM member_tags WHERE id=$1")
        .bind(id)
        .execute(&mut *tx)
        .await
        .map_err(storage)?;
    validation::audit(
        &mut tx,
        admin.0.id,
        "Member tag deleted",
        json!({"member_tag_id":id,"removed_assignments":removed}),
    )
    .await?;
    tx.commit().await.map_err(storage)?;
    Ok(Json(
        json!({"deleted":true,"id":id,"removed_assignments":removed}),
    ))
}
fn ids(mut ids: Vec<i64>, max: usize) -> Result<Vec<i64>, ApiError> {
    if ids.is_empty() || ids.len() > max || ids.iter().any(|id| *id <= 0) {
        return Err(ApiError::validation_error(
            "Provide a bounded nonempty list of positive member and tag identifiers",
        ));
    }
    ids.sort_unstable();
    ids.dedup();
    Ok(ids)
}
async fn assign(
    pool: &PgPool,
    actor: i64,
    users: Vec<i64>,
    tags: Vec<i64>,
) -> Result<Value, ApiError> {
    let users = ids(users, 1000)?;
    let tags = ids(tags, 50)?;
    let mut tx = pool.begin().await.map_err(storage)?;
    let found: Vec<i64> =
        sqlx::query_scalar("SELECT id FROM member_tags WHERE id=ANY($1) ORDER BY id FOR UPDATE")
            .bind(&tags)
            .fetch_all(&mut *tx)
            .await
            .map_err(storage)?;
    if found != tags {
        return Err(ApiError::not_found("Member tag"));
    }
    let count: i64 =
        sqlx::query_scalar("SELECT COUNT(*) FROM users WHERE id=ANY($1) AND deleted_at IS NULL AND COALESCE(LOWER(role),'')<>'deleted'")
            .bind(&users)
            .fetch_one(&mut *tx)
            .await
            .map_err(storage)?;
    if count != users.len() as i64 {
        return Err(ApiError::not_found("Member"));
    }
    let assigned=sqlx::query("INSERT INTO user_member_tags(user_id,tag_id,added_by) SELECT member_id,tag_id,$3 FROM unnest($1::bigint[]) member_id CROSS JOIN unnest($2::bigint[]) tag_id ON CONFLICT(user_id,tag_id) DO NOTHING").bind(&users).bind(&tags).bind(actor).execute(&mut *tx).await.map_err(storage)?.rows_affected();
    sqlx::query("UPDATE member_tags tag SET usage_count=(SELECT COUNT(*)::integer FROM user_member_tags assignment JOIN users member ON member.id=assignment.user_id WHERE assignment.tag_id=tag.id AND member.deleted_at IS NULL AND COALESCE(LOWER(member.role),'')<>'deleted'),updated_at=NOW() WHERE tag.id=ANY($1)").bind(&tags).execute(&mut *tx).await.map_err(storage)?;
    let rows: Vec<MemberTag> = sqlx::query_as(tag_sql!(
        "SELECT ",
        " FROM member_tags WHERE id=ANY($1) ORDER BY id"
    ))
    .bind(&tags)
    .fetch_all(&mut *tx)
    .await
    .map_err(storage)?;
    let requested = users.len() * tags.len();
    validation::audit(
        &mut tx,
        actor,
        "Member tag assignments recorded",
        json!({"member_ids":users,"member_tag_ids":tags,"assigned":assigned,"requested":requested}),
    )
    .await?;
    tx.commit().await.map_err(storage)?;
    Ok(
        json!({"message":"Member tag assignments recorded","assigned":assigned,"already_assigned":requested as u64-assigned,"requested":requested,"tags":rows}),
    )
}
pub(super) async fn assign_tag_to_user(
    State(state): State<AppState>,
    admin: AdminUser,
    Json(input): Json<AssignTagRequest>,
) -> Result<Json<Value>, ApiError> {
    Ok(Json(
        assign(
            &state.db.pool,
            admin.0.id,
            vec![input.user_id],
            vec![input.tag_id],
        )
        .await?,
    ))
}
pub(super) async fn bulk_assign_tags(
    State(state): State<AppState>,
    admin: AdminUser,
    Json(input): Json<BulkAssignTagsRequest>,
) -> Result<Json<Value>, ApiError> {
    Ok(Json(
        assign(&state.db.pool, admin.0.id, input.user_ids, input.tag_ids).await?,
    ))
}
pub(super) async fn unassign_tag_from_user(
    State(state): State<AppState>,
    admin: AdminUser,
    Json(input): Json<AssignTagRequest>,
) -> Result<Json<Value>, ApiError> {
    ids(vec![input.user_id], 1)?;
    ids(vec![input.tag_id], 1)?;
    let mut tx = state.db.pool.begin().await.map_err(storage)?;
    sqlx::query("SELECT id FROM member_tags WHERE id=$1 FOR UPDATE")
        .bind(input.tag_id)
        .fetch_optional(&mut *tx)
        .await
        .map_err(storage)?
        .ok_or_else(|| ApiError::not_found("Member tag"))?;
    let removed = sqlx::query("DELETE FROM user_member_tags WHERE user_id=$1 AND tag_id=$2")
        .bind(input.user_id)
        .bind(input.tag_id)
        .execute(&mut *tx)
        .await
        .map_err(storage)?
        .rows_affected();
    sqlx::query("UPDATE member_tags tag SET usage_count=(SELECT COUNT(*)::integer FROM user_member_tags assignment JOIN users member ON member.id=assignment.user_id WHERE assignment.tag_id=tag.id AND member.deleted_at IS NULL AND COALESCE(LOWER(member.role),'')<>'deleted'),updated_at=NOW() WHERE tag.id=$1").bind(input.tag_id).execute(&mut *tx).await.map_err(storage)?;
    validation::audit(
        &mut tx,
        admin.0.id,
        "Member tag assignment removal recorded",
        json!({"member_id":input.user_id,"member_tag_id":input.tag_id,"removed":removed}),
    )
    .await?;
    tx.commit().await.map_err(storage)?;
    Ok(Json(
        json!({"message":if removed>0{"Member tag removed"}else{"No tag assignment existed"},"removed":removed>0,"user_id":input.user_id,"tag_id":input.tag_id}),
    ))
}
