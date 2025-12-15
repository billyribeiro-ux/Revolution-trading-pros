//! API routes

pub mod health;
pub mod auth;
pub mod users;
pub mod courses;
pub mod payments;
pub mod search;

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
}
