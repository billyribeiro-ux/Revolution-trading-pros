//! Blog/Post routes - Revolution Trading Pros
//! ICT Level 7 Principal Engineer Grade - February 2026
//!
//! Full blog post CRUD with admin authentication, SEO metadata,
//! draft/publish states, and related posts.

#![allow(clippy::needless_borrows_for_generic_args)]

use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    routing::{delete, get, post, put},
    Json, Router,
};
use serde::Deserialize;
use serde_json::json;

use crate::{middleware::admin::AdminUser, AppState};

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
    // Added in migration 045 — admin form sends these and now the API persists them.
    pub featured_media_id: Option<i64>,
    pub featured_image_alt: Option<String>,
    pub featured_image_caption: Option<String>,
    pub featured_image_title: Option<String>,
    pub featured_image_description: Option<String>,
    pub meta_keywords: Option<Vec<String>>,
    pub allow_comments: bool,
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
    // Featured-image metadata (matches frontend admin form fields).
    pub featured_media_id: Option<i64>,
    pub featured_image_alt: Option<String>,
    pub featured_image_caption: Option<String>,
    pub featured_image_title: Option<String>,
    pub featured_image_description: Option<String>,
    pub meta_keywords: Option<Vec<String>>,
    pub allow_comments: Option<bool>,
    // Taxonomy: category slugs (resolved server-side via the categories table).
    pub categories: Option<Vec<String>>,
    // Tag names (created on demand via tags table; joined via post_tags).
    pub tags: Option<Vec<String>>,
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
    pub featured_media_id: Option<i64>,
    pub featured_image_alt: Option<String>,
    pub featured_image_caption: Option<String>,
    pub featured_image_title: Option<String>,
    pub featured_image_description: Option<String>,
    pub meta_keywords: Option<Vec<String>>,
    pub allow_comments: Option<bool>,
    pub categories: Option<Vec<String>>,
    pub tags: Option<Vec<String>>,
}

/// Resolve category slugs to IDs and rewrite post_categories. Unknown slugs are
/// dropped silently; the admin UI uses a closed set seeded by migration 045.
async fn sync_post_categories(
    pool: &sqlx::PgPool,
    post_id: i64,
    slugs: &[String],
) -> Result<(), sqlx::Error> {
    sqlx::query("DELETE FROM post_categories WHERE post_id = $1")
        .bind(post_id)
        .execute(pool)
        .await?;
    if slugs.is_empty() {
        return Ok(());
    }
    let ids: Vec<i64> = sqlx::query_scalar("SELECT id FROM categories WHERE slug = ANY($1)")
        .bind(slugs)
        .fetch_all(pool)
        .await?;
    for cid in ids {
        sqlx::query("INSERT INTO post_categories (post_id, category_id) VALUES ($1, $2) ON CONFLICT DO NOTHING")
            .bind(post_id)
            .bind(cid)
            .execute(pool)
            .await?;
    }
    Ok(())
}

/// Resolve tag names to IDs (creating any that don't exist) and rewrite post_tags.
async fn sync_post_tags(
    pool: &sqlx::PgPool,
    post_id: i64,
    names: &[String],
) -> Result<(), sqlx::Error> {
    sqlx::query("DELETE FROM post_tags WHERE post_id = $1")
        .bind(post_id)
        .execute(pool)
        .await?;
    if names.is_empty() {
        return Ok(());
    }
    let mut ids: Vec<i64> = Vec::with_capacity(names.len());
    for name in names {
        let trimmed = name.trim();
        if trimmed.is_empty() {
            continue;
        }
        let tag_slug = slug::slugify(trimmed);
        // Upsert (create-if-missing). RETURNING from ON CONFLICT skips on no-op,
        // so do an explicit second SELECT to resolve the id either way.
        sqlx::query("INSERT INTO tags (name, slug) VALUES ($1, $2) ON CONFLICT (slug) DO NOTHING")
            .bind(trimmed)
            .bind(&tag_slug)
            .execute(pool)
            .await?;
        let id: Option<i64> = sqlx::query_scalar("SELECT id FROM tags WHERE slug = $1")
            .bind(&tag_slug)
            .fetch_optional(pool)
            .await?;
        if let Some(id) = id {
            ids.push(id);
        }
    }
    for tid in ids {
        sqlx::query(
            "INSERT INTO post_tags (post_id, tag_id) VALUES ($1, $2) ON CONFLICT DO NOTHING",
        )
        .bind(post_id)
        .bind(tid)
        .execute(pool)
        .await?;
    }
    Ok(())
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

    // Build query with parameterized bindings for security (ICT 11+ SQL injection prevention)
    let search_pattern = query
        .search
        .as_ref()
        .map(|s| format!("%{}%", s.replace('%', "\\%").replace('_', "\\_")));

    // Fetch posts with proper parameterized query
    let posts: Vec<PostRow> = if let Some(ref pattern) = search_pattern {
        if let Some(author_id) = query.author_id {
            sqlx::query_as(
                "SELECT * FROM posts WHERE status = $1 AND author_id = $2 AND (title ILIKE $3 OR excerpt ILIKE $3) ORDER BY published_at DESC NULLS LAST, created_at DESC LIMIT $4 OFFSET $5"
            )
            .bind(&status)
            .bind(author_id)
            .bind(pattern)
            .bind(per_page)
            .bind(offset)
            .fetch_all(&state.db.pool)
            .await
        } else {
            sqlx::query_as(
                "SELECT * FROM posts WHERE status = $1 AND (title ILIKE $2 OR excerpt ILIKE $2) ORDER BY published_at DESC NULLS LAST, created_at DESC LIMIT $3 OFFSET $4"
            )
            .bind(&status)
            .bind(pattern)
            .bind(per_page)
            .bind(offset)
            .fetch_all(&state.db.pool)
            .await
        }
    } else if let Some(author_id) = query.author_id {
        sqlx::query_as(
            "SELECT * FROM posts WHERE status = $1 AND author_id = $2 ORDER BY published_at DESC NULLS LAST, created_at DESC LIMIT $3 OFFSET $4"
        )
        .bind(&status)
        .bind(author_id)
        .bind(per_page)
        .bind(offset)
        .fetch_all(&state.db.pool)
        .await
    } else {
        sqlx::query_as(
            "SELECT * FROM posts WHERE status = $1 ORDER BY published_at DESC NULLS LAST, created_at DESC LIMIT $2 OFFSET $3"
        )
        .bind(&status)
        .bind(per_page)
        .bind(offset)
        .fetch_all(&state.db.pool)
        .await
    }.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()}))))?;

    // Count total with same filters
    let total: (i64,) = if let Some(ref pattern) = search_pattern {
        if let Some(author_id) = query.author_id {
            sqlx::query_as("SELECT COUNT(*) FROM posts WHERE status = $1 AND author_id = $2 AND (title ILIKE $3 OR excerpt ILIKE $3)")
                .bind(&status)
                .bind(author_id)
                .bind(pattern)
                .fetch_one(&state.db.pool)
                .await
        } else {
            sqlx::query_as("SELECT COUNT(*) FROM posts WHERE status = $1 AND (title ILIKE $2 OR excerpt ILIKE $2)")
                .bind(&status)
                .bind(pattern)
                .fetch_one(&state.db.pool)
                .await
        }
    } else if let Some(author_id) = query.author_id {
        sqlx::query_as("SELECT COUNT(*) FROM posts WHERE status = $1 AND author_id = $2")
            .bind(&status)
            .bind(author_id)
            .fetch_one(&state.db.pool)
            .await
    } else {
        sqlx::query_as("SELECT COUNT(*) FROM posts WHERE status = $1")
            .bind(&status)
            .fetch_one(&state.db.pool)
            .await
    }.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()}))))?;

    let last_page = (total.0 as f64 / per_page as f64).ceil() as i64;

    Ok(Json(json!({
        "data": posts,
        "current_page": page,
        "last_page": last_page,
        "per_page": per_page,
        "total": total.0,
        "links": {
            "first": null,
            "last": null,
            "prev": if page > 1 { Some(format!("/api/posts?page={}", page - 1)) } else { None::<String> },
            "next": if page < last_page { Some(format!("/api/posts?page={}", page + 1)) } else { None::<String> }
        }
    })))
}

/// Load category slugs and tag names joined to a post id.
async fn load_post_taxonomy(
    pool: &sqlx::PgPool,
    post_id: i64,
) -> Result<(Vec<String>, Vec<String>), sqlx::Error> {
    let categories: Vec<String> = sqlx::query_scalar(
        "SELECT c.slug FROM categories c
         JOIN post_categories pc ON pc.category_id = c.id
         WHERE pc.post_id = $1
         ORDER BY c.name",
    )
    .bind(post_id)
    .fetch_all(pool)
    .await?;
    let tags: Vec<String> = sqlx::query_scalar(
        "SELECT t.name FROM tags t
         JOIN post_tags pt ON pt.tag_id = t.id
         WHERE pt.post_id = $1
         ORDER BY t.name",
    )
    .bind(post_id)
    .fetch_all(pool)
    .await?;
    Ok((categories, tags))
}

/// Get post by slug (public)
async fn get_post(
    State(state): State<AppState>,
    Path(slug): Path<String>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let post: PostRow =
        sqlx::query_as("SELECT * FROM posts WHERE slug = $1 AND status = 'published'")
            .bind(&slug)
            .fetch_optional(&state.db.pool)
            .await
            .map_err(|e| {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({"error": e.to_string()})),
                )
            })?
            .ok_or_else(|| {
                (
                    StatusCode::NOT_FOUND,
                    Json(json!({"error": "Post not found"})),
                )
            })?;

    let (categories, tags) = load_post_taxonomy(&state.db.pool, post.id)
        .await
        .unwrap_or_default();
    let mut value = serde_json::to_value(post).unwrap_or_else(|_| json!({}));
    if let Some(obj) = value.as_object_mut() {
        obj.insert("categories".into(), serde_json::Value::from(categories));
        obj.insert("tags".into(), serde_json::Value::from(tags));
    }
    Ok(Json(value))
}

/// Create post (admin only) - ICT 7: AdminUser required + slug uniqueness
async fn create_post(
    admin: AdminUser,
    State(state): State<AppState>,
    Json(input): Json<CreatePostRequest>,
) -> Result<Json<PostRow>, (StatusCode, Json<serde_json::Value>)> {
    // ICT 7: Validate title
    if input.title.trim().is_empty() {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(json!({"error": "Title is required"})),
        ));
    }

    // Generate slug and ensure uniqueness
    let base_slug = slug::slugify(&input.title);

    // ICT 7: Check for slug uniqueness and append suffix if needed
    let mut final_slug = base_slug.clone();
    let mut counter = 1;
    loop {
        let exists: bool = sqlx::query_scalar("SELECT EXISTS(SELECT 1 FROM posts WHERE slug = $1)")
            .bind(&final_slug)
            .fetch_one(&state.db.pool)
            .await
            .map_err(|e| {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({"error": e.to_string()})),
                )
            })?;

        if !exists {
            break;
        }
        final_slug = format!("{base_slug}-{counter}");
        counter += 1;
        if counter > 100 {
            return Err((
                StatusCode::BAD_REQUEST,
                Json(json!({"error": "Could not generate unique slug"})),
            ));
        }
    }

    // Validate status
    let status = input.status.unwrap_or_else(|| "draft".to_string());
    if !["draft", "published", "archived"].contains(&status.as_str()) {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(json!({"error": "Invalid status. Must be draft, published, or archived"})),
        ));
    }

    // FIX-2026-04-27: auto-stamp published_at when creating a published post
    // without an explicit timestamp. Without this, published rows had NULL
    // published_at, breaking date-aware sorting and JSON-LD article schemas.
    let published_at = match (&status[..], input.published_at) {
        ("published", None) => Some(chrono::Utc::now().naive_utc()),
        (_, explicit) => explicit,
    };

    let post: PostRow = sqlx::query_as(
        r"
        INSERT INTO posts (
            author_id, title, slug, excerpt, content_blocks, featured_image,
            status, published_at, meta_title, meta_description, indexable,
            canonical_url, schema_markup,
            featured_media_id, featured_image_alt, featured_image_caption,
            featured_image_title, featured_image_description, meta_keywords,
            allow_comments,
            created_at, updated_at
        )
        VALUES (
            $1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13,
            $14, $15, $16, $17, $18, $19, $20,
            NOW(), NOW()
        )
        RETURNING *
        ",
    )
    .bind(admin.0.id)
    .bind(&input.title)
    .bind(&final_slug)
    .bind(&input.excerpt)
    .bind(&input.content_blocks)
    .bind(&input.featured_image)
    .bind(&status)
    .bind(&published_at)
    .bind(&input.meta_title)
    .bind(&input.meta_description)
    .bind(input.indexable.unwrap_or(true))
    .bind(&input.canonical_url)
    .bind(&input.schema_markup)
    .bind(&input.featured_media_id)
    .bind(&input.featured_image_alt)
    .bind(&input.featured_image_caption)
    .bind(&input.featured_image_title)
    .bind(&input.featured_image_description)
    .bind(&input.meta_keywords)
    .bind(input.allow_comments.unwrap_or(true))
    .fetch_one(&state.db.pool)
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": e.to_string()})),
        )
    })?;

    // Resolve & write categories/tags joins (best-effort: a join failure should
    // not roll back a successful post insert; surface the error if it fails).
    if let Some(ref cats) = input.categories {
        sync_post_categories(&state.db.pool, post.id, cats)
            .await
            .map_err(|e| {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({"error": format!("category sync failed: {}", e)})),
                )
            })?;
    }
    if let Some(ref tags) = input.tags {
        sync_post_tags(&state.db.pool, post.id, tags)
            .await
            .map_err(|e| {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({"error": format!("tag sync failed: {}", e)})),
                )
            })?;
    }

    Ok(Json(post))
}

/// Update post (admin only) - ICT 7: AdminUser required + slug uniqueness
async fn update_post(
    _admin: AdminUser,
    State(state): State<AppState>,
    Path(id): Path<i64>,
    Json(input): Json<UpdatePostRequest>,
) -> Result<Json<PostRow>, (StatusCode, Json<serde_json::Value>)> {
    // Get current post first
    let current: PostRow = sqlx::query_as("SELECT * FROM posts WHERE id = $1")
        .bind(id)
        .fetch_optional(&state.db.pool)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": e.to_string()})),
            )
        })?
        .ok_or_else(|| {
            (
                StatusCode::NOT_FOUND,
                Json(json!({"error": "Post not found"})),
            )
        })?;

    // Determine the new title and generate slug if title changed
    let title = input.title.unwrap_or(current.title.clone());
    let title_changed = title != current.title;

    // ICT 7: Only regenerate slug if title changed, and ensure uniqueness
    let final_slug = if title_changed {
        let base_slug = slug::slugify(&title);
        let mut candidate_slug = base_slug.clone();
        let mut counter = 1;
        loop {
            let exists: bool = sqlx::query_scalar(
                "SELECT EXISTS(SELECT 1 FROM posts WHERE slug = $1 AND id != $2)",
            )
            .bind(&candidate_slug)
            .bind(id)
            .fetch_one(&state.db.pool)
            .await
            .map_err(|e| {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({"error": e.to_string()})),
                )
            })?;

            if !exists {
                break;
            }
            candidate_slug = format!("{base_slug}-{counter}");
            counter += 1;
            if counter > 100 {
                return Err((
                    StatusCode::BAD_REQUEST,
                    Json(json!({"error": "Could not generate unique slug"})),
                ));
            }
        }
        candidate_slug
    } else {
        current.slug.clone()
    };

    // Validate status if provided
    let prev_status = current.status.clone();
    let status = input.status.unwrap_or(current.status);
    if !["draft", "published", "archived"].contains(&status.as_str()) {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(json!({"error": "Invalid status. Must be draft, published, or archived"})),
        ));
    }

    let excerpt = input.excerpt.or(current.excerpt);
    let indexable = input.indexable.unwrap_or(current.indexable);
    let content_blocks = input.content_blocks.or(current.content_blocks);
    let featured_image = input.featured_image.or(current.featured_image);
    let meta_title = input.meta_title.or(current.meta_title);
    let meta_description = input.meta_description.or(current.meta_description);
    let canonical_url = input.canonical_url.or(current.canonical_url);
    let schema_markup = input.schema_markup.or(current.schema_markup);
    // FIX-2026-04-27: when transitioning TO 'published' from a non-published status,
    // and there's no published_at on either side, stamp it now. Same rationale as
    // create_post — keeps date-aware queries and Article schema correct.
    let published_at = {
        let merged = input.published_at.or(current.published_at);
        if status == "published" && prev_status != "published" && merged.is_none() {
            Some(chrono::Utc::now().naive_utc())
        } else {
            merged
        }
    };
    let featured_media_id = input.featured_media_id.or(current.featured_media_id);
    let featured_image_alt = input.featured_image_alt.or(current.featured_image_alt);
    let featured_image_caption = input
        .featured_image_caption
        .or(current.featured_image_caption);
    let featured_image_title = input.featured_image_title.or(current.featured_image_title);
    let featured_image_description = input
        .featured_image_description
        .or(current.featured_image_description);
    let meta_keywords = input.meta_keywords.or(current.meta_keywords);
    let allow_comments = input.allow_comments.unwrap_or(current.allow_comments);

    let post: PostRow = sqlx::query_as(
        r"UPDATE posts SET
            title = $1, slug = $2, excerpt = $3, status = $4, indexable = $5,
            content_blocks = $6, featured_image = $7, meta_title = $8,
            meta_description = $9, canonical_url = $10, schema_markup = $11,
            published_at = $12,
            featured_media_id = $13, featured_image_alt = $14,
            featured_image_caption = $15, featured_image_title = $16,
            featured_image_description = $17, meta_keywords = $18,
            allow_comments = $19,
            updated_at = NOW()
        WHERE id = $20 RETURNING *",
    )
    .bind(&title)
    .bind(&final_slug)
    .bind(&excerpt)
    .bind(&status)
    .bind(indexable)
    .bind(&content_blocks)
    .bind(&featured_image)
    .bind(&meta_title)
    .bind(&meta_description)
    .bind(&canonical_url)
    .bind(&schema_markup)
    .bind(&published_at)
    .bind(&featured_media_id)
    .bind(&featured_image_alt)
    .bind(&featured_image_caption)
    .bind(&featured_image_title)
    .bind(&featured_image_description)
    .bind(&meta_keywords)
    .bind(allow_comments)
    .bind(id)
    .fetch_one(&state.db.pool)
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": e.to_string()})),
        )
    })?;

    // Resolve & rewrite categories/tags joins. None means "leave as-is";
    // an explicit empty array clears them.
    if let Some(ref cats) = input.categories {
        sync_post_categories(&state.db.pool, post.id, cats)
            .await
            .map_err(|e| {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({"error": format!("category sync failed: {}", e)})),
                )
            })?;
    }
    if let Some(ref tags) = input.tags {
        sync_post_tags(&state.db.pool, post.id, tags)
            .await
            .map_err(|e| {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({"error": format!("tag sync failed: {}", e)})),
                )
            })?;
    }

    Ok(Json(post))
}

/// Delete post (admin only) - ICT 7: AdminUser required
async fn delete_post(
    _admin: AdminUser,
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    // ICT 7: Check if post exists first
    let exists: bool = sqlx::query_scalar("SELECT EXISTS(SELECT 1 FROM posts WHERE id = $1)")
        .bind(id)
        .fetch_one(&state.db.pool)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": e.to_string()})),
            )
        })?;

    if !exists {
        return Err((
            StatusCode::NOT_FOUND,
            Json(json!({"error": "Post not found"})),
        ));
    }

    // Delete associated post_tags first
    sqlx::query("DELETE FROM post_tags WHERE post_id = $1")
        .bind(id)
        .execute(&state.db.pool)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": e.to_string()})),
            )
        })?;

    // Delete the post
    sqlx::query("DELETE FROM posts WHERE id = $1")
        .bind(id)
        .execute(&state.db.pool)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": e.to_string()})),
            )
        })?;

    Ok(Json(json!({"message": "Post deleted successfully"})))
}

/// Get post by ID (admin only) - ICT 7: separate from public slug-based lookup
async fn get_post_by_id(
    _admin: AdminUser,
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let post: PostRow = sqlx::query_as("SELECT * FROM posts WHERE id = $1")
        .bind(id)
        .fetch_optional(&state.db.pool)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": e.to_string()})),
            )
        })?
        .ok_or_else(|| {
            (
                StatusCode::NOT_FOUND,
                Json(json!({"error": "Post not found"})),
            )
        })?;

    let (categories, tags) = load_post_taxonomy(&state.db.pool, post.id)
        .await
        .unwrap_or_default();
    let mut value = serde_json::to_value(post).unwrap_or_else(|_| json!({}));
    if let Some(obj) = value.as_object_mut() {
        obj.insert("categories".into(), serde_json::Value::from(categories));
        obj.insert("tags".into(), serde_json::Value::from(tags));
    }
    Ok(Json(value))
}

/// POST /admin/posts/:id/publish - Publish a post (ICT 7: draft/publish workflow)
async fn publish_post(
    _admin: AdminUser,
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> Result<Json<PostRow>, (StatusCode, Json<serde_json::Value>)> {
    let post: PostRow = sqlx::query_as(
        r"UPDATE posts SET
            status = 'published',
            published_at = COALESCE(published_at, NOW()),
            updated_at = NOW()
        WHERE id = $1 RETURNING *",
    )
    .bind(id)
    .fetch_optional(&state.db.pool)
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": e.to_string()})),
        )
    })?
    .ok_or_else(|| {
        (
            StatusCode::NOT_FOUND,
            Json(json!({"error": "Post not found"})),
        )
    })?;

    Ok(Json(post))
}

/// POST /admin/posts/:id/unpublish - Unpublish a post (ICT 7: draft/publish workflow)
async fn unpublish_post(
    _admin: AdminUser,
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> Result<Json<PostRow>, (StatusCode, Json<serde_json::Value>)> {
    let post: PostRow = sqlx::query_as(
        r"UPDATE posts SET status = 'draft', updated_at = NOW() WHERE id = $1 RETURNING *",
    )
    .bind(id)
    .fetch_optional(&state.db.pool)
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": e.to_string()})),
        )
    })?
    .ok_or_else(|| {
        (
            StatusCode::NOT_FOUND,
            Json(json!({"error": "Post not found"})),
        )
    })?;

    Ok(Json(post))
}

/// POST /admin/posts/:id/archive - Archive a post (ICT 7: draft/publish workflow)
async fn archive_post(
    _admin: AdminUser,
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> Result<Json<PostRow>, (StatusCode, Json<serde_json::Value>)> {
    let post: PostRow = sqlx::query_as(
        r"UPDATE posts SET status = 'archived', updated_at = NOW() WHERE id = $1 RETURNING *",
    )
    .bind(id)
    .fetch_optional(&state.db.pool)
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": e.to_string()})),
        )
    })?
    .ok_or_else(|| {
        (
            StatusCode::NOT_FOUND,
            Json(json!({"error": "Post not found"})),
        )
    })?;

    Ok(Json(post))
}

/// GET /posts/:slug/related - Get related posts (ICT 7: related posts functionality)
async fn get_related_posts(
    State(state): State<AppState>,
    Path(slug): Path<String>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    // First get the current post to find its category/tags
    let current: Option<PostRow> =
        sqlx::query_as("SELECT * FROM posts WHERE slug = $1 AND status = 'published'")
            .bind(&slug)
            .fetch_optional(&state.db.pool)
            .await
            .map_err(|e| {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({"error": e.to_string()})),
                )
            })?;

    let current = match current {
        Some(p) => p,
        None => {
            return Err((
                StatusCode::NOT_FOUND,
                Json(json!({"error": "Post not found"})),
            ))
        }
    };

    // Related posts = published posts that share a tag OR a category with the
    // current post, shared-tag matches ranked first.
    //
    // ROOT-CAUSE NOTE (audit P2-D): the prior query referenced `p.category_id`,
    // a column that does NOT exist — `posts` has no `category_id`; categories
    // are a many-to-many via the `post_categories(post_id, category_id)` join
    // table (001_initial §193). That made this query fail on EVERY request,
    // and the `.unwrap_or_default()` (whose own comment blamed "schema drift")
    // silently masked it as an empty result. Both defects are fixed here: the
    // query now joins `post_categories` correctly, and a real DB fault now
    // propagates via the same house 500 shape used elsewhere in this file
    // (e.g. `archive_post`) instead of being swallowed.
    let related: Vec<PostRow> = sqlx::query_as(
        r"
        SELECT DISTINCT p.* FROM posts p
        LEFT JOIN post_tags pt1 ON p.id = pt1.post_id
        LEFT JOIN post_tags pt2 ON pt1.tag_id = pt2.tag_id AND pt2.post_id = $1
        LEFT JOIN post_categories pc1 ON p.id = pc1.post_id
        LEFT JOIN post_categories pc2 ON pc1.category_id = pc2.category_id AND pc2.post_id = $1
        WHERE p.id != $1
          AND p.status = 'published'
          AND (
            pt2.post_id IS NOT NULL  -- Has a shared tag
            OR pc2.post_id IS NOT NULL  -- Has a shared category
          )
        ORDER BY
          CASE WHEN pt2.post_id IS NOT NULL THEN 0 ELSE 1 END,  -- Prioritize shared tags
          p.published_at DESC NULLS LAST
        LIMIT 5
        ",
    )
    .bind(current.id)
    .fetch_all(&state.db.pool)
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": e.to_string()})),
        )
    })?;

    // If no related posts found via tags/category, fall back to recent posts.
    let final_related = if related.is_empty() {
        sqlx::query_as(
            r"
            SELECT * FROM posts
            WHERE id != $1 AND status = 'published'
            ORDER BY published_at DESC NULLS LAST
            LIMIT 5
            ",
        )
        .bind(current.id)
        .fetch_all(&state.db.pool)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": e.to_string()})),
            )
        })?
    } else {
        related
    };

    Ok(Json(json!({ "data": final_related })))
}

/// Admin list posts (can see all statuses)
async fn admin_list_posts(
    _admin: AdminUser,
    State(state): State<AppState>,
    Query(query): Query<PostListQuery>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let page = query.page.unwrap_or(1).max(1);
    let per_page = query.per_page.unwrap_or(20).min(100);
    let offset = (page - 1) * per_page;

    // ICT 7: Parameterized search
    let search_pattern = query
        .search
        .as_ref()
        .map(|s| format!("%{}%", s.replace('%', "\\%").replace('_', "\\_")));

    // Admin can filter by any status (or see all)
    let posts: Vec<PostRow> = match (&query.status, &search_pattern, query.author_id) {
        (Some(status), Some(pattern), Some(author_id)) => {
            sqlx::query_as(
                "SELECT * FROM posts WHERE status = $1 AND author_id = $2 AND (title ILIKE $3 OR excerpt ILIKE $3) ORDER BY updated_at DESC LIMIT $4 OFFSET $5"
            )
            .bind(status)
            .bind(author_id)
            .bind(pattern)
            .bind(per_page)
            .bind(offset)
            .fetch_all(&state.db.pool)
            .await
        }
        (Some(status), Some(pattern), None) => {
            sqlx::query_as(
                "SELECT * FROM posts WHERE status = $1 AND (title ILIKE $2 OR excerpt ILIKE $2) ORDER BY updated_at DESC LIMIT $3 OFFSET $4"
            )
            .bind(status)
            .bind(pattern)
            .bind(per_page)
            .bind(offset)
            .fetch_all(&state.db.pool)
            .await
        }
        (Some(status), None, Some(author_id)) => {
            sqlx::query_as(
                "SELECT * FROM posts WHERE status = $1 AND author_id = $2 ORDER BY updated_at DESC LIMIT $3 OFFSET $4"
            )
            .bind(status)
            .bind(author_id)
            .bind(per_page)
            .bind(offset)
            .fetch_all(&state.db.pool)
            .await
        }
        (Some(status), None, None) => {
            sqlx::query_as(
                "SELECT * FROM posts WHERE status = $1 ORDER BY updated_at DESC LIMIT $2 OFFSET $3"
            )
            .bind(status)
            .bind(per_page)
            .bind(offset)
            .fetch_all(&state.db.pool)
            .await
        }
        (None, Some(pattern), Some(author_id)) => {
            sqlx::query_as(
                "SELECT * FROM posts WHERE author_id = $1 AND (title ILIKE $2 OR excerpt ILIKE $2) ORDER BY updated_at DESC LIMIT $3 OFFSET $4"
            )
            .bind(author_id)
            .bind(pattern)
            .bind(per_page)
            .bind(offset)
            .fetch_all(&state.db.pool)
            .await
        }
        (None, Some(pattern), None) => {
            sqlx::query_as(
                "SELECT * FROM posts WHERE (title ILIKE $1 OR excerpt ILIKE $1) ORDER BY updated_at DESC LIMIT $2 OFFSET $3"
            )
            .bind(pattern)
            .bind(per_page)
            .bind(offset)
            .fetch_all(&state.db.pool)
            .await
        }
        (None, None, Some(author_id)) => {
            sqlx::query_as(
                "SELECT * FROM posts WHERE author_id = $1 ORDER BY updated_at DESC LIMIT $2 OFFSET $3"
            )
            .bind(author_id)
            .bind(per_page)
            .bind(offset)
            .fetch_all(&state.db.pool)
            .await
        }
        (None, None, None) => {
            sqlx::query_as(
                "SELECT * FROM posts ORDER BY updated_at DESC LIMIT $1 OFFSET $2"
            )
            .bind(per_page)
            .bind(offset)
            .fetch_all(&state.db.pool)
            .await
        }
    }.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()}))))?;

    // Get total count
    let total: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM posts")
        .fetch_one(&state.db.pool)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": e.to_string()})),
            )
        })?;

    let last_page = (total.0 as f64 / per_page as f64).ceil() as i64;

    Ok(Json(json!({
        "data": posts,
        "current_page": page,
        "last_page": last_page,
        "per_page": per_page,
        "total": total.0
    })))
}

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/", get(list_posts))
        .route("/{slug}", get(get_post))
        .route("/{slug}/related", get(get_related_posts))
}

pub fn admin_router() -> Router<AppState> {
    Router::new()
        .route("/", get(admin_list_posts).post(create_post))
        .route(
            "/{id}",
            get(get_post_by_id).put(update_post).delete(delete_post),
        )
        .route("/{id}/publish", post(publish_post))
        .route("/{id}/unpublish", post(unpublish_post))
        .route("/{id}/archive", post(archive_post))
}
