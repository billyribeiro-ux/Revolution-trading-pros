//! API Route Definitions
//!
//! ICT 11+ Principal Engineer Grade
//! Complete route mapping matching Laravel API

use axum::{
    middleware,
    routing::{delete, get, patch, post, put},
    Router,
};

use crate::{
    handlers,
    middleware as app_middleware,
    AppState,
};

/// Build all API routes
pub fn api_routes(state: AppState) -> Router {
    Router::new()
        // Health check endpoints (no auth)
        .nest("/api/health", health_routes())
        // Public routes (no auth)
        .nest("/api", public_routes())
        // Auth routes (no auth required)
        .nest("/api/auth", auth_routes(state.clone()))
        // Protected routes (auth required)
        .nest("/api", protected_routes(state.clone()))
        // Admin routes (auth + admin role)
        .nest("/api/admin", admin_routes(state.clone()))
        // Webhook routes (signature verified)
        .nest("/api/webhooks", webhook_routes(state.clone()))
        .with_state(state)
}

/// Health check routes (Kubernetes probes)
fn health_routes() -> Router<AppState> {
    Router::new()
        .route("/live", get(handlers::health::liveness))
        .route("/ready", get(handlers::health::readiness))
        .route("/optimization", get(handlers::health::optimization))
}

/// Public routes (no authentication required)
fn public_routes() -> Router<AppState> {
    Router::new()
        // Time
        .route("/time/now", get(handlers::time::now))
        // Posts (public blog)
        .route("/posts", get(handlers::posts::index))
        .route("/posts/:slug", get(handlers::posts::show))
        // Indicators (public)
        .route("/indicators", get(handlers::indicators::index))
        .route("/indicators/:slug", get(handlers::indicators::show))
        // Videos (public)
        .route("/videos", get(handlers::videos::index))
        .route("/videos/:id", get(handlers::videos::show))
        // Newsletter
        .route("/newsletter/subscribe", post(handlers::newsletter::subscribe))
        .route("/newsletter/confirm", get(handlers::newsletter::confirm))
        .route("/newsletter/unsubscribe", get(handlers::newsletter::unsubscribe))
        // Popups
        .route("/popups/active", get(handlers::popups::active))
        .route("/popups/:id/impression", post(handlers::popups::impression))
        .route("/popups/:id/conversion", post(handlers::popups::conversion))
        // Payments config
        .route("/payments/config", get(handlers::payments::config))
        // Analytics tracking
        .route("/analytics/track", post(handlers::analytics::track_event))
        .route("/analytics/pageview", post(handlers::analytics::track_pageview))
        // Consent config
        .route("/consent/config", get(handlers::consent::public_settings))
        // Sitemap
        .route("/sitemap", get(handlers::sitemap::index))
        .route("/sitemap/posts", get(handlers::sitemap::posts))
        .route("/sitemap/categories", get(handlers::sitemap::categories))
}

/// Authentication routes
fn auth_routes(state: AppState) -> Router<AppState> {
    Router::new()
        .route("/register", post(handlers::auth::register))
        .route("/login", post(handlers::auth::login))
        .route("/logout", post(handlers::auth::logout))
        .route("/refresh", post(handlers::auth::refresh_token))
        .route("/forgot-password", post(handlers::auth::forgot_password))
        .route("/reset-password", post(handlers::auth::reset_password))
        .route("/verify-email/:token", get(handlers::auth::verify_email))
        .with_state(state)
}

/// Protected routes (require authentication)
fn protected_routes(state: AppState) -> Router<AppState> {
    Router::new()
        // Current user (/me)
        .route("/me", get(handlers::me::show))
        .route("/me", put(handlers::me::update))
        .route("/me/memberships", get(handlers::me::memberships))
        .route("/me/products", get(handlers::me::products))
        .route("/me/indicators", get(handlers::me::indicators))
        // User profile
        .route("/user/profile", get(handlers::user::get_profile))
        .route("/user/profile", put(handlers::user::update_profile))
        // User memberships (alias)
        .route("/user/memberships", get(handlers::me::memberships))
        .route("/user/indicators", get(handlers::me::indicators))
        // User orders
        .route("/my/orders", get(handlers::orders::index))
        .route("/my/orders/:id", get(handlers::orders::show))
        // User subscriptions
        .route("/my/subscriptions", get(handlers::subscriptions::index))
        .route("/my/subscriptions/:id", get(handlers::subscriptions::show))
        .route("/my/subscriptions/:id/cancel", post(handlers::subscriptions::cancel))
        .route("/my/subscriptions/:id/pause", post(handlers::subscriptions::pause))
        .route("/my/subscriptions/:id/resume", post(handlers::subscriptions::resume))
        .route("/my/subscriptions/:id/reactivate", post(handlers::subscriptions::reactivate))
        .route("/my/subscriptions/:id/invoices", get(handlers::subscriptions::invoices))
        .route("/my/subscriptions/:id/payments", get(handlers::subscriptions::payments))
        // Payment methods
        .route("/user/payment-methods", get(handlers::payment_methods::index))
        .route("/user/payment-methods", post(handlers::payment_methods::store))
        .route("/user/payment-methods/:id", delete(handlers::payment_methods::destroy))
        // User indicators
        .route("/user/indicators", get(handlers::user_indicators::index))
        .route("/user/indicators/:id", get(handlers::user_indicators::show))
        .route("/user/indicators/:id/download", get(handlers::user_indicators::download))
        // Cart & Checkout
        .route("/cart/checkout", post(handlers::cart::checkout))
        .route("/cart/calculate-tax", post(handlers::cart::calculate_tax))
        // Payments
        .route("/payments/create-intent", post(handlers::payments::create_intent))
        .route("/payments/create-checkout", post(handlers::payments::create_checkout))
        .route("/payments/confirm", post(handlers::payments::confirm))
        .route("/payments/order/:order_number/status", get(handlers::payments::order_status))
        // Trading rooms
        .route("/trading-rooms", get(handlers::trading_rooms::index))
        .route("/trading-rooms/:slug", get(handlers::trading_rooms::show))
        .route("/trading-rooms/:slug/videos", get(handlers::trading_rooms::videos))
        .route("/trading-rooms/:slug/sso", post(handlers::trading_rooms::generate_sso))
        // Logout
        .route("/logout", post(handlers::auth::logout))
        // Apply auth middleware
        .layer(middleware::from_fn_with_state(
            state.clone(),
            app_middleware::auth::require_auth,
        ))
        .with_state(state)
}

/// Admin routes (require admin role)
fn admin_routes(state: AppState) -> Router<AppState> {
    Router::new()
        // Users management
        .route("/users", get(handlers::admin::users::index))
        .route("/users", post(handlers::admin::users::store))
        .route("/users/:id", get(handlers::admin::users::show))
        .route("/users/:id", put(handlers::admin::users::update))
        .route("/users/:id", delete(handlers::admin::users::destroy))
        // Members management
        .route("/members", get(handlers::admin::members::index))
        .route("/members/:id", get(handlers::admin::members::show))
        // Subscriptions management
        .route("/subscriptions", get(handlers::admin::subscriptions::index))
        .route("/subscriptions/:id", get(handlers::admin::subscriptions::show))
        .route("/subscriptions/:id", put(handlers::admin::subscriptions::update))
        .route("/subscriptions/:id/cancel", post(handlers::admin::subscriptions::cancel))
        // Products management
        .route("/products", get(handlers::admin::products::index))
        .route("/products", post(handlers::admin::products::store))
        .route("/products/:id", get(handlers::admin::products::show))
        .route("/products/:id", put(handlers::admin::products::update))
        .route("/products/:id", delete(handlers::admin::products::destroy))
        // Coupons
        .route("/coupons", get(handlers::admin::coupons::index))
        .route("/coupons", post(handlers::admin::coupons::store))
        .route("/coupons/:id", get(handlers::admin::coupons::show))
        .route("/coupons/:id", put(handlers::admin::coupons::update))
        .route("/coupons/:id", delete(handlers::admin::coupons::destroy))
        .route("/coupons/user/available", get(handlers::admin::coupons::user_coupons))
        // Apply admin middleware
        .layer(middleware::from_fn_with_state(
            state.clone(),
            app_middleware::auth::require_admin,
        ))
        .layer(middleware::from_fn_with_state(
            state.clone(),
            app_middleware::auth::require_auth,
        ))
        .with_state(state)
}

/// Webhook routes (signature verified, no JWT)
fn webhook_routes(state: AppState) -> Router<AppState> {
    Router::new()
        // Stripe webhooks
        .route("/stripe", post(handlers::webhooks::stripe::handle))
        // Postmark inbound email
        .route("/postmark/inbound", post(handlers::webhooks::email::postmark))
        .with_state(state)
}
