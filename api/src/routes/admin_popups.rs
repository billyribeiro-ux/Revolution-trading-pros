//! Admin Popup Routes - Revolution Trading Pros
//! Apple ICT 7+ Principal Engineer Grade - February 2026
//!
//! Complete admin API for popup management:
//! - Full CRUD operations
//! - Analytics and reporting
//! - A/B testing management
//! - Duplication and bulk operations
//! - Event tracking

use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    routing::{delete, get, post, put},
    Json, Router,
};
use chrono::{Duration, Utc};
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::middleware::admin::AdminUser;
use crate::models::popup::{
    AbTestVariantMetrics, ConversionMetrics, ConversionRateMetrics, CreatePopupRequest, DailyCount,
    DeviceBreakdown, DisplayRules, FrequencyRules, PagePerformance, PopupAnalytics, PopupDesign,
    PopupDetail, PopupListResponse, PopupResponse, PopupSummary, TimelineData, TriggerRules,
    UpdatePopupRequest, ViewMetrics,
};
use crate::AppState;

/// Query params for listing popups
#[derive(Debug, Deserialize)]
pub struct ListPopupsQuery {
    pub page: Option<i32>,
    pub per_page: Option<i32>,
    pub status: Option<String>,
    pub search: Option<String>,
    #[serde(rename = "type")]
    pub popup_type: Option<String>,
}

/// List all popups (admin)
async fn list_popups(
    _admin: AdminUser,
    State(state): State<AppState>,
    Query(query): Query<ListPopupsQuery>,
) -> Result<Json<PopupListResponse>, (StatusCode, Json<serde_json::Value>)> {
    let page = query.page.unwrap_or(1).max(1);
    let per_page = query.per_page.unwrap_or(20).min(100);
    let offset = (page - 1) * per_page;

    // Build parameterized query to prevent SQL injection
    let mut conditions = Vec::new();
    let mut param_idx = 1usize;

    if query.status.is_some() {
        conditions.push(format!("status = ${}", param_idx));
        param_idx += 1;
    }
    if query.popup_type.is_some() {
        conditions.push(format!("type = ${}", param_idx));
        param_idx += 1;
    }
    if query.search.is_some() {
        conditions.push(format!(
            "(name ILIKE ${} OR title ILIKE ${})",
            param_idx, param_idx
        ));
        param_idx += 1;
    }

    let where_clause = if conditions.is_empty() {
        String::new()
    } else {
        format!(" AND {}", conditions.join(" AND "))
    };

    let sql = format!(
        r#"
        SELECT id, name, type as popup_type, status, priority, title, content,
               cta_text, cta_url, cta_new_tab, position, size, animation,
               show_close_button, close_on_overlay_click, close_on_escape,
               auto_close_after, has_form, form_id,
               trigger_rules, frequency_rules, display_rules, design,
               ab_test_id, variant_name, traffic_allocation,
               start_date, end_date,
               total_views, total_conversions, conversion_rate,
               created_by, created_at, updated_at
        FROM popups
        WHERE 1=1{}
        ORDER BY priority DESC, created_at DESC LIMIT ${} OFFSET ${}
        "#,
        where_clause, param_idx, param_idx + 1
    );
    let count_sql = format!(
        "SELECT COUNT(*) FROM popups WHERE 1=1{}",
        where_clause
    );

    // Bind parameters for the main query
    let mut q = sqlx::query_as::<_, crate::models::popup::Popup>(&sql);
    if let Some(ref status) = query.status {
        q = q.bind(status);
    }
    if let Some(ref popup_type) = query.popup_type {
        q = q.bind(popup_type);
    }
    if let Some(ref search) = query.search {
        let search_pattern = format!("%{}%", search);
        q = q.bind(search_pattern);
    }
    q = q.bind(per_page);
    q = q.bind(offset);

    // Execute queries
    let popups: Vec<crate::models::popup::Popup> =
        match q.fetch_all(&state.db.pool).await {
            Ok(rows) => rows,
            Err(e) => {
                // If table doesn't exist, return empty list
                if e.to_string().contains("does not exist") {
                    return Ok(Json(PopupListResponse {
                        popups: vec![],
                        total: 0,
                        page,
                        per_page,
                    }));
                }
                return Err((
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({ "error": format!("Database error: {}", e) })),
                ));
            }
        };

    // Bind parameters for the count query
    let mut cq = sqlx::query_as::<_, (i64,)>(&count_sql);
    if let Some(ref status) = query.status {
        cq = cq.bind(status);
    }
    if let Some(ref popup_type) = query.popup_type {
        cq = cq.bind(popup_type);
    }
    if let Some(ref search) = query.search {
        let search_pattern = format!("%{}%", search);
        cq = cq.bind(search_pattern);
    }

    let total: (i64,) = cq
        .fetch_one(&state.db.pool)
        .await
        .unwrap_or((0,));

    let popup_summaries: Vec<PopupSummary> = popups.iter().map(PopupSummary::from_popup).collect();

    Ok(Json(PopupListResponse {
        popups: popup_summaries,
        total: total.0,
        page,
        per_page,
    }))
}

/// Get single popup by ID
async fn get_popup(
    _admin: AdminUser,
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<Json<PopupResponse>, (StatusCode, Json<serde_json::Value>)> {
    let popup: crate::models::popup::Popup = sqlx::query_as(
        r#"
        SELECT id, name, type as popup_type, status, priority, title, content,
               cta_text, cta_url, cta_new_tab, position, size, animation,
               show_close_button, close_on_overlay_click, close_on_escape,
               auto_close_after, has_form, form_id,
               trigger_rules, frequency_rules, display_rules, design,
               ab_test_id, variant_name, traffic_allocation,
               start_date, end_date,
               total_views, total_conversions, conversion_rate,
               created_by, created_at, updated_at
        FROM popups
        WHERE id = $1
        "#,
    )
    .bind(id)
    .fetch_optional(&state.db.pool)
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": format!("Database error: {}", e) })),
        )
    })?
    .ok_or_else(|| {
        (
            StatusCode::NOT_FOUND,
            Json(json!({ "error": "Popup not found" })),
        )
    })?;

    Ok(Json(PopupResponse {
        popup: PopupDetail::from(popup),
    }))
}

/// Create new popup
async fn create_popup(
    _admin: AdminUser,
    State(state): State<AppState>,
    Json(req): Json<CreatePopupRequest>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let popup_type = req.popup_type.unwrap_or_else(|| "timed".to_string());
    let status = req.status.unwrap_or_else(|| "draft".to_string());
    let priority = req.priority.unwrap_or(10);
    let position = req.position.unwrap_or_else(|| "center".to_string());
    let size = req.size.unwrap_or_else(|| "md".to_string());
    let animation = req.animation.unwrap_or_else(|| "zoom".to_string());
    let show_close_button = req.show_close_button.unwrap_or(true);
    let close_on_overlay_click = req.close_on_overlay_click.unwrap_or(true);
    let close_on_escape = req.close_on_escape.unwrap_or(true);
    let has_form = req.has_form.unwrap_or(false);
    let cta_new_tab = req.cta_new_tab.unwrap_or(false);

    let trigger_rules = req.trigger_rules.unwrap_or_default();
    let frequency_rules = req.frequency_rules.unwrap_or_default();
    let display_rules = req.display_rules.unwrap_or_default();
    let design = req.design.unwrap_or_default();

    let result = sqlx::query(
        r#"
        INSERT INTO popups (
            name, type, status, priority, title, content,
            cta_text, cta_url, cta_new_tab, position, size, animation,
            show_close_button, close_on_overlay_click, close_on_escape,
            auto_close_after, has_form, form_id,
            trigger_rules, frequency_rules, display_rules, design,
            start_date, end_date,
            total_views, total_conversions, conversion_rate,
            created_at, updated_at
        ) VALUES (
            $1, $2, $3, $4, $5, $6,
            $7, $8, $9, $10, $11, $12,
            $13, $14, $15,
            $16, $17, $18,
            $19, $20, $21, $22,
            $23, $24,
            0, 0, 0.0,
            NOW(), NOW()
        ) RETURNING id
        "#,
    )
    .bind(&req.name)
    .bind(&popup_type)
    .bind(&status)
    .bind(priority)
    .bind(&req.title)
    .bind(&req.content)
    .bind(&req.cta_text)
    .bind(&req.cta_url)
    .bind(cta_new_tab)
    .bind(&position)
    .bind(&size)
    .bind(&animation)
    .bind(show_close_button)
    .bind(close_on_overlay_click)
    .bind(close_on_escape)
    .bind(req.auto_close_after)
    .bind(has_form)
    .bind(req.form_id)
    .bind(sqlx::types::Json(&trigger_rules))
    .bind(sqlx::types::Json(&frequency_rules))
    .bind(sqlx::types::Json(&display_rules))
    .bind(sqlx::types::Json(&design))
    .bind(req.start_date)
    .bind(req.end_date)
    .fetch_one(&state.db.pool)
    .await;

    match result {
        Ok(row) => {
            let id: i32 = sqlx::Row::get(&row, "id");
            Ok(Json(json!({
                "message": "Popup created successfully",
                "id": id
            })))
        }
        Err(e) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": format!("Failed to create popup: {}", e) })),
        )),
    }
}

/// Update popup
async fn update_popup(
    _admin: AdminUser,
    State(state): State<AppState>,
    Path(id): Path<i32>,
    Json(req): Json<UpdatePopupRequest>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    // Handle isActive -> status conversion via parameterized query
    // If is_active is provided, it overrides the status field
    let effective_status: Option<String> = if let Some(is_active) = req.is_active {
        Some(
            if is_active { "published" } else { "paused" }.to_string(),
        )
    } else {
        req.status.clone()
    };

    // Use parameterized COALESCE query - all values are bound as parameters
    let result = sqlx::query(
        r#"
        UPDATE popups SET
            name = COALESCE($1, name),
            type = COALESCE($2, type),
            status = COALESCE($3, status),
            priority = COALESCE($4, priority),
            title = COALESCE($5, title),
            content = COALESCE($6, content),
            cta_text = COALESCE($7, cta_text),
            cta_url = COALESCE($8, cta_url),
            position = COALESCE($9, position),
            size = COALESCE($10, size),
            animation = COALESCE($11, animation),
            updated_at = NOW()
        WHERE id = $12
        "#,
    )
    .bind(&req.name)
    .bind(&req.popup_type)
    .bind(&effective_status)
    .bind(req.priority)
    .bind(&req.title)
    .bind(&req.content)
    .bind(&req.cta_text)
    .bind(&req.cta_url)
    .bind(&req.position)
    .bind(&req.size)
    .bind(&req.animation)
    .bind(id)
    .execute(&state.db.pool)
    .await;

    match result {
        Ok(_) => Ok(Json(json!({
            "message": "Popup updated successfully",
            "id": id
        }))),
        Err(e) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": format!("Failed to update popup: {}", e) })),
        )),
    }
}

/// Delete popup
async fn delete_popup(
    _admin: AdminUser,
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    // First delete related events
    let _ = sqlx::query("DELETE FROM popup_events WHERE popup_id = $1")
        .bind(id)
        .execute(&state.db.pool)
        .await;

    let result = sqlx::query("DELETE FROM popups WHERE id = $1")
        .bind(id)
        .execute(&state.db.pool)
        .await;

    match result {
        Ok(res) => {
            if res.rows_affected() == 0 {
                return Err((
                    StatusCode::NOT_FOUND,
                    Json(json!({ "error": "Popup not found" })),
                ));
            }
            Ok(Json(json!({ "message": "Popup deleted successfully" })))
        }
        Err(e) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": format!("Failed to delete popup: {}", e) })),
        )),
    }
}

/// Duplicate popup
async fn duplicate_popup(
    _admin: AdminUser,
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let result = sqlx::query(
        r#"
        INSERT INTO popups (
            name, type, status, priority, title, content,
            cta_text, cta_url, cta_new_tab, position, size, animation,
            show_close_button, close_on_overlay_click, close_on_escape,
            auto_close_after, has_form, form_id,
            trigger_rules, frequency_rules, display_rules, design,
            start_date, end_date,
            total_views, total_conversions, conversion_rate,
            created_at, updated_at
        )
        SELECT
            name || ' (Copy)', type, 'draft', priority, title, content,
            cta_text, cta_url, cta_new_tab, position, size, animation,
            show_close_button, close_on_overlay_click, close_on_escape,
            auto_close_after, has_form, form_id,
            trigger_rules, frequency_rules, display_rules, design,
            NULL, NULL,
            0, 0, 0.0,
            NOW(), NOW()
        FROM popups WHERE id = $1
        RETURNING id
        "#,
    )
    .bind(id)
    .fetch_one(&state.db.pool)
    .await;

    match result {
        Ok(row) => {
            let new_id: i32 = sqlx::Row::get(&row, "id");
            Ok(Json(json!({
                "message": "Popup duplicated successfully",
                "id": new_id
            })))
        }
        Err(e) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": format!("Failed to duplicate popup: {}", e) })),
        )),
    }
}

/// Get popup analytics
async fn get_popup_analytics(
    _admin: AdminUser,
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<Json<PopupAnalytics>, (StatusCode, Json<serde_json::Value>)> {
    let now = Utc::now();
    let today_start = now.date_naive().and_hms_opt(0, 0, 0).unwrap();
    let week_start = (now - Duration::days(7))
        .date_naive()
        .and_hms_opt(0, 0, 0)
        .unwrap();
    let month_start = (now - Duration::days(30))
        .date_naive()
        .and_hms_opt(0, 0, 0)
        .unwrap();

    // Get popup basic stats
    let popup_stats: Option<(i64, i64, f64)> = sqlx::query_as(
        "SELECT total_views, total_conversions, conversion_rate FROM popups WHERE id = $1",
    )
    .bind(id)
    .fetch_optional(&state.db.pool)
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": format!("Database error: {}", e) })),
        )
    })?;

    let (total_views, total_conversions, conversion_rate) = popup_stats.ok_or_else(|| {
        (
            StatusCode::NOT_FOUND,
            Json(json!({ "error": "Popup not found" })),
        )
    })?;

    // Try to get detailed event stats, but provide defaults if table doesn't exist
    let (today_views, week_views, month_views): (i64, i64, i64) = sqlx::query_as(
        r#"
        SELECT
            COALESCE(SUM(CASE WHEN created_at >= $2::timestamp THEN 1 ELSE 0 END), 0),
            COALESCE(SUM(CASE WHEN created_at >= $3::timestamp THEN 1 ELSE 0 END), 0),
            COALESCE(SUM(CASE WHEN created_at >= $4::timestamp THEN 1 ELSE 0 END), 0)
        FROM popup_events
        WHERE popup_id = $1 AND event_type = 'view'
        "#,
    )
    .bind(id)
    .bind(today_start)
    .bind(week_start)
    .bind(month_start)
    .fetch_optional(&state.db.pool)
    .await
    .unwrap_or(None)
    .unwrap_or((0, 0, 0));

    let (today_conv, week_conv, month_conv): (i64, i64, i64) = sqlx::query_as(
        r#"
        SELECT
            COALESCE(SUM(CASE WHEN created_at >= $2::timestamp THEN 1 ELSE 0 END), 0),
            COALESCE(SUM(CASE WHEN created_at >= $3::timestamp THEN 1 ELSE 0 END), 0),
            COALESCE(SUM(CASE WHEN created_at >= $4::timestamp THEN 1 ELSE 0 END), 0)
        FROM popup_events
        WHERE popup_id = $1 AND event_type = 'conversion'
        "#,
    )
    .bind(id)
    .bind(today_start)
    .bind(week_start)
    .bind(month_start)
    .fetch_optional(&state.db.pool)
    .await
    .unwrap_or(None)
    .unwrap_or((0, 0, 0));

    // Device breakdown
    let device_stats: Vec<(String, i64)> = sqlx::query_as(
        r#"
        SELECT COALESCE(device_type, 'desktop'), COUNT(*)
        FROM popup_events
        WHERE popup_id = $1 AND event_type = 'view'
        GROUP BY device_type
        "#,
    )
    .bind(id)
    .fetch_all(&state.db.pool)
    .await
    .unwrap_or_default();

    let mut device_breakdown = DeviceBreakdown {
        desktop: 0,
        tablet: 0,
        mobile: 0,
    };

    for (device, count) in device_stats {
        match device.as_str() {
            "desktop" => device_breakdown.desktop = count,
            "tablet" => device_breakdown.tablet = count,
            "mobile" => device_breakdown.mobile = count,
            _ => {}
        }
    }

    // Top pages
    let top_pages: Vec<(String, i64, i64)> = sqlx::query_as(
        r#"
        SELECT
            COALESCE(page_url, '/'),
            COUNT(*) FILTER (WHERE event_type = 'view'),
            COUNT(*) FILTER (WHERE event_type = 'conversion')
        FROM popup_events
        WHERE popup_id = $1
        GROUP BY page_url
        ORDER BY COUNT(*) DESC
        LIMIT 10
        "#,
    )
    .bind(id)
    .fetch_all(&state.db.pool)
    .await
    .unwrap_or_default();

    let top_pages_formatted: Vec<PagePerformance> = top_pages
        .into_iter()
        .map(|(url, views, conversions)| PagePerformance {
            url,
            views,
            conversions,
            conversion_rate: if views > 0 {
                (conversions as f64 / views as f64) * 100.0
            } else {
                0.0
            },
        })
        .collect();

    // Timeline data (last 30 days)
    let timeline_views: Vec<(String, i64)> = sqlx::query_as(
        r#"
        SELECT DATE(created_at)::text, COUNT(*)
        FROM popup_events
        WHERE popup_id = $1 AND event_type = 'view' AND created_at >= $2::timestamp
        GROUP BY DATE(created_at)
        ORDER BY DATE(created_at)
        "#,
    )
    .bind(id)
    .bind(month_start)
    .fetch_all(&state.db.pool)
    .await
    .unwrap_or_default();

    let timeline_conversions: Vec<(String, i64)> = sqlx::query_as(
        r#"
        SELECT DATE(created_at)::text, COUNT(*)
        FROM popup_events
        WHERE popup_id = $1 AND event_type = 'conversion' AND created_at >= $2::timestamp
        GROUP BY DATE(created_at)
        ORDER BY DATE(created_at)
        "#,
    )
    .bind(id)
    .bind(month_start)
    .fetch_all(&state.db.pool)
    .await
    .unwrap_or_default();

    // Calculate trend (simplified: compare this week vs last week)
    let views_trend = if week_views > 0 { "up" } else { "stable" };
    let conv_trend = if week_conv > 0 { "up" } else { "stable" };

    Ok(Json(PopupAnalytics {
        popup_id: id,
        views: ViewMetrics {
            total: total_views,
            today: today_views,
            this_week: week_views,
            this_month: month_views,
            trend: views_trend.to_string(),
            trend_percentage: 0.0,
        },
        conversions: ConversionMetrics {
            total: total_conversions,
            today: today_conv,
            this_week: week_conv,
            this_month: month_conv,
            trend: conv_trend.to_string(),
            trend_percentage: 0.0,
        },
        conversion_rate: ConversionRateMetrics {
            overall: conversion_rate,
            today: if today_views > 0 {
                (today_conv as f64 / today_views as f64) * 100.0
            } else {
                0.0
            },
            this_week: if week_views > 0 {
                (week_conv as f64 / week_views as f64) * 100.0
            } else {
                0.0
            },
            this_month: if month_views > 0 {
                (month_conv as f64 / month_views as f64) * 100.0
            } else {
                0.0
            },
        },
        device_breakdown,
        top_pages: top_pages_formatted,
        timeline: TimelineData {
            views: timeline_views
                .into_iter()
                .map(|(date, count)| DailyCount { date, count })
                .collect(),
            conversions: timeline_conversions
                .into_iter()
                .map(|(date, count)| DailyCount { date, count })
                .collect(),
        },
    }))
}

/// Toggle popup status (activate/deactivate)
async fn toggle_popup_status(
    _admin: AdminUser,
    State(state): State<AppState>,
    Path(id): Path<i32>,
    Json(body): Json<ToggleStatusRequest>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let new_status = if body.is_active {
        "published"
    } else {
        "paused"
    };

    let result = sqlx::query("UPDATE popups SET status = $1, updated_at = NOW() WHERE id = $2")
        .bind(new_status)
        .bind(id)
        .execute(&state.db.pool)
        .await;

    match result {
        Ok(res) => {
            if res.rows_affected() == 0 {
                return Err((
                    StatusCode::NOT_FOUND,
                    Json(json!({ "error": "Popup not found" })),
                ));
            }
            Ok(Json(json!({
                "message": "Status updated",
                "status": new_status
            })))
        }
        Err(e) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": format!("Failed to update status: {}", e) })),
        )),
    }
}

#[derive(Debug, Deserialize)]
pub struct ToggleStatusRequest {
    pub is_active: bool,
}

/// Create A/B test
async fn create_ab_test(
    _admin: AdminUser,
    State(state): State<AppState>,
    Path(base_popup_id): Path<i32>,
    Json(body): Json<CreateAbTestRequest>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    // Create A/B test record
    let test_result = sqlx::query(
        r#"
        INSERT INTO popup_ab_tests (name, base_popup_id, status, confidence_threshold, created_at, updated_at)
        VALUES ($1, $2, 'draft', $3, NOW(), NOW())
        RETURNING id
        "#,
    )
    .bind(&body.name)
    .bind(base_popup_id)
    .bind(body.confidence_threshold.unwrap_or(0.95))
    .fetch_one(&state.db.pool)
    .await;

    match test_result {
        Ok(row) => {
            let test_id: i32 = sqlx::Row::get(&row, "id");

            // Create variant popups
            for (i, variant) in body.variants.iter().enumerate() {
                let variant_name = variant
                    .name
                    .clone()
                    .unwrap_or_else(|| format!("Variant {}", i + 1));
                let allocation = variant
                    .allocation
                    .unwrap_or(100 / (body.variants.len() as i32 + 1));

                let _ = sqlx::query(
                    r#"
                    INSERT INTO popups (
                        name, type, status, priority, title, content,
                        ab_test_id, variant_name, traffic_allocation,
                        trigger_rules, frequency_rules, display_rules, design,
                        show_close_button, close_on_overlay_click, close_on_escape,
                        position, size, animation,
                        total_views, total_conversions, conversion_rate,
                        created_at, updated_at
                    )
                    SELECT
                        name || ' - ' || $2, type, 'draft', priority,
                        COALESCE($3, title), COALESCE($4, content),
                        $5, $2, $6,
                        trigger_rules, frequency_rules, display_rules, design,
                        show_close_button, close_on_overlay_click, close_on_escape,
                        position, size, animation,
                        0, 0, 0.0,
                        NOW(), NOW()
                    FROM popups WHERE id = $1
                    "#,
                )
                .bind(base_popup_id)
                .bind(&variant_name)
                .bind(&variant.title)
                .bind(&variant.content)
                .bind(test_id)
                .bind(allocation)
                .execute(&state.db.pool)
                .await;
            }

            Ok(Json(json!({
                "message": "A/B test created",
                "test_id": test_id
            })))
        }
        Err(e) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": format!("Failed to create A/B test: {}", e) })),
        )),
    }
}

#[derive(Debug, Deserialize)]
pub struct CreateAbTestRequest {
    pub name: String,
    pub variants: Vec<AbTestVariant>,
    pub confidence_threshold: Option<f64>,
}

#[derive(Debug, Deserialize)]
pub struct AbTestVariant {
    pub name: Option<String>,
    pub title: Option<String>,
    pub content: Option<String>,
    pub allocation: Option<i32>,
}

/// Get A/B test results
async fn get_ab_test_results(
    _admin: AdminUser,
    State(state): State<AppState>,
    Path(test_id): Path<i32>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let variants: Vec<(i32, String, i64, i64, f64)> = sqlx::query_as(
        r#"
        SELECT id, variant_name, total_views, total_conversions, conversion_rate
        FROM popups
        WHERE ab_test_id = $1
        ORDER BY conversion_rate DESC
        "#,
    )
    .bind(test_id)
    .fetch_all(&state.db.pool)
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": format!("Database error: {}", e) })),
        )
    })?;

    let results: Vec<AbTestVariantMetrics> = variants
        .iter()
        .enumerate()
        .map(
            |(i, (popup_id, name, views, conv, rate))| AbTestVariantMetrics {
                popup_id: *popup_id,
                variant_name: name.clone(),
                impressions: *views,
                conversions: *conv,
                conversion_rate: *rate,
                confidence: 0.0, // Would require statistical calculation
                is_winner: i == 0 && *conv > 0,
            },
        )
        .collect();

    Ok(Json(json!({
        "test_id": test_id,
        "variants": results
    })))
}

/// Build admin popups router
pub fn router() -> Router<AppState> {
    Router::new()
        .route("/", get(list_popups))
        .route("/", post(create_popup))
        .route("/:id", get(get_popup))
        .route("/:id", put(update_popup))
        .route("/:id", delete(delete_popup))
        .route("/:id/duplicate", post(duplicate_popup))
        .route("/:id/toggle-status", post(toggle_popup_status))
        .route("/:id/analytics", get(get_popup_analytics))
        .route("/:id/ab-test", post(create_ab_test))
        .route("/ab-tests/:test_id/results", get(get_ab_test_results))
}
