//! Public posts routes

use worker::{Request, Response, RouteContext};
use crate::AppState;
use crate::error::ApiError;
use crate::models::post::{PostWithRelations, PostListQuery, PaginatedResponse, PaginationMeta};

/// GET /api/posts - List published posts
pub async fn list(req: Request, ctx: RouteContext<AppState>) -> worker::Result<Response> {
    let url = req.url()?;
    let query: PostListQuery = serde_urlencoded::from_str(url.query().unwrap_or(""))
        .unwrap_or_default();

    let page = query.page();
    let per_page = query.per_page();
    let offset = query.offset();

    // Build query with filters
    let mut sql = String::from(
        r#"
        SELECT p.*, 
               json_build_object('id', u.id, 'name', u.name, 'avatar_url', u.avatar_url) as author,
               json_build_object('id', c.id, 'name', c.name, 'slug', c.slug) as category
        FROM posts p
        LEFT JOIN users u ON u.id = p.author_id
        LEFT JOIN categories c ON c.id = p.category_id
        WHERE p.status = 'published'
        "#
    );

    let mut params: Vec<serde_json::Value> = vec![];
    let mut param_idx = 1;

    if let Some(category_id) = &query.category_id {
        sql.push_str(&format!(" AND p.category_id = ${}", param_idx));
        params.push(serde_json::json!(category_id.to_string()));
        param_idx += 1;
    }

    if let Some(search) = &query.search {
        sql.push_str(&format!(
            " AND (p.title ILIKE ${} OR p.content ILIKE ${})",
            param_idx, param_idx
        ));
        params.push(serde_json::json!(format!("%{}%", search)));
        param_idx += 1;
    }

    // Count total
    let count_sql = format!(
        "SELECT COUNT(*) as count FROM posts p WHERE p.status = 'published' {}",
        if query.category_id.is_some() { "AND p.category_id = $1" } else { "" }
    );
    
    let total: i64 = ctx.data.db.query_one::<CountResult>(&count_sql, params.clone())
        .await?
        .map(|r| r.count)
        .unwrap_or(0);

    // Add sorting and pagination
    let sort_by = query.sort_by.as_deref().unwrap_or("published_at");
    let sort_order = query.sort_order.as_deref().unwrap_or("desc");
    sql.push_str(&format!(" ORDER BY p.{} {}", sort_by, sort_order));
    sql.push_str(&format!(" LIMIT {} OFFSET {}", per_page, offset));

    let posts: Vec<PostWithRelations> = ctx.data.db.query(&sql, params).await?;

    let response = PaginatedResponse {
        data: posts,
        meta: PaginationMeta::new(page, per_page, total),
    };

    Response::from_json(&response)
}

/// GET /api/posts/:slug - Get a single post by slug
pub async fn show(_req: Request, ctx: RouteContext<AppState>) -> worker::Result<Response> {
    let slug = ctx.param("slug")
        .ok_or_else(|| ApiError::BadRequest("Missing slug".to_string()))?;

    let post: Option<PostWithRelations> = ctx.data.db.query_one(
        r#"
        SELECT p.*, 
               json_build_object('id', u.id, 'name', u.name, 'avatar_url', u.avatar_url) as author,
               json_build_object('id', c.id, 'name', c.name, 'slug', c.slug) as category,
               COALESCE(
                   (SELECT json_agg(json_build_object('id', t.id, 'name', t.name, 'slug', t.slug))
                    FROM tags t
                    JOIN post_tag pt ON pt.tag_id = t.id
                    WHERE pt.post_id = p.id),
                   '[]'
               ) as tags
        FROM posts p
        LEFT JOIN users u ON u.id = p.author_id
        LEFT JOIN categories c ON c.id = p.category_id
        WHERE p.slug = $1 AND p.status = 'published'
        "#,
        vec![serde_json::json!(slug)]
    ).await?;

    match post {
        Some(p) => {
            // Increment view count asynchronously
            let _ = ctx.data.db.execute(
                "UPDATE posts SET view_count = view_count + 1 WHERE slug = $1",
                vec![serde_json::json!(slug)]
            ).await;

            Response::from_json(&p)
        }
        None => Err(ApiError::NotFound("Post not found".to_string()).into())
    }
}

#[derive(serde::Deserialize)]
struct CountResult {
    count: i64,
}

impl Default for PostListQuery {
    fn default() -> Self {
        Self {
            page: Some(1),
            per_page: Some(10),
            status: None,
            category_id: None,
            tag_id: None,
            author_id: None,
            search: None,
            sort_by: None,
            sort_order: None,
        }
    }
}
