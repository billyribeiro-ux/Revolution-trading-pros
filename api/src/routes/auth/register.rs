//! Create one ordinary account and one current recipient-bound verification capability.
use super::helpers::{ClientAddr, client_ip, enforce_ip_rate_limit_strict};
use crate::{
    AppState,
    services::{account_links, crm_logs},
    utils,
};
use axum::{
    Json,
    extract::State,
    http::{HeaderMap, StatusCode},
};
use serde::Deserialize;
use serde_json::{Value, json};
use validator::ValidateEmail;
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub(super) struct RegisterInput {
    name: String,
    email: String,
    password: String,
    password_confirmation: Option<String>,
    terms_accepted: Option<bool>,
    marketing_consent: Option<bool>,
}
fn storage(error: sqlx::Error) -> (StatusCode, Json<Value>) {
    if error
        .as_database_error()
        .is_some_and(|e| e.is_unique_violation())
    {
        return (
            StatusCode::UNPROCESSABLE_ENTITY,
            Json(
                json!({"message":"Validation failed","errors":{"email":["Email already registered"]}}),
            ),
        );
    }
    tracing::error!(%error,"Registration persistence failed");
    (
        StatusCode::SERVICE_UNAVAILABLE,
        Json(json!({"error":"Registration is unavailable"})),
    )
}
fn invalid(message: &str) -> (StatusCode, Json<Value>) {
    (StatusCode::BAD_REQUEST, Json(json!({"error":message})))
}
pub(super) async fn register(
    State(state): State<AppState>,
    peer: ClientAddr,
    headers: HeaderMap,
    Json(input): Json<RegisterInput>,
) -> Result<Json<Value>, (StatusCode, Json<Value>)> {
    let ip = client_ip(peer.0, &headers, &state.config.trusted_proxy_cidrs);
    enforce_ip_rate_limit_strict(&state, &ip, "register", 5, 900).await?;
    let name = input.name.trim();
    let email = input.email.trim().to_lowercase();
    if name.is_empty()
        || name.chars().count() > 200
        || name.chars().any(char::is_control)
        || email.len() > 254
        || !email.validate_email()
    {
        return Err(invalid("Provide a valid bounded name and email"));
    }
    if input
        .password_confirmation
        .as_ref()
        .is_some_and(|confirm| confirm != &input.password)
    {
        return Err(invalid("Passwords do not match"));
    }
    utils::validate_password(&input.password).map_err(invalid)?;
    let exists: bool =
        sqlx::query_scalar("SELECT EXISTS(SELECT 1 FROM users WHERE LOWER(email)=$1)")
            .bind(&email)
            .fetch_one(&state.db.pool)
            .await
            .map_err(storage)?;
    if exists {
        return Err((
            StatusCode::UNPROCESSABLE_ENTITY,
            Json(
                json!({"message":"Validation failed","errors":{"email":["Email already registered"]}}),
            ),
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
    let mut tx = state.db.pool.begin().await.map_err(storage)?;
    let metadata = json!({"registration_preferences":{"terms_accepted":input.terms_accepted,"marketing_consent_requested":input.marketing_consent}});
    let id:i64=sqlx::query_scalar("INSERT INTO users(name,email,password_hash,role,is_active,email_verified_at,metadata,created_at,updated_at) VALUES($1,$2,$3,'user',true,NULL,$4,NOW() AT TIME ZONE 'UTC',NOW() AT TIME ZONE 'UTC') RETURNING id").bind(name).bind(&email).bind(hash).bind(metadata).fetch_one(&mut *tx).await.map_err(storage)?;
    let (raw, recipient, actual_name) =
        account_links::verification_on(&mut tx, id)
            .await
            .map_err(|error| {
                (
                    StatusCode::from_bytes(error.status.as_bytes())
                        .unwrap_or(StatusCode::INTERNAL_SERVER_ERROR),
                    Json(json!({"error":error.message})),
                )
            })?;
    let context = json!({"operation":"account_registered","member_id":id,"actor_id":null,"source":"public_registration","marketing_consent_requested":input.marketing_consent,"description":"Account registered; email ownership is pending"});
    sqlx::query("INSERT INTO user_activity_log(user_id,action,entity_type,entity_id,details,created_at) VALUES($1,'account_registered','member_account',$1,$2,NOW())").bind(id).bind(&context).execute(&mut *tx).await.map_err(storage)?;
    crm_logs::record(
        &mut tx,
        "info",
        "system",
        "Account registered",
        context,
        None,
    )
    .await
    .map_err(storage)?;
    tx.commit().await.map_err(storage)?;
    let welcome = state
        .services
        .email
        .send_transactional_linked(
            &state.db.pool,
            id,
            &recipient,
            "welcome",
            json!({"name":actual_name,"app_url":state.config.app_url}),
        )
        .await;
    if welcome.is_err() {
        tracing::warn!(
            user_id = id,
            "Registration welcome was not accepted; linked receipt retains its actual state"
        )
    }
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
            &recipient,
            "email-verification",
            json!({"name":actual_name,"verification_link":link,"expires_in_hours":24}),
        )
        .await
        .is_err()
    {
        tracing::warn!(
            user_id = id,
            "Registration verification was not accepted; linked receipt retains its actual state"
        )
    }
    Ok(Json(
        json!({"message":"Registration created. Email verification is required; check your mailbox or request a new verification link.","email":recipient,"requires_verification":true}),
    ))
}
