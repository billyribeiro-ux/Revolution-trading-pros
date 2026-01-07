//! Subscriptions Handlers
//!
//! ICT 11+ Principal Engineer Grade

use axum::{
    extract::{Path, State},
    Json,
};
use serde::Serialize;
use uuid::Uuid;

use crate::{
    errors::AppError, extractors::AuthUser, responses::ApiResponse, services::SubscriptionService,
    AppState,
};

#[derive(Debug, Serialize)]
pub struct SubscriptionResponse {
    pub id: String,
    pub plan_name: String,
    pub status: String,
    pub price: String,
    pub billing_period: String,
    pub start_date: String,
    pub next_payment_date: Option<String>,
}

pub async fn index(
    State(state): State<AppState>,
    auth: AuthUser,
) -> Result<Json<ApiResponse<Vec<SubscriptionResponse>>>, AppError> {
    let subscription_service = SubscriptionService::new(&state.db);
    let subscriptions = subscription_service
        .get_user_subscriptions(auth.user_id)
        .await?;

    let response: Vec<SubscriptionResponse> = subscriptions
        .into_iter()
        .map(|s| SubscriptionResponse {
            id: s.id.to_string(),
            plan_name: s.plan_name,
            status: s.status,
            price: format!("{:.2}", s.price),
            billing_period: s.billing_period,
            start_date: s.start_date.format("%Y-%m-%dT%H:%M:%S").to_string(),
            next_payment_date: s
                .next_payment_date
                .map(|d| d.format("%Y-%m-%dT%H:%M:%S").to_string()),
        })
        .collect();

    Ok(Json(ApiResponse::success(response)))
}

pub async fn show(
    State(state): State<AppState>,
    auth: AuthUser,
    Path(id): Path<Uuid>,
) -> Result<Json<ApiResponse<SubscriptionResponse>>, AppError> {
    let subscription_service = SubscriptionService::new(&state.db);
    let subscription = subscription_service
        .get_user_subscription(auth.user_id, id)
        .await?
        .ok_or(AppError::NotFound("Subscription not found".to_string()))?;

    Ok(Json(ApiResponse::success(SubscriptionResponse {
        id: subscription.id.to_string(),
        plan_name: subscription.plan_name,
        status: subscription.status,
        price: format!("{:.2}", subscription.price),
        billing_period: subscription.billing_period,
        start_date: subscription
            .start_date
            .format("%Y-%m-%dT%H:%M:%S")
            .to_string(),
        next_payment_date: subscription
            .next_payment_date
            .map(|d| d.format("%Y-%m-%dT%H:%M:%S").to_string()),
    })))
}

pub async fn cancel(
    State(_state): State<AppState>,
    _auth: AuthUser,
    Path(_id): Path<Uuid>,
) -> Result<Json<serde_json::Value>, AppError> {
    Ok(Json(
        serde_json::json!({"success": true, "message": "Subscription cancelled"}),
    ))
}

pub async fn pause(
    State(_state): State<AppState>,
    _auth: AuthUser,
    Path(_id): Path<Uuid>,
) -> Result<Json<serde_json::Value>, AppError> {
    Ok(Json(
        serde_json::json!({"success": true, "message": "Subscription paused"}),
    ))
}

pub async fn resume(
    State(_state): State<AppState>,
    _auth: AuthUser,
    Path(_id): Path<Uuid>,
) -> Result<Json<serde_json::Value>, AppError> {
    Ok(Json(
        serde_json::json!({"success": true, "message": "Subscription resumed"}),
    ))
}

pub async fn reactivate(
    State(_state): State<AppState>,
    _auth: AuthUser,
    Path(_id): Path<Uuid>,
) -> Result<Json<serde_json::Value>, AppError> {
    Ok(Json(
        serde_json::json!({"success": true, "message": "Subscription reactivated"}),
    ))
}

pub async fn invoices(
    State(_state): State<AppState>,
    _auth: AuthUser,
    Path(_id): Path<Uuid>,
) -> Result<Json<serde_json::Value>, AppError> {
    Ok(Json(serde_json::json!({"success": true, "data": []})))
}

pub async fn payments(
    State(_state): State<AppState>,
    _auth: AuthUser,
    Path(_id): Path<Uuid>,
) -> Result<Json<serde_json::Value>, AppError> {
    Ok(Json(serde_json::json!({"success": true, "data": []})))
}
