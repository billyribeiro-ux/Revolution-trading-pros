//! Content Repository - Apple ICT 7+ Principal Engineer Grade
//!
//! Repository pattern implementation for content persistence.
//! Provides clean abstraction over database operations with:
//! - Type-safe queries via SQLx
//! - Optimistic locking support
//! - Search and filtering capabilities
//! - Revision management
//!
//! @version 2.0.0 - January 2026

use anyhow::Result;
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use serde_json::Value as JsonValue;
use sqlx::PgPool;
use uuid::Uuid;

use super::entity::{Content, ContentError, ContentStatus, ContentType};
use crate::models::cms::{
    CmsContent, CmsContentStatus, CmsContentSummary, CmsContentType, CmsRevision, ContentListQuery,
    PaginatedResponse, PaginationMeta,
};

/// Repository trait for content operations
#[async_trait]
pub trait ContentRepository: Send + Sync {
    /// Find content by ID
    async fn find_by_id(&self, id: Uuid) -> Result<Option<Content>>;

    /// Find content by slug and type
    async fn find_by_slug(
        &self,
        content_type: ContentType,
        slug: &str,
        locale: Option<&str>,
    ) -> Result<Option<Content>>;

    /// List content with filters
    async fn list(&self, query: ContentListQuery) -> Result<PaginatedResponse<CmsContentSummary>>;

    /// Create new content
    async fn create(&self, content: &Content, user_id: Option<Uuid>) -> Result<Content>;

    /// Update existing content
    async fn update(&self, content: &Content, user_id: Option<Uuid>) -> Result<Content>;

    /// Soft delete content
    async fn soft_delete(&self, id: Uuid, user_id: Option<Uuid>) -> Result<()>;

    /// Restore deleted content
    async fn restore(&self, id: Uuid, user_id: Option<Uuid>) -> Result<()>;

    /// Permanently delete content
    async fn hard_delete(&self, id: Uuid) -> Result<()>;

    /// Transition content status
    async fn transition_status(
        &self,
        id: Uuid,
        new_status: ContentStatus,
        user_id: Option<Uuid>,
        comment: Option<String>,
    ) -> Result<Content>;

    /// Create revision
    async fn create_revision(
        &self,
        content: &Content,
        user_id: Option<Uuid>,
    ) -> Result<CmsRevision>;

    /// List revisions for content
    async fn list_revisions(&self, content_id: Uuid) -> Result<Vec<CmsRevision>>;

    /// Restore revision
    async fn restore_revision(&self, content_id: Uuid, revision_id: Uuid) -> Result<Content>;

    /// Search content
    async fn search(
        &self,
        query: &str,
        content_types: Option<Vec<ContentType>>,
        limit: i32,
    ) -> Result<Vec<CmsContentSummary>>;

    /// Count content by type and status
    async fn count(
        &self,
        content_type: Option<ContentType>,
        status: Option<ContentStatus>,
    ) -> Result<i64>;

    /// Check if slug exists
    async fn slug_exists(
        &self,
        content_type: ContentType,
        slug: &str,
        exclude_id: Option<Uuid>,
    ) -> Result<bool>;
}

/// PostgreSQL implementation of ContentRepository
pub struct PgContentRepository {
    pool: PgPool,
}

impl PgContentRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    /// Convert CmsContent to domain Content
    fn to_domain(cms: CmsContent) -> Content {
        Content {
            id: cms.id,
            content_type: ContentType::from(cms.content_type),
            slug: cms.slug,
            locale: cms.locale,
            is_primary_locale: cms.is_primary_locale,
            parent_content_id: cms.parent_content_id,
            title: cms.title,
            subtitle: cms.subtitle,
            excerpt: cms.excerpt,
            content: cms.content,
            content_blocks: cms.content_blocks,
            featured_image_id: cms.featured_image_id,
            og_image_id: cms.og_image_id,
            gallery_ids: cms.gallery_ids.unwrap_or_default(),
            meta_title: cms.meta_title,
            meta_description: cms.meta_description,
            meta_keywords: cms.meta_keywords.unwrap_or_default(),
            canonical_url: cms.canonical_url,
            robots_directives: cms.robots_directives,
            structured_data: cms.structured_data,
            author_id: cms.author_id,
            contributors: cms.contributors.unwrap_or_default(),
            status: ContentStatus::from(cms.status),
            published_at: cms.published_at,
            scheduled_publish_at: cms.scheduled_publish_at,
            scheduled_unpublish_at: cms.scheduled_unpublish_at,
            primary_category_id: cms.primary_category_id,
            categories: cms.categories.unwrap_or_default(),
            custom_fields: cms.custom_fields,
            template: cms.template.unwrap_or_else(|| "default".to_string()),
            deleted_at: cms.deleted_at,
            version: cms.version,
            created_at: cms.created_at,
            updated_at: cms.updated_at,
            created_by: cms.created_by,
            updated_by: cms.updated_by,
        }
    }
}

#[async_trait]
impl ContentRepository for PgContentRepository {
    async fn find_by_id(&self, id: Uuid) -> Result<Option<Content>> {
        let result = sqlx::query_as::<_, CmsContent>(
            r#"
            SELECT * FROM cms_content
            WHERE id = $1 AND deleted_at IS NULL
            "#,
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(result.map(Self::to_domain))
    }

    async fn find_by_slug(
        &self,
        content_type: ContentType,
        slug: &str,
        locale: Option<&str>,
    ) -> Result<Option<Content>> {
        let cms_type: CmsContentType = content_type.into();
        let locale = locale.unwrap_or("en");

        let result = sqlx::query_as::<_, CmsContent>(
            r#"
            SELECT * FROM cms_content
            WHERE content_type = $1
              AND slug = $2
              AND locale = $3
              AND deleted_at IS NULL
            "#,
        )
        .bind(cms_type)
        .bind(slug)
        .bind(locale)
        .fetch_optional(&self.pool)
        .await?;

        Ok(result.map(Self::to_domain))
    }

    async fn list(&self, query: ContentListQuery) -> Result<PaginatedResponse<CmsContentSummary>> {
        let limit = query.limit.unwrap_or(20).min(100);
        let offset = query.offset.unwrap_or(0);
        let sort_by = query
            .sort_by
            .clone()
            .unwrap_or_else(|| "updated_at".to_string());
        let sort_order = query
            .sort_order
            .clone()
            .unwrap_or_else(|| "desc".to_string());

        let order_direction = if sort_order.to_lowercase() == "asc" {
            "ASC"
        } else {
            "DESC"
        };

        // Build dynamic query
        let base_query = format!(
            r#"
            SELECT id, content_type, slug, locale, title, excerpt, featured_image_id,
                   author_id, status, published_at, created_at, updated_at
            FROM cms_content
            WHERE deleted_at IS NULL
              AND ($1::cms_content_type IS NULL OR content_type = $1)
              AND ($2::cms_content_status IS NULL OR status = $2)
              AND ($3::uuid IS NULL OR author_id = $3)
              AND ($4::varchar IS NULL OR locale = $4)
              AND ($5::varchar IS NULL OR title ILIKE '%' || $5 || '%')
            ORDER BY {} {}
            LIMIT $6 OFFSET $7
            "#,
            sort_by, order_direction
        );

        let count_query = r#"
            SELECT COUNT(*)
            FROM cms_content
            WHERE deleted_at IS NULL
              AND ($1::cms_content_type IS NULL OR content_type = $1)
              AND ($2::cms_content_status IS NULL OR status = $2)
              AND ($3::uuid IS NULL OR author_id = $3)
              AND ($4::varchar IS NULL OR locale = $4)
              AND ($5::varchar IS NULL OR title ILIKE '%' || $5 || '%')
        "#;

        let cms_type: Option<CmsContentType> = query.content_type;
        let cms_status: Option<CmsContentStatus> = query.status;

        let items = sqlx::query_as::<_, CmsContentSummary>(&base_query)
            .bind(&cms_type)
            .bind(&cms_status)
            .bind(&query.author_id)
            .bind(&query.locale)
            .bind(&query.search)
            .bind(limit)
            .bind(offset)
            .fetch_all(&self.pool)
            .await?;

        let total: (i64,) = sqlx::query_as(count_query)
            .bind(&cms_type)
            .bind(&cms_status)
            .bind(&query.author_id)
            .bind(&query.locale)
            .bind(&query.search)
            .fetch_one(&self.pool)
            .await?;

        Ok(PaginatedResponse {
            data: items,
            meta: PaginationMeta {
                total: total.0,
                limit,
                offset,
                has_more: offset + limit < total.0,
            },
        })
    }

    async fn create(&self, content: &Content, user_id: Option<Uuid>) -> Result<Content> {
        let cms_type: CmsContentType = content.content_type.into();
        let cms_status: CmsContentStatus = content.status.into();

        let result = sqlx::query_as::<_, CmsContent>(
            r#"
            INSERT INTO cms_content (
                id, content_type, slug, locale, is_primary_locale, parent_content_id,
                title, subtitle, excerpt, content, content_blocks,
                featured_image_id, og_image_id, gallery_ids,
                meta_title, meta_description, meta_keywords, canonical_url, robots_directives,
                structured_data, author_id, contributors, status,
                published_at, scheduled_publish_at, scheduled_unpublish_at,
                primary_category_id, categories, custom_fields, template,
                version, created_by, updated_by
            ) VALUES (
                $1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14,
                $15, $16, $17, $18, $19, $20, $21, $22, $23, $24, $25, $26,
                $27, $28, $29, $30, $31, $32, $32
            )
            RETURNING *
            "#,
        )
        .bind(content.id)
        .bind(cms_type)
        .bind(&content.slug)
        .bind(&content.locale)
        .bind(content.is_primary_locale)
        .bind(content.parent_content_id)
        .bind(&content.title)
        .bind(&content.subtitle)
        .bind(&content.excerpt)
        .bind(&content.content)
        .bind(&content.content_blocks)
        .bind(content.featured_image_id)
        .bind(content.og_image_id)
        .bind(&content.gallery_ids)
        .bind(&content.meta_title)
        .bind(&content.meta_description)
        .bind(&content.meta_keywords)
        .bind(&content.canonical_url)
        .bind(&content.robots_directives)
        .bind(&content.structured_data)
        .bind(content.author_id)
        .bind(&content.contributors)
        .bind(cms_status)
        .bind(content.published_at)
        .bind(content.scheduled_publish_at)
        .bind(content.scheduled_unpublish_at)
        .bind(content.primary_category_id)
        .bind(&content.categories)
        .bind(&content.custom_fields)
        .bind(&content.template)
        .bind(content.version)
        .bind(user_id)
        .fetch_one(&self.pool)
        .await?;

        Ok(Self::to_domain(result))
    }

    async fn update(&self, content: &Content, user_id: Option<Uuid>) -> Result<Content> {
        let cms_type: CmsContentType = content.content_type.into();
        let cms_status: CmsContentStatus = content.status.into();

        // Optimistic locking check
        let result = sqlx::query_as::<_, CmsContent>(
            r#"
            UPDATE cms_content SET
                content_type = $2,
                slug = $3,
                locale = $4,
                is_primary_locale = $5,
                parent_content_id = $6,
                title = $7,
                subtitle = $8,
                excerpt = $9,
                content = $10,
                content_blocks = $11,
                featured_image_id = $12,
                og_image_id = $13,
                gallery_ids = $14,
                meta_title = $15,
                meta_description = $16,
                meta_keywords = $17,
                canonical_url = $18,
                robots_directives = $19,
                structured_data = $20,
                author_id = $21,
                contributors = $22,
                status = $23,
                published_at = $24,
                scheduled_publish_at = $25,
                scheduled_unpublish_at = $26,
                primary_category_id = $27,
                categories = $28,
                custom_fields = $29,
                template = $30,
                version = version + 1,
                updated_at = NOW(),
                updated_by = $31
            WHERE id = $1 AND version = $32 AND deleted_at IS NULL
            RETURNING *
            "#,
        )
        .bind(content.id)
        .bind(cms_type)
        .bind(&content.slug)
        .bind(&content.locale)
        .bind(content.is_primary_locale)
        .bind(content.parent_content_id)
        .bind(&content.title)
        .bind(&content.subtitle)
        .bind(&content.excerpt)
        .bind(&content.content)
        .bind(&content.content_blocks)
        .bind(content.featured_image_id)
        .bind(content.og_image_id)
        .bind(&content.gallery_ids)
        .bind(&content.meta_title)
        .bind(&content.meta_description)
        .bind(&content.meta_keywords)
        .bind(&content.canonical_url)
        .bind(&content.robots_directives)
        .bind(&content.structured_data)
        .bind(content.author_id)
        .bind(&content.contributors)
        .bind(cms_status)
        .bind(content.published_at)
        .bind(content.scheduled_publish_at)
        .bind(content.scheduled_unpublish_at)
        .bind(content.primary_category_id)
        .bind(&content.categories)
        .bind(&content.custom_fields)
        .bind(&content.template)
        .bind(user_id)
        .bind(content.version)
        .fetch_optional(&self.pool)
        .await?;

        match result {
            Some(cms) => Ok(Self::to_domain(cms)),
            None => {
                // Check if not found or version conflict
                let exists = self.find_by_id(content.id).await?;
                match exists {
                    Some(current) => Err(ContentError::VersionConflict {
                        expected: content.version,
                        actual: current.version,
                    }
                    .into()),
                    None => Err(ContentError::NotFound(content.id).into()),
                }
            }
        }
    }

    async fn soft_delete(&self, id: Uuid, user_id: Option<Uuid>) -> Result<()> {
        sqlx::query(
            r#"
            UPDATE cms_content
            SET deleted_at = NOW(), updated_at = NOW(), updated_by = $2
            WHERE id = $1 AND deleted_at IS NULL
            "#,
        )
        .bind(id)
        .bind(user_id)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    async fn restore(&self, id: Uuid, user_id: Option<Uuid>) -> Result<()> {
        sqlx::query(
            r#"
            UPDATE cms_content
            SET deleted_at = NULL, updated_at = NOW(), updated_by = $2
            WHERE id = $1 AND deleted_at IS NOT NULL
            "#,
        )
        .bind(id)
        .bind(user_id)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    async fn hard_delete(&self, id: Uuid) -> Result<()> {
        sqlx::query("DELETE FROM cms_content WHERE id = $1")
            .bind(id)
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    async fn transition_status(
        &self,
        id: Uuid,
        new_status: ContentStatus,
        user_id: Option<Uuid>,
        comment: Option<String>,
    ) -> Result<Content> {
        let cms_status: CmsContentStatus = new_status.into();

        // Handle published_at for publishing transitions
        let published_at = if new_status == ContentStatus::Published {
            Some(Utc::now())
        } else {
            None
        };

        let result = sqlx::query_as::<_, CmsContent>(
            r#"
            UPDATE cms_content
            SET status = $2,
                published_at = COALESCE($3, published_at),
                version = version + 1,
                updated_at = NOW(),
                updated_by = $4
            WHERE id = $1 AND deleted_at IS NULL
            RETURNING *
            "#,
        )
        .bind(id)
        .bind(cms_status)
        .bind(published_at)
        .bind(user_id)
        .fetch_optional(&self.pool)
        .await?;

        match result {
            Some(cms) => {
                // Log workflow transition
                if let Some(comment) = comment {
                    sqlx::query(
                        r#"
                        INSERT INTO cms_audit_log (action, entity_type, entity_id, user_id, metadata)
                        VALUES ('status_transition', 'content', $1, $2, $3)
                        "#,
                    )
                    .bind(id)
                    .bind(user_id)
                    .bind(serde_json::json!({
                        "new_status": new_status,
                        "comment": comment
                    }))
                    .execute(&self.pool)
                    .await?;
                }

                Ok(Self::to_domain(cms))
            }
            None => Err(ContentError::NotFound(id).into()),
        }
    }

    async fn create_revision(
        &self,
        content: &Content,
        user_id: Option<Uuid>,
    ) -> Result<CmsRevision> {
        // Get next revision number
        let max_revision: Option<(i32,)> =
            sqlx::query_as("SELECT MAX(revision_number) FROM cms_revisions WHERE content_id = $1")
                .bind(content.id)
                .fetch_optional(&self.pool)
                .await?;

        let next_revision = max_revision.and_then(|r| r.0.into()).unwrap_or(0) + 1;

        // Serialize content data
        let data = serde_json::to_value(content)?;

        let revision = sqlx::query_as::<_, CmsRevision>(
            r#"
            INSERT INTO cms_revisions (content_id, revision_number, is_current, data, created_by)
            VALUES ($1, $2, true, $3, $4)
            RETURNING *
            "#,
        )
        .bind(content.id)
        .bind(next_revision)
        .bind(&data)
        .bind(user_id)
        .fetch_one(&self.pool)
        .await?;

        // Mark previous revisions as not current
        sqlx::query(
            "UPDATE cms_revisions SET is_current = false WHERE content_id = $1 AND id != $2",
        )
        .bind(content.id)
        .bind(revision.id)
        .execute(&self.pool)
        .await?;

        Ok(revision)
    }

    async fn list_revisions(&self, content_id: Uuid) -> Result<Vec<CmsRevision>> {
        let revisions = sqlx::query_as::<_, CmsRevision>(
            r#"
            SELECT * FROM cms_revisions
            WHERE content_id = $1
            ORDER BY revision_number DESC
            "#,
        )
        .bind(content_id)
        .fetch_all(&self.pool)
        .await?;

        Ok(revisions)
    }

    async fn restore_revision(&self, content_id: Uuid, revision_id: Uuid) -> Result<Content> {
        // Get revision data
        let revision = sqlx::query_as::<_, CmsRevision>(
            "SELECT * FROM cms_revisions WHERE id = $1 AND content_id = $2",
        )
        .bind(revision_id)
        .bind(content_id)
        .fetch_optional(&self.pool)
        .await?
        .ok_or_else(|| anyhow::anyhow!("Revision not found"))?;

        // Parse revision data and update content
        let content: Content = serde_json::from_value(revision.data)?;
        self.update(&content, None).await
    }

    async fn search(
        &self,
        query: &str,
        content_types: Option<Vec<ContentType>>,
        limit: i32,
    ) -> Result<Vec<CmsContentSummary>> {
        let cms_types: Option<Vec<CmsContentType>> =
            content_types.map(|types| types.into_iter().map(|t| t.into()).collect());

        let results = sqlx::query_as::<_, CmsContentSummary>(
            r#"
            SELECT id, content_type, slug, locale, title, excerpt, featured_image_id,
                   author_id, status, published_at, created_at, updated_at
            FROM cms_content
            WHERE deleted_at IS NULL
              AND status = 'published'
              AND ($2::cms_content_type[] IS NULL OR content_type = ANY($2))
              AND (
                title ILIKE '%' || $1 || '%'
                OR excerpt ILIKE '%' || $1 || '%'
                OR content ILIKE '%' || $1 || '%'
              )
            ORDER BY
              CASE WHEN title ILIKE $1 || '%' THEN 1
                   WHEN title ILIKE '%' || $1 || '%' THEN 2
                   ELSE 3
              END,
              published_at DESC
            LIMIT $3
            "#,
        )
        .bind(query)
        .bind(&cms_types)
        .bind(limit)
        .fetch_all(&self.pool)
        .await?;

        Ok(results)
    }

    async fn count(
        &self,
        content_type: Option<ContentType>,
        status: Option<ContentStatus>,
    ) -> Result<i64> {
        let cms_type: Option<CmsContentType> = content_type.map(|t| t.into());
        let cms_status: Option<CmsContentStatus> = status.map(|s| s.into());

        let result: (i64,) = sqlx::query_as(
            r#"
            SELECT COUNT(*)
            FROM cms_content
            WHERE deleted_at IS NULL
              AND ($1::cms_content_type IS NULL OR content_type = $1)
              AND ($2::cms_content_status IS NULL OR status = $2)
            "#,
        )
        .bind(&cms_type)
        .bind(&cms_status)
        .fetch_one(&self.pool)
        .await?;

        Ok(result.0)
    }

    async fn slug_exists(
        &self,
        content_type: ContentType,
        slug: &str,
        exclude_id: Option<Uuid>,
    ) -> Result<bool> {
        let cms_type: CmsContentType = content_type.into();

        let result: (bool,) = sqlx::query_as(
            r#"
            SELECT EXISTS(
                SELECT 1 FROM cms_content
                WHERE content_type = $1
                  AND slug = $2
                  AND deleted_at IS NULL
                  AND ($3::uuid IS NULL OR id != $3)
            )
            "#,
        )
        .bind(cms_type)
        .bind(slug)
        .bind(exclude_id)
        .fetch_one(&self.pool)
        .await?;

        Ok(result.0)
    }
}
