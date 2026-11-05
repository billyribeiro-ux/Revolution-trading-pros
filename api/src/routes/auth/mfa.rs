//! Existing advertised self-service MFA endpoints, with explicit token replacement.
use super::helpers::{ClientAddr, client_ip};
use crate::{
    AppState,
    models::User,
    services::{account_authority::VerifiedAuthority, account_mfa},
    utils::errors::ApiError,
};
use axum::{
    Extension, Json,
    extract::State,
    http::{HeaderMap, header},
    response::{IntoResponse, Response},
};
use serde::Deserialize;
use std::net::IpAddr;
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub(super) struct Password {
    password: String,
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub(super) struct Code {
    code: String,
}
fn observed_ip(
    state: &AppState,
    peer: ClientAddr,
    headers: &HeaderMap,
) -> Result<IpAddr, ApiError> {
    if peer.0.is_none() {
        return Err(ApiError::new(
            axum::http::StatusCode::SERVICE_UNAVAILABLE,
            "The client address is unavailable for this security operation",
        ));
    }
    client_ip(peer.0, headers, &state.config.trusted_proxy_cidrs)
        .parse()
        .map_err(|_| ApiError::internal_error("Client address is invalid"))
}
fn response(value: serde_json::Value) -> Response {
    (
        [
            (header::CACHE_CONTROL, "private, no-store"),
            (header::VARY, "Authorization, Cookie"),
        ],
        Json(value),
    )
        .into_response()
}
pub(super) async fn setup(
    State(state): State<AppState>,
    _user: User,
    Extension(verified): Extension<VerifiedAuthority>,
    peer: ClientAddr,
    headers: HeaderMap,
    Json(input): Json<Password>,
) -> Result<Response, ApiError> {
    let ip = observed_ip(&state, peer, &headers)?;
    Ok(response(
        account_mfa::setup(&state, verified, &input.password, Some(ip)).await?,
    ))
}
pub(super) async fn verify(
    State(state): State<AppState>,
    _user: User,
    Extension(verified): Extension<VerifiedAuthority>,
    peer: ClientAddr,
    headers: HeaderMap,
    Json(input): Json<Code>,
) -> Result<Response, ApiError> {
    let ip = observed_ip(&state, peer, &headers)?;
    Ok(response(
        account_mfa::enable(&state, verified, &input.code, Some(ip)).await?,
    ))
}
pub(super) async fn disable(
    State(state): State<AppState>,
    _user: User,
    Extension(verified): Extension<VerifiedAuthority>,
    peer: ClientAddr,
    headers: HeaderMap,
    Json(input): Json<Password>,
) -> Result<Response, ApiError> {
    let ip = observed_ip(&state, peer, &headers)?;
    Ok(response(
        account_mfa::disable(&state, verified, &input.password, Some(ip)).await?,
    ))
}
