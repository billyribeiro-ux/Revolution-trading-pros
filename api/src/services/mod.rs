//! External services
//! Apple ICT 11+ Principal Engineer - January 2026

pub mod bunny;
pub mod cms;
pub mod email;
pub mod order_service;
pub mod redis;
pub mod search;
pub mod storage;
pub mod stripe;
pub mod subscription_service;

use crate::config::Config;
use anyhow::Result;

/// Container for all external services
#[derive(Clone)]
pub struct Services {
    pub redis: Option<redis::RedisService>,
    pub storage: storage::StorageService,
    pub stripe: stripe::StripeService,
    pub search: search::SearchService,
    pub email: Option<email::EmailService>,
}

impl Services {
    pub async fn new(config: &Config) -> Result<Self> {
        let search =
            search::SearchService::new(&config.meilisearch_host, &config.meilisearch_api_key)?;

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

        // Initialize Redis (optional - timeout after 2 seconds)
        let redis = tokio::time::timeout(
            std::time::Duration::from_secs(2),
            redis::RedisService::new(&config.redis_url),
        )
        .await;

        let redis = match redis {
            Ok(Ok(r)) => {
                tracing::info!("Redis service initialized");
                Some(r)
            }
            Ok(Err(e)) => {
                tracing::warn!("Redis connection failed: {}", e);
                None
            }
            Err(_) => {
                tracing::warn!("Redis connection timeout - continuing without Redis");
                None
            }
        };

        Ok(Self {
            redis,
            storage: storage::StorageService::new(config).await?,
            stripe: stripe::StripeService::new(&config.stripe_secret_key),
            search,
            email,
        })
    }
}
