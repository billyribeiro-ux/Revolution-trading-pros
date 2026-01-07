//! Stripe Webhook Handler
use crate::{errors::AppError, AppState};
use axum::{body::Bytes, extract::State, http::HeaderMap};

pub async fn handle(
    State(_state): State<AppState>,
    _headers: HeaderMap,
    _body: Bytes,
) -> Result<&'static str, AppError> {
    // TODO: Verify Stripe signature and process webhook events
    Ok("Webhook received")
}
