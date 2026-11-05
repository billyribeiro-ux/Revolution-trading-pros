//! Contact notes and catalog assignments persist under the authenticated actor.
use super::activity;
use crate::{
    AppState,
    routes::crm::manager_permissions::CrmUser as AdminUser,
    services::crm_logs,
    utils::errors::{self, ApiError},
};
use axum::{
    Json, Router,
    extract::{Path, Query, State},
    routing::{get, post},
};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use sqlx::{PgConnection, Postgres, Transaction};
#[path = "contact_messages.rs"]
mod messages;

#[derive(Deserialize, Serialize)]
#[serde(untagged)]
pub(super) enum RecordId {
    Number(i64),
    Text(String),
}
impl RecordId {
    pub(super) fn value(&self) -> Result<i64, ApiError> {
        let value = match self {
            Self::Number(value) => Some(*value),
            Self::Text(value) if value.bytes().all(|byte| byte.is_ascii_digit()) => {
                value.parse().ok()
            }
            _ => None,
        };
        value
            .filter(|value| *value > 0)
            .ok_or_else(|| ApiError::validation_error("Record IDs must be positive integers"))
    }
}
fn ids(values: &[RecordId], maximum: usize) -> Result<Vec<i64>, ApiError> {
    if values.is_empty() || values.len() > maximum {
        return Err(ApiError::validation_error(
            "Select a bounded nonempty set of records",
        ));
    }
    let mut ids = values
        .iter()
        .map(RecordId::value)
        .collect::<Result<Vec<_>, _>>()?;
    ids.sort_unstable();
    ids.dedup();
    Ok(ids)
}
#[derive(Default, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub(super) struct Page {
    pub(super) page: Option<i64>,
    pub(super) per_page: Option<i64>,
}
impl Page {
    pub(super) fn bounds(&self) -> Result<(i64, i64, i64), ApiError> {
        let page = self.page.unwrap_or(1);
        let size = self.per_page.unwrap_or(50);
        if !(1..=100_000).contains(&page) || !(1..=100).contains(&size) {
            return Err(ApiError::validation_error(
                "Use a positive bounded page and page size1–100",
            ));
        }
        Ok((page, size, (page - 1) * size))
    }
}
pub(super) async fn contact(
    connection: &mut PgConnection,
    id: i64,
    lock: bool,
) -> Result<Value, ApiError> {
    let row = if lock {
        sqlx::query_scalar("SELECT to_jsonb(c) FROM contacts c WHERE id=$1 FOR NO KEY UPDATE")
            .bind(id)
            .fetch_optional(connection)
            .await
    } else {
        sqlx::query_scalar("SELECT to_jsonb(c) FROM contacts c WHERE id=$1")
            .bind(id)
            .fetch_optional(connection)
            .await
    }
    .map_err(activity::db)?;
    row.ok_or_else(|| errors::not_found("Contact"))
}
pub(super) async fn read_contact(pool: &sqlx::PgPool, id: i64) -> Result<Value, ApiError> {
    contact(&mut *pool.acquire().await.map_err(activity::db)?, id, false).await
}
fn envelope(id: i64, data: Vec<Value>, page: i64, size: i64, total: i64) -> Json<Value> {
    Json(
        json!({"contact_id":id.to_string(),"data":data,"meta":{"current_page":page,"per_page":size,"total":total,"last_page":((total+size-1)/size).max(1)}}),
    )
}
async fn notes(
    State(state): State<AppState>,
    _: AdminUser,
    Path(id): Path<i64>,
    Query(page): Query<Page>,
) -> Result<Json<Value>, ApiError> {
    let (page, size, offset) = page.bounds()?;
    let mut tx = state.db.pool.begin().await.map_err(activity::db)?;
    sqlx::query("SET TRANSACTION ISOLATION LEVEL REPEATABLE READ READ ONLY")
        .execute(&mut *tx)
        .await
        .map_err(activity::db)?;
    contact(&mut tx, id, false).await?;
    let total: i64 =
        sqlx::query_scalar("SELECT COUNT(*) FROM crm_contact_notes WHERE contact_id=$1")
            .bind(id)
            .fetch_one(&mut *tx)
            .await
            .map_err(activity::db)?;
    let rows:Vec<Value>=sqlx::query_scalar("SELECT jsonb_build_object('id',n.id::text,'content',n.content,'created_at',n.created_at,'created_by',CASE WHEN u.id IS NULL THEN NULL ELSE jsonb_build_object('id',u.id::text,'name',u.name) END) FROM crm_contact_notes n LEFT JOIN users u ON u.id=n.created_by WHERE n.contact_id=$1 ORDER BY n.created_at DESC,n.id DESC LIMIT $2 OFFSET $3")
        .bind(id).bind(size).bind(offset).fetch_all(&mut *tx).await.map_err(activity::db)?;
    tx.commit().await.map_err(activity::db)?;
    Ok(envelope(id, rows, page, size, total))
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
    if content.is_empty() || content.chars().count() > 10000 || content.contains('\0') {
        return Err(ApiError::validation_error(
            "Provide1–10000characters without null characters",
        ));
    }
    let mut tx = state.db.pool.begin().await.map_err(activity::db)?;
    contact(&mut tx, id, true).await?;
    let note:i64=sqlx::query_scalar("INSERT INTO crm_contact_notes(contact_id,content,created_by) VALUES($1,$2,$3) RETURNING id").bind(id).bind(content).bind(admin.0.id).fetch_one(&mut *tx).await.map_err(activity::db)?;
    crm_logs::record(
        &mut tx,
        "info",
        "api",
        "Contact note added",
        json!({"contact_id":id,"note_id":note}),
        Some(admin.0.id),
    )
    .await
    .map_err(activity::db)?;
    let row:Value=sqlx::query_scalar("SELECT jsonb_build_object('id',n.id::text,'content',n.content,'created_at',n.created_at,'created_by',CASE WHEN u.id IS NULL THEN NULL ELSE jsonb_build_object('id',u.id::text,'name',u.name) END) FROM crm_contact_notes n LEFT JOIN users u ON u.id=n.created_by WHERE n.id=$1")
        .bind(note).fetch_one(&mut *tx).await.map_err(activity::db)?;
    tx.commit().await.map_err(activity::db)?;
    Ok(Json(json!({"contact_id":id.to_string(),"data":row})))
}
async fn lists(
    State(state): State<AppState>,
    _: AdminUser,
    Path(id): Path<i64>,
    Query(page): Query<Page>,
) -> Result<Json<Value>, ApiError> {
    let (page, size, offset) = page.bounds()?;
    let mut tx = state.db.pool.begin().await.map_err(activity::db)?;
    sqlx::query("SET TRANSACTION ISOLATION LEVEL REPEATABLE READ READ ONLY")
        .execute(&mut *tx)
        .await
        .map_err(activity::db)?;
    contact(&mut tx, id, false).await?;
    let total: i64 =
        sqlx::query_scalar("SELECT COUNT(*) FROM crm_list_members WHERE contact_id=$1")
            .bind(id)
            .fetch_one(&mut *tx)
            .await
            .map_err(activity::db)?;
    let rows:Vec<Value>=sqlx::query_scalar("SELECT jsonb_build_object('id',l.id::text,'name',COALESCE(l.title,l.name),'subscribed_at',m.subscribed_at) FROM crm_list_members m JOIN crm_lists l ON l.id=m.list_id WHERE m.contact_id=$1 ORDER BY m.subscribed_at DESC,l.id DESC LIMIT $2 OFFSET $3")
        .bind(id).bind(size).bind(offset).fetch_all(&mut *tx).await.map_err(activity::db)?;
    tx.commit().await.map_err(activity::db)?;
    Ok(envelope(id, rows, page, size, total))
}
async fn tags(
    State(state): State<AppState>,
    _: AdminUser,
    Path(id): Path<i64>,
) -> Result<Json<Value>, ApiError> {
    let mut tx = state.db.pool.begin().await.map_err(activity::db)?;
    sqlx::query("SET TRANSACTION ISOLATION LEVEL REPEATABLE READ READ ONLY")
        .execute(&mut *tx)
        .await
        .map_err(activity::db)?;
    let saved = contact(&mut tx, id, false).await?;
    let values = saved["tags"].as_array().cloned().unwrap_or_default();
    if (!saved["tags"].is_null() && !saved["tags"].is_array())
        || values.len() > 100
        || values.iter().any(|value| !value.is_string())
    {
        return Err(errors::conflict(
            "Review malformed stored contact tags before catalog assignment",
        ));
    }
    let names = values
        .iter()
        .filter_map(Value::as_str)
        .map(str::to_owned)
        .collect::<Vec<_>>();
    let ambiguous:bool=sqlx::query_scalar("SELECT EXISTS(SELECT 1 FROM crm_tags WHERE COALESCE(title,name)=ANY($1) GROUP BY COALESCE(title,name) HAVING COUNT(*)>1)").bind(&names).fetch_one(&mut *tx).await.map_err(activity::db)?;
    if ambiguous {
        return Err(errors::conflict(
            "Several legacy definitions share a tag name; resolve them before assigning identities",
        ));
    }
    let rows:Vec<Value>=sqlx::query_scalar("SELECT jsonb_build_object('id',t.id::text,'name',v.name,'color',t.color,'definition_status',CASE WHEN t.id IS NULL THEN 'unregistered' ELSE 'registered' END) FROM (SELECT name,MIN(position) AS position FROM unnest($1::text[]) WITH ORDINALITY entry(name,position) GROUP BY name) v LEFT JOIN crm_tags t ON COALESCE(t.title,t.name)=v.name ORDER BY v.position")
        .bind(&names).fetch_all(&mut *tx).await.map_err(activity::db)?;
    tx.commit().await.map_err(activity::db)?;
    Ok(Json(json!({"contact_id":id.to_string(),"data":rows})))
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct TagInput {
    tag_id: RecordId,
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct ListInput {
    list_id: RecordId,
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct ContactsInput {
    contact_ids: Vec<RecordId>,
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct BulkTags {
    contact_ids: Vec<RecordId>,
    tag_ids: Vec<RecordId>,
}
async fn membership_lock(
    tx: &mut Transaction<'_, Postgres>,
    actor: i64,
    contacts: &[i64],
) -> Result<(), ApiError> {
    sqlx::query("LOCK TABLE contacts IN SHARE ROW EXCLUSIVE MODE")
        .execute(&mut **tx)
        .await
        .map_err(activity::db)?;
    activity::actor(tx, actor).await?;
    let found: Vec<i64> = sqlx::query_scalar(
        "SELECT id FROM contacts WHERE id=ANY($1) ORDER BY id FOR NO KEY UPDATE",
    )
    .bind(contacts)
    .fetch_all(&mut **tx)
    .await
    .map_err(activity::db)?;
    if found.len() != contacts.len() {
        return Err(errors::not_found("One or more contacts"));
    }
    Ok(())
}
async fn change_tags(
    state: &AppState,
    actor: i64,
    contacts: &[i64],
    tags: &[i64],
    add: bool,
) -> Result<u64, ApiError> {
    let mut tx = state.db.pool.begin().await.map_err(activity::db)?;
    membership_lock(&mut tx, actor, contacts).await?;
    let rows: Vec<(i64, String)> = sqlx::query_as(
        "SELECT id,COALESCE(title,name) FROM crm_tags WHERE id=ANY($1) ORDER BY id FOR UPDATE",
    )
    .bind(tags)
    .fetch_all(&mut *tx)
    .await
    .map_err(activity::db)?;
    if rows.len() != tags.len() {
        return Err(errors::not_found("One or more contact tags"));
    }
    let names = rows
        .iter()
        .map(|(_, name)| name.clone())
        .collect::<Vec<_>>();
    if names.iter().any(|name| {
        name.trim().is_empty() || name.chars().count() > 100 || name.chars().any(char::is_control)
    }) {
        return Err(errors::conflict(
            "Review legacy tag titles before assigning them",
        ));
    }
    let ambiguous:bool=sqlx::query_scalar("SELECT EXISTS(SELECT 1 FROM crm_tags WHERE COALESCE(title,name)=ANY($1) GROUP BY COALESCE(title,name) HAVING COUNT(*)>1)").bind(&names).fetch_one(&mut *tx).await.map_err(activity::db)?;
    if ambiguous {
        return Err(errors::conflict(
            "Several legacy definitions share a tag name; resolve them before changing assignments",
        ));
    }
    let malformed:bool=sqlx::query_scalar("SELECT EXISTS(SELECT 1 FROM contacts c WHERE c.id=ANY($1) AND c.tags IS NOT NULL AND (jsonb_typeof(c.tags)<>'array' OR EXISTS(SELECT 1 FROM jsonb_array_elements(CASE WHEN jsonb_typeof(c.tags)='array' THEN c.tags ELSE '[]'::jsonb END) value WHERE jsonb_typeof(value)<>'string')))").bind(contacts).fetch_one(&mut *tx).await.map_err(activity::db)?;
    if malformed {
        return Err(errors::conflict(
            "Review malformed stored contact tags before changing assignments",
        ));
    }
    if add {
        let too_many:bool=sqlx::query_scalar("SELECT EXISTS(SELECT 1 FROM contacts c WHERE c.id=ANY($1) AND (SELECT COUNT(DISTINCT name) FROM (SELECT jsonb_array_elements_text(COALESCE(c.tags,'[]'::jsonb)) AS name UNION ALL SELECT unnest($2::text[])) names)>100)")
            .bind(contacts).bind(&names).fetch_one(&mut *tx).await.map_err(activity::db)?;
        if too_many {
            return Err(ApiError::validation_error(
                "A contact supports at most100distinct tags",
            ));
        }
    }
    let changed:i64=sqlx::query_scalar("SELECT COUNT(*) FROM contacts c CROSS JOIN unnest($2::text[]) AS requested(name) WHERE c.id=ANY($1) AND CASE WHEN $3 THEN NOT(COALESCE(c.tags,'[]'::jsonb)?requested.name) ELSE COALESCE(c.tags,'[]'::jsonb)?requested.name END")
        .bind(contacts).bind(&names).bind(add).fetch_one(&mut *tx).await.map_err(activity::db)?;
    if add {
        sqlx::query("UPDATE contacts c SET tags=(SELECT COALESCE(jsonb_agg(v ORDER BY position),'[]'::jsonb) FROM (SELECT v,MIN(position) AS position FROM (SELECT value AS v,position FROM jsonb_array_elements(COALESCE(c.tags,'[]'::jsonb)) WITH ORDINALITY entry(value,position) UNION ALL SELECT to_jsonb(name),1000000+position FROM unnest($2::text[]) WITH ORDINALITY entry(name,position)) all_values GROUP BY v) ordered),updated_at=NOW() AT TIME ZONE 'UTC' WHERE c.id=ANY($1) AND NOT(COALESCE(c.tags,'[]'::jsonb)@>to_jsonb($2::text[]))")
            .bind(contacts).bind(&names).execute(&mut *tx).await.map_err(activity::db)?;
    } else {
        sqlx::query("UPDATE contacts c SET tags=(SELECT COALESCE(jsonb_agg(value ORDER BY position),'[]'::jsonb) FROM jsonb_array_elements(c.tags) WITH ORDINALITY entry(value,position) WHERE (value#>>'{}')<>ALL($2::text[])),updated_at=NOW() AT TIME ZONE 'UTC' WHERE c.id=ANY($1) AND COALESCE(c.tags,'[]'::jsonb)?|$2::text[]")
            .bind(contacts).bind(&names).execute(&mut *tx).await.map_err(activity::db)?;
    }
    crm_logs::record(
        &mut tx,
        "info",
        "api",
        if add {
            "Contact tags applied"
        } else {
            "Contact tags removed"
        },
        json!({"contact_ids":contacts,"tag_ids":tags,"changed_count":changed}),
        Some(actor),
    )
    .await
    .map_err(activity::db)?;
    tx.commit().await.map_err(activity::db)?;
    u64::try_from(changed)
        .map_err(|_| errors::service_unavailable("Tag assignment count is invalid"))
}
async fn change_list(
    state: &AppState,
    actor: i64,
    contact: i64,
    list: i64,
    add: bool,
) -> Result<u64, ApiError> {
    let mut tx = state.db.pool.begin().await.map_err(activity::db)?;
    membership_lock(&mut tx, actor, &[contact]).await?;
    let found: Option<i64> = sqlx::query_scalar("SELECT id FROM crm_lists WHERE id=$1 FOR UPDATE")
        .bind(list)
        .fetch_optional(&mut *tx)
        .await
        .map_err(activity::db)?;
    if found.is_none() {
        return Err(errors::not_found("Contact list"));
    }
    let changed = if add {
        sqlx::query(
            "INSERT INTO crm_list_members(list_id,contact_id) VALUES($1,$2) ON CONFLICT DO NOTHING",
        )
        .bind(list)
        .bind(contact)
        .execute(&mut *tx)
        .await
        .map_err(activity::db)?
        .rows_affected()
    } else {
        sqlx::query("DELETE FROM crm_list_members WHERE list_id=$1 AND contact_id=$2")
            .bind(list)
            .bind(contact)
            .execute(&mut *tx)
            .await
            .map_err(activity::db)?
            .rows_affected()
    };
    sqlx::query("UPDATE crm_lists SET contacts_count=(SELECT COUNT(*) FROM crm_list_members WHERE list_id=$1),updated_at=NOW() WHERE id=$1").bind(list).execute(&mut *tx).await.map_err(activity::db)?;
    crm_logs::record(
        &mut tx,
        "info",
        "api",
        if add {
            "Contact added to list"
        } else {
            "Contact removed from list"
        },
        json!({"contact_id":contact,"list_id":list,"changed_count":changed}),
        Some(actor),
    )
    .await
    .map_err(activity::db)?;
    tx.commit().await.map_err(activity::db)?;
    Ok(changed)
}
async fn add_tag(
    State(state): State<AppState>,
    admin: AdminUser,
    Path(id): Path<i64>,
    Json(input): Json<TagInput>,
) -> Result<Json<Value>, ApiError> {
    let tag = input.tag_id.value()?;
    let count = change_tags(&state, admin.0.id, &[id], &[tag], true).await?;
    Ok(Json(
        json!({"contact_id":id.to_string(),"tag_id":tag.to_string(),"applied_count":count}),
    ))
}
async fn remove_tag(
    State(state): State<AppState>,
    admin: AdminUser,
    Path((id, tag)): Path<(i64, i64)>,
) -> Result<Json<Value>, ApiError> {
    if id <= 0 || tag <= 0 {
        return Err(ApiError::validation_error("Record IDs must be positive"));
    }
    let count = change_tags(&state, admin.0.id, &[id], &[tag], false).await?;
    Ok(Json(
        json!({"contact_id":id.to_string(),"tag_id":tag.to_string(),"removed_count":count}),
    ))
}
async fn add_list(
    State(state): State<AppState>,
    admin: AdminUser,
    Path(id): Path<i64>,
    Json(input): Json<ListInput>,
) -> Result<Json<Value>, ApiError> {
    let list = input.list_id.value()?;
    let count = change_list(&state, admin.0.id, id, list, true).await?;
    Ok(Json(
        json!({"contact_id":id.to_string(),"list_id":list.to_string(),"added_count":count}),
    ))
}
async fn remove_list(
    State(state): State<AppState>,
    admin: AdminUser,
    Path((id, list)): Path<(i64, i64)>,
) -> Result<Json<Value>, ApiError> {
    if id <= 0 || list <= 0 {
        return Err(ApiError::validation_error("Record IDs must be positive"));
    }
    let count = change_list(&state, admin.0.id, id, list, false).await?;
    Ok(Json(
        json!({"contact_id":id.to_string(),"list_id":list.to_string(),"removed_count":count}),
    ))
}
async fn tag_apply(
    State(state): State<AppState>,
    admin: AdminUser,
    Path(tag): Path<i64>,
    Json(input): Json<ContactsInput>,
) -> Result<Json<Value>, ApiError> {
    if tag <= 0 {
        return Err(ApiError::validation_error("Tag ID must be positive"));
    }
    let contacts = ids(&input.contact_ids, 1000)?;
    let count = change_tags(&state, admin.0.id, &contacts, &[tag], true).await?;
    Ok(Json(
        json!({"tag_id":tag.to_string(),"applied_count":count,"requested_contacts":contacts.len()}),
    ))
}
async fn tag_remove(
    State(state): State<AppState>,
    admin: AdminUser,
    Path(tag): Path<i64>,
    Json(input): Json<ContactsInput>,
) -> Result<Json<Value>, ApiError> {
    if tag <= 0 {
        return Err(ApiError::validation_error("Tag ID must be positive"));
    }
    let contacts = ids(&input.contact_ids, 1000)?;
    let count = change_tags(&state, admin.0.id, &contacts, &[tag], false).await?;
    Ok(Json(
        json!({"tag_id":tag.to_string(),"removed_count":count,"requested_contacts":contacts.len()}),
    ))
}
async fn bulk_tags(
    State(state): State<AppState>,
    admin: AdminUser,
    Json(input): Json<BulkTags>,
) -> Result<Json<Value>, ApiError> {
    let contacts = ids(&input.contact_ids, 1000)?;
    let tags = ids(&input.tag_ids, 100)?;
    let count = change_tags(&state, admin.0.id, &contacts, &tags, true).await?;
    Ok(Json(
        json!({"applied_count":count,"requested_contacts":contacts.len(),"requested_tags":tags.len()}),
    ))
}
async fn tag_contacts(
    State(state): State<AppState>,
    _: AdminUser,
    Path(tag): Path<i64>,
    Query(page): Query<Page>,
) -> Result<Json<Value>, ApiError> {
    let (page, size, offset) = page.bounds()?;
    let mut tx = state.db.pool.begin().await.map_err(activity::db)?;
    sqlx::query("SET TRANSACTION ISOLATION LEVEL REPEATABLE READ READ ONLY")
        .execute(&mut *tx)
        .await
        .map_err(activity::db)?;
    let name: String = sqlx::query_scalar("SELECT COALESCE(title,name) FROM crm_tags WHERE id=$1")
        .bind(tag)
        .fetch_optional(&mut *tx)
        .await
        .map_err(activity::db)?
        .ok_or_else(|| errors::not_found("Contact tag"))?;
    let total: i64 =
        sqlx::query_scalar("SELECT COUNT(*) FROM contacts WHERE COALESCE(tags,'[]'::jsonb)?$1")
            .bind(&name)
            .fetch_one(&mut *tx)
            .await
            .map_err(activity::db)?;
    let rows:Vec<Value>=sqlx::query_scalar("SELECT to_jsonb(c) FROM contacts c WHERE COALESCE(tags,'[]'::jsonb)?$1 ORDER BY c.id LIMIT $2 OFFSET $3").bind(name).bind(size).bind(offset).fetch_all(&mut *tx).await.map_err(activity::db)?;
    let policy = activity::policy(&mut tx).await?;
    let rows = rows
        .into_iter()
        .map(|row| activity::dto(row, &policy))
        .collect::<Vec<_>>();
    tx.commit().await.map_err(activity::db)?;
    Ok(Json(
        json!({"tag_id":tag.to_string(),"data":rows,"meta":{"current_page":page,"per_page":size,"total":total,"total_pages":((total+size-1)/size).max(1)}}),
    ))
}
pub(super) fn router() -> Router<AppState> {
    Router::new()
        .merge(messages::router())
        .route("/contacts/{id}/notes", get(notes).post(add_note))
        .route("/contacts/{id}/tags", get(tags).post(add_tag))
        .route(
            "/contacts/{id}/tags/{tag}",
            axum::routing::delete(remove_tag),
        )
        .route("/contacts/{id}/lists", get(lists).post(add_list))
        .route(
            "/contacts/{id}/lists/{list}",
            axum::routing::delete(remove_list),
        )
        .route("/contact-tags/{id}/contacts", get(tag_contacts))
        .route("/contact-tags/{id}/apply", post(tag_apply))
        .route("/contact-tags/{id}/remove", post(tag_remove))
        .route("/contact-tags/bulk-apply", post(bulk_tags))
}
