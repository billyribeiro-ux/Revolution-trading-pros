//! CRM contact-lists + tags handlers
//! (split from `crm.rs` lines 90-112 types + 504-713 handlers).

use axum::{
    Json, Router,
    extract::{Path, Query, State},
    http::StatusCode,
    routing::get,
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use sqlx::FromRow;

use super::ListFilters;
use crate::{AppState, middleware::admin::AdminUser};

// ═══════════════════════════════════════════════════════════════════════════
// TYPES
// ═══════════════════════════════════════════════════════════════════════════

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct ContactList {
    pub id: i64,
    pub title: String,
    pub slug: String,
    pub description: Option<String>,
    pub is_public: bool,
    pub contacts_count: i32,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct ContactTag {
    pub id: i64,
    pub title: String,
    pub slug: String,
    pub description: Option<String>,
    pub color: Option<String>,
    pub contacts_count: i32,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Deserialize)]
pub struct CreateListInput {
    pub title: String,
    pub description: Option<String>,
    pub is_public: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct CreateTagInput {
    pub title: String,
    pub description: Option<String>,
    pub color: Option<String>,
}

// ═══════════════════════════════════════════════════════════════════════════
// HANDLERS - Lists
// ═══════════════════════════════════════════════════════════════════════════

#[derive(Deserialize)]
struct ContactListFilters {
    page: Option<i64>,
    per_page: Option<i64>,
    search: Option<String>,
    is_public: Option<bool>,
}

async fn list_contact_lists(
    State(state): State<AppState>,
    _admin: AdminUser,
    Query(filters): Query<ContactListFilters>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let per_page = filters.per_page.unwrap_or(50).clamp(1, 100);
    let page = filters.page.unwrap_or(1).max(1);
    let search_pattern = filters
        .search
        .as_ref()
        .map(|s| format!("%{}%", s.replace('\0', "")));

    let lists: Vec<ContactList> = sqlx::query_as(
        r"
        SELECT id, COALESCE(title, name) AS title,
               COALESCE(slug, LOWER(REPLACE(name, ' ', '-'))) AS slug,
               description, COALESCE(is_public, false) AS is_public,
               (SELECT COUNT(*) FROM crm_list_members WHERE list_id=crm_lists.id)::int AS contacts_count, created_at, updated_at
        FROM crm_lists
        WHERE ($1::text IS NULL OR COALESCE(title, name) ILIKE $1)
          AND ($2::boolean IS NULL OR COALESCE(is_public,false) = $2)
        ORDER BY created_at DESC, id DESC
        LIMIT $3 OFFSET $4
        ",
    )
    .bind(search_pattern.as_deref())
    .bind(filters.is_public)
    .bind(per_page)
    .bind(page.saturating_sub(1).saturating_mul(per_page))
    .fetch_all(&state.db.pool)
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": e.to_string()})),
        )
    })?;

    let total: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM crm_lists WHERE ($1::text IS NULL OR COALESCE(title,name) ILIKE $1) AND ($2::boolean IS NULL OR COALESCE(is_public,false)=$2)")
        .bind(search_pattern.as_deref()).bind(filters.is_public).fetch_one(&state.db.pool).await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR,Json(json!({"error":e.to_string()}))))?;
    Ok(Json(json!({
        "data": lists,
        "meta": { "total": total, "current_page": page, "per_page": per_page, "total_pages": ((total+per_page-1)/per_page).max(1) }
    })))
}

fn validate_list(input: &mut CreateListInput) -> Result<(), (StatusCode, Json<serde_json::Value>)> {
    input.title = input.title.trim().to_string();
    if input.title.is_empty()
        || input.title.len() > 255
        || input.description.as_ref().is_some_and(|s| s.len() > 50_000)
    {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(
                json!({"error":"Provide a list title under 255 characters and description under 50 KB"}),
            ),
        ));
    }
    Ok(())
}

async fn update_contact_list(
    State(state): State<AppState>,
    _admin: AdminUser,
    Path(id): Path<i64>,
    Json(mut input): Json<CreateListInput>,
) -> Result<Json<ContactList>, (StatusCode, Json<serde_json::Value>)> {
    validate_list(&mut input)?;
    let result = sqlx::query_as("UPDATE crm_lists SET name=$2,title=$2,description=$3,is_public=$4,updated_at=NOW() WHERE id=$1 RETURNING id,COALESCE(title,name) AS title,COALESCE(slug,'') AS slug,description,COALESCE(is_public,false) AS is_public,(SELECT COUNT(*) FROM crm_list_members WHERE list_id=crm_lists.id)::int AS contacts_count,created_at,updated_at")
        .bind(id).bind(input.title).bind(input.description).bind(input.is_public.unwrap_or(false))
        .fetch_optional(&state.db.pool).await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR,Json(json!({"error":e.to_string()}))))?
        .ok_or_else(|| (StatusCode::NOT_FOUND,Json(json!({"error":"List not found"}))))?;
    Ok(Json(result))
}

async fn create_contact_list(
    State(state): State<AppState>,
    _admin: AdminUser,
    Json(mut input): Json<CreateListInput>,
) -> Result<Json<ContactList>, (StatusCode, Json<serde_json::Value>)> {
    validate_list(&mut input)?;
    let slug = input.title.to_lowercase().replace(' ', "-");

    let list: ContactList = sqlx::query_as(
        r"
        INSERT INTO crm_lists (name, title, slug, description, is_public, contacts_count, created_at, updated_at)
        VALUES ($1, $1, $2, $3, $4, 0, NOW(), NOW())
        RETURNING id, COALESCE(title, name) AS title, COALESCE(slug, '') AS slug, description,
                  COALESCE(is_public, false) AS is_public, (SELECT COUNT(*) FROM crm_list_members WHERE list_id=crm_lists.id)::int AS contacts_count,
                  created_at, updated_at
        ",
    )
    .bind(&input.title)
    .bind(&slug)
    .bind(&input.description)
    .bind(input.is_public.unwrap_or(false))
    .fetch_one(&state.db.pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()}))))?;

    Ok(Json(list))
}

async fn get_contact_list(
    State(state): State<AppState>,
    _admin: AdminUser,
    Path(id): Path<i64>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let list: ContactList = sqlx::query_as(
        r"SELECT id, COALESCE(title, name) AS title,
                 COALESCE(slug, LOWER(REPLACE(name, ' ', '-'))) AS slug,
                 description, COALESCE(is_public, false) AS is_public,
                 (SELECT COUNT(*) FROM crm_list_members WHERE list_id=crm_lists.id)::int AS contacts_count, created_at, updated_at
          FROM crm_lists WHERE id = $1",
    )
    .bind(id)
    .fetch_optional(&state.db.pool)
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": e.to_string()})),
        )
    })?
    .ok_or_else(|| {
        (
            StatusCode::NOT_FOUND,
            Json(json!({"error": "List not found"})),
        )
    })?;

    Ok(Json(json!({
        "list": list,
        "contacts_count": list.contacts_count
    })))
}

async fn delete_contact_list(
    State(state): State<AppState>,
    _admin: AdminUser,
    Path(id): Path<i64>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let result = sqlx::query("DELETE FROM crm_lists WHERE id = $1")
        .bind(id)
        .execute(&state.db.pool)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": e.to_string()})),
            )
        })?;

    if result.rows_affected() == 0 {
        return Err((
            StatusCode::NOT_FOUND,
            Json(json!({"error": "List not found"})),
        ));
    }

    Ok(Json(json!({"message": "List deleted successfully"})))
}

// ═══════════════════════════════════════════════════════════════════════════
// HANDLERS - Tags
// ═══════════════════════════════════════════════════════════════════════════

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct TagFilters {
    search: Option<String>,
    page: Option<i64>,
    per_page: Option<i64>,
}
type TagError = (StatusCode, Json<serde_json::Value>);
fn tag_invalid(message: &str) -> TagError {
    (StatusCode::BAD_REQUEST, Json(json!({"error":message})))
}
fn tag_db(error: sqlx::Error) -> TagError {
    if error
        .as_database_error()
        .is_some_and(|error| error.is_unique_violation())
    {
        (
            StatusCode::CONFLICT,
            Json(json!({"error":"Tag with this title already exists"})),
        )
    } else {
        tracing::error!(%error,"CRM tag persistence failed");
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error":"Unable to save contact tag"})),
        )
    }
}
fn validate_tag(input: &mut CreateTagInput) -> Result<(), TagError> {
    input.title = input.title.trim().to_string();
    if input.title.is_empty()
        || input.title.chars().count() > 100
        || input.title.chars().any(char::is_control)
        || input
            .description
            .as_ref()
            .is_some_and(|value| value.len() > 50000 || value.contains('\0'))
    {
        return Err(tag_invalid(
            "Provide a tag title of1–100characters and a description under50KiB without null characters",
        ));
    }
    if input.color.as_ref().is_some_and(|value| {
        value.len() != 7
            || !value.starts_with('#')
            || !value[1..].bytes().all(|value| value.is_ascii_hexdigit())
    }) {
        return Err(tag_invalid("Tag color must use #RRGGBB"));
    }
    Ok(())
}
async fn tag_row(connection: &mut sqlx::PgConnection, id: i64) -> Result<ContactTag, TagError> {
    sqlx::query_as("SELECT id,COALESCE(title,name) AS title,COALESCE(slug,'') AS slug,description,color,(SELECT COUNT(*) FROM contacts c WHERE COALESCE(c.tags,'[]'::jsonb)?COALESCE(crm_tags.title,crm_tags.name))::int AS contacts_count,created_at,updated_at FROM crm_tags WHERE id=$1")
        .bind(id).fetch_optional(connection).await.map_err(tag_db)?.ok_or_else(||(StatusCode::NOT_FOUND,Json(json!({"error":"Contact tag not found"}))))
}
async fn list_contact_tags(
    State(state): State<AppState>,
    _admin: AdminUser,
    Query(filters): Query<TagFilters>,
) -> Result<Json<serde_json::Value>, TagError> {
    let page = filters.page.unwrap_or(1);
    let size = filters.per_page.unwrap_or(50);
    if !(1..=1_000_000).contains(&page)
        || !(1..=100).contains(&size)
        || filters
            .search
            .as_ref()
            .is_some_and(|value| value.len() > 500 || value.contains('\0'))
    {
        return Err(tag_invalid(
            "Use a positive page, page size1–100 and bounded search text",
        ));
    }
    let search = filters
        .search
        .map(|value| format!("%{}%", crate::utils::escape_like(&value)));
    let tags:Vec<ContactTag>=sqlx::query_as("SELECT id,COALESCE(title,name) AS title,COALESCE(slug,'') AS slug,description,color,(SELECT COUNT(*) FROM contacts c WHERE COALESCE(c.tags,'[]'::jsonb)?COALESCE(crm_tags.title,crm_tags.name))::int AS contacts_count,created_at,updated_at FROM crm_tags WHERE ($1::text IS NULL OR COALESCE(title,name) ILIKE $1) ORDER BY created_at DESC,id DESC LIMIT $2 OFFSET $3")
        .bind(&search).bind(size).bind((page-1)*size).fetch_all(&state.db.pool).await.map_err(tag_db)?;
    let total: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM crm_tags WHERE $1::text IS NULL OR COALESCE(title,name) ILIKE $1",
    )
    .bind(&search)
    .fetch_one(&state.db.pool)
    .await
    .map_err(tag_db)?;
    Ok(Json(
        json!({"data":tags,"meta":{"total":total,"current_page":page,"per_page":size,"total_pages":((total+size-1)/size).max(1)}}),
    ))
}
async fn get_contact_tag(
    State(state): State<AppState>,
    _admin: AdminUser,
    Path(id): Path<i64>,
) -> Result<Json<serde_json::Value>, TagError> {
    if id <= 0 {
        return Err(tag_invalid("Tag ID must be positive"));
    }
    let tag = tag_row(&mut *state.db.pool.acquire().await.map_err(tag_db)?, id).await?;
    Ok(Json(json!({"contacts_count":tag.contacts_count,"tag":tag})))
}
async fn save_contact_tag(
    state: AppState,
    actor: i64,
    id: Option<i64>,
    mut input: CreateTagInput,
) -> Result<Json<ContactTag>, TagError> {
    validate_tag(&mut input)?;
    if id.is_some_and(|id| id <= 0) {
        return Err(tag_invalid("Tag ID must be positive"));
    }
    let mut tx = state.db.pool.begin().await.map_err(tag_db)?;
    // Rename/delete and import all acquire the contacts write lock before
    // catalog rows. This avoids losing name-based assignments or deadlocks.
    sqlx::query("LOCK TABLE contacts IN SHARE ROW EXCLUSIVE MODE")
        .execute(&mut *tx)
        .await
        .map_err(tag_db)?;
    let duplicate:bool=sqlx::query_scalar("SELECT EXISTS(SELECT 1 FROM crm_tags WHERE LOWER(COALESCE(title,name))=LOWER($1) AND ($2::bigint IS NULL OR id<>$2))")
        .bind(&input.title).bind(id).fetch_one(&mut *tx).await.map_err(tag_db)?;
    if duplicate {
        return Err((
            StatusCode::CONFLICT,
            Json(json!({"error":"Tag with this title already exists"})),
        ));
    }
    let slug = slug::slugify(&input.title);
    let saved = if let Some(id) = id {
        let old: String =
            sqlx::query_scalar("SELECT COALESCE(title,name) FROM crm_tags WHERE id=$1 FOR UPDATE")
                .bind(id)
                .fetch_optional(&mut *tx)
                .await
                .map_err(tag_db)?
                .ok_or_else(|| {
                    (
                        StatusCode::NOT_FOUND,
                        Json(json!({"error":"Contact tag not found"})),
                    )
                })?;
        if old != input.title {
            let ambiguous: bool = sqlx::query_scalar(
                "SELECT EXISTS(SELECT 1 FROM crm_tags WHERE COALESCE(title,name)=$1 AND id<>$2)",
            )
            .bind(&old)
            .bind(id)
            .fetch_one(&mut *tx)
            .await
            .map_err(tag_db)?;
            if ambiguous {
                return Err((
                    StatusCode::CONFLICT,
                    Json(
                        json!({"error":"This legacy title identifies several tags; resolve duplicate definitions before renaming"}),
                    ),
                ));
            }
            sqlx::query("UPDATE contacts c SET tags=(SELECT COALESCE(jsonb_agg(v ORDER BY n),'[]'::jsonb) FROM (SELECT CASE WHEN value=to_jsonb($1::text) THEN to_jsonb($2::text) ELSE value END AS v,MIN(position) AS n FROM jsonb_array_elements(c.tags) WITH ORDINALITY AS entry(value,position) GROUP BY 1) preserved),updated_at=NOW() AT TIME ZONE 'UTC' WHERE jsonb_typeof(tags)='array' AND tags?$1")
                .bind(&old).bind(&input.title).execute(&mut *tx).await.map_err(tag_db)?;
        }
        sqlx::query("UPDATE crm_tags SET name=$2,title=$2,slug=$3,description=$4,color=COALESCE($5,color),updated_at=NOW() WHERE id=$1").bind(id).bind(&input.title).bind(&slug).bind(&input.description).bind(&input.color).execute(&mut *tx).await.map_err(tag_db)?;
        id
    } else {
        sqlx::query_scalar("INSERT INTO crm_tags(name,title,slug,description,color) VALUES($1,$1,$2,$3,COALESCE($4,'#6366f1')) RETURNING id").bind(&input.title).bind(&slug).bind(&input.description).bind(&input.color).fetch_one(&mut *tx).await.map_err(tag_db)?
    };
    crate::services::crm_logs::record(
        &mut tx,
        "info",
        "api",
        if id.is_some() {
            "Contact tag updated"
        } else {
            "Contact tag created"
        },
        json!({"tag_id":saved.to_string()}),
        Some(actor),
    )
    .await
    .map_err(tag_db)?;
    let tag = tag_row(&mut tx, saved).await?;
    tx.commit().await.map_err(tag_db)?;
    Ok(Json(tag))
}
async fn create_contact_tag(
    State(state): State<AppState>,
    admin: AdminUser,
    Json(input): Json<CreateTagInput>,
) -> Result<Json<ContactTag>, TagError> {
    save_contact_tag(state, admin.0.id, None, input).await
}
async fn update_contact_tag(
    State(state): State<AppState>,
    admin: AdminUser,
    Path(id): Path<i64>,
    Json(input): Json<CreateTagInput>,
) -> Result<Json<ContactTag>, TagError> {
    save_contact_tag(state, admin.0.id, Some(id), input).await
}
async fn delete_contact_tag(
    State(state): State<AppState>,
    admin: AdminUser,
    Path(id): Path<i64>,
) -> Result<Json<serde_json::Value>, TagError> {
    if id <= 0 {
        return Err(tag_invalid("Tag ID must be positive"));
    }
    let mut tx = state.db.pool.begin().await.map_err(tag_db)?;
    sqlx::query("LOCK TABLE contacts IN SHARE ROW EXCLUSIVE MODE")
        .execute(&mut *tx)
        .await
        .map_err(tag_db)?;
    let title: String =
        sqlx::query_scalar("SELECT COALESCE(title,name) FROM crm_tags WHERE id=$1 FOR UPDATE")
            .bind(id)
            .fetch_optional(&mut *tx)
            .await
            .map_err(tag_db)?
            .ok_or_else(|| {
                (
                    StatusCode::NOT_FOUND,
                    Json(json!({"error":"Contact tag not found"})),
                )
            })?;
    let shared: bool = sqlx::query_scalar(
        "SELECT EXISTS(SELECT 1 FROM crm_tags WHERE COALESCE(title,name)=$1 AND id<>$2)",
    )
    .bind(&title)
    .bind(id)
    .fetch_one(&mut *tx)
    .await
    .map_err(tag_db)?;
    if !shared {
        sqlx::query("UPDATE contacts SET tags=tags-$1,updated_at=NOW() AT TIME ZONE 'UTC' WHERE jsonb_typeof(tags)='array' AND tags?$1").bind(&title).execute(&mut *tx).await.map_err(tag_db)?;
    }
    sqlx::query("DELETE FROM crm_tags WHERE id=$1")
        .bind(id)
        .execute(&mut *tx)
        .await
        .map_err(tag_db)?;
    crate::services::crm_logs::record(
        &mut tx,
        "info",
        "api",
        "Contact tag deleted",
        json!({"tag_id":id.to_string()}),
        Some(admin.0.id),
    )
    .await
    .map_err(tag_db)?;
    tx.commit().await.map_err(tag_db)?;
    Ok(Json(json!({"deleted":true,"id":id.to_string()})))
}

// ═══════════════════════════════════════════════════════════════════════════
// ROUTER
// ═══════════════════════════════════════════════════════════════════════════

pub(super) fn router() -> Router<AppState> {
    Router::new()
        .route("/lists", get(list_contact_lists).post(create_contact_list))
        .route(
            "/lists/{id}",
            get(get_contact_list)
                .put(update_contact_list)
                .delete(delete_contact_list),
        )
        .route(
            "/contact-tags",
            get(list_contact_tags).post(create_contact_tag),
        )
        .route(
            "/contact-tags/{id}",
            get(get_contact_tag)
                .put(update_contact_tag)
                .delete(delete_contact_tag),
        )
}
