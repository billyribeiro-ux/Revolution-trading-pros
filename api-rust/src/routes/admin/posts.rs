//! Admin posts routes

use worker::{Request, Response, RouteContext};
use crate::AppState;
use crate::error::ApiError;
use crate::models::post::{Post, PostWithRelations, CreatePostRequest, UpdatePostRequest, PostListQuery, PaginatedResponse, PaginationMeta};
use crate::middleware::auth::require_admin;

/// GET /api/admin/posts - List all posts (admin)
pub async fn list(req: Request, ctx: RouteContext<AppState>) -> worker::Result<Response> {
    require_admin(&req, &ctx).await?;

    let url = req.url()?;
    let query: PostListQuery = serde_urlencoded::from_str(url.query().unwrap_or(""))
        .unwrap_or_default();

    let page = query.page();
    let per_page = query.per_page();
    let offset = query.offset();

    let mut sql = String::from(
        r#"
        SELECT p.*, 
               json_build_object('id', u.id, 'name', u.name) as author,
               json_build_object('id', c.id, 'name', c.name, 'slug', c.slug) as category
        FROM posts p
        LEFT JOIN users u ON u.id = p.author_id
        LEFT JOIN categories c ON c.id = p.category_id
        WHERE 1=1
        "#
    );

    let mut params: Vec<serde_json::Value> = vec![];
    let param_idx = 1;

    if let Some(status) = &query.status {
        sql.push_str(&format!(" AND p.status = ${}", param_idx));
        params.push(serde_json::json!(format!("{:?}", status).to_lowercase()));
    }

    if let Some(category_id) = &query.category_id {
        sql.push_str(&format!(" AND p.category_id = ${}", param_idx));
        params.push(serde_json::json!(category_id.to_string()));
    }

    if let Some(search) = &query.search {
        sql.push_str(&format!(" AND (p.title ILIKE ${} OR p.content ILIKE ${})", param_idx, param_idx));
        params.push(serde_json::json!(format!("%{}%", search)));
    }

    // Count total
    let count_sql = sql.replace(
        "SELECT p.*, json_build_object('id', u.id, 'name', u.name) as author, json_build_object('id', c.id, 'name', c.name, 'slug', c.slug) as category",
        "SELECT COUNT(*) as count"
    );
    let total: i64 = ctx.data.db.query_one::<CountResult>(&count_sql, params.clone())
        .await?
        .map(|r| r.count)
        .unwrap_or(0);

    // Add sorting and pagination
    let sort_by = query.sort_by.as_deref().unwrap_or("created_at");
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

/// POST /api/admin/posts - Create a new post
pub async fn create(mut req: Request, ctx: RouteContext<AppState>) -> worker::Result<Response> {
    let user = require_admin(&req, &ctx).await?;

    let body: CreatePostRequest = req.json().await
        .map_err(|e| ApiError::BadRequest(format!("Invalid request body: {}", e)))?;

    let post_id = uuid::Uuid::new_v4();
    let now = crate::utils::now();
    
    // Generate slug from title if not provided
    let slug = body.slug.unwrap_or_else(|| slug::slugify(&body.title));

    // Check slug uniqueness
    let existing: Option<Post> = ctx.data.db.query_one(
        "SELECT * FROM posts WHERE slug = $1",
        vec![serde_json::json!(&slug)]
    ).await?;

    if existing.is_some() {
        return Err(ApiError::Conflict("A post with this slug already exists".to_string()).into());
    }

    // Calculate reading time (rough estimate: 200 words per minute)
    let word_count = body.content.split_whitespace().count();
    let reading_time = ((word_count as f32) / 200.0).ceil() as i32;

    ctx.data.db.execute(
        r#"
        INSERT INTO posts (
            id, title, slug, excerpt, content, featured_image, author_id, category_id,
            status, visibility, published_at, meta_title, meta_description, reading_time,
            view_count, created_at, updated_at
        ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, 0, $15, $15)
        "#,
        vec![
            serde_json::json!(post_id.to_string()),
            serde_json::json!(body.title),
            serde_json::json!(slug),
            serde_json::json!(body.excerpt),
            serde_json::json!(body.content),
            serde_json::json!(body.featured_image),
            serde_json::json!(user.id.to_string()),
            serde_json::json!(body.category_id.map(|id| id.to_string())),
            serde_json::json!(body.status.map(|s| format!("{:?}", s).to_lowercase()).unwrap_or_else(|| "draft".to_string())),
            serde_json::json!(body.visibility.map(|v| format!("{:?}", v).to_lowercase()).unwrap_or_else(|| "public".to_string())),
            serde_json::json!(body.published_at.map(|dt| dt.to_rfc3339())),
            serde_json::json!(body.meta_title),
            serde_json::json!(body.meta_description),
            serde_json::json!(reading_time),
            serde_json::json!(now.to_rfc3339()),
        ]
    ).await?;

    // Handle tags
    if let Some(tag_ids) = body.tag_ids {
        for tag_id in tag_ids {
            ctx.data.db.execute(
                "INSERT INTO post_tag (post_id, tag_id) VALUES ($1, $2) ON CONFLICT DO NOTHING",
                vec![serde_json::json!(post_id.to_string()), serde_json::json!(tag_id.to_string())]
            ).await?;
        }
    }

    // Return created post
    let post: PostWithRelations = ctx.data.db.query_one(
        r#"
        SELECT p.*, 
               json_build_object('id', u.id, 'name', u.name) as author,
               json_build_object('id', c.id, 'name', c.name, 'slug', c.slug) as category,
               '[]'::json as tags
        FROM posts p
        LEFT JOIN users u ON u.id = p.author_id
        LEFT JOIN categories c ON c.id = p.category_id
        WHERE p.id = $1
        "#,
        vec![serde_json::json!(post_id.to_string())]
    ).await?
    .ok_or_else(|| ApiError::Internal("Failed to retrieve created post".to_string()))?;

    Response::from_json(&post).map(|r| r.with_status(201))
}

/// GET /api/admin/posts/:id - Get a single post
pub async fn show(req: Request, ctx: RouteContext<AppState>) -> worker::Result<Response> {
    require_admin(&req, &ctx).await?;

    let id = ctx.param("id")
        .ok_or_else(|| ApiError::BadRequest("Missing post id".to_string()))?;

    let post: Option<PostWithRelations> = ctx.data.db.query_one(
        r#"
        SELECT p.*, 
               json_build_object('id', u.id, 'name', u.name) as author,
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
        WHERE p.id = $1
        "#,
        vec![serde_json::json!(id)]
    ).await?;

    match post {
        Some(p) => Response::from_json(&p),
        None => Err(ApiError::NotFound("Post not found".to_string()).into())
    }
}

/// PUT /api/admin/posts/:id - Update a post
pub async fn update(mut req: Request, ctx: RouteContext<AppState>) -> worker::Result<Response> {
    require_admin(&req, &ctx).await?;

    let id = ctx.param("id")
        .ok_or_else(|| ApiError::BadRequest("Missing post id".to_string()))?;

    let body: UpdatePostRequest = req.json().await
        .map_err(|e| ApiError::BadRequest(format!("Invalid request body: {}", e)))?;

    let now = crate::utils::now();

    // Build dynamic update query
    let mut updates = vec!["updated_at = $1".to_string()];
    let mut params: Vec<serde_json::Value> = vec![serde_json::json!(now.to_rfc3339())];
    let param_idx = 2;

    if let Some(title) = &body.title {
        updates.push(format!("title = ${}", param_idx));
        params.push(serde_json::json!(title));
    }

    if let Some(slug) = &body.slug {
        updates.push(format!("slug = ${}", param_idx));
        params.push(serde_json::json!(slug));
    }

    if let Some(content) = &body.content {
        updates.push(format!("content = ${}", param_idx));
        params.push(serde_json::json!(content));

        // Update reading time
        let word_count = content.split_whitespace().count();
        let reading_time = ((word_count as f32) / 200.0).ceil() as i32;
        updates.push(format!("reading_time = ${}", param_idx));
        params.push(serde_json::json!(reading_time));
    }

    if let Some(excerpt) = &body.excerpt {
        updates.push(format!("excerpt = ${}", param_idx));
        params.push(serde_json::json!(excerpt));
    }

    if let Some(featured_image) = &body.featured_image {
        updates.push(format!("featured_image = ${}", param_idx));
        params.push(serde_json::json!(featured_image));
    }

    if let Some(category_id) = &body.category_id {
        updates.push(format!("category_id = ${}", param_idx));
        params.push(serde_json::json!(category_id.to_string()));
    }

    if let Some(status) = &body.status {
        updates.push(format!("status = ${}", param_idx));
        params.push(serde_json::json!(format!("{:?}", status).to_lowercase()));
    }

    if let Some(visibility) = &body.visibility {
        updates.push(format!("visibility = ${}", param_idx));
        params.push(serde_json::json!(format!("{:?}", visibility).to_lowercase()));
    }

    if let Some(published_at) = &body.published_at {
        updates.push(format!("published_at = ${}", param_idx));
        params.push(serde_json::json!(published_at.to_rfc3339()));
    }

    if let Some(meta_title) = &body.meta_title {
        updates.push(format!("meta_title = ${}", param_idx));
        params.push(serde_json::json!(meta_title));
    }

    if let Some(meta_description) = &body.meta_description {
        updates.push(format!("meta_description = ${}", param_idx));
        params.push(serde_json::json!(meta_description));
    }

    params.push(serde_json::json!(id));

    let sql = format!(
        "UPDATE posts SET {} WHERE id = ${}",
        updates.join(", "),
        param_idx
    );

    let updated = ctx.data.db.execute(&sql, params).await?;

    if updated == 0 {
        return Err(ApiError::NotFound("Post not found".to_string()).into());
    }

    // Handle tags if provided
    if let Some(tag_ids) = body.tag_ids {
        // Remove existing tags
        ctx.data.db.execute(
            "DELETE FROM post_tag WHERE post_id = $1",
            vec![serde_json::json!(id)]
        ).await?;

        // Add new tags
        for tag_id in tag_ids {
            ctx.data.db.execute(
                "INSERT INTO post_tag (post_id, tag_id) VALUES ($1, $2)",
                vec![serde_json::json!(id), serde_json::json!(tag_id.to_string())]
            ).await?;
        }
    }

    // Return updated post
    let post: PostWithRelations = ctx.data.db.query_one(
        r#"
        SELECT p.*, 
               json_build_object('id', u.id, 'name', u.name) as author,
               json_build_object('id', c.id, 'name', c.name, 'slug', c.slug) as category,
               '[]'::json as tags
        FROM posts p
        LEFT JOIN users u ON u.id = p.author_id
        LEFT JOIN categories c ON c.id = p.category_id
        WHERE p.id = $1
        "#,
        vec![serde_json::json!(id)]
    ).await?
    .ok_or_else(|| ApiError::NotFound("Post not found".to_string()))?;

    Response::from_json(&post)
}

/// DELETE /api/admin/posts/:id - Delete a post
pub async fn delete(req: Request, ctx: RouteContext<AppState>) -> worker::Result<Response> {
    require_admin(&req, &ctx).await?;

    let id = ctx.param("id")
        .ok_or_else(|| ApiError::BadRequest("Missing post id".to_string()))?;

    // Delete tags first
    ctx.data.db.execute(
        "DELETE FROM post_tag WHERE post_id = $1",
        vec![serde_json::json!(id)]
    ).await?;

    // Delete post
    let deleted = ctx.data.db.execute(
        "DELETE FROM posts WHERE id = $1",
        vec![serde_json::json!(id)]
    ).await?;

    if deleted == 0 {
        return Err(ApiError::NotFound("Post not found".to_string()).into());
    }

    Response::from_json(&serde_json::json!({
        "message": "Post deleted successfully"
    }))
}

#[derive(serde::Deserialize)]
struct CountResult {
    count: i64,
}
