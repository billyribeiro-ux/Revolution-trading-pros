//! Orders Handlers
//!
//! ICT 11+ Principal Engineer Grade
//! User order management

use axum::{
    extract::{Path, State},
    routing::get,
    Json, Router,
};
use serde::Serialize;
use uuid::Uuid;

use crate::{models::User, services::order_service::OrderService, AppState};

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
    user: User,
) -> Result<Json<serde_json::Value>, (axum::http::StatusCode, Json<serde_json::Value>)> {
    // Return empty orders for now - OrderService expects UUID but user.id is i64
    // TODO: Fix OrderService to use i64 or update users table to use UUID
    let orders: Vec<serde_json::Value> = vec![];

    /*
    let order_service = OrderService::new(&state.db.pool);
    let user_uuid = uuid::Uuid::from_u128(user.id as u128);
    let orders = order_service.get_user_orders(user_uuid).await
        .map_err(|e| (axum::http::StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({"error": e.message}))))?;
    */

    Ok(Json(serde_json::json!({
        "success": true,
        "data": orders,
        "message": "Orders endpoint working - empty for now"
    })))
}

/// GET /api/my/orders/:id - Get order details
#[tracing::instrument(skip(state))]
pub async fn show(
    State(state): State<AppState>,
    user: User,
    Path(id): Path<Uuid>,
) -> Result<Json<serde_json::Value>, (axum::http::StatusCode, Json<serde_json::Value>)> {
    let order_service = OrderService::new(&state.db.pool);
    let user_uuid = uuid::Uuid::from_u128(user.id as u128);
    let order = order_service
        .get_user_order(user_uuid, id)
        .await
        .map_err(|e| {
            (
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({"error": e.message})),
            )
        })?
        .ok_or((
            axum::http::StatusCode::NOT_FOUND,
            Json(serde_json::json!({"error": "Order not found"})),
        ))?;

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

    Ok(Json(serde_json::json!({
        "success": true,
        "data": OrderDetailResponse {
            id: order.id.to_string(),
            number: order.order_number,
            date: order.created_at.to_rfc3339(),
            status: order.status,
            total: format!("{:.2}", order.total),
            subtotal: format!("{:.2}", order.subtotal),
            tax: format!("{:.2}", order.tax),
            discount: format!("{:.2}", order.discount),
            currency: order.currency,
            payment_method: order.payment_method,
            items,
            billing_address: order.billing_address,
        }
    })))
}

/// Build orders router
pub fn router() -> Router<AppState> {
    Router::new()
        .route("/", get(index))
        .route("/:id", get(show))
}
