//! Search routes

use axum::{
    Router,
    routing::get,
    extract::{Query, State},
    Json,
};
use serde::{Deserialize, Serialize};
use crate::AppState;

/// Search query parameters
#[derive(Debug, Deserialize)]
pub struct SearchQuery {
    pub q: String,
    #[serde(default = "default_limit")]
    pub limit: usize,
}

fn default_limit() -> usize {
    10
}

/// Search response
#[derive(Debug, Serialize)]
#[allow(dead_code)]
pub struct SearchResponse {
    pub courses: Vec<crate::services::search::SearchableCourse>,
    pub posts: Vec<crate::services::search::SearchablePost>,
    pub query: String,
}

/// Build the search router
pub fn router() -> Router<AppState> {
    Router::new()
        .route("/", get(search_all))
        .route("/courses", get(search_courses))
        .route("/posts", get(search_posts))
}

/// Global search across all indexes
async fn search_all(
    State(state): State<AppState>,
    Query(params): Query<SearchQuery>,
) -> Json<serde_json::Value> {
    let result = state.services.search.search_all(&params.q).await;

    match result {
        Ok(data) => Json(data),
        Err(e) => Json(serde_json::json!({
            "error": e.to_string(),
            "courses": [],
            "posts": []
        })),
    }
}

/// Search courses only
async fn search_courses(
    State(state): State<AppState>,
    Query(params): Query<SearchQuery>,
) -> Json<serde_json::Value> {
    let result = state.services.search.search_courses(&params.q, params.limit).await;

    match result {
        Ok(courses) => Json(serde_json::json!({
            "courses": courses,
            "query": params.q,
            "count": courses.len()
        })),
        Err(e) => Json(serde_json::json!({
            "error": e.to_string(),
            "courses": []
        })),
    }
}

/// Search posts only
async fn search_posts(
    State(state): State<AppState>,
    Query(params): Query<SearchQuery>,
) -> Json<serde_json::Value> {
    let result = state.services.search.search_posts(&params.q, params.limit).await;

    match result {
        Ok(posts) => Json(serde_json::json!({
            "posts": posts,
            "query": params.q,
            "count": posts.len()
        })),
        Err(e) => Json(serde_json::json!({
            "error": e.to_string(),
            "posts": []
        })),
    }
}
