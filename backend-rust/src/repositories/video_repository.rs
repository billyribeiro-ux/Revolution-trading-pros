//! Video Repository
//!
//! ICT 11+ Principal Engineer Grade
//! Database operations for room_daily_videos with Previous/Next navigation

use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    errors::AppError,
    models::{
        RoomDailyVideo, TraderInfo, VideoListQuery, VideoNavReference, 
        VideoWithNavigation, VideoWithTrader, CreateVideo, UpdateVideo,
    },
};

pub struct VideoRepository<'a> {
    pool: &'a PgPool,
}

impl<'a> VideoRepository<'a> {
    pub fn new(pool: &'a PgPool) -> Self {
        Self { pool }
    }

    /// Count total videos matching query
    pub async fn count(&self, query: &VideoListQuery) -> Result<i64, AppError> {
        let mut sql = String::from(
            "SELECT COUNT(*) FROM room_daily_videos v
             LEFT JOIN trading_rooms tr ON v.trading_room_id = tr.id
             WHERE 1=1"
        );
        
        if query.room_slug.is_some() {
            sql.push_str(" AND tr.slug = $1");
        }
        if query.is_published.unwrap_or(true) {
            sql.push_str(" AND v.is_published = true");
        }
        if query.search.is_some() {
            sql.push_str(" AND v.title ILIKE '%' || $2 || '%'");
        }

        let count: (i64,) = if let Some(ref room_slug) = query.room_slug {
            if let Some(ref search) = query.search {
                sqlx::query_as(&sql)
                    .bind(room_slug)
                    .bind(search)
                    .fetch_one(self.pool)
                    .await?
            } else {
                sqlx::query_as(&sql)
                    .bind(room_slug)
                    .fetch_one(self.pool)
                    .await?
            }
        } else if let Some(ref search) = query.search {
            // Adjust query for search without room_slug
            let sql = "SELECT COUNT(*) FROM room_daily_videos v
                       WHERE v.is_published = true AND v.title ILIKE '%' || $1 || '%'";
            sqlx::query_as(sql)
                .bind(search)
                .fetch_one(self.pool)
                .await?
        } else {
            let sql = "SELECT COUNT(*) FROM room_daily_videos v WHERE v.is_published = true";
            sqlx::query_as(sql)
                .fetch_one(self.pool)
                .await?
        };

        Ok(count.0)
    }

    /// List videos with pagination and filters
    pub async fn list(&self, query: &VideoListQuery) -> Result<Vec<VideoWithTrader>, AppError> {
        // Build dynamic query based on filters
        let videos: Vec<RoomDailyVideo> = if let Some(ref room_slug) = query.room_slug {
            sqlx::query_as::<_, RoomDailyVideo>(
                r#"
                SELECT v.id, v.trading_room_id, v.trader_id, v.title, v.slug,
                       v.description, v.video_url, v.thumbnail_url, v.duration,
                       v.is_published, v.is_featured, v.views_count,
                       v.published_at, v.created_at
                FROM room_daily_videos v
                INNER JOIN trading_rooms tr ON v.trading_room_id = tr.id
                WHERE tr.slug = $1
                  AND v.is_published = true
                ORDER BY v.published_at DESC
                LIMIT $2 OFFSET $3
                "#
            )
            .bind(room_slug)
            .bind(query.per_page())
            .bind(query.offset())
            .fetch_all(self.pool)
            .await?
        } else {
            sqlx::query_as::<_, RoomDailyVideo>(
                r#"
                SELECT id, trading_room_id, trader_id, title, slug,
                       description, video_url, thumbnail_url, duration,
                       is_published, is_featured, views_count,
                       published_at, created_at
                FROM room_daily_videos
                WHERE is_published = true
                ORDER BY published_at DESC
                LIMIT $1 OFFSET $2
                "#
            )
            .bind(query.per_page())
            .bind(query.offset())
            .fetch_all(self.pool)
            .await?
        };

        // Fetch trader info for each video
        let mut result = Vec::with_capacity(videos.len());
        for video in videos {
            let trader = if let Some(trader_id) = video.trader_id {
                self.get_trader_info(trader_id).await?
            } else {
                None
            };

            result.push(VideoWithTrader {
                id: video.id,
                trading_room_id: video.trading_room_id,
                title: video.title,
                slug: video.slug,
                description: video.description,
                video_url: video.video_url,
                thumbnail_url: video.thumbnail_url,
                duration: video.duration,
                is_published: video.is_published,
                is_featured: video.is_featured,
                views_count: video.views_count,
                published_at: video.published_at,
                created_at: video.created_at,
                trader,
            });
        }

        Ok(result)
    }

    /// Find video by ID
    pub async fn find(&self, id: Uuid) -> Result<Option<RoomDailyVideo>, AppError> {
        let video = sqlx::query_as::<_, RoomDailyVideo>(
            r#"
            SELECT id, trading_room_id, trader_id, title, slug,
                   description, video_url, thumbnail_url, duration,
                   is_published, is_featured, views_count,
                   published_at, created_at
            FROM room_daily_videos
            WHERE id = $1
            "#
        )
        .bind(id)
        .fetch_optional(self.pool)
        .await?;

        Ok(video)
    }

    /// Find video by slug
    pub async fn find_by_slug(&self, slug: &str) -> Result<Option<RoomDailyVideo>, AppError> {
        let video = sqlx::query_as::<_, RoomDailyVideo>(
            r#"
            SELECT id, trading_room_id, trader_id, title, slug,
                   description, video_url, thumbnail_url, duration,
                   is_published, is_featured, views_count,
                   published_at, created_at
            FROM room_daily_videos
            WHERE slug = $1 AND is_published = true
            "#
        )
        .bind(slug)
        .fetch_optional(self.pool)
        .await?;

        Ok(video)
    }

    /// Find video by slug within a specific trading room
    pub async fn find_by_room_and_slug(
        &self,
        room_slug: &str,
        video_slug: &str,
    ) -> Result<Option<RoomDailyVideo>, AppError> {
        let video = sqlx::query_as::<_, RoomDailyVideo>(
            r#"
            SELECT v.id, v.trading_room_id, v.trader_id, v.title, v.slug,
                   v.description, v.video_url, v.thumbnail_url, v.duration,
                   v.is_published, v.is_featured, v.views_count,
                   v.published_at, v.created_at
            FROM room_daily_videos v
            INNER JOIN trading_rooms tr ON v.trading_room_id = tr.id
            WHERE tr.slug = $1 AND v.slug = $2 AND v.is_published = true
            "#
        )
        .bind(room_slug)
        .bind(video_slug)
        .fetch_optional(self.pool)
        .await?;

        Ok(video)
    }

    /// Get video with full navigation (previous/next)
    /// Videos are ordered by published_at DESC (newest first)
    /// Previous = older video (published before current)
    /// Next = newer video (published after current)
    pub async fn get_with_navigation(
        &self,
        room_slug: &str,
        video_slug: &str,
    ) -> Result<Option<VideoWithNavigation>, AppError> {
        // First, get the current video
        let video = match self.find_by_room_and_slug(room_slug, video_slug).await? {
            Some(v) => v,
            None => return Ok(None),
        };

        // Get previous video (older - published before current)
        let previous = sqlx::query_as::<_, (Uuid, String, String)>(
            r#"
            SELECT v.id, v.title, v.slug
            FROM room_daily_videos v
            WHERE v.trading_room_id = $1
              AND v.is_published = true
              AND v.published_at < $2
            ORDER BY v.published_at DESC
            LIMIT 1
            "#
        )
        .bind(video.trading_room_id)
        .bind(video.published_at)
        .fetch_optional(self.pool)
        .await?
        .map(|(id, title, slug)| VideoNavReference { id, title, slug });

        // Get next video (newer - published after current)
        let next = sqlx::query_as::<_, (Uuid, String, String)>(
            r#"
            SELECT v.id, v.title, v.slug
            FROM room_daily_videos v
            WHERE v.trading_room_id = $1
              AND v.is_published = true
              AND v.published_at > $2
            ORDER BY v.published_at ASC
            LIMIT 1
            "#
        )
        .bind(video.trading_room_id)
        .bind(video.published_at)
        .fetch_optional(self.pool)
        .await?
        .map(|(id, title, slug)| VideoNavReference { id, title, slug });

        // Get trader info
        let trader = if let Some(trader_id) = video.trader_id {
            self.get_trader_info(trader_id).await?
        } else {
            None
        };

        let video_with_trader = VideoWithTrader {
            id: video.id,
            trading_room_id: video.trading_room_id,
            title: video.title,
            slug: video.slug,
            description: video.description,
            video_url: video.video_url,
            thumbnail_url: video.thumbnail_url,
            duration: video.duration,
            is_published: video.is_published,
            is_featured: video.is_featured,
            views_count: video.views_count,
            published_at: video.published_at,
            created_at: video.created_at,
            trader,
        };

        Ok(Some(VideoWithNavigation {
            video: video_with_trader,
            previous_video: previous,
            next_video: next,
        }))
    }

    /// Get trader info by ID
    async fn get_trader_info(&self, trader_id: Uuid) -> Result<Option<TraderInfo>, AppError> {
        let trader = sqlx::query_as::<_, (Uuid, String, String, Option<String>)>(
            "SELECT id, name, slug, avatar_url FROM room_traders WHERE id = $1 AND is_active = true"
        )
        .bind(trader_id)
        .fetch_optional(self.pool)
        .await?
        .map(|(id, name, slug, avatar_url)| TraderInfo {
            id,
            name,
            slug,
            avatar_url,
        });

        Ok(trader)
    }

    /// Create a new video
    pub async fn create(&self, data: CreateVideo) -> Result<RoomDailyVideo, AppError> {
        let slug = slugify(&data.title);
        let published_at = data.published_at.unwrap_or_else(chrono::Utc::now);

        let video = sqlx::query_as::<_, RoomDailyVideo>(
            r#"
            INSERT INTO room_daily_videos (
                trading_room_id, trader_id, title, slug, description,
                video_url, thumbnail_url, duration, is_published, is_featured,
                views_count, published_at, created_at
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, 0, $11, NOW())
            RETURNING id, trading_room_id, trader_id, title, slug, description,
                      video_url, thumbnail_url, duration, is_published, is_featured,
                      views_count, published_at, created_at
            "#
        )
        .bind(data.trading_room_id)
        .bind(data.trader_id)
        .bind(&data.title)
        .bind(&slug)
        .bind(&data.description)
        .bind(&data.video_url)
        .bind(&data.thumbnail_url)
        .bind(data.duration)
        .bind(data.is_published.unwrap_or(true))
        .bind(data.is_featured.unwrap_or(false))
        .bind(published_at)
        .fetch_one(self.pool)
        .await?;

        Ok(video)
    }

    /// Update a video
    pub async fn update(&self, id: Uuid, data: UpdateVideo) -> Result<Option<RoomDailyVideo>, AppError> {
        // Build dynamic update query
        let video = sqlx::query_as::<_, RoomDailyVideo>(
            r#"
            UPDATE room_daily_videos SET
                trader_id = COALESCE($2, trader_id),
                title = COALESCE($3, title),
                slug = COALESCE($4, slug),
                description = COALESCE($5, description),
                video_url = COALESCE($6, video_url),
                thumbnail_url = COALESCE($7, thumbnail_url),
                duration = COALESCE($8, duration),
                is_published = COALESCE($9, is_published),
                is_featured = COALESCE($10, is_featured),
                published_at = COALESCE($11, published_at)
            WHERE id = $1
            RETURNING id, trading_room_id, trader_id, title, slug, description,
                      video_url, thumbnail_url, duration, is_published, is_featured,
                      views_count, published_at, created_at
            "#
        )
        .bind(id)
        .bind(data.trader_id)
        .bind(&data.title)
        .bind(data.title.as_ref().map(|t| slugify(t)))
        .bind(&data.description)
        .bind(&data.video_url)
        .bind(&data.thumbnail_url)
        .bind(data.duration)
        .bind(data.is_published)
        .bind(data.is_featured)
        .bind(data.published_at)
        .fetch_optional(self.pool)
        .await?;

        Ok(video)
    }

    /// Delete a video
    pub async fn delete(&self, id: Uuid) -> Result<bool, AppError> {
        let result = sqlx::query("DELETE FROM room_daily_videos WHERE id = $1")
            .bind(id)
            .execute(self.pool)
            .await?;

        Ok(result.rows_affected() > 0)
    }

    /// Increment view count
    pub async fn increment_views(&self, id: Uuid) -> Result<(), AppError> {
        sqlx::query("UPDATE room_daily_videos SET views_count = views_count + 1 WHERE id = $1")
            .bind(id)
            .execute(self.pool)
            .await?;

        Ok(())
    }
}

/// Generate URL-safe slug from title
fn slugify(title: &str) -> String {
    title
        .to_lowercase()
        .chars()
        .map(|c| if c.is_alphanumeric() { c } else { '-' })
        .collect::<String>()
        .split('-')
        .filter(|s| !s.is_empty())
        .collect::<Vec<_>>()
        .join("-")
}
