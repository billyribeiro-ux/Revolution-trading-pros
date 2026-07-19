//! Orders Handlers — Revolution Trading Pros
//! Apple ICT 11+ Principal Engineer Grade - January 2026
//!
//! R23-B (2026-05-20): the original 1,382-LOC `orders.rs` was split
//! into this `orders/` directory as a PURE STRUCTURAL MOVE. Every
//! handler, SQL statement, error mapping, Stripe call site, tracing
//! field, status-transition table, and money invariant (`i64` cents
//! end-to-end) is preserved byte-for-byte.
//!
//! Complete user order management with:
//! - Order listing with pagination
//! - Order details by ID or order number
//! - Order item details with product metadata
//! - Support for thank-you page order fetching
//!
//! Admin order management with:
//! - Admin listing + stats
//! - Status transitions, refund (Stripe), cancel, fulfill, resend
//!   confirmation
//!
//! Public API contract:
//!  - `pub fn router()` — mounted at `/my/orders`.
//!  - `pub fn admin_router()` — mounted at `/admin/orders`.
//!  - Admin handlers re-exported below (consumed by
//!    `routes/admin_orders.rs` which references them as
//!    `routes::orders::{admin_index, admin_show, admin_update_status,
//!    admin_refund, admin_cancel, admin_fulfill,
//!    admin_resend_confirmation}`).
//!  - DTOs re-exported below (consumed by `tests/orders_test.rs`):
//!    `OrderDetailResponse`, `OrderItemDetailResponse`,
//!    `AdminOrderResponse`, `AdminOrderStats`, `RefundOrderRequest`.

use axum::{
    routing::{get, post},
    Router,
};

use crate::AppState;

mod admin_actions;
mod admin_list;
mod dtos;
mod helpers;
mod user;

// Re-export the DTOs the integration tests + sibling admin module
// import directly. Keeping `routes::orders::{...}` resolvable
// preserves the module's public API across the R23-B split.
pub use dtos::{
    AdminOrderListQuery, AdminOrderResponse, AdminOrderStats, OrderDetailResponse,
    OrderItemDetailResponse, OrderItemResponse, OrderListQuery, OrderResponse, RefundOrderRequest,
    UpdateOrderStatusRequest,
};

// Re-export admin handlers so `routes/admin_orders.rs` continues to
// reference them as `crate::routes::orders::admin_*`.
pub use admin_actions::{
    admin_cancel, admin_fulfill, admin_refund, admin_resend_confirmation, admin_update_status,
};
pub use admin_list::{admin_index, admin_show};

/// Build orders router with all endpoints
pub fn router() -> Router<AppState> {
    Router::new()
        .route("/", get(user::index))
        .route("/by-number/{order_number}", get(user::show_by_number))
        .route("/{id}", get(user::show))
}

/// Build admin orders router - ICT 7 COMPLETE
pub fn admin_router() -> Router<AppState> {
    Router::new()
        .route("/", get(admin_list::admin_index))
        .route("/{id}", get(admin_list::admin_show))
        // Order management endpoints - ICT 7 FIX
        .route("/{id}/status", post(admin_actions::admin_update_status))
        .route("/{id}/refund", post(admin_actions::admin_refund))
        .route("/{id}/cancel", post(admin_actions::admin_cancel))
        .route("/{id}/fulfill", post(admin_actions::admin_fulfill))
        .route(
            "/{id}/resend-confirmation",
            post(admin_actions::admin_resend_confirmation),
        )
}
