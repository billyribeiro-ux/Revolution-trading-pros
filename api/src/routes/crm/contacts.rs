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
    middleware::admin::AdminUser,
    services::crm_logs,
    utils::errors::{self, ApiError},
};
#[path = "contact_activity.rs"]
mod activity;

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

#[derive(Debug, Deserialize)]
pub struct ContactFilters {
    pub page: Option<i64>,
    pub per_page: Option<i64>,
    pub status: Option<String>,
    pub search: Option<String>,
    pub tag: Option<String>,
    pub list_id: Option<String>,
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
pub(super) fn contact_db_error(error: sqlx::Error) -> (StatusCode, Json<serde_json::Value>) {
    if error
        .as_database_error()
        .is_some_and(|e| e.is_unique_violation())
    {
        (
            StatusCode::CONFLICT,
            Json(json!({"error":"A contact with this email already exists"})),
        )
    } else {
        tracing::error!(%error,"CRM contact persistence failed");
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error":"Unable to save contact"})),
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
async fn list_contacts(
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
    if let Some(status) = &filters.status {
        checked(validate_fields(json!({"status":status}), false))?;
    }
    let list = filters
        .list_id
        .as_deref()
        .map(str::parse::<i64>)
        .transpose()
        .map_err(|_| errors::validation_error("Invalid list ID"))?;
    if list.is_some_and(|value| value <= 0) {
        return Err(errors::validation_error("List ID must be positive"));
    }
    let search = filters.search.map(|value| format!("%{value}%"));
    let mut tx = state.db.pool.begin().await.map_err(activity::db)?;
    sqlx::query("SET TRANSACTION ISOLATION LEVEL REPEATABLE READ READ ONLY")
        .execute(&mut *tx)
        .await
        .map_err(activity::db)?;
    let rows:Vec<Value>=sqlx::query_scalar("SELECT to_jsonb(c) FROM contacts c WHERE ($1::text IS NULL OR c.status=$1) AND ($2::text IS NULL OR c.email ILIKE $2 OR c.first_name ILIKE $2 OR c.last_name ILIKE $2) AND ($3::bigint IS NULL OR EXISTS(SELECT 1 FROM crm_list_members m WHERE m.contact_id=c.id AND m.list_id=$3)) AND ($4::text IS NULL OR COALESCE(c.tags,'[]'::jsonb) ? $4) ORDER BY c.created_at DESC NULLS LAST,c.id DESC LIMIT $5 OFFSET $6")
        .bind(&filters.status).bind(&search).bind(list).bind(&filters.tag).bind(size).bind((page-1)*size).fetch_all(&mut *tx).await.map_err(activity::db)?;
    let total:i64=sqlx::query_scalar("SELECT COUNT(*) FROM contacts c WHERE ($1::text IS NULL OR c.status=$1) AND ($2::text IS NULL OR c.email ILIKE $2 OR c.first_name ILIKE $2 OR c.last_name ILIKE $2) AND ($3::bigint IS NULL OR EXISTS(SELECT 1 FROM crm_list_members m WHERE m.contact_id=c.id AND m.list_id=$3)) AND ($4::text IS NULL OR COALESCE(c.tags,'[]'::jsonb) ? $4)")
        .bind(&filters.status).bind(search).bind(list).bind(&filters.tag).fetch_one(&mut *tx).await.map_err(activity::db)?;
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
async fn get_contact(State(state): State<AppState>, _: AdminUser, Path(id): Path<i64>) -> Response {
    let mut tx = state.db.pool.begin().await.map_err(activity::db)?;
    sqlx::query("SET TRANSACTION ISOLATION LEVEL REPEATABLE READ READ ONLY")
        .execute(&mut *tx)
        .await
        .map_err(activity::db)?;
    let row: Value = sqlx::query_scalar("SELECT to_jsonb(c) FROM contacts c WHERE id=$1")
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
async fn create_contact(
    State(state): State<AppState>,
    admin: AdminUser,
    Json(input): Json<CreateContactInput>,
) -> Response {
    checked(input.validate(false))?;
    let mut tx = state.db.pool.begin().await.map_err(activity::db)?;
    activity::actor(&mut tx, admin.0.id).await?;
    let row:Value=sqlx::query_scalar("INSERT INTO contacts(email,first_name,last_name,phone,company,job_title,source,status,tags,custom_fields,score,created_at,updated_at) VALUES($1,$2,$3,$4,$5,$6,$7,$8,$9,$10,NULL,NOW(),NOW()) RETURNING to_jsonb(contacts)")
        .bind(&input.email).bind(&input.first_name).bind(&input.last_name).bind(&input.phone).bind(&input.company).bind(&input.job_title).bind(&input.source).bind(input.status.as_deref().unwrap_or("lead")).bind(input.tags.map(|tags|json!(tags))).bind(&input.custom_fields)
        .fetch_one(&mut *tx).await.map_err(activity::db)?;
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
    let policy = activity::policy(&mut tx).await?;
    tx.commit().await.map_err(activity::db)?;
    Ok(Json(activity::dto(row, &policy)))
}
async fn update_contact(
    State(state): State<AppState>,
    admin: AdminUser,
    Path(id): Path<i64>,
    Json(input): Json<UpdateContactInput>,
) -> Response {
    checked(input.validate(false))?;
    let mut tx = state.db.pool.begin().await.map_err(activity::db)?;
    activity::actor(&mut tx, admin.0.id).await?;
    let row:Value=sqlx::query_scalar("UPDATE contacts SET email=COALESCE($2,email),first_name=COALESCE($3,first_name),last_name=COALESCE($4,last_name),phone=COALESCE($5,phone),company=COALESCE($6,company),job_title=COALESCE($7,job_title),status=COALESCE($8,status),tags=COALESCE($9,tags),custom_fields=COALESCE($10,custom_fields),updated_at=NOW() WHERE id=$1 RETURNING to_jsonb(contacts)")
        .bind(id).bind(&input.email).bind(&input.first_name).bind(&input.last_name).bind(&input.phone).bind(&input.company).bind(&input.job_title).bind(&input.status).bind(input.tags.map(|tags|json!(tags))).bind(&input.custom_fields)
        .fetch_optional(&mut *tx).await.map_err(activity::db)?.ok_or_else(||errors::not_found("Contact"))?;
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
    let policy = activity::policy(&mut tx).await?;
    tx.commit().await.map_err(activity::db)?;
    Ok(Json(activity::dto(row, &policy)))
}
async fn delete_contact(
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
