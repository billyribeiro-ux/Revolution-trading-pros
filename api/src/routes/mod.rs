//! API routes - Revolution Trading Pros
//! Apple ICT 11+ Principal Engineer Grade - January 2026

pub mod health;
pub mod auth;
pub mod users;
pub mod user;  // Singular /user routes for frontend compatibility
pub mod robots;
pub mod sitemap;
pub mod categories;
pub mod tags;
pub mod redirects;
pub mod media;
pub mod members;
// pub mod settings; // Already handled by admin.rs
pub mod courses;
pub mod payments;
pub mod search;
pub mod products;
// pub mod indicators; // Legacy - replaced by member_indicators
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
pub mod schedules;
pub mod cms;
pub mod realtime;
pub mod unified_videos;
pub mod video_advanced;
pub mod courses_admin;
pub mod indicators_admin;

use axum::Router;
use crate::AppState;

/// Build the API router with all routes
pub fn api_router() -> Router<AppState> {
    Router::new()
        .nest("/auth", auth::router())
        .nest("/users", users::router())
        .nest("/user", user::router())  // Singular /user routes for frontend
        // Legacy courses route - replaced by member_courses
        // .nest("/courses", courses::router())
        .nest("/payments", payments::router())
        .nest("/search", search::router())
        .nest("/products", products::router())
        // .nest("/indicators", indicators::router()) // Removed - replaced by member_indicators
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
        .nest("/schedules", schedules::router())
        .nest("/admin/schedules", schedules::admin_router())
        .nest("/admin/unified-videos", unified_videos::router())
        .nest("/unified-videos", unified_videos::router()) // Public access route
        .nest("/admin/video-advanced", video_advanced::router())
        .nest("/video-advanced", video_advanced::router())
        // Enhanced Course & Indicator Management - ICT 7 Grade
        .nest("/admin/courses-enhanced", courses_admin::router())
        .nest("/admin/indicators-enhanced", indicators_admin::router())
        .nest("/migrate", migrate::router())
        .nest("/my/orders", orders::router())
        .nest("/my/subscriptions", subscriptions::my_router())
        .nest("/logout", auth::logout_router())
        // CMS routes - ICT 11+ Advanced Features
        .nest("/admin/cms", cms::admin_router())
        .nest("/preview", cms::preview_router())
        // Real-time updates - SSE
        .nest("/realtime", realtime::router())
        .nest("/popups", popups::router())
        .nest("/trading-rooms", trading_rooms::router())
        .nest("/admin/courses", admin_courses::router())
        .nest("/courses", member_courses::public_router())
        .nest("/my/courses", member_courses::member_router())
        // Indicator Management System
        .nest("/admin/indicators", admin_indicators::router())
        .nest("/admin/page-layouts", admin_page_layouts::router())
        .nest("/indicators", member_indicators::public_router())
        .nest("/my/indicators", member_indicators::member_router())
        .nest("/download", member_indicators::download_router())
        .merge(robots::router())
        .merge(sitemap::router())
        .merge(categories::router())
        .merge(tags::router())
        .merge(redirects::router())
        .merge(media::router())
        .merge(members::router())
}
