//! API routes - Revolution Trading Pros
//! Apple ICT 11+ Principal Engineer Grade - January 2026

pub mod auth;
pub mod categories;
pub mod health;
pub mod media;
pub mod members;
pub mod redirects;
pub mod robots;
pub mod sitemap;
pub mod tags;
pub mod user; // Singular /user routes for frontend compatibility
pub mod users;
// pub mod settings; // Already handled by admin.rs
pub mod courses;
pub mod payments;
pub mod products;
pub mod search;
// pub mod indicators; // Legacy - replaced by member_indicators
pub mod admin;
pub mod analytics;
pub mod checkout;
pub mod cms_delivery;
pub mod cms_revisions;
pub mod cms_v2;
pub mod cms_v2_enterprise;
pub mod contacts;
pub mod coupons;
pub mod courses_admin;
pub mod newsletter;
pub mod orders;
pub mod posts;
pub mod realtime;
pub mod schedules;
pub mod security;
pub mod subscriptions;
pub mod videos;
// pub mod indicators_admin; // TODO: Fix SQLx tuple decoding issues
pub mod admin_courses;
pub mod admin_indicators;
pub mod admin_member_management; // ICT 11+: Full member CRUD, ban, export
pub mod admin_members; // ICT 7: Member segments, tags, and filters
pub mod admin_page_layouts;
pub mod admin_videos;
pub mod bunny_upload; // ICT 7: Bunny.net video upload API
pub mod cms_ai_assist; // ICT 7+: AI-powered content assistance
pub mod cms_reusable_blocks;
pub mod connections;
pub mod crm; // ICT 7: CRM Admin Routes - FluentCRM Pro equivalent
pub mod email_templates;
pub mod favorites; // ICT 7: User favorites persistence
pub mod forms;
pub mod member_courses;
pub mod member_indicators;
pub mod migrate;
pub mod organization;
pub mod popups;
pub mod room_content;
pub mod room_resources;
pub mod subscriptions_admin;
pub mod trading_rooms;
pub mod watchlist; // ICT 7: Service connection status // ICT 7: Teams & Departments // ICT 7+: Reusable content blocks library

use crate::AppState;
use axum::Router;

/// Build the API router with all routes
pub fn api_router() -> Router<AppState> {
    Router::new()
        .nest("/auth", auth::router())
        .nest("/users", users::router())
        .nest("/user", user::router()) // Singular /user routes for frontend
        // Legacy courses route - replaced by member_courses
        // .nest("/courses", courses::router())
        .nest("/payments", payments::router())
        .nest("/search", search::router())
        .nest("/products", products::router())
        // .nest("/indicators", indicators::router()) // Removed - replaced by member_indicators
        .nest("/posts", posts::router())
        .nest("/admin/posts", posts::admin_router())
        .nest("/subscriptions", subscriptions::router())
        .nest("/newsletter", newsletter::router())
        .nest("/admin", admin::router())
        .nest("/admin/products", products::admin_router()) // Admin product CRUD
        .nest("/checkout", checkout::router())
        .nest("/videos", videos::router())
        .nest("/analytics", analytics::router())
        .nest("/contacts", contacts::router())
        .nest("/coupons", coupons::router())
        .nest("/security", security::router())
        .nest("/schedules", schedules::router())
        .nest("/admin/schedules", schedules::admin_router())
        // Enhanced Course & Indicator Management - ICT 7 Grade
        .nest("/admin/courses-enhanced", courses_admin::router())
        // .nest("/admin/indicators-enhanced", indicators_admin::router()) // TODO: Fix SQLx issues
        .nest("/migrate", migrate::router())
        .nest("/my/orders", orders::router())
        .nest("/my/subscriptions", subscriptions::my_router())
        .nest("/logout", auth::logout_router())
        // CMS v2 - Custom CMS Implementation (surpasses Storyblok)
        .nest("/admin/cms-v2", cms_v2::admin_router())
        .nest("/cms", cms_v2::public_router())
        // CMS v2 Enterprise - Audit, Workflow, Preview Tokens
        .nest("/admin/cms-v2/enterprise", cms_v2_enterprise::router())
        .merge(cms_v2_enterprise::public_router())
        // CMS Revisions - Revision comparison and restore - ICT 7+ Grade
        .nest("/api/cms", cms_revisions::router())
        // CMS Delivery API - Public content delivery with search
        .nest("/delivery", cms_delivery::delivery_router())
        // Real-time updates - SSE
        .nest("/realtime", realtime::router())
        .nest("/popups", popups::router())
        .nest("/trading-rooms", trading_rooms::router())
        .nest("/admin/trading-rooms", trading_rooms::admin_router())
        .nest("/admin/courses", admin_courses::router())
        .nest("/courses", member_courses::public_router())
        .nest("/my/courses", member_courses::member_router())
        // Indicator Management System
        .nest("/admin/indicators", admin_indicators::router())
        // Video Management System (Learning Center, Daily Videos, Weekly Watchlist, Room Archives)
        .nest("/admin/videos", admin_videos::router())
        // Video Advanced Analytics - ICT 7 Grade (frontend compatibility)
        .nest("/video-advanced", admin_videos::analytics_router())
        .nest("/admin/page-layouts", admin_page_layouts::router())
        // Media Admin - ICT 7 Grade
        .nest("/admin/media", media::admin_router())
        // Forms - ICT 7 Grade (public + admin)
        .nest("/forms", forms::public_router())
        .nest("/admin/forms", forms::admin_router())
        // Email Templates Admin - ICT 7 Grade
        .nest("/admin/email/templates", email_templates::admin_router())
        // Email Settings - frontend compatibility
        .nest("/admin/email", email_templates::admin_router())
        // Subscriptions Admin - ICT 7 Grade
        .nest(
            "/admin/subscriptions",
            subscriptions_admin::subscriptions_router(),
        )
        .nest(
            "/admin/subscriptions/plans",
            subscriptions_admin::plans_router(),
        )
        .nest("/indicators", member_indicators::public_router())
        .nest("/my/indicators", member_indicators::member_router())
        .nest("/download", member_indicators::download_router())
        // Room Content Management - Trade Plans, Alerts, Weekly Videos
        .nest("/room-content", room_content::public_router())
        .nest("/admin/room-content", room_content::admin_router())
        // Weekly Watchlist
        .nest("/watchlist", watchlist::router())
        // Room Resources - Unified content management (videos, PDFs, docs, images)
        .nest("/room-resources", room_resources::public_router())
        .nest("/admin/room-resources", room_resources::admin_router())
        // Bunny.net Video Upload - ICT 7 Grade
        .nest("/admin/bunny", bunny_upload::admin_router())
        // CRM Admin - ICT 7 Grade (FluentCRM Pro equivalent)
        .nest("/admin/crm", crm::router())
        // Connections Status - ICT 7 Grade
        .nest("/admin/connections", connections::admin_router())
        // Admin Members - Segments, Tags, Filters - ICT 7 Grade
        .nest("/admin/members", admin_members::router())
        // Admin Member Management - Full CRUD, Ban, Export - ICT 11+ Grade
        .nest(
            "/admin/member-management",
            admin_member_management::router(),
        )
        // Organization - Teams & Departments - ICT 7 Grade
        .nest("/admin/organization/teams", organization::teams_router())
        .nest(
            "/admin/organization/departments",
            organization::departments_router(),
        )
        // User Favorites - ICT 7 Grade
        .nest("/favorites", favorites::router())
        // CMS AI Assist - ICT 7+ AI-powered content assistance
        .nest("/cms/ai", cms_ai_assist::router())
        // CMS Reusable Blocks - ICT 7+ Block library management
        .nest("/cms/reusable-blocks", cms_reusable_blocks::admin_router())
        .nest(
            "/cms/reusable-blocks/public",
            cms_reusable_blocks::public_router(),
        )
        .merge(robots::router())
        .merge(sitemap::router())
        .merge(categories::router())
        .merge(tags::router())
        .merge(redirects::router())
        .merge(members::router())
}
