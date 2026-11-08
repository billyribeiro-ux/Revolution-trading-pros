//! Popup Routes - Revolution Trading Pros
//! Apple ICT 7+ Principal Engineer Grade - February 2026
//!
//! Public popup API for:
//! - Fetching active popups for a page
//! - Tracking impressions and conversions
//! - Form submissions
//! - A/B test variant selection

use axum::{
    Json, Router,
    extract::{Path, Query, State},
    http::StatusCode,
    routing::{get, post},
};
use chrono::Utc;
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::models::popup::{DisplayRules, FrequencyRules, PopupDesign, TriggerRules};
use crate::{AppState, utils::html};

fn safe_page_path(value: Option<&str>) -> Option<String> {
    let value = value?;
    let path = if value.starts_with('/') && !value.starts_with("//") {
        value.split(['?', '#']).next()?.to_string()
    } else {
        let url = reqwest::Url::parse(value).ok()?;
        if !matches!(url.scheme(), "http" | "https") {
            return None;
        }
        url.path().to_string()
    };
    if path.len() > 512
        || path.contains('@')
        || path.to_ascii_lowercase().contains("%40")
        || [
            "/account",
            "/admin",
            "/auth",
            "/checkout",
            "/dashboard",
            "/login",
            "/my",
            "/register",
        ]
        .iter()
        .any(|prefix| path == *prefix || path.starts_with(&format!("{prefix}/")))
    {
        return None;
    }
    Some(path)
}

fn safe_session_id(value: Option<&str>) -> Option<&str> {
    value.filter(|id| {
        id.len() <= 100
            && id
                .bytes()
                .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'-' | b'_'))
    })
}

fn safe_device(value: Option<&str>) -> Option<&str> {
    value.filter(|device| matches!(*device, "desktop" | "tablet" | "mobile"))
}

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
        r"
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
        ",
    )
    .bind(now)
    .fetch_all(&state.db.pool)
    .await;

    let popups = match popups_result {
        Ok(rows) => rows,
        Err(e) => {
            tracing::error!(error = %e, "Failed to load active popups");
            return Err(popup_db_error());
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
            if let Some(ref sources) = rules.utm_source
                && !sources.is_empty()
                && !query
                    .utm_source
                    .as_ref()
                    .is_some_and(|source| sources.contains(source))
            {
                return false;
            }

            if let Some(ref campaigns) = rules.utm_campaign
                && !campaigns.is_empty()
                && !query
                    .utm_campaign
                    .as_ref()
                    .is_some_and(|campaign| campaigns.contains(campaign))
            {
                return false;
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
                content: popup.content.as_deref().map(html::sanitize_html),
                cta_text: popup.cta_text.clone(),
                cta_url: popup.cta_url.clone().filter(|url| html::is_safe_url(url)),
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
                    frequency: frequency_rules.frequency,
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
    let mut tx = state.db.pool.begin().await.map_err(|_| popup_db_error())?;
    let updated = sqlx::query(
        "UPDATE popups SET total_views = total_views + 1 WHERE id = $1 AND status = 'published'",
    )
    .bind(id)
    .execute(&mut *tx)
    .await
    .map_err(|_| popup_db_error())?;
    if updated.rows_affected() == 0 {
        return Err((
            StatusCode::NOT_FOUND,
            Json(json!({ "error": "Popup not found" })),
        ));
    }
    sqlx::query(
        r"
        INSERT INTO popup_events (popup_id, event_type, session_id, page_url, device_type, created_at)
        VALUES ($1, 'view', $2, $3, $4, NOW())
        ",
    )
    .bind(id)
    .bind(safe_session_id(body.session_id.as_deref()))
    .bind(safe_page_path(body.page_url.as_deref()))
    .bind(safe_device(body.device_type.as_deref()))
    .execute(&mut *tx)
    .await
    .map_err(|_| popup_db_error())?;
    tx.commit().await.map_err(|_| popup_db_error())?;

    Ok(Json(json!({ "success": true })))
}

/// Track popup conversion
async fn track_conversion(
    State(state): State<AppState>,
    Path(id): Path<i32>,
    Json(body): Json<TrackConversionBody>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let mut tx = state.db.pool.begin().await.map_err(|_| popup_db_error())?;
    let updated = sqlx::query(
        "UPDATE popups SET total_conversions = total_conversions + 1 WHERE id = $1 AND status = 'published'",
    )
    .bind(id)
    .execute(&mut *tx)
    .await
    .map_err(|_| popup_db_error())?;
    if updated.rows_affected() == 0 {
        return Err((
            StatusCode::NOT_FOUND,
            Json(json!({ "error": "Popup not found" })),
        ));
    }
    sqlx::query(
        r"
        INSERT INTO popup_events (popup_id, event_type, session_id, page_url, device_type, metadata, created_at)
        VALUES ($1, 'conversion', $2, $3, $4, $5, NOW())
        ",
    )
    .bind(id)
    .bind(safe_session_id(body.session_id.as_deref()))
    .bind(safe_page_path(body.page_url.as_deref()))
    .bind(safe_device(body.device_type.as_deref()))
    .bind(sqlx::types::Json(json!({})))
    .execute(&mut *tx)
    .await
    .map_err(|_| popup_db_error())?;
    tx.commit().await.map_err(|_| popup_db_error())?;

    Ok(Json(json!({ "success": true })))
}

fn popup_db_error() -> (StatusCode, Json<serde_json::Value>) {
    (
        StatusCode::INTERNAL_SERVER_ERROR,
        Json(json!({ "error": "Popup service unavailable" })),
    )
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
    let popup: Option<(bool, Option<i32>)> = sqlx::query_as(
        "SELECT has_form, form_id FROM popups WHERE id = $1 AND status = 'published'",
    )
    .bind(id)
    .fetch_optional(&state.db.pool)
    .await
    .map_err(|e| {
        tracing::error!(error = %e, "Failed to load popup form configuration");
        popup_db_error()
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

    let mut tx = state.db.pool.begin().await.map_err(|_| popup_db_error())?;
    let updated = sqlx::query(
        "UPDATE popups SET total_conversions = total_conversions + 1 WHERE id = $1 AND status = 'published'",
    )
    .bind(id)
    .execute(&mut *tx)
    .await
    .map_err(|_| popup_db_error())?;
    if updated.rows_affected() == 0 {
        return Err((
            StatusCode::NOT_FOUND,
            Json(json!({ "error": "Popup not found" })),
        ));
    }
    sqlx::query(
        r"
        INSERT INTO popup_form_submissions (popup_id, form_id, data, session_id, metadata, created_at)
        VALUES ($1, $2, $3, $4, $5, NOW())
        ",
    )
    .bind(id)
    .bind(form_id)
    .bind(sqlx::types::Json(&body.form_data))
    .bind(safe_session_id(body.session_id.as_deref()))
    .bind(sqlx::types::Json(json!({})))
    .execute(&mut *tx)
    .await
    .map_err(|_| popup_db_error())?;

    // Track as conversion
    sqlx::query(
        r"
        INSERT INTO popup_events (popup_id, event_type, session_id, metadata, created_at)
        VALUES ($1, 'conversion', $2, $3, NOW())
        ",
    )
    .bind(id)
    .bind(safe_session_id(body.session_id.as_deref()))
    .bind(sqlx::types::Json(json!({ "type": "form_submit" })))
    .execute(&mut *tx)
    .await
    .map_err(|_| popup_db_error())?;

    tx.commit().await.map_err(|_| popup_db_error())?;

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
    if body.events.len() > 50 {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(json!({ "error": "Maximum 50 events per batch" })),
        ));
    }

    let event_count = body.events.len();
    let mut tx = state.db.pool.begin().await.map_err(|_| popup_db_error())?;
    for event in body.events {
        let popup_id = event
            .popup_id
            .or_else(|| event.popup_id_alt.as_deref()?.parse().ok())
            .ok_or_else(|| {
                (
                    StatusCode::BAD_REQUEST,
                    Json(json!({ "error": "Invalid popup ID" })),
                )
            })?;
        let event_type = match event.event_type.as_str() {
            "impression" | "view" => "view",
            "conversion" => "conversion",
            _ => {
                return Err((
                    StatusCode::BAD_REQUEST,
                    Json(json!({ "error": "Invalid event type" })),
                ));
            }
        };
        let statement = if event_type == "view" {
            "UPDATE popups SET total_views = total_views + 1 WHERE id = $1 AND status = 'published'"
        } else {
            "UPDATE popups SET total_conversions = total_conversions + 1 WHERE id = $1 AND status = 'published'"
        };
        let updated = sqlx::query(statement)
            .bind(popup_id)
            .execute(&mut *tx)
            .await
            .map_err(|_| popup_db_error())?;
        if updated.rows_affected() == 0 {
            return Err((
                StatusCode::NOT_FOUND,
                Json(json!({ "error": "Popup not found" })),
            ));
        }
        sqlx::query(
            r"
            INSERT INTO popup_events (popup_id, event_type, session_id, page_url, device_type, metadata, created_at)
            VALUES ($1, $2, $3, $4, $5, $6, NOW())
            ",
        )
        .bind(popup_id)
        .bind(event_type)
        .bind(safe_session_id(event.session_id.as_deref()))
        .bind(safe_page_path(event.page_url.as_deref()))
        .bind(safe_device(event.device_type.as_deref()))
        .bind(sqlx::types::Json(json!({})))
        .execute(&mut *tx)
        .await
        .map_err(|_| popup_db_error())?;
    }
    tx.commit().await.map_err(|_| popup_db_error())?;

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
        .route("/{id}/impression", post(track_impression))
        .route("/{id}/conversion", post(track_conversion))
        .route("/{id}/form-submit", post(submit_popup_form))
        .route("/events", post(track_events_batch))
}

#[cfg(test)]
mod tests {
    use super::{safe_device, safe_page_path, safe_session_id};

    #[test]
    fn public_event_fields_drop_private_data() {
        assert_eq!(
            safe_page_path(Some(
                "https://example.test/blog/article?email=private@example.test#token"
            )),
            Some("/blog/article".to_string())
        );
        assert_eq!(safe_page_path(Some("/checkout?secret=token")), None);
        assert_eq!(safe_page_path(Some("javascript:alert(1)")), None);
        assert_eq!(
            safe_session_id(Some("safe_session-1")),
            Some("safe_session-1")
        );
        assert_eq!(safe_session_id(Some("email@example.test")), None);
        assert_eq!(safe_device(Some("desktop")), Some("desktop"));
        assert_eq!(safe_device(Some("unknown")), None);
    }
}
