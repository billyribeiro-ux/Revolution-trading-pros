//! Robots.txt Controller
//! ICT 11+ Principal Engineer - Dynamic robots.txt generation
//!
//! Generates robots.txt based on environment with caching.
//! Blocks AI crawlers, protects admin areas, includes sitemap references.

use axum::{
    extract::State,
    http::{header, StatusCode},
    response::{IntoResponse, Response},
    routing::get,
    Router,
};
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::RwLock;

use crate::AppState;

/// Cache for robots.txt content
struct RobotsCache {
    content: String,
    expires_at: Instant,
}

lazy_static::lazy_static! {
    static ref ROBOTS_CACHE: Arc<RwLock<Option<RobotsCache>>> = Arc::new(RwLock::new(None));
}

const CACHE_TTL: Duration = Duration::from_secs(3600); // 1 hour

/// GET /robots.txt - Generate robots.txt
#[tracing::instrument(skip(state))]
pub async fn robots_txt(State(state): State<AppState>) -> Response {
    // Check cache
    {
        let cache = ROBOTS_CACHE.read().await;
        if let Some(cached) = cache.as_ref() {
            if Instant::now() < cached.expires_at {
                return (
                    StatusCode::OK,
                    [
                        (header::CONTENT_TYPE, "text/plain"),
                        (header::CACHE_CONTROL, "public, max-age=3600"),
                    ],
                    cached.content.clone(),
                )
                    .into_response();
            }
        }
    }

    // Generate new content
    let content = generate_robots_txt(&state).await;

    // Update cache
    {
        let mut cache = ROBOTS_CACHE.write().await;
        *cache = Some(RobotsCache {
            content: content.clone(),
            expires_at: Instant::now() + CACHE_TTL,
        });
    }

    (
        StatusCode::OK,
        [
            (header::CONTENT_TYPE, "text/plain"),
            (header::CACHE_CONTROL, "public, max-age=3600"),
        ],
        content,
    )
        .into_response()
}

/// Generate robots.txt content based on environment
async fn generate_robots_txt(state: &AppState) -> String {
    let is_production =
        std::env::var("ENVIRONMENT").unwrap_or_else(|_| "development".to_string()) == "production";

    if !is_production {
        return get_disallow_all();
    }

    let site_url = std::env::var("SITE_URL")
        .unwrap_or_else(|_| "https://revolutiontradingpros.com".to_string());

    get_production_rules(&site_url)
}

/// Get production robots.txt rules
fn get_production_rules(site_url: &str) -> String {
    let mut rules = Vec::new();

    // Header
    rules.push("# Revolution Trading Pros - robots.txt".to_string());
    rules.push("# Generated dynamically by Rust API".to_string());
    rules.push(String::new());
    rules.push("User-agent: *".to_string());

    // Allow general access
    rules.push("Allow: /".to_string());

    // Disallow admin and private paths
    rules.push(String::new());
    rules.push("# Admin & Private Areas".to_string());
    rules.push("Disallow: /admin".to_string());
    rules.push("Disallow: /admin/".to_string());
    rules.push("Disallow: /api/admin/".to_string());
    rules.push("Disallow: /dashboard".to_string());
    rules.push("Disallow: /dashboard/".to_string());
    rules.push("Disallow: /login".to_string());
    rules.push("Disallow: /register".to_string());
    rules.push("Disallow: /forgot-password".to_string());
    rules.push("Disallow: /reset-password".to_string());

    // Disallow utility paths
    rules.push(String::new());
    rules.push("# Utility Paths".to_string());
    rules.push("Disallow: /api/".to_string());
    rules.push("Allow: /api/sitemap".to_string());
    rules.push("Allow: /api/feed/".to_string());
    rules.push("Disallow: /*.json$".to_string());
    rules.push("Disallow: /_app/".to_string());
    rules.push("Disallow: /cdn-cgi/".to_string());

    // Disallow search and filter pages (duplicate content)
    rules.push(String::new());
    rules.push("# Search & Filter Pages (prevent duplicate content)".to_string());
    rules.push("Disallow: /*?search=".to_string());
    rules.push("Disallow: /*?sort=".to_string());
    rules.push("Disallow: /*?filter=".to_string());
    rules.push("Disallow: /*?page=".to_string());

    // Googlebot-specific rules
    rules.push(String::new());
    rules.push("# Googlebot Specific".to_string());
    rules.push("User-agent: Googlebot".to_string());
    rules.push("Allow: /".to_string());
    rules.push("Allow: /*.js".to_string());
    rules.push("Allow: /*.css".to_string());
    rules.push("Disallow: /admin".to_string());

    // Bingbot-specific rules (favorable for IndexNow)
    rules.push(String::new());
    rules.push("# Bingbot Specific".to_string());
    rules.push("User-agent: Bingbot".to_string());
    rules.push("Allow: /".to_string());
    rules.push("Crawl-delay: 1".to_string());

    // GPTBot and AI crawlers
    rules.push(String::new());
    rules.push("# AI Crawlers".to_string());
    rules.push("User-agent: GPTBot".to_string());
    rules.push("Disallow: /".to_string());
    rules.push(String::new());
    rules.push("User-agent: ChatGPT-User".to_string());
    rules.push("Disallow: /".to_string());
    rules.push(String::new());
    rules.push("User-agent: Google-Extended".to_string());
    rules.push("Disallow: /".to_string());
    rules.push(String::new());
    rules.push("User-agent: CCBot".to_string());
    rules.push("Disallow: /".to_string());
    rules.push(String::new());
    rules.push("User-agent: anthropic-ai".to_string());
    rules.push("Disallow: /".to_string());

    // Sitemap location
    rules.push(String::new());
    rules.push("# Sitemaps".to_string());
    rules.push(format!("Sitemap: {}/api/sitemap", site_url));
    rules.push(format!("Sitemap: {}/api/sitemap/categories", site_url));
    rules.push(format!("Sitemap: {}/api/sitemap/tags", site_url));

    // Host directive
    rules.push(String::new());
    rules.push("# Canonical Host".to_string());
    let host = site_url
        .trim_start_matches("https://")
        .trim_start_matches("http://")
        .split('/')
        .next()
        .unwrap_or("revolutiontradingpros.com");
    rules.push(format!("Host: {}", host));

    rules.join("\n")
}

/// Get disallow-all rules for non-production
fn get_disallow_all() -> String {
    [
        "# Revolution Trading Pros - robots.txt",
        "# Non-production environment - all crawling disabled",
        "",
        "User-agent: *",
        "Disallow: /",
    ]
    .join("\n")
}

/// Build the robots router
pub fn router() -> Router<AppState> {
    Router::new().route("/robots.txt", get(robots_txt))
}
