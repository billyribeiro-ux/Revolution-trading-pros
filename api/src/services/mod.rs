//! External services
//! Apple ICT 7+ Principal Engineer - January 2026
//! ICT 7+: Redis required in production for security (rate limiting, session management)

pub mod analytics; // ICT 7+ Phase 4: Analytics Dashboard Service
pub mod bunny;
pub mod cms_audit;
pub mod cms_content;
pub mod cms_preview;
pub mod cms_scheduler;
pub mod cms_upload;
pub mod cms_webhooks;
pub mod cms_workflow;
pub mod credential_resolver; // PE7 invariant 2A: DB-backed credential lookup with env fallback
pub mod email;
pub mod event_broadcaster; // ICT 7+ Phase 3: Unified WebSocket + SSE event broadcasting
pub mod export; // ICT 7+ Phase 4: Export Functionality
pub mod mfa; // ICT 7: TOTP/2FA Multi-Factor Authentication
pub mod order_service;
pub mod rate_limit; // ICT 7: Multi-tier rate limiting with fallback
pub mod redis;
pub mod room_analytics; // ICT 11+ Phase 5: Room Performance Analytics Service
pub mod room_search; // ICT 7+ Phase 4: Full-Text Search for Room Content
pub mod search;
pub mod storage;
pub mod stripe;
pub mod subscription_service;

use crate::cache::{CacheInvalidator, CacheService};
use crate::config::Config;
use anyhow::{anyhow, Result};

/// Container for all external services
#[derive(Clone)]
pub struct Services {
    pub redis: Option<redis::RedisService>,
    pub storage: storage::StorageService,
    pub stripe: stripe::StripeService,
    pub search: search::SearchService,
    pub email: Option<email::EmailService>,
    /// Cache service for Explosive Swings content caching
    pub cache: CacheService,
    /// Cache invalidation helper
    pub cache_invalidator: CacheInvalidator,
    /// PE7 invariant 2A: DB-first / env-fallback credential resolver. Used
    /// by every Stripe-touching endpoint so admin paste in /admin/settings
    /// takes effect without a redeploy.
    pub credentials: credential_resolver::CredentialResolver,
}

impl Services {
    pub async fn new(config: &Config) -> Result<Self> {
        let is_production = config.environment == "production";

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

        // ICT 7+: Initialize Redis with production requirement
        // In production, Redis is REQUIRED for:
        // - Rate limiting (login, API)
        // - Session management
        // - User cache (auth performance)
        // - Account lockout protection
        let redis = tokio::time::timeout(
            std::time::Duration::from_secs(5), // Increased timeout for production reliability
            redis::RedisService::new(&config.redis_url),
        )
        .await;

        let redis = match redis {
            Ok(Ok(r)) => {
                tracing::info!("✅ Redis service initialized successfully");
                Some(r)
            }
            Ok(Err(e)) => {
                if is_production {
                    tracing::error!(
                        target: "security",
                        event = "redis_required_failed",
                        error = %e,
                        "ICT 7+ SECURITY: Redis connection FAILED in production - this is CRITICAL"
                    );
                    return Err(anyhow!(
                        "Redis is required in production for security features (rate limiting, session management). Error: {}",
                        e
                    ));
                }
                tracing::warn!("Redis connection failed (dev mode - continuing): {}", e);
                None
            }
            Err(_) => {
                if is_production {
                    tracing::error!(
                        target: "security",
                        event = "redis_timeout_production",
                        "ICT 7+ SECURITY: Redis connection TIMEOUT in production - this is CRITICAL"
                    );
                    return Err(anyhow!(
                        "Redis connection timeout in production. Redis is required for security features."
                    ));
                }
                tracing::warn!("Redis connection timeout - continuing without Redis (dev mode)");
                None
            }
        };

        // ICT 7+: Log security warning if Redis is unavailable in non-production
        if redis.is_none() && !is_production {
            tracing::warn!(
                target: "security",
                "⚠️  Running without Redis - rate limiting, session management, and user caching DISABLED"
            );
        }

        // Initialize cache service with Redis (graceful degradation if unavailable)
        let cache = CacheService::new(redis.clone(), None);
        let cache_invalidator = CacheInvalidator::new(cache.clone());

        tracing::info!(
            target: "cache",
            has_redis = %cache.has_redis(),
            "Cache service initialized"
        );

        // Build the static stripe service from env (legacy; kept for code paths
        // not yet migrated to the resolver). New code SHOULD prefer
        // `services.credentials.stripe_client(pool, env).await` so DB-stored
        // keys win over env at request time.
        let stripe = stripe::StripeService::new(&config.stripe_secret_key);
        let stripe = if config.stripe_webhook_secret.is_empty() {
            stripe
        } else {
            stripe.with_webhook_secret(&config.stripe_webhook_secret)
        };

        let credentials = credential_resolver::CredentialResolver::new(config.clone());

        Ok(Self {
            redis,
            storage: storage::StorageService::new(config).await?,
            stripe,
            search,
            email,
            cache,
            cache_invalidator,
            credentials,
        })
    }
}
