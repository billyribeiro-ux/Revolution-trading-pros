//! Email Webhook Handlers (Postmark, etc.)
use crate::{errors::AppError, AppState};
use axum::{extract::State, Json};

pub async fn postmark(
    State(_state): State<AppState>,
    Json(_payload): Json<serde_json::Value>,
) -> Result<&'static str, AppError> {
    // TODO: Process Postmark inbound email webhook
    Ok("Webhook received")
}
