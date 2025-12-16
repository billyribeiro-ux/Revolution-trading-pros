//! Revolution Trading Pros API
//! 
//! Ultimate Backend - Rust + Cloudflare Workers
//! December 2025

pub mod config;
pub mod error;
pub mod db;
pub mod models;
pub mod routes;
pub mod services;
pub mod middleware;

use worker::*;
use crate::config::Config;
use crate::db::{Database, Cache};
use crate::services::{JwtService, StripeService, PostmarkService, R2Service, SearchService};

/// Application state shared across all routes
#[derive(Clone)]
pub struct AppState {
    pub db: Database,
    pub cache: Cache,
    pub config: Config,
    pub services: AppServices,
}

#[derive(Clone)]
pub struct AppServices {
    pub jwt: JwtService,
    pub stripe: StripeService,
    pub postmark: PostmarkService,
    pub r2: R2Service,
    pub search: SearchService,
}

fn log_request(req: &Request) {
    console_log!(
        "{} - [{}] \"{}\"",
        Date::now().to_string(),
        req.method().to_string(),
        req.path(),
    );
}

#[event(fetch)]
pub async fn main(req: Request, env: Env, _ctx: Context) -> Result<Response> {
    log_request(&req);

    // Initialize panic hook for better error messages
    console_error_panic_hook::set_once();

    // Load configuration
    let config = match Config::from_env(&env) {
        Ok(c) => c,
        Err(e) => {
            console_error!("Failed to load config: {:?}", e);
            return Response::error("Internal Server Error", 500);
        }
    };

    // Initialize database
    let db = Database::new(&config.database_url);
    
    // Initialize cache
    let cache = match Cache::new(&config.redis_url) {
        Ok(c) => c,
        Err(e) => {
            console_error!("Failed to connect to cache: {:?}", e);
            return Response::error("Internal Server Error", 500);
        }
    };

    // Initialize services
    let services = AppServices {
        jwt: JwtService::new(&config.jwt_secret, &config.jwt_issuer, &config.jwt_audience),
        stripe: StripeService::new(&config.stripe_secret_key, &config.stripe_webhook_secret),
        postmark: PostmarkService::new(&config.postmark_api_key, "noreply@revolutiontradingpros.com", "Revolution Trading Pros"),
        r2: R2Service::new(
            "revolution-trading-media",
            "https://pub-a6d59af18a9645e6a7b38dca4d53f2af.r2.dev",
            "",
            "",
            "",
        ),
        search: SearchService::new(&config.meilisearch_url, &config.meilisearch_api_key),
    };

    // Create app state
    let state = AppState {
        db,
        cache,
        config,
        services,
    };

    // Build and run router
    let router = Router::with_data(state);

    router
        // Health routes
        .get("/health", |_, _| async move {
            Response::from_json(&serde_json::json!({
                "status": "ok",
                "service": "revolution-api",
                "timestamp": chrono::Utc::now().to_rfc3339()
            }))
        })
        .get("/health/live", |_, _| async move {
            Response::from_json(&serde_json::json!({"status": "ok"}))
        })
        .get("/health/ready", |_, _| async move {
            Response::from_json(&serde_json::json!({"status": "ready"}))
        })
        
        // Public API routes
        .get("/api/posts", |req, ctx| async move {
            routes::posts::list(req, ctx).await
        })
        .get("/api/posts/:slug", |req, ctx| async move {
            routes::posts::show(req, ctx).await
        })
        .get("/api/products", |req, ctx| async move {
            routes::products::list(req, ctx).await
        })
        .get("/api/products/:id", |req, ctx| async move {
            routes::products::show(req, ctx).await
        })
        .get("/api/indicators", |req, ctx| async move {
            routes::products::indicators(req, ctx).await
        })
        .get("/api/indicators/:slug", |req, ctx| async move {
            routes::products::indicator_show(req, ctx).await
        })
        
        // Auth routes
        .post("/api/register", |req, ctx| async move {
            routes::auth::register(req, ctx).await
        })
        .post("/api/login", |req, ctx| async move {
            routes::auth::login(req, ctx).await
        })
        .post("/api/login/mfa", |req, ctx| async move {
            routes::auth::login_mfa(req, ctx).await
        })
        .post("/api/logout", |req, ctx| async move {
            routes::auth::logout(req, ctx).await
        })
        .post("/api/auth/refresh", |req, ctx| async move {
            routes::auth::refresh_token(req, ctx).await
        })
        .get("/api/auth/check", |req, ctx| async move {
            routes::auth::check(req, ctx).await
        })
        .post("/api/forgot-password", |req, ctx| async move {
            routes::auth::forgot_password(req, ctx).await
        })
        .post("/api/reset-password", |req, ctx| async move {
            routes::auth::reset_password(req, ctx).await
        })
        
        // User routes
        .get("/api/me", |req, ctx| async move {
            routes::users::me(req, ctx).await
        })
        .get("/api/me/memberships", |req, ctx| async move {
            routes::users::memberships(req, ctx).await
        })
        .get("/api/me/products", |req, ctx| async move {
            routes::users::products(req, ctx).await
        })
        .get("/api/me/sessions", |req, ctx| async move {
            routes::users::sessions(req, ctx).await
        })
        .put("/api/me/password", |req, ctx| async move {
            routes::users::change_password(req, ctx).await
        })
        
        // Subscription routes
        .get("/api/my/subscriptions", |req, ctx| async move {
            routes::subscriptions::list(req, ctx).await
        })
        .post("/api/my/subscriptions", |req, ctx| async move {
            routes::subscriptions::create(req, ctx).await
        })
        .get("/api/my/subscriptions/:id", |req, ctx| async move {
            routes::subscriptions::show(req, ctx).await
        })
        .post("/api/my/subscriptions/:id/cancel", |req, ctx| async move {
            routes::subscriptions::cancel(req, ctx).await
        })
        
        // Cart routes
        .post("/api/cart/checkout", |req, ctx| async move {
            routes::cart::checkout(req, ctx).await
        })
        .post("/api/cart/calculate-tax", |req, ctx| async move {
            routes::cart::calculate_tax(req, ctx).await
        })
        
        // Newsletter routes
        .post("/api/newsletter/subscribe", |req, ctx| async move {
            routes::newsletter::subscribe(req, ctx).await
        })
        .get("/api/newsletter/confirm", |req, ctx| async move {
            routes::newsletter::confirm(req, ctx).await
        })
        .get("/api/newsletter/unsubscribe", |req, ctx| async move {
            routes::newsletter::unsubscribe(req, ctx).await
        })
        
        // SEO routes
        .get("/api/sitemap", |req, ctx| async move {
            routes::seo::sitemap_index(req, ctx).await
        })
        .get("/api/sitemap/posts/:page", |req, ctx| async move {
            routes::seo::sitemap_posts(req, ctx).await
        })
        .get("/api/robots.txt", |req, ctx| async move {
            routes::seo::robots(req, ctx).await
        })
        
        // Webhook routes
        .post("/api/webhooks/stripe", |req, ctx| async move {
            routes::webhooks::stripe(req, ctx).await
        })
        .post("/api/webhooks/postmark/inbound", |req, ctx| async move {
            routes::webhooks::postmark_inbound(req, ctx).await
        })
        
        // Admin routes
        .get("/api/admin/posts", |req, ctx| async move {
            routes::admin::posts::list(req, ctx).await
        })
        .post("/api/admin/posts", |req, ctx| async move {
            routes::admin::posts::create(req, ctx).await
        })
        .get("/api/admin/posts/:id", |req, ctx| async move {
            routes::admin::posts::show(req, ctx).await
        })
        .put("/api/admin/posts/:id", |req, ctx| async move {
            routes::admin::posts::update(req, ctx).await
        })
        .delete("/api/admin/posts/:id", |req, ctx| async move {
            routes::admin::posts::delete(req, ctx).await
        })
        .get("/api/admin/categories", |req, ctx| async move {
            routes::admin::categories::list(req, ctx).await
        })
        .post("/api/admin/categories", |req, ctx| async move {
            routes::admin::categories::create(req, ctx).await
        })
        .put("/api/admin/categories/:id", |req, ctx| async move {
            routes::admin::categories::update(req, ctx).await
        })
        .delete("/api/admin/categories/:id", |req, ctx| async move {
            routes::admin::categories::delete(req, ctx).await
        })
        .get("/api/admin/users", |req, ctx| async move {
            routes::admin::users::list(req, ctx).await
        })
        .get("/api/admin/users/:id", |req, ctx| async move {
            routes::admin::users::show(req, ctx).await
        })
        .post("/api/admin/users/:id/ban", |req, ctx| async move {
            routes::admin::users::ban(req, ctx).await
        })
        .post("/api/admin/users/:id/unban", |req, ctx| async move {
            routes::admin::users::unban(req, ctx).await
        })
        .get("/api/admin/products", |req, ctx| async move {
            routes::admin::products::list(req, ctx).await
        })
        .post("/api/admin/products", |req, ctx| async move {
            routes::admin::products::create(req, ctx).await
        })
        .put("/api/admin/products/:id", |req, ctx| async move {
            routes::admin::products::update(req, ctx).await
        })
        .delete("/api/admin/products/:id", |req, ctx| async move {
            routes::admin::products::delete(req, ctx).await
        })
        .get("/api/admin/subscriptions", |req, ctx| async move {
            routes::admin::subscriptions::list(req, ctx).await
        })
        .get("/api/admin/subscriptions/plans", |req, ctx| async move {
            routes::admin::subscriptions::plans(req, ctx).await
        })
        .get("/api/admin/media", |req, ctx| async move {
            routes::admin::media::list(req, ctx).await
        })
        .post("/api/admin/media/upload", |req, ctx| async move {
            routes::admin::media::upload(req, ctx).await
        })
        .delete("/api/admin/media/:id", |req, ctx| async move {
            routes::admin::media::delete(req, ctx).await
        })
        .get("/api/admin/email/templates", |req, ctx| async move {
            routes::admin::email::templates_list(req, ctx).await
        })
        .post("/api/admin/email/templates", |req, ctx| async move {
            routes::admin::email::templates_create(req, ctx).await
        })
        .get("/api/admin/email/campaigns", |req, ctx| async move {
            routes::admin::email::campaigns_list(req, ctx).await
        })
        .post("/api/admin/email/campaigns", |req, ctx| async move {
            routes::admin::email::campaigns_create(req, ctx).await
        })
        .get("/api/admin/email/subscribers", |req, ctx| async move {
            routes::admin::email::subscribers_list(req, ctx).await
        })
        .run(req, env)
        .await
}
