//! External services

pub mod redis;
pub mod storage;
pub mod email;
pub mod stripe;

use anyhow::Result;
use crate::config::Config;

/// Container for all external services
#[derive(Clone)]
pub struct Services {
    pub redis: redis::RedisService,
    pub storage: storage::StorageService,
    pub stripe: stripe::StripeService,
}

impl Services {
    pub async fn new(config: &Config) -> Result<Self> {
        Ok(Self {
            redis: redis::RedisService::new(&config.redis_url).await?,
            storage: storage::StorageService::new(config).await?,
            stripe: stripe::StripeService::new(&config.stripe_secret_key),
        })
    }
}
