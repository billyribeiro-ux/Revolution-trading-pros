//! CMS Preview Token Service - Apple ICT 7 Principal Engineer Grade
//!
//! Shareable preview links for unpublished content.
//! Enables stakeholder review without publishing.
//!
//! Features:
//! - Time-based expiration
//! - View count limits
//! - Secure token generation
//! - Access tracking
//!
//! @version 1.0.0 - January 27, 2026

use anyhow::Result;
use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;

// ═══════════════════════════════════════════════════════════════════════════════════════
// TYPES
// ═══════════════════════════════════════════════════════════════════════════════════════

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct PreviewToken {
    pub id: Uuid,
    pub content_id: Uuid,
    pub token: Uuid,
    pub max_views: Option<i32>,
    pub view_count: i32,
    pub expires_at: DateTime<Utc>,
    pub created_by: Option<i64>,
    pub created_at: DateTime<Utc>,
    pub last_accessed_at: Option<DateTime<Utc>>,
}

// ═══════════════════════════════════════════════════════════════════════════════════════
// PREVIEW TOKEN FUNCTIONS
// ═══════════════════════════════════════════════════════════════════════════════════════

/// Generate a preview token for content
pub async fn generate_preview_token(
    pool: &PgPool,
    content_id: Uuid,
    created_by: i64,
    expires_in_hours: i64,
    max_views: Option<i32>,
) -> Result<PreviewToken> {
    let expires_at = Utc::now() + Duration::hours(expires_in_hours);

    let token: PreviewToken = sqlx::query_as(
        r#"
        INSERT INTO cms_preview_tokens (content_id, expires_at, max_views, created_by)
        VALUES ($1, $2, $3, $4)
        RETURNING id, content_id, token, max_views, view_count, expires_at, created_by, created_at, last_accessed_at
        "#,
    )
    .bind(content_id)
    .bind(expires_at)
    .bind(max_views)
    .bind(created_by)
    .fetch_one(pool)
    .await?;

    Ok(token)
}

/// Validate and track preview token access
pub async fn validate_preview_token(pool: &PgPool, token: Uuid) -> Result<Option<PreviewToken>> {
    // Get token and increment view count
    let preview: Option<PreviewToken> = sqlx::query_as(
        r#"
        UPDATE cms_preview_tokens
        SET view_count = view_count + 1,
            last_accessed_at = NOW()
        WHERE token = $1
          AND expires_at > NOW()
          AND (max_views IS NULL OR view_count < max_views)
        RETURNING id, content_id, token, max_views, view_count, expires_at, created_by, created_at, last_accessed_at
        "#,
    )
    .bind(token)
    .fetch_optional(pool)
    .await?;

    Ok(preview)
}

/// Get preview token by token UUID
pub async fn get_preview_token(pool: &PgPool, token: Uuid) -> Result<Option<PreviewToken>> {
    let preview: Option<PreviewToken> = sqlx::query_as(
        r#"
        SELECT id, content_id, token, max_views, view_count, expires_at, created_by, created_at, last_accessed_at
        FROM cms_preview_tokens
        WHERE token = $1
        "#,
    )
    .bind(token)
    .fetch_optional(pool)
    .await?;

    Ok(preview)
}

/// Get all preview tokens for content
pub async fn get_content_preview_tokens(
    pool: &PgPool,
    content_id: Uuid,
) -> Result<Vec<PreviewToken>> {
    let tokens: Vec<PreviewToken> = sqlx::query_as(
        r#"
        SELECT id, content_id, token, max_views, view_count, expires_at, created_by, created_at, last_accessed_at
        FROM cms_preview_tokens
        WHERE content_id = $1
        ORDER BY created_at DESC
        "#,
    )
    .bind(content_id)
    .fetch_all(pool)
    .await?;

    Ok(tokens)
}

/// Revoke a preview token
pub async fn revoke_preview_token(pool: &PgPool, token: Uuid) -> Result<bool> {
    let result = sqlx::query(
        r#"
        DELETE FROM cms_preview_tokens
        WHERE token = $1
        "#,
    )
    .bind(token)
    .execute(pool)
    .await?;

    Ok(result.rows_affected() > 0)
}

/// Revoke all preview tokens for content
pub async fn revoke_content_preview_tokens(pool: &PgPool, content_id: Uuid) -> Result<i64> {
    let result = sqlx::query(
        r#"
        DELETE FROM cms_preview_tokens
        WHERE content_id = $1
        "#,
    )
    .bind(content_id)
    .execute(pool)
    .await?;

    Ok(result.rows_affected() as i64)
}

/// Clean up expired preview tokens
pub async fn cleanup_expired_tokens(pool: &PgPool) -> Result<i64> {
    let result = sqlx::query(
        r#"
        DELETE FROM cms_preview_tokens
        WHERE expires_at < NOW()
           OR (max_views IS NOT NULL AND view_count >= max_views)
        "#,
    )
    .execute(pool)
    .await?;

    Ok(result.rows_affected() as i64)
}

/// Get active preview token count for content
pub async fn get_active_token_count(pool: &PgPool, content_id: Uuid) -> Result<i64> {
    let count: (i64,) = sqlx::query_as(
        r#"
        SELECT COUNT(*)
        FROM cms_preview_tokens
        WHERE content_id = $1
          AND expires_at > NOW()
          AND (max_views IS NULL OR view_count < max_views)
        "#,
    )
    .bind(content_id)
    .fetch_one(pool)
    .await?;

    Ok(count.0)
}

/// Extend preview token expiration
pub async fn extend_preview_token(
    pool: &PgPool,
    token: Uuid,
    additional_hours: i64,
) -> Result<PreviewToken> {
    let preview: PreviewToken = sqlx::query_as(
        r#"
        UPDATE cms_preview_tokens
        SET expires_at = expires_at + INTERVAL '1 hour' * $1
        WHERE token = $2
        RETURNING id, content_id, token, max_views, view_count, expires_at, created_by, created_at, last_accessed_at
        "#,
    )
    .bind(additional_hours)
    .bind(token)
    .fetch_one(pool)
    .await?;

    Ok(preview)
}
