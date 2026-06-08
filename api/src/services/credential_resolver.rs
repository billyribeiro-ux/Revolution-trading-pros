//! Credential Resolver - PE7 invariant 2A
//! ═══════════════════════════════════════════════════════════════════════════
//! Resolves third-party service credentials (Stripe today; SendGrid, Bunny.net,
//! etc. later) at REQUEST time, not at startup.
//!
//! Lookup order:
//!   1. `service_connections` table where service_key='stripe' AND
//!      environment matches the current scope (or any-env if unset).
//!   2. Env-var fallback (config::Config) — keeps dev workflows working when
//!      the admin has not pasted keys into /admin/settings yet.
//!
//! Cache:
//!   - 60-second TTL keyed by (service, environment).
//!   - `tokio::sync::RwLock<HashMap<...>>` (no extra deps; project already has
//!     tokio "full"). Reads are common, writes rare.
//!   - `invalidate(service, environment)` is called by connections.rs's
//!     connect/disconnect/test handlers so admins don't wait 60s for a paste
//!     to take effect.
//!
//! Discipline:
//!   - Never log secret_key / webhook_secret values. Boundary log only the
//!     SOURCE ("db"|"env") so we can debug "is admin's paste being read?".
//!   - On DB error, emit a warning and fall back to env. We do NOT fail the
//!     request just because the connections table is unreachable — that
//!     would brick payments during a partial DB outage.

use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, Instant};

use base64::{engine::general_purpose::STANDARD, Engine as _};
use sqlx::PgPool;
use tokio::sync::RwLock;

use crate::config::Config;
use crate::services::stripe::StripeService;

/// Cache TTL for resolved credentials. Long enough that we don't hammer the
/// DB on every payment endpoint; short enough that a forgotten invalidate()
/// still self-corrects within a minute.
const CACHE_TTL: Duration = Duration::from_secs(60);

/// Resolved credential set for a single service/environment pair.
#[derive(Clone, Debug)]
pub struct ResolvedCreds {
    /// Service key (e.g. "stripe").
    pub service: String,
    /// Environment scope ("production" | "sandbox" | "" for unscoped).
    pub environment: String,
    /// Decrypted credential map (e.g. {"secret_key": "sk_...", "publishable_key": "..."}).
    pub credentials: HashMap<String, String>,
    /// Optional webhook signing secret stored alongside the row.
    pub webhook_secret: Option<String>,
    /// Where these creds came from. "db" or "env".
    pub source: &'static str,
}

#[derive(Clone)]
struct CacheEntry {
    creds: ResolvedCreds,
    fetched_at: Instant,
}

/// Resolver for third-party service credentials.
///
/// Cheap to clone (Arc-wrapped). Stash one on AppState.
#[derive(Clone)]
pub struct CredentialResolver {
    config: Arc<Config>,
    cache: Arc<RwLock<HashMap<String, CacheEntry>>>,
}

impl CredentialResolver {
    pub fn new(config: Config) -> Self {
        Self {
            config: Arc::new(config),
            cache: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Resolve credentials for `(service, environment)`. Cache hit returns
    /// fast; cache miss queries the DB then falls back to env.
    pub async fn resolve(&self, pool: &PgPool, service: &str, environment: &str) -> ResolvedCreds {
        let key = cache_key(service, environment);

        // Fast path: cached and fresh.
        if let Some(hit) = self.cache_get(&key).await {
            return hit;
        }

        // Slow path: query DB, fall back to env.
        let resolved = self.fetch_fresh(pool, service, environment).await;
        self.cache_put(key, resolved.clone()).await;
        resolved
    }

    /// Build a `StripeService` ready for outbound calls. Convenience wrapper
    /// around `resolve()` for the most common caller. Falls back to env-vars
    /// if no DB row exists for the requested environment.
    pub async fn stripe_client(&self, pool: &PgPool, environment: &str) -> StripeService {
        let creds = self.resolve(pool, "stripe", environment).await;
        let secret = creds
            .credentials
            .get("secret_key")
            .cloned()
            .unwrap_or_default();
        let svc = StripeService::new(&secret);
        match creds.webhook_secret.as_deref() {
            Some(s) if !s.is_empty() => svc.with_webhook_secret(s),
            _ => svc,
        }
    }

    /// Drop any cached entry for `(service, environment)` so the next call
    /// reads fresh from the DB. Called by connections.rs after a save/test.
    pub async fn invalidate(&self, service: &str, environment: &str) {
        let key = cache_key(service, environment);
        self.cache.write().await.remove(&key);
        // Also nuke the unscoped variant — admins commonly paste keys without
        // selecting an environment, and we want both lookups to repoll.
        if !environment.is_empty() {
            self.cache.write().await.remove(&cache_key(service, ""));
        }
    }

    /// Clear the entire cache. Used by tests and after env-var hot-swaps.
    pub async fn clear(&self) {
        self.cache.write().await.clear();
    }

    // ── internals ──────────────────────────────────────────────────────────

    async fn cache_get(&self, key: &str) -> Option<ResolvedCreds> {
        let guard = self.cache.read().await;
        let entry = guard.get(key)?;
        if entry.fetched_at.elapsed() > CACHE_TTL {
            return None;
        }
        Some(entry.creds.clone())
    }

    async fn cache_put(&self, key: String, creds: ResolvedCreds) {
        let mut guard = self.cache.write().await;
        guard.insert(
            key,
            CacheEntry {
                creds,
                fetched_at: Instant::now(),
            },
        );
    }

    async fn fetch_fresh(&self, pool: &PgPool, service: &str, environment: &str) -> ResolvedCreds {
        // Try the DB first. We accept either an exact env match OR a row
        // with NULL environment — that's the "set-and-forget" default for
        // admins who don't toggle test/live.
        let row: Result<Option<DbCreds>, sqlx::Error> = sqlx::query_as::<_, DbCreds>(
            r"
            SELECT credentials_encrypted, webhook_secret, environment
            FROM service_connections
            WHERE service_key = $1
              AND status = 'connected'
              AND ($2::text = '' OR environment = $2 OR environment IS NULL)
            ORDER BY (environment = $2) DESC NULLS LAST,
                     last_verified_at DESC NULLS LAST
            LIMIT 1
            ",
        )
        .bind(service)
        .bind(environment)
        .fetch_optional(pool)
        .await;

        match row {
            Ok(Some(db)) => {
                let credentials = db
                    .credentials_encrypted
                    .as_deref()
                    .map(|e| {
                        crate::utils::crypto::decrypt_map(
                            &self.config.credentials_encryption_key,
                            e,
                        )
                    })
                    .unwrap_or_default();

                tracing::debug!(
                    target: "credentials",
                    service = %service,
                    environment = %environment,
                    source = "db",
                    "resolved credentials from database"
                );

                return ResolvedCreds {
                    service: service.to_string(),
                    environment: environment.to_string(),
                    credentials,
                    webhook_secret: db.webhook_secret,
                    source: "db",
                };
            }
            Ok(None) => {
                tracing::debug!(
                    target: "credentials",
                    service = %service,
                    environment = %environment,
                    source = "env",
                    "no DB row; falling back to env"
                );
            }
            Err(e) => {
                tracing::warn!(
                    target: "credentials",
                    service = %service,
                    environment = %environment,
                    error = %e,
                    "DB lookup failed; falling back to env"
                );
            }
        }

        self.env_fallback(service, environment)
    }

    fn env_fallback(&self, service: &str, environment: &str) -> ResolvedCreds {
        let mut credentials = HashMap::new();
        let mut webhook_secret = None;

        if service == "stripe" {
            if !self.config.stripe_secret_key.is_empty() {
                credentials.insert(
                    "secret_key".to_string(),
                    self.config.stripe_secret_key.clone(),
                );
            }
            if !self.config.stripe_publishable_key.is_empty() {
                credentials.insert(
                    "publishable_key".to_string(),
                    self.config.stripe_publishable_key.clone(),
                );
            }
            if !self.config.stripe_webhook_secret.is_empty() {
                webhook_secret = Some(self.config.stripe_webhook_secret.clone());
            }
        }

        ResolvedCreds {
            service: service.to_string(),
            environment: environment.to_string(),
            credentials,
            webhook_secret,
            source: "env",
        }
    }
}

fn cache_key(service: &str, environment: &str) -> String {
    format!("{service}|{environment}")
}

#[derive(sqlx::FromRow)]
struct DbCreds {
    credentials_encrypted: Option<String>,
    webhook_secret: Option<String>,
    #[allow(dead_code)]
    environment: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    fn dummy_config(secret: &str) -> Config {
        Config {
            port: 8080,
            environment: "test".into(),
            database_url: String::new(),
            redis_url: String::new(),
            r2_endpoint: String::new(),
            r2_access_key_id: String::new(),
            r2_secret_access_key: String::new(),
            r2_bucket: String::new(),
            r2_public_url: String::new(),
            jwt_secret: "test".into(),
            jwt_expires_in: 24,
            stripe_secret_key: secret.into(),
            stripe_publishable_key: "pk_test_123".into(),
            stripe_webhook_secret: "whsec_test".into(),
            cors_origins: vec![],
            // P1-3 (FULL_REPO_AUDIT_2026-05-17): new required Config field;
            // test fixture trusts no proxy (mechanical struct-literal update).
            trusted_proxy_cidrs: vec![],
            postmark_token: None,
            from_email: String::new(),
            app_url: String::new(),
            admin_notification_email: None,
            meilisearch_host: String::new(),
            meilisearch_api_key: String::new(),
            superadmin_emails: vec![],
            developer_emails: vec![],
            developer_mode: false,
            developer_bootstrap_email: None,
            developer_bootstrap_password_hash: None,
            developer_bootstrap_name: None,
            google_client_id: None,
            google_client_secret: None,
            apple_client_id: None,
            apple_team_id: None,
            apple_key_id: None,
            apple_private_key: None,
            member_indicator_secret: String::new(),
            member_license_secret: String::new(),
            credentials_encryption_key: "test-credentials-encryption-key".into(),
        }
    }

    #[test]
    fn env_fallback_returns_stripe_keys() {
        let resolver = CredentialResolver::new(dummy_config("sk_test_xyz"));
        let creds = resolver.env_fallback("stripe", "");
        assert_eq!(creds.source, "env");
        assert_eq!(
            creds.credentials.get("secret_key").map(String::as_str),
            Some("sk_test_xyz")
        );
        assert_eq!(
            creds.credentials.get("publishable_key").map(String::as_str),
            Some("pk_test_123")
        );
        assert_eq!(creds.webhook_secret.as_deref(), Some("whsec_test"));
    }

    #[test]
    fn env_fallback_omits_empty_secret() {
        let resolver = CredentialResolver::new(dummy_config(""));
        let creds = resolver.env_fallback("stripe", "");
        assert!(!creds.credentials.contains_key("secret_key"));
    }

    #[test]
    fn cache_key_disambiguates_envs() {
        assert_ne!(cache_key("stripe", "production"), cache_key("stripe", ""));
        assert_ne!(
            cache_key("stripe", "production"),
            cache_key("stripe", "sandbox")
        );
    }

    #[test]
    fn decrypt_round_trip() {
        let mut map = HashMap::new();
        map.insert("secret_key".to_string(), "sk_test_round".to_string());
        let json = serde_json::to_string(&map).unwrap();
        let encoded = STANDARD.encode(json.as_bytes());

        // Legacy base64 rows must still decrypt (the key is ignored on this path).
        let decoded = crate::utils::crypto::decrypt_map("any-key", &encoded);
        assert_eq!(
            decoded.get("secret_key").map(String::as_str),
            Some("sk_test_round")
        );
    }

    #[tokio::test]
    async fn invalidate_clears_entry() {
        let resolver = CredentialResolver::new(dummy_config("sk_test_inv"));
        let key = cache_key("stripe", "production");
        resolver
            .cache_put(
                key.clone(),
                ResolvedCreds {
                    service: "stripe".into(),
                    environment: "production".into(),
                    credentials: HashMap::new(),
                    webhook_secret: None,
                    source: "db",
                },
            )
            .await;
        assert!(resolver.cache_get(&key).await.is_some());
        resolver.invalidate("stripe", "production").await;
        assert!(resolver.cache_get(&key).await.is_none());
    }
}
