//! Revolution Trading Pros API
//! 
//! Ultimate Backend - Rust + Cloudflare Workers
//! December 2025 - Using worker-rs v0.7.1

pub mod config;
pub mod error;
pub mod db;
pub mod models;
pub mod routes;
pub mod services;
pub mod middleware;
pub mod utils;

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

/// Create CORS preflight response
fn cors_preflight() -> Result<Response> {
    let mut headers = Headers::new();
    headers.set("Access-Control-Allow-Origin", "*")?;
    headers.set("Access-Control-Allow-Methods", "GET, POST, PUT, DELETE, OPTIONS")?;
    headers.set("Access-Control-Allow-Headers", "Content-Type, Authorization, X-Requested-With")?;
    headers.set("Access-Control-Max-Age", "86400")?;
    Ok(Response::empty()?.with_headers(headers).with_status(204))
}

#[event(fetch)]
pub async fn main(req: Request, env: Env, _ctx: Context) -> Result<Response> {
    log_request(&req);

    // Handle CORS preflight requests
    if req.method() == Method::Options {
        return cors_preflight();
    }

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
        .get_async("/health", |_, _| async move {
            Response::from_json(&serde_json::json!({
                "status": "ok",
                "service": "revolution-api",
                "timestamp": crate::utils::now().to_rfc3339()
            }))
        })
        .get_async("/health/live", |_, _| async move {
            Response::from_json(&serde_json::json!({"status": "ok"}))
        })
        .get_async("/health/ready", |_, _| async move {
            Response::from_json(&serde_json::json!({"status": "ready"}))
        })
        
        // Public API routes
        .get_async("/api/posts", |req, ctx| async move {
            routes::posts::list(req, ctx).await
        })
        .get_async("/api/posts/:slug", |req, ctx| async move {
            routes::posts::show(req, ctx).await
        })
        .get_async("/api/products", |req, ctx| async move {
            routes::products::list(req, ctx).await
        })
        .get_async("/api/products/:id", |req, ctx| async move {
            routes::products::show(req, ctx).await
        })
        .get_async("/api/indicators", |req, ctx| async move {
            routes::products::indicators(req, ctx).await
        })
        .get_async("/api/indicators/:slug", |req, ctx| async move {
            routes::products::indicator_show(req, ctx).await
        })
        
        // Auth routes
        .post_async("/api/register", |req, ctx| async move {
            routes::auth::register(req, ctx).await
        })
        .post_async("/api/login", |req, ctx| async move {
            routes::auth::login(req, ctx).await
        })
        .post_async("/api/login/mfa", |req, ctx| async move {
            routes::auth::login_mfa(req, ctx).await
        })
        .post_async("/api/logout", |req, ctx| async move {
            routes::auth::logout(req, ctx).await
        })
        .post_async("/api/auth/refresh", |req, ctx| async move {
            routes::auth::refresh_token(req, ctx).await
        })
        .get_async("/api/auth/check", |req, ctx| async move {
            routes::auth::check(req, ctx).await
        })
        .post_async("/api/forgot-password", |req, ctx| async move {
            routes::auth::forgot_password(req, ctx).await
        })
        .post_async("/api/reset-password", |req, ctx| async move {
            routes::auth::reset_password(req, ctx).await
        })
        
        // User routes
        .get_async("/api/me", |req, ctx| async move {
            routes::users::me(req, ctx).await
        })
        .get_async("/api/me/memberships", |req, ctx| async move {
            routes::users::memberships(req, ctx).await
        })
        .get_async("/api/me/products", |req, ctx| async move {
            routes::users::products(req, ctx).await
        })
        .get_async("/api/me/sessions", |req, ctx| async move {
            routes::users::sessions(req, ctx).await
        })
        .put_async("/api/me/password", |req, ctx| async move {
            routes::users::change_password(req, ctx).await
        })
        
        // Subscription routes
        .get_async("/api/my/subscriptions", |req, ctx| async move {
            routes::subscriptions::list(req, ctx).await
        })
        .post_async("/api/my/subscriptions", |req, ctx| async move {
            routes::subscriptions::create(req, ctx).await
        })
        .get_async("/api/my/subscriptions/:id", |req, ctx| async move {
            routes::subscriptions::show(req, ctx).await
        })
        .post_async("/api/my/subscriptions/:id/cancel", |req, ctx| async move {
            routes::subscriptions::cancel(req, ctx).await
        })
        
        // Cart routes
        .post_async("/api/cart/checkout", |req, ctx| async move {
            routes::cart::checkout(req, ctx).await
        })
        .post_async("/api/cart/calculate-tax", |req, ctx| async move {
            routes::cart::calculate_tax(req, ctx).await
        })
        
        // Newsletter routes
        .post_async("/api/newsletter/subscribe", |req, ctx| async move {
            routes::newsletter::subscribe(req, ctx).await
        })
        .get_async("/api/newsletter/confirm", |req, ctx| async move {
            routes::newsletter::confirm(req, ctx).await
        })
        .get_async("/api/newsletter/unsubscribe", |req, ctx| async move {
            routes::newsletter::unsubscribe(req, ctx).await
        })
        
        // SEO routes
        .get_async("/api/sitemap", |req, ctx| async move {
            routes::seo::sitemap_index(req, ctx).await
        })
        .get_async("/api/sitemap/posts/:page", |req, ctx| async move {
            routes::seo::sitemap_posts(req, ctx).await
        })
        .get_async("/api/robots.txt", |req, ctx| async move {
            routes::seo::robots(req, ctx).await
        })
        
        // Webhook routes
        .post_async("/api/webhooks/stripe", |req, ctx| async move {
            routes::webhooks::stripe(req, ctx).await
        })
        .post_async("/api/webhooks/postmark/inbound", |req, ctx| async move {
            routes::webhooks::postmark_inbound(req, ctx).await
        })
        
        // Admin routes
        .get_async("/api/admin/posts", |req, ctx| async move {
            routes::admin::posts::list(req, ctx).await
        })
        .post_async("/api/admin/posts", |req, ctx| async move {
            routes::admin::posts::create(req, ctx).await
        })
        .get_async("/api/admin/posts/:id", |req, ctx| async move {
            routes::admin::posts::show(req, ctx).await
        })
        .put_async("/api/admin/posts/:id", |req, ctx| async move {
            routes::admin::posts::update(req, ctx).await
        })
        .delete_async("/api/admin/posts/:id", |req, ctx| async move {
            routes::admin::posts::delete(req, ctx).await
        })
        .get_async("/api/admin/categories", |req, ctx| async move {
            routes::admin::categories::list(req, ctx).await
        })
        .post_async("/api/admin/categories", |req, ctx| async move {
            routes::admin::categories::create(req, ctx).await
        })
        .put_async("/api/admin/categories/:id", |req, ctx| async move {
            routes::admin::categories::update(req, ctx).await
        })
        .delete_async("/api/admin/categories/:id", |req, ctx| async move {
            routes::admin::categories::delete(req, ctx).await
        })
        .get_async("/api/admin/users", |req, ctx| async move {
            routes::admin::users::list(req, ctx).await
        })
        .get_async("/api/admin/users/:id", |req, ctx| async move {
            routes::admin::users::show(req, ctx).await
        })
        .post_async("/api/admin/users/:id/ban", |req, ctx| async move {
            routes::admin::users::ban(req, ctx).await
        })
        .post_async("/api/admin/users/:id/unban", |req, ctx| async move {
            routes::admin::users::unban(req, ctx).await
        })
        .get_async("/api/admin/products", |req, ctx| async move {
            routes::admin::products::list(req, ctx).await
        })
        .post_async("/api/admin/products", |req, ctx| async move {
            routes::admin::products::create(req, ctx).await
        })
        .put_async("/api/admin/products/:id", |req, ctx| async move {
            routes::admin::products::update(req, ctx).await
        })
        .delete_async("/api/admin/products/:id", |req, ctx| async move {
            routes::admin::products::delete(req, ctx).await
        })
        .get_async("/api/admin/subscriptions", |req, ctx| async move {
            routes::admin::subscriptions::list(req, ctx).await
        })
        .get_async("/api/admin/subscriptions/plans", |req, ctx| async move {
            routes::admin::subscriptions::plans(req, ctx).await
        })
        .get_async("/api/admin/media", |req, ctx| async move {
            routes::admin::media::list(req, ctx).await
        })
        .post_async("/api/admin/media/upload", |req, ctx| async move {
            routes::admin::media::upload(req, ctx).await
        })
        .delete_async("/api/admin/media/:id", |req, ctx| async move {
            routes::admin::media::delete(req, ctx).await
        })
        .get_async("/api/admin/email/templates", |req, ctx| async move {
            routes::admin::email::templates_list(req, ctx).await
        })
        .post_async("/api/admin/email/templates", |req, ctx| async move {
            routes::admin::email::templates_create(req, ctx).await
        })
        .get_async("/api/admin/email/campaigns", |req, ctx| async move {
            routes::admin::email::campaigns_list(req, ctx).await
        })
        .post_async("/api/admin/email/campaigns", |req, ctx| async move {
            routes::admin::email::campaigns_create(req, ctx).await
        })
        .get_async("/api/admin/email/subscribers", |req, ctx| async move {
            routes::admin::email::subscribers_list(req, ctx).await
        })
        .run(req, env)
        .await
        .map(|response| {
            let mut headers = Headers::new();
            let _ = headers.set("Access-Control-Allow-Origin", "*");
            let _ = headers.set("Access-Control-Allow-Methods", "GET, POST, PUT, DELETE, OPTIONS");
            let _ = headers.set("Access-Control-Allow-Headers", "Content-Type, Authorization, X-Requested-With");
            response.with_headers(headers)
        })
}
