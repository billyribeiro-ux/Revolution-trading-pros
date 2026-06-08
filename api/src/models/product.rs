//! Product models - Revolution Trading Pros
//! Apple ICT 11+ Principal Engineer Grade - December 2025
//!
//! Handles products (courses, indicators, bundles) with full CRUD operations.

use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use validator::Validate;

/// Product type enumeration
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type, PartialEq, Eq)]
#[sqlx(type_name = "text", rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum ProductType {
    Course,
    Indicator,
    Bundle,
}

/// Product entity - represents courses, indicators, and bundles.
/// Monetary values are integer cents per architecture standard §1.2.
/// SQL queries reading from `products.price` (NUMERIC) must alias as
/// `(price * 100)::BIGINT AS price_cents`.
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Product {
    pub id: i64,
    pub name: String,
    pub slug: String,
    #[sqlx(rename = "type")]
    pub product_type: String,
    pub description: Option<String>,
    pub long_description: Option<String>,
    pub price_cents: i64,
    pub is_active: bool,
    pub metadata: Option<serde_json::Value>,
    pub thumbnail: Option<String>,
    // SEO fields
    pub meta_title: Option<String>,
    pub meta_description: Option<String>,
    pub indexable: bool,
    pub canonical_url: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

/// Product response for API. Monetary values are integer cents.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductResponse {
    pub id: i64,
    pub name: String,
    pub slug: String,
    pub product_type: String,
    pub description: Option<String>,
    pub long_description: Option<String>,
    pub price_cents: i64,
    pub is_active: bool,
    pub metadata: Option<serde_json::Value>,
    pub thumbnail: Option<String>,
    pub meta_title: Option<String>,
    pub meta_description: Option<String>,
    pub indexable: bool,
    pub canonical_url: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

/// Create product request. Price is integer cents.
#[derive(Debug, Deserialize, Validate)]
pub struct CreateProduct {
    #[validate(length(min = 1, max = 255, message = "Name must be 1-255 characters"))]
    pub name: String,
    #[validate(length(
        min = 1,
        max = 20,
        message = "Type must be course, indicator, or bundle"
    ))]
    pub product_type: String,
    pub description: Option<String>,
    pub long_description: Option<String>,
    #[validate(range(min = 0, message = "Price must be non-negative"))]
    pub price_cents: i64,
    pub is_active: Option<bool>,
    pub metadata: Option<serde_json::Value>,
    pub thumbnail: Option<String>,
    pub meta_title: Option<String>,
    pub meta_description: Option<String>,
    pub indexable: Option<bool>,
    pub canonical_url: Option<String>,
}

/// Update product request. Price is integer cents.
#[derive(Debug, Deserialize, Validate)]
pub struct UpdateProduct {
    #[validate(length(min = 1, max = 255, message = "Name must be 1-255 characters"))]
    pub name: Option<String>,
    pub product_type: Option<String>,
    pub description: Option<String>,
    pub long_description: Option<String>,
    pub price_cents: Option<i64>,
    pub is_active: Option<bool>,
    pub metadata: Option<serde_json::Value>,
    pub thumbnail: Option<String>,
    pub meta_title: Option<String>,
    pub meta_description: Option<String>,
    pub indexable: Option<bool>,
    pub canonical_url: Option<String>,
}

/// User product ownership record
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct UserProduct {
    pub id: i64,
    pub user_id: i64,
    pub product_id: i64,
    pub purchased_at: NaiveDateTime,
    pub order_id: Option<i64>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

/// User product with product details (joined query)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserProductWithDetails {
    pub id: i64,
    pub user_id: i64,
    pub product_id: i64,
    pub purchased_at: NaiveDateTime,
    pub product: ProductResponse,
}
