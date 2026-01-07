//! Sitemap Handlers
//!
//! ICT 11+ Principal Engineer Grade

use axum::{extract::State, http::header, response::Response};

use crate::AppState;

pub async fn index(State(_state): State<AppState>) -> Response<String> {
    let xml = r#"<?xml version="1.0" encoding="UTF-8"?>
<sitemapindex xmlns="http://www.sitemaps.org/schemas/sitemap/0.9">
</sitemapindex>"#;

    Response::builder()
        .header(header::CONTENT_TYPE, "application/xml")
        .body(xml.to_string())
        .unwrap()
}

pub async fn posts(State(_state): State<AppState>) -> Response<String> {
    let xml = r#"<?xml version="1.0" encoding="UTF-8"?>
<urlset xmlns="http://www.sitemaps.org/schemas/sitemap/0.9">
</urlset>"#;

    Response::builder()
        .header(header::CONTENT_TYPE, "application/xml")
        .body(xml.to_string())
        .unwrap()
}

pub async fn categories(State(_state): State<AppState>) -> Response<String> {
    let xml = r#"<?xml version="1.0" encoding="UTF-8"?>
<urlset xmlns="http://www.sitemaps.org/schemas/sitemap/0.9">
</urlset>"#;

    Response::builder()
        .header(header::CONTENT_TYPE, "application/xml")
        .body(xml.to_string())
        .unwrap()
}
