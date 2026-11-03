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

use super::ListFilters;
use crate::{AppState, middleware::admin::AdminUser};

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
}

// ═══════════════════════════════════════════════════════════════════════════
// HANDLERS
// ═══════════════════════════════════════════════════════════════════════════

async fn list_companies(
    State(state): State<AppState>,
    _admin: AdminUser,
    Query(filters): Query<ListFilters>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let per_page = filters.per_page.unwrap_or(50).clamp(1, 100);
    let search_pattern = filters
        .search
        .as_ref()
        .map(|s| format!("%{}%", s.replace('\0', "")));

    let companies: Vec<CrmCompany> = sqlx::query_as(
        r"
        SELECT id, name, website, industry, size, logo_url,
               COALESCE(contacts_count, 0) AS contacts_count,
               COALESCE(deals_count, 0) AS deals_count,
               ROUND(COALESCE(total_deal_value, 0) * 100)::BIGINT AS total_deal_value_cents,
               created_at, updated_at
        FROM crm_companies
        WHERE ($1::text IS NULL OR name ILIKE $1)
        ORDER BY created_at DESC
        LIMIT $2
        ",
    )
    .bind(search_pattern.as_deref())
    .bind(per_page)
    .fetch_all(&state.db.pool)
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": e.to_string()})),
        )
    })?;

    Ok(Json(json!({
        "data": companies,
        "meta": { "total": companies.len() }
    })))
}

async fn create_company(
    State(state): State<AppState>,
    _admin: AdminUser,
    Json(input): Json<CreateCompanyInput>,
) -> Result<Json<CrmCompany>, (StatusCode, Json<serde_json::Value>)> {
    let company: CrmCompany = sqlx::query_as(
        r"
        INSERT INTO crm_companies (name, website, industry, size, contacts_count, deals_count, total_deal_value, created_at, updated_at)
        VALUES ($1, $2, $3, $4, 0, 0, 0.0, NOW(), NOW())
        RETURNING id, name, website, industry, size, logo_url,
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
    .fetch_one(&state.db.pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()}))))?;

    Ok(Json(company))
}

async fn get_company(
    State(state): State<AppState>,
    _admin: AdminUser,
    Path(id): Path<i64>,
) -> Result<Json<CrmCompany>, (StatusCode, Json<serde_json::Value>)> {
    let company: CrmCompany = sqlx::query_as(
        r"SELECT id, name, website, industry, size, logo_url,
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
        .route("/companies/{id}", get(get_company).delete(delete_company))
}
