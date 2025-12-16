//! SEO routes - sitemaps, robots.txt

use worker::{Request, Response, RouteContext};
use crate::AppState;
use crate::error::ApiError;

/// GET /api/sitemap - Sitemap index
pub async fn sitemap_index(_req: Request, ctx: RouteContext<AppState>) -> worker::Result<Response> {
    let site_url = &ctx.data.config.site_url;
    
    // Get total post count for pagination
    let total: i64 = ctx.data.db.query_one::<CountResult>(
        "SELECT COUNT(*) as count FROM posts WHERE status = 'published'",
        vec![]
    ).await?
    .map(|r| r.count)
    .unwrap_or(0);

    let posts_per_sitemap = 1000;
    let total_pages = ((total as f64) / (posts_per_sitemap as f64)).ceil() as i64;

    let mut xml = String::from(r#"<?xml version="1.0" encoding="UTF-8"?>
<sitemapindex xmlns="http://www.sitemaps.org/schemas/sitemap/0.9">"#);

    // Posts sitemaps
    for page in 1..=total_pages.max(1) {
        xml.push_str(&format!(r#"
  <sitemap>
    <loc>{}/api/sitemap/posts/{}</loc>
    <lastmod>{}</lastmod>
  </sitemap>"#,
            site_url,
            page,
            chrono::Utc::now().format("%Y-%m-%d")
        ));
    }

    // Static pages
    xml.push_str(&format!(r#"
  <sitemap>
    <loc>{}/api/sitemap/pages</loc>
    <lastmod>{}</lastmod>
  </sitemap>"#,
        site_url,
        chrono::Utc::now().format("%Y-%m-%d")
    ));

    xml.push_str("\n</sitemapindex>");

    Response::ok(xml)
        .map(|r| r.with_headers({
            let mut headers = worker::Headers::new();
            headers.set("Content-Type", "application/xml").ok();
            headers.set("Cache-Control", "public, max-age=3600").ok();
            headers
        }))
}

/// GET /api/sitemap/posts/:page - Posts sitemap
pub async fn sitemap_posts(_req: Request, ctx: RouteContext<AppState>) -> worker::Result<Response> {
    let page: i64 = ctx.param("page")
        .and_then(|p| p.parse().ok())
        .unwrap_or(1);

    let per_page = 1000;
    let offset = (page - 1) * per_page;
    let site_url = &ctx.data.config.site_url;

    let posts: Vec<SitemapPost> = ctx.data.db.query(
        r#"
        SELECT slug, updated_at 
        FROM posts 
        WHERE status = 'published'
        ORDER BY published_at DESC
        LIMIT $1 OFFSET $2
        "#,
        vec![serde_json::json!(per_page), serde_json::json!(offset)]
    ).await?;

    let mut xml = String::from(r#"<?xml version="1.0" encoding="UTF-8"?>
<urlset xmlns="http://www.sitemaps.org/schemas/sitemap/0.9">"#);

    for post in posts {
        xml.push_str(&format!(r#"
  <url>
    <loc>{}/blog/{}</loc>
    <lastmod>{}</lastmod>
    <changefreq>weekly</changefreq>
    <priority>0.8</priority>
  </url>"#,
            site_url,
            post.slug,
            post.updated_at.format("%Y-%m-%d")
        ));
    }

    xml.push_str("\n</urlset>");

    Response::ok(xml)
        .map(|r| r.with_headers({
            let mut headers = worker::Headers::new();
            headers.set("Content-Type", "application/xml").ok();
            headers.set("Cache-Control", "public, max-age=3600").ok();
            headers
        }))
}

/// GET /api/robots.txt - Robots.txt
pub async fn robots(_req: Request, ctx: RouteContext<AppState>) -> worker::Result<Response> {
    let site_url = &ctx.data.config.site_url;
    
    let robots = format!(r#"# Revolution Trading Pros
# https://revolutiontradingpros.com

User-agent: *
Allow: /

# Sitemaps
Sitemap: {}/api/sitemap

# Disallow admin and API routes
Disallow: /admin/
Disallow: /api/admin/
Disallow: /api/webhooks/

# Disallow user-specific pages
Disallow: /dashboard/
Disallow: /account/
Disallow: /checkout/

# Allow search engines to crawl public content
Allow: /blog/
Allow: /courses/
Allow: /indicators/
Allow: /live-trading-rooms/

# Crawl delay for politeness
Crawl-delay: 1
"#, site_url);

    Response::ok(robots)
        .map(|r| r.with_headers({
            let mut headers = worker::Headers::new();
            headers.set("Content-Type", "text/plain").ok();
            headers.set("Cache-Control", "public, max-age=86400").ok();
            headers
        }))
}

#[derive(serde::Deserialize)]
struct CountResult {
    count: i64,
}

#[derive(serde::Deserialize)]
struct SitemapPost {
    slug: String,
    updated_at: chrono::DateTime<chrono::Utc>,
}
