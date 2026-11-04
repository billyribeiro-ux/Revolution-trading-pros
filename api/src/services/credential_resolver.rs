//! Credential Resolver - PE7 invariant 2A
//! ═══════════════════════════════════════════════════════════════════════════
//! Resolves third-party service credentials (Stripe today; SendGrid, Bunny.net,
//! etc. later) at REQUEST time, not at startup.
//!
//! A named production/sandbox row is authoritative, including revocation.
//! Legacy unscoped or unverified rows require explicit provider verification.
//! Environment keys are used only when no applicable row exists and their mode
//! matches the requested provider scope. Database failures never reactivate keys.
//! Cache invalidation advances a generation so an in-flight old lookup cannot
//! repopulate credentials after rotation/revocation. Secret values are never logged.

use std::collections::HashMap;
use std::sync::{
    Arc,
    atomic::{AtomicU64, Ordering},
};
use std::time::{Duration, Instant};

use base64::{Engine as _, engine::general_purpose::STANDARD};
use sqlx::PgPool;
use tokio::sync::RwLock;

use crate::config::Config;
use crate::services::stripe::StripeService;

/// Cache TTL for resolved credentials. Long enough that we don't hammer the
/// DB on every payment endpoint; short enough that a forgotten invalidate()
/// still self-corrects within a minute.
const CACHE_TTL: Duration = Duration::from_secs(60);

/// Provider scope used by request-time application clients.
pub fn runtime_environment(config: &Config) -> &'static str {
    if config.environment == "production" {
        "production"
    } else {
        "sandbox"
    }
}

/// Resolved credential set for a single service/environment pair.
#[derive(Clone)]
pub struct ResolvedCreds {
    /// Service key (e.g. "stripe").
    pub service: String,
    /// Environment scope ("production" | "sandbox" | "" for unscoped).
    pub environment: String,
    /// Decrypted credential map (e.g. {"secret_key": "sk_...", "publishable_key": "..."}).
    pub credentials: HashMap<String, String>,
    /// Optional webhook signing secret stored alongside the row.
    pub webhook_secret: Option<String>,
    /// Origin: db, env, blocked configuration, or unavailable storage.
    pub source: &'static str,
}

impl std::fmt::Debug for ResolvedCreds {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("ResolvedCreds")
            .field("service", &self.service)
            .field("environment", &self.environment)
            .field("credential_count", &self.credentials.len())
            .field("webhook_secret_set", &self.webhook_secret.is_some())
            .field("source", &self.source)
            .finish()
    }
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
    generation: Arc<AtomicU64>,
}

impl CredentialResolver {
    pub fn new(config: Config) -> Self {
        Self {
            config: Arc::new(config),
            cache: Arc::new(RwLock::new(HashMap::new())),
            generation: Arc::new(AtomicU64::new(0)),
        }
    }

    /// Resolve credentials for `(service, environment)`. Cache hit returns
    /// fast; cache miss queries the DB then falls back to env.
    pub async fn resolve(&self, pool: &PgPool, service: &str, environment: &str) -> ResolvedCreds {
        let Some(environment) = self.scope(environment) else {
            return self.empty(service, environment, "blocked");
        };
        let key = cache_key(service, environment);
        for _ in 0..3 {
            let generation = self.generation.load(Ordering::Acquire);
            if let Some(hit) = self.cache_get(&key).await {
                return hit;
            }
            let resolved = self.fetch_fresh(pool, service, environment).await;
            if resolved.source == "unavailable" {
                return resolved;
            }
            if self
                .cache_put_current(key.clone(), resolved.clone(), generation)
                .await
            {
                return resolved;
            }
        }
        self.empty(service, environment, "unavailable")
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

    /// Invalidate all aliases/scopes for this service before another read returns.
    pub async fn invalidate(&self, service: &str, _environment: &str) {
        self.generation.fetch_add(1, Ordering::AcqRel);
        let prefix = format!("{service}|");
        self.cache
            .write()
            .await
            .retain(|key, _| !key.starts_with(&prefix));
    }
    pub async fn clear(&self) {
        self.generation.fetch_add(1, Ordering::AcqRel);
        self.cache.write().await.clear();
    }
    fn scope<'a>(&'a self, environment: &'a str) -> Option<&'a str> {
        match environment {
            "production" => Some("production"),
            "sandbox" | "development" | "staging" | "test" => Some("sandbox"),
            "" => Some(runtime_environment(&self.config)),
            _ => None,
        }
    }
    fn empty(&self, service: &str, environment: &str, source: &'static str) -> ResolvedCreds {
        ResolvedCreds {
            service: service.into(),
            environment: environment.into(),
            credentials: HashMap::new(),
            webhook_secret: None,
            source,
        }
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

    async fn cache_put_current(&self, key: String, creds: ResolvedCreds, generation: u64) -> bool {
        let mut guard = self.cache.write().await;
        if self.generation.load(Ordering::Acquire) != generation {
            return false;
        }
        guard.insert(
            key,
            CacheEntry {
                creds,
                fetched_at: Instant::now(),
            },
        );
        true
    }
    #[cfg(test)]
    async fn cache_put(&self, key: String, creds: ResolvedCreds) {
        self.cache_put_current(key, creds, self.generation.load(Ordering::Acquire))
            .await;
    }

    async fn fetch_fresh(&self, pool: &PgPool, service: &str, environment: &str) -> ResolvedCreds {
        let row=sqlx::query_as::<_,DbCreds>("SELECT c.id,c.credentials_encrypted,c.webhook_secret,c.environment,c.status,c.health_status,EXISTS(SELECT 1 FROM service_connection_revocations r WHERE r.service_key=$1 AND r.environment=$2) AS revoked FROM (SELECT 1) seed LEFT JOIN LATERAL (SELECT id,credentials_encrypted,webhook_secret,environment,status,health_status FROM service_connections WHERE service_key=$1 AND (environment=$2 OR environment IS NULL) ORDER BY (environment=$2) DESC NULLS LAST,last_verified_at DESC NULLS LAST,id DESC LIMIT 1) c ON TRUE")
            .bind(service).bind(environment).fetch_optional(pool).await;
        match row {
            Ok(Some(db)) => {
                if db.revoked {
                    return self.empty(service, environment, "blocked");
                }
                if db.id.is_none() {
                    return self.env_fallback(service, environment);
                }
                if db.environment.as_deref() != Some(environment)
                    || db.status.as_deref() != Some("connected")
                    || db.health_status.as_deref() != Some("verified")
                {
                    return self.empty(service, environment, "blocked");
                }
                let credentials = db
                    .credentials_encrypted
                    .as_deref()
                    .map(|encrypted| {
                        crate::utils::crypto::decrypt_map(
                            &self.config.credentials_encryption_key,
                            encrypted,
                        )
                    })
                    .unwrap_or_default();
                if credentials.is_empty()
                    || (service == "stripe"
                        && credentials
                            .get("secret_key")
                            .is_none_or(|value| value.trim().is_empty()))
                {
                    return self.empty(service, environment, "blocked");
                }
                let webhook_secret = credentials
                    .get("webhook_secret")
                    .filter(|value| !value.is_empty())
                    .cloned()
                    .or(db.webhook_secret.filter(|value| !value.is_empty()));
                return ResolvedCreds {
                    service: service.into(),
                    environment: environment.into(),
                    credentials,
                    webhook_secret,
                    source: "db",
                };
            }
            Ok(None) => {}
            Err(error) => {
                tracing::warn!(target:"credentials",service,environment,error=%error,"Credential storage unavailable; refusing environment fallback");
                return self.empty(service, environment, "unavailable");
            }
        }
        self.env_fallback(service, environment)
    }

    fn env_fallback(&self, service: &str, environment: &str) -> ResolvedCreds {
        let mut credentials = HashMap::new();
        let mut webhook_secret = None;

        let Some(environment) = self.scope(environment) else {
            return self.empty(service, environment, "blocked");
        };
        if service == "stripe" {
            let expected = if environment == "production" {
                "sk_live_"
            } else {
                "sk_test_"
            };
            if self.config.stripe_secret_key == "sk_test_placeholder"
                || !self.config.stripe_secret_key.starts_with(expected)
            {
                return self.empty(service, environment, "blocked");
            }
            if !self.config.stripe_secret_key.is_empty() {
                credentials.insert(
                    "secret_key".to_string(),
                    self.config.stripe_secret_key.clone(),
                );
            }
            let public_prefix = if environment == "production" {
                "pk_live_"
            } else {
                "pk_test_"
            };
            if self
                .config
                .stripe_publishable_key
                .starts_with(public_prefix)
            {
                credentials.insert(
                    "publishable_key".to_string(),
                    self.config.stripe_publishable_key.clone(),
                );
            }
            if !self.config.stripe_webhook_secret.is_empty()
                && self.config.stripe_webhook_secret != "whsec_placeholder"
            {
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
    id: Option<i64>,
    revoked: bool,
    credentials_encrypted: Option<String>,
    webhook_secret: Option<String>,
    environment: Option<String>,
    status: Option<String>,
    health_status: Option<String>,
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
    #[test]
    fn scope_normalizes_application_environments_without_mixing_live_keys() {
        let resolver = CredentialResolver::new(dummy_config("sk_test_fixture"));
        for (environment, expected) in [
            ("production", "production"),
            ("development", "sandbox"),
            ("staging", "sandbox"),
            ("test", "sandbox"),
        ] {
            let mut config = dummy_config("sk_test_fixture");
            config.environment = environment.into();
            assert_eq!(runtime_environment(&config), expected);
        }
        assert_eq!(resolver.scope("development"), Some("sandbox"));
        assert_eq!(resolver.scope("staging"), Some("sandbox"));
        assert!(
            resolver
                .env_fallback("stripe", "production")
                .credentials
                .is_empty()
        );
        assert!(
            resolver
                .env_fallback("stripe", "unrecognized")
                .credentials
                .is_empty()
        );
        let live = CredentialResolver::new(dummy_config("sk_live_fixture"));
        assert!(
            live.env_fallback("stripe", "sandbox")
                .credentials
                .is_empty()
        );
        assert!(
            live.env_fallback("stripe", "production")
                .credentials
                .contains_key("secret_key")
        );
        assert!(
            !live
                .env_fallback("stripe", "production")
                .credentials
                .contains_key("publishable_key")
        );
        let debug = format!("{:?}", resolver.env_fallback("stripe", "sandbox"));
        assert!(!debug.contains("sk_test_fixture"));
        assert!(!debug.contains("whsec_test"));
    }
    #[tokio::test]
    async fn invalidation_prevents_a_stale_fetch_repopulating_the_cache() {
        let resolver = CredentialResolver::new(dummy_config("sk_test_fixture"));
        let generation = resolver.generation.load(Ordering::Acquire);
        let old = resolver.env_fallback("stripe", "sandbox");
        resolver.invalidate("stripe", "development").await;
        assert!(
            !resolver
                .cache_put_current(cache_key("stripe", "sandbox"), old, generation)
                .await
        );
        assert!(
            resolver
                .cache_get(&cache_key("stripe", "sandbox"))
                .await
                .is_none()
        );
    }
}
