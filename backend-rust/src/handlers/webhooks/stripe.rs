//! Stripe Webhook Handler
use axum::{extract::State, http::HeaderMap, body::Bytes};
use crate::{errors::AppError, AppState};

pub async fn handle(
    State(_state): State<AppState>,
    _headers: HeaderMap,
    _body: Bytes,
) -> Result<&'static str, AppError> {
    // TODO: Verify Stripe signature and process webhook events
    Ok("Webhook received")
}
