//! Hardcoded fallback data used when the database is unavailable.
//!
//! R27-B5 split: moved verbatim from the pre-split `trading_rooms.rs`.
//! The canonical 6-room order and section list is preserved exactly so
//! the admin dashboard / sections page still renders the same shape
//! when the DB query returns zero rows or errors.

use serde_json::json;

pub(super) fn get_fallback_rooms() -> serde_json::Value {
    json!([
        {
            "id": 1,
            "name": "Day Trading Room",
            "slug": "day-trading-room",
            "type": "trading_room",
            "short_description": "Live day trading sessions with real-time trade alerts and daily market analysis.",
            "icon": "chart-line",
            "color": "#3b82f6",
            "sort_order": 1,
            "is_active": true,
            "is_featured": true,
            "is_public": true,
            "available_sections": ["introduction", "latest_updates", "premium_daily_videos", "watchlist", "learning_center"],
            "created_at": "2026-01-01T00:00:00Z",
            "updated_at": "2026-01-15T00:00:00Z"
        },
        {
            "id": 2,
            "name": "Swing Trading Room",
            "slug": "swing-trading-room",
            "type": "trading_room",
            "short_description": "Live swing trading sessions with swing trade alerts and weekly watchlist.",
            "icon": "trending-up",
            "color": "#10b981",
            "sort_order": 2,
            "is_active": true,
            "is_featured": false,
            "is_public": true,
            "available_sections": ["introduction", "latest_updates", "premium_daily_videos", "watchlist", "learning_center"],
            "created_at": "2026-01-01T00:00:00Z",
            "updated_at": "2026-01-15T00:00:00Z"
        },
        {
            "id": 3,
            "name": "Small Account Mentorship",
            "slug": "small-account-mentorship",
            "type": "mentorship",
            "short_description": "Personalized mentorship for growing small trading accounts with proper risk management.",
            "icon": "wallet",
            "color": "#f59e0b",
            "sort_order": 3,
            "is_active": true,
            "is_featured": false,
            "is_public": true,
            "available_sections": ["introduction", "latest_updates", "premium_daily_videos", "learning_center"],
            "created_at": "2026-01-01T00:00:00Z",
            "updated_at": "2026-01-15T00:00:00Z"
        },
        {
            "id": 4,
            "name": "Explosive Swings",
            "slug": "explosive-swings",
            "type": "alert_service",
            "short_description": "High momentum swing trade alerts with detailed analysis and risk/reward ratios.",
            "icon": "rocket",
            "color": "#ef4444",
            "sort_order": 4,
            "is_active": true,
            "is_featured": false,
            "is_public": true,
            "available_sections": ["introduction", "latest_updates", "premium_daily_videos", "watchlist", "weekly_alerts", "learning_center"],
            "created_at": "2026-01-01T00:00:00Z",
            "updated_at": "2026-01-15T00:00:00Z"
        },
        {
            "id": 5,
            "name": "SPX Profit Pulse",
            "slug": "spx-profit-pulse",
            "type": "alert_service",
            "short_description": "Premium SPX options alerts for intraday opportunities with high win rate setups.",
            "icon": "activity",
            "color": "#8b5cf6",
            "sort_order": 5,
            "is_active": true,
            "is_featured": false,
            "is_public": true,
            "available_sections": ["introduction", "latest_updates", "premium_daily_videos", "learning_center"],
            "created_at": "2026-01-01T00:00:00Z",
            "updated_at": "2026-01-15T00:00:00Z"
        },
        {
            "id": 6,
            "name": "High Octane Scanner",
            "slug": "high-octane-scanner",
            "type": "alert_service",
            "short_description": "Real-time stock scanner with momentum alerts and breakout detection.",
            "icon": "radar",
            "color": "#06b6d4",
            "sort_order": 6,
            "is_active": true,
            "is_featured": false,
            "is_public": true,
            "available_sections": ["introduction", "latest_updates", "premium_daily_videos", "learning_center"],
            "created_at": "2026-01-01T00:00:00Z",
            "updated_at": "2026-01-15T00:00:00Z"
        }
    ])
}

pub(super) fn get_fallback_sections() -> serde_json::Value {
    json!([
        {
            "id": 1,
            "section_key": "introduction",
            "name": "Introduction",
            "description": "Main welcome and overview videos for the room",
            "icon": "video",
            "sort_order": 1,
            "is_active": true,
            "allowed_resource_types": ["video"],
            "max_items": 3,
            "restricted_to_rooms": null
        },
        {
            "id": 2,
            "section_key": "latest_updates",
            "name": "Latest Updates",
            "description": "Recent announcements, news, and updates",
            "icon": "bell",
            "sort_order": 2,
            "is_active": true,
            "allowed_resource_types": ["video", "pdf", "document"],
            "max_items": null,
            "restricted_to_rooms": null
        },
        {
            "id": 3,
            "section_key": "premium_daily_videos",
            "name": "Premium Daily Videos",
            "description": "Daily trading analysis and market commentary",
            "icon": "calendar",
            "sort_order": 3,
            "is_active": true,
            "allowed_resource_types": ["video"],
            "max_items": null,
            "restricted_to_rooms": null
        },
        {
            "id": 4,
            "section_key": "watchlist",
            "name": "Watchlist",
            "description": "Weekly stock watchlist videos and documents",
            "icon": "list-check",
            "sort_order": 4,
            "is_active": true,
            "allowed_resource_types": ["video", "pdf", "spreadsheet"],
            "max_items": null,
            "restricted_to_rooms": null
        },
        {
            "id": 5,
            "section_key": "weekly_alerts",
            "name": "Weekly Alerts",
            "description": "Weekly alert summaries and trade recaps",
            "icon": "alert-triangle",
            "sort_order": 5,
            "is_active": true,
            "allowed_resource_types": ["video"],
            "max_items": null,
            "restricted_to_rooms": ["explosive-swings"]
        },
        {
            "id": 6,
            "section_key": "learning_center",
            "name": "Learning Center",
            "description": "Educational tutorials, guides, and courses",
            "icon": "school",
            "sort_order": 6,
            "is_active": true,
            "allowed_resource_types": ["video", "pdf", "document"],
            "max_items": null,
            "restricted_to_rooms": null
        }
    ])
}
