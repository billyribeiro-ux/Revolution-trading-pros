//! External services
//! Apple ICT 11+ Principal Engineer - January 2026

pub mod redis;
pub mod storage;
pub mod email;
pub mod stripe;
pub mod search;
pub mod order_service;
pub mod subscription_service;
pub mod cms;
pub mod bunny;

use anyhow::Result;
use crate::config::Config;

/// Container for all external services
#[derive(Clone)]
pub struct Services {
    pub redis: redis::RedisService,
    pub storage: storage::StorageService,
    pub stripe: stripe::StripeService,
    pub search: search::SearchService,
    pub email: Option<email::EmailService>,
}

impl Services {
    pub async fn new(config: &Config) -> Result<Self> {
        let search = search::SearchService::new(&config.meilisearch_host, &config.meilisearch_api_key)?;

        // Initialize search indexes (non-blocking, runs in background)
        let search_clone = search.clone();
        tokio::spawn(async move {
            if let Err(e) = search_clone.setup_indexes().await {
                tracing::warn!("Failed to setup search indexes: {}", e);
            }
        });

        // Initialize email service (optional - requires POSTMARK_TOKEN)
        let email = config.postmark_token.as_ref().map(|token| {
            tracing::info!("Email service initialized with Postmark");
            email::EmailService::new(token, &config.from_email, &config.app_url)
        });

        if email.is_none() {
            tracing::warn!("Email service not initialized (POSTMARK_TOKEN not set)");
        }

        Ok(Self {
            redis: redis::RedisService::new(&config.redis_url).await?,
            storage: storage::StorageService::new(config).await?,
            stripe: stripe::StripeService::new(&config.stripe_secret_key),
            search,
            email,
        })
    }
}
