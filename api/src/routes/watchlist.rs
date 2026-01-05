//! Weekly Watchlist API Routes - Revolution Trading Pros
//! ═══════════════════════════════════════════════════════════════════════════
//! Apple ICT 11+ Principal Engineer Grade - January 2026
//!
//! Complete CRUD API for Weekly Watchlist entries with date switcher support

use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    routing::{delete, get, post, put},
    Json, Router,
};
use chrono::NaiveDate;
use serde::Deserialize;
use serde_json::json;
use tracing::{error, info};

use crate::models::watchlist::*;
use crate::AppState;

// ═══════════════════════════════════════════════════════════════════════════
// QUERY PARAMETERS
// ═══════════════════════════════════════════════════════════════════════════

#[derive(Debug, Deserialize)]
pub struct ListQuery {
    page: Option<i64>,
    per_page: Option<i64>,
    status: Option<String>,
    search: Option<String>,
    room: Option<String>,
}

// ═══════════════════════════════════════════════════════════════════════════
// LIST ALL WATCHLIST ENTRIES
// ═══════════════════════════════════════════════════════════════════════════

async fn list_watchlist(
    State(state): State<AppState>,
    Query(query): Query<ListQuery>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let page = query.page.unwrap_or(1).max(1);
    let per_page = query.per_page.unwrap_or(20).min(100);
    let offset = (page - 1) * per_page;

    // Build query
    let mut sql = String::from(
        "SELECT * FROM watchlist_entries WHERE deleted_at IS NULL"
    );
    let mut count_sql = String::from(
        "SELECT COUNT(*) FROM watchlist_entries WHERE deleted_at IS NULL"
    );

    // Filter by status
    if let Some(status) = &query.status {
        let status_filter = format!(" AND status = '{}'", status.replace('\'', "''"));
        sql.push_str(&status_filter);
        count_sql.push_str(&status_filter);
    }

    // Filter by room
    if let Some(room) = &query.room {
        let room_filter = format!(" AND rooms @> '[\"{}\"]]'", room.replace('\'', "''"));
        sql.push_str(&room_filter);
        count_sql.push_str(&room_filter);
    }

    // Search
    if let Some(search) = &query.search {
        let search_term = search.replace('\'', "''");
        let search_filter = format!(
            " AND (title ILIKE '%{}%' OR trader ILIKE '%{}%' OR description ILIKE '%{}%')",
            search_term, search_term, search_term
        );
        sql.push_str(&search_filter);
        count_sql.push_str(&search_filter);
    }

    // Order by date (newest first)
    sql.push_str(" ORDER BY week_of DESC");
    sql.push_str(&format!(" LIMIT {} OFFSET {}", per_page, offset));

    // Execute queries
    let entries_result = sqlx::query_as::<_, WatchlistEntry>(&sql)
        .fetch_all(&state.db.pool)
        .await;

    let total_result: Result<(i64,), _> = sqlx::query_as(&count_sql)
        .fetch_one(&state.db.pool)
        .await;

    match (entries_result, total_result) {
        (Ok(entries), Ok((total,))) => {
            // Convert to response format and add navigation
            let mut responses: Vec<WatchlistResponse> = entries
                .iter()
                .map(|e| e.to_response())
                .collect();

            // Add previous/next navigation
            for (i, response) in responses.iter_mut().enumerate() {
                // Previous = older (higher index in sorted array)
                if i < entries.len() - 1 {
                    response.previous = Some(NavigationLink {
                        slug: entries[i + 1].slug.clone(),
                        title: entries[i + 1].title.clone(),
                    });
                }

                // Next = newer (lower index in sorted array)
                if i > 0 {
                    response.next = Some(NavigationLink {
                        slug: entries[i - 1].slug.clone(),
                        title: entries[i - 1].title.clone(),
                    });
                }
            }

            let last_page = (total as f64 / per_page as f64).ceil() as i64;

            info!("Listed {} watchlist entries (page {}/{})", responses.len(), page, last_page);

            Ok(Json(json!({
                "success": true,
                "data": responses,
                "pagination": {
                    "current_page": page,
                    "per_page": per_page,
                    "total": total,
                    "last_page": last_page
                }
            })))
        }
        (Err(e), _) | (_, Err(e)) => {
            error!("Failed to fetch watchlist entries: {}", e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"success": false, "message": "Failed to fetch watchlist entries"}))
            ))
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// GET SINGLE WATCHLIST ENTRY BY SLUG
// ═══════════════════════════════════════════════════════════════════════════

async fn get_watchlist(
    State(state): State<AppState>,
    Path(slug): Path<String>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let result = sqlx::query_as::<_, WatchlistEntry>(
        "SELECT * FROM watchlist_entries WHERE slug = $1 AND deleted_at IS NULL"
    )
    .bind(&slug)
    .fetch_optional(&state.db.pool)
    .await;

    match result {
        Ok(Some(entry)) => {
            info!("Retrieved watchlist entry: {}", slug);
            Ok(Json(json!({
                "success": true,
                "data": entry.to_response()
            })))
        }
        Ok(None) => {
            error!("Watchlist entry not found: {}", slug);
            Err((
                StatusCode::NOT_FOUND,
                Json(json!({"success": false, "message": "Watchlist entry not found"}))
            ))
        }
        Err(e) => {
            error!("Database error fetching watchlist {}: {}", slug, e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"success": false, "message": "Failed to fetch watchlist entry"}))
            ))
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// CREATE NEW WATCHLIST ENTRY
// ═══════════════════════════════════════════════════════════════════════════

async fn create_watchlist(
    State(state): State<AppState>,
    Json(body): Json<CreateWatchlistRequest>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    // Parse week_of date
    let week_of = match NaiveDate::parse_from_str(&body.week_of, "%Y-%m-%d") {
        Ok(date) => date,
        Err(_) => {
            return Err((
                StatusCode::BAD_REQUEST,
                Json(json!({"success": false, "message": "Invalid date format for weekOf. Expected YYYY-MM-DD"}))
            ));
        }
    };

    // Generate slug if not provided
    let slug = body.slug.clone().unwrap_or_else(|| {
        let date_str = week_of.format("%m%d%Y").to_string();
        let trader_slug = body.trader.to_lowercase().replace(' ', "-");
        format!("{}-{}", date_str, trader_slug)
    });

    // Generate date_posted
    let date_posted = week_of.format("%B %-d, %Y").to_string();

    // Generate subtitle
    let subtitle = format!("Week of {}", date_posted);

    // Default rooms to all if not specified
    let rooms = body.rooms.clone().unwrap_or_else(|| vec![
        "day-trading-room".to_string(),
        "swing-trading-room".to_string(),
        "small-account-mentorship".to_string(),
        "explosive-swings".to_string(),
        "spx-profit-pulse".to_string(),
        "high-octane-scanner".to_string(),
    ]);
    let rooms_json = serde_json::to_value(&rooms).unwrap();

    // Convert watchlist_dates to JSON
    let watchlist_dates_json = body.watchlist_dates
        .as_ref()
        .map(|dates| serde_json::to_value(dates).unwrap());

    // Insert into database
    let result = sqlx::query_as::<_, WatchlistEntry>(
        r#"
        INSERT INTO watchlist_entries (
            slug, title, subtitle, trader, trader_image,
            date_posted, week_of,
            video_url, video_poster, video_title,
            spreadsheet_url, watchlist_dates,
            description, status, rooms
        ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15)
        RETURNING *
        "#
    )
    .bind(&slug)
    .bind(&body.title)
    .bind(&subtitle)
    .bind(&body.trader)
    .bind(&body.trader_image)
    .bind(&date_posted)
    .bind(&week_of)
    .bind(body.video_src.as_ref().unwrap_or(&String::new()))
    .bind(&body.video_poster)
    .bind(&body.title)
    .bind(body.spreadsheet_src.as_ref().unwrap_or(&String::new()))
    .bind(&watchlist_dates_json)
    .bind(body.description.as_ref().unwrap_or(&format!("Week starting on {}.", date_posted)))
    .bind(body.status.as_ref().unwrap_or(&"draft".to_string()))
    .bind(&rooms_json)
    .fetch_one(&state.db.pool)
    .await;

    match result {
        Ok(entry) => {
            info!("Created watchlist entry: {}", slug);
            Ok(Json(json!({
                "success": true,
                "data": entry.to_response()
            })))
        }
        Err(e) => {
            if e.to_string().contains("duplicate key") {
                error!("Duplicate slug: {}", slug);
                Err((
                    StatusCode::CONFLICT,
                    Json(json!({"success": false, "message": "A watchlist entry with this slug already exists"}))
                ))
            } else {
                error!("Failed to create watchlist entry: {}", e);
                Err((
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({"success": false, "message": format!("Failed to create watchlist entry: {}", e)}))
                ))
            }
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// UPDATE WATCHLIST ENTRY
// ═══════════════════════════════════════════════════════════════════════════

async fn update_watchlist(
    State(state): State<AppState>,
    Path(slug): Path<String>,
    Json(body): Json<UpdateWatchlistRequest>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    // Build dynamic update query
    let mut updates = Vec::new();
    let mut bind_count = 1;

    if body.title.is_some() {
        updates.push(format!("title = ${}", bind_count));
        bind_count += 1;
    }
    if body.trader.is_some() {
        updates.push(format!("trader = ${}", bind_count));
        bind_count += 1;
    }
    if body.description.is_some() {
        updates.push(format!("description = ${}", bind_count));
        bind_count += 1;
    }
    if body.status.is_some() {
        updates.push(format!("status = ${}", bind_count));
        bind_count += 1;
    }

    if updates.is_empty() {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(json!({"success": false, "message": "No fields to update"}))
        ));
    }

    updates.push("updated_at = NOW()".to_string());

    let query_str = format!(
        "UPDATE watchlist_entries SET {} WHERE slug = ${} AND deleted_at IS NULL RETURNING *",
        updates.join(", "),
        bind_count
    );

    // Build query with bindings
    let mut query = sqlx::query_as::<_, WatchlistEntry>(&query_str);
    
    if let Some(title) = &body.title {
        query = query.bind(title);
    }
    if let Some(trader) = &body.trader {
        query = query.bind(trader);
    }
    if let Some(description) = &body.description {
        query = query.bind(description);
    }
    if let Some(status) = &body.status {
        query = query.bind(status);
    }
    query = query.bind(&slug);

    let result = query.fetch_optional(&state.db.pool).await;

    match result {
        Ok(Some(entry)) => {
            info!("Updated watchlist entry: {}", slug);
            Ok(Json(json!({
                "success": true,
                "data": entry.to_response()
            })))
        }
        Ok(None) => {
            error!("Watchlist entry not found for update: {}", slug);
            Err((
                StatusCode::NOT_FOUND,
                Json(json!({"success": false, "message": "Watchlist entry not found"}))
            ))
        }
        Err(e) => {
            error!("Failed to update watchlist entry {}: {}", slug, e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"success": false, "message": "Failed to update watchlist entry"}))
            ))
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// DELETE WATCHLIST ENTRY (SOFT DELETE)
// ═══════════════════════════════════════════════════════════════════════════

async fn delete_watchlist(
    State(state): State<AppState>,
    Path(slug): Path<String>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let result = sqlx::query(
        "UPDATE watchlist_entries SET deleted_at = NOW() WHERE slug = $1 AND deleted_at IS NULL"
    )
    .bind(&slug)
    .execute(&state.db.pool)
    .await;

    match result {
        Ok(result) if result.rows_affected() > 0 => {
            info!("Deleted watchlist entry: {}", slug);
            Ok(Json(json!({
                "success": true,
                "message": "Watchlist entry deleted successfully"
            })))
        }
        Ok(_) => {
            error!("Watchlist entry not found for deletion: {}", slug);
            Err((
                StatusCode::NOT_FOUND,
                Json(json!({"success": false, "message": "Watchlist entry not found"}))
            ))
        }
        Err(e) => {
            error!("Failed to delete watchlist entry {}: {}", slug, e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"success": false, "message": "Failed to delete watchlist entry"}))
            ))
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// ROUTER
// ═══════════════════════════════════════════════════════════════════════════

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/watchlist", get(list_watchlist))
        .route("/watchlist/:slug", get(get_watchlist))
        .route("/admin/watchlist", post(create_watchlist))
        .route("/watchlist/:slug", put(update_watchlist))
        .route("/watchlist/:slug", delete(delete_watchlist))
}
