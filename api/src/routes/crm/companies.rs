//! CRM company handlers
//! (split from `crm.rs` lines 245-258 types + 1492-1598 handlers).

use axum::{
    Json, Router,
    extract::{Path, Query, State},
    http::StatusCode,
    routing::get,
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use sqlx::FromRow;

use crate::{AppState, routes::crm::manager_permissions::CrmUser as AdminUser};

// ═══════════════════════════════════════════════════════════════════════════
// TYPES
// ═══════════════════════════════════════════════════════════════════════════

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct CrmCompany {
    pub id: i64,
    pub name: String,
    pub website: Option<String>,
    pub industry: Option<String>,
    pub size: Option<String>,
    pub logo_url: Option<String>,
    pub description: Option<String>,
    pub contacts_count: i32,
    pub deals_count: i32,
    pub total_deal_value_cents: i64,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Deserialize)]
pub struct CreateCompanyInput {
    pub name: String,
    pub website: Option<String>,
    pub industry: Option<String>,
    pub size: Option<String>,
    pub logo_url: Option<String>,
    pub description: Option<String>,
}

// ═══════════════════════════════════════════════════════════════════════════
// HANDLERS
// ═══════════════════════════════════════════════════════════════════════════

#[derive(Deserialize)]
struct CompanyFilters {
    page: Option<i64>,
    per_page: Option<i64>,
    search: Option<String>,
    industry: Option<String>,
    size: Option<String>,
}

async fn list_companies(
    State(state): State<AppState>,
    _admin: AdminUser,
    Query(filters): Query<CompanyFilters>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let per_page = filters.per_page.unwrap_or(50).clamp(1, 100);
    let page = filters.page.unwrap_or(1).max(1);
    let offset = page.saturating_sub(1).saturating_mul(per_page);
    let search_pattern = filters
        .search
        .as_ref()
        .map(|s| format!("%{}%", s.replace('\0', "")));

    let companies: Vec<CrmCompany> = sqlx::query_as(
        r"
        SELECT id, name, website, industry, size, logo_url, notes AS description,
               COALESCE(contacts_count, 0) AS contacts_count,
               COALESCE(deals_count, 0) AS deals_count,
               ROUND(COALESCE(total_deal_value, 0) * 100)::BIGINT AS total_deal_value_cents,
               created_at, updated_at
        FROM crm_companies
        WHERE ($1::text IS NULL OR name ILIKE $1 OR website ILIKE $1)
          AND ($3::text IS NULL OR industry=$3) AND ($4::text IS NULL OR size=$4)
        ORDER BY created_at DESC, id DESC
        LIMIT $2 OFFSET $5
        ",
    )
    .bind(search_pattern.as_deref())
    .bind(per_page)
    .bind(&filters.industry)
    .bind(&filters.size)
    .bind(offset)
    .fetch_all(&state.db.pool)
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": e.to_string()})),
        )
    })?;

    let total: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM crm_companies WHERE ($1::text IS NULL OR name ILIKE $1 OR website ILIKE $1) AND ($2::text IS NULL OR industry=$2) AND ($3::text IS NULL OR size=$3)")
        .bind(search_pattern.as_deref()).bind(&filters.industry).bind(&filters.size)
        .fetch_one(&state.db.pool).await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR,Json(json!({"error":e.to_string()}))))?;

    Ok(Json(json!({
        "data": companies,
        "meta": { "total": total, "current_page": page, "per_page": per_page, "total_pages": ((total + per_page - 1) / per_page).max(1) }
    })))
}

async fn create_company(
    State(state): State<AppState>,
    _admin: AdminUser,
    Json(mut input): Json<CreateCompanyInput>,
) -> Result<Json<CrmCompany>, (StatusCode, Json<serde_json::Value>)> {
    validate_company(&mut input)?;
    let company: CrmCompany = sqlx::query_as(
        r"
        INSERT INTO crm_companies (name, website, industry, size, logo_url, notes, contacts_count, deals_count, total_deal_value, created_at, updated_at)
        VALUES ($1, $2, $3, $4, $5, $6, 0, 0, 0.0, NOW(), NOW())
        RETURNING id, name, website, industry, size, logo_url, notes AS description,
                  COALESCE(contacts_count, 0) AS contacts_count,
                  COALESCE(deals_count, 0) AS deals_count,
                  ROUND(COALESCE(total_deal_value, 0) * 100)::BIGINT AS total_deal_value_cents,
                  created_at, updated_at
        ",
    )
    .bind(&input.name)
    .bind(&input.website)
    .bind(&input.industry)
    .bind(&input.size)
    .bind(&input.logo_url)
    .bind(&input.description)
    .fetch_one(&state.db.pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()}))))?;

    Ok(Json(company))
}

fn validate_company(
    input: &mut CreateCompanyInput,
) -> Result<(), (StatusCode, Json<serde_json::Value>)> {
    let invalid = |message: &str| (StatusCode::BAD_REQUEST, Json(json!({"error":message})));
    input.name = input.name.trim().to_string();
    if input.name.is_empty() || input.name.len() > 255 {
        return Err(invalid("Company name must be 1–255 characters"));
    }
    for value in [&mut input.website, &mut input.logo_url] {
        if value.as_ref().is_some_and(|s| s.trim().is_empty()) {
            *value = None;
        }
        if let Some(url) = value
            && (url.len() > 500
                || !reqwest::Url::parse(url).is_ok_and(|u| {
                    matches!(u.scheme(), "http" | "https")
                        && u.host_str().is_some()
                        && u.username().is_empty()
                        && u.password().is_none()
                }))
        {
            return Err(invalid(
                "Website and logo must be valid HTTP(S) URLs under 500 characters",
            ));
        }
    }
    if input.industry.as_ref().is_some_and(|s| s.len() > 100)
        || input.size.as_ref().is_some_and(|s| s.len() > 50)
        || input.description.as_ref().is_some_and(|s| s.len() > 50_000)
    {
        return Err(invalid("Company details exceed field limits"));
    }
    Ok(())
}

async fn update_company(
    State(state): State<AppState>,
    _admin: AdminUser,
    Path(id): Path<i64>,
    Json(mut input): Json<CreateCompanyInput>,
) -> Result<Json<CrmCompany>, (StatusCode, Json<serde_json::Value>)> {
    validate_company(&mut input)?;
    let company = sqlx::query_as("UPDATE crm_companies SET name=$2,website=$3,industry=$4,size=$5,logo_url=$6,notes=$7,updated_at=NOW() WHERE id=$1 RETURNING id,name,website,industry,size,logo_url,notes AS description,COALESCE(contacts_count,0) AS contacts_count,COALESCE(deals_count,0) AS deals_count,ROUND(COALESCE(total_deal_value,0)*100)::bigint AS total_deal_value_cents,created_at,updated_at")
        .bind(id).bind(input.name).bind(input.website).bind(input.industry).bind(input.size).bind(input.logo_url).bind(input.description)
        .fetch_optional(&state.db.pool).await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR,Json(json!({"error":e.to_string()}))))?
        .ok_or_else(|| (StatusCode::NOT_FOUND,Json(json!({"error":"Company not found"}))))?;
    Ok(Json(company))
}

async fn duplicate_company(
    State(state): State<AppState>,
    _admin: AdminUser,
    Path(id): Path<i64>,
) -> Result<Json<CrmCompany>, (StatusCode, Json<serde_json::Value>)> {
    let company = sqlx::query_as("INSERT INTO crm_companies (name,website,industry,size,logo_url,notes,created_at,updated_at) SELECT LEFT(name,248)||' (copy)',website,industry,size,logo_url,notes,NOW(),NOW() FROM crm_companies WHERE id=$1 RETURNING id,name,website,industry,size,logo_url,notes AS description,COALESCE(contacts_count,0) AS contacts_count,COALESCE(deals_count,0) AS deals_count,ROUND(COALESCE(total_deal_value,0)*100)::bigint AS total_deal_value_cents,created_at,updated_at")
        .bind(id).fetch_optional(&state.db.pool).await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR,Json(json!({"error":e.to_string()}))))?
        .ok_or_else(|| (StatusCode::NOT_FOUND,Json(json!({"error":"Company not found"}))))?;
    Ok(Json(company))
}

async fn get_company(
    State(state): State<AppState>,
    _admin: AdminUser,
    Path(id): Path<i64>,
) -> Result<Json<CrmCompany>, (StatusCode, Json<serde_json::Value>)> {
    let company: CrmCompany = sqlx::query_as(
        r"SELECT id, name, website, industry, size, logo_url, notes AS description,
                 COALESCE(contacts_count, 0) AS contacts_count,
                 COALESCE(deals_count, 0) AS deals_count,
                 ROUND(COALESCE(total_deal_value, 0) * 100)::BIGINT AS total_deal_value_cents,
                 created_at, updated_at
          FROM crm_companies WHERE id = $1",
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
            Json(json!({"error": "Company not found"})),
        )
    })?;

    Ok(Json(company))
}

async fn delete_company(
    State(state): State<AppState>,
    _admin: AdminUser,
    Path(id): Path<i64>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let result = sqlx::query("DELETE FROM crm_companies WHERE id = $1")
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
            Json(json!({"error": "Company not found"})),
        ));
    }

    Ok(Json(json!({"message": "Company deleted successfully"})))
}

// ═══════════════════════════════════════════════════════════════════════════
// ROUTER
// ═══════════════════════════════════════════════════════════════════════════

pub(super) fn router() -> Router<AppState> {
    Router::new()
        .route("/companies", get(list_companies).post(create_company))
        .route(
            "/companies/{id}",
            get(get_company).put(update_company).delete(delete_company),
        )
        .route(
            "/companies/{id}/duplicate",
            axum::routing::post(duplicate_company),
        )
}
