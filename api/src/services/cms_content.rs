//! CMS Content Service - Apple ICT 7+ Principal Engineer Grade
//!
//! Comprehensive content management service providing:
//! - Content CRUD operations with full validation
//! - Asset management with folder organization
//! - Revision history and rollback
//! - Workflow state management
//! - Tag and relation management
//! - SEO optimization
//!
//! @version 2.0.0 - January 2026

use anyhow::{anyhow, Result};
use chrono::{DateTime, Utc};
use serde_json::{json, Value as JsonValue};
use sqlx::PgPool;
use uuid::Uuid;

use crate::models::cms::{
    AssetListQuery, CmsAsset, CmsAssetFolder, CmsAssetSummary, CmsComment, CmsContent,
    CmsContentRelation, CmsContentStatus, CmsContentSummary, CmsContentType, CmsNavigationMenu,
    CmsRedirect, CmsRevision, CmsSiteSettings, CmsTag, CmsUser, CmsUserRole, CmsWebhook,
    CmsWebhookDelivery, ContentBlock, ContentListQuery, CreateAssetFolderRequest,
    CreateAssetRequest, CreateCommentRequest, CreateContentRequest, CreateNavigationMenuRequest,
    CreateRedirectRequest, CreateRelationRequest, CreateTagRequest, CreateWebhookRequest,
    PaginatedResponse, PaginationMeta, TransitionStatusRequest, UpdateAssetFolderRequest,
    UpdateAssetRequest, UpdateContentRequest, UpdateNavigationMenuRequest,
    UpdateSiteSettingsRequest,
};

// ═══════════════════════════════════════════════════════════════════════════════════════
// CMS USERS
// ═══════════════════════════════════════════════════════════════════════════════════════

/// Get CMS user by ID
pub async fn get_cms_user(pool: &PgPool, id: Uuid) -> Result<Option<CmsUser>> {
    let user: Option<CmsUser> = sqlx::query_as(
        r#"
        SELECT id, user_id, display_name, avatar_url, bio, role,
               permissions, allowed_content_types, preferences, notification_settings,
               is_active, last_login_at, created_at, updated_at, created_by, updated_by
        FROM cms_users
        WHERE id = $1
        "#,
    )
    .bind(id)
    .fetch_optional(pool)
    .await?;

    Ok(user)
}

/// Get CMS user by main user ID
pub async fn get_cms_user_by_user_id(pool: &PgPool, user_id: i64) -> Result<Option<CmsUser>> {
    let user: Option<CmsUser> = sqlx::query_as(
        r#"
        SELECT id, user_id, display_name, avatar_url, bio, role,
               permissions, allowed_content_types, preferences, notification_settings,
               is_active, last_login_at, created_at, updated_at, created_by, updated_by
        FROM cms_users
        WHERE user_id = $1
        "#,
    )
    .bind(user_id)
    .fetch_optional(pool)
    .await?;

    Ok(user)
}

/// Create or update CMS user from main user
pub async fn upsert_cms_user(
    pool: &PgPool,
    user_id: i64,
    display_name: &str,
    role: CmsUserRole,
) -> Result<CmsUser> {
    let user: CmsUser = sqlx::query_as(
        r#"
        INSERT INTO cms_users (user_id, display_name, role)
        VALUES ($1, $2, $3)
        ON CONFLICT (user_id) DO UPDATE SET
            display_name = EXCLUDED.display_name,
            last_login_at = NOW(),
            updated_at = NOW()
        RETURNING id, user_id, display_name, avatar_url, bio, role,
                  permissions, allowed_content_types, preferences, notification_settings,
                  is_active, last_login_at, created_at, updated_at, created_by, updated_by
        "#,
    )
    .bind(user_id)
    .bind(display_name)
    .bind(&role)
    .fetch_one(pool)
    .await?;

    Ok(user)
}

// ═══════════════════════════════════════════════════════════════════════════════════════
// ASSET FOLDERS
// ═══════════════════════════════════════════════════════════════════════════════════════

/// List all asset folders
pub async fn list_asset_folders(pool: &PgPool) -> Result<Vec<CmsAssetFolder>> {
    let folders: Vec<CmsAssetFolder> = sqlx::query_as(
        r#"
        SELECT id, name, slug, parent_id, path, depth, description, color, icon,
               is_public, allowed_roles, sort_order, created_at, updated_at, created_by, updated_by
        FROM cms_asset_folders
        ORDER BY path, sort_order
        "#,
    )
    .fetch_all(pool)
    .await?;

    Ok(folders)
}

/// Get asset folder by ID
pub async fn get_asset_folder(pool: &PgPool, id: Uuid) -> Result<Option<CmsAssetFolder>> {
    let folder: Option<CmsAssetFolder> = sqlx::query_as(
        r#"
        SELECT id, name, slug, parent_id, path, depth, description, color, icon,
               is_public, allowed_roles, sort_order, created_at, updated_at, created_by, updated_by
        FROM cms_asset_folders
        WHERE id = $1
        "#,
    )
    .bind(id)
    .fetch_optional(pool)
    .await?;

    Ok(folder)
}

/// Create asset folder
pub async fn create_asset_folder(
    pool: &PgPool,
    request: CreateAssetFolderRequest,
    created_by: Option<Uuid>,
) -> Result<CmsAssetFolder> {
    let slug = slug::slugify(&request.name);

    let folder: CmsAssetFolder = sqlx::query_as(
        r#"
        INSERT INTO cms_asset_folders (name, slug, parent_id, description, color, icon, is_public, created_by)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
        RETURNING id, name, slug, parent_id, path, depth, description, color, icon,
                  is_public, allowed_roles, sort_order, created_at, updated_at, created_by, updated_by
        "#,
    )
    .bind(&request.name)
    .bind(&slug)
    .bind(request.parent_id)
    .bind(&request.description)
    .bind(&request.color)
    .bind(&request.icon)
    .bind(request.is_public.unwrap_or(false))
    .bind(created_by)
    .fetch_one(pool)
    .await?;

    Ok(folder)
}

/// Update asset folder
pub async fn update_asset_folder(
    pool: &PgPool,
    id: Uuid,
    request: UpdateAssetFolderRequest,
    updated_by: Option<Uuid>,
) -> Result<CmsAssetFolder> {
    let folder: CmsAssetFolder = sqlx::query_as(
        r#"
        UPDATE cms_asset_folders SET
            name = COALESCE($1, name),
            slug = COALESCE($2, slug),
            parent_id = COALESCE($3, parent_id),
            description = COALESCE($4, description),
            color = COALESCE($5, color),
            icon = COALESCE($6, icon),
            is_public = COALESCE($7, is_public),
            sort_order = COALESCE($8, sort_order),
            updated_by = $9
        WHERE id = $10
        RETURNING id, name, slug, parent_id, path, depth, description, color, icon,
                  is_public, allowed_roles, sort_order, created_at, updated_at, created_by, updated_by
        "#,
    )
    .bind(&request.name)
    .bind(request.name.as_ref().map(slug::slugify))
    .bind(request.parent_id)
    .bind(&request.description)
    .bind(&request.color)
    .bind(&request.icon)
    .bind(request.is_public)
    .bind(request.sort_order)
    .bind(updated_by)
    .bind(id)
    .fetch_one(pool)
    .await?;

    Ok(folder)
}

/// Delete asset folder (and optionally move contents)
pub async fn delete_asset_folder(pool: &PgPool, id: Uuid, move_to: Option<Uuid>) -> Result<()> {
    // Move assets to new folder if specified
    if let Some(target_folder_id) = move_to {
        sqlx::query("UPDATE cms_assets SET folder_id = $1 WHERE folder_id = $2")
            .bind(target_folder_id)
            .bind(id)
            .execute(pool)
            .await?;
    }

    sqlx::query("DELETE FROM cms_asset_folders WHERE id = $1")
        .bind(id)
        .execute(pool)
        .await?;

    Ok(())
}

// ═══════════════════════════════════════════════════════════════════════════════════════
// ASSETS
// ═══════════════════════════════════════════════════════════════════════════════════════

/// List assets with pagination and filtering
pub async fn list_assets(
    pool: &PgPool,
    query: AssetListQuery,
) -> Result<PaginatedResponse<CmsAssetSummary>> {
    let limit = query.limit.unwrap_or(50).min(100);
    let offset = query.offset.unwrap_or(0);
    let include_deleted = query.include_deleted.unwrap_or(false);

    let sort_by = query.sort_by.as_deref().unwrap_or("created_at");
    let sort_order = query.sort_order.as_deref().unwrap_or("DESC");

    let assets: Vec<CmsAssetSummary> = sqlx::query_as(
        r#"
        SELECT id, folder_id, filename, mime_type, file_size, cdn_url,
               width, height, blurhash, thumbnail_url, title, alt_text, usage_count, created_at
        FROM cms_assets
        WHERE ($1::uuid IS NULL OR folder_id = $1)
          AND ($2::text IS NULL OR mime_type LIKE $2 || '%')
          AND ($3::text IS NULL OR filename ILIKE '%' || $3 || '%' OR title ILIKE '%' || $3 || '%')
          AND ($4::boolean OR deleted_at IS NULL)
        ORDER BY
            CASE WHEN $5 = 'created_at' AND $6 = 'DESC' THEN created_at END DESC,
            CASE WHEN $5 = 'created_at' AND $6 = 'ASC' THEN created_at END ASC,
            CASE WHEN $5 = 'filename' AND $6 = 'DESC' THEN filename END DESC,
            CASE WHEN $5 = 'filename' AND $6 = 'ASC' THEN filename END ASC,
            CASE WHEN $5 = 'file_size' AND $6 = 'DESC' THEN file_size END DESC,
            CASE WHEN $5 = 'file_size' AND $6 = 'ASC' THEN file_size END ASC,
            CASE WHEN $5 = 'usage_count' AND $6 = 'DESC' THEN usage_count END DESC,
            CASE WHEN $5 = 'usage_count' AND $6 = 'ASC' THEN usage_count END ASC,
            created_at DESC
        LIMIT $7 OFFSET $8
        "#,
    )
    .bind(query.folder_id)
    .bind(&query.mime_type)
    .bind(&query.search)
    .bind(include_deleted)
    .bind(sort_by)
    .bind(sort_order)
    .bind(limit)
    .bind(offset)
    .fetch_all(pool)
    .await?;

    let total: (i64,) = sqlx::query_as(
        r#"
        SELECT COUNT(*)
        FROM cms_assets
        WHERE ($1::uuid IS NULL OR folder_id = $1)
          AND ($2::text IS NULL OR mime_type LIKE $2 || '%')
          AND ($3::text IS NULL OR filename ILIKE '%' || $3 || '%' OR title ILIKE '%' || $3 || '%')
          AND ($4::boolean OR deleted_at IS NULL)
        "#,
    )
    .bind(query.folder_id)
    .bind(&query.mime_type)
    .bind(&query.search)
    .bind(include_deleted)
    .fetch_one(pool)
    .await?;

    Ok(PaginatedResponse {
        data: assets,
        meta: PaginationMeta {
            total: total.0,
            limit,
            offset,
            has_more: offset + limit < total.0,
        },
    })
}

/// Get asset by ID
pub async fn get_asset(pool: &PgPool, id: Uuid) -> Result<Option<CmsAsset>> {
    let asset: Option<CmsAsset> = sqlx::query_as(
        r#"
        SELECT id, folder_id, filename, original_filename, mime_type, file_size, file_extension,
               storage_provider, storage_key, cdn_url, width, height, aspect_ratio, blurhash, dominant_color,
               duration_seconds, video_codec, audio_codec, bunny_video_id, bunny_library_id, thumbnail_url,
               variants, title, alt_text, caption, description, credits, seo_title, seo_description,
               tags, usage_count, last_used_at, deleted_at, version, created_at, updated_at, created_by, updated_by
        FROM cms_assets
        WHERE id = $1
        "#,
    )
    .bind(id)
    .fetch_optional(pool)
    .await?;

    Ok(asset)
}

/// Create asset
pub async fn create_asset(
    pool: &PgPool,
    request: CreateAssetRequest,
    created_by: Option<Uuid>,
) -> Result<CmsAsset> {
    let aspect_ratio = match (request.width, request.height) {
        (Some(w), Some(h)) if h > 0 => {
            Some(rust_decimal::Decimal::from(w) / rust_decimal::Decimal::from(h))
        }
        _ => None,
    };

    let asset: CmsAsset = sqlx::query_as(
        r#"
        INSERT INTO cms_assets (
            folder_id, filename, original_filename, mime_type, file_size, file_extension,
            storage_key, cdn_url, width, height, aspect_ratio, blurhash, dominant_color,
            title, alt_text, tags, created_by
        ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17)
        RETURNING id, folder_id, filename, original_filename, mime_type, file_size, file_extension,
                  storage_provider, storage_key, cdn_url, width, height, aspect_ratio, blurhash, dominant_color,
                  duration_seconds, video_codec, audio_codec, bunny_video_id, bunny_library_id, thumbnail_url,
                  variants, title, alt_text, caption, description, credits, seo_title, seo_description,
                  tags, usage_count, last_used_at, deleted_at, version, created_at, updated_at, created_by, updated_by
        "#,
    )
    .bind(request.folder_id)
    .bind(&request.filename)
    .bind(&request.original_filename)
    .bind(&request.mime_type)
    .bind(request.file_size)
    .bind(&request.file_extension)
    .bind(&request.storage_key)
    .bind(&request.cdn_url)
    .bind(request.width)
    .bind(request.height)
    .bind(aspect_ratio)
    .bind(&request.blurhash)
    .bind(&request.dominant_color)
    .bind(&request.title)
    .bind(&request.alt_text)
    .bind(&request.tags)
    .bind(created_by)
    .fetch_one(pool)
    .await?;

    Ok(asset)
}

/// Update asset metadata
pub async fn update_asset(
    pool: &PgPool,
    id: Uuid,
    request: UpdateAssetRequest,
    updated_by: Option<Uuid>,
) -> Result<CmsAsset> {
    let asset: CmsAsset = sqlx::query_as(
        r#"
        UPDATE cms_assets SET
            folder_id = COALESCE($1, folder_id),
            title = COALESCE($2, title),
            alt_text = COALESCE($3, alt_text),
            caption = COALESCE($4, caption),
            description = COALESCE($5, description),
            credits = COALESCE($6, credits),
            seo_title = COALESCE($7, seo_title),
            seo_description = COALESCE($8, seo_description),
            tags = COALESCE($9, tags),
            updated_by = $10,
            version = version + 1
        WHERE id = $11 AND deleted_at IS NULL
        RETURNING id, folder_id, filename, original_filename, mime_type, file_size, file_extension,
                  storage_provider, storage_key, cdn_url, width, height, aspect_ratio, blurhash, dominant_color,
                  duration_seconds, video_codec, audio_codec, bunny_video_id, bunny_library_id, thumbnail_url,
                  variants, title, alt_text, caption, description, credits, seo_title, seo_description,
                  tags, usage_count, last_used_at, deleted_at, version, created_at, updated_at, created_by, updated_by
        "#,
    )
    .bind(request.folder_id)
    .bind(&request.title)
    .bind(&request.alt_text)
    .bind(&request.caption)
    .bind(&request.description)
    .bind(&request.credits)
    .bind(&request.seo_title)
    .bind(&request.seo_description)
    .bind(&request.tags)
    .bind(updated_by)
    .bind(id)
    .fetch_one(pool)
    .await?;

    Ok(asset)
}

/// Soft delete asset
pub async fn delete_asset(pool: &PgPool, id: Uuid) -> Result<()> {
    sqlx::query("UPDATE cms_assets SET deleted_at = NOW() WHERE id = $1")
        .bind(id)
        .execute(pool)
        .await?;

    Ok(())
}

/// Permanently delete asset
pub async fn hard_delete_asset(pool: &PgPool, id: Uuid) -> Result<()> {
    sqlx::query("DELETE FROM cms_assets WHERE id = $1")
        .bind(id)
        .execute(pool)
        .await?;

    Ok(())
}

// ═══════════════════════════════════════════════════════════════════════════════════════
// CONTENT
// ═══════════════════════════════════════════════════════════════════════════════════════

/// List content with pagination and filtering
pub async fn list_content(
    pool: &PgPool,
    query: ContentListQuery,
) -> Result<PaginatedResponse<CmsContentSummary>> {
    let limit = query.limit.unwrap_or(50).min(100);
    let offset = query.offset.unwrap_or(0);
    let include_deleted = query.include_deleted.unwrap_or(false);

    let sort_by = query.sort_by.as_deref().unwrap_or("created_at");
    let sort_order = query.sort_order.as_deref().unwrap_or("DESC");

    let content: Vec<CmsContentSummary> = sqlx::query_as(
        r#"
        SELECT id, content_type, slug, locale, title, excerpt, featured_image_id,
               author_id, status, published_at, created_at, updated_at
        FROM cms_content
        WHERE ($1::cms_content_type IS NULL OR content_type = $1)
          AND ($2::cms_content_status IS NULL OR status = $2)
          AND ($3::uuid IS NULL OR author_id = $3)
          AND ($4::text IS NULL OR locale = $4)
          AND ($5::text IS NULL OR title ILIKE '%' || $5 || '%' OR slug ILIKE '%' || $5 || '%')
          AND ($6::boolean OR deleted_at IS NULL)
        ORDER BY
            CASE WHEN $7 = 'created_at' AND $8 = 'DESC' THEN created_at END DESC,
            CASE WHEN $7 = 'created_at' AND $8 = 'ASC' THEN created_at END ASC,
            CASE WHEN $7 = 'updated_at' AND $8 = 'DESC' THEN updated_at END DESC,
            CASE WHEN $7 = 'updated_at' AND $8 = 'ASC' THEN updated_at END ASC,
            CASE WHEN $7 = 'published_at' AND $8 = 'DESC' THEN published_at END DESC NULLS LAST,
            CASE WHEN $7 = 'published_at' AND $8 = 'ASC' THEN published_at END ASC NULLS LAST,
            CASE WHEN $7 = 'title' AND $8 = 'DESC' THEN title END DESC,
            CASE WHEN $7 = 'title' AND $8 = 'ASC' THEN title END ASC,
            created_at DESC
        LIMIT $9 OFFSET $10
        "#,
    )
    .bind(&query.content_type)
    .bind(&query.status)
    .bind(query.author_id)
    .bind(&query.locale)
    .bind(&query.search)
    .bind(include_deleted)
    .bind(sort_by)
    .bind(sort_order)
    .bind(limit)
    .bind(offset)
    .fetch_all(pool)
    .await?;

    let total: (i64,) = sqlx::query_as(
        r#"
        SELECT COUNT(*)
        FROM cms_content
        WHERE ($1::cms_content_type IS NULL OR content_type = $1)
          AND ($2::cms_content_status IS NULL OR status = $2)
          AND ($3::uuid IS NULL OR author_id = $3)
          AND ($4::text IS NULL OR locale = $4)
          AND ($5::text IS NULL OR title ILIKE '%' || $5 || '%' OR slug ILIKE '%' || $5 || '%')
          AND ($6::boolean OR deleted_at IS NULL)
        "#,
    )
    .bind(&query.content_type)
    .bind(&query.status)
    .bind(query.author_id)
    .bind(&query.locale)
    .bind(&query.search)
    .bind(include_deleted)
    .fetch_one(pool)
    .await?;

    Ok(PaginatedResponse {
        data: content,
        meta: PaginationMeta {
            total: total.0,
            limit,
            offset,
            has_more: offset + limit < total.0,
        },
    })
}

/// Get content by ID
pub async fn get_content(pool: &PgPool, id: Uuid) -> Result<Option<CmsContent>> {
    let content: Option<CmsContent> = sqlx::query_as(
        r#"
        SELECT id, content_type, slug, locale, is_primary_locale, parent_content_id,
               title, subtitle, excerpt, content, content_blocks,
               featured_image_id, og_image_id, gallery_ids,
               meta_title, meta_description, meta_keywords, canonical_url, robots_directives,
               structured_data, author_id, contributors, status, published_at,
               scheduled_publish_at, scheduled_unpublish_at,
               primary_category_id, categories, custom_fields, template,
               deleted_at, version, created_at, updated_at, created_by, updated_by
        FROM cms_content
        WHERE id = $1
        "#,
    )
    .bind(id)
    .fetch_optional(pool)
    .await?;

    Ok(content)
}

/// Get content by type and slug
pub async fn get_content_by_slug(
    pool: &PgPool,
    content_type: CmsContentType,
    slug: &str,
    locale: Option<&str>,
) -> Result<Option<CmsContent>> {
    let locale = locale.unwrap_or("en");

    let content: Option<CmsContent> = sqlx::query_as(
        r#"
        SELECT id, content_type, slug, locale, is_primary_locale, parent_content_id,
               title, subtitle, excerpt, content, content_blocks,
               featured_image_id, og_image_id, gallery_ids,
               meta_title, meta_description, meta_keywords, canonical_url, robots_directives,
               structured_data, author_id, contributors, status, published_at,
               scheduled_publish_at, scheduled_unpublish_at,
               primary_category_id, categories, custom_fields, template,
               deleted_at, version, created_at, updated_at, created_by, updated_by
        FROM cms_content
        WHERE content_type = $1 AND slug = $2 AND locale = $3 AND deleted_at IS NULL
        "#,
    )
    .bind(&content_type)
    .bind(slug)
    .bind(locale)
    .fetch_optional(pool)
    .await?;

    Ok(content)
}

/// Create content
pub async fn create_content(
    pool: &PgPool,
    request: CreateContentRequest,
    created_by: Option<Uuid>,
) -> Result<CmsContent> {
    let locale = request.locale.as_deref().unwrap_or("en");
    let slug = slug::slugify(&request.slug);

    let content: CmsContent = sqlx::query_as(
        r#"
        INSERT INTO cms_content (
            content_type, slug, locale, title, subtitle, excerpt, content, content_blocks,
            featured_image_id, og_image_id, meta_title, meta_description, meta_keywords,
            canonical_url, author_id, custom_fields, template, created_by
        ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18)
        RETURNING id, content_type, slug, locale, is_primary_locale, parent_content_id,
                  title, subtitle, excerpt, content, content_blocks,
                  featured_image_id, og_image_id, gallery_ids,
                  meta_title, meta_description, meta_keywords, canonical_url, robots_directives,
                  structured_data, author_id, contributors, status, published_at,
                  scheduled_publish_at, scheduled_unpublish_at,
                  primary_category_id, categories, custom_fields, template,
                  deleted_at, version, created_at, updated_at, created_by, updated_by
        "#,
    )
    .bind(&request.content_type)
    .bind(&slug)
    .bind(locale)
    .bind(&request.title)
    .bind(&request.subtitle)
    .bind(&request.excerpt)
    .bind(&request.content)
    .bind(&request.content_blocks)
    .bind(request.featured_image_id)
    .bind(request.og_image_id)
    .bind(&request.meta_title)
    .bind(&request.meta_description)
    .bind(&request.meta_keywords)
    .bind(&request.canonical_url)
    .bind(request.author_id)
    .bind(&request.custom_fields)
    .bind(&request.template)
    .bind(created_by)
    .fetch_one(pool)
    .await?;

    // Create initial revision
    create_revision(
        pool,
        content.id,
        &content,
        created_by,
        Some("Initial creation"),
        None,
    )
    .await?;

    // Log to audit
    log_cms_audit(
        pool,
        "content.created",
        "cms_content",
        Some(content.id),
        created_by,
        None,
        Some(json!({"content_type": content.content_type, "slug": content.slug})),
        None,
    )
    .await?;

    Ok(content)
}

/// Update content
pub async fn update_content(
    pool: &PgPool,
    id: Uuid,
    request: UpdateContentRequest,
    updated_by: Option<Uuid>,
) -> Result<CmsContent> {
    // Get current content for revision
    let old_content = get_content(pool, id)
        .await?
        .ok_or_else(|| anyhow!("Content not found"))?;

    let content: CmsContent = sqlx::query_as(
        r#"
        UPDATE cms_content SET
            slug = COALESCE($1, slug),
            title = COALESCE($2, title),
            subtitle = COALESCE($3, subtitle),
            excerpt = COALESCE($4, excerpt),
            content = COALESCE($5, content),
            content_blocks = COALESCE($6, content_blocks),
            featured_image_id = COALESCE($7, featured_image_id),
            og_image_id = COALESCE($8, og_image_id),
            gallery_ids = COALESCE($9, gallery_ids),
            meta_title = COALESCE($10, meta_title),
            meta_description = COALESCE($11, meta_description),
            meta_keywords = COALESCE($12, meta_keywords),
            canonical_url = COALESCE($13, canonical_url),
            robots_directives = COALESCE($14, robots_directives),
            structured_data = COALESCE($15, structured_data),
            author_id = COALESCE($16, author_id),
            contributors = COALESCE($17, contributors),
            primary_category_id = COALESCE($18, primary_category_id),
            categories = COALESCE($19, categories),
            custom_fields = COALESCE($20, custom_fields),
            template = COALESCE($21, template),
            updated_by = $22,
            version = version + 1
        WHERE id = $23 AND deleted_at IS NULL
        RETURNING id, content_type, slug, locale, is_primary_locale, parent_content_id,
                  title, subtitle, excerpt, content, content_blocks,
                  featured_image_id, og_image_id, gallery_ids,
                  meta_title, meta_description, meta_keywords, canonical_url, robots_directives,
                  structured_data, author_id, contributors, status, published_at,
                  scheduled_publish_at, scheduled_unpublish_at,
                  primary_category_id, categories, custom_fields, template,
                  deleted_at, version, created_at, updated_at, created_by, updated_by
        "#,
    )
    .bind(request.slug.as_ref().map(slug::slugify))
    .bind(&request.title)
    .bind(&request.subtitle)
    .bind(&request.excerpt)
    .bind(&request.content)
    .bind(&request.content_blocks)
    .bind(request.featured_image_id)
    .bind(request.og_image_id)
    .bind(&request.gallery_ids)
    .bind(&request.meta_title)
    .bind(&request.meta_description)
    .bind(&request.meta_keywords)
    .bind(&request.canonical_url)
    .bind(&request.robots_directives)
    .bind(&request.structured_data)
    .bind(request.author_id)
    .bind(&request.contributors)
    .bind(request.primary_category_id)
    .bind(&request.categories)
    .bind(&request.custom_fields)
    .bind(&request.template)
    .bind(updated_by)
    .bind(id)
    .fetch_one(pool)
    .await?;

    // Create revision
    create_revision(
        pool,
        content.id,
        &content,
        updated_by,
        Some("Content updated"),
        None,
    )
    .await?;

    // Log to audit
    log_cms_audit(
        pool,
        "content.updated",
        "cms_content",
        Some(content.id),
        updated_by,
        Some(json!({"version": old_content.version})),
        Some(json!({"version": content.version})),
        None,
    )
    .await?;

    Ok(content)
}

/// Transition content status
pub async fn transition_content_status(
    pool: &PgPool,
    id: Uuid,
    request: TransitionStatusRequest,
    transitioned_by: Option<Uuid>,
) -> Result<CmsContent> {
    let old_content = get_content(pool, id)
        .await?
        .ok_or_else(|| anyhow!("Content not found"))?;

    let published_at =
        if request.status == CmsContentStatus::Published && old_content.published_at.is_none() {
            Some(Utc::now())
        } else {
            old_content.published_at
        };

    let content: CmsContent = sqlx::query_as(
        r#"
        UPDATE cms_content SET
            status = $1,
            published_at = $2,
            scheduled_publish_at = $3,
            updated_by = $4,
            version = version + 1
        WHERE id = $5 AND deleted_at IS NULL
        RETURNING id, content_type, slug, locale, is_primary_locale, parent_content_id,
                  title, subtitle, excerpt, content, content_blocks,
                  featured_image_id, og_image_id, gallery_ids,
                  meta_title, meta_description, meta_keywords, canonical_url, robots_directives,
                  structured_data, author_id, contributors, status, published_at,
                  scheduled_publish_at, scheduled_unpublish_at,
                  primary_category_id, categories, custom_fields, template,
                  deleted_at, version, created_at, updated_at, created_by, updated_by
        "#,
    )
    .bind(&request.status)
    .bind(published_at)
    .bind(request.scheduled_at)
    .bind(transitioned_by)
    .bind(id)
    .fetch_one(pool)
    .await?;

    // Log workflow transition
    sqlx::query(
        r#"
        INSERT INTO cms_workflow_log (content_id, from_status, to_status, transitioned_by, comment)
        VALUES ($1, $2, $3, $4, $5)
        "#,
    )
    .bind(id)
    .bind(&old_content.status)
    .bind(&request.status)
    .bind(transitioned_by)
    .bind(&request.comment)
    .execute(pool)
    .await?;

    // Log to audit
    log_cms_audit(
        pool,
        "content.status_changed",
        "cms_content",
        Some(content.id),
        transitioned_by,
        Some(json!({"status": old_content.status})),
        Some(json!({"status": content.status})),
        Some(json!({"comment": request.comment})),
    )
    .await?;

    Ok(content)
}

/// Soft delete content
pub async fn delete_content(pool: &PgPool, id: Uuid, deleted_by: Option<Uuid>) -> Result<()> {
    sqlx::query("UPDATE cms_content SET deleted_at = NOW(), updated_by = $1 WHERE id = $2")
        .bind(deleted_by)
        .bind(id)
        .execute(pool)
        .await?;

    log_cms_audit(
        pool,
        "content.deleted",
        "cms_content",
        Some(id),
        deleted_by,
        None,
        None,
        None,
    )
    .await?;

    Ok(())
}

// ═══════════════════════════════════════════════════════════════════════════════════════
// REVISIONS
// ═══════════════════════════════════════════════════════════════════════════════════════

/// Create a revision for content
pub async fn create_revision(
    pool: &PgPool,
    content_id: Uuid,
    content: &CmsContent,
    created_by: Option<Uuid>,
    change_summary: Option<&str>,
    changed_fields: Option<Vec<String>>,
) -> Result<Uuid> {
    let data = serde_json::to_value(content)?;

    let revision_id: Uuid = sqlx::query_scalar("SELECT cms_create_revision($1, $2, $3, $4, $5)")
        .bind(content_id)
        .bind(&data)
        .bind(created_by)
        .bind(change_summary)
        .bind(changed_fields.as_ref())
        .fetch_one(pool)
        .await?;

    Ok(revision_id)
}

/// Get revision history for content
pub async fn get_revisions(
    pool: &PgPool,
    content_id: Uuid,
    limit: i64,
) -> Result<Vec<CmsRevision>> {
    let revisions: Vec<CmsRevision> = sqlx::query_as(
        r#"
        SELECT id, content_id, revision_number, is_current, data, change_summary, changed_fields, created_at, created_by
        FROM cms_revisions
        WHERE content_id = $1
        ORDER BY revision_number DESC
        LIMIT $2
        "#,
    )
    .bind(content_id)
    .bind(limit)
    .fetch_all(pool)
    .await?;

    Ok(revisions)
}

/// Restore a revision
pub async fn restore_revision(
    pool: &PgPool,
    content_id: Uuid,
    revision_number: i32,
    restored_by: Option<Uuid>,
) -> Result<CmsContent> {
    let revision: CmsRevision = sqlx::query_as(
        r#"
        SELECT id, content_id, revision_number, is_current, data, change_summary, changed_fields, created_at, created_by
        FROM cms_revisions
        WHERE content_id = $1 AND revision_number = $2
        "#,
    )
    .bind(content_id)
    .bind(revision_number)
    .fetch_one(pool)
    .await?;

    // Extract the stored content data and update
    let stored_content: CmsContent = serde_json::from_value(revision.data)?;

    let content: CmsContent = sqlx::query_as(
        r#"
        UPDATE cms_content SET
            title = $1, subtitle = $2, excerpt = $3, content = $4, content_blocks = $5,
            featured_image_id = $6, meta_title = $7, meta_description = $8,
            custom_fields = $9, updated_by = $10, version = version + 1
        WHERE id = $11
        RETURNING id, content_type, slug, locale, is_primary_locale, parent_content_id,
                  title, subtitle, excerpt, content, content_blocks,
                  featured_image_id, og_image_id, gallery_ids,
                  meta_title, meta_description, meta_keywords, canonical_url, robots_directives,
                  structured_data, author_id, contributors, status, published_at,
                  scheduled_publish_at, scheduled_unpublish_at,
                  primary_category_id, categories, custom_fields, template,
                  deleted_at, version, created_at, updated_at, created_by, updated_by
        "#,
    )
    .bind(&stored_content.title)
    .bind(&stored_content.subtitle)
    .bind(&stored_content.excerpt)
    .bind(&stored_content.content)
    .bind(&stored_content.content_blocks)
    .bind(stored_content.featured_image_id)
    .bind(&stored_content.meta_title)
    .bind(&stored_content.meta_description)
    .bind(&stored_content.custom_fields)
    .bind(restored_by)
    .bind(content_id)
    .fetch_one(pool)
    .await?;

    // Create a new revision for the restore
    create_revision(
        pool,
        content_id,
        &content,
        restored_by,
        Some(&format!("Restored from revision {}", revision_number)),
        None,
    )
    .await?;

    Ok(content)
}

// ═══════════════════════════════════════════════════════════════════════════════════════
// TAGS
// ═══════════════════════════════════════════════════════════════════════════════════════

/// List all tags
pub async fn list_tags(pool: &PgPool) -> Result<Vec<CmsTag>> {
    let tags: Vec<CmsTag> = sqlx::query_as(
        r#"
        SELECT id, name, slug, parent_id, description, color, icon,
               meta_title, meta_description, usage_count, created_at, updated_at, created_by
        FROM cms_tags
        ORDER BY name
        "#,
    )
    .fetch_all(pool)
    .await?;

    Ok(tags)
}

/// Create tag
pub async fn create_tag(
    pool: &PgPool,
    request: CreateTagRequest,
    created_by: Option<Uuid>,
) -> Result<CmsTag> {
    let slug = slug::slugify(&request.name);

    let tag: CmsTag = sqlx::query_as(
        r#"
        INSERT INTO cms_tags (name, slug, parent_id, description, color, icon, created_by)
        VALUES ($1, $2, $3, $4, $5, $6, $7)
        RETURNING id, name, slug, parent_id, description, color, icon,
                  meta_title, meta_description, usage_count, created_at, updated_at, created_by
        "#,
    )
    .bind(&request.name)
    .bind(&slug)
    .bind(request.parent_id)
    .bind(&request.description)
    .bind(&request.color)
    .bind(&request.icon)
    .bind(created_by)
    .fetch_one(pool)
    .await?;

    Ok(tag)
}

/// Add tag to content
pub async fn add_tag_to_content(
    pool: &PgPool,
    content_id: Uuid,
    tag_id: Uuid,
    created_by: Option<Uuid>,
) -> Result<()> {
    sqlx::query(
        r#"
        INSERT INTO cms_content_tags (content_id, tag_id, created_by)
        VALUES ($1, $2, $3)
        ON CONFLICT (content_id, tag_id) DO NOTHING
        "#,
    )
    .bind(content_id)
    .bind(tag_id)
    .bind(created_by)
    .execute(pool)
    .await?;

    Ok(())
}

/// Remove tag from content
pub async fn remove_tag_from_content(pool: &PgPool, content_id: Uuid, tag_id: Uuid) -> Result<()> {
    sqlx::query("DELETE FROM cms_content_tags WHERE content_id = $1 AND tag_id = $2")
        .bind(content_id)
        .bind(tag_id)
        .execute(pool)
        .await?;

    Ok(())
}

/// Get tags for content
pub async fn get_content_tags(pool: &PgPool, content_id: Uuid) -> Result<Vec<CmsTag>> {
    let tags: Vec<CmsTag> = sqlx::query_as(
        r#"
        SELECT t.id, t.name, t.slug, t.parent_id, t.description, t.color, t.icon,
               t.meta_title, t.meta_description, t.usage_count, t.created_at, t.updated_at, t.created_by
        FROM cms_tags t
        JOIN cms_content_tags ct ON t.id = ct.tag_id
        WHERE ct.content_id = $1
        ORDER BY ct.sort_order, t.name
        "#,
    )
    .bind(content_id)
    .fetch_all(pool)
    .await?;

    Ok(tags)
}

// ═══════════════════════════════════════════════════════════════════════════════════════
// COMMENTS
// ═══════════════════════════════════════════════════════════════════════════════════════

/// Get comments for content
pub async fn get_content_comments(pool: &PgPool, content_id: Uuid) -> Result<Vec<CmsComment>> {
    let comments: Vec<CmsComment> = sqlx::query_as(
        r#"
        SELECT id, content_id, parent_id, thread_id, body, block_id, selection_start, selection_end,
               is_resolved, resolved_by, resolved_at, mentioned_users, deleted_at,
               created_at, updated_at, created_by
        FROM cms_comments
        WHERE content_id = $1 AND deleted_at IS NULL
        ORDER BY created_at
        "#,
    )
    .bind(content_id)
    .fetch_all(pool)
    .await?;

    Ok(comments)
}

/// Create comment
pub async fn create_comment(
    pool: &PgPool,
    content_id: Uuid,
    request: CreateCommentRequest,
    created_by: Option<Uuid>,
) -> Result<CmsComment> {
    let thread_id = if request.parent_id.is_some() {
        // Get thread_id from parent
        let parent_thread: Option<Uuid> =
            sqlx::query_scalar("SELECT COALESCE(thread_id, id) FROM cms_comments WHERE id = $1")
                .bind(request.parent_id)
                .fetch_optional(pool)
                .await?;
        parent_thread
    } else {
        None
    };

    let comment: CmsComment = sqlx::query_as(
        r#"
        INSERT INTO cms_comments (content_id, parent_id, thread_id, body, block_id,
                                  selection_start, selection_end, mentioned_users, created_by)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
        RETURNING id, content_id, parent_id, thread_id, body, block_id, selection_start, selection_end,
                  is_resolved, resolved_by, resolved_at, mentioned_users, deleted_at,
                  created_at, updated_at, created_by
        "#,
    )
    .bind(content_id)
    .bind(request.parent_id)
    .bind(thread_id)
    .bind(&request.body)
    .bind(&request.block_id)
    .bind(request.selection_start)
    .bind(request.selection_end)
    .bind(&request.mentioned_users)
    .bind(created_by)
    .fetch_one(pool)
    .await?;

    Ok(comment)
}

/// Resolve comment
pub async fn resolve_comment(pool: &PgPool, id: Uuid, resolved_by: Uuid) -> Result<()> {
    sqlx::query(
        "UPDATE cms_comments SET is_resolved = true, resolved_by = $1, resolved_at = NOW() WHERE id = $2",
    )
    .bind(resolved_by)
    .bind(id)
    .execute(pool)
    .await?;

    Ok(())
}

// ═══════════════════════════════════════════════════════════════════════════════════════
// SITE SETTINGS
// ═══════════════════════════════════════════════════════════════════════════════════════

/// Get site settings (singleton)
pub async fn get_site_settings(pool: &PgPool) -> Result<CmsSiteSettings> {
    let settings: CmsSiteSettings = sqlx::query_as(
        r#"
        SELECT id, site_name, site_tagline, site_description,
               logo_light_id, logo_dark_id, favicon_id, og_default_image_id,
               contact_email, support_email, phone, address, social_links,
               default_meta_title_suffix, default_robots,
               google_analytics_id, google_tag_manager_id,
               maintenance_mode, maintenance_message,
               head_scripts, body_start_scripts, body_end_scripts, custom_css,
               settings, updated_at, updated_by
        FROM cms_site_settings
        WHERE id = '00000000-0000-0000-0000-000000000001'::uuid
        "#,
    )
    .fetch_one(pool)
    .await?;

    Ok(settings)
}

/// Update site settings
pub async fn update_site_settings(
    pool: &PgPool,
    request: UpdateSiteSettingsRequest,
    updated_by: Option<Uuid>,
) -> Result<CmsSiteSettings> {
    let settings: CmsSiteSettings = sqlx::query_as(
        r#"
        UPDATE cms_site_settings SET
            site_name = COALESCE($1, site_name),
            site_tagline = COALESCE($2, site_tagline),
            site_description = COALESCE($3, site_description),
            logo_light_id = COALESCE($4, logo_light_id),
            logo_dark_id = COALESCE($5, logo_dark_id),
            favicon_id = COALESCE($6, favicon_id),
            og_default_image_id = COALESCE($7, og_default_image_id),
            contact_email = COALESCE($8, contact_email),
            support_email = COALESCE($9, support_email),
            phone = COALESCE($10, phone),
            address = COALESCE($11, address),
            social_links = COALESCE($12, social_links),
            default_meta_title_suffix = COALESCE($13, default_meta_title_suffix),
            default_robots = COALESCE($14, default_robots),
            google_analytics_id = COALESCE($15, google_analytics_id),
            google_tag_manager_id = COALESCE($16, google_tag_manager_id),
            maintenance_mode = COALESCE($17, maintenance_mode),
            maintenance_message = COALESCE($18, maintenance_message),
            head_scripts = COALESCE($19, head_scripts),
            body_start_scripts = COALESCE($20, body_start_scripts),
            body_end_scripts = COALESCE($21, body_end_scripts),
            custom_css = COALESCE($22, custom_css),
            settings = COALESCE($23, settings),
            updated_by = $24,
            updated_at = NOW()
        WHERE id = '00000000-0000-0000-0000-000000000001'::uuid
        RETURNING id, site_name, site_tagline, site_description,
                  logo_light_id, logo_dark_id, favicon_id, og_default_image_id,
                  contact_email, support_email, phone, address, social_links,
                  default_meta_title_suffix, default_robots,
                  google_analytics_id, google_tag_manager_id,
                  maintenance_mode, maintenance_message,
                  head_scripts, body_start_scripts, body_end_scripts, custom_css,
                  settings, updated_at, updated_by
        "#,
    )
    .bind(&request.site_name)
    .bind(&request.site_tagline)
    .bind(&request.site_description)
    .bind(request.logo_light_id)
    .bind(request.logo_dark_id)
    .bind(request.favicon_id)
    .bind(request.og_default_image_id)
    .bind(&request.contact_email)
    .bind(&request.support_email)
    .bind(&request.phone)
    .bind(&request.address)
    .bind(&request.social_links)
    .bind(&request.default_meta_title_suffix)
    .bind(&request.default_robots)
    .bind(&request.google_analytics_id)
    .bind(&request.google_tag_manager_id)
    .bind(request.maintenance_mode)
    .bind(&request.maintenance_message)
    .bind(&request.head_scripts)
    .bind(&request.body_start_scripts)
    .bind(&request.body_end_scripts)
    .bind(&request.custom_css)
    .bind(&request.settings)
    .bind(updated_by)
    .fetch_one(pool)
    .await?;

    Ok(settings)
}

// ═══════════════════════════════════════════════════════════════════════════════════════
// NAVIGATION MENUS
// ═══════════════════════════════════════════════════════════════════════════════════════

/// List navigation menus
pub async fn list_navigation_menus(pool: &PgPool) -> Result<Vec<CmsNavigationMenu>> {
    let menus: Vec<CmsNavigationMenu> = sqlx::query_as(
        r#"
        SELECT id, name, slug, location, items, is_active, created_at, updated_at, created_by, updated_by
        FROM cms_navigation_menus
        ORDER BY name
        "#,
    )
    .fetch_all(pool)
    .await?;

    Ok(menus)
}

/// Get navigation menu by slug
pub async fn get_navigation_menu(pool: &PgPool, slug: &str) -> Result<Option<CmsNavigationMenu>> {
    let menu: Option<CmsNavigationMenu> = sqlx::query_as(
        r#"
        SELECT id, name, slug, location, items, is_active, created_at, updated_at, created_by, updated_by
        FROM cms_navigation_menus
        WHERE slug = $1
        "#,
    )
    .bind(slug)
    .fetch_optional(pool)
    .await?;

    Ok(menu)
}

/// Create navigation menu
pub async fn create_navigation_menu(
    pool: &PgPool,
    request: CreateNavigationMenuRequest,
    created_by: Option<Uuid>,
) -> Result<CmsNavigationMenu> {
    let slug = slug::slugify(&request.name);

    let menu: CmsNavigationMenu = sqlx::query_as(
        r#"
        INSERT INTO cms_navigation_menus (name, slug, location, items, created_by)
        VALUES ($1, $2, $3, $4, $5)
        RETURNING id, name, slug, location, items, is_active, created_at, updated_at, created_by, updated_by
        "#,
    )
    .bind(&request.name)
    .bind(&slug)
    .bind(&request.location)
    .bind(&request.items)
    .bind(created_by)
    .fetch_one(pool)
    .await?;

    Ok(menu)
}

/// Update navigation menu
pub async fn update_navigation_menu(
    pool: &PgPool,
    id: Uuid,
    request: UpdateNavigationMenuRequest,
    updated_by: Option<Uuid>,
) -> Result<CmsNavigationMenu> {
    let menu: CmsNavigationMenu = sqlx::query_as(
        r#"
        UPDATE cms_navigation_menus SET
            name = COALESCE($1, name),
            slug = COALESCE($2, slug),
            location = COALESCE($3, location),
            items = COALESCE($4, items),
            is_active = COALESCE($5, is_active),
            updated_by = $6
        WHERE id = $7
        RETURNING id, name, slug, location, items, is_active, created_at, updated_at, created_by, updated_by
        "#,
    )
    .bind(&request.name)
    .bind(request.name.as_ref().map(slug::slugify))
    .bind(&request.location)
    .bind(&request.items)
    .bind(request.is_active)
    .bind(updated_by)
    .bind(id)
    .fetch_one(pool)
    .await?;

    Ok(menu)
}

// ═══════════════════════════════════════════════════════════════════════════════════════
// REDIRECTS
// ═══════════════════════════════════════════════════════════════════════════════════════

/// List redirects
pub async fn list_redirects(pool: &PgPool) -> Result<Vec<CmsRedirect>> {
    let redirects: Vec<CmsRedirect> = sqlx::query_as(
        r#"
        SELECT id, source_path, target_path, status_code, is_regex, preserve_query_string,
               hit_count, last_hit_at, is_active, created_at, updated_at, created_by
        FROM cms_redirects
        ORDER BY source_path
        "#,
    )
    .fetch_all(pool)
    .await?;

    Ok(redirects)
}

/// Create redirect
pub async fn create_redirect(
    pool: &PgPool,
    request: CreateRedirectRequest,
    created_by: Option<Uuid>,
) -> Result<CmsRedirect> {
    let redirect: CmsRedirect = sqlx::query_as(
        r#"
        INSERT INTO cms_redirects (source_path, target_path, status_code, is_regex, preserve_query_string, created_by)
        VALUES ($1, $2, $3, $4, $5, $6)
        RETURNING id, source_path, target_path, status_code, is_regex, preserve_query_string,
                  hit_count, last_hit_at, is_active, created_at, updated_at, created_by
        "#,
    )
    .bind(&request.source_path)
    .bind(&request.target_path)
    .bind(request.status_code.unwrap_or(301))
    .bind(request.is_regex.unwrap_or(false))
    .bind(request.preserve_query_string.unwrap_or(true))
    .bind(created_by)
    .fetch_one(pool)
    .await?;

    Ok(redirect)
}

/// Delete redirect
pub async fn delete_redirect(pool: &PgPool, id: Uuid) -> Result<()> {
    sqlx::query("DELETE FROM cms_redirects WHERE id = $1")
        .bind(id)
        .execute(pool)
        .await?;

    Ok(())
}

/// Match redirect for a path
pub async fn match_redirect(pool: &PgPool, path: &str) -> Result<Option<CmsRedirect>> {
    // First try exact match
    let exact: Option<CmsRedirect> = sqlx::query_as(
        r#"
        SELECT id, source_path, target_path, status_code, is_regex, preserve_query_string,
               hit_count, last_hit_at, is_active, created_at, updated_at, created_by
        FROM cms_redirects
        WHERE source_path = $1 AND is_active = true AND is_regex = false
        "#,
    )
    .bind(path)
    .fetch_optional(pool)
    .await?;

    if exact.is_some() {
        // Update hit count
        sqlx::query("UPDATE cms_redirects SET hit_count = hit_count + 1, last_hit_at = NOW() WHERE source_path = $1")
            .bind(path)
            .execute(pool)
            .await?;
        return Ok(exact);
    }

    // TODO: Implement regex matching if needed
    Ok(None)
}

// ═══════════════════════════════════════════════════════════════════════════════════════
// AUDIT LOGGING
// ═══════════════════════════════════════════════════════════════════════════════════════

/// Log an audit event to the CMS audit log
#[allow(clippy::too_many_arguments)]
pub async fn log_cms_audit(
    pool: &PgPool,
    action: &str,
    entity_type: &str,
    entity_id: Option<Uuid>,
    user_id: Option<Uuid>,
    old_values: Option<JsonValue>,
    new_values: Option<JsonValue>,
    metadata: Option<JsonValue>,
) -> Result<Uuid> {
    let audit_id: Uuid = sqlx::query_scalar(
        r#"
        INSERT INTO cms_audit_log (action, entity_type, entity_id, user_id, old_values, new_values, metadata)
        VALUES ($1, $2, $3, $4, $5, $6, $7)
        RETURNING id
        "#,
    )
    .bind(action)
    .bind(entity_type)
    .bind(entity_id)
    .bind(user_id)
    .bind(&old_values)
    .bind(&new_values)
    .bind(&metadata)
    .fetch_one(pool)
    .await?;

    Ok(audit_id)
}
