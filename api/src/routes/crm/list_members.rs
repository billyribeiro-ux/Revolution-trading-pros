//! Durable contact-list membership. Removing a membership never deletes a contact.
use super::contacts::CrmContact;
use crate::{AppState, middleware::admin::AdminUser, utils::errors::ApiError};
use axum::{
    Json, Router,
    extract::{Path, Query, State},
    routing::get,
};
use serde::Deserialize;
use serde_json::{Value, json};

#[derive(Deserialize)]
struct Filters {
    page: Option<i64>,
    per_page: Option<i64>,
    search: Option<String>,
}

#[derive(Deserialize)]
#[serde(untagged)]
enum ContactId {
    Number(i64),
    Text(String),
}

#[derive(Deserialize)]
struct MembersInput {
    contact_ids: Vec<ContactId>,
}

impl MembersInput {
    fn ids(self) -> Result<Vec<i64>, ApiError> {
        if self.contact_ids.is_empty() || self.contact_ids.len() > 1000 {
            return Err(ApiError::validation_error("Provide 1–1000 contact IDs"));
        }
        let mut ids = self
            .contact_ids
            .into_iter()
            .map(|id| {
                let id = match id {
                    ContactId::Number(id) => Some(id),
                    ContactId::Text(id) => id.parse().ok(),
                };
                id.filter(|id| *id > 0).ok_or_else(|| {
                    ApiError::validation_error("Contact IDs must be positive integers")
                })
            })
            .collect::<Result<Vec<_>, _>>()?;
        ids.sort_unstable();
        ids.dedup();
        Ok(ids)
    }
}

fn db_error(error: sqlx::Error) -> ApiError {
    ApiError::database_error(&error.to_string())
}

async fn members(
    State(state): State<AppState>,
    _admin: AdminUser,
    Path(id): Path<i64>,
    Query(filters): Query<Filters>,
) -> Result<Json<Value>, ApiError> {
    let exists: bool = sqlx::query_scalar("SELECT EXISTS(SELECT 1 FROM crm_lists WHERE id=$1)")
        .bind(id)
        .fetch_one(&state.db.pool)
        .await
        .map_err(db_error)?;
    if !exists {
        return Err(ApiError::not_found("List not found"));
    }
    let page = filters.page.unwrap_or(1).max(1);
    let per_page = filters.per_page.unwrap_or(50).clamp(1, 100);
    let search = filters.search.map(|s| format!("%{}%", s.replace('\0', "")));
    let contacts: Vec<CrmContact> = sqlx::query_as("SELECT c.id,c.email,c.first_name,c.last_name,c.phone,c.company,c.job_title,c.status,c.source,COALESCE(c.score,0) AS score,c.tags,c.custom_fields,c.last_contacted_at,c.created_at,c.updated_at FROM contacts c JOIN crm_list_members m ON m.contact_id=c.id WHERE m.list_id=$1 AND ($2::text IS NULL OR c.email ILIKE $2 OR c.first_name ILIKE $2 OR c.last_name ILIKE $2) ORDER BY c.id LIMIT $3 OFFSET $4")
        .bind(id).bind(&search).bind(per_page).bind(page.saturating_sub(1).saturating_mul(per_page)).fetch_all(&state.db.pool).await.map_err(db_error)?;
    let total: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM contacts c JOIN crm_list_members m ON m.contact_id=c.id WHERE m.list_id=$1 AND ($2::text IS NULL OR c.email ILIKE $2 OR c.first_name ILIKE $2 OR c.last_name ILIKE $2)")
        .bind(id).bind(&search).fetch_one(&state.db.pool).await.map_err(db_error)?;
    Ok(Json(
        json!({"data":contacts,"meta":{"total":total,"current_page":page,"per_page":per_page,"total_pages":((total+per_page-1)/per_page).max(1)}}),
    ))
}

async fn change_members(
    state: &AppState,
    id: i64,
    input: MembersInput,
    add: bool,
) -> Result<Json<Value>, ApiError> {
    let ids = input.ids()?;
    let mut tx = state.db.pool.begin().await.map_err(db_error)?;
    let list: Option<i64> = sqlx::query_scalar("SELECT id FROM crm_lists WHERE id=$1 FOR UPDATE")
        .bind(id)
        .fetch_optional(&mut *tx)
        .await
        .map_err(db_error)?;
    if list.is_none() {
        return Err(ApiError::not_found("List not found"));
    }
    let changed = if add {
        let found: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM contacts WHERE id=ANY($1)")
            .bind(&ids)
            .fetch_one(&mut *tx)
            .await
            .map_err(db_error)?;
        if found != ids.len() as i64 {
            return Err(ApiError::validation_error(
                "One or more contacts do not exist",
            ));
        }
        sqlx::query("INSERT INTO crm_list_members (list_id,contact_id) SELECT $1,UNNEST($2::bigint[]) ON CONFLICT DO NOTHING").bind(id).bind(&ids).execute(&mut *tx).await.map_err(db_error)?.rows_affected()
    } else {
        sqlx::query("DELETE FROM crm_list_members WHERE list_id=$1 AND contact_id=ANY($2)")
            .bind(id)
            .bind(&ids)
            .execute(&mut *tx)
            .await
            .map_err(db_error)?
            .rows_affected()
    };
    sqlx::query("UPDATE crm_lists SET contacts_count=(SELECT COUNT(*) FROM crm_list_members WHERE list_id=$1),updated_at=NOW() WHERE id=$1").bind(id).execute(&mut *tx).await.map_err(db_error)?;
    tx.commit().await.map_err(db_error)?;
    Ok(Json(if add {
        json!({"added_count":changed})
    } else {
        json!({"removed_count":changed})
    }))
}

async fn add(
    State(state): State<AppState>,
    _admin: AdminUser,
    Path(id): Path<i64>,
    Json(input): Json<MembersInput>,
) -> Result<Json<Value>, ApiError> {
    change_members(&state, id, input, true).await
}
async fn remove(
    State(state): State<AppState>,
    _admin: AdminUser,
    Path(id): Path<i64>,
    Json(input): Json<MembersInput>,
) -> Result<Json<Value>, ApiError> {
    change_members(&state, id, input, false).await
}

pub(super) fn router() -> Router<AppState> {
    Router::new().route(
        "/lists/{id}/contacts",
        get(members).post(add).delete(remove),
    )
}
