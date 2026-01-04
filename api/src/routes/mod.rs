//! API routes - Revolution Trading Pros
//! Apple ICT 11+ Principal Engineer Grade - December 2025

pub mod health;
pub mod auth;
pub mod users;
pub mod user;  // Singular /user routes for frontend compatibility
pub mod robots;
pub mod sitemap;
pub mod categories;
pub mod tags;
pub mod redirects;
// pub mod settings; // Already handled by admin.rs
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
pub mod videos;
pub mod analytics;
pub mod contacts;
pub mod coupons;
pub mod security;
pub mod orders;

use axum::Router;
use crate::AppState;

/// Build the API router with all routes
pub fn api_router() -> Router<AppState> {
    Router::new()
        .nest("/auth", auth::router())
        .nest("/users", users::router())
        .nest("/user", user::router())  // Singular /user routes for frontend
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
        .nest("/videos", videos::router())
        .nest("/analytics", analytics::router())
        .nest("/contacts", contacts::router())
        .nest("/coupons", coupons::router())
        .nest("/security", security::router())
        .nest("/my/orders", orders::router())
        .nest("/my/subscriptions", subscriptions::my_router())
        .nest("/logout", auth::logout_router())
        .merge(robots::router())
        .merge(sitemap::router())
        .merge(categories::router())
        .merge(tags::router())
        .merge(redirects::router())
}
