//! Persisted deal notes and an activity/history timeline, never synthetic events.
use crate::{
    AppState, routes::crm::manager_permissions::CrmUser as AdminUser, utils::errors::ApiError,
};
use axum::{
    Json, Router,
    extract::{Path, Query, State},
    routing::get,
};
use serde::Deserialize;
use serde_json::{Value, json};

fn db(error: sqlx::Error) -> ApiError {
    ApiError::database_error(&error.to_string())
}
#[derive(Deserialize)]
struct Pagination {
    page: Option<i64>,
    per_page: Option<i64>,
}
impl Pagination {
    fn bounds(&self) -> Result<(i64, i64, i64), ApiError> {
        let page = self.page.unwrap_or(1);
        let per_page = self.per_page.unwrap_or(50);
        if !(1..=100_000).contains(&page) || !(1..=100).contains(&per_page) {
            return Err(ApiError::validation_error("Invalid activity pagination"));
        }
        Ok((page, per_page, (page - 1) * per_page))
    }
}
async fn ensure_deal(state: &AppState, id: i64) -> Result<(), ApiError> {
    let exists: bool = sqlx::query_scalar("SELECT EXISTS(SELECT 1 FROM crm_deals WHERE id=$1)")
        .bind(id)
        .fetch_one(state.db.pool())
        .await
        .map_err(db)?;
    if exists {
        Ok(())
    } else {
        Err(ApiError::not_found("Deal not found"))
    }
}
fn envelope(data: Vec<Value>, page: i64, per_page: i64, total: i64) -> Json<Value> {
    Json(
        json!({"data":data,"meta":{"current_page":page,"per_page":per_page,"total":total,"last_page":((total+per_page-1)/per_page).max(1)}}),
    )
}
async fn notes(
    State(state): State<AppState>,
    _admin: AdminUser,
    Path(id): Path<i64>,
    Query(pagination): Query<Pagination>,
) -> Result<Json<Value>, ApiError> {
    ensure_deal(&state, id).await?;
    let (page, per_page, offset) = pagination.bounds()?;
    let total: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM crm_deal_activities WHERE deal_id=$1 AND activity_type='note'",
    )
    .bind(id)
    .fetch_one(state.db.pool())
    .await
    .map_err(db)?;
    let rows:Vec<Value>=sqlx::query_scalar("SELECT jsonb_build_object('id',a.id::text,'content',a.description,'created_at',a.created_at,'updated_at',a.updated_at,'created_by',CASE WHEN u.id IS NULL THEN NULL ELSE jsonb_build_object('id',u.id::text,'name',u.name) END) FROM crm_deal_activities a LEFT JOIN users u ON u.id=a.created_by WHERE a.deal_id=$1 AND a.activity_type='note' ORDER BY a.created_at DESC NULLS LAST,a.id DESC LIMIT $2 OFFSET $3")
        .bind(id).bind(per_page).bind(offset).fetch_all(state.db.pool()).await.map_err(db)?;
    Ok(envelope(rows, page, per_page, total))
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Note {
    content: String,
}
async fn add_note(
    State(state): State<AppState>,
    admin: AdminUser,
    Path(id): Path<i64>,
    Json(input): Json<Note>,
) -> Result<Json<Value>, ApiError> {
    let content = input.content.trim();
    if content.is_empty() || content.chars().count() > 10_000 || content.contains('\0') {
        return Err(ApiError::validation_error(
            "Note content must contain 1–10000 characters without null characters",
        ));
    }
    let note_id:i64=sqlx::query_scalar("INSERT INTO crm_deal_activities(deal_id,activity_type,title,description,created_by,created_at,updated_at) SELECT id,'note','Note added',$2,$3,NOW(),NOW() FROM crm_deals WHERE id=$1 RETURNING id")
        .bind(id).bind(content).bind(admin.0.id).fetch_optional(state.db.pool()).await.map_err(db)?.ok_or_else(||ApiError::not_found("Deal not found"))?;
    let row:Value=sqlx::query_scalar("SELECT jsonb_build_object('id',a.id::text,'content',a.description,'created_at',a.created_at,'updated_at',a.updated_at,'created_by',CASE WHEN u.id IS NULL THEN NULL ELSE jsonb_build_object('id',u.id::text,'name',u.name) END) FROM crm_deal_activities a LEFT JOIN users u ON u.id=a.created_by WHERE a.id=$1")
        .bind(note_id).fetch_one(state.db.pool()).await.map_err(db)?;
    Ok(Json(json!({"data":row})))
}
async fn timeline(
    State(state): State<AppState>,
    _admin: AdminUser,
    Path(id): Path<i64>,
    Query(pagination): Query<Pagination>,
) -> Result<Json<Value>, ApiError> {
    ensure_deal(&state, id).await?;
    let (page, per_page, offset) = pagination.bounds()?;
    // Stage history owns stage transitions; exclude duplicate activity rows of
    // that type while retaining every stored note and other real activity.
    let total:i64=sqlx::query_scalar("SELECT (SELECT COUNT(*) FROM crm_deal_activities WHERE deal_id=$1 AND activity_type<>'stage_changed')+(SELECT COUNT(*) FROM crm_deal_stage_history WHERE deal_id=$1)").bind(id).fetch_one(state.db.pool()).await.map_err(db)?;
    let rows:Vec<Value>=sqlx::query_scalar("SELECT to_jsonb(events) FROM (SELECT 'activity-'||a.id::text id,a.activity_type type,'deal'::text category,a.title,a.description,a.metadata,a.created_at occurred_at,CASE WHEN u.id IS NULL THEN NULL ELSE jsonb_build_object('id',u.id::text,'name',u.name) END created_by FROM crm_deal_activities a LEFT JOIN users u ON u.id=a.created_by WHERE a.deal_id=$1 AND a.activity_type<>'stage_changed' UNION ALL SELECT 'stage-'||h.id::text,'stage_changed','deal','Stage changed',h.reason,jsonb_build_object('from_stage_id',h.from_stage_id,'to_stage_id',h.to_stage_id,'duration_in_stage',h.duration_in_stage),h.created_at,CASE WHEN u.id IS NULL THEN NULL ELSE jsonb_build_object('id',u.id::text,'name',u.name) END FROM crm_deal_stage_history h LEFT JOIN users u ON u.id=h.changed_by WHERE h.deal_id=$1) events ORDER BY occurred_at DESC NULLS LAST,id DESC LIMIT $2 OFFSET $3")
        .bind(id).bind(per_page).bind(offset).fetch_all(state.db.pool()).await.map_err(db)?;
    Ok(envelope(rows, page, per_page, total))
}
pub(super) fn router() -> Router<AppState> {
    Router::new()
        .route("/deals/{id}/notes", get(notes).post(add_note))
        .route("/deals/{id}/timeline", get(timeline))
}
