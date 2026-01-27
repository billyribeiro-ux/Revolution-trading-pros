//! CMS Models - Apple ICT 7+ Principal Engineer Grade
//!
//! Comprehensive data models for the custom CMS implementation.
//! All models are designed for:
//! - Type safety with explicit types (no Any)
//! - SQLx integration with FromRow derive
//! - Serde serialization for API responses
//! - Validation-ready structures
//!
//! @version 2.0.0 - January 2026

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use sqlx::FromRow;
use uuid::Uuid;

// ═══════════════════════════════════════════════════════════════════════════════════════
// ENUMS
// ═══════════════════════════════════════════════════════════════════════════════════════

/// All supported content types in the CMS
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "cms_content_type", rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum CmsContentType {
    Page,
    BlogPost,
    AlertService,
    TradingRoom,
    Indicator,
    Course,
    Lesson,
    Testimonial,
    Faq,
    Author,
    TopicCluster,
    WeeklyWatchlist,
    Resource,
    NavigationMenu,
    SiteSettings,
    Redirect,
}

impl std::fmt::Display for CmsContentType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CmsContentType::Page => write!(f, "page"),
            CmsContentType::BlogPost => write!(f, "blog_post"),
            CmsContentType::AlertService => write!(f, "alert_service"),
            CmsContentType::TradingRoom => write!(f, "trading_room"),
            CmsContentType::Indicator => write!(f, "indicator"),
            CmsContentType::Course => write!(f, "course"),
            CmsContentType::Lesson => write!(f, "lesson"),
            CmsContentType::Testimonial => write!(f, "testimonial"),
            CmsContentType::Faq => write!(f, "faq"),
            CmsContentType::Author => write!(f, "author"),
            CmsContentType::TopicCluster => write!(f, "topic_cluster"),
            CmsContentType::WeeklyWatchlist => write!(f, "weekly_watchlist"),
            CmsContentType::Resource => write!(f, "resource"),
            CmsContentType::NavigationMenu => write!(f, "navigation_menu"),
            CmsContentType::SiteSettings => write!(f, "site_settings"),
            CmsContentType::Redirect => write!(f, "redirect"),
        }
    }
}

/// CMS user roles with specific permissions
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "cms_user_role", rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum CmsUserRole {
    SuperAdmin,
    MarketingManager,
    ContentEditor,
    WeeklyEditor,
    Developer,
    Viewer,
}

impl CmsUserRole {
    /// Check if role can create content
    pub fn can_create(&self) -> bool {
        matches!(
            self,
            CmsUserRole::SuperAdmin
                | CmsUserRole::MarketingManager
                | CmsUserRole::ContentEditor
                | CmsUserRole::WeeklyEditor
        )
    }

    /// Check if role can edit all content
    pub fn can_edit_all(&self) -> bool {
        matches!(
            self,
            CmsUserRole::SuperAdmin | CmsUserRole::MarketingManager
        )
    }

    /// Check if role can approve content
    pub fn can_approve(&self) -> bool {
        matches!(
            self,
            CmsUserRole::SuperAdmin | CmsUserRole::MarketingManager
        )
    }

    /// Check if role can publish content
    pub fn can_publish(&self) -> bool {
        matches!(
            self,
            CmsUserRole::SuperAdmin | CmsUserRole::MarketingManager
        )
    }

    /// Check if role can delete content
    pub fn can_delete(&self) -> bool {
        matches!(self, CmsUserRole::SuperAdmin)
    }
}

/// Content workflow status
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "cms_content_status", rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum CmsContentStatus {
    Draft,
    InReview,
    Approved,
    Scheduled,
    Published,
    Archived,
}

impl std::fmt::Display for CmsContentStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CmsContentStatus::Draft => write!(f, "draft"),
            CmsContentStatus::InReview => write!(f, "in_review"),
            CmsContentStatus::Approved => write!(f, "approved"),
            CmsContentStatus::Scheduled => write!(f, "scheduled"),
            CmsContentStatus::Published => write!(f, "published"),
            CmsContentStatus::Archived => write!(f, "archived"),
        }
    }
}

/// All supported block types for the page builder
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "cms_block_type")]
#[serde(rename_all = "kebab-case")]
pub enum CmsBlockType {
    #[sqlx(rename = "hero-slider")]
    HeroSlider,
    #[sqlx(rename = "trading-rooms-grid")]
    TradingRoomsGrid,
    #[sqlx(rename = "alert-services-grid")]
    AlertServicesGrid,
    #[sqlx(rename = "testimonials-masonry")]
    TestimonialsMasonry,
    #[sqlx(rename = "features-grid")]
    FeaturesGrid,
    #[sqlx(rename = "rich-text")]
    RichText,
    #[sqlx(rename = "image")]
    Image,
    #[sqlx(rename = "video-embed")]
    VideoEmbed,
    #[sqlx(rename = "spacer")]
    Spacer,
    #[sqlx(rename = "email-capture")]
    EmailCapture,
    #[sqlx(rename = "blog-feed")]
    BlogFeed,
    #[sqlx(rename = "indicators-showcase")]
    IndicatorsShowcase,
    #[sqlx(rename = "courses-grid")]
    CoursesGrid,
    #[sqlx(rename = "faq-accordion")]
    FaqAccordion,
    #[sqlx(rename = "pricing-table")]
    PricingTable,
    #[sqlx(rename = "cta-banner")]
    CtaBanner,
    #[sqlx(rename = "stats-counter")]
    StatsCounter,
    #[sqlx(rename = "two-column-layout")]
    TwoColumnLayout,
}

// ═══════════════════════════════════════════════════════════════════════════════════════
// CMS USER
// ═══════════════════════════════════════════════════════════════════════════════════════

/// CMS user profile with roles and permissions
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct CmsUser {
    pub id: Uuid,
    pub user_id: Option<i64>,
    pub display_name: String,
    pub avatar_url: Option<String>,
    pub bio: Option<String>,
    pub role: CmsUserRole,
    pub permissions: Option<JsonValue>,
    pub allowed_content_types: Option<Vec<CmsContentType>>,
    pub preferences: Option<JsonValue>,
    pub notification_settings: Option<JsonValue>,
    pub is_active: bool,
    pub last_login_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub created_by: Option<Uuid>,
    pub updated_by: Option<Uuid>,
}

/// Request to create a CMS user
#[derive(Debug, Deserialize)]
pub struct CreateCmsUserRequest {
    pub user_id: Option<i64>,
    pub display_name: String,
    pub avatar_url: Option<String>,
    pub bio: Option<String>,
    pub role: CmsUserRole,
    pub allowed_content_types: Option<Vec<CmsContentType>>,
}

/// Request to update a CMS user
#[derive(Debug, Deserialize)]
pub struct UpdateCmsUserRequest {
    pub display_name: Option<String>,
    pub avatar_url: Option<String>,
    pub bio: Option<String>,
    pub role: Option<CmsUserRole>,
    pub allowed_content_types: Option<Vec<CmsContentType>>,
    pub is_active: Option<bool>,
}

// ═══════════════════════════════════════════════════════════════════════════════════════
// ASSET FOLDERS
// ═══════════════════════════════════════════════════════════════════════════════════════

/// Asset folder for organizing digital assets
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct CmsAssetFolder {
    pub id: Uuid,
    pub name: String,
    pub slug: String,
    pub parent_id: Option<Uuid>,
    pub path: String,
    pub depth: i32,
    pub description: Option<String>,
    pub color: Option<String>,
    pub icon: Option<String>,
    pub is_public: bool,
    pub allowed_roles: Option<Vec<CmsUserRole>>,
    pub sort_order: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub created_by: Option<Uuid>,
    pub updated_by: Option<Uuid>,
}

/// Request to create an asset folder
#[derive(Debug, Deserialize)]
pub struct CreateAssetFolderRequest {
    pub name: String,
    pub parent_id: Option<Uuid>,
    pub description: Option<String>,
    pub color: Option<String>,
    pub icon: Option<String>,
    pub is_public: Option<bool>,
}

/// Request to update an asset folder
#[derive(Debug, Deserialize)]
pub struct UpdateAssetFolderRequest {
    pub name: Option<String>,
    pub parent_id: Option<Uuid>,
    pub description: Option<String>,
    pub color: Option<String>,
    pub icon: Option<String>,
    pub is_public: Option<bool>,
    pub sort_order: Option<i32>,
}

// ═══════════════════════════════════════════════════════════════════════════════════════
// ASSETS
// ═══════════════════════════════════════════════════════════════════════════════════════

/// Digital asset (image, video, document)
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct CmsAsset {
    pub id: Uuid,
    pub folder_id: Option<Uuid>,
    pub filename: String,
    pub original_filename: String,
    pub mime_type: String,
    pub file_size: i64,
    pub file_extension: String,
    pub storage_provider: String,
    pub storage_key: String,
    pub cdn_url: String,
    pub width: Option<i32>,
    pub height: Option<i32>,
    pub aspect_ratio: Option<rust_decimal::Decimal>,
    pub blurhash: Option<String>,
    pub dominant_color: Option<String>,
    pub duration_seconds: Option<i32>,
    pub video_codec: Option<String>,
    pub audio_codec: Option<String>,
    pub bunny_video_id: Option<String>,
    pub bunny_library_id: Option<i32>,
    pub thumbnail_url: Option<String>,
    pub variants: Option<JsonValue>,
    pub title: Option<String>,
    pub alt_text: Option<String>,
    pub caption: Option<String>,
    pub description: Option<String>,
    pub credits: Option<String>,
    pub seo_title: Option<String>,
    pub seo_description: Option<String>,
    pub tags: Option<Vec<String>>,
    pub usage_count: i32,
    pub last_used_at: Option<DateTime<Utc>>,
    pub deleted_at: Option<DateTime<Utc>>,
    pub version: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub created_by: Option<Uuid>,
    pub updated_by: Option<Uuid>,
}

/// Lightweight asset for listings
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct CmsAssetSummary {
    pub id: Uuid,
    pub folder_id: Option<Uuid>,
    pub filename: String,
    pub mime_type: String,
    pub file_size: i64,
    pub cdn_url: String,
    pub width: Option<i32>,
    pub height: Option<i32>,
    pub blurhash: Option<String>,
    pub thumbnail_url: Option<String>,
    pub title: Option<String>,
    pub alt_text: Option<String>,
    pub usage_count: i32,
    pub created_at: DateTime<Utc>,
}

/// Request to create an asset
#[derive(Debug, Deserialize)]
pub struct CreateAssetRequest {
    pub folder_id: Option<Uuid>,
    pub filename: String,
    pub original_filename: String,
    pub mime_type: String,
    pub file_size: i64,
    pub file_extension: String,
    pub storage_key: String,
    pub cdn_url: String,
    pub width: Option<i32>,
    pub height: Option<i32>,
    pub blurhash: Option<String>,
    pub dominant_color: Option<String>,
    pub title: Option<String>,
    pub alt_text: Option<String>,
    pub tags: Option<Vec<String>>,
}

/// Request to update an asset
#[derive(Debug, Deserialize)]
pub struct UpdateAssetRequest {
    pub folder_id: Option<Uuid>,
    pub title: Option<String>,
    pub alt_text: Option<String>,
    pub caption: Option<String>,
    pub description: Option<String>,
    pub credits: Option<String>,
    pub seo_title: Option<String>,
    pub seo_description: Option<String>,
    pub tags: Option<Vec<String>>,
}

/// Image variant information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImageVariant {
    pub width: i32,
    pub height: i32,
    pub url: String,
    pub format: String,
    pub size: i64,
}

// ═══════════════════════════════════════════════════════════════════════════════════════
// CONTENT
// ═══════════════════════════════════════════════════════════════════════════════════════

/// CMS content item (unified storage for all content types)
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct CmsContent {
    pub id: Uuid,
    pub content_type: CmsContentType,
    pub slug: String,
    pub locale: String,
    pub is_primary_locale: bool,
    pub parent_content_id: Option<Uuid>,
    pub title: String,
    pub subtitle: Option<String>,
    pub excerpt: Option<String>,
    pub content: Option<String>,
    pub content_blocks: Option<JsonValue>,
    pub featured_image_id: Option<Uuid>,
    pub og_image_id: Option<Uuid>,
    pub gallery_ids: Option<Vec<Uuid>>,
    pub meta_title: Option<String>,
    pub meta_description: Option<String>,
    pub meta_keywords: Option<Vec<String>>,
    pub canonical_url: Option<String>,
    pub robots_directives: Option<String>,
    pub structured_data: Option<JsonValue>,
    pub author_id: Option<Uuid>,
    pub contributors: Option<Vec<Uuid>>,
    pub status: CmsContentStatus,
    pub published_at: Option<DateTime<Utc>>,
    pub scheduled_publish_at: Option<DateTime<Utc>>,
    pub scheduled_unpublish_at: Option<DateTime<Utc>>,
    pub primary_category_id: Option<Uuid>,
    pub categories: Option<Vec<Uuid>>,
    pub custom_fields: Option<JsonValue>,
    pub template: Option<String>,
    pub deleted_at: Option<DateTime<Utc>>,
    pub version: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub created_by: Option<Uuid>,
    pub updated_by: Option<Uuid>,
}

/// Lightweight content for listings
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct CmsContentSummary {
    pub id: Uuid,
    pub content_type: CmsContentType,
    pub slug: String,
    pub locale: String,
    pub title: String,
    pub excerpt: Option<String>,
    pub featured_image_id: Option<Uuid>,
    pub author_id: Option<Uuid>,
    pub status: CmsContentStatus,
    pub published_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Request to create content
#[derive(Debug, Deserialize)]
pub struct CreateContentRequest {
    pub content_type: CmsContentType,
    pub slug: String,
    pub locale: Option<String>,
    pub title: String,
    pub subtitle: Option<String>,
    pub excerpt: Option<String>,
    pub content: Option<String>,
    pub content_blocks: Option<JsonValue>,
    pub featured_image_id: Option<Uuid>,
    pub og_image_id: Option<Uuid>,
    pub meta_title: Option<String>,
    pub meta_description: Option<String>,
    pub meta_keywords: Option<Vec<String>>,
    pub canonical_url: Option<String>,
    pub author_id: Option<Uuid>,
    pub custom_fields: Option<JsonValue>,
    pub template: Option<String>,
}

/// Request to update content
#[derive(Debug, Deserialize)]
pub struct UpdateContentRequest {
    pub slug: Option<String>,
    pub title: Option<String>,
    pub subtitle: Option<String>,
    pub excerpt: Option<String>,
    pub content: Option<String>,
    pub content_blocks: Option<JsonValue>,
    pub featured_image_id: Option<Uuid>,
    pub og_image_id: Option<Uuid>,
    pub gallery_ids: Option<Vec<Uuid>>,
    pub meta_title: Option<String>,
    pub meta_description: Option<String>,
    pub meta_keywords: Option<Vec<String>>,
    pub canonical_url: Option<String>,
    pub robots_directives: Option<String>,
    pub structured_data: Option<JsonValue>,
    pub author_id: Option<Uuid>,
    pub contributors: Option<Vec<Uuid>>,
    pub primary_category_id: Option<Uuid>,
    pub categories: Option<Vec<Uuid>>,
    pub custom_fields: Option<JsonValue>,
    pub template: Option<String>,
}

/// Request to transition content status
#[derive(Debug, Deserialize)]
pub struct TransitionStatusRequest {
    pub status: CmsContentStatus,
    pub comment: Option<String>,
    pub scheduled_at: Option<DateTime<Utc>>,
}

// ═══════════════════════════════════════════════════════════════════════════════════════
// CONTENT BLOCKS
// ═══════════════════════════════════════════════════════════════════════════════════════

/// Content block structure for page builder
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContentBlock {
    pub id: String,
    pub block_type: String,
    pub order: i32,
    pub data: JsonValue,
    pub settings: Option<BlockSettings>,
}

/// Block settings (visibility, spacing, etc.)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockSettings {
    pub is_hidden: Option<bool>,
    pub css_class: Option<String>,
    pub css_id: Option<String>,
    pub margin_top: Option<String>,
    pub margin_bottom: Option<String>,
    pub padding_top: Option<String>,
    pub padding_bottom: Option<String>,
    pub background_color: Option<String>,
    pub background_image_id: Option<Uuid>,
}

// ═══════════════════════════════════════════════════════════════════════════════════════
// REVISIONS
// ═══════════════════════════════════════════════════════════════════════════════════════

/// Content revision for version history
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct CmsRevision {
    pub id: Uuid,
    pub content_id: Uuid,
    pub revision_number: i32,
    pub is_current: bool,
    pub data: JsonValue,
    pub change_summary: Option<String>,
    pub changed_fields: Option<Vec<String>>,
    pub created_at: DateTime<Utc>,
    pub created_by: Option<Uuid>,
}

// ═══════════════════════════════════════════════════════════════════════════════════════
// TAGS
// ═══════════════════════════════════════════════════════════════════════════════════════

/// Taxonomy tag
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct CmsTag {
    pub id: Uuid,
    pub name: String,
    pub slug: String,
    pub parent_id: Option<Uuid>,
    pub description: Option<String>,
    pub color: Option<String>,
    pub icon: Option<String>,
    pub meta_title: Option<String>,
    pub meta_description: Option<String>,
    pub usage_count: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub created_by: Option<Uuid>,
}

/// Request to create a tag
#[derive(Debug, Deserialize)]
pub struct CreateTagRequest {
    pub name: String,
    pub parent_id: Option<Uuid>,
    pub description: Option<String>,
    pub color: Option<String>,
    pub icon: Option<String>,
}

// ═══════════════════════════════════════════════════════════════════════════════════════
// CONTENT RELATIONS
// ═══════════════════════════════════════════════════════════════════════════════════════

/// Relation between content items
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct CmsContentRelation {
    pub id: Uuid,
    pub source_id: Uuid,
    pub target_id: Uuid,
    pub relation_type: String,
    pub metadata: Option<JsonValue>,
    pub sort_order: i32,
    pub created_at: DateTime<Utc>,
    pub created_by: Option<Uuid>,
}

/// Request to create a content relation
#[derive(Debug, Deserialize)]
pub struct CreateRelationRequest {
    pub target_id: Uuid,
    pub relation_type: String,
    pub metadata: Option<JsonValue>,
}

// ═══════════════════════════════════════════════════════════════════════════════════════
// COMMENTS
// ═══════════════════════════════════════════════════════════════════════════════════════

/// Editorial comment on content
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct CmsComment {
    pub id: Uuid,
    pub content_id: Uuid,
    pub parent_id: Option<Uuid>,
    pub thread_id: Option<Uuid>,
    pub body: String,
    pub block_id: Option<String>,
    pub selection_start: Option<i32>,
    pub selection_end: Option<i32>,
    pub is_resolved: bool,
    pub resolved_by: Option<Uuid>,
    pub resolved_at: Option<DateTime<Utc>>,
    pub mentioned_users: Option<Vec<Uuid>>,
    pub deleted_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub created_by: Option<Uuid>,
}

/// Request to create a comment
#[derive(Debug, Deserialize)]
pub struct CreateCommentRequest {
    pub parent_id: Option<Uuid>,
    pub body: String,
    pub block_id: Option<String>,
    pub selection_start: Option<i32>,
    pub selection_end: Option<i32>,
    pub mentioned_users: Option<Vec<Uuid>>,
}

// ═══════════════════════════════════════════════════════════════════════════════════════
// SCHEDULED JOBS
// ═══════════════════════════════════════════════════════════════════════════════════════

/// Scheduled CMS job
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct CmsScheduledJob {
    pub id: Uuid,
    pub job_type: String,
    pub content_id: Option<Uuid>,
    pub scheduled_at: DateTime<Utc>,
    pub timezone: Option<String>,
    pub payload: Option<JsonValue>,
    pub status: String,
    pub attempts: i32,
    pub max_attempts: i32,
    pub started_at: Option<DateTime<Utc>>,
    pub completed_at: Option<DateTime<Utc>>,
    pub error_message: Option<String>,
    pub result: Option<JsonValue>,
    pub locked_by: Option<String>,
    pub locked_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub created_by: Option<Uuid>,
}

// ═══════════════════════════════════════════════════════════════════════════════════════
// WEBHOOKS
// ═══════════════════════════════════════════════════════════════════════════════════════

/// CMS webhook configuration
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct CmsWebhook {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub url: String,
    pub secret: Option<String>,
    pub events: Vec<String>,
    pub content_types: Option<Vec<CmsContentType>>,
    pub is_active: bool,
    pub retry_count: i32,
    pub timeout_seconds: i32,
    pub headers: Option<JsonValue>,
    pub last_triggered_at: Option<DateTime<Utc>>,
    pub success_count: i32,
    pub failure_count: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub created_by: Option<Uuid>,
}

/// Request to create a webhook
#[derive(Debug, Deserialize)]
pub struct CreateWebhookRequest {
    pub name: String,
    pub description: Option<String>,
    pub url: String,
    pub secret: Option<String>,
    pub events: Vec<String>,
    pub content_types: Option<Vec<CmsContentType>>,
    pub headers: Option<JsonValue>,
    pub retry_count: Option<i32>,
    pub timeout_seconds: Option<i32>,
}

/// Webhook delivery record
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct CmsWebhookDelivery {
    pub id: Uuid,
    pub webhook_id: Uuid,
    pub event_type: String,
    pub content_id: Option<Uuid>,
    pub payload: JsonValue,
    pub status: String,
    pub attempts: i32,
    pub response_status: Option<i32>,
    pub response_body: Option<String>,
    pub response_time_ms: Option<i32>,
    pub error_message: Option<String>,
    pub created_at: DateTime<Utc>,
    pub delivered_at: Option<DateTime<Utc>>,
    pub next_retry_at: Option<DateTime<Utc>>,
}

// ═══════════════════════════════════════════════════════════════════════════════════════
// SITE SETTINGS
// ═══════════════════════════════════════════════════════════════════════════════════════

/// Global site settings (singleton)
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct CmsSiteSettings {
    pub id: Uuid,
    pub site_name: String,
    pub site_tagline: Option<String>,
    pub site_description: Option<String>,
    pub logo_light_id: Option<Uuid>,
    pub logo_dark_id: Option<Uuid>,
    pub favicon_id: Option<Uuid>,
    pub og_default_image_id: Option<Uuid>,
    pub contact_email: Option<String>,
    pub support_email: Option<String>,
    pub phone: Option<String>,
    pub address: Option<String>,
    pub social_links: Option<JsonValue>,
    pub default_meta_title_suffix: Option<String>,
    pub default_robots: Option<String>,
    pub google_analytics_id: Option<String>,
    pub google_tag_manager_id: Option<String>,
    pub maintenance_mode: bool,
    pub maintenance_message: Option<String>,
    pub head_scripts: Option<String>,
    pub body_start_scripts: Option<String>,
    pub body_end_scripts: Option<String>,
    pub custom_css: Option<String>,
    pub settings: Option<JsonValue>,
    pub updated_at: DateTime<Utc>,
    pub updated_by: Option<Uuid>,
}

/// Request to update site settings
#[derive(Debug, Deserialize)]
pub struct UpdateSiteSettingsRequest {
    pub site_name: Option<String>,
    pub site_tagline: Option<String>,
    pub site_description: Option<String>,
    pub logo_light_id: Option<Uuid>,
    pub logo_dark_id: Option<Uuid>,
    pub favicon_id: Option<Uuid>,
    pub og_default_image_id: Option<Uuid>,
    pub contact_email: Option<String>,
    pub support_email: Option<String>,
    pub phone: Option<String>,
    pub address: Option<String>,
    pub social_links: Option<JsonValue>,
    pub default_meta_title_suffix: Option<String>,
    pub default_robots: Option<String>,
    pub google_analytics_id: Option<String>,
    pub google_tag_manager_id: Option<String>,
    pub maintenance_mode: Option<bool>,
    pub maintenance_message: Option<String>,
    pub head_scripts: Option<String>,
    pub body_start_scripts: Option<String>,
    pub body_end_scripts: Option<String>,
    pub custom_css: Option<String>,
    pub settings: Option<JsonValue>,
}

// ═══════════════════════════════════════════════════════════════════════════════════════
// NAVIGATION MENUS
// ═══════════════════════════════════════════════════════════════════════════════════════

/// Navigation menu
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct CmsNavigationMenu {
    pub id: Uuid,
    pub name: String,
    pub slug: String,
    pub location: Option<String>,
    pub items: JsonValue,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub created_by: Option<Uuid>,
    pub updated_by: Option<Uuid>,
}

/// Navigation menu item (recursive structure in items JSON)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NavigationMenuItem {
    pub id: String,
    pub label: String,
    pub url: Option<String>,
    pub content_id: Option<Uuid>,
    pub target: Option<String>,
    pub icon: Option<String>,
    pub css_class: Option<String>,
    pub is_visible: bool,
    pub children: Vec<NavigationMenuItem>,
}

/// Request to create a navigation menu
#[derive(Debug, Deserialize)]
pub struct CreateNavigationMenuRequest {
    pub name: String,
    pub location: Option<String>,
    pub items: JsonValue,
}

/// Request to update a navigation menu
#[derive(Debug, Deserialize)]
pub struct UpdateNavigationMenuRequest {
    pub name: Option<String>,
    pub location: Option<String>,
    pub items: Option<JsonValue>,
    pub is_active: Option<bool>,
}

// ═══════════════════════════════════════════════════════════════════════════════════════
// REDIRECTS
// ═══════════════════════════════════════════════════════════════════════════════════════

/// URL redirect rule
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct CmsRedirect {
    pub id: Uuid,
    pub source_path: String,
    pub target_path: String,
    pub status_code: i32,
    pub is_regex: bool,
    pub preserve_query_string: bool,
    pub hit_count: i32,
    pub last_hit_at: Option<DateTime<Utc>>,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub created_by: Option<Uuid>,
}

/// Request to create a redirect
#[derive(Debug, Deserialize)]
pub struct CreateRedirectRequest {
    pub source_path: String,
    pub target_path: String,
    pub status_code: Option<i32>,
    pub is_regex: Option<bool>,
    pub preserve_query_string: Option<bool>,
}

// ═══════════════════════════════════════════════════════════════════════════════════════
// AUDIT LOG
// ═══════════════════════════════════════════════════════════════════════════════════════

/// CMS audit log entry
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct CmsAuditLogEntry {
    pub id: Uuid,
    pub action: String,
    pub entity_type: String,
    pub entity_id: Option<Uuid>,
    pub user_id: Option<Uuid>,
    pub user_email: Option<String>,
    pub user_role: Option<CmsUserRole>,
    pub ip_address: Option<String>,
    pub user_agent: Option<String>,
    pub request_method: Option<String>,
    pub request_path: Option<String>,
    pub old_values: Option<JsonValue>,
    pub new_values: Option<JsonValue>,
    pub metadata: Option<JsonValue>,
    pub status: String,
    pub error_message: Option<String>,
    pub created_at: DateTime<Utc>,
}

// ═══════════════════════════════════════════════════════════════════════════════════════
// QUERY PARAMETERS
// ═══════════════════════════════════════════════════════════════════════════════════════

/// Query parameters for content listing
#[derive(Debug, Deserialize, Default)]
pub struct ContentListQuery {
    pub content_type: Option<CmsContentType>,
    pub status: Option<CmsContentStatus>,
    pub author_id: Option<Uuid>,
    pub locale: Option<String>,
    pub search: Option<String>,
    pub tag_id: Option<Uuid>,
    pub category_id: Option<Uuid>,
    pub published_before: Option<DateTime<Utc>>,
    pub published_after: Option<DateTime<Utc>>,
    pub sort_by: Option<String>,
    pub sort_order: Option<String>,
    pub limit: Option<i64>,
    pub offset: Option<i64>,
    pub include_deleted: Option<bool>,
}

/// Query parameters for asset listing
#[derive(Debug, Deserialize, Default)]
pub struct AssetListQuery {
    pub folder_id: Option<Uuid>,
    pub mime_type: Option<String>,
    pub search: Option<String>,
    pub tags: Option<Vec<String>>,
    pub sort_by: Option<String>,
    pub sort_order: Option<String>,
    pub limit: Option<i64>,
    pub offset: Option<i64>,
    pub include_deleted: Option<bool>,
}

/// Paginated response wrapper
#[derive(Debug, Serialize)]
pub struct PaginatedResponse<T> {
    pub data: Vec<T>,
    pub meta: PaginationMeta,
}

/// Pagination metadata
#[derive(Debug, Serialize)]
pub struct PaginationMeta {
    pub total: i64,
    pub limit: i64,
    pub offset: i64,
    pub has_more: bool,
}
