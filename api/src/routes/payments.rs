//! Payment routes

use axum::{
    extract::State,
    http::StatusCode,
    routing::post,
    Json, Router,
};
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::AppState;

#[derive(Deserialize)]
struct CreateCheckoutRequest {
    price_id: String,
    success_url: String,
    cancel_url: String,
    email: String,
}

#[derive(Serialize)]
struct CheckoutResponse {
    url: String,
}

/// Create a Stripe checkout session
async fn create_checkout(
    State(state): State<AppState>,
    Json(input): Json<CreateCheckoutRequest>,
) -> Result<Json<CheckoutResponse>, (StatusCode, Json<serde_json::Value>)> {
    let url = state
        .services
        .stripe
        .create_checkout_session(
            &input.email,
            &input.price_id,
            &input.success_url,
            &input.cancel_url,
        )
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": e.to_string()})),
            )
        })?;

    Ok(Json(CheckoutResponse { url }))
}

#[derive(Deserialize)]
struct CreatePortalRequest {
    customer_id: String,
    return_url: String,
}

/// Create a Stripe customer portal session
async fn create_portal(
    State(state): State<AppState>,
    Json(input): Json<CreatePortalRequest>,
) -> Result<Json<CheckoutResponse>, (StatusCode, Json<serde_json::Value>)> {
    let url = state
        .services
        .stripe
        .create_portal_session(&input.customer_id, &input.return_url)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": e.to_string()})),
            )
        })?;

    Ok(Json(CheckoutResponse { url }))
}

/// Handle Stripe webhooks
async fn webhook(
    State(_state): State<AppState>,
    body: String,
) -> Result<StatusCode, (StatusCode, Json<serde_json::Value>)> {
    // Parse and handle webhook event
    let event: serde_json::Value = serde_json::from_str(&body).map_err(|e| {
        (
            StatusCode::BAD_REQUEST,
            Json(json!({"error": e.to_string()})),
        )
    })?;

    let event_type = event["type"].as_str().unwrap_or("");

    match event_type {
        "checkout.session.completed" => {
            tracing::info!("Checkout completed: {:?}", event["data"]["object"]);
            // Update user subscription
        }
        "customer.subscription.updated" => {
            tracing::info!("Subscription updated: {:?}", event["data"]["object"]);
        }
        "customer.subscription.deleted" => {
            tracing::info!("Subscription deleted: {:?}", event["data"]["object"]);
        }
        "invoice.paid" => {
            tracing::info!("Invoice paid: {:?}", event["data"]["object"]);
        }
        "invoice.payment_failed" => {
            tracing::info!("Payment failed: {:?}", event["data"]["object"]);
        }
        _ => {
            tracing::debug!("Unhandled event type: {}", event_type);
        }
    }

    Ok(StatusCode::OK)
}

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/checkout", post(create_checkout))
        .route("/portal", post(create_portal))
        .route("/webhook", post(webhook))
}
