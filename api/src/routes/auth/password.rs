//! Password capability requests and one-use resets, bound to actual user identity and epoch.
use super::helpers::{ClientAddr, client_ip, enforce_ip_rate_limit_strict};
use crate::{
    AppState,
    middleware::validated_json::ValidatedJson,
    models::{ForgotPasswordRequest, ResetPasswordRequest},
    services::account_links,
    utils::{self, errors::ApiError},
};
use axum::{
    Json,
    extract::State,
    http::{HeaderMap, StatusCode},
};
use serde_json::{Value, json};
fn storage(error: sqlx::Error) -> (StatusCode, Json<Value>) {
    tracing::error!(%error,"Password capability storage failed");
    (
        StatusCode::SERVICE_UNAVAILABLE,
        Json(json!({"error":"Password request is unavailable"})),
    )
}
fn error(error: ApiError) -> (StatusCode, Json<Value>) {
    (
        StatusCode::from_bytes(error.status.as_bytes())
            .unwrap_or(StatusCode::INTERNAL_SERVER_ERROR),
        Json(json!({"error":error.message})),
    )
}
fn requested() -> Json<Value> {
    Json(
        json!({"success":true,"message":"If this email is eligible, reset instructions were requested. Check your mailbox or try again later."}),
    )
}
pub(super) async fn forgot_password(
    State(state): State<AppState>,
    peer: ClientAddr,
    headers: HeaderMap,
    ValidatedJson(input): ValidatedJson<ForgotPasswordRequest>,
) -> Result<Json<Value>, (StatusCode, Json<Value>)> {
    let ip = client_ip(peer.0, &headers, &state.config.trusted_proxy_cidrs);
    enforce_ip_rate_limit_strict(&state, &ip, "forgot_password", 3, 3600).await?;
    let ids:Vec<i64>=sqlx::query_scalar("SELECT id FROM users WHERE LOWER(email)=LOWER($1) AND deleted_at IS NULL AND COALESCE(LOWER(role),'')<>'deleted' ORDER BY id LIMIT 2").bind(&input.email).fetch_all(&state.db.pool).await.map_err(storage)?;
    if ids.len() != 1 {
        return Ok(requested());
    }
    let id = ids[0];
    let mut tx = state.db.pool.begin().await.map_err(storage)?;
    sqlx::query("LOCK TABLE users IN SHARE ROW EXCLUSIVE MODE")
        .execute(&mut *tx)
        .await
        .map_err(storage)?;
    let (raw, email, name) = account_links::password_on(&mut tx, id)
        .await
        .map_err(error)?;
    tx.commit().await.map_err(storage)?;
    let link = format!(
        "{}/reset-password?token={raw}",
        state.config.app_url.trim_end_matches('/')
    );
    if state
        .services
        .email
        .send_transactional_linked(
            &state.db.pool,
            id,
            &email,
            "password-reset",
            json!({"name":name,"reset_link":link,"expires_in_minutes":60}),
        )
        .await
        .is_err()
    {
        tracing::warn!(
            user_id = id,
            "Reset mail was not accepted; its linked receipt retains the actual outcome"
        )
    }
    Ok(requested())
}
pub(super) async fn reset_password(
    State(state): State<AppState>,
    peer: ClientAddr,
    headers: HeaderMap,
    ValidatedJson(input): ValidatedJson<ResetPasswordRequest>,
) -> Result<Json<Value>, (StatusCode, Json<Value>)> {
    let ip = client_ip(peer.0, &headers, &state.config.trusted_proxy_cidrs);
    enforce_ip_rate_limit_strict(&state, &ip, "reset_password", 5, 3600).await?;
    if input.password != input.password_confirmation {
        return Err((
            StatusCode::UNPROCESSABLE_ENTITY,
            Json(
                json!({"message":"Passwords do not match","errors":{"password_confirmation":["Passwords do not match"]}}),
            ),
        ));
    }
    utils::validate_password(&input.password).map_err(|message| {
        (
            StatusCode::UNPROCESSABLE_ENTITY,
            Json(json!({"message":message,"errors":{"password":[message]}})),
        )
    })?;
    // Avoid expensive hashing without an actual current capability; consume rechecks under lock.
    let valid:bool=sqlx::query_scalar("SELECT EXISTS(SELECT 1 FROM password_resets r JOIN users u ON u.id=r.user_id WHERE r.token=$1 AND LOWER(r.email)=LOWER($2) AND r.email=u.email AND r.issued_auth_version=u.auth_version AND r.expires_at>NOW() AT TIME ZONE 'UTC' AND u.deleted_at IS NULL AND COALESCE(LOWER(u.role),'')<>'deleted')").bind(utils::hash_token(&input.token)).bind(&input.email).fetch_one(&state.db.pool).await.map_err(storage)?;
    if !valid {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(json!({"error":"This link is invalid or expired; request a new link"})),
        ));
    }
    let hash = utils::hash_password_async(&input.password)
        .await
        .map_err(|_| {
            (
                StatusCode::SERVICE_UNAVAILABLE,
                Json(json!({"error":"Password hashing is unavailable"})),
            )
        })?;
    Ok(Json(
        account_links::reset(&state.db.pool, &input.token, &input.email, hash)
            .await
            .map_err(error)?,
    ))
}
