//! External services

pub mod redis;
pub mod storage;
pub mod email;
pub mod stripe;
pub mod search;

use anyhow::Result;
use crate::config::Config;

/// Container for all external services
#[derive(Clone)]
pub struct Services {
    pub redis: redis::RedisService,
    pub storage: storage::StorageService,
    pub stripe: stripe::StripeService,
    pub search: search::SearchService,
}

impl Services {
    pub async fn new(config: &Config) -> Result<Self> {
        let search = search::SearchService::new(&config.meilisearch_host, &config.meilisearch_api_key);

        // Initialize search indexes (non-blocking, runs in background)
        let search_clone = search.clone();
        tokio::spawn(async move {
            if let Err(e) = search_clone.setup_indexes().await {
                tracing::warn!("Failed to setup search indexes: {}", e);
            }
        });

        Ok(Self {
            redis: redis::RedisService::new(&config.redis_url).await?,
            storage: storage::StorageService::new(config).await?,
            stripe: stripe::StripeService::new(&config.stripe_secret_key),
            search,
        })
    }
}
