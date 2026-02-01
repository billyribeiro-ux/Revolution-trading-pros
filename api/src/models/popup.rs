//! Popup Model - Revolution Trading Pros
//! Apple ICT 7+ Principal Engineer Grade - February 2026
//!
//! Complete popup/modal system model supporting:
//! - Multiple trigger types (time delay, scroll, exit intent, click, inactivity)
//! - Frequency controls (once, session, daily, weekly, always)
//! - Device targeting (desktop, tablet, mobile)
//! - Page targeting with wildcards
//! - A/B testing support
//! - Scheduling (start/end dates)
//! - Analytics tracking (impressions, conversions, CVR)
//! - Design customization (colors, fonts, overlay, custom CSS)
//! - Form integration
//! - Template system
//! - Video/image embed support
//! - Secondary CTA buttons

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

/// Main Popup entity
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Popup {
    pub id: i32,
    pub name: String,
    #[serde(rename = "type")]
    #[sqlx(rename = "type")]
    pub popup_type: String, // newsletter, exit_intent, timed, scroll, click_trigger, content_locker
    pub status: String,     // draft, published, paused, archived
    pub priority: i32,

    // Content
    pub title: Option<String>,
    pub content: Option<String>,
    pub cta_text: Option<String>,
    pub cta_url: Option<String>,
    pub cta_new_tab: bool,

    // Display settings
    pub position: String, // center, top, bottom, corner
    pub size: String,     // sm, md, lg, xl, full
    pub animation: String, // fade, slide, zoom, bounce, rotate, flip
    pub show_close_button: bool,
    pub close_on_overlay_click: bool,
    pub close_on_escape: bool,
    pub auto_close_after: Option<i32>, // seconds

    // Form integration
    pub has_form: bool,
    pub form_id: Option<i32>,

    // Rules stored as JSONB
    pub trigger_rules: sqlx::types::Json<TriggerRules>,
    pub frequency_rules: sqlx::types::Json<FrequencyRules>,
    pub display_rules: sqlx::types::Json<DisplayRules>,
    pub design: sqlx::types::Json<PopupDesign>,

    // A/B Testing
    pub ab_test_id: Option<i32>,
    pub variant_name: Option<String>,
    pub traffic_allocation: Option<i32>, // percentage 0-100

    // Scheduling
    pub start_date: Option<DateTime<Utc>>,
    pub end_date: Option<DateTime<Utc>>,

    // Analytics
    pub total_views: i64,
    pub total_conversions: i64,
    pub conversion_rate: f64,

    // Metadata
    pub created_by: Option<i32>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Trigger configuration for popups
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TriggerRules {
    /// Delay in milliseconds before showing
    #[serde(default)]
    pub delay: Option<i32>,

    /// Scroll depth percentage (0-100) to trigger
    #[serde(default)]
    pub scroll_depth: Option<i32>,

    /// CSS selector for click-triggered popups
    #[serde(default)]
    pub selector: Option<String>,

    /// Enable exit intent detection
    #[serde(default)]
    pub exit_intent: bool,

    /// Inactivity timeout in seconds
    #[serde(default)]
    pub inactivity_timeout: Option<i32>,

    /// Video embed URL for video popups
    #[serde(default)]
    pub video_url: Option<String>,

    /// Video autoplay setting
    #[serde(default)]
    pub video_autoplay: bool,
}

/// Frequency control settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FrequencyRules {
    /// Frequency type: once, once_per_session, daily, weekly, always
    pub frequency: String,

    /// Maximum number of times to show (optional)
    #[serde(default)]
    pub max_impressions: Option<i32>,

    /// Cooldown period in hours between shows
    #[serde(default)]
    pub cooldown_hours: Option<i32>,
}

impl Default for FrequencyRules {
    fn default() -> Self {
        Self {
            frequency: "once_per_session".to_string(),
            max_impressions: None,
            cooldown_hours: None,
        }
    }
}

/// Display targeting rules
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DisplayRules {
    /// Target devices: desktop, tablet, mobile
    #[serde(default = "default_devices")]
    pub devices: Vec<String>,

    /// Page URL patterns to show on (supports wildcards)
    #[serde(default)]
    pub include_pages: Vec<String>,

    /// Page URL patterns to exclude
    #[serde(default)]
    pub exclude_pages: Vec<String>,

    /// Show only to new visitors
    #[serde(default)]
    pub new_visitors_only: bool,

    /// Show only to returning visitors
    #[serde(default)]
    pub returning_visitors_only: bool,

    /// Minimum page views required
    #[serde(default)]
    pub min_page_views: Option<i32>,

    /// UTM source filter
    #[serde(default)]
    pub utm_source: Option<Vec<String>>,

    /// UTM campaign filter
    #[serde(default)]
    pub utm_campaign: Option<Vec<String>>,

    /// Geographic targeting (country codes)
    #[serde(default)]
    pub countries: Option<Vec<String>>,
}

fn default_devices() -> Vec<String> {
    vec![
        "desktop".to_string(),
        "tablet".to_string(),
        "mobile".to_string(),
    ]
}

impl Default for DisplayRules {
    fn default() -> Self {
        Self {
            devices: default_devices(),
            include_pages: vec![],
            exclude_pages: vec![],
            new_visitors_only: false,
            returning_visitors_only: false,
            min_page_views: None,
            utm_source: None,
            utm_campaign: None,
            countries: None,
        }
    }
}

/// Design customization for popups
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PopupDesign {
    #[serde(default = "default_bg_color")]
    pub background_color: String,

    #[serde(default = "default_title_color")]
    pub title_color: String,

    #[serde(default = "default_text_color")]
    pub text_color: String,

    #[serde(default = "default_button_color")]
    pub button_color: String,

    #[serde(default = "default_button_text_color")]
    pub button_text_color: String,

    #[serde(default = "default_button_radius")]
    pub button_border_radius: i32,

    #[serde(default)]
    pub button_shadow: Option<String>,

    #[serde(default)]
    pub button_padding: Option<String>,

    /// Secondary button color
    #[serde(default)]
    pub secondary_button_color: Option<String>,

    /// Secondary button text color
    #[serde(default)]
    pub secondary_button_text_color: Option<String>,

    /// Overlay background color
    #[serde(default = "default_overlay_color")]
    pub overlay_color: String,

    /// Overlay opacity (0-100)
    #[serde(default = "default_overlay_opacity")]
    pub overlay_opacity: i32,

    /// Overlay blur amount in pixels
    #[serde(default)]
    pub overlay_blur: Option<i32>,

    /// Custom CSS
    #[serde(default)]
    pub custom_css: Option<String>,

    /// Background image URL
    #[serde(default)]
    pub background_image: Option<String>,

    /// Header image URL (displayed at top of popup)
    #[serde(default)]
    pub header_image: Option<String>,

    /// Video embed URL
    #[serde(default)]
    pub video_url: Option<String>,

    /// Video autoplay setting
    #[serde(default)]
    pub video_autoplay: bool,

    /// Border radius in pixels
    #[serde(default = "default_border_radius")]
    pub border_radius: i32,

    /// Template ID (for pre-designed templates)
    #[serde(default)]
    pub template_id: Option<String>,
}

/// Popup template for pre-designed popup layouts
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PopupTemplate {
    pub id: String,
    pub name: String,
    pub category: String, // newsletter, exit_intent, promotional, announcement, etc.
    pub thumbnail: Option<String>,
    pub design: PopupDesign,
    pub default_content: Option<String>,
    pub default_title: Option<String>,
    pub description: Option<String>,
}

fn default_bg_color() -> String {
    "#ffffff".to_string()
}
fn default_title_color() -> String {
    "#1f2937".to_string()
}
fn default_text_color() -> String {
    "#4b5563".to_string()
}
fn default_button_color() -> String {
    "#3b82f6".to_string()
}
fn default_button_text_color() -> String {
    "#ffffff".to_string()
}
fn default_button_radius() -> i32 {
    8
}
fn default_overlay_color() -> String {
    "#000000".to_string()
}
fn default_overlay_opacity() -> i32 {
    50
}
fn default_border_radius() -> i32 {
    12
}

impl Default for PopupDesign {
    fn default() -> Self {
        Self {
            background_color: default_bg_color(),
            title_color: default_title_color(),
            text_color: default_text_color(),
            button_color: default_button_color(),
            button_text_color: default_button_text_color(),
            button_border_radius: default_button_radius(),
            button_shadow: Some(
                "0 4px 6px -1px rgba(0, 0, 0, 0.1), 0 2px 4px -1px rgba(0, 0, 0, 0.06)".to_string(),
            ),
            button_padding: Some("0.875rem 1.5rem".to_string()),
            overlay_color: default_overlay_color(),
            overlay_opacity: default_overlay_opacity(),
            custom_css: None,
            background_image: None,
            border_radius: default_border_radius(),
        }
    }
}

/// Popup analytics data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PopupAnalytics {
    pub popup_id: i32,

    pub views: ViewMetrics,
    pub conversions: ConversionMetrics,
    pub conversion_rate: ConversionRateMetrics,
    pub device_breakdown: DeviceBreakdown,
    pub top_pages: Vec<PagePerformance>,
    pub timeline: TimelineData,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ViewMetrics {
    pub total: i64,
    pub today: i64,
    pub this_week: i64,
    pub this_month: i64,
    pub trend: String,           // up, down, stable
    pub trend_percentage: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConversionMetrics {
    pub total: i64,
    pub today: i64,
    pub this_week: i64,
    pub this_month: i64,
    pub trend: String,
    pub trend_percentage: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConversionRateMetrics {
    pub overall: f64,
    pub today: f64,
    pub this_week: f64,
    pub this_month: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceBreakdown {
    pub desktop: i64,
    pub tablet: i64,
    pub mobile: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PagePerformance {
    pub url: String,
    pub views: i64,
    pub conversions: i64,
    pub conversion_rate: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimelineData {
    pub views: Vec<DailyCount>,
    pub conversions: Vec<DailyCount>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DailyCount {
    pub date: String,
    pub count: i64,
}

/// Popup event for analytics tracking
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct PopupEvent {
    pub id: i64,
    pub popup_id: i32,
    pub event_type: String, // view, conversion, close, minimize
    pub session_id: Option<String>,
    pub user_id: Option<i32>,
    pub page_url: Option<String>,
    pub device_type: Option<String>,
    pub metadata: Option<sqlx::types::Json<serde_json::Value>>,
    pub created_at: DateTime<Utc>,
}

/// A/B Test configuration
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct PopupAbTest {
    pub id: i32,
    pub name: String,
    pub status: String, // draft, running, paused, completed
    pub base_popup_id: i32,
    pub winner_popup_id: Option<i32>,
    pub confidence_threshold: f64, // e.g., 0.95 for 95% confidence
    pub start_date: Option<DateTime<Utc>>,
    pub end_date: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// A/B Test variant metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AbTestVariantMetrics {
    pub popup_id: i32,
    pub variant_name: String,
    pub impressions: i64,
    pub conversions: i64,
    pub conversion_rate: f64,
    pub confidence: f64,
    pub is_winner: bool,
}

// ============================================================================
// Request/Response DTOs
// ============================================================================

/// Create popup request
#[derive(Debug, Deserialize)]
pub struct CreatePopupRequest {
    pub name: String,
    #[serde(rename = "type")]
    pub popup_type: Option<String>,
    pub status: Option<String>,
    pub priority: Option<i32>,
    pub title: Option<String>,
    pub content: Option<String>,
    pub cta_text: Option<String>,
    pub cta_url: Option<String>,
    pub cta_new_tab: Option<bool>,
    pub position: Option<String>,
    pub size: Option<String>,
    pub animation: Option<String>,
    pub show_close_button: Option<bool>,
    pub close_on_overlay_click: Option<bool>,
    pub close_on_escape: Option<bool>,
    pub auto_close_after: Option<i32>,
    pub has_form: Option<bool>,
    pub form_id: Option<i32>,
    pub trigger_rules: Option<TriggerRules>,
    pub frequency_rules: Option<FrequencyRules>,
    pub display_rules: Option<DisplayRules>,
    pub design: Option<PopupDesign>,
    pub start_date: Option<DateTime<Utc>>,
    pub end_date: Option<DateTime<Utc>>,
}

/// Update popup request
#[derive(Debug, Deserialize)]
pub struct UpdatePopupRequest {
    pub name: Option<String>,
    #[serde(rename = "type")]
    pub popup_type: Option<String>,
    pub status: Option<String>,
    pub priority: Option<i32>,
    pub title: Option<String>,
    pub content: Option<String>,
    pub cta_text: Option<String>,
    pub cta_url: Option<String>,
    pub cta_new_tab: Option<bool>,
    pub position: Option<String>,
    pub size: Option<String>,
    pub animation: Option<String>,
    pub show_close_button: Option<bool>,
    pub close_on_overlay_click: Option<bool>,
    pub close_on_escape: Option<bool>,
    pub auto_close_after: Option<i32>,
    pub has_form: Option<bool>,
    pub form_id: Option<i32>,
    pub trigger_rules: Option<TriggerRules>,
    pub frequency_rules: Option<FrequencyRules>,
    pub display_rules: Option<DisplayRules>,
    pub design: Option<PopupDesign>,
    pub start_date: Option<DateTime<Utc>>,
    pub end_date: Option<DateTime<Utc>>,
    #[serde(rename = "isActive")]
    pub is_active: Option<bool>,
}

/// Track popup event request
#[derive(Debug, Deserialize)]
pub struct TrackEventRequest {
    pub event_type: String,
    pub session_id: Option<String>,
    pub page_url: Option<String>,
    pub device_type: Option<String>,
    pub metadata: Option<serde_json::Value>,
}

/// Popup list response
#[derive(Debug, Serialize)]
pub struct PopupListResponse {
    pub popups: Vec<PopupSummary>,
    pub total: i64,
    pub page: i32,
    pub per_page: i32,
}

/// Popup summary for list views
#[derive(Debug, Serialize)]
pub struct PopupSummary {
    pub id: i32,
    pub name: String,
    #[serde(rename = "type")]
    pub popup_type: String,
    pub status: String,
    pub priority: i32,
    pub title: Option<String>,
    #[serde(rename = "isActive")]
    pub is_active: bool,
    pub total_views: i64,
    pub total_conversions: i64,
    pub conversion_rate: f64,
    pub performance_status: String, // excellent, good, average, poor
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,

    // For frontend compatibility
    pub impressions: i64,
    pub conversions: i64,
    #[serde(rename = "displayRules")]
    pub display_rules: DisplayRules,
}

impl PopupSummary {
    pub fn from_popup(popup: &Popup) -> Self {
        let performance_status = if popup.conversion_rate >= 5.0 {
            "excellent"
        } else if popup.conversion_rate >= 3.0 {
            "good"
        } else if popup.conversion_rate >= 1.0 {
            "average"
        } else {
            "poor"
        };

        Self {
            id: popup.id,
            name: popup.name.clone(),
            popup_type: popup.popup_type.clone(),
            status: popup.status.clone(),
            priority: popup.priority,
            title: popup.title.clone(),
            is_active: popup.status == "published",
            total_views: popup.total_views,
            total_conversions: popup.total_conversions,
            conversion_rate: popup.conversion_rate,
            performance_status: performance_status.to_string(),
            created_at: popup.created_at,
            updated_at: popup.updated_at,
            impressions: popup.total_views,
            conversions: popup.total_conversions,
            display_rules: popup.display_rules.0.clone(),
        }
    }
}

/// Single popup response
#[derive(Debug, Serialize)]
pub struct PopupResponse {
    pub popup: PopupDetail,
}

/// Detailed popup for edit views
#[derive(Debug, Serialize)]
pub struct PopupDetail {
    pub id: i32,
    pub name: String,
    #[serde(rename = "type")]
    pub popup_type: String,
    pub status: String,
    pub priority: i32,
    pub title: Option<String>,
    pub content: Option<String>,
    pub cta_text: Option<String>,
    pub cta_url: Option<String>,
    pub cta_new_tab: bool,
    pub position: String,
    pub size: String,
    pub animation: String,
    pub show_close_button: bool,
    pub close_on_overlay_click: bool,
    pub close_on_escape: bool,
    pub auto_close_after: Option<i32>,
    pub has_form: bool,
    pub form_id: Option<i32>,
    pub trigger_rules: TriggerRules,
    pub frequency_rules: FrequencyRules,
    pub display_rules: DisplayRules,
    pub design: PopupDesign,
    pub start_date: Option<DateTime<Utc>>,
    pub end_date: Option<DateTime<Utc>>,
    pub total_views: i64,
    pub total_conversions: i64,
    pub conversion_rate: f64,
    pub performance_status: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl From<Popup> for PopupDetail {
    fn from(p: Popup) -> Self {
        let performance_status = if p.conversion_rate >= 5.0 {
            "excellent"
        } else if p.conversion_rate >= 3.0 {
            "good"
        } else if p.conversion_rate >= 1.0 {
            "average"
        } else {
            "poor"
        };

        Self {
            id: p.id,
            name: p.name,
            popup_type: p.popup_type,
            status: p.status,
            priority: p.priority,
            title: p.title,
            content: p.content,
            cta_text: p.cta_text,
            cta_url: p.cta_url,
            cta_new_tab: p.cta_new_tab,
            position: p.position,
            size: p.size,
            animation: p.animation,
            show_close_button: p.show_close_button,
            close_on_overlay_click: p.close_on_overlay_click,
            close_on_escape: p.close_on_escape,
            auto_close_after: p.auto_close_after,
            has_form: p.has_form,
            form_id: p.form_id,
            trigger_rules: p.trigger_rules.0,
            frequency_rules: p.frequency_rules.0,
            display_rules: p.display_rules.0,
            design: p.design.0,
            start_date: p.start_date,
            end_date: p.end_date,
            total_views: p.total_views,
            total_conversions: p.total_conversions,
            conversion_rate: p.conversion_rate,
            performance_status: performance_status.to_string(),
            created_at: p.created_at,
            updated_at: p.updated_at,
        }
    }
}
