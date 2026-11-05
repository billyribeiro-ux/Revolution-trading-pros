//! Recipient/epoch-bound email ownership, consumed once against the current account.
use super::helpers::{ClientAddr, client_ip, enforce_ip_rate_limit_strict};
use crate::{AppState, services::account_links, utils::errors::ApiError};
use axum::{
    Json,
    extract::{Query, State},
    http::{HeaderMap, StatusCode},
};
use serde::Deserialize;
use serde_json::{Value, json};
use validator::ValidateEmail;
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub(super) struct VerifyEmailQuery {
    token: String,
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub(super) struct ResendVerificationRequest {
    email: String,
}
fn error(error: ApiError) -> (StatusCode, Json<Value>) {
    (
        StatusCode::from_bytes(error.status.as_bytes())
            .unwrap_or(StatusCode::INTERNAL_SERVER_ERROR),
        Json(json!({"error":error.message})),
    )
}
fn storage(error: sqlx::Error) -> (StatusCode, Json<Value>) {
    tracing::error!(%error,"Verification link storage failed");
    (
        StatusCode::SERVICE_UNAVAILABLE,
        Json(json!({"error":"Verification request is unavailable"})),
    )
}
fn requested() -> Json<Value> {
    Json(
        json!({"success":true,"message":"If this email is eligible, verification instructions were requested. Check your mailbox or try again later."}),
    )
}
pub(super) async fn verify_email(
    State(state): State<AppState>,
    Query(query): Query<VerifyEmailQuery>,
) -> Result<Json<Value>, (StatusCode, Json<Value>)> {
    Ok(Json(
        account_links::verify(&state.db.pool, &query.token)
            .await
            .map_err(error)?,
    ))
}
pub(super) async fn resend_verification(
    State(state): State<AppState>,
    peer: ClientAddr,
    headers: HeaderMap,
    Json(input): Json<ResendVerificationRequest>,
) -> Result<Json<Value>, (StatusCode, Json<Value>)> {
    if input.email.len() > 254 || !input.email.validate_email() {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(json!({"error":"Provide a valid bounded email"})),
        ));
    }
    let ip = client_ip(peer.0, &headers, &state.config.trusted_proxy_cidrs);
    enforce_ip_rate_limit_strict(&state, &ip, "resend_verify", 3, 3600).await?;
    let ids:Vec<i64>=sqlx::query_scalar("SELECT id FROM users WHERE LOWER(email)=LOWER($1) AND deleted_at IS NULL AND COALESCE(LOWER(role),'')<>'deleted' AND email_verified_at IS NULL ORDER BY id LIMIT 2").bind(&input.email).fetch_all(&state.db.pool).await.map_err(storage)?;
    if ids.len() != 1 {
        return Ok(requested());
    }
    let id = ids[0];
    let mut tx = state.db.pool.begin().await.map_err(storage)?;
    sqlx::query("LOCK TABLE users IN SHARE ROW EXCLUSIVE MODE")
        .execute(&mut *tx)
        .await
        .map_err(storage)?;
    let eligible:bool=sqlx::query_scalar("SELECT EXISTS(SELECT 1 FROM users WHERE id=$1 AND email_verified_at IS NULL AND deleted_at IS NULL AND COALESCE(LOWER(role),'')<>'deleted')").bind(id).fetch_one(&mut *tx).await.map_err(storage)?;
    if !eligible {
        tx.commit().await.map_err(storage)?;
        return Ok(requested());
    }
    let (raw, email, name) = account_links::verification_on(&mut tx, id)
        .await
        .map_err(error)?;
    tx.commit().await.map_err(storage)?;
    let link = format!(
        "{}/verify-email?token={raw}",
        state.config.app_url.trim_end_matches('/')
    );
    if state
        .services
        .email
        .send_transactional_linked(
            &state.db.pool,
            id,
            &email,
            "email-verification",
            json!({"name":name,"verification_link":link,"expires_in_hours":24}),
        )
        .await
        .is_err()
    {
        tracing::warn!(
            user_id = id,
            "Verification submission was not accepted; its linked receipt retains the actual outcome"
        )
    }
    Ok(requested())
}
