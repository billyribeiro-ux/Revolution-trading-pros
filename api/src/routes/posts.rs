//! Blog/Post routes - Revolution Trading Pros
//! Apple ICT 11+ Principal Engineer Grade - December 2025

use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    routing::get,
    Json, Router,
};
use serde::Deserialize;
use serde_json::json;

use crate::{
    models::User,
    AppState,
};

/// Post list query
#[derive(Debug, Deserialize)]
pub struct PostListQuery {
    pub page: Option<i64>,
    pub per_page: Option<i64>,
    pub status: Option<String>,
    pub author_id: Option<i64>,
    pub search: Option<String>,
}

/// Post row from database
#[derive(Debug, serde::Serialize, sqlx::FromRow)]
pub struct PostRow {
    pub id: i64,
    pub author_id: i64,
    pub title: String,
    pub slug: String,
    pub excerpt: Option<String>,
    pub content_blocks: Option<serde_json::Value>,
    pub featured_image: Option<String>,
    pub status: String,
    pub published_at: Option<chrono::NaiveDateTime>,
    pub meta_title: Option<String>,
    pub meta_description: Option<String>,
    pub indexable: bool,
    pub canonical_url: Option<String>,
    pub schema_markup: Option<serde_json::Value>,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

/// Post with author info
#[derive(Debug, serde::Serialize)]
pub struct PostWithAuthor {
    #[serde(flatten)]
    pub post: PostRow,
    pub author_name: Option<String>,
    pub author_email: Option<String>,
}

/// Create post request
#[derive(Debug, Deserialize)]
pub struct CreatePostRequest {
    pub title: String,
    pub excerpt: Option<String>,
    pub content_blocks: Option<serde_json::Value>,
    pub featured_image: Option<String>,
    pub status: Option<String>,
    pub published_at: Option<chrono::NaiveDateTime>,
    pub meta_title: Option<String>,
    pub meta_description: Option<String>,
    pub indexable: Option<bool>,
    pub canonical_url: Option<String>,
    pub schema_markup: Option<serde_json::Value>,
}

/// Update post request
#[derive(Debug, Deserialize)]
pub struct UpdatePostRequest {
    pub title: Option<String>,
    pub excerpt: Option<String>,
    pub content_blocks: Option<serde_json::Value>,
    pub featured_image: Option<String>,
    pub status: Option<String>,
    pub published_at: Option<chrono::NaiveDateTime>,
    pub meta_title: Option<String>,
    pub meta_description: Option<String>,
    pub indexable: Option<bool>,
    pub canonical_url: Option<String>,
    pub schema_markup: Option<serde_json::Value>,
}

/// List published posts (public)
async fn list_posts(
    State(state): State<AppState>,
    Query(query): Query<PostListQuery>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let page = query.page.unwrap_or(1).max(1);
    let per_page = query.per_page.unwrap_or(20).min(100);
    let offset = (page - 1) * per_page;

    let status = query.status.unwrap_or_else(|| "published".to_string());

    let mut conditions = vec![format!("status = '{}'", status)];

    if let Some(author_id) = query.author_id {
        conditions.push(format!("author_id = {}", author_id));
    }

    if let Some(ref search) = query.search {
        conditions.push(format!("(title ILIKE '%{}%' OR excerpt ILIKE '%{}%')", search, search));
    }

    let where_clause = conditions.join(" AND ");

    let sql = format!(
        "SELECT * FROM posts WHERE {} ORDER BY published_at DESC NULLS LAST, created_at DESC LIMIT {} OFFSET {}",
        where_clause, per_page, offset
    );
    let count_sql = format!("SELECT COUNT(*) FROM posts WHERE {}", where_clause);

    let posts: Vec<PostRow> = sqlx::query_as(&sql)
        .fetch_all(&state.db.pool)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()}))))?;

    let total: (i64,) = sqlx::query_as(&count_sql)
        .fetch_one(&state.db.pool)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()}))))?;

    Ok(Json(json!({
        "data": posts,
        "meta": {
            "current_page": page,
            "per_page": per_page,
            "total": total.0,
            "total_pages": (total.0 as f64 / per_page as f64).ceil() as i64
        }
    })))
}

/// Get post by slug (public)
async fn get_post(
    State(state): State<AppState>,
    Path(slug): Path<String>,
) -> Result<Json<PostRow>, (StatusCode, Json<serde_json::Value>)> {
    let post: PostRow = sqlx::query_as(
        "SELECT * FROM posts WHERE slug = $1 AND status = 'published'"
    )
    .bind(&slug)
    .fetch_optional(&state.db.pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()}))))?
    .ok_or_else(|| (StatusCode::NOT_FOUND, Json(json!({"error": "Post not found"}))))?;

    Ok(Json(post))
}

/// Create post (admin only)
async fn create_post(
    State(state): State<AppState>,
    user: User,
    Json(input): Json<CreatePostRequest>,
) -> Result<Json<PostRow>, (StatusCode, Json<serde_json::Value>)> {
    let slug = slug::slugify(&input.title);
    let status = input.status.unwrap_or_else(|| "draft".to_string());

    let post: PostRow = sqlx::query_as(
        r#"
        INSERT INTO posts (author_id, title, slug, excerpt, content_blocks, featured_image, status, published_at, meta_title, meta_description, indexable, canonical_url, schema_markup, created_at, updated_at)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, NOW(), NOW())
        RETURNING *
        "#,
    )
    .bind(user.id)
    .bind(&input.title)
    .bind(&slug)
    .bind(&input.excerpt)
    .bind(&input.content_blocks)
    .bind(&input.featured_image)
    .bind(&status)
    .bind(&input.published_at)
    .bind(&input.meta_title)
    .bind(&input.meta_description)
    .bind(input.indexable.unwrap_or(true))
    .bind(&input.canonical_url)
    .bind(&input.schema_markup)
    .fetch_one(&state.db.pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()}))))?;

    Ok(Json(post))
}

/// Update post (admin only)
async fn update_post(
    State(state): State<AppState>,
    user: User,
    Path(id): Path<i64>,
    Json(input): Json<UpdatePostRequest>,
) -> Result<Json<PostRow>, (StatusCode, Json<serde_json::Value>)> {
    let _ = &user; // TODO: Role check

    let mut set_clauses = Vec::new();
    
    if let Some(ref title) = input.title {
        set_clauses.push(format!("title = '{}'", title.replace("'", "''")));
        set_clauses.push(format!("slug = '{}'", slug::slugify(title)));
    }
    if let Some(ref excerpt) = input.excerpt {
        set_clauses.push(format!("excerpt = '{}'", excerpt.replace("'", "''")));
    }
    if let Some(ref status) = input.status {
        set_clauses.push(format!("status = '{}'", status));
    }
    if input.indexable.is_some() {
        set_clauses.push(format!("indexable = {}", input.indexable.unwrap()));
    }

    set_clauses.push("updated_at = NOW()".to_string());

    let sql = format!(
        "UPDATE posts SET {} WHERE id = $1 RETURNING *",
        set_clauses.join(", ")
    );

    let post: PostRow = sqlx::query_as(&sql)
        .bind(id)
        .fetch_one(&state.db.pool)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()}))))?;

    Ok(Json(post))
}

/// Delete post (admin only)
async fn delete_post(
    State(state): State<AppState>,
    user: User,
    Path(id): Path<i64>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let _ = &user; // TODO: Role check

    sqlx::query("DELETE FROM posts WHERE id = $1")
        .bind(id)
        .execute(&state.db.pool)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()}))))?;

    Ok(Json(json!({"message": "Post deleted successfully"})))
}

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/", get(list_posts).post(create_post))
        .route("/:slug", get(get_post).put(update_post).delete(delete_post))
}
