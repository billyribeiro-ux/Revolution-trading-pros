//! Sitemap Controller
//! ICT 11+ Principal Engineer - Dynamic XML sitemap generation for SEO
//!
//! Generates XML sitemaps with caching for optimal performance.
//! Supports sitemap index for large sites (>50,000 URLs).

use axum::{
    extract::{Path, State},
    http::{header, StatusCode},
    response::{IntoResponse, Response},
    routing::get,
    Router,
};
use chrono::{DateTime, Utc};
use sqlx::PgPool;
use std::sync::Arc;
use tokio::sync::RwLock;
use std::time::{Duration, Instant};

use crate::AppState;

/// Cache for sitemap content
struct SitemapCache {
    content: String,
    expires_at: Instant,
}

lazy_static::lazy_static! {
    static ref SITEMAP_CACHE: Arc<RwLock<std::collections::HashMap<String, SitemapCache>>> = 
        Arc::new(RwLock::new(std::collections::HashMap::new()));
}

const CACHE_TTL: Duration = Duration::from_secs(3600); // 1 hour
const MAX_URLS_PER_SITEMAP: i64 = 50000;

/// GET /sitemap - Main sitemap or sitemap index
#[tracing::instrument(skip(state))]
pub async fn index(State(state): State<AppState>) -> Response {
    // Check if we need sitemap index
    let post_count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM posts WHERE status = 'published'")
        .fetch_one(state.db.pool())
        .await
        .unwrap_or(0);

    if post_count > MAX_URLS_PER_SITEMAP {
        sitemap_index(State(state)).await
    } else {
        main_sitemap(State(state)).await
    }
}

/// Generate sitemap index for large sites
#[tracing::instrument(skip(state))]
pub async fn sitemap_index(State(state): State<AppState>) -> Response {
    // Check cache
    let cache_key = "sitemap:index".to_string();
    {
        let cache = SITEMAP_CACHE.read().await;
        if let Some(cached) = cache.get(&cache_key) {
            if Instant::now() < cached.expires_at {
                return xml_response(&cached.content);
            }
        }
    }

    // Generate sitemap index
    let post_count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM posts WHERE status = 'published'")
        .fetch_one(state.db.pool())
        .await
        .unwrap_or(0);

    let sitemap_count = (post_count as f64 / MAX_URLS_PER_SITEMAP as f64).ceil() as i64;
    let site_url = std::env::var("SITE_URL").unwrap_or_else(|_| "https://revolutiontradingpros.com".to_string());

    let mut content = String::from("<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n");
    content.push_str("<sitemapindex xmlns=\"http://www.sitemaps.org/schemas/sitemap/0.9\">\n");

    // Posts sitemaps
    for i in 1..=sitemap_count {
        content.push_str("  <sitemap>\n");
        content.push_str(&format!("    <loc>{}/api/sitemap/posts/{}</loc>\n", site_url, i));
        content.push_str(&format!("    <lastmod>{}</lastmod>\n", Utc::now().format("%Y-%m-%d")));
        content.push_str("  </sitemap>\n");
    }

    // Categories sitemap
    content.push_str("  <sitemap>\n");
    content.push_str(&format!("    <loc>{}/api/sitemap/categories</loc>\n", site_url));
    content.push_str(&format!("    <lastmod>{}</lastmod>\n", Utc::now().format("%Y-%m-%d")));
    content.push_str("  </sitemap>\n");

    // Tags sitemap
    content.push_str("  <sitemap>\n");
    content.push_str(&format!("    <loc>{}/api/sitemap/tags</loc>\n", site_url));
    content.push_str(&format!("    <lastmod>{}</lastmod>\n", Utc::now().format("%Y-%m-%d")));
    content.push_str("  </sitemap>\n");

    content.push_str("</sitemapindex>");

    // Update cache
    {
        let mut cache = SITEMAP_CACHE.write().await;
        cache.insert(cache_key, SitemapCache {
            content: content.clone(),
            expires_at: Instant::now() + CACHE_TTL,
        });
    }

    xml_response(&content)
}

/// Generate main sitemap with all URLs
#[tracing::instrument(skip(state))]
pub async fn main_sitemap(State(state): State<AppState>) -> Response {
    // Check cache
    let cache_key = "sitemap:main".to_string();
    {
        let cache = SITEMAP_CACHE.read().await;
        if let Some(cached) = cache.get(&cache_key) {
            if Instant::now() < cached.expires_at {
                return xml_response(&cached.content);
            }
        }
    }

    let site_url = std::env::var("SITE_URL").unwrap_or_else(|_| "https://revolutiontradingpros.com".to_string());
    let mut content = build_xml_header(false);

    // Static pages
    content.push_str(&build_url(&format!("{}/", site_url), &Utc::now(), "daily", "1.0", None));
    content.push_str(&build_url(&format!("{}/blog", site_url), &Utc::now(), "daily", "0.9", None));
    content.push_str(&build_url(&format!("{}/about", site_url), &Utc::now(), "monthly", "0.7", None));
    content.push_str(&build_url(&format!("{}/contact", site_url), &Utc::now(), "monthly", "0.6", None));

    // Blog posts
    let posts: Vec<(String, DateTime<Utc>)> = sqlx::query_as(
        "SELECT slug, COALESCE(updated_at, published_at) as last_modified 
         FROM posts 
         WHERE status = 'published' 
         ORDER BY published_at DESC"
    )
    .fetch_all(state.db.pool())
    .await
    .unwrap_or_default();

    for (slug, last_modified) in posts {
        content.push_str(&build_url(&format!("{}/blog/{}", site_url, slug), &last_modified, "weekly", "0.8", None));
    }

    // Categories
    let categories: Vec<(String, DateTime<Utc>)> = sqlx::query_as(
        "SELECT slug, updated_at FROM categories WHERE is_active = true"
    )
    .fetch_all(state.db.pool())
    .await
    .unwrap_or_default();

    for (slug, updated_at) in categories {
        content.push_str(&build_url(&format!("{}/blog/category/{}", site_url, slug), &updated_at, "weekly", "0.6", None));
    }

    // Tags
    let tags: Vec<(String, DateTime<Utc>)> = sqlx::query_as(
        "SELECT slug, updated_at FROM tags"
    )
    .fetch_all(state.db.pool())
    .await
    .unwrap_or_default();

    for (slug, updated_at) in tags {
        content.push_str(&build_url(&format!("{}/blog/tag/{}", site_url, slug), &updated_at, "weekly", "0.5", None));
    }

    content.push_str(&build_xml_footer());

    // Update cache
    {
        let mut cache = SITEMAP_CACHE.write().await;
        cache.insert(cache_key, SitemapCache {
            content: content.clone(),
            expires_at: Instant::now() + CACHE_TTL,
        });
    }

    xml_response(&content)
}

/// GET /sitemap/posts/:page - Generate posts sitemap (paginated)
#[tracing::instrument(skip(state))]
pub async fn posts(State(state): State<AppState>, Path(page): Path<i64>) -> Response {
    let cache_key = format!("sitemap:posts:{}", page);
    {
        let cache = SITEMAP_CACHE.read().await;
        if let Some(cached) = cache.get(&cache_key) {
            if Instant::now() < cached.expires_at {
                return xml_response(&cached.content);
            }
        }
    }

    let site_url = std::env::var("SITE_URL").unwrap_or_else(|_| "https://revolutiontradingpros.com".to_string());
    let offset = (page - 1) * MAX_URLS_PER_SITEMAP;

    let posts: Vec<(String, DateTime<Utc>, Option<String>)> = sqlx::query_as(
        "SELECT slug, COALESCE(updated_at, published_at) as last_modified, featured_image 
         FROM posts 
         WHERE status = 'published' 
         ORDER BY published_at DESC 
         LIMIT $1 OFFSET $2"
    )
    .bind(MAX_URLS_PER_SITEMAP)
    .bind(offset)
    .fetch_all(state.db.pool())
    .await
    .unwrap_or_default();

    let mut content = build_xml_header(true);

    for (slug, last_modified, featured_image) in posts {
        content.push_str(&build_url(&format!("{}/blog/{}", site_url, slug), &last_modified, "weekly", "0.8", featured_image.as_deref()));
    }

    content.push_str(&build_xml_footer());

    // Update cache
    {
        let mut cache = SITEMAP_CACHE.write().await;
        cache.insert(cache_key, SitemapCache {
            content: content.clone(),
            expires_at: Instant::now() + CACHE_TTL,
        });
    }

    xml_response(&content)
}

/// GET /sitemap/categories - Generate categories sitemap
#[tracing::instrument(skip(state))]
pub async fn categories(State(state): State<AppState>) -> Response {
    let cache_key = "sitemap:categories".to_string();
    {
        let cache = SITEMAP_CACHE.read().await;
        if let Some(cached) = cache.get(&cache_key) {
            if Instant::now() < cached.expires_at {
                return xml_response(&cached.content);
            }
        }
    }

    let site_url = std::env::var("SITE_URL").unwrap_or_else(|_| "https://revolutiontradingpros.com".to_string());

    let categories: Vec<(String, DateTime<Utc>)> = sqlx::query_as(
        "SELECT slug, updated_at FROM categories WHERE is_active = true"
    )
    .fetch_all(state.db.pool())
    .await
    .unwrap_or_default();

    let mut content = build_xml_header(false);

    for (slug, updated_at) in categories {
        content.push_str(&build_url(&format!("{}/blog/category/{}", site_url, slug), &updated_at, "weekly", "0.6", None));
    }

    content.push_str(&build_xml_footer());

    // Update cache
    {
        let mut cache = SITEMAP_CACHE.write().await;
        cache.insert(cache_key, SitemapCache {
            content: content.clone(),
            expires_at: Instant::now() + CACHE_TTL,
        });
    }

    xml_response(&content)
}

/// GET /sitemap/tags - Generate tags sitemap
#[tracing::instrument(skip(state))]
pub async fn tags_sitemap(State(state): State<AppState>) -> Response {
    let cache_key = "sitemap:tags".to_string();
    {
        let cache = SITEMAP_CACHE.read().await;
        if let Some(cached) = cache.get(&cache_key) {
            if Instant::now() < cached.expires_at {
                return xml_response(&cached.content);
            }
        }
    }

    let site_url = std::env::var("SITE_URL").unwrap_or_else(|_| "https://revolutiontradingpros.com".to_string());

    let tags: Vec<(String, DateTime<Utc>)> = sqlx::query_as(
        "SELECT slug, updated_at FROM tags"
    )
    .fetch_all(state.db.pool())
    .await
    .unwrap_or_default();

    let mut content = build_xml_header(false);

    for (slug, updated_at) in tags {
        content.push_str(&build_url(&format!("{}/blog/tag/{}", site_url, slug), &updated_at, "weekly", "0.5", None));
    }

    content.push_str(&build_xml_footer());

    // Update cache
    {
        let mut cache = SITEMAP_CACHE.write().await;
        cache.insert(cache_key, SitemapCache {
            content: content.clone(),
            expires_at: Instant::now() + CACHE_TTL,
        });
    }

    xml_response(&content)
}

/// Build XML header
fn build_xml_header(include_image: bool) -> String {
    let mut header = String::from("<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n");
    if include_image {
        header.push_str("<urlset xmlns=\"http://www.sitemaps.org/schemas/sitemap/0.9\" xmlns:image=\"http://www.google.com/schemas/sitemap-image/1.1\">\n");
    } else {
        header.push_str("<urlset xmlns=\"http://www.sitemaps.org/schemas/sitemap/0.9\">\n");
    }
    header
}

/// Build XML footer
fn build_xml_footer() -> String {
    String::from("</urlset>")
}

/// Build URL entry
fn build_url(
    loc: &str,
    lastmod: &DateTime<Utc>,
    changefreq: &str,
    priority: &str,
    image: Option<&str>,
) -> String {
    let mut url = String::from("  <url>\n");
    url.push_str(&format!("    <loc>{}</loc>\n", html_escape::encode_text(loc)));
    url.push_str(&format!("    <lastmod>{}</lastmod>\n", lastmod.format("%Y-%m-%d")));
    url.push_str(&format!("    <changefreq>{}</changefreq>\n", changefreq));
    url.push_str(&format!("    <priority>{}</priority>\n", priority));

    if let Some(img) = image {
        url.push_str("    <image:image>\n");
        url.push_str(&format!("      <image:loc>{}</image:loc>\n", html_escape::encode_text(img)));
        url.push_str("    </image:image>\n");
    }

    url.push_str("  </url>\n");
    url
}

/// Create XML response
fn xml_response(content: &str) -> Response {
    (
        StatusCode::OK,
        [
            (header::CONTENT_TYPE, "application/xml"),
            (header::CACHE_CONTROL, "public, max-age=3600"),
        ],
        content.to_string(),
    )
        .into_response()
}

/// Build the sitemap router
pub fn router() -> Router<AppState> {
    Router::new()
        .route("/sitemap", get(index))
        .route("/sitemap/posts/:page", get(posts))
        .route("/sitemap/categories", get(categories))
        .route("/sitemap/tags", get(tags_sitemap))
}
