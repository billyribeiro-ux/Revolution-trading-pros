//! API routes module

pub mod health;
pub mod auth;
pub mod users;
pub mod posts;
pub mod products;
pub mod subscriptions;
pub mod cart;
pub mod newsletter;
pub mod seo;
pub mod webhooks;
pub mod admin;

use worker::Router;
use crate::AppState;

/// Build the complete API router
pub fn build_router() -> Router<'static, AppState> {
    Router::new()
        // Health checks
        .get("/health", health::liveness)
        .get("/health/live", health::liveness)
        .get("/health/ready", health::readiness)
        
        // Public auth routes
        .post("/api/register", auth::register)
        .post("/api/login", auth::login)
        .post("/api/login/mfa", auth::login_mfa)
        .post("/api/auth/refresh", auth::refresh_token)
        .post("/api/forgot-password", auth::forgot_password)
        .post("/api/reset-password", auth::reset_password)
        
        // Protected auth routes
        .post("/api/logout", auth::logout)
        .get("/api/auth/check", auth::check)
        
        // User routes
        .get("/api/me", users::me)
        .get("/api/me/memberships", users::memberships)
        .get("/api/me/products", users::products)
        .get("/api/me/sessions", users::sessions)
        .put("/api/me/password", users::change_password)
        .post("/api/me/mfa/enable", users::enable_mfa)
        .post("/api/me/mfa/disable", users::disable_mfa)
        .delete("/api/me/sessions/:session_id", users::revoke_session)
        
        // Public posts
        .get("/api/posts", posts::list)
        .get("/api/posts/:slug", posts::show)
        
        // Public products
        .get("/api/products", products::list)
        .get("/api/products/:id", products::show)
        .get("/api/indicators", products::indicators)
        .get("/api/indicators/:slug", products::indicator_show)
        
        // Subscriptions (protected)
        .get("/api/my/subscriptions", subscriptions::list)
        .post("/api/my/subscriptions", subscriptions::create)
        .get("/api/my/subscriptions/:id", subscriptions::show)
        .post("/api/my/subscriptions/:id/cancel", subscriptions::cancel)
        .post("/api/my/subscriptions/:id/pause", subscriptions::pause)
        .post("/api/my/subscriptions/:id/resume", subscriptions::resume)
        
        // Cart & Checkout
        .post("/api/cart/checkout", cart::checkout)
        .post("/api/cart/calculate-tax", cart::calculate_tax)
        
        // Newsletter
        .post("/api/newsletter/subscribe", newsletter::subscribe)
        .get("/api/newsletter/confirm", newsletter::confirm)
        .get("/api/newsletter/unsubscribe", newsletter::unsubscribe)
        
        // SEO
        .get("/api/sitemap", seo::sitemap_index)
        .get("/api/sitemap/posts/:page", seo::sitemap_posts)
        .get("/api/robots.txt", seo::robots)
        
        // Webhooks
        .post("/api/webhooks/stripe", webhooks::stripe)
        .post("/api/webhooks/postmark/inbound", webhooks::postmark_inbound)
        
        // Admin routes
        .get("/api/admin/posts", admin::posts::list)
        .post("/api/admin/posts", admin::posts::create)
        .get("/api/admin/posts/:id", admin::posts::show)
        .put("/api/admin/posts/:id", admin::posts::update)
        .delete("/api/admin/posts/:id", admin::posts::delete)
        
        .get("/api/admin/categories", admin::categories::list)
        .post("/api/admin/categories", admin::categories::create)
        .put("/api/admin/categories/:id", admin::categories::update)
        .delete("/api/admin/categories/:id", admin::categories::delete)
        
        .get("/api/admin/users", admin::users::list)
        .get("/api/admin/users/:id", admin::users::show)
        .post("/api/admin/users/:id/ban", admin::users::ban)
        .post("/api/admin/users/:id/unban", admin::users::unban)
        
        .get("/api/admin/products", admin::products::list)
        .post("/api/admin/products", admin::products::create)
        .put("/api/admin/products/:id", admin::products::update)
        .delete("/api/admin/products/:id", admin::products::delete)
        
        .get("/api/admin/subscriptions", admin::subscriptions::list)
        .get("/api/admin/subscriptions/plans", admin::subscriptions::plans)
        
        .get("/api/admin/media", admin::media::list)
        .post("/api/admin/media/upload", admin::media::upload)
        .delete("/api/admin/media/:id", admin::media::delete)
}
