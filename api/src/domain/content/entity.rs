//! Content Entity - Apple ICT 7+ Principal Engineer Grade
//!
//! Rich domain entity with business logic, status transitions,
//! and validation rules. Implements the Content aggregate root.
//!
//! @version 2.0.0 - January 2026

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use thiserror::Error;
use uuid::Uuid;

use crate::models::cms::{CmsContentStatus, CmsContentType, CmsUserRole};

/// Content transition errors
#[derive(Debug, Error)]
pub enum ContentError {
    #[error("Invalid status transition from {from:?} to {to:?}")]
    InvalidTransition {
        from: ContentStatus,
        to: ContentStatus,
    },

    #[error("User with role {role:?} cannot perform action {action}")]
    PermissionDenied { role: CmsUserRole, action: String },

    #[error("Content validation failed: {0}")]
    ValidationFailed(String),

    #[error("Content not found: {0}")]
    NotFound(Uuid),

    #[error("Optimistic locking conflict: expected version {expected}, found {actual}")]
    VersionConflict { expected: i32, actual: i32 },

    #[error("Cannot {action} content in {status:?} status")]
    InvalidStatusForAction {
        status: ContentStatus,
        action: String,
    },
}

/// Content status with transition logic (mirrors database enum)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ContentStatus {
    Draft,
    InReview,
    Approved,
    Scheduled,
    Published,
    Archived,
}

impl From<CmsContentStatus> for ContentStatus {
    fn from(status: CmsContentStatus) -> Self {
        match status {
            CmsContentStatus::Draft => ContentStatus::Draft,
            CmsContentStatus::InReview => ContentStatus::InReview,
            CmsContentStatus::Approved => ContentStatus::Approved,
            CmsContentStatus::Scheduled => ContentStatus::Scheduled,
            CmsContentStatus::Published => ContentStatus::Published,
            CmsContentStatus::Archived => ContentStatus::Archived,
        }
    }
}

impl From<ContentStatus> for CmsContentStatus {
    fn from(status: ContentStatus) -> Self {
        match status {
            ContentStatus::Draft => CmsContentStatus::Draft,
            ContentStatus::InReview => CmsContentStatus::InReview,
            ContentStatus::Approved => CmsContentStatus::Approved,
            ContentStatus::Scheduled => CmsContentStatus::Scheduled,
            ContentStatus::Published => CmsContentStatus::Published,
            ContentStatus::Archived => CmsContentStatus::Archived,
        }
    }
}

impl ContentStatus {
    /// Check if this status allows editing
    pub fn is_editable(&self) -> bool {
        matches!(self, ContentStatus::Draft | ContentStatus::InReview)
    }

    /// Check if this status is publicly visible
    pub fn is_public(&self) -> bool {
        matches!(self, ContentStatus::Published)
    }

    /// Get valid transitions from this status
    pub fn valid_transitions(&self) -> Vec<ContentStatus> {
        match self {
            ContentStatus::Draft => vec![ContentStatus::InReview, ContentStatus::Archived],
            ContentStatus::InReview => vec![
                ContentStatus::Draft,
                ContentStatus::Approved,
                ContentStatus::Archived,
            ],
            ContentStatus::Approved => vec![
                ContentStatus::InReview,
                ContentStatus::Scheduled,
                ContentStatus::Published,
                ContentStatus::Archived,
            ],
            ContentStatus::Scheduled => vec![
                ContentStatus::Approved,
                ContentStatus::Published,
                ContentStatus::Archived,
            ],
            ContentStatus::Published => vec![ContentStatus::Archived],
            ContentStatus::Archived => vec![ContentStatus::Draft],
        }
    }

    /// Check if transition to target status is valid
    pub fn can_transition_to(&self, target: ContentStatus) -> bool {
        self.valid_transitions().contains(&target)
    }
}

/// Content type with schema validation
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ContentType {
    Page,
    BlogPost,
    AlertService,
    TradingRoom,
    Indicator,
    Course,
    Lesson,
    Testimonial,
    Faq,
    Author,
    TopicCluster,
    WeeklyWatchlist,
    Resource,
    NavigationMenu,
    SiteSettings,
    Redirect,
}

impl From<CmsContentType> for ContentType {
    fn from(ct: CmsContentType) -> Self {
        match ct {
            CmsContentType::Page => ContentType::Page,
            CmsContentType::BlogPost => ContentType::BlogPost,
            CmsContentType::AlertService => ContentType::AlertService,
            CmsContentType::TradingRoom => ContentType::TradingRoom,
            CmsContentType::Indicator => ContentType::Indicator,
            CmsContentType::Course => ContentType::Course,
            CmsContentType::Lesson => ContentType::Lesson,
            CmsContentType::Testimonial => ContentType::Testimonial,
            CmsContentType::Faq => ContentType::Faq,
            CmsContentType::Author => ContentType::Author,
            CmsContentType::TopicCluster => ContentType::TopicCluster,
            CmsContentType::WeeklyWatchlist => ContentType::WeeklyWatchlist,
            CmsContentType::Resource => ContentType::Resource,
            CmsContentType::NavigationMenu => ContentType::NavigationMenu,
            CmsContentType::SiteSettings => ContentType::SiteSettings,
            CmsContentType::Redirect => ContentType::Redirect,
        }
    }
}

impl From<ContentType> for CmsContentType {
    fn from(ct: ContentType) -> Self {
        match ct {
            ContentType::Page => CmsContentType::Page,
            ContentType::BlogPost => CmsContentType::BlogPost,
            ContentType::AlertService => CmsContentType::AlertService,
            ContentType::TradingRoom => CmsContentType::TradingRoom,
            ContentType::Indicator => CmsContentType::Indicator,
            ContentType::Course => CmsContentType::Course,
            ContentType::Lesson => CmsContentType::Lesson,
            ContentType::Testimonial => CmsContentType::Testimonial,
            ContentType::Faq => CmsContentType::Faq,
            ContentType::Author => CmsContentType::Author,
            ContentType::TopicCluster => CmsContentType::TopicCluster,
            ContentType::WeeklyWatchlist => CmsContentType::WeeklyWatchlist,
            ContentType::Resource => CmsContentType::Resource,
            ContentType::NavigationMenu => CmsContentType::NavigationMenu,
            ContentType::SiteSettings => CmsContentType::SiteSettings,
            ContentType::Redirect => CmsContentType::Redirect,
        }
    }
}

impl ContentType {
    /// Get default template for this content type
    pub fn default_template(&self) -> &'static str {
        match self {
            ContentType::Page => "default",
            ContentType::BlogPost => "article",
            ContentType::AlertService => "service",
            ContentType::TradingRoom => "service",
            ContentType::Indicator => "product",
            ContentType::Course => "course",
            ContentType::Lesson => "lesson",
            ContentType::Testimonial => "testimonial",
            ContentType::Faq => "faq",
            ContentType::Author => "author",
            ContentType::TopicCluster => "topic",
            ContentType::WeeklyWatchlist => "watchlist",
            ContentType::Resource => "resource",
            ContentType::NavigationMenu => "menu",
            ContentType::SiteSettings => "settings",
            ContentType::Redirect => "redirect",
        }
    }

    /// Check if this content type requires approval
    pub fn requires_approval(&self) -> bool {
        matches!(
            self,
            ContentType::Page
                | ContentType::BlogPost
                | ContentType::AlertService
                | ContentType::TradingRoom
                | ContentType::Indicator
                | ContentType::Course
        )
    }

    /// Get URL prefix for this content type
    pub fn url_prefix(&self) -> &'static str {
        match self {
            ContentType::Page => "",
            ContentType::BlogPost => "blog",
            ContentType::AlertService => "services/alerts",
            ContentType::TradingRoom => "services/rooms",
            ContentType::Indicator => "indicators",
            ContentType::Course => "courses",
            ContentType::Lesson => "courses",
            ContentType::Testimonial => "testimonials",
            ContentType::Faq => "faq",
            ContentType::Author => "authors",
            ContentType::TopicCluster => "topics",
            ContentType::WeeklyWatchlist => "watchlist",
            ContentType::Resource => "resources",
            ContentType::NavigationMenu => "",
            ContentType::SiteSettings => "",
            ContentType::Redirect => "",
        }
    }
}

/// Content entity - the aggregate root for content domain
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Content {
    pub id: Uuid,
    pub content_type: ContentType,
    pub slug: String,
    pub locale: String,
    pub is_primary_locale: bool,
    pub parent_content_id: Option<Uuid>,
    pub title: String,
    pub subtitle: Option<String>,
    pub excerpt: Option<String>,
    pub content: Option<String>,
    pub content_blocks: Option<JsonValue>,
    pub featured_image_id: Option<Uuid>,
    pub og_image_id: Option<Uuid>,
    pub gallery_ids: Vec<Uuid>,
    pub meta_title: Option<String>,
    pub meta_description: Option<String>,
    pub meta_keywords: Vec<String>,
    pub canonical_url: Option<String>,
    pub robots_directives: Option<String>,
    pub structured_data: Option<JsonValue>,
    pub author_id: Option<Uuid>,
    pub contributors: Vec<Uuid>,
    pub status: ContentStatus,
    pub published_at: Option<DateTime<Utc>>,
    pub scheduled_publish_at: Option<DateTime<Utc>>,
    pub scheduled_unpublish_at: Option<DateTime<Utc>>,
    pub primary_category_id: Option<Uuid>,
    pub categories: Vec<Uuid>,
    pub custom_fields: Option<JsonValue>,
    pub template: String,
    pub deleted_at: Option<DateTime<Utc>>,
    pub version: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub created_by: Option<Uuid>,
    pub updated_by: Option<Uuid>,
}

impl Content {
    /// Create a new content entity
    pub fn new(
        content_type: ContentType,
        slug: String,
        title: String,
        created_by: Option<Uuid>,
    ) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            content_type,
            slug,
            locale: "en".to_string(),
            is_primary_locale: true,
            parent_content_id: None,
            title,
            subtitle: None,
            excerpt: None,
            content: None,
            content_blocks: None,
            featured_image_id: None,
            og_image_id: None,
            gallery_ids: vec![],
            meta_title: None,
            meta_description: None,
            meta_keywords: vec![],
            canonical_url: None,
            robots_directives: None,
            structured_data: None,
            author_id: created_by,
            contributors: vec![],
            status: ContentStatus::Draft,
            published_at: None,
            scheduled_publish_at: None,
            scheduled_unpublish_at: None,
            primary_category_id: None,
            categories: vec![],
            custom_fields: None,
            template: content_type.default_template().to_string(),
            deleted_at: None,
            version: 1,
            created_at: now,
            updated_at: now,
            created_by,
            updated_by: created_by,
        }
    }

    /// Check if content can be edited
    pub fn can_edit(&self, user_role: &CmsUserRole) -> bool {
        if !self.status.is_editable() {
            return user_role.can_edit_all();
        }
        true
    }

    /// Validate content before save
    pub fn validate(&self) -> Result<(), ContentError> {
        // Title validation
        if self.title.trim().is_empty() {
            return Err(ContentError::ValidationFailed(
                "Title cannot be empty".to_string(),
            ));
        }
        if self.title.len() > 200 {
            return Err(ContentError::ValidationFailed(
                "Title must be less than 200 characters".to_string(),
            ));
        }

        // Slug validation
        if self.slug.trim().is_empty() {
            return Err(ContentError::ValidationFailed(
                "Slug cannot be empty".to_string(),
            ));
        }
        if !self
            .slug
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || c == '-')
        {
            return Err(ContentError::ValidationFailed(
                "Slug can only contain alphanumeric characters and hyphens".to_string(),
            ));
        }

        // SEO validation
        if let Some(ref meta_title) = self.meta_title {
            if meta_title.len() > 70 {
                return Err(ContentError::ValidationFailed(
                    "Meta title should be less than 70 characters".to_string(),
                ));
            }
        }
        if let Some(ref meta_desc) = self.meta_description {
            if meta_desc.len() > 160 {
                return Err(ContentError::ValidationFailed(
                    "Meta description should be less than 160 characters".to_string(),
                ));
            }
        }

        Ok(())
    }

    /// Transition content status with validation
    pub fn transition_status(
        &mut self,
        new_status: ContentStatus,
        user_role: &CmsUserRole,
    ) -> Result<ContentStatus, ContentError> {
        // Check if transition is valid
        if !self.status.can_transition_to(new_status) {
            return Err(ContentError::InvalidTransition {
                from: self.status,
                to: new_status,
            });
        }

        // Check permissions for specific transitions
        match new_status {
            ContentStatus::Approved | ContentStatus::Published => {
                if !user_role.can_approve() {
                    return Err(ContentError::PermissionDenied {
                        role: user_role.clone(),
                        action: format!("transition to {:?}", new_status),
                    });
                }
            }
            ContentStatus::Scheduled => {
                if self.scheduled_publish_at.is_none() {
                    return Err(ContentError::ValidationFailed(
                        "Scheduled publish date required for scheduling".to_string(),
                    ));
                }
            }
            _ => {}
        }

        let old_status = self.status;
        self.status = new_status;
        self.updated_at = Utc::now();
        self.version += 1;

        // Set published_at when publishing
        if new_status == ContentStatus::Published && self.published_at.is_none() {
            self.published_at = Some(Utc::now());
        }

        Ok(old_status)
    }

    /// Submit content for review
    pub fn submit_for_review(&mut self) -> Result<(), ContentError> {
        if self.status != ContentStatus::Draft {
            return Err(ContentError::InvalidStatusForAction {
                status: self.status,
                action: "submit for review".to_string(),
            });
        }

        self.validate()?;
        self.status = ContentStatus::InReview;
        self.updated_at = Utc::now();
        self.version += 1;
        Ok(())
    }

    /// Approve content (requires approval permission)
    pub fn approve(&mut self, user_role: &CmsUserRole) -> Result<(), ContentError> {
        if !user_role.can_approve() {
            return Err(ContentError::PermissionDenied {
                role: user_role.clone(),
                action: "approve content".to_string(),
            });
        }

        if self.status != ContentStatus::InReview {
            return Err(ContentError::InvalidStatusForAction {
                status: self.status,
                action: "approve".to_string(),
            });
        }

        self.status = ContentStatus::Approved;
        self.updated_at = Utc::now();
        self.version += 1;
        Ok(())
    }

    /// Publish content immediately
    pub fn publish(&mut self, user_role: &CmsUserRole) -> Result<(), ContentError> {
        if !user_role.can_publish() {
            return Err(ContentError::PermissionDenied {
                role: user_role.clone(),
                action: "publish content".to_string(),
            });
        }

        if !matches!(
            self.status,
            ContentStatus::Approved | ContentStatus::Scheduled
        ) {
            return Err(ContentError::InvalidStatusForAction {
                status: self.status,
                action: "publish".to_string(),
            });
        }

        self.status = ContentStatus::Published;
        self.published_at = Some(Utc::now());
        self.updated_at = Utc::now();
        self.version += 1;
        Ok(())
    }

    /// Schedule content for future publication
    pub fn schedule(
        &mut self,
        publish_at: DateTime<Utc>,
        user_role: &CmsUserRole,
    ) -> Result<(), ContentError> {
        if !user_role.can_publish() {
            return Err(ContentError::PermissionDenied {
                role: user_role.clone(),
                action: "schedule content".to_string(),
            });
        }

        if publish_at <= Utc::now() {
            return Err(ContentError::ValidationFailed(
                "Scheduled publish date must be in the future".to_string(),
            ));
        }

        if self.status != ContentStatus::Approved {
            return Err(ContentError::InvalidStatusForAction {
                status: self.status,
                action: "schedule".to_string(),
            });
        }

        self.scheduled_publish_at = Some(publish_at);
        self.status = ContentStatus::Scheduled;
        self.updated_at = Utc::now();
        self.version += 1;
        Ok(())
    }

    /// Archive content
    pub fn archive(&mut self) -> Result<(), ContentError> {
        if self.status == ContentStatus::Archived {
            return Err(ContentError::InvalidStatusForAction {
                status: self.status,
                action: "archive".to_string(),
            });
        }

        self.status = ContentStatus::Archived;
        self.updated_at = Utc::now();
        self.version += 1;
        Ok(())
    }

    /// Restore archived content to draft
    pub fn restore(&mut self) -> Result<(), ContentError> {
        if self.status != ContentStatus::Archived {
            return Err(ContentError::InvalidStatusForAction {
                status: self.status,
                action: "restore".to_string(),
            });
        }

        self.status = ContentStatus::Draft;
        self.updated_at = Utc::now();
        self.version += 1;
        Ok(())
    }

    /// Soft delete content
    pub fn soft_delete(&mut self, user_role: &CmsUserRole) -> Result<(), ContentError> {
        if !user_role.can_delete() {
            return Err(ContentError::PermissionDenied {
                role: user_role.clone(),
                action: "delete content".to_string(),
            });
        }

        self.deleted_at = Some(Utc::now());
        self.updated_at = Utc::now();
        self.version += 1;
        Ok(())
    }

    /// Generate full URL for this content
    pub fn full_url(&self, base_url: &str) -> String {
        let prefix = self.content_type.url_prefix();
        if prefix.is_empty() {
            format!("{}/{}", base_url.trim_end_matches('/'), self.slug)
        } else {
            format!(
                "{}/{}/{}",
                base_url.trim_end_matches('/'),
                prefix,
                self.slug
            )
        }
    }

    /// Check version for optimistic locking
    pub fn check_version(&self, expected: i32) -> Result<(), ContentError> {
        if self.version != expected {
            return Err(ContentError::VersionConflict {
                expected,
                actual: self.version,
            });
        }
        Ok(())
    }
}

/// Content builder for fluent construction
pub struct ContentBuilder {
    content: Content,
}

impl ContentBuilder {
    pub fn new(content_type: ContentType, slug: String, title: String) -> Self {
        Self {
            content: Content::new(content_type, slug, title, None),
        }
    }

    pub fn created_by(mut self, user_id: Uuid) -> Self {
        self.content.created_by = Some(user_id);
        self.content.author_id = Some(user_id);
        self
    }

    pub fn subtitle(mut self, subtitle: String) -> Self {
        self.content.subtitle = Some(subtitle);
        self
    }

    pub fn excerpt(mut self, excerpt: String) -> Self {
        self.content.excerpt = Some(excerpt);
        self
    }

    pub fn content_body(mut self, content: String) -> Self {
        self.content.content = Some(content);
        self
    }

    pub fn content_blocks(mut self, blocks: JsonValue) -> Self {
        self.content.content_blocks = Some(blocks);
        self
    }

    pub fn featured_image(mut self, image_id: Uuid) -> Self {
        self.content.featured_image_id = Some(image_id);
        self
    }

    pub fn meta_title(mut self, title: String) -> Self {
        self.content.meta_title = Some(title);
        self
    }

    pub fn meta_description(mut self, description: String) -> Self {
        self.content.meta_description = Some(description);
        self
    }

    pub fn meta_keywords(mut self, keywords: Vec<String>) -> Self {
        self.content.meta_keywords = keywords;
        self
    }

    pub fn template(mut self, template: String) -> Self {
        self.content.template = template;
        self
    }

    pub fn custom_fields(mut self, fields: JsonValue) -> Self {
        self.content.custom_fields = Some(fields);
        self
    }

    pub fn locale(mut self, locale: String) -> Self {
        self.content.locale = locale;
        self
    }

    pub fn build(self) -> Result<Content, ContentError> {
        self.content.validate()?;
        Ok(self.content)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_status_transitions() {
        assert!(ContentStatus::Draft.can_transition_to(ContentStatus::InReview));
        assert!(ContentStatus::Draft.can_transition_to(ContentStatus::Archived));
        assert!(!ContentStatus::Draft.can_transition_to(ContentStatus::Published));

        assert!(ContentStatus::InReview.can_transition_to(ContentStatus::Approved));
        assert!(ContentStatus::Approved.can_transition_to(ContentStatus::Published));
        assert!(ContentStatus::Published.can_transition_to(ContentStatus::Archived));
        assert!(ContentStatus::Archived.can_transition_to(ContentStatus::Draft));
    }

    #[test]
    fn test_content_validation() {
        let content = Content::new(
            ContentType::BlogPost,
            "test-post".to_string(),
            "Test Post".to_string(),
            None,
        );
        assert!(content.validate().is_ok());

        let mut bad_content = Content::new(
            ContentType::BlogPost,
            "".to_string(),
            "Test".to_string(),
            None,
        );
        assert!(bad_content.validate().is_err());

        bad_content.slug = "test post".to_string(); // Invalid: contains space
        assert!(bad_content.validate().is_err());
    }

    #[test]
    fn test_submit_for_review() {
        let mut content = Content::new(
            ContentType::BlogPost,
            "test".to_string(),
            "Test".to_string(),
            None,
        );
        assert!(content.submit_for_review().is_ok());
        assert_eq!(content.status, ContentStatus::InReview);
    }

    #[test]
    fn test_approval_requires_permission() {
        let mut content = Content::new(
            ContentType::BlogPost,
            "test".to_string(),
            "Test".to_string(),
            None,
        );
        content.status = ContentStatus::InReview;

        // Editor cannot approve
        assert!(content.approve(&CmsUserRole::ContentEditor).is_err());

        // Manager can approve
        assert!(content.approve(&CmsUserRole::MarketingManager).is_ok());
    }

    #[test]
    fn test_version_increment() {
        let mut content = Content::new(
            ContentType::BlogPost,
            "test".to_string(),
            "Test".to_string(),
            None,
        );
        let initial_version = content.version;

        content.submit_for_review().unwrap();
        assert_eq!(content.version, initial_version + 1);
    }
}
