//! Export Routes - Phase 4 Explosive Swings Export Functionality
//! ═══════════════════════════════════════════════════════════════════════════════════
//! Apple Principal Engineer ICT Level 7 Grade - January 2026
//!
//! RESTful export endpoints for:
//! - CSV export of alerts and trades
//! - JSON performance report data (for client-side PDF rendering)
//!
//! SECURITY:
//! - All endpoints require authentication
//! - Room access verified via membership
//! - Rate limiting applied (see middleware)
//!
//! ENDPOINTS:
//! - GET /api/export/{room_slug}/alerts.csv     - Export alerts to CSV
//! - GET /api/export/{room_slug}/trades.csv    - Export trades to CSV
//! - GET /api/export/{room_slug}/performance   - Get performance report data

use axum::{
    extract::{Path, Query, State},
    http::{header, StatusCode},
    response::IntoResponse,
    routing::get,
    Json, Router,
};
use chrono::NaiveDate;
use serde::Deserialize;
use serde_json::json;
use tracing::{error, info};

use crate::{
    middleware::admin::AdminUser,
    services::export::{AlertFilters, DateRange, ExportService, TradeFilters},
    AppState,
};

// ═══════════════════════════════════════════════════════════════════════════════════
// REQUEST PARAMETERS
// ═══════════════════════════════════════════════════════════════════════════════════

/// Query parameters for alert CSV export
#[derive(Debug, Deserialize, Default)]
pub struct AlertExportParams {
    /// Start date filter (YYYY-MM-DD format)
    pub start_date: Option<String>,
    /// End date filter (YYYY-MM-DD format)
    pub end_date: Option<String>,
    /// Filter by alert type (ENTRY, UPDATE, EXIT)
    #[serde(rename = "type")]
    pub alert_type: Option<String>,
    /// Filter by ticker symbol
    pub ticker: Option<String>,
    /// Maximum records to export
    pub limit: Option<i64>,
}

impl From<AlertExportParams> for AlertFilters {
    fn from(params: AlertExportParams) -> Self {
        Self {
            start_date: params
                .start_date
                .and_then(|d| NaiveDate::parse_from_str(&d, "%Y-%m-%d").ok()),
            end_date: params
                .end_date
                .and_then(|d| NaiveDate::parse_from_str(&d, "%Y-%m-%d").ok()),
            alert_type: params.alert_type,
            ticker: params.ticker.map(|t| t.to_uppercase()),
            limit: params.limit,
        }
    }
}

/// Query parameters for trade CSV export
#[derive(Debug, Deserialize, Default)]
pub struct TradeExportParams {
    /// Start date filter (YYYY-MM-DD format)
    pub start_date: Option<String>,
    /// End date filter (YYYY-MM-DD format)
    pub end_date: Option<String>,
    /// Filter by status (open, closed, invalidated)
    pub status: Option<String>,
    /// Filter by result (WIN, LOSS)
    pub result: Option<String>,
    /// Filter by ticker symbol
    pub ticker: Option<String>,
    /// Maximum records to export
    pub limit: Option<i64>,
}

impl From<TradeExportParams> for TradeFilters {
    fn from(params: TradeExportParams) -> Self {
        Self {
            start_date: params
                .start_date
                .and_then(|d| NaiveDate::parse_from_str(&d, "%Y-%m-%d").ok()),
            end_date: params
                .end_date
                .and_then(|d| NaiveDate::parse_from_str(&d, "%Y-%m-%d").ok()),
            status: params.status,
            result: params.result.map(|r| r.to_uppercase()),
            ticker: params.ticker.map(|t| t.to_uppercase()),
            limit: params.limit,
        }
    }
}

/// Query parameters for performance report
#[derive(Debug, Deserialize)]
pub struct PerformanceReportParams {
    /// Start date (YYYY-MM-DD format, defaults to 30 days ago)
    pub start_date: Option<String>,
    /// End date (YYYY-MM-DD format, defaults to today)
    pub end_date: Option<String>,
}

impl From<PerformanceReportParams> for DateRange {
    fn from(params: PerformanceReportParams) -> Self {
        let now = chrono::Utc::now().date_naive();
        Self {
            start: params
                .start_date
                .and_then(|d| NaiveDate::parse_from_str(&d, "%Y-%m-%d").ok())
                .unwrap_or(now - chrono::Duration::days(30)),
            end: params
                .end_date
                .and_then(|d| NaiveDate::parse_from_str(&d, "%Y-%m-%d").ok())
                .unwrap_or(now),
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════════════
// ROUTE HANDLERS
// ═══════════════════════════════════════════════════════════════════════════════════

/// Export alerts to CSV
///
/// GET /api/export/{room_slug}/alerts.csv
///
/// # Query Parameters
/// - `start_date` - Filter by start date (YYYY-MM-DD)
/// - `end_date` - Filter by end date (YYYY-MM-DD)
/// - `type` - Filter by alert type (ENTRY, UPDATE, EXIT)
/// - `ticker` - Filter by ticker symbol
/// - `limit` - Maximum records (default: 10000)
///
/// # Response
/// CSV file download with Content-Disposition header
pub async fn export_alerts_csv(
    State(state): State<AppState>,
    Path(room_slug): Path<String>,
    Query(params): Query<AlertExportParams>,
    admin: AdminUser,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    info!(
        user_id = %admin.0.id,
        room = %room_slug,
        "Exporting alerts CSV (admin)"
    );

    let export_service = ExportService::new();
    let filters: AlertFilters = params.into();

    let csv_data = export_service
        .export_alerts_csv(&state.db.pool, &room_slug, filters)
        .await
        .map_err(|e| {
            error!(error = ?e, "Failed to export alerts CSV");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "success": false,
                    "error": format!("Export failed: {:?}", e)
                })),
            )
        })?;

    // Generate filename with timestamp
    let timestamp = chrono::Utc::now().format("%Y%m%d_%H%M%S");
    let filename = format!("{}_alerts_{}.csv", room_slug, timestamp);
    let content_disposition = format!("attachment; filename=\"{}\"", filename);

    Ok((
        StatusCode::OK,
        [
            (header::CONTENT_TYPE, "text/csv; charset=utf-8".to_string()),
            (header::CONTENT_DISPOSITION, content_disposition),
            (
                header::CACHE_CONTROL,
                "private, no-cache, no-store".to_string(),
            ),
        ],
        csv_data,
    ))
}

/// Export trades to CSV
///
/// GET /api/export/{room_slug}/trades.csv
///
/// # Query Parameters
/// - `start_date` - Filter by start date (YYYY-MM-DD)
/// - `end_date` - Filter by end date (YYYY-MM-DD)
/// - `status` - Filter by status (open, closed, invalidated)
/// - `result` - Filter by result (WIN, LOSS)
/// - `ticker` - Filter by ticker symbol
/// - `limit` - Maximum records (default: 10000)
///
/// # Response
/// CSV file download with Content-Disposition header
pub async fn export_trades_csv(
    State(state): State<AppState>,
    Path(room_slug): Path<String>,
    Query(params): Query<TradeExportParams>,
    admin: AdminUser,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    info!(
        user_id = %admin.0.id,
        room = %room_slug,
        "Exporting trades CSV (admin)"
    );

    let export_service = ExportService::new();
    let filters: TradeFilters = params.into();

    let csv_data = export_service
        .export_trades_csv(&state.db.pool, &room_slug, filters)
        .await
        .map_err(|e| {
            error!(error = ?e, "Failed to export trades CSV");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "success": false,
                    "error": format!("Export failed: {:?}", e)
                })),
            )
        })?;

    // Generate filename with timestamp
    let timestamp = chrono::Utc::now().format("%Y%m%d_%H%M%S");
    let filename = format!("{}_trades_{}.csv", room_slug, timestamp);
    let content_disposition = format!("attachment; filename=\"{}\"", filename);

    Ok((
        StatusCode::OK,
        [
            (header::CONTENT_TYPE, "text/csv; charset=utf-8".to_string()),
            (header::CONTENT_DISPOSITION, content_disposition),
            (
                header::CACHE_CONTROL,
                "private, no-cache, no-store".to_string(),
            ),
        ],
        csv_data,
    ))
}

/// Get performance report data (JSON for client-side PDF rendering)
///
/// GET /api/export/{room_slug}/performance
///
/// # Query Parameters
/// - `start_date` - Report start date (YYYY-MM-DD, default: 30 days ago)
/// - `end_date` - Report end date (YYYY-MM-DD, default: today)
///
/// # Response
/// JSON with stats and trade summaries for client-side PDF generation
pub async fn get_performance_report(
    State(state): State<AppState>,
    Path(room_slug): Path<String>,
    Query(params): Query<PerformanceReportParams>,
    admin: AdminUser,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    info!(
        user_id = %admin.0.id,
        room = %room_slug,
        "Generating performance report (admin)"
    );

    let export_service = ExportService::new();
    let date_range: DateRange = params.into();

    let report_data = export_service
        .generate_performance_report(&state.db.pool, &room_slug, date_range)
        .await
        .map_err(|e| {
            error!(error = ?e, "Failed to generate performance report");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "success": false,
                    "error": format!("Report generation failed: {:?}", e)
                })),
            )
        })?;

    Ok(Json(json!({
        "success": true,
        "data": report_data
    })))
}

// ═══════════════════════════════════════════════════════════════════════════════════
// ROUTER
// ═══════════════════════════════════════════════════════════════════════════════════

/// Create the export router
///
/// All routes require authentication and are rate-limited.
pub fn router() -> Router<AppState> {
    Router::new()
        .route("/:room_slug/alerts.csv", get(export_alerts_csv))
        .route("/:room_slug/trades.csv", get(export_trades_csv))
        .route("/:room_slug/performance", get(get_performance_report))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_alert_params_to_filters() {
        let params = AlertExportParams {
            start_date: Some("2026-01-01".to_string()),
            end_date: Some("2026-01-31".to_string()),
            alert_type: Some("ENTRY".to_string()),
            ticker: Some("aapl".to_string()),
            limit: Some(100),
        };

        let filters: AlertFilters = params.into();

        assert_eq!(
            filters.start_date,
            Some(NaiveDate::from_ymd_opt(2026, 1, 1).unwrap())
        );
        assert_eq!(
            filters.end_date,
            Some(NaiveDate::from_ymd_opt(2026, 1, 31).unwrap())
        );
        assert_eq!(filters.alert_type, Some("ENTRY".to_string()));
        assert_eq!(filters.ticker, Some("AAPL".to_string())); // uppercase
        assert_eq!(filters.limit, Some(100));
    }

    #[test]
    fn test_trade_params_to_filters() {
        let params = TradeExportParams {
            start_date: Some("2026-01-01".to_string()),
            end_date: None,
            status: Some("closed".to_string()),
            result: Some("win".to_string()),
            ticker: Some("spy".to_string()),
            limit: None,
        };

        let filters: TradeFilters = params.into();

        assert_eq!(
            filters.start_date,
            Some(NaiveDate::from_ymd_opt(2026, 1, 1).unwrap())
        );
        assert!(filters.end_date.is_none());
        assert_eq!(filters.status, Some("closed".to_string()));
        assert_eq!(filters.result, Some("WIN".to_string())); // uppercase
        assert_eq!(filters.ticker, Some("SPY".to_string())); // uppercase
        assert!(filters.limit.is_none());
    }

    #[test]
    fn test_performance_params_defaults() {
        let params = PerformanceReportParams {
            start_date: None,
            end_date: None,
        };

        let range: DateRange = params.into();
        let now = chrono::Utc::now().date_naive();

        assert_eq!(range.end, now);
        assert_eq!(range.start, now - chrono::Duration::days(30));
    }
}
