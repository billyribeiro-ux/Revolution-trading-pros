//! Trading Indicator models - Revolution Trading Pros
//! Apple ICT 11+ Principal Engineer Grade - December 2025

use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use validator::Validate;

/// Trading indicator entity
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Indicator {
    pub id: i64,
    pub name: String,
    pub slug: String,
    pub description: Option<String>,
    pub long_description: Option<String>,
    pub price: f64,
    pub is_active: bool,
    pub platform: String, // tradingview, thinkorswim, etc.
    pub version: String,
    pub download_url: Option<String>,
    pub documentation_url: Option<String>,
    pub thumbnail: Option<String>,
    pub screenshots: Option<serde_json::Value>,
    pub features: Option<serde_json::Value>,
    pub requirements: Option<serde_json::Value>,
    // SEO fields
    pub meta_title: Option<String>,
    pub meta_description: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

/// Create indicator request
#[derive(Debug, Deserialize, Validate)]
pub struct CreateIndicator {
    #[validate(length(min = 1, max = 255))]
    pub name: String,
    pub description: Option<String>,
    pub long_description: Option<String>,
    #[validate(range(min = 0.0))]
    pub price: f64,
    pub is_active: Option<bool>,
    #[validate(length(min = 1, max = 50))]
    pub platform: String,
    pub version: Option<String>,
    pub download_url: Option<String>,
    pub documentation_url: Option<String>,
    pub thumbnail: Option<String>,
    pub screenshots: Option<serde_json::Value>,
    pub features: Option<serde_json::Value>,
    pub requirements: Option<serde_json::Value>,
    pub meta_title: Option<String>,
    pub meta_description: Option<String>,
}

/// Update indicator request
#[derive(Debug, Deserialize, Validate)]
pub struct UpdateIndicator {
    pub name: Option<String>,
    pub description: Option<String>,
    pub long_description: Option<String>,
    pub price: Option<f64>,
    pub is_active: Option<bool>,
    pub platform: Option<String>,
    pub version: Option<String>,
    pub download_url: Option<String>,
    pub documentation_url: Option<String>,
    pub thumbnail: Option<String>,
    pub screenshots: Option<serde_json::Value>,
    pub features: Option<serde_json::Value>,
    pub requirements: Option<serde_json::Value>,
    pub meta_title: Option<String>,
    pub meta_description: Option<String>,
}

/// User indicator ownership
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct UserIndicator {
    pub id: i64,
    pub user_id: i64,
    pub indicator_id: i64,
    pub purchased_at: NaiveDateTime,
    pub license_key: Option<String>,
    pub expires_at: Option<NaiveDateTime>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

/// Indicator with user ownership status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndicatorWithOwnership {
    #[serde(flatten)]
    pub indicator: Indicator,
    pub owned: bool,
    pub purchased_at: Option<NaiveDateTime>,
    pub license_key: Option<String>,
}
