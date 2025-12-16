//! Product model and related types (Courses, Indicators, Memberships)

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::error::ApiError;
use crate::utils::{deserialize_i64_from_string, deserialize_option_i64_from_string};

/// Product entity (courses, indicators, memberships)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Product {
    pub id: Uuid,
    pub name: String,
    pub slug: String,
    pub description: Option<String>,
    pub short_description: Option<String>,
    pub product_type: ProductType,
    #[serde(deserialize_with = "deserialize_i64_from_string")]
    pub price: i64,  // cents
    #[serde(default, deserialize_with = "deserialize_option_i64_from_string")]
    pub compare_price: Option<i64>,
    pub currency: String,
    pub image_url: Option<String>,
    pub gallery: Option<Vec<String>>,
    pub features: Option<Vec<String>>,
    pub is_active: bool,
    pub is_featured: bool,
    pub stock: Option<i32>,
    pub max_purchases: Option<i32>,
    pub download_url: Option<String>,
    pub download_limit: Option<i32>,
    pub meta_title: Option<String>,
    pub meta_description: Option<String>,
    pub sort_order: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProductType {
    Course,
    Indicator,
    Membership,
    TradingRoom,
    AlertService,
    Bundle,
    Digital,
}

impl Default for ProductType {
    fn default() -> Self {
        Self::Digital
    }
}

/// Indicator (trading indicator product)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Indicator {
    pub id: Uuid,
    pub product_id: Uuid,
    pub name: String,
    pub slug: String,
    pub description: Option<String>,
    pub short_description: Option<String>,
    pub platform: IndicatorPlatform,
    pub version: String,
    pub features: Vec<String>,
    pub screenshots: Vec<String>,
    pub video_url: Option<String>,
    pub documentation_url: Option<String>,
    pub download_url: Option<String>,
    #[serde(deserialize_with = "deserialize_i64_from_string")]
    pub price: i64,
    #[serde(default, deserialize_with = "deserialize_option_i64_from_string")]
    pub compare_price: Option<i64>,
    pub is_active: bool,
    pub is_featured: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum IndicatorPlatform {
    TradingView,
    ThinkOrSwim,
    NinjaTrader,
    MetaTrader4,
    MetaTrader5,
    TradeStation,
    MultiPlatform,
}

/// Course entity
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Course {
    pub id: Uuid,
    pub product_id: Uuid,
    pub title: String,
    pub slug: String,
    pub description: Option<String>,
    pub short_description: Option<String>,
    pub instructor_id: Uuid,
    pub thumbnail_url: Option<String>,
    pub preview_video_url: Option<String>,
    pub level: CourseLevel,
    pub duration_hours: Option<f32>,
    pub lesson_count: i32,
    pub student_count: i64,
    pub rating: Option<f32>,
    pub review_count: i64,
    pub syllabus: Option<serde_json::Value>,
    pub requirements: Option<Vec<String>>,
    pub outcomes: Option<Vec<String>>,
    pub is_active: bool,
    pub is_featured: bool,
    pub price: i64,
    pub compare_price: Option<i64>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CourseLevel {
    Beginner,
    Intermediate,
    Advanced,
    AllLevels,
}

/// Course module/section
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CourseModule {
    pub id: Uuid,
    pub course_id: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub sort_order: i32,
    pub lesson_count: i32,
    pub duration_minutes: Option<i32>,
    pub created_at: DateTime<Utc>,
}

/// Course lesson
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CourseLesson {
    pub id: Uuid,
    pub module_id: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub content_type: LessonContentType,
    pub video_url: Option<String>,
    pub video_duration: Option<i32>,
    pub content: Option<String>,
    pub attachments: Option<Vec<String>>,
    pub is_preview: bool,
    pub sort_order: i32,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LessonContentType {
    Video,
    Text,
    Quiz,
    Assignment,
    Download,
}

/// User's course progress
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CourseProgress {
    pub id: Uuid,
    pub user_id: Uuid,
    pub course_id: Uuid,
    pub completed_lessons: Vec<Uuid>,
    pub current_lesson_id: Option<Uuid>,
    pub progress_percent: f32,
    pub started_at: DateTime<Utc>,
    pub completed_at: Option<DateTime<Utc>>,
    pub last_accessed_at: DateTime<Utc>,
}

/// Create product request
#[derive(Debug, Deserialize)]
pub struct CreateProductRequest {
    pub name: String,
    pub slug: Option<String>,
    pub description: Option<String>,
    pub short_description: Option<String>,
    pub product_type: ProductType,
    pub price: i64,
    pub compare_price: Option<i64>,
    pub currency: Option<String>,
    pub image_url: Option<String>,
    pub gallery: Option<Vec<String>>,
    pub features: Option<Vec<String>>,
    pub is_active: Option<bool>,
    pub is_featured: Option<bool>,
    pub stock: Option<i32>,
    pub download_url: Option<String>,
    pub meta_title: Option<String>,
    pub meta_description: Option<String>,
}

impl CreateProductRequest {
    pub fn validate(&self) -> Result<(), ApiError> {
        if self.name.trim().is_empty() {
            return Err(ApiError::Validation("Name is required".to_string()));
        }
        if self.price < 0 {
            return Err(ApiError::Validation("Price must be positive".to_string()));
        }
        Ok(())
    }
}

/// Update product request
#[derive(Debug, Deserialize)]
pub struct UpdateProductRequest {
    pub name: Option<String>,
    pub slug: Option<String>,
    pub description: Option<String>,
    pub short_description: Option<String>,
    pub price: Option<i64>,
    pub compare_price: Option<i64>,
    pub image_url: Option<String>,
    pub gallery: Option<Vec<String>>,
    pub features: Option<Vec<String>>,
    pub is_active: Option<bool>,
    pub is_featured: Option<bool>,
    pub stock: Option<i32>,
    pub download_url: Option<String>,
    pub meta_title: Option<String>,
    pub meta_description: Option<String>,
}

/// Product list query
#[derive(Debug, Deserialize)]
pub struct ProductListQuery {
    pub page: Option<i64>,
    pub per_page: Option<i64>,
    pub product_type: Option<ProductType>,
    pub is_active: Option<bool>,
    pub is_featured: Option<bool>,
    pub search: Option<String>,
    pub min_price: Option<i64>,
    pub max_price: Option<i64>,
    pub sort_by: Option<String>,
    pub sort_order: Option<String>,
}

/// Coupon entity
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Coupon {
    pub id: Uuid,
    pub code: String,
    pub description: Option<String>,
    pub discount_type: DiscountType,
    pub discount_value: i64,
    pub min_purchase: Option<i64>,
    pub max_discount: Option<i64>,
    pub usage_limit: Option<i32>,
    pub usage_count: i32,
    pub per_user_limit: Option<i32>,
    pub product_ids: Option<Vec<Uuid>>,
    pub excluded_product_ids: Option<Vec<Uuid>>,
    pub starts_at: Option<DateTime<Utc>>,
    pub expires_at: Option<DateTime<Utc>>,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DiscountType {
    Percentage,
    FixedAmount,
}

/// Cart item
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CartItem {
    pub product_id: Uuid,
    pub quantity: i32,
    pub price: i64,
    pub name: String,
    pub image_url: Option<String>,
}

/// Checkout request
#[derive(Debug, Deserialize)]
pub struct CheckoutRequest {
    pub items: Vec<CartItem>,
    pub coupon_code: Option<String>,
    pub payment_method_id: Option<String>,
    pub billing_address: Option<BillingAddress>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BillingAddress {
    pub name: String,
    pub email: String,
    pub line1: String,
    pub line2: Option<String>,
    pub city: String,
    pub state: Option<String>,
    pub postal_code: String,
    pub country: String,
}

/// Order entity
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Order {
    pub id: Uuid,
    pub user_id: Uuid,
    pub order_number: String,
    pub status: OrderStatus,
    pub subtotal: i64,
    pub discount: i64,
    pub tax: i64,
    pub total: i64,
    pub currency: String,
    pub coupon_id: Option<Uuid>,
    pub coupon_code: Option<String>,
    pub payment_intent_id: Option<String>,
    pub payment_method: Option<String>,
    pub billing_address: Option<serde_json::Value>,
    pub notes: Option<String>,
    pub completed_at: Option<DateTime<Utc>>,
    pub refunded_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OrderStatus {
    Pending,
    Processing,
    Completed,
    Failed,
    Refunded,
    Cancelled,
}

/// Order item
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderItem {
    pub id: Uuid,
    pub order_id: Uuid,
    pub product_id: Uuid,
    pub product_name: String,
    pub product_type: ProductType,
    pub quantity: i32,
    pub unit_price: i64,
    pub total_price: i64,
    pub created_at: DateTime<Utc>,
}
