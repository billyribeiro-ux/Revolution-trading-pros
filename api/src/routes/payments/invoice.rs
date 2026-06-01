//! Invoice generation handler.
//!
//! R16-B (2026-05-20): extracted from the original 2,368-LOC
//! `payments.rs` as a pure structural move. Owner/admin gating,
//! integer-cents money path, and JSON response shape preserved
//! byte-for-byte.

use axum::{extract::State, http::StatusCode, Json};
use serde_json::json;

use super::dtos::GenerateInvoiceRequest;
use crate::{models::User, AppState};

/// Generate invoice for an order
/// POST /api/payments/invoice
pub(super) async fn generate_invoice(
    State(state): State<AppState>,
    user: User,
    Json(input): Json<GenerateInvoiceRequest>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    // Verify order belongs to user. All monetary values are integer cents.
    #[derive(sqlx::FromRow)]
    struct OrderInvoiceData {
        id: i64,
        order_number: String,
        user_id: i64,
        status: String,
        subtotal_cents: i64,
        discount_cents: i64,
        tax_cents: i64,
        total_cents: i64,
        currency: String,
        billing_name: Option<String>,
        billing_email: Option<String>,
        billing_address: Option<serde_json::Value>,
        coupon_code: Option<String>,
        created_at: chrono::NaiveDateTime,
        completed_at: Option<chrono::NaiveDateTime>,
    }

    let order: OrderInvoiceData = sqlx::query_as(
        r"SELECT id, order_number, user_id, status,
                  (subtotal * 100)::BIGINT AS subtotal_cents,
                  (discount * 100)::BIGINT AS discount_cents,
                  (tax * 100)::BIGINT      AS tax_cents,
                  (total * 100)::BIGINT    AS total_cents,
                  currency, billing_name, billing_email,
                  billing_address, coupon_code, created_at, completed_at
           FROM orders WHERE id = $1",
    )
    .bind(input.order_id)
    .fetch_optional(&state.db.pool)
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": e.to_string()})),
        )
    })?
    .ok_or_else(|| {
        (
            StatusCode::NOT_FOUND,
            Json(json!({"error": "Order not found"})),
        )
    })?;

    // Check ownership unless admin
    let is_admin =
        user.role.as_deref() == Some("admin") || user.role.as_deref() == Some("super_admin");
    if order.user_id != user.id && !is_admin {
        return Err((
            StatusCode::FORBIDDEN,
            Json(json!({"error": "Access denied"})),
        ));
    }

    // Fetch order items (all monetary values in integer cents)
    #[derive(sqlx::FromRow, serde::Serialize)]
    struct OrderItem {
        name: String,
        quantity: i32,
        unit_price_cents: i64,
        total_cents: i64,
    }

    let items: Vec<OrderItem> = sqlx::query_as(
        "SELECT name, quantity,
                (unit_price * 100)::BIGINT AS unit_price_cents,
                (total      * 100)::BIGINT AS total_cents
         FROM order_items WHERE order_id = $1",
    )
    .bind(order.id)
    .fetch_all(&state.db.pool)
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": e.to_string()})),
        )
    })?;

    // Generate invoice number
    let invoice_number = format!(
        "INV-{}-{}",
        order.order_number,
        chrono::Utc::now().format("%Y%m%d")
    );

    // Build invoice response
    let invoice = json!({
        "invoice_number": invoice_number,
        "order_number": order.order_number,
        "status": order.status,
        "date": order.created_at.and_utc().to_rfc3339(),
        "completed_at": order.completed_at.map(|d| d.and_utc().to_rfc3339()),
        "billing": {
            "name": order.billing_name,
            "email": order.billing_email,
            "address": order.billing_address
        },
        "items": items,
        "subtotal_cents": order.subtotal_cents,
        "discount_cents": order.discount_cents,
        "coupon_code": order.coupon_code,
        "tax_cents": order.tax_cents,
        "total_cents": order.total_cents,
        "currency": order.currency,
        "company": {
            "name": "Revolution Trading Pros",
            "address": "Trading Professional Services",
            "email": "billing@revolutiontradingpros.com"
        }
    });

    tracing::info!(
        target: "payments",
        event = "invoice_generated",
        invoice_number = %invoice_number,
        order_id = %order.id,
        user_id = %user.id,
        "Invoice generated"
    );

    Ok(Json(json!({
        "success": true,
        "invoice": invoice
    })))
}
