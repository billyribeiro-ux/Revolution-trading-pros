//! Orders Handlers
//!
//! ICT 11+ Principal Engineer Grade
//! User order management

use axum::{
    extract::{Path, State},
    Json,
};
use serde::Serialize;
use uuid::Uuid;

use crate::{
    errors::AppError, extractors::AuthUser, responses::ApiResponse, services::OrderService,
    AppState,
};

#[derive(Debug, Serialize)]
pub struct OrderResponse {
    pub id: String,
    pub number: String,
    pub date: String,
    pub status: String,
    pub total: String,
    pub currency: String,
    pub item_count: i32,
}

#[derive(Debug, Serialize)]
pub struct OrderDetailResponse {
    pub id: String,
    pub number: String,
    pub date: String,
    pub status: String,
    pub total: String,
    pub subtotal: String,
    pub tax: String,
    pub discount: String,
    pub currency: String,
    pub payment_method: Option<String>,
    pub items: Vec<OrderItemResponse>,
    pub billing_address: Option<serde_json::Value>,
}

#[derive(Debug, Serialize)]
pub struct OrderItemResponse {
    pub id: String,
    pub name: String,
    pub quantity: i32,
    pub price: String,
    pub total: String,
}

/// GET /api/my/orders - List user's orders
#[tracing::instrument(skip(state))]
pub async fn index(
    State(state): State<AppState>,
    auth_user: AuthUser,
) -> Result<Json<ApiResponse<Vec<OrderResponse>>>, AppError> {
    let order_service = OrderService::new(&state.db);
    let orders = order_service.get_user_orders(auth_user.user_id).await?;

    let response: Vec<OrderResponse> = orders
        .into_iter()
        .map(|o| OrderResponse {
            id: o.id.to_string(),
            number: o.order_number,
            date: o.created_at.format("%Y-%m-%dT%H:%M:%S").to_string(),
            status: o.status,
            total: format!("{:.2}", o.total),
            currency: o.currency,
            item_count: o.item_count,
        })
        .collect();

    Ok(Json(ApiResponse::success(response)))
}

/// GET /api/my/orders/:id - Get order details
#[tracing::instrument(skip(state))]
pub async fn show(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Path(id): Path<Uuid>,
) -> Result<Json<ApiResponse<OrderDetailResponse>>, AppError> {
    let order_service = OrderService::new(&state.db);
    let order = order_service
        .get_user_order(auth_user.user_id, id)
        .await?
        .ok_or(AppError::NotFound("Order not found".to_string()))?;

    let items: Vec<OrderItemResponse> = order
        .items
        .into_iter()
        .map(|i| OrderItemResponse {
            id: i.id.to_string(),
            name: i.name,
            quantity: i.quantity,
            price: format!("{:.2}", i.price),
            total: format!("{:.2}", i.total),
        })
        .collect();

    Ok(Json(ApiResponse::success(OrderDetailResponse {
        id: order.id.to_string(),
        number: order.order_number,
        date: order.created_at.format("%Y-%m-%dT%H:%M:%S").to_string(),
        status: order.status,
        total: format!("{:.2}", order.total),
        subtotal: format!("{:.2}", order.subtotal),
        tax: format!("{:.2}", order.tax),
        discount: format!("{:.2}", order.discount),
        currency: order.currency,
        payment_method: order.payment_method,
        items,
        billing_address: order.billing_address,
    })))
}
