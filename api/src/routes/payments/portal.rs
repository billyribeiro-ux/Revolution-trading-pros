//! Stripe customer portal handler.
//!
//! R16-B (2026-05-20): extracted from the original 2,368-LOC
//! `payments.rs` as a pure structural move. All SQL, Stripe calls,
//! and error mapping preserved byte-for-byte.

use axum::{Json, extract::State, http::StatusCode};
use serde_json::json;

use super::dtos::{CreatePortalRequest, PortalResponse};
use crate::{AppState, models::User};

/// Get Stripe customer portal URL// Get Stripe customer portal URL
/// POST /api/payments/portal
pub(super) async fn create_portal(
    State(state): State<AppState>,
    user: User,
    Json(input): Json<CreatePortalRequest>,
) -> Result<Json<PortalResponse>, (StatusCode, Json<serde_json::Value>)> {
    let app_url = state.config.app_url.trim_end_matches('/');
    let is_safe_relative = input.return_url.starts_with('/') && !input.return_url.starts_with("//");
    let is_same_origin = input.return_url == app_url
        || input
            .return_url
            .strip_prefix(app_url)
            .is_some_and(|rest| rest.is_empty() || rest.starts_with('/') || rest.starts_with('?'));

    if !is_safe_relative && !is_same_origin {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(
                json!({"error": "Invalid return_url: must be a relative path or match application origin"}),
            ),
        ));
    }

    let resolved_return_url = if is_safe_relative {
        format!("{}{}", app_url, input.return_url)
    } else {
        input.return_url.clone()
    };

    // Get user's Stripe customer ID
    let customer_id: Option<String> = sqlx::query_scalar(
        "SELECT stripe_customer_id FROM user_memberships WHERE user_id = $1 AND stripe_customer_id IS NOT NULL LIMIT 1"
    )
    .bind(user.id)
    .fetch_optional(&state.db.pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()}))))?
    .flatten();

    let customer_id = customer_id.ok_or_else(|| {
        (
            StatusCode::NOT_FOUND,
            Json(json!({"error": "No payment method on file"})),
        )
    })?;

    let url = state
        .services
        .stripe
        .create_portal_session(&customer_id, &resolved_return_url)
        .await
        .map_err(|e| {
            tracing::error!("Stripe portal error: {}", e);
            (
                StatusCode::BAD_GATEWAY,
                Json(json!({"error": "Failed to create portal session"})),
            )
        })?;

    Ok(Json(PortalResponse { url }))
}
