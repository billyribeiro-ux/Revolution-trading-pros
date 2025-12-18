//! API routes - Revolution Trading Pros
//! Apple ICT 11+ Principal Engineer Grade - December 2025

pub mod health;
pub mod auth;
pub mod users;
pub mod courses;
pub mod payments;
pub mod search;
pub mod products;
pub mod indicators;
pub mod posts;
pub mod subscriptions;
pub mod newsletter;
pub mod admin;
pub mod checkout;

use axum::Router;
use crate::AppState;

/// Build the API router with all routes
pub fn api_router() -> Router<AppState> {
    Router::new()
        .nest("/auth", auth::router())
        .nest("/users", users::router())
        .nest("/courses", courses::router())
        .nest("/payments", payments::router())
        .nest("/search", search::router())
        .nest("/products", products::router())
        .nest("/indicators", indicators::router())
        .nest("/posts", posts::router())
        .nest("/subscriptions", subscriptions::router())
        .nest("/newsletter", newsletter::router())
        .nest("/admin", admin::router())
        .nest("/checkout", checkout::router())
}
