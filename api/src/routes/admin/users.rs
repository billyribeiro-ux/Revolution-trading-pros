//! Admin user management (admin staff) — extracted from the original
//! `routes/admin.rs` as part of the R6-B split (2026-05-20).
//!
//! Handler bodies are byte-identical to the pre-split source.

use axum::{
    Extension, Json,
    extract::{Path, Query, State},
    http::{HeaderMap, StatusCode},
};
use serde::Deserialize;
use serde_json::json;

use crate::routes::admin_member_management::account_actions as accounts;
use crate::{
    AppState,
    middleware::admin::{AdminUser, SuperAdminUser},
    services::account_authority::VerifiedAuthority,
    utils::errors::ApiError,
};
use axum::response::{IntoResponse, Response};
use uuid::Uuid;

// ═══════════════════════════════════════════════════════════════════════════
// USER MANAGEMENT (Admin Staff)
// ═══════════════════════════════════════════════════════════════════════════

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct UserListQuery {
    pub page: Option<i64>,
    pub per_page: Option<i64>,
    pub role: Option<String>,
    pub search: Option<String>,
    pub is_active: Option<bool>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CreateUserRequest {
    pub request_id: Uuid,
    pub name: String,
    pub email: String,
    pub password: String,
    pub role: Option<String>,
    pub roles: Option<Vec<String>>,
    pub send_welcome_email: Option<bool>,
}
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct UpdateUserRequest {
    pub expected_version: i64,
    pub name: Option<String>,
    pub email: Option<String>,
    pub role: Option<String>,
    pub roles: Option<Vec<String>>,
    pub is_active: Option<bool>,
    pub password: Option<String>,
    #[serde(default, deserialize_with = "accounts::nullable_patch")]
    pub avatar_url: Option<Option<String>>,
}
fn one_role(role: Option<String>, roles: Option<Vec<String>>) -> Result<Option<String>, ApiError> {
    if role.is_some() && roles.is_some() {
        return Err(ApiError::validation_error(
            "Provide either one role or a one-role array",
        ));
    }
    if let Some(roles) = roles {
        if roles.len() != 1 {
            return Err(ApiError::validation_error(
                "The account stores exactly one role; provide one role",
            ));
        }
        return Ok(roles.into_iter().next());
    }
    Ok(role)
}

/// List all users (admin)
/// ICT 11+ SECURITY FIX: Refactored to use safe parameterized queries with optional filters
/// ICT 7 SECURITY FIX: Added require_admin check - was missing!
fn storage(error: sqlx::Error) -> ApiError {
    tracing::error!(%error,"Admin account read failed");
    ApiError::new(
        StatusCode::SERVICE_UNAVAILABLE,
        "Account records are unavailable",
    )
}
fn private(value: serde_json::Value) -> Response {
    (
        [
            (axum::http::header::CACHE_CONTROL, "private, no-store"),
            (axum::http::header::VARY, "Authorization, Cookie"),
        ],
        Json(value),
    )
        .into_response()
}
pub(super) async fn list_users(
    State(state): State<AppState>,
    _admin: AdminUser,
    Query(query): Query<UserListQuery>,
) -> Result<Response, ApiError> {
    let page = query.page.unwrap_or(1);
    let size = query.per_page.unwrap_or(50);
    if page < 1
        || !(1..=100).contains(&size)
        || query
            .search
            .as_ref()
            .is_some_and(|v| v.chars().count() > 200 || v.chars().any(char::is_control))
        || query
            .role
            .as_ref()
            .is_some_and(|v| v.len() > 50 || v.chars().any(char::is_control))
    {
        return Err(ApiError::validation_error(
            "Provide bounded account filters and positive pagination",
        ));
    }
    let offset = (page - 1)
        .checked_mul(size)
        .ok_or_else(|| ApiError::validation_error("Requested page is too large"))?;
    let search = query.search.as_ref().map(|s| format!("%{s}%"));
    let mut tx = state.db.pool.begin().await.map_err(storage)?;
    sqlx::query("SET TRANSACTION ISOLATION LEVEL REPEATABLE READ READ ONLY")
        .execute(&mut *tx)
        .await
        .map_err(storage)?;
    let total:i64=sqlx::query_scalar("SELECT COUNT(*) FROM users WHERE deleted_at IS NULL AND COALESCE(LOWER(role),'')<>'deleted' AND($1::text IS NULL OR LOWER(role)=LOWER($1)) AND($2::boolean IS NULL OR is_active=$2) AND($3::text IS NULL OR name ILIKE $3 OR email ILIKE $3)").bind(&query.role).bind(query.is_active).bind(&search).fetch_one(&mut *tx).await.map_err(storage)?;
    let data:Vec<serde_json::Value>=sqlx::query_scalar("SELECT jsonb_build_object('id',id,'name',name,'email',email,'role',role,'is_active',is_active,'email_verified_at',email_verified_at,'last_login_at',last_login_at,'created_at',created_at,'updated_at',updated_at,'version',account_version) FROM users WHERE deleted_at IS NULL AND COALESCE(LOWER(role),'')<>'deleted' AND($1::text IS NULL OR LOWER(role)=LOWER($1)) AND($2::boolean IS NULL OR is_active=$2) AND($3::text IS NULL OR name ILIKE $3 OR email ILIKE $3) ORDER BY created_at DESC NULLS LAST,id DESC LIMIT $4 OFFSET $5").bind(&query.role).bind(query.is_active).bind(search).bind(size).bind(offset).fetch_all(&mut *tx).await.map_err(storage)?;
    tx.commit().await.map_err(storage)?;
    Ok(private(
        json!({"data":data,"meta":{"current_page":page,"per_page":size,"total":total,"total_pages":(total/size+i64::from(total%size!=0)).max(1)},"timestamp_basis":"Recorded naive dates have no historical timezone provenance"}),
    ))
}

/// Get user by ID (admin)
/// ICT 7 SECURITY FIX: Added require_admin check
pub(super) async fn get_user(
    state: State<AppState>,
    admin: AdminUser,
    path: Path<i64>,
) -> Result<Response, ApiError> {
    crate::routes::admin_member_management::read_handlers::show(state, admin, path).await
}
pub(super) async fn create_user(
    state: State<AppState>,
    admin: SuperAdminUser,
    verified: Extension<VerifiedAuthority>,
    Json(input): Json<CreateUserRequest>,
) -> Result<Response, ApiError> {
    accounts::create_member(
        state,
        admin,
        verified,
        Json(accounts::CreateMemberRequest {
            request_id: input.request_id,
            name: input.name,
            email: input.email,
            password: Some(input.password),
            role: one_role(input.role, input.roles)?,
            send_welcome_email: input.send_welcome_email,
        }),
    )
    .await
}
pub(super) async fn update_user(
    state: State<AppState>,
    admin: SuperAdminUser,
    verified: Extension<VerifiedAuthority>,
    path: Path<i64>,
    Json(input): Json<UpdateUserRequest>,
) -> Result<Response, ApiError> {
    accounts::update_member(
        state,
        admin,
        verified,
        path,
        Json(accounts::UpdateMemberRequest {
            expected_version: input.expected_version,
            name: input.name,
            email: input.email,
            role: one_role(input.role, input.roles)?,
            is_active: input.is_active,
            password: input.password,
            avatar_url: input.avatar_url,
        }),
    )
    .await
}
pub(super) async fn delete_user(
    state: State<AppState>,
    admin: SuperAdminUser,
    verified: Extension<VerifiedAuthority>,
    path: Path<i64>,
    headers: HeaderMap,
) -> Result<Response, ApiError> {
    accounts::delete_member(state, admin, verified, path, headers).await
}
pub(super) async fn ban_user(
    State(state): State<AppState>,
    SuperAdminUser(actor): SuperAdminUser,
    Extension(verified): Extension<VerifiedAuthority>,
    Path(id): Path<i64>,
    Json(input): Json<accounts::BanMemberRequest>,
) -> Result<Response, ApiError> {
    accounts::restrict(state, actor.id, verified, id, input, "banned", true).await
}
pub(super) async fn unban_user(
    State(state): State<AppState>,
    SuperAdminUser(actor): SuperAdminUser,
    Extension(verified): Extension<VerifiedAuthority>,
    Path(id): Path<i64>,
    Json(input): Json<accounts::UnbanMemberRequest>,
) -> Result<Response, ApiError> {
    accounts::unban(state, actor.id, verified, id, input, true).await
}

/// User stats (admin)
/// ICT 7 SECURITY FIX: Added require_admin check
pub(super) async fn user_stats(
    State(state): State<AppState>,
    _admin: AdminUser,
) -> Result<Response, ApiError> {
    let roles: serde_json::Value = serde_json::from_str(include_str!("../../../role-policy.json"))
        .map_err(|_| {
            ApiError::new(
                StatusCode::SERVICE_UNAVAILABLE,
                "Role policy is unavailable",
            )
        })?;
    let data:serde_json::Value=sqlx::query_scalar("SELECT jsonb_build_object('total',COUNT(*),'active',COUNT(*) FILTER(WHERE is_active=true),'unknown_activation',COUNT(*) FILTER(WHERE is_active IS NULL),'verified',COUNT(*) FILTER(WHERE email_verified_at IS NOT NULL),'admins',COUNT(*) FILTER(WHERE LOWER(role) IN(SELECT jsonb_array_elements_text($1::jsonb->'admin')))) FROM users WHERE deleted_at IS NULL AND COALESCE(LOWER(role),'')<>'deleted'").bind(roles).fetch_one(&state.db.pool).await.map_err(storage)?;
    Ok(private(data))
}
