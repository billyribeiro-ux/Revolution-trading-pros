//! Application configuration

use anyhow::{Context, Result};
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};

/// A single IPv4 or IPv6 CIDR block, used for the trusted-proxy allowlist.
///
/// P1-3 (FULL_REPO_AUDIT_2026-05-17): hand-rolled (no `ipnet` dependency) —
/// containment is a fixed-width prefix-bit comparison, ~30 lines, fully
/// unit-tested. The whole point of the trusted-proxy check is correctness
/// under adversarial input, so the algorithm is kept here, small and visible,
/// rather than behind a transitive crate.
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum IpCidr {
    /// Network base address (host bits already zeroed) + prefix length 0..=32.
    V4 { base: u32, prefix: u8 },
    /// Network base address (host bits already zeroed) + prefix length 0..=128.
    V6 { base: u128, prefix: u8 },
}

impl IpCidr {
    /// Parse a single CIDR string. A bare IP (no `/`) is treated as a
    /// host route (`/32` for IPv4, `/128` for IPv6). Host bits beyond the
    /// prefix are masked off so equality/containment is canonical.
    pub fn parse(s: &str) -> Result<Self, String> {
        let s = s.trim();
        let (addr_part, prefix_part) = match s.split_once('/') {
            Some((a, p)) => (a, Some(p)),
            None => (s, None),
        };

        let addr: IpAddr = addr_part
            .parse()
            .map_err(|_| format!("invalid IP address in CIDR {s:?}"))?;

        match addr {
            IpAddr::V4(v4) => {
                let prefix = match prefix_part {
                    Some(p) => p
                        .parse::<u8>()
                        .map_err(|_| format!("invalid prefix in CIDR {s:?}"))?,
                    None => 32,
                };
                if prefix > 32 {
                    return Err(format!("IPv4 prefix out of range in CIDR {s:?}"));
                }
                let bits = u32::from(v4);
                let mask = ipv4_mask(prefix);
                Ok(IpCidr::V4 {
                    base: bits & mask,
                    prefix,
                })
            }
            IpAddr::V6(v6) => {
                let prefix = match prefix_part {
                    Some(p) => p
                        .parse::<u8>()
                        .map_err(|_| format!("invalid prefix in CIDR {s:?}"))?,
                    None => 128,
                };
                if prefix > 128 {
                    return Err(format!("IPv6 prefix out of range in CIDR {s:?}"));
                }
                let bits = u128::from(v6);
                let mask = ipv6_mask(prefix);
                Ok(IpCidr::V6 {
                    base: bits & mask,
                    prefix,
                })
            }
        }
    }

    /// True iff `ip` falls inside this CIDR block.
    ///
    /// IPv4-mapped IPv6 peers (`::ffff:a.b.c.d`, the form a dual-stack
    /// listener reports for IPv4 clients) are unwrapped so an operator can
    /// express the allowlist in plain IPv4 and still match real IPv4 hops.
    pub fn contains(&self, ip: IpAddr) -> bool {
        let ip = normalize_ip(ip);
        match (self, ip) {
            (IpCidr::V4 { base, prefix }, IpAddr::V4(v4)) => {
                let mask = ipv4_mask(*prefix);
                (u32::from(v4) & mask) == *base
            }
            (IpCidr::V6 { base, prefix }, IpAddr::V6(v6)) => {
                let mask = ipv6_mask(*prefix);
                (u128::from(v6) & mask) == *base
            }
            // Cross-family never matches (an IPv4 hop is not in an IPv6 block).
            _ => false,
        }
    }
}

impl std::fmt::Debug for IpCidr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            IpCidr::V4 { base, prefix } => {
                write!(f, "{}/{}", Ipv4Addr::from(*base), prefix)
            }
            IpCidr::V6 { base, prefix } => {
                write!(f, "{}/{}", Ipv6Addr::from(*base), prefix)
            }
        }
    }
}

/// IPv4 netmask for a prefix length. `prefix == 0` yields an all-zero
/// mask (the `<< 32` shift is UB in Rust for `u32`, so it is special-cased).
fn ipv4_mask(prefix: u8) -> u32 {
    if prefix == 0 {
        0
    } else {
        u32::MAX << (32 - prefix as u32)
    }
}

/// IPv6 netmask for a prefix length. `prefix == 0` special-cased for the
/// same overflow-shift reason as the IPv4 helper.
fn ipv6_mask(prefix: u8) -> u128 {
    if prefix == 0 {
        0
    } else {
        u128::MAX << (128 - prefix as u32)
    }
}

/// Collapse an IPv4-mapped IPv6 address (`::ffff:0:0/96`) back to its
/// IPv4 form so a single allowlist entry covers both representations.
fn normalize_ip(ip: IpAddr) -> IpAddr {
    match ip {
        IpAddr::V6(v6) => match v6.to_ipv4_mapped() {
            Some(v4) => IpAddr::V4(v4),
            None => IpAddr::V6(v6),
        },
        v4 => v4,
    }
}

/// Parse the `TRUSTED_PROXY_CIDRS` env value (comma-separated CIDRs).
///
/// Empty / unset / all-blank → empty Vec, which the resolver treats as
/// "trust NO proxy" (XFF/X-Real-IP are ignored entirely). A malformed
/// entry is a hard configuration error: silently dropping it would
/// re-open the spoofing hole this fix closes.
fn parse_trusted_proxy_cidrs(raw: &str) -> Result<Vec<IpCidr>> {
    raw.split(',')
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .map(|s| {
            IpCidr::parse(s).map_err(|msg| {
                anyhow::anyhow!("TRUSTED_PROXY_CIDRS contains an invalid entry: {msg}")
            })
        })
        .collect()
}

/// Application configuration loaded from environment variables
#[derive(Clone)]
pub struct Config {
    // Server
    pub port: u16,
    pub environment: String,

    // Database (Fly.io PostgreSQL)
    pub database_url: String,

    // Redis (Upstash)
    pub redis_url: String,

    // Cloudflare R2
    pub r2_endpoint: String,
    pub r2_access_key_id: String,
    pub r2_secret_access_key: String,
    pub r2_bucket: String,
    pub r2_public_url: String,

    // JWT Authentication
    pub jwt_secret: String,
    pub jwt_expires_in: i64, // hours

    // Stripe
    pub stripe_secret_key: String,
    pub stripe_publishable_key: String,
    pub stripe_webhook_secret: String,

    // CORS
    pub cors_origins: Vec<String>,

    /// P1-3 (FULL_REPO_AUDIT_2026-05-17): trusted reverse-proxy / load-balancer
    /// CIDR allowlist, parsed from `TRUSTED_PROXY_CIDRS` (comma-separated,
    /// IPv4 + IPv6). Empty/unset = trust NO proxy → `X-Forwarded-For` /
    /// `X-Real-IP` are ignored and the real TCP peer is used. Non-secret
    /// (it's network topology, not a credential) so it is printed in Debug.
    pub trusted_proxy_cidrs: Vec<IpCidr>,

    // Email (Postmark)
    pub postmark_token: Option<String>,
    pub from_email: String,
    pub app_url: String, // Frontend URL for email links

    /// Batch 6 — destination for operator-facing notifications
    /// (currently `dispute-created`). Empty/missing → those alerts are
    /// logged at WARN level but no email is sent.
    pub admin_notification_email: Option<String>,

    // Meilisearch
    pub meilisearch_host: String,
    pub meilisearch_api_key: String,

    // Superadmin Configuration
    pub superadmin_emails: Vec<String>,

    // Developer Configuration (Enterprise Pattern)
    pub developer_emails: Vec<String>,
    pub developer_mode: bool,

    // Developer Bootstrap (ICT 7 - No Hardcoded Credentials)
    // These are read from environment variables and used to bootstrap developer account on startup
    pub developer_bootstrap_email: Option<String>,
    pub developer_bootstrap_password_hash: Option<String>,
    pub developer_bootstrap_name: Option<String>,

    // OAuth Configuration (ICT Level 7 - Google & Apple Sign-In)
    pub google_client_id: Option<String>,
    pub google_client_secret: Option<String>,
    pub apple_client_id: Option<String>,
    pub apple_team_id: Option<String>,
    pub apple_key_id: Option<String>,
    pub apple_private_key: Option<String>,

    // Member-content signing secrets (download capability tokens + license keys).
    // RUST_DEEP_AUDIT_2026-06-07 (P0-2/P1-3): these key the HMAC for member
    // download tokens and license keys. Previously read inline via
    // `std::env::var(...).unwrap_or_default()`, which silently signed those
    // capability tokens with an EMPTY secret when unset, making them forgeable.
    // Now read at boot (fail-closed in production) and validated below.
    pub member_indicator_secret: String,
    pub member_license_secret: String,
}

impl std::fmt::Debug for Config {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Config")
            // Non-sensitive: server configuration
            .field("port", &self.port)
            .field("environment", &self.environment)
            // Sensitive: database credentials
            .field("database_url", &"[REDACTED]")
            .field("redis_url", &"[REDACTED]")
            // Sensitive: R2 credentials (endpoint/bucket/public_url are non-sensitive)
            .field("r2_endpoint", &self.r2_endpoint)
            .field("r2_access_key_id", &"[REDACTED]")
            .field("r2_secret_access_key", &"[REDACTED]")
            .field("r2_bucket", &self.r2_bucket)
            .field("r2_public_url", &self.r2_public_url)
            // Sensitive: JWT
            .field("jwt_secret", &"[REDACTED]")
            .field("jwt_expires_in", &self.jwt_expires_in)
            // Sensitive: Stripe
            .field("stripe_secret_key", &"[REDACTED]")
            .field("stripe_publishable_key", &"[REDACTED]")
            .field("stripe_webhook_secret", &"[REDACTED]")
            // Non-sensitive: CORS
            .field("cors_origins", &self.cors_origins)
            // Non-sensitive: network topology, not a credential.
            .field("trusted_proxy_cidrs", &self.trusted_proxy_cidrs)
            // Sensitive: Postmark token
            .field(
                "postmark_token",
                &self.postmark_token.as_ref().map(|_| "[REDACTED]"),
            )
            .field("from_email", &self.from_email)
            .field("app_url", &self.app_url)
            // Sensitive: Meilisearch
            .field("meilisearch_host", &self.meilisearch_host)
            .field("meilisearch_api_key", &"[REDACTED]")
            // Non-sensitive: admin/developer config
            .field("superadmin_emails", &self.superadmin_emails)
            .field("developer_emails", &self.developer_emails)
            .field("developer_mode", &self.developer_mode)
            // Sensitive: developer bootstrap
            .field("developer_bootstrap_email", &self.developer_bootstrap_email)
            .field(
                "developer_bootstrap_password_hash",
                &self
                    .developer_bootstrap_password_hash
                    .as_ref()
                    .map(|_| "[REDACTED]"),
            )
            .field("developer_bootstrap_name", &self.developer_bootstrap_name)
            // Sensitive: OAuth
            .field("google_client_id", &self.google_client_id)
            .field(
                "google_client_secret",
                &self.google_client_secret.as_ref().map(|_| "[REDACTED]"),
            )
            .field("apple_client_id", &self.apple_client_id)
            .field("apple_team_id", &self.apple_team_id)
            .field("apple_key_id", &"[REDACTED]")
            .field(
                "apple_private_key",
                &self.apple_private_key.as_ref().map(|_| "[REDACTED]"),
            )
            // Sensitive: member-content signing secrets
            .field("member_indicator_secret", &"[REDACTED]")
            .field("member_license_secret", &"[REDACTED]")
            .finish()
    }
}

impl Config {
    pub fn from_env() -> Result<Self> {
        let environment =
            std::env::var("ENVIRONMENT").unwrap_or_else(|_| "development".to_string());
        let is_dev = environment == "development";

        // FIX-2026-04-27: defence-in-depth. The dev-only fallback below would
        // silently use placeholder R2/Stripe/Meili creds if ENVIRONMENT=development
        // ever landed in production secrets by accident. Refuse to boot when
        // ENVIRONMENT is "development" but APP_URL points at a known prod host.
        if is_dev {
            let app_url = std::env::var("APP_URL").unwrap_or_default();
            // Deploy target deferred — extend this list when production hosts are chosen.
            const PROD_INDICATORS: &[&str] = &["revolutiontradingpros.com"];
            if PROD_INDICATORS.iter().any(|d| app_url.contains(d)) {
                panic!(
                    "FATAL: ENVIRONMENT=development but APP_URL ({app_url}) looks like production. \
                     Refusing to start with placeholder credentials in production. \
                     Set ENVIRONMENT=production or fix APP_URL."
                );
            }
        }

        // FIX-2026-04-27: dev-only fallback. In production these still hard-fail
        // via `.context(...)?`; in development we accept missing values so the
        // local stack can boot without R2/Stripe/Meili creds. Real uploads /
        // payments / search will still 500 at the call site if used.
        fn required_or_dev(key: &str, dev: bool, fallback: &str) -> Result<String> {
            match std::env::var(key) {
                Ok(v) => Ok(v),
                Err(_) if dev => {
                    tracing::warn!(
                        "{key} not set - using development fallback. Feature using {key} will not work end-to-end."
                    );
                    Ok(fallback.to_string())
                }
                Err(e) => Err(anyhow::Error::new(e).context(format!("{key} is required"))),
            }
        }

        Ok(Self {
            port: std::env::var("PORT")
                .unwrap_or_else(|_| "8080".to_string())
                .parse()
                .context("Invalid PORT")?,
            environment,

            database_url: std::env::var("DATABASE_URL").context("DATABASE_URL required")?,
            redis_url: std::env::var("REDIS_URL")
                .unwrap_or_else(|_| "redis://localhost:6379".to_string()),

            r2_endpoint: required_or_dev(
                "R2_ENDPOINT",
                is_dev,
                "https://example-account.r2.cloudflarestorage.com",
            )?,
            r2_access_key_id: required_or_dev("R2_ACCESS_KEY_ID", is_dev, "dev-placeholder")?,
            r2_secret_access_key: required_or_dev(
                "R2_SECRET_ACCESS_KEY",
                is_dev,
                "dev-placeholder",
            )?,
            r2_bucket: required_or_dev("R2_BUCKET", is_dev, "revolution-trading-media")?,
            r2_public_url: std::env::var("R2_PUBLIC_URL").unwrap_or_else(|_| {
                "https://pub-2e5bd1b702b440bd888a0fc47f3493ae.r2.dev".to_string()
            }),

            jwt_secret: std::env::var("JWT_SECRET").context("JWT_SECRET required")?,
            // FIX-2026-04-27 (H-1): Default TTL reduced from 24h to 1h.
            // 24h is excessive for a site handling PII/payments; OWASP guidance is 15-60 min.
            jwt_expires_in: std::env::var("JWT_EXPIRES_IN")
                .unwrap_or_else(|_| "1".to_string())
                .parse()
                .unwrap_or(1),

            stripe_secret_key: required_or_dev("STRIPE_SECRET", is_dev, "sk_test_placeholder")?,
            stripe_publishable_key: std::env::var("STRIPE_PUBLISHABLE_KEY").unwrap_or_else(|_| {
                tracing::warn!("STRIPE_PUBLISHABLE_KEY not set - payment features will not work");
                String::new()
            }),
            stripe_webhook_secret: required_or_dev(
                "STRIPE_WEBHOOK_SECRET",
                is_dev,
                "whsec_placeholder",
            )?,

            cors_origins: std::env::var("CORS_ORIGINS")
                .unwrap_or_else(|_| {
                    // ICT 7: Include all localhost origins for development and E2E testing
                    let origins = [
                        "https://revolution-trading-pros.pages.dev",
                        "https://www.revolution-trading-pros.pages.dev",
                        "http://localhost:5173",
                        "http://localhost:5174", // Playwright E2E tests
                        "http://localhost:3000",
                        "http://127.0.0.1:5173",
                        "http://127.0.0.1:5174", // Playwright E2E tests
                        "http://127.0.0.1:3000",
                    ];
                    origins.join(",")
                })
                .split(',')
                .map(|s| s.trim().to_string())
                .collect(),

            // P1-3 (FULL_REPO_AUDIT_2026-05-17): trusted-proxy allowlist.
            // Hard-fail on a malformed entry — a typo here must not silently
            // degrade to "trust everything" / "trust nothing" without notice.
            trusted_proxy_cidrs: parse_trusted_proxy_cidrs(
                &std::env::var("TRUSTED_PROXY_CIDRS").unwrap_or_default(),
            )
            .context("Invalid TRUSTED_PROXY_CIDRS")?,

            postmark_token: std::env::var("POSTMARK_TOKEN")
                .ok()
                .filter(|s| !s.trim().is_empty()),
            from_email: std::env::var("POSTMARK_FROM_EMAIL")
                .or_else(|_| std::env::var("FROM_EMAIL"))
                .unwrap_or_else(|_| "noreply@example.com".to_string()),
            app_url: std::env::var("APP_URL")
                .unwrap_or_else(|_| "https://revolution-trading-pros.pages.dev".to_string()),
            admin_notification_email: std::env::var("ADMIN_NOTIFICATION_EMAIL")
                .ok()
                .filter(|s| !s.trim().is_empty()),

            meilisearch_host: std::env::var("MEILISEARCH_HOST")
                .unwrap_or_else(|_| "http://localhost:7700".to_string()),
            meilisearch_api_key: required_or_dev("MEILISEARCH_API_KEY", is_dev, "dev-placeholder")?,

            // ICT 11+: NO HARDCODED EMAILS - must be set via environment variables
            superadmin_emails: std::env::var("SUPERADMIN_EMAILS")
                .unwrap_or_default()
                .split(',')
                .map(|s| s.trim().to_lowercase())
                .filter(|s| !s.is_empty())
                .collect(),

            // ICT 11+: NO HARDCODED EMAILS - must be set via environment variables
            developer_emails: std::env::var("DEVELOPER_EMAILS")
                .unwrap_or_default()
                .split(',')
                .map(|s| s.trim().to_lowercase())
                .filter(|s| !s.is_empty())
                .collect(),

            developer_mode: std::env::var("DEVELOPER_MODE")
                .unwrap_or_else(|_| "false".to_string())
                .parse()
                .unwrap_or(false),

            // ICT 7 Developer Bootstrap - Read from secure environment variables
            developer_bootstrap_email: std::env::var("DEVELOPER_BOOTSTRAP_EMAIL").ok(),
            developer_bootstrap_password_hash: std::env::var("DEVELOPER_BOOTSTRAP_PASSWORD_HASH")
                .ok(),
            developer_bootstrap_name: std::env::var("DEVELOPER_BOOTSTRAP_NAME").ok(),

            // ICT Level 7: OAuth Configuration for Google & Apple Sign-In
            google_client_id: std::env::var("GOOGLE_CLIENT_ID").ok(),
            google_client_secret: std::env::var("GOOGLE_CLIENT_SECRET").ok(),
            apple_client_id: std::env::var("APPLE_CLIENT_ID").ok(),
            apple_team_id: std::env::var("APPLE_TEAM_ID").ok(),
            apple_key_id: std::env::var("APPLE_KEY_ID").ok(),
            apple_private_key: std::env::var("APPLE_PRIVATE_KEY").ok(),

            // RUST_DEEP_AUDIT_2026-06-07 (P0-2/P1-3): required in production
            // (hard-fail via `required_or_dev`), with a non-empty dev fallback so
            // the local stack still boots. Strength is enforced in
            // `validate_production_secrets`.
            member_indicator_secret: required_or_dev(
                "MEMBER_INDICATOR_SECRET",
                is_dev,
                "dev-member-indicator-secret-placeholder-not-for-production",
            )?,
            member_license_secret: required_or_dev(
                "MEMBER_LICENSE_SECRET",
                is_dev,
                "dev-member-license-secret-placeholder-not-for-production",
            )?,
        })
    }

    pub fn is_production(&self) -> bool {
        self.environment == "production"
    }

    /// Pre-launch assertion: in production, every secret that has a
    /// distinct test/live form MUST be the live form. Catches the
    /// "test key in prod" misconfiguration that Batch 7 §AC was filed
    /// for. Intended to be called from `main.rs` immediately after
    /// `from_env()` so a misconfigured deploy crashes at boot rather
    /// than silently routing real customer payments through Stripe
    /// test mode.
    ///
    /// Hard-fails (panics) rather than returning an error because a
    /// production binary booting with placeholder credentials is a
    /// security incident, not a startup hiccup.
    pub fn validate_production_secrets(&self) {
        if !self.is_production() {
            return;
        }

        // FIX-H-1 (2026-04-29): JWT_SECRET must be a real, sufficiently
        // long random secret in production. The reader at line 208 only
        // checks "is set" — if the placeholder string from .env.example
        // ever leaks into a production deploy, JWTs become forgeable in
        // minutes. We fail-fast at boot rather than serve a single request.
        //
        // 32 bytes is the OWASP minimum for HS256 (== HMAC key length
        // recommended by NIST SP 800-117). Hex-encoded that is 64 chars;
        // we accept >=32 chars to also allow base64 / random ASCII forms.
        if self.jwt_secret.len() < 32 {
            panic!(
                "FATAL: ENVIRONMENT=production but JWT_SECRET is < 32 chars (got {} chars). \
                 Refusing to boot — short secrets are brute-forceable. Generate one with \
                 `openssl rand -hex 32` and set JWT_SECRET in your production secret store.",
                self.jwt_secret.len()
            );
        }
        // Reject the .env.example placeholder explicitly. The placeholder
        // string is "replace-me-with-a-long-random-secret-at-least-32-characters-long"
        // which is >= 32 chars, so the length check above does not catch it.
        let lower = self.jwt_secret.to_ascii_lowercase();
        if lower.contains("replace-me")
            || lower.contains("placeholder")
            || lower.contains("changeme")
            || lower.contains("your-secret-here")
        {
            panic!(
                "FATAL: ENVIRONMENT=production but JWT_SECRET appears to be a placeholder. \
                 Refusing to boot — this is the value from .env.example, not a real secret. \
                 Generate one with `openssl rand -hex 32` and rotate it in your production \
                 secret store."
            );
        }

        // Stripe secret key
        if !self.stripe_secret_key.starts_with("sk_live_") {
            let prefix = self
                .stripe_secret_key
                .split('_')
                .take(2)
                .collect::<Vec<_>>()
                .join("_");
            panic!(
                "FATAL: ENVIRONMENT=production but STRIPE_SECRET does not start with 'sk_live_' \
                 (got prefix: '{prefix}_...'). Refusing to boot — would route real customer \
                 payments through Stripe test mode. Set STRIPE_SECRET to a live key, or set \
                 ENVIRONMENT=staging if this is intentional."
            );
        }

        // Stripe webhook secret — should not be the placeholder
        if self.stripe_webhook_secret == "whsec_placeholder"
            || self.stripe_webhook_secret.is_empty()
        {
            panic!(
                "FATAL: ENVIRONMENT=production but STRIPE_WEBHOOK_SECRET is unset or placeholder. \
                 Webhook signature verification would fall through. Refusing to boot."
            );
        }

        // Webhook secret prefix sanity. Stripe live webhook secrets
        // start with "whsec_" and are >= 32 chars after the prefix.
        if !self.stripe_webhook_secret.starts_with("whsec_") {
            panic!("FATAL: STRIPE_WEBHOOK_SECRET does not start with 'whsec_'. Refusing to boot.");
        }

        // Stripe publishable key parity check — if set, must match live mode.
        if !self.stripe_publishable_key.is_empty()
            && !self.stripe_publishable_key.starts_with("pk_live_")
        {
            panic!(
                "FATAL: ENVIRONMENT=production but STRIPE_PUBLISHABLE_KEY does not start with \
                 'pk_live_'. Refusing to boot — frontend would send card data to a test-mode \
                 Stripe.js."
            );
        }

        // RUST_DEEP_AUDIT_2026-06-07 (P0-2/P1-3): member-content signing secrets
        // key the HMAC for download capability tokens and license keys. An empty,
        // short, or placeholder value makes those tokens forgeable offline. Same
        // >=32-char bar as JWT_SECRET; fail-fast at boot rather than serve one
        // forgeable token.
        for (name, value) in [
            ("MEMBER_INDICATOR_SECRET", &self.member_indicator_secret),
            ("MEMBER_LICENSE_SECRET", &self.member_license_secret),
        ] {
            if value.len() < 32 || value.to_ascii_lowercase().contains("placeholder") {
                panic!(
                    "FATAL: ENVIRONMENT=production but {name} is unset, < 32 chars, or a \
                     placeholder. It keys the HMAC for member download/license tokens; a weak \
                     value makes them forgeable. Generate one with `openssl rand -hex 32`."
                );
            }
        }

        tracing::info!(
            target: "startup",
            event = "production_secrets_validated",
            "All production secrets passed prefix validation (sk_live_, whsec_, pk_live_)"
        );
    }

    /// security-M1 (FULL_REPO_AUDIT_2026-05-17): raw env-list membership ONLY.
    ///
    /// !!! HARDENING — DO NOT USE FOR AUTHORIZATION/ELEVATION DECISIONS !!!
    ///
    /// This is a *break-glass hint*, never a standalone authorization
    /// decision. Treating "the caller controls the configured
    /// `SUPERADMIN_EMAILS` value" as proof of identity let anyone who could
    /// set/spoof that email obtain instant, *unverified* superadmin
    /// elevation. Root cause: an env email-list string was treated as an
    /// identity credential.
    ///
    /// Callers that make a role/elevation decision MUST use
    /// [`Config::is_superadmin_email_strict`], which additionally requires the
    /// account to be email-verified AND to carry a real DB
    /// superadmin/admin/developer role. The bare form below is retained only
    /// for non-elevating diagnostics (e.g. logging "this address is on the
    /// configured list").
    ///
    /// A compiler `#[deprecated]` is deliberately NOT applied here: the
    /// remaining caller is `src/middleware/admin.rs` (outside this fix's edit
    /// ownership), and `#[deprecated]` would turn that file's usage into a
    /// hard error under the repo's `clippy -D warnings` gate without a way to
    /// fix it from an owned file. The enforcement is instead done at every
    /// owned call site (`auth.rs` login) plus the precise REQUIRED-COMPANION
    /// marker below for the middleware caller.
    pub fn is_superadmin_email(&self, email: &str) -> bool {
        self.superadmin_emails.contains(&email.to_lowercase())
    }

    /// security-M1 (FULL_REPO_AUDIT_2026-05-17): raw env-list membership ONLY.
    ///
    /// !!! HARDENING — DO NOT USE FOR AUTHORIZATION/ELEVATION DECISIONS !!!
    ///
    /// Same hardening rationale as [`Config::is_superadmin_email`] — use
    /// [`Config::is_developer_email_strict`] for any elevation decision; the
    /// bare form is break-glass / non-elevating-diagnostics only.
    pub fn is_developer_email(&self, email: &str) -> bool {
        self.developer_emails.contains(&email.to_lowercase())
    }

    /// security-M1 (FULL_REPO_AUDIT_2026-05-17): strict superadmin check.
    ///
    /// Honors the configured `SUPERADMIN_EMAILS` allowlist as *at most a
    /// secondary hint* layered on top of a verified, DB-role'd account.
    /// Returns `true` iff ALL of:
    ///
    /// 1. `email` is in the configured `SUPERADMIN_EMAILS` list, AND
    /// 2. `email_verified` is `true` (the user row's `email_verified_at` is
    ///    set — controlling the configured email is not enough; the address
    ///    must have actually been proven), AND
    /// 3. `role` is a real DB privileged role (`super_admin` / `super-admin`
    ///    / `admin` / `developer`) — the email list can only *confirm*
    ///    elevation that the database already grants, never *create* it.
    ///
    /// This closes the "controls the configured email ⇒ instant unverified
    /// elevation" hole while preserving every legitimate verified-admin path:
    /// a real superadmin whose row is verified and DB-role'd is unaffected.
    pub fn is_superadmin_email_strict(
        &self,
        email: &str,
        role: Option<&str>,
        email_verified: bool,
    ) -> bool {
        if !email_verified {
            return false;
        }
        if !self.superadmin_emails.contains(&email.to_lowercase()) {
            return false;
        }
        matches!(
            role,
            Some("super_admin") | Some("super-admin") | Some("admin") | Some("developer")
        )
    }

    /// security-M1 (FULL_REPO_AUDIT_2026-05-17): strict developer check.
    ///
    /// Same three-part contract as [`Config::is_superadmin_email_strict`]
    /// but against the `DEVELOPER_EMAILS` allowlist. Requires the account to
    /// be email-verified AND to carry a real DB privileged role; the env
    /// list is only a secondary confirmation, never a standalone gate.
    pub fn is_developer_email_strict(
        &self,
        email: &str,
        role: Option<&str>,
        email_verified: bool,
    ) -> bool {
        if !email_verified {
            return false;
        }
        if !self.developer_emails.contains(&email.to_lowercase()) {
            return false;
        }
        matches!(
            role,
            Some("super_admin") | Some("super-admin") | Some("admin") | Some("developer")
        )
    }

    // ── security-M1 — COMPANION IMPLEMENTED ──────────────────────────────
    // The `AdminUser` / `SuperAdminUser` extractors in
    // `src/middleware/admin.rs` were switched from the bare
    // `is_superadmin_email(email)` to `is_superadmin_email_strict(
    // &user.email, user.role.as_deref(), user.email_verified_at.is_some())`,
    // closing the email-list-as-identity elevation path at the extractors
    // too. The bare helpers above remain documented break-glass only.

    /// Check if developer mode is enabled
    pub fn is_developer_mode(&self) -> bool {
        self.developer_mode || !self.is_production()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Build a `Config` with all-live-prefix placeholders, suitable as a
    /// starting point for `validate_production_secrets()` panic-branch
    /// tests. Each test mutates the field it wants to exercise. NOT
    /// exposed outside `#[cfg(test)]` because real boot must go through
    /// `from_env()`.
    ///
    /// Secret values are assembled at runtime from prefix + body to
    /// avoid tripping GitHub's push-protection secret scanner on the
    /// literal `sk_live_<24+ chars>` regex.
    fn live_config() -> Config {
        // Build clearly-synthetic test values that satisfy each prefix
        // gate without producing a literal that looks like a real key.
        let body = "TEST".repeat(8); // 32 ASCII chars — passes any "long enough" sanity check
        Config {
            port: 8080,
            environment: "production".to_string(),
            database_url: "postgres://test/test".to_string(),
            redis_url: "redis://localhost:6379".to_string(),
            r2_endpoint: String::new(),
            r2_access_key_id: String::new(),
            r2_secret_access_key: String::new(),
            r2_bucket: String::new(),
            r2_public_url: String::new(),
            // FIX-H-1 (2026-04-29): live_config now provides a JWT_SECRET that
            // satisfies the new production assertion (>=32 chars, not a placeholder).
            // Previously was "x" (1 char) — caught by the new check.
            jwt_secret: "test-jwt-secret-thirty-two-chars-or-more-for-tests-only-not-a-real-secret"
                .to_string(),
            jwt_expires_in: 1,
            stripe_secret_key: format!("sk_live_{body}"),
            stripe_publishable_key: format!("pk_live_{body}"),
            stripe_webhook_secret: format!("whsec_{body}"),
            cors_origins: vec![],
            trusted_proxy_cidrs: vec![],
            postmark_token: None,
            from_email: "n@e".to_string(),
            app_url: "https://example.com".to_string(),
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
            member_indicator_secret: format!("member-indicator-{body}"),
            member_license_secret: format!("member-license-{body}"),
        }
    }

    #[test]
    fn validate_production_secrets_passes_with_all_live_keys() {
        let cfg = live_config();
        // Should not panic.
        cfg.validate_production_secrets();
    }

    #[test]
    fn validate_production_secrets_is_noop_outside_production() {
        let body = "TEST".repeat(8);
        let mut cfg = live_config();
        cfg.environment = "development".to_string();
        cfg.stripe_secret_key = format!("sk_test_{body}");
        cfg.stripe_webhook_secret = "whsec_placeholder".to_string();
        cfg.stripe_publishable_key = format!("pk_test_{body}");
        // Should not panic; dev/staging keep test-mode keys.
        cfg.validate_production_secrets();
    }

    #[test]
    #[should_panic(expected = "STRIPE_SECRET does not start with 'sk_live_'")]
    fn validate_production_secrets_panics_on_test_secret() {
        let body = "TEST".repeat(8);
        let mut cfg = live_config();
        cfg.stripe_secret_key = format!("sk_test_{body}");
        cfg.validate_production_secrets();
    }

    #[test]
    #[should_panic(expected = "STRIPE_WEBHOOK_SECRET is unset or placeholder")]
    fn validate_production_secrets_panics_on_placeholder_webhook_secret() {
        let mut cfg = live_config();
        cfg.stripe_webhook_secret = "whsec_placeholder".to_string();
        cfg.validate_production_secrets();
    }

    #[test]
    #[should_panic(expected = "STRIPE_WEBHOOK_SECRET is unset or placeholder")]
    fn validate_production_secrets_panics_on_empty_webhook_secret() {
        let mut cfg = live_config();
        cfg.stripe_webhook_secret = String::new();
        cfg.validate_production_secrets();
    }

    #[test]
    #[should_panic(expected = "STRIPE_WEBHOOK_SECRET does not start with 'whsec_'")]
    fn validate_production_secrets_panics_on_bad_webhook_prefix() {
        let body = "TEST".repeat(8);
        let mut cfg = live_config();
        cfg.stripe_webhook_secret = format!("wrong_prefix_{body}");
        cfg.validate_production_secrets();
    }

    #[test]
    #[should_panic(expected = "STRIPE_PUBLISHABLE_KEY does not start with 'pk_live_'")]
    fn validate_production_secrets_panics_on_test_publishable() {
        let body = "TEST".repeat(8);
        let mut cfg = live_config();
        cfg.stripe_publishable_key = format!("pk_test_{body}");
        cfg.validate_production_secrets();
    }

    #[test]
    fn validate_production_secrets_allows_empty_publishable() {
        // Publishable key is optional (frontend may pull from a separate
        // env var); only check the prefix when one IS set.
        let mut cfg = live_config();
        cfg.stripe_publishable_key = String::new();
        cfg.validate_production_secrets();
    }

    // ── RUST_DEEP_AUDIT_2026-06-07 (P0-2/P1-3): member-content signing secrets ──

    #[test]
    #[should_panic(expected = "MEMBER_INDICATOR_SECRET is unset, < 32 chars")]
    fn validate_production_secrets_panics_on_short_member_indicator_secret() {
        let mut cfg = live_config();
        cfg.member_indicator_secret = "too-short".to_string();
        cfg.validate_production_secrets();
    }

    #[test]
    #[should_panic(expected = "MEMBER_LICENSE_SECRET is unset, < 32 chars")]
    fn validate_production_secrets_panics_on_placeholder_member_license_secret() {
        let mut cfg = live_config();
        // > 32 chars but contains "placeholder" — must still be rejected.
        cfg.member_license_secret =
            "dev-member-license-secret-placeholder-not-for-production".to_string();
        cfg.validate_production_secrets();
    }

    // ── FIX-H-1 (2026-04-29): JWT_SECRET production assertion tests ──

    #[test]
    #[should_panic(expected = "JWT_SECRET is < 32 chars")]
    fn validate_production_secrets_panics_on_short_jwt_secret() {
        let mut cfg = live_config();
        cfg.jwt_secret = "too-short".to_string();
        cfg.validate_production_secrets();
    }

    #[test]
    #[should_panic(expected = "JWT_SECRET is < 32 chars")]
    fn validate_production_secrets_panics_on_empty_jwt_secret() {
        let mut cfg = live_config();
        cfg.jwt_secret = String::new();
        cfg.validate_production_secrets();
    }

    #[test]
    #[should_panic(expected = "JWT_SECRET appears to be a placeholder")]
    fn validate_production_secrets_panics_on_replace_me_placeholder() {
        let mut cfg = live_config();
        // The literal placeholder from .env.example. >=32 chars so the
        // length check is not what catches it — the substring scan is.
        cfg.jwt_secret =
            "replace-me-with-a-long-random-secret-at-least-32-characters-long".to_string();
        cfg.validate_production_secrets();
    }

    #[test]
    #[should_panic(expected = "JWT_SECRET appears to be a placeholder")]
    fn validate_production_secrets_panics_on_placeholder_substring() {
        let mut cfg = live_config();
        cfg.jwt_secret =
            "this-string-contains-the-placeholder-marker-and-is-long-enough".to_string();
        cfg.validate_production_secrets();
    }

    #[test]
    #[should_panic(expected = "JWT_SECRET appears to be a placeholder")]
    fn validate_production_secrets_panics_on_changeme() {
        let mut cfg = live_config();
        cfg.jwt_secret = "please-changeme-this-is-not-a-real-secret-yet-no-no".to_string();
        cfg.validate_production_secrets();
    }

    // ── P1-3 (FULL_REPO_AUDIT_2026-05-17): IpCidr containment primitive ──
    //
    // The spoof-resistant client-IP resolver is end-to-end tested in
    // tests/client_ip_test.rs. These unit tests pin the CIDR math itself
    // (boundary prefixes, /0, /32, /128, IPv4-mapped IPv6, cross-family).

    fn ip(s: &str) -> std::net::IpAddr {
        s.parse().unwrap()
    }

    #[test]
    fn cidr_parse_bare_ip_is_host_route() {
        assert_eq!(
            IpCidr::parse("10.0.0.5").unwrap(),
            IpCidr::V4 {
                base: u32::from(std::net::Ipv4Addr::new(10, 0, 0, 5)),
                prefix: 32
            }
        );
        let v6 = IpCidr::parse("2001:db8::1").unwrap();
        assert!(matches!(v6, IpCidr::V6 { prefix: 128, .. }));
    }

    #[test]
    fn cidr_v4_containment_and_masking() {
        let net = IpCidr::parse("10.0.0.0/8").unwrap();
        assert!(net.contains(ip("10.255.255.255")));
        assert!(net.contains(ip("10.0.0.1")));
        assert!(!net.contains(ip("11.0.0.1")));
        assert!(!net.contains(ip("9.255.255.255")));

        // Host bits in the spec are masked off → canonical base.
        let masked = IpCidr::parse("10.1.2.3/8").unwrap();
        assert!(masked.contains(ip("10.9.9.9")));

        // /32 is an exact host match.
        let host = IpCidr::parse("192.168.1.50/32").unwrap();
        assert!(host.contains(ip("192.168.1.50")));
        assert!(!host.contains(ip("192.168.1.51")));
    }

    #[test]
    fn cidr_v4_zero_prefix_matches_everything() {
        let any = IpCidr::parse("0.0.0.0/0").unwrap();
        assert!(any.contains(ip("8.8.8.8")));
        assert!(any.contains(ip("255.255.255.255")));
        assert!(any.contains(ip("0.0.0.0")));
    }

    #[test]
    fn cidr_v6_containment() {
        let net = IpCidr::parse("2001:db8::/32").unwrap();
        assert!(net.contains(ip("2001:db8::1")));
        assert!(net.contains(ip("2001:db8:ffff::1")));
        assert!(!net.contains(ip("2001:db9::1")));

        let any6 = IpCidr::parse("::/0").unwrap();
        assert!(any6.contains(ip("::1")));
        assert!(any6.contains(ip("2001:db8::dead:beef")));
    }

    #[test]
    fn cidr_v4_mapped_v6_peer_matches_v4_block() {
        // A dual-stack listener reports IPv4 clients as ::ffff:a.b.c.d.
        // An operator-written IPv4 allowlist must still match them.
        let net = IpCidr::parse("203.0.113.0/24").unwrap();
        assert!(net.contains(ip("::ffff:203.0.113.7")));
        assert!(!net.contains(ip("::ffff:203.0.114.7")));
    }

    #[test]
    fn cidr_cross_family_never_matches() {
        let v4 = IpCidr::parse("10.0.0.0/8").unwrap();
        assert!(!v4.contains(ip("2001:db8::1")));
        let v6 = IpCidr::parse("2001:db8::/32").unwrap();
        assert!(!v6.contains(ip("10.0.0.1")));
    }

    #[test]
    fn cidr_parse_rejects_garbage() {
        assert!(IpCidr::parse("not-an-ip").is_err());
        assert!(IpCidr::parse("10.0.0.0/33").is_err());
        assert!(IpCidr::parse("2001:db8::/129").is_err());
        assert!(IpCidr::parse("10.0.0.0/abc").is_err());
    }

    #[test]
    fn trusted_proxy_cidrs_parsing() {
        // Empty / blank → trust nobody.
        assert!(parse_trusted_proxy_cidrs("").unwrap().is_empty());
        assert!(parse_trusted_proxy_cidrs("  ,  , ").unwrap().is_empty());

        // Mixed IPv4 + IPv6, whitespace tolerated.
        let v = parse_trusted_proxy_cidrs("10.0.0.0/8 , 2001:db8::/32, 192.168.1.1").unwrap();
        assert_eq!(v.len(), 3);

        // One bad entry fails the whole parse (fail-loud, not fail-open).
        assert!(parse_trusted_proxy_cidrs("10.0.0.0/8, garbage").is_err());
    }
}
