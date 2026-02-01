//! Popup Routes - Revolution Trading Pros
//! Apple ICT 7+ Principal Engineer Grade - February 2026
//!
//! Public popup API for:
//! - Fetching active popups for a page
//! - Tracking impressions and conversions
//! - Form submissions
//! - A/B test variant selection

use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};
use chrono::Utc;
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::models::popup::{DisplayRules, FrequencyRules, PopupDesign, TriggerRules};
use crate::AppState;

/// Query params for active popups
#[derive(Debug, Deserialize)]
pub struct ActivePopupsQuery {
    pub page: Option<String>,
    pub device: Option<String>,
    pub utm_source: Option<String>,
    pub utm_campaign: Option<String>,
    pub is_returning: Option<bool>,
}

/// Public popup response (simplified for frontend)
#[derive(Debug, Serialize)]
pub struct PublicPopup {
    pub id: i32,
    pub name: String,
    #[serde(rename = "type")]
    pub popup_type: String,
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
    pub priority: i32,

    // Frontend compatibility fields
    #[serde(rename = "isActive")]
    pub is_active: bool,
    #[serde(rename = "closeButton")]
    pub close_button: bool,
    #[serde(rename = "closeOnOverlayClick")]
    pub close_on_overlay_click_compat: bool,
    #[serde(rename = "displayRules")]
    pub display_rules_compat: DisplayRulesCompat,
    pub impressions: i64,
    pub conversions: i64,
}

#[derive(Debug, Serialize)]
pub struct DisplayRulesCompat {
    pub frequency: String,
    #[serde(rename = "delaySeconds")]
    pub delay_seconds: i32,
    #[serde(rename = "showOnScroll")]
    pub show_on_scroll: bool,
    #[serde(rename = "scrollPercentage")]
    pub scroll_percentage: i32,
    #[serde(rename = "showOnExit")]
    pub show_on_exit: bool,
    #[serde(rename = "deviceTargeting")]
    pub device_targeting: String,
    pub pages: String,
    #[serde(rename = "excludePages")]
    pub exclude_pages: String,
}

/// Get active popups for a page (public)
async fn get_active_popups(
    State(state): State<AppState>,
    Query(query): Query<ActivePopupsQuery>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let page = query.page.unwrap_or_else(|| "/".to_string());
    let device = query.device.unwrap_or_else(|| "desktop".to_string());

    let now = Utc::now();

    // Fetch all published popups that match the targeting criteria
    let popups_result: Result<Vec<PopupRow>, _> = sqlx::query_as(
        r#"
        SELECT
            id, name, type as popup_type, status, priority,
            title, content, cta_text, cta_url, cta_new_tab,
            position, size, animation,
            show_close_button, close_on_overlay_click, close_on_escape,
            auto_close_after, has_form, form_id,
            trigger_rules, frequency_rules, display_rules, design,
            start_date, end_date,
            total_views, total_conversions
        FROM popups
        WHERE status = 'published'
          AND (start_date IS NULL OR start_date <= $1)
          AND (end_date IS NULL OR end_date >= $1)
        ORDER BY priority DESC
        "#,
    )
    .bind(now)
    .fetch_all(&state.db.pool)
    .await;

    let popups = match popups_result {
        Ok(rows) => rows,
        Err(e) => {
            // If table doesn't exist, return empty list gracefully
            if e.to_string().contains("does not exist") {
                return Ok(Json(json!({ "popups": [] })));
            }
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "error": format!("Database error: {}", e) })),
            ));
        }
    };

    // Filter popups based on targeting rules
    let eligible_popups: Vec<PublicPopup> = popups
        .into_iter()
        .filter(|popup| {
            let rules = &popup.display_rules.0;

            // Check device targeting
            if !rules.devices.is_empty() && !rules.devices.contains(&device) {
                return false;
            }

            // Check page targeting
            if !rules.include_pages.is_empty() {
                let page_matches = rules.include_pages.iter().any(|pattern| {
                    if pattern.ends_with('*') {
                        page.starts_with(&pattern[..pattern.len() - 1])
                    } else {
                        &page == pattern
                    }
                });
                if !page_matches {
                    return false;
                }
            }

            // Check excluded pages
            if rules.exclude_pages.iter().any(|pattern| {
                if pattern.ends_with('*') {
                    page.starts_with(&pattern[..pattern.len() - 1])
                } else {
                    &page == pattern
                }
            }) {
                return false;
            }

            // Check UTM targeting
            if let Some(ref sources) = rules.utm_source {
                if let Some(ref user_source) = query.utm_source {
                    if !sources.contains(user_source) {
                        return false;
                    }
                }
            }

            if let Some(ref campaigns) = rules.utm_campaign {
                if let Some(ref user_campaign) = query.utm_campaign {
                    if !campaigns.contains(user_campaign) {
                        return false;
                    }
                }
            }

            // Check visitor type
            if rules.new_visitors_only && query.is_returning == Some(true) {
                return false;
            }
            if rules.returning_visitors_only && query.is_returning == Some(false) {
                return false;
            }

            true
        })
        .map(|popup| {
            let trigger_rules = popup.trigger_rules.0.clone();
            let frequency_rules = popup.frequency_rules.0.clone();
            let display_rules = popup.display_rules.0.clone();

            PublicPopup {
                id: popup.id,
                name: popup.name.clone(),
                popup_type: popup.popup_type.clone(),
                title: popup.title.clone(),
                content: popup.content.clone(),
                cta_text: popup.cta_text.clone(),
                cta_url: popup.cta_url.clone(),
                cta_new_tab: popup.cta_new_tab,
                position: popup.position.clone(),
                size: popup.size.clone(),
                animation: popup.animation.clone(),
                show_close_button: popup.show_close_button,
                close_on_overlay_click: popup.close_on_overlay_click,
                close_on_escape: popup.close_on_escape,
                auto_close_after: popup.auto_close_after,
                has_form: popup.has_form,
                form_id: popup.form_id,
                trigger_rules: trigger_rules.clone(),
                frequency_rules: frequency_rules.clone(),
                display_rules: display_rules.clone(),
                design: popup.design.0.clone(),
                priority: popup.priority,
                is_active: true,
                close_button: popup.show_close_button,
                close_on_overlay_click_compat: popup.close_on_overlay_click,
                display_rules_compat: DisplayRulesCompat {
                    frequency: frequency_rules.frequency.clone(),
                    delay_seconds: trigger_rules.delay.unwrap_or(0) / 1000,
                    show_on_scroll: trigger_rules.scroll_depth.is_some(),
                    scroll_percentage: trigger_rules.scroll_depth.unwrap_or(50),
                    show_on_exit: trigger_rules.exit_intent,
                    device_targeting: if display_rules.devices.len() == 3 {
                        "all".to_string()
                    } else if display_rules.devices.contains(&"mobile".to_string()) {
                        "mobile".to_string()
                    } else {
                        "desktop".to_string()
                    },
                    pages: display_rules.include_pages.join("\n"),
                    exclude_pages: display_rules.exclude_pages.join("\n"),
                },
                impressions: popup.total_views,
                conversions: popup.total_conversions,
            }
        })
        .collect();

    Ok(Json(json!({ "popups": eligible_popups })))
}

/// Row type for fetching popups
#[derive(Debug, sqlx::FromRow)]
struct PopupRow {
    id: i32,
    name: String,
    popup_type: String,
    status: String,
    priority: i32,
    title: Option<String>,
    content: Option<String>,
    cta_text: Option<String>,
    cta_url: Option<String>,
    cta_new_tab: bool,
    position: String,
    size: String,
    animation: String,
    show_close_button: bool,
    close_on_overlay_click: bool,
    close_on_escape: bool,
    auto_close_after: Option<i32>,
    has_form: bool,
    form_id: Option<i32>,
    trigger_rules: sqlx::types::Json<TriggerRules>,
    frequency_rules: sqlx::types::Json<FrequencyRules>,
    display_rules: sqlx::types::Json<DisplayRules>,
    design: sqlx::types::Json<PopupDesign>,
    start_date: Option<chrono::DateTime<Utc>>,
    end_date: Option<chrono::DateTime<Utc>>,
    total_views: i64,
    total_conversions: i64,
}

/// Track popup impression
async fn track_impression(
    State(state): State<AppState>,
    Path(id): Path<i32>,
    Json(body): Json<TrackEventBody>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    // Insert event
    let _ = sqlx::query(
        r#"
        INSERT INTO popup_events (popup_id, event_type, session_id, page_url, device_type, created_at)
        VALUES ($1, 'view', $2, $3, $4, NOW())
        "#,
    )
    .bind(id)
    .bind(&body.session_id)
    .bind(&body.page_url)
    .bind(&body.device_type)
    .execute(&state.db.pool)
    .await;

    // Update popup total views
    let _ = sqlx::query(
        r#"
        UPDATE popups
        SET total_views = total_views + 1,
            conversion_rate = CASE
                WHEN total_views + 1 > 0
                THEN (total_conversions::float / (total_views + 1)::float) * 100
                ELSE 0
            END
        WHERE id = $1
        "#,
    )
    .bind(id)
    .execute(&state.db.pool)
    .await;

    Ok(Json(json!({ "success": true })))
}

/// Track popup conversion
async fn track_conversion(
    State(state): State<AppState>,
    Path(id): Path<i32>,
    Json(body): Json<TrackConversionBody>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    // Insert event
    let _ = sqlx::query(
        r#"
        INSERT INTO popup_events (popup_id, event_type, session_id, page_url, device_type, metadata, created_at)
        VALUES ($1, 'conversion', $2, $3, $4, $5, NOW())
        "#,
    )
    .bind(id)
    .bind(&body.session_id)
    .bind(&body.page_url)
    .bind(&body.device_type)
    .bind(sqlx::types::Json(&body.metadata))
    .execute(&state.db.pool)
    .await;

    // Update popup totals
    let _ = sqlx::query(
        r#"
        UPDATE popups
        SET total_conversions = total_conversions + 1,
            conversion_rate = CASE
                WHEN total_views > 0
                THEN ((total_conversions + 1)::float / total_views::float) * 100
                ELSE 0
            END
        WHERE id = $1
        "#,
    )
    .bind(id)
    .execute(&state.db.pool)
    .await;

    Ok(Json(json!({ "success": true })))
}

#[derive(Debug, Deserialize)]
pub struct TrackEventBody {
    pub session_id: Option<String>,
    pub page_url: Option<String>,
    pub device_type: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct TrackConversionBody {
    pub session_id: Option<String>,
    pub page_url: Option<String>,
    pub device_type: Option<String>,
    pub action: Option<String>,
    pub value: Option<String>,
    #[serde(rename = "conversionTime")]
    pub conversion_time: Option<i64>,
    pub metadata: Option<serde_json::Value>,
}

/// Submit popup form
async fn submit_popup_form(
    State(state): State<AppState>,
    Path(id): Path<i32>,
    Json(body): Json<PopupFormSubmission>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    // Get popup to check form configuration
    let popup: Option<(bool, Option<i32>)> =
        sqlx::query_as("SELECT has_form, form_id FROM popups WHERE id = $1")
            .bind(id)
            .fetch_optional(&state.db.pool)
            .await
            .map_err(|e| {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({ "error": format!("Database error: {}", e) })),
                )
            })?;

    let (has_form, form_id) = popup.ok_or_else(|| {
        (
            StatusCode::NOT_FOUND,
            Json(json!({ "error": "Popup not found" })),
        )
    })?;

    if !has_form {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(json!({ "error": "This popup does not have a form" })),
        ));
    }

    // Store form submission
    let _ = sqlx::query(
        r#"
        INSERT INTO popup_form_submissions (popup_id, form_id, data, session_id, metadata, created_at)
        VALUES ($1, $2, $3, $4, $5, NOW())
        "#,
    )
    .bind(id)
    .bind(form_id)
    .bind(sqlx::types::Json(&body.form_data))
    .bind(&body.session_id)
    .bind(sqlx::types::Json(&body.metadata))
    .execute(&state.db.pool)
    .await;

    // Track as conversion
    let _ = sqlx::query(
        r#"
        INSERT INTO popup_events (popup_id, event_type, session_id, metadata, created_at)
        VALUES ($1, 'conversion', $2, $3, NOW())
        "#,
    )
    .bind(id)
    .bind(&body.session_id)
    .bind(sqlx::types::Json(json!({ "type": "form_submit" })))
    .execute(&state.db.pool)
    .await;

    // Update conversion count
    let _ = sqlx::query(
        r#"
        UPDATE popups
        SET total_conversions = total_conversions + 1,
            conversion_rate = CASE
                WHEN total_views > 0
                THEN ((total_conversions + 1)::float / total_views::float) * 100
                ELSE 0
            END
        WHERE id = $1
        "#,
    )
    .bind(id)
    .execute(&state.db.pool)
    .await;

    Ok(Json(json!({
        "status": "ok",
        "message": "Form submitted successfully"
    })))
}

#[derive(Debug, Deserialize)]
pub struct PopupFormSubmission {
    #[serde(rename = "formData")]
    pub form_data: serde_json::Value,
    pub session_id: Option<String>,
    pub metadata: Option<serde_json::Value>,
}

/// Batch track events (for analytics buffering)
async fn track_events_batch(
    State(state): State<AppState>,
    Json(body): Json<BatchEventsBody>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let event_count = body.events.len();
    for event in body.events {
        let _ = sqlx::query(
            r#"
            INSERT INTO popup_events (popup_id, event_type, session_id, page_url, device_type, metadata, created_at)
            VALUES ($1, $2, $3, $4, $5, $6, NOW())
            "#,
        )
        .bind(event.popup_id)
        .bind(&event.event_type)
        .bind(&event.session_id)
        .bind(&event.page_url)
        .bind(&event.device_type)
        .bind(sqlx::types::Json(&event.data))
        .execute(&state.db.pool)
        .await;
    }

    Ok(Json(json!({ "success": true, "processed": event_count })))
}

#[derive(Debug, Deserialize)]
pub struct BatchEventsBody {
    pub events: Vec<BatchEvent>,
}

#[derive(Debug, Deserialize)]
pub struct BatchEvent {
    pub popup_id: Option<i32>,
    #[serde(rename = "type")]
    pub event_type: String,
    #[serde(rename = "popupId")]
    pub popup_id_alt: Option<String>,
    pub session_id: Option<String>,
    pub page_url: Option<String>,
    pub device_type: Option<String>,
    pub data: Option<serde_json::Value>,
    pub timestamp: Option<String>,
}

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/active", get(get_active_popups))
        .route("/:id/impression", post(track_impression))
        .route("/:id/conversion", post(track_conversion))
        .route("/:id/form-submit", post(submit_popup_form))
        .route("/events", post(track_events_batch))
}
