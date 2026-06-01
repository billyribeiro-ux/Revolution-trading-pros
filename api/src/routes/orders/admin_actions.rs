//! Admin write-side actions: status update / refund / cancel / fulfill /
//! resend confirmation. Mounted under `/admin/orders/:id/...`.
//!
//! R23-B (2026-05-20): extracted from the original 1,382-LOC
//! `orders.rs` as a pure structural move. Every SQL statement, Stripe
//! call site, error mapping, audit-log emission, and money invariant
//! (`i64` cents end-to-end) is preserved byte-for-byte.

use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};

use super::dtos::{RefundOrderRequest, UpdateOrderStatusRequest};
use super::helpers::{grant_order_products, is_valid_transition};
use crate::{middleware::admin::AdminUser, AppState};

/// POST /api/admin/orders/:id/status - Update order status
/// ICT 7 FIX: Complete order status management with validation
#[tracing::instrument(skip(state, admin))]
pub async fn admin_update_status(
    State(state): State<AppState>,
    AdminUser(admin): AdminUser,
    Path(id): Path<i64>,
    Json(input): Json<UpdateOrderStatusRequest>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    // Validate status value
    let valid_statuses = [
        "pending",
        "processing",
        "completed",
        "cancelled",
        "failed",
        "refunded",
        "partial_refund",
    ];
    if !valid_statuses.contains(&input.status.as_str()) {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(
                serde_json::json!({"error": format!("Invalid status. Must be one of: {}", valid_statuses.join(", "))}),
            ),
        ));
    }

    // Get current order status
    let current_status: Option<String> = sqlx::query_scalar(
        "SELECT status FROM orders WHERE id = $1"
    )
    .bind(id)
    .fetch_optional(&state.db.pool)
    .await
    .map_err(|e| {
        tracing::error!(target: "orders", error = %e, order_id = %id, "Failed to fetch order status");
        (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({"error": "Database error"})))
    })?;

    let current_status = current_status.ok_or_else(|| {
        (
            StatusCode::NOT_FOUND,
            Json(serde_json::json!({"error": "Order not found"})),
        )
    })?;

    // Validate transition
    if !is_valid_transition(&current_status, &input.status) {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({
                "error": format!("Invalid status transition from '{}' to '{}'", current_status, input.status)
            })),
        ));
    }

    // Update status
    let completed_at = if input.status == "completed" {
        Some(chrono::Utc::now().naive_utc())
    } else {
        None
    };

    let refunded_at = if input.status == "refunded" || input.status == "partial_refund" {
        Some(chrono::Utc::now().naive_utc())
    } else {
        None
    };

    sqlx::query(
        r"UPDATE orders SET
            status = $2,
            completed_at = COALESCE($3, completed_at),
            refunded_at = COALESCE($4, refunded_at),
            updated_at = NOW()
        WHERE id = $1"
    )
    .bind(id)
    .bind(&input.status)
    .bind(completed_at)
    .bind(refunded_at)
    .execute(&state.db.pool)
    .await
    .map_err(|e| {
        tracing::error!(target: "orders", error = %e, order_id = %id, "Failed to update order status");
        (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({"error": "Failed to update order status"})))
    })?;

    // Log status change
    tracing::info!(
        target: "orders",
        event = "order_status_updated",
        order_id = %id,
        from_status = %current_status,
        to_status = %input.status,
        admin_id = %admin.id,
        notes = ?input.notes,
        "Order status updated"
    );

    // If completed, grant product access
    if input.status == "completed" {
        grant_order_products(&state, id).await.ok();
    }

    Ok(Json(serde_json::json!({
        "success": true,
        "message": "Order status updated",
        "order_id": id,
        "previous_status": current_status,
        "new_status": input.status
    })))
}

/// POST /api/admin/orders/:id/refund - Refund an order
/// ICT 7 FIX: Complete refund functionality
#[tracing::instrument(skip(state, admin))]
pub async fn admin_refund(
    State(state): State<AppState>,
    AdminUser(admin): AdminUser,
    Path(id): Path<i64>,
    Json(input): Json<RefundOrderRequest>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    // Get order details (total in integer cents)
    #[derive(sqlx::FromRow)]
    struct OrderInfo {
        status: String,
        total_cents: i64,
        payment_intent_id: Option<String>,
    }

    let order: OrderInfo = sqlx::query_as(
        "SELECT status, (total * 100)::BIGINT AS total_cents, payment_intent_id FROM orders WHERE id = $1",
    )
    .bind(id)
    .fetch_optional(&state.db.pool)
    .await
    .map_err(|e| {
        tracing::error!(target: "orders", error = %e, order_id = %id, "Failed to fetch order");
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({"error": "Database error"})),
        )
    })?
    .ok_or_else(|| {
        (
            StatusCode::NOT_FOUND,
            Json(serde_json::json!({"error": "Order not found"})),
        )
    })?;

    // Validate order can be refunded
    if order.status != "completed"
        && order.status != "processing"
        && order.status != "partial_refund"
    {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(
                serde_json::json!({"error": format!("Cannot refund order with status '{}'", order.status)}),
            ),
        ));
    }

    let refund_amount_cents: i64 = input.amount_cents.unwrap_or(order.total_cents);

    // Validate refund amount
    if refund_amount_cents <= 0 || refund_amount_cents > order.total_cents {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({"error": "Invalid refund amount"})),
        ));
    }

    let new_status = if refund_amount_cents == order.total_cents {
        "refunded"
    } else {
        "partial_refund"
    };

    // If we have a payment intent, attempt Stripe refund
    if let Some(ref payment_intent_id) = order.payment_intent_id {
        // Attempt Stripe refund (amount already in cents)
        let refund_result = state
            .services
            .stripe
            .create_refund(
                payment_intent_id,
                Some(refund_amount_cents),
                input.reason.as_deref(),
            )
            .await;

        if let Err(e) = refund_result {
            tracing::error!(
                target: "orders",
                error = %e,
                order_id = %id,
                payment_intent_id = %payment_intent_id,
                "Stripe refund failed"
            );
            return Err((
                StatusCode::BAD_GATEWAY,
                Json(serde_json::json!({"error": format!("Stripe refund failed: {}", e)})),
            ));
        }
    }

    // Update order status
    sqlx::query(
        r"UPDATE orders SET
            status = $2,
            refunded_at = NOW(),
            updated_at = NOW()
        WHERE id = $1"
    )
    .bind(id)
    .bind(new_status)
    .execute(&state.db.pool)
    .await
    .map_err(|e| {
        tracing::error!(target: "orders", error = %e, order_id = %id, "Failed to update order status");
        (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({"error": "Failed to update order"})))
    })?;

    tracing::info!(
        target: "orders",
        event = "order_refunded",
        order_id = %id,
        refund_amount_cents = %refund_amount_cents,
        new_status = %new_status,
        admin_id = %admin.id,
        reason = ?input.reason,
        "Order refunded"
    );

    Ok(Json(serde_json::json!({
        "success": true,
        "message": "Order refunded successfully",
        "order_id": id,
        "refund_amount_cents": refund_amount_cents,
        "new_status": new_status
    })))
}

/// POST /api/admin/orders/:id/cancel - Cancel an order
/// ICT 7 FIX: Complete cancel functionality
#[tracing::instrument(skip(state, admin))]
pub async fn admin_cancel(
    State(state): State<AppState>,
    AdminUser(admin): AdminUser,
    Path(id): Path<i64>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    // Get current status
    let current_status: Option<String> = sqlx::query_scalar(
        "SELECT status FROM orders WHERE id = $1",
    )
    .bind(id)
    .fetch_optional(&state.db.pool)
    .await
    .map_err(|e| {
        tracing::error!(target: "orders", error = %e, order_id = %id, "Failed to fetch order");
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({"error": "Database error"})),
        )
    })?;

    let current_status = current_status.ok_or_else(|| {
        (
            StatusCode::NOT_FOUND,
            Json(serde_json::json!({"error": "Order not found"})),
        )
    })?;

    // Only pending or processing orders can be cancelled
    if current_status != "pending" && current_status != "processing" {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(
                serde_json::json!({"error": format!("Cannot cancel order with status '{}'", current_status)}),
            ),
        ));
    }

    // Update order status
    sqlx::query("UPDATE orders SET status = 'cancelled', updated_at = NOW() WHERE id = $1")
        .bind(id)
        .execute(&state.db.pool)
        .await
        .map_err(|e| {
            tracing::error!(target: "orders", error = %e, order_id = %id, "Failed to cancel order");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({"error": "Failed to cancel order"})),
            )
        })?;

    tracing::info!(
        target: "orders",
        event = "order_cancelled",
        order_id = %id,
        previous_status = %current_status,
        admin_id = %admin.id,
        "Order cancelled"
    );

    Ok(Json(serde_json::json!({
        "success": true,
        "message": "Order cancelled successfully",
        "order_id": id,
        "previous_status": current_status
    })))
}

/// POST /api/admin/orders/:id/fulfill - Mark order as completed/fulfilled
/// ICT 7 FIX: Quick fulfill endpoint
#[tracing::instrument(skip(state, admin))]
pub async fn admin_fulfill(
    State(state): State<AppState>,
    AdminUser(admin): AdminUser,
    Path(id): Path<i64>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    // Get current status
    let current_status: Option<String> = sqlx::query_scalar(
        "SELECT status FROM orders WHERE id = $1",
    )
    .bind(id)
    .fetch_optional(&state.db.pool)
    .await
    .map_err(|e| {
        tracing::error!(target: "orders", error = %e, order_id = %id, "Failed to fetch order");
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({"error": "Database error"})),
        )
    })?;

    let current_status = current_status.ok_or_else(|| {
        (
            StatusCode::NOT_FOUND,
            Json(serde_json::json!({"error": "Order not found"})),
        )
    })?;

    // Only pending or processing orders can be fulfilled
    if current_status != "pending" && current_status != "processing" {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(
                serde_json::json!({"error": format!("Cannot fulfill order with status '{}'", current_status)}),
            ),
        ));
    }

    // Update order status
    sqlx::query(
        "UPDATE orders SET status = 'completed', completed_at = NOW(), updated_at = NOW() WHERE id = $1"
    )
    .bind(id)
    .execute(&state.db.pool)
    .await
    .map_err(|e| {
        tracing::error!(target: "orders", error = %e, order_id = %id, "Failed to fulfill order");
        (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({"error": "Failed to fulfill order"})))
    })?;

    // Grant product access
    grant_order_products(&state, id).await.ok();

    tracing::info!(
        target: "orders",
        event = "order_fulfilled",
        order_id = %id,
        previous_status = %current_status,
        admin_id = %admin.id,
        "Order fulfilled"
    );

    Ok(Json(serde_json::json!({
        "success": true,
        "message": "Order fulfilled successfully",
        "order_id": id,
        "previous_status": current_status
    })))
}

/// POST /api/admin/orders/:id/resend-confirmation - Resend order confirmation email
/// ICT 7 FIX: Resend confirmation endpoint
#[tracing::instrument(skip(state, admin))]
pub async fn admin_resend_confirmation(
    State(state): State<AppState>,
    AdminUser(admin): AdminUser,
    Path(id): Path<i64>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    // Get order and user details
    #[derive(sqlx::FromRow)]
    struct OrderEmailInfo {
        order_number: String,
        billing_email: Option<String>,
        user_id: i64,
    }

    let order: OrderEmailInfo = sqlx::query_as(
        "SELECT order_number, billing_email, user_id FROM orders WHERE id = $1",
    )
    .bind(id)
    .fetch_optional(&state.db.pool)
    .await
    .map_err(|e| {
        tracing::error!(target: "orders", error = %e, order_id = %id, "Failed to fetch order");
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({"error": "Database error"})),
        )
    })?
    .ok_or_else(|| {
        (
            StatusCode::NOT_FOUND,
            Json(serde_json::json!({"error": "Order not found"})),
        )
    })?;

    // Get user email if billing email not set
    let email = if let Some(email) = order.billing_email {
        email
    } else {
        sqlx::query_scalar::<_, String>("SELECT email FROM users WHERE id = $1")
            .bind(order.user_id)
            .fetch_one(&state.db.pool)
            .await
            .map_err(|e| {
                tracing::error!(target: "orders", error = %e, user_id = %order.user_id, "Failed to fetch user email");
                (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({"error": "Failed to get user email"})))
            })?
    };

    // Queue email (implementation depends on email service)
    tracing::info!(
        target: "orders",
        event = "confirmation_resent",
        order_id = %id,
        order_number = %order.order_number,
        email = %email,
        admin_id = %admin.id,
        "Order confirmation email queued for resend"
    );

    Ok(Json(serde_json::json!({
        "success": true,
        "message": "Order confirmation email queued",
        "order_id": id,
        "email": email
    })))
}
