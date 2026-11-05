//! CRM contacts handlers (split from `crm.rs` lines 30-84 + 278-503).

use axum::{
    Json, Router,
    extract::{Path, Query, State},
    http::StatusCode,
    routing::{get, post},
};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use sqlx::FromRow;

use crate::{
    AppState,
    routes::crm::manager_permissions::CrmUser as AdminUser,
    services::crm_logs,
    utils::errors::{self, ApiError},
};
#[path = "contact_activity.rs"]
mod activity;
#[path = "contact_interactions.rs"]
mod interactions;

// ═══════════════════════════════════════════════════════════════════════════
// TYPES
// ═══════════════════════════════════════════════════════════════════════════

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct CrmContact {
    pub id: i64,
    pub email: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub phone: Option<String>,
    pub company: Option<String>,
    pub job_title: Option<String>,
    pub status: String,
    pub source: Option<String>,
    pub score: Option<i32>,
    pub tags: Option<serde_json::Value>,
    pub custom_fields: Option<serde_json::Value>,
    pub last_contacted_at: Option<chrono::NaiveDateTime>,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(Debug, Deserialize, Default)]
#[serde(deny_unknown_fields)]
pub struct ContactFilters {
    pub page: Option<i64>,
    pub per_page: Option<i64>,
    pub status: Option<String>,
    pub search: Option<String>,
    pub tag: Option<String>,
    pub list_id: Option<String>,
    pub lifecycle_stage: Option<String>,
    pub owner_id: Option<String>,
    pub sort_by: Option<String>,
    pub sort_order: Option<String>,
    pub source: Option<String>,
    #[serde(skip)]
    pub lead_only: bool,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CreateContactInput {
    pub email: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub phone: Option<String>,
    #[serde(alias = "company_name")]
    pub company: Option<String>,
    pub job_title: Option<String>,
    pub status: Option<String>,
    pub source: Option<String>,
    pub tags: Option<Vec<String>>,
    pub custom_fields: Option<serde_json::Value>,
    #[serde(default, deserialize_with = "nullable_patch")]
    pub lifecycle_stage: Option<Option<String>>,
    #[serde(default, deserialize_with = "nullable_patch")]
    pub owner_id: Option<Option<Value>>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UpdateContactInput {
    pub email: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub phone: Option<String>,
    #[serde(alias = "company_name")]
    pub company: Option<String>,
    pub job_title: Option<String>,
    pub status: Option<String>,
    pub tags: Option<Vec<String>>,
    pub custom_fields: Option<serde_json::Value>,
    #[serde(default, deserialize_with = "nullable_patch")]
    pub lifecycle_stage: Option<Option<String>>,
    #[serde(default, deserialize_with = "nullable_patch")]
    pub owner_id: Option<Option<Value>>,
}

fn nullable_patch<'de, D, T>(deserializer: D) -> Result<Option<Option<T>>, D::Error>
where
    D: serde::Deserializer<'de>,
    T: Deserialize<'de>,
{
    Option::<T>::deserialize(deserializer).map(Some)
}
pub(super) fn lifecycle(value: &str) -> bool {
    matches!(
        value,
        "subscriber" | "lead" | "mql" | "sql" | "opportunity" | "customer" | "evangelist"
    )
}
fn owner_value(value: &Value) -> Option<i64> {
    match value {
        Value::Number(value) => value.as_i64(),
        Value::String(value) if value.bytes().all(|byte| byte.is_ascii_digit()) => {
            value.parse().ok()
        }
        _ => None,
    }
    .filter(|value| *value > 0)
}
fn custom_fields(input: Option<Value>, stage: &Option<Option<String>>) -> Value {
    let mut value = input.unwrap_or_else(|| json!({}));
    if let Some(stage) = stage {
        if let Some(stage) = stage {
            value["lifecycle_stage"] = json!(stage);
        } else if let Some(value) = value.as_object_mut() {
            value.remove("lifecycle_stage");
        }
    }
    value
}
async fn lock_owner(
    connection: &mut sqlx::PgConnection,
    owner: Option<&Value>,
) -> Result<Option<i64>, ApiError> {
    let Some(owner) = owner else { return Ok(None) };
    let owner = owner_value(owner)
        .ok_or_else(|| ApiError::validation_error("Owner ID must identify an active user"))?;
    let exists: Option<i64> = sqlx::query_scalar(
        "SELECT id FROM users WHERE id=$1 AND deleted_at IS NULL AND is_active=true FOR KEY SHARE",
    )
    .bind(owner)
    .fetch_optional(connection)
    .await
    .map_err(activity::db)?;
    exists
        .map(Some)
        .ok_or_else(|| ApiError::validation_error("Owner must reference an active user"))
}

type ContactResult = Result<(), (StatusCode, Json<serde_json::Value>)>;
fn invalid(message: &str) -> (StatusCode, Json<serde_json::Value>) {
    (StatusCode::BAD_REQUEST, Json(json!({"error":message})))
}
fn contains_null(value: &serde_json::Value) -> bool {
    match value {
        serde_json::Value::String(text) => text.contains('\0'),
        serde_json::Value::Array(values) => values.iter().any(contains_null),
        serde_json::Value::Object(values) => values
            .iter()
            .any(|(key, value)| key.contains('\0') || contains_null(value)),
        _ => false,
    }
}
fn validate_fields(value: serde_json::Value, lead: bool) -> ContactResult {
    use validator::ValidateEmail;
    for stage in [
        &value["lifecycle_stage"],
        &value["custom_fields"]["lifecycle_stage"],
    ] {
        if !stage.is_null() && stage.as_str().is_none_or(|stage| !lifecycle(stage)) {
            return Err(invalid("Choose a supported lifecycle stage or null"));
        }
    }
    if !value["owner_id"].is_null() && owner_value(&value["owner_id"]).is_none() {
        return Err(invalid("Owner ID must be a positive record ID or null"));
    }
    if let Some(email) = value["email"].as_str()
        && (!email.validate_email() || email.len() > 255)
    {
        return Err(invalid("Provide a valid email address up to 255 bytes"));
    }
    for (field, limit) in [
        ("first_name", 255),
        ("last_name", 255),
        ("company", 255),
        ("job_title", 255),
        ("phone", 50),
        ("source", 100),
    ] {
        if value[field]
            .as_str()
            .is_some_and(|s| s.chars().count() > limit || s.contains('\0'))
        {
            return Err(invalid(&format!(
                "{field} exceeds its length limit or contains invalid characters"
            )));
        }
    }
    if let Some(status) = value["status"].as_str() {
        let lead_status = matches!(
            status,
            "new"
                | "contacted"
                | "qualified"
                | "proposal"
                | "negotiation"
                | "won"
                | "lost"
                | "lead"
        );
        if !lead_status
            && (lead || !matches!(status, "prospect" | "customer" | "churned" | "unqualified"))
        {
            return Err(invalid("Invalid contact status"));
        }
    }
    if let Some(tags) = value["tags"].as_array()
        && (tags.len() > 100
            || tags.iter().any(|v| {
                v.as_str()
                    .is_none_or(|s| s.is_empty() || s.chars().count() > 100 || s.contains('\0'))
            }))
    {
        return Err(invalid(
            "Provide at most 100 nonempty tags up to 100 characters",
        ));
    }
    if !value["custom_fields"].is_null()
        && (!value["custom_fields"].is_object()
            || value["custom_fields"].to_string().len() > 65536
            || contains_null(&value["custom_fields"]))
    {
        return Err(invalid(
            "Custom fields must be an object up to 64 KiB without null characters",
        ));
    }
    Ok(())
}
impl CreateContactInput {
    pub(super) fn validate(&self, lead: bool) -> ContactResult {
        validate_fields(
            serde_json::to_value(self).map_err(|_| invalid("Invalid contact fields"))?,
            lead,
        )
    }
}
impl UpdateContactInput {
    pub(super) fn validate(&self, lead: bool) -> ContactResult {
        validate_fields(
            serde_json::to_value(self).map_err(|_| invalid("Invalid contact fields"))?,
            lead,
        )
    }
}
// ═══════════════════════════════════════════════════════════════════════════
// HANDLERS
// ═══════════════════════════════════════════════════════════════════════════

type Response = Result<Json<Value>, ApiError>;
fn checked(result: ContactResult) -> Result<(), ApiError> {
    result.map_err(|(status, Json(body))| {
        ApiError::new(
            status,
            body["error"].as_str().unwrap_or("Invalid contact fields"),
        )
    })
}
pub(super) async fn list_contacts(
    State(state): State<AppState>,
    _: AdminUser,
    Query(filters): Query<ContactFilters>,
) -> Response {
    let page = filters.page.unwrap_or(1);
    let size = filters.per_page.unwrap_or(50);
    if !(1..=100_000).contains(&page) || !(1..=100).contains(&size) {
        return Err(errors::validation_error(
            "Choose a positive bounded page and page size1–100",
        ));
    }
    if filters
        .search
        .as_ref()
        .is_some_and(|value| value.chars().count() > 255 || value.contains('\0'))
        || filters.tag.as_ref().is_some_and(|value| {
            value.is_empty() || value.chars().count() > 100 || value.contains('\0')
        })
    {
        return Err(errors::validation_error(
            "Provide bounded contact search and tag values without null characters",
        ));
    }
    if filters
        .source
        .as_ref()
        .is_some_and(|value| value.len() > 100 || value.contains('\0'))
    {
        return Err(ApiError::validation_error(
            "Source filter must be bounded text without null characters",
        ));
    }
    if let Some(status) = &filters.status {
        checked(validate_fields(json!({"status":status}), false))?;
    }
    if filters
        .lifecycle_stage
        .as_deref()
        .is_some_and(|stage| !lifecycle(stage))
    {
        return Err(ApiError::validation_error(
            "Choose a supported lifecycle stage",
        ));
    }
    let owner = filters
        .owner_id
        .as_deref()
        .map(|owner| {
            if owner == "unassigned" {
                Ok(owner.to_string())
            } else {
                owner_value(&json!(owner))
                    .map(|id| id.to_string())
                    .ok_or_else(|| {
                        ApiError::validation_error(
                            "Owner filter must identify a positive user ID or unassigned",
                        )
                    })
            }
        })
        .transpose()?;
    let list = filters
        .list_id
        .as_deref()
        .map(str::parse::<i64>)
        .transpose()
        .map_err(|_| errors::validation_error("Invalid list ID"))?;
    if list.is_some_and(|id| id <= 0) {
        return Err(errors::validation_error("List ID must be positive"));
    }
    let sort = filters.sort_by.as_deref().unwrap_or("created_at");
    let direction = filters.sort_order.as_deref().unwrap_or("desc");
    if !matches!(
        sort,
        "created_at"
            | "updated_at"
            | "email"
            | "name"
            | "score"
            | "lead_score"
            | "last_activity_at"
    ) || !matches!(direction, "asc" | "desc")
    {
        return Err(ApiError::validation_error(
            "Choose a supported contact sort and direction",
        ));
    }
    let search = filters.search.map(|value| format!("%{value}%"));
    let mut tx = state.db.pool.begin().await.map_err(activity::db)?;
    sqlx::query("SET TRANSACTION ISOLATION LEVEL REPEATABLE READ READ ONLY")
        .execute(&mut *tx)
        .await
        .map_err(activity::db)?;
    let rows:Vec<Value>=sqlx::query_scalar("SELECT to_jsonb(q)-'current_score' FROM (SELECT c.*,crm_contact_last_activity(c.id) AS last_activity_at,CASE WHEN c.score_assessment->>'status'='calculated' AND c.score_assessed_at IS NOT NULL AND c.score_assessment->'policy'=(SELECT value->'scoring_settings' FROM settings WHERE key='crm.settings') AND c.score_assessment->'score'=to_jsonb(c.score) THEN c.score ELSE NULL END AS current_score FROM contacts c WHERE ($1::text IS NULL OR c.status=$1) AND ($2::text IS NULL OR c.email ILIKE $2 OR c.first_name ILIKE $2 OR c.last_name ILIKE $2) AND ($3::bigint IS NULL OR EXISTS(SELECT 1 FROM crm_list_members m WHERE m.contact_id=c.id AND m.list_id=$3)) AND ($4::text IS NULL OR COALESCE(c.tags,'[]'::jsonb)?$4) AND ($5::text IS NULL OR c.custom_fields->>'lifecycle_stage'=$5) AND ($6::text IS NULL OR CASE WHEN $6='unassigned' THEN c.owner_id IS NULL ELSE c.owner_id::text=$6 END) AND (NOT $11 OR c.status IN ('new','contacted','qualified','proposal','negotiation','won','lost','lead')) AND ($12::text IS NULL OR c.source=$12)) q ORDER BY CASE WHEN $7='created_at' AND $8='asc' THEN q.created_at END ASC NULLS LAST,CASE WHEN $7='created_at' AND $8='desc' THEN q.created_at END DESC NULLS LAST,CASE WHEN $7='updated_at' AND $8='asc' THEN q.updated_at END ASC NULLS LAST,CASE WHEN $7='updated_at' AND $8='desc' THEN q.updated_at END DESC NULLS LAST,CASE WHEN $7='last_activity_at' AND $8='asc' THEN q.last_activity_at END ASC NULLS LAST,CASE WHEN $7='last_activity_at' AND $8='desc' THEN q.last_activity_at END DESC NULLS LAST,CASE WHEN $7 IN ('score','lead_score') AND $8='asc' THEN q.current_score END ASC NULLS LAST,CASE WHEN $7 IN ('score','lead_score') AND $8='desc' THEN q.current_score END DESC NULLS LAST,CASE WHEN $7='email' AND $8='asc' THEN LOWER(q.email) END ASC NULLS LAST,CASE WHEN $7='email' AND $8='desc' THEN LOWER(q.email) END DESC NULLS LAST,CASE WHEN $7='name' AND $8='asc' THEN LOWER(COALESCE(NULLIF(TRIM(CONCAT_WS(' ',q.first_name,q.last_name)),''),q.email)) END ASC NULLS LAST,CASE WHEN $7='name' AND $8='desc' THEN LOWER(COALESCE(NULLIF(TRIM(CONCAT_WS(' ',q.first_name,q.last_name)),''),q.email)) END DESC NULLS LAST,q.id DESC LIMIT $9 OFFSET $10")
        .bind(&filters.status).bind(&search).bind(list).bind(&filters.tag).bind(&filters.lifecycle_stage).bind(&owner).bind(sort).bind(direction).bind(size).bind((page-1)*size).bind(filters.lead_only).bind(&filters.source).fetch_all(&mut *tx).await.map_err(activity::db)?;
    let total:i64=sqlx::query_scalar("SELECT COUNT(*) FROM contacts c WHERE ($1::text IS NULL OR c.status=$1) AND ($2::text IS NULL OR c.email ILIKE $2 OR c.first_name ILIKE $2 OR c.last_name ILIKE $2) AND ($3::bigint IS NULL OR EXISTS(SELECT 1 FROM crm_list_members m WHERE m.contact_id=c.id AND m.list_id=$3)) AND ($4::text IS NULL OR COALESCE(c.tags,'[]'::jsonb)?$4) AND ($5::text IS NULL OR c.custom_fields->>'lifecycle_stage'=$5) AND ($6::text IS NULL OR CASE WHEN $6='unassigned' THEN c.owner_id IS NULL ELSE c.owner_id::text=$6 END) AND (NOT $7 OR c.status IN ('new','contacted','qualified','proposal','negotiation','won','lost','lead')) AND ($8::text IS NULL OR c.source=$8)")
        .bind(&filters.status).bind(&search).bind(list).bind(&filters.tag).bind(&filters.lifecycle_stage).bind(&owner).bind(filters.lead_only).bind(&filters.source).fetch_one(&mut *tx).await.map_err(activity::db)?;
    let policy = activity::policy(&mut tx).await?;
    let rows = rows
        .into_iter()
        .map(|row| activity::dto(row, &policy))
        .collect::<Vec<_>>();
    tx.commit().await.map_err(activity::db)?;
    Ok(Json(
        json!({"data":rows,"meta":{"current_page":page,"per_page":size,"total":total,"total_pages":((total+size-1)/size).max(1)}}),
    ))
}
pub(super) async fn get_contact(
    State(state): State<AppState>,
    _: AdminUser,
    Path(id): Path<i64>,
) -> Response {
    let mut tx = state.db.pool.begin().await.map_err(activity::db)?;
    sqlx::query("SET TRANSACTION ISOLATION LEVEL REPEATABLE READ READ ONLY")
        .execute(&mut *tx)
        .await
        .map_err(activity::db)?;
    let row: Value = sqlx::query_scalar("SELECT to_jsonb(c)||jsonb_build_object('last_activity_at',crm_contact_last_activity(c.id)) FROM contacts c WHERE id=$1")
        .bind(id)
        .fetch_optional(&mut *tx)
        .await
        .map_err(activity::db)?
        .ok_or_else(|| errors::not_found("Contact"))?;
    let policy = activity::policy(&mut tx).await?;
    let mut row = activity::dto(row, &policy);
    row["facts"] = activity::facts(&mut tx, id).await?;
    tx.commit().await.map_err(activity::db)?;
    Ok(Json(row))
}
pub(super) async fn create_contact(
    State(state): State<AppState>,
    admin: AdminUser,
    Json(input): Json<CreateContactInput>,
) -> Response {
    admin.body(
        &serde_json::to_value(&input)
            .map_err(|_| ApiError::validation_error("Invalid contact fields"))?,
    )?;
    checked(input.validate(false))?;
    let mut tx = state.db.pool.begin().await.map_err(activity::db)?;
    activity::actor(&mut tx, admin.0.id).await?;
    let owner = lock_owner(&mut tx, input.owner_id.as_ref().and_then(Option::as_ref)).await?;
    let fields = if input.custom_fields.is_some() || input.lifecycle_stage.is_some() {
        Some(custom_fields(input.custom_fields, &input.lifecycle_stage))
    } else {
        None
    };
    let mut row:Value=sqlx::query_scalar("INSERT INTO contacts(email,first_name,last_name,phone,company,job_title,source,status,tags,custom_fields,score,owner_id,created_at,updated_at) VALUES($1,$2,$3,$4,$5,$6,$7,$8,$9,$10,NULL,$11,NOW() AT TIME ZONE 'UTC',NOW() AT TIME ZONE 'UTC') RETURNING to_jsonb(contacts)")
        .bind(&input.email).bind(&input.first_name).bind(&input.last_name).bind(&input.phone).bind(&input.company).bind(&input.job_title).bind(&input.source).bind(input.status.as_deref().unwrap_or("lead")).bind(input.tags.map(|tags|json!(tags))).bind(fields).bind(owner).fetch_one(&mut *tx).await.map_err(activity::db)?;
    crm_logs::record(
        &mut tx,
        "info",
        "api",
        "CRM contact created",
        json!({"contact_id":row["id"]}),
        Some(admin.0.id),
    )
    .await
    .map_err(activity::db)?;
    row["last_activity_at"] = sqlx::query_scalar::<_, Value>(
        "SELECT COALESCE(to_jsonb(crm_contact_last_activity($1)),'null'::jsonb)",
    )
    .bind(row["id"].as_i64())
    .fetch_one(&mut *tx)
    .await
    .map_err(activity::db)?;
    let policy = activity::policy(&mut tx).await?;
    tx.commit().await.map_err(activity::db)?;
    Ok(Json(activity::dto(row, &policy)))
}
pub(super) async fn update_contact(
    State(state): State<AppState>,
    admin: AdminUser,
    Path(id): Path<i64>,
    Json(input): Json<UpdateContactInput>,
) -> Response {
    admin.body(
        &serde_json::to_value(&input)
            .map_err(|_| ApiError::validation_error("Invalid contact fields"))?,
    )?;
    checked(input.validate(false))?;
    let mut tx = state.db.pool.begin().await.map_err(activity::db)?;
    activity::actor(&mut tx, admin.0.id).await?;
    let owner = lock_owner(&mut tx, input.owner_id.as_ref().and_then(Option::as_ref)).await?;
    let mut row:Value=sqlx::query_scalar("UPDATE contacts SET email=COALESCE($2,email),first_name=COALESCE($3,first_name),last_name=COALESCE($4,last_name),phone=COALESCE($5,phone),company=COALESCE($6,company),job_title=COALESCE($7,job_title),status=COALESCE($8,status),tags=COALESCE($9,tags),custom_fields=CASE WHEN $10::jsonb IS NULL AND NOT $11 THEN custom_fields WHEN $11 AND $12::text IS NULL THEN (COALESCE(custom_fields,'{}'::jsonb)||COALESCE($10,'{}'::jsonb))-'lifecycle_stage' WHEN $11 THEN jsonb_set(COALESCE(custom_fields,'{}'::jsonb)||COALESCE($10,'{}'::jsonb),'{lifecycle_stage}',to_jsonb($12::text),true) ELSE COALESCE(custom_fields,'{}'::jsonb)||$10 END,owner_id=CASE WHEN $13 THEN $14::bigint ELSE owner_id END,updated_at=NOW() AT TIME ZONE 'UTC' WHERE id=$1 RETURNING to_jsonb(contacts)")
        .bind(id).bind(&input.email).bind(&input.first_name).bind(&input.last_name).bind(&input.phone).bind(&input.company).bind(&input.job_title).bind(&input.status).bind(input.tags.map(|tags|json!(tags))).bind(&input.custom_fields)
        .bind(input.lifecycle_stage.is_some()).bind(input.lifecycle_stage.as_ref().and_then(|stage|stage.as_deref())).bind(input.owner_id.is_some()).bind(owner).fetch_optional(&mut *tx).await.map_err(activity::db)?.ok_or_else(||errors::not_found("Contact"))?;
    if row["custom_fields"].to_string().len() > 65536 {
        return Err(ApiError::validation_error(
            "Merged custom fields exceed64KiB",
        ));
    }
    crm_logs::record(
        &mut tx,
        "info",
        "api",
        "CRM contact updated",
        json!({"contact_id":id}),
        Some(admin.0.id),
    )
    .await
    .map_err(activity::db)?;
    row["last_activity_at"] = sqlx::query_scalar::<_, Value>(
        "SELECT COALESCE(to_jsonb(crm_contact_last_activity($1)),'null'::jsonb)",
    )
    .bind(id)
    .fetch_one(&mut *tx)
    .await
    .map_err(activity::db)?;
    let policy = activity::policy(&mut tx).await?;
    tx.commit().await.map_err(activity::db)?;
    Ok(Json(activity::dto(row, &policy)))
}
pub(super) async fn delete_contact(
    State(state): State<AppState>,
    admin: AdminUser,
    Path(id): Path<i64>,
) -> Response {
    let mut tx = state.db.pool.begin().await.map_err(activity::db)?;
    activity::actor(&mut tx, admin.0.id).await?;
    let deleted = sqlx::query("DELETE FROM contacts WHERE id=$1")
        .bind(id)
        .execute(&mut *tx)
        .await
        .map_err(activity::db)?;
    if deleted.rows_affected() != 1 {
        return Err(errors::not_found("Contact"));
    }
    crm_logs::record(
        &mut tx,
        "info",
        "api",
        "CRM contact deleted",
        json!({"contact_id":id}),
        Some(admin.0.id),
    )
    .await
    .map_err(activity::db)?;
    tx.commit().await.map_err(activity::db)?;
    Ok(Json(
        json!({"deleted":true,"id":id.to_string(),"message":"Contact deleted successfully"}),
    ))
}

// ═══════════════════════════════════════════════════════════════════════════
// ROUTER
// ═══════════════════════════════════════════════════════════════════════════

pub(super) fn router() -> Router<AppState> {
    Router::new()
        .merge(interactions::router())
        .route("/contacts", get(list_contacts).post(create_contact))
        .route(
            "/contacts/{id}",
            get(get_contact).put(update_contact).delete(delete_contact),
        )
        .route("/contacts/{id}/timeline", get(activity::timeline))
        .route(
            "/contacts/{id}/recalculate-score",
            post(activity::recalculate),
        )
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn validates_nested_custom_values_without_rejecting_literal_escape_text() {
        let literal: UpdateContactInput =
            serde_json::from_value(json!({"custom_fields":{"pattern":"\\u0000"}})).unwrap();
        assert!(literal.validate(false).is_ok());
        let invalid: UpdateContactInput =
            serde_json::from_value(json!({"custom_fields":{"nested":[{"value":"\0"}]}})).unwrap();
        assert!(invalid.validate(false).is_err());
    }
}
