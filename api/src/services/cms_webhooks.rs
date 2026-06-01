//! CMS Webhook Delivery Service - Apple ICT 7+ Principal Engineer Grade
//!
//! Handles webhook delivery with:
//! - Async delivery with retries
//! - HMAC signature generation
//! - Exponential backoff
//! - Delivery logging
//!
//! @version 2.0.0 - January 2026

use crate::config::IpCidr;
use anyhow::{anyhow, Result};
use chrono::{DateTime, Duration, Utc};
use hmac::{Hmac, Mac};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value as JsonValue};
use sha2::Sha256;
use sqlx::PgPool;
use std::net::{IpAddr, ToSocketAddrs};
use std::sync::OnceLock;
use uuid::Uuid;

type HmacSha256 = Hmac<Sha256>;

// ─────────────────────────────────────────────────────────────────────────────
// P2-E (FULL_REPO_AUDIT_2026-05-17): SSRF egress guard for admin-stored
// webhook URLs.
//
// Root cause: `config.url` is a free-form value an admin (or any future
// lower-privilege CMS role with write access to `cms_webhooks.url`) controls.
// Without an egress policy the delivery worker will happily `POST` it at
// `http://169.254.169.254/...` (cloud instance-metadata → IAM creds),
// `http://127.0.0.1:<port>` (loopback admin panels), or any RFC-1918 /
// unique-local internal service and stream the response body back into
// `cms_webhook_deliveries.response_body`, where the same admin can read it.
//
// Policy (defence in depth, fail-closed):
//   1. Parse the URL. Reject anything that is not absolute / not parseable.
//   2. https-only egress. Webhook targets are external customer endpoints;
//      there is no legitimate plaintext-http use in this repo (no http seed
//      URL, no schema default, no caller), so http is rejected outright
//      rather than env-gated — env-gating an unused, footgun-shaped path
//      only widens the attack surface for the next person.
//   3. Reject by HOST literal IP *before* DNS, so `http(s)://127.0.0.1`,
//      `https://[::1]`, `https://169.254.169.254` are caught even if no
//      resolver is reachable (pre-DNS short-circuit; no rebinding window).
//   4. Resolve the host to *every* address it maps to and reject if ANY of
//      them is forbidden. A name that resolves to one public + one private
//      A record is rejected (closes the "split-horizon / DNS-rebind to a
//      mixed answer" trick).
//   5. Resolution failure → reject (fail closed, never "allow on error").
//
// The IP decision is a pure function (`is_forbidden_egress_ip`) so it is
// exhaustively unit-testable with no network and no DB.
//
// DRY: the CIDR containment primitive is `crate::config::IpCidr` (P1-3,
// already unit-tested, handles IPv4 / IPv6 / `::ffff:` IPv4-mapped / `/0`).
// We only assemble the forbidden *set* here; we do not re-implement masking.
// ─────────────────────────────────────────────────────────────────────────────

/// The set of CIDR blocks a webhook target is NEVER allowed to resolve into.
///
/// Built once and cached. Every literal here is a constant, so `parse()`
/// cannot fail at runtime in practice; an unreachable parse error is still
/// surfaced via `expect` rather than silently dropped (a missing entry would
/// re-open the hole this fix closes — exactly the failure mode the trusted
/// proxy parser warns about).
fn forbidden_egress_cidrs() -> &'static [IpCidr] {
    static SET: OnceLock<Vec<IpCidr>> = OnceLock::new();
    SET.get_or_init(|| {
        // NOTE: IpCidr::contains() already collapses `::ffff:a.b.c.d`
        // (IPv4-mapped IPv6) to its IPv4 form, so the IPv4 entries below
        // also cover the mapped representation — no separate ::ffff:
        // entries are required for the IPv4 ranges.
        const FORBIDDEN: &[&str] = &[
            // ── Unspecified / "this host" ──
            "0.0.0.0/8", // 0.0.0.0/8 incl. the 0.0.0.0 unspecified addr
            "::/128",    // IPv6 unspecified ::
            // ── Loopback ──
            "127.0.0.0/8", // 127.0.0.1 .. 127.255.255.255
            "::1/128",     // IPv6 loopback
            // ── Link-local — INCLUDES cloud metadata 169.254.169.254 ──
            "169.254.0.0/16", // IPv4 link-local (AWS/GCP/Azure/DO IMDS)
            "fe80::/10",      // IPv6 link-local
            // ── RFC-1918 private ──
            "10.0.0.0/8",
            "172.16.0.0/12",
            "192.168.0.0/16",
            // ── Carrier-grade NAT (shared address space, RFC 6598) ──
            "100.64.0.0/10",
            // ── IPv6 unique-local (fc00::/7 covers fc00:: and fd00::) ──
            "fc00::/7",
        ];
        FORBIDDEN
            .iter()
            .map(|c| {
                IpCidr::parse(c).unwrap_or_else(|e| panic!("SSRF guard CIDR {c:?} is invalid: {e}"))
            })
            .collect()
    })
}

/// PURE predicate: is `ip` inside any forbidden egress range?
///
/// No DNS, no DB, no clock — only arithmetic over the static CIDR set.
/// IPv4-mapped IPv6 (`::ffff:127.0.0.1`) is handled by `IpCidr::contains`,
/// so the IPv4 ranges above also reject the mapped form.
pub fn is_forbidden_egress_ip(ip: IpAddr) -> bool {
    forbidden_egress_cidrs().iter().any(|c| c.contains(ip))
}

/// Why a webhook URL was refused before any bytes left the process.
///
/// This is the egress guard's house error type: a small, explicit,
/// `std::error::Error` enum (no `anyhow` string-stew) so the caller can
/// log a precise security event and so the unit tests can assert on the
/// variant rather than a fuzzy message.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WebhookEgressError {
    /// URL did not parse / was not absolute.
    Unparseable,
    /// Scheme was not `https`.
    SchemeNotHttps(String),
    /// URL had no host component.
    MissingHost,
    /// DNS resolution failed → fail closed (we never "allow on error").
    ResolutionFailed,
    /// Host literal-IP or a resolved IP fell in a forbidden range.
    ForbiddenTarget(IpAddr),
}

impl std::fmt::Display for WebhookEgressError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Unparseable => write!(f, "webhook URL is not a valid absolute URL"),
            Self::SchemeNotHttps(s) => {
                write!(f, "webhook URL scheme {s:?} rejected (https-only egress)")
            }
            Self::MissingHost => write!(f, "webhook URL has no host"),
            Self::ResolutionFailed => {
                write!(f, "webhook host DNS resolution failed (fail-closed)")
            }
            Self::ForbiddenTarget(ip) => {
                write!(
                    f,
                    "webhook host resolves to forbidden internal address {ip}"
                )
            }
        }
    }
}

impl std::error::Error for WebhookEgressError {}

/// Guard an admin-supplied webhook URL: https-only + no internal targets.
///
/// On success returns the parsed [`reqwest::Url`] (callers should send to
/// the validated value, not re-parse the raw string). Fail-closed: any
/// parse / scheme / host / resolution / range problem is an `Err` and the
/// request is NOT sent.
///
/// Pre-DNS literal-IP check first (so `https://127.0.0.1` is rejected with
/// no resolver round-trip and no rebinding window), then resolve the host
/// and reject if *any* returned address is forbidden.
pub fn validate_webhook_url(raw: &str) -> std::result::Result<reqwest::Url, WebhookEgressError> {
    // reqwest re-exports the `url` crate's parser — no new dependency.
    let url = reqwest::Url::parse(raw).map_err(|_| WebhookEgressError::Unparseable)?;

    if url.scheme() != "https" {
        return Err(WebhookEgressError::SchemeNotHttps(url.scheme().to_string()));
    }

    let host_raw = url.host_str().ok_or(WebhookEgressError::MissingHost)?;

    // The `url` crate returns IPv6 *literals* bracketed (`[::1]`) from
    // `host_str()`, bare for IPv4 / domains. A domain can never contain
    // `[`/`]` (invalid host bytes — the parser would have rejected the
    // URL), so stripping a single matching bracket pair is unambiguous
    // and yields the bare host string both the literal-IP parse and the
    // std resolver expect. (reqwest does not re-export `url::Host`, and
    // adding the `url` crate just for the typed enum would be a new
    // dependency for no security gain — this is exact, not heuristic.)
    let host = host_raw
        .strip_prefix('[')
        .and_then(|h| h.strip_suffix(']'))
        .unwrap_or(host_raw);

    // (3) Pre-DNS literal-IP short-circuit. If the (bracket-stripped)
    // host parses as an IP we decide purely on that value — no DNS at
    // all, so an unreachable resolver can never turn `https://[::1]/`
    // or `https://169.254.169.254/` into a soft "ResolutionFailed"; it
    // is a hard, deterministic ForbiddenTarget. No rebinding window.
    if let Ok(literal) = host.parse::<IpAddr>() {
        if is_forbidden_egress_ip(literal) {
            return Err(WebhookEgressError::ForbiddenTarget(literal));
        }
        // A public literal IP is allowed (skip resolution — nothing to do).
        return Ok(url);
    }

    // (4) Resolve the name to EVERY address and reject if ANY is forbidden.
    // Port is irrelevant to the IP decision; use the URL's effective port
    // (or 443) just to satisfy ToSocketAddrs' "host:port" contract.
    let port = url.port_or_known_default().unwrap_or(443);
    let addrs = (host, port)
        .to_socket_addrs()
        .map_err(|_| WebhookEgressError::ResolutionFailed)?;

    let mut saw_any = false;
    for sa in addrs {
        saw_any = true;
        let ip = sa.ip();
        if is_forbidden_egress_ip(ip) {
            return Err(WebhookEgressError::ForbiddenTarget(ip));
        }
    }
    if !saw_any {
        // Resolver returned zero addresses → nothing safe to send to.
        return Err(WebhookEgressError::ResolutionFailed);
    }

    Ok(url)
}

/// Webhook delivery status
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DeliveryStatus {
    Pending,
    Delivered,
    Failed,
    Retrying,
}

impl DeliveryStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Pending => "pending",
            Self::Delivered => "delivered",
            Self::Failed => "failed",
            Self::Retrying => "retrying",
        }
    }
}

/// Webhook delivery record
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct WebhookDelivery {
    pub id: Uuid,
    pub webhook_id: Uuid,
    pub event_type: String,
    pub content_id: Option<Uuid>,
    pub payload: JsonValue,
    pub status: String,
    pub attempts: i32,
    pub response_status: Option<i32>,
    pub response_body: Option<String>,
    pub response_time_ms: Option<i32>,
    pub error_message: Option<String>,
    pub created_at: DateTime<Utc>,
    pub delivered_at: Option<DateTime<Utc>>,
    pub next_retry_at: Option<DateTime<Utc>>,
}

/// Webhook configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebhookConfig {
    pub id: Uuid,
    pub name: String,
    pub url: String,
    pub secret: Option<String>,
    pub events: Vec<String>,
    pub headers: Option<JsonValue>,
    pub retry_count: i32,
    pub timeout_seconds: i32,
}

/// Webhook delivery result
#[derive(Debug, Serialize)]
pub struct DeliveryResult {
    pub success: bool,
    pub status_code: Option<i32>,
    pub response_body: Option<String>,
    pub response_time_ms: u64,
    pub error: Option<String>,
}

/// Generate HMAC-SHA256 signature for webhook payload
pub fn generate_signature(payload: &str, secret: &str) -> String {
    let mut mac = HmacSha256::new_from_slice(secret.as_bytes())
        .expect("HMAC-SHA256 accepts any key length per RFC 2104");
    mac.update(payload.as_bytes());
    let result = mac.finalize();
    hex::encode(result.into_bytes())
}

/// Create webhook payload
pub fn create_webhook_payload(
    event_type: &str,
    content_id: Option<Uuid>,
    data: JsonValue,
) -> JsonValue {
    json!({
        "event": event_type,
        "timestamp": Utc::now().to_rfc3339(),
        "contentId": content_id,
        "data": data
    })
}

/// Deliver webhook with signature
pub async fn deliver_webhook(
    client: &Client,
    config: &WebhookConfig,
    payload: &JsonValue,
) -> DeliveryResult {
    let start = std::time::Instant::now();
    let payload_str = serde_json::to_string(payload).unwrap_or_default();

    // ── P2-E (FULL_REPO_AUDIT_2026-05-17): SSRF egress guard ──
    // Validate the admin-stored target BEFORE constructing/sending the
    // request. Fail-closed: a rejected URL yields a failed DeliveryResult
    // (same shape the network-error arm produces), so the existing
    // retry/backoff/stat bookkeeping in `process_pending_deliveries`
    // records it without special-casing — and crucially, no bytes leave
    // the process toward an internal address.
    let validated_url = match validate_webhook_url(&config.url) {
        Ok(u) => u,
        Err(e) => {
            tracing::warn!(
                target: "security",
                event = "webhook_ssrf_blocked",
                webhook_id = %config.id,
                webhook_name = %config.name,
                reason = %e,
                "🛡️  SSRF guard refused webhook delivery to a non-https / internal target"
            );
            return DeliveryResult {
                success: false,
                status_code: None,
                response_body: None,
                response_time_ms: start.elapsed().as_millis() as u64,
                error: Some(format!("blocked by SSRF egress guard: {e}")),
            };
        }
    };

    // Build request — send to the *validated* URL, not the raw string.
    let mut request = client
        .post(validated_url)
        .header("Content-Type", "application/json")
        .header("User-Agent", "RevolutionTradingPros-CMS/2.0")
        .header(
            "X-Webhook-Event",
            payload
                .get("event")
                .and_then(|e| e.as_str())
                .unwrap_or("unknown"),
        );

    // Add signature if secret is configured
    if let Some(secret) = &config.secret {
        let signature = generate_signature(&payload_str, secret);
        request = request
            .header("X-Webhook-Signature", format!("sha256={signature}"))
            .header("X-Webhook-Timestamp", Utc::now().timestamp().to_string());
    }

    // Add custom headers if configured
    if let Some(headers) = &config.headers {
        if let Some(headers_obj) = headers.as_object() {
            for (key, value) in headers_obj {
                if let Some(val_str) = value.as_str() {
                    request = request.header(key.as_str(), val_str);
                }
            }
        }
    }

    // Set timeout
    request = request.timeout(std::time::Duration::from_secs(
        config.timeout_seconds as u64,
    ));

    // Execute request
    match request.body(payload_str).send().await {
        Ok(response) => {
            let status = response.status().as_u16() as i32;
            let elapsed = start.elapsed().as_millis() as u64;

            let body = response.text().await.ok();
            let success = (200..300).contains(&(status as u16));

            DeliveryResult {
                success,
                status_code: Some(status),
                response_body: body,
                response_time_ms: elapsed,
                error: if success {
                    None
                } else {
                    Some(format!("HTTP {status}"))
                },
            }
        }
        Err(e) => {
            let elapsed = start.elapsed().as_millis() as u64;
            DeliveryResult {
                success: false,
                status_code: None,
                response_body: None,
                response_time_ms: elapsed,
                error: Some(e.to_string()),
            }
        }
    }
}

/// Process pending webhook deliveries
pub async fn process_pending_deliveries(
    pool: &PgPool,
    client: &Client,
    batch_size: i32,
) -> Result<i32> {
    let mut processed = 0;

    // Get pending deliveries
    let deliveries: Vec<(Uuid, Uuid, JsonValue, i32)> = sqlx::query_as(
        r"
        SELECT d.id, d.webhook_id, d.payload, d.attempts
        FROM cms_webhook_deliveries d
        WHERE d.status = 'pending'
           OR (d.status = 'retrying' AND d.next_retry_at <= NOW())
        ORDER BY d.created_at
        LIMIT $1
        FOR UPDATE SKIP LOCKED
        ",
    )
    .bind(batch_size)
    .fetch_all(pool)
    .await?;

    for (delivery_id, webhook_id, payload, attempts) in deliveries {
        // Get webhook config
        let webhook: Option<WebhookConfig> = sqlx::query_as!(
            WebhookConfig,
            r#"
            SELECT id, name, url, secret, events, headers, retry_count, timeout_seconds
            FROM cms_webhooks
            WHERE id = $1 AND is_active = true
            "#,
            webhook_id
        )
        .fetch_optional(pool)
        .await?;

        let Some(config) = webhook else {
            // Webhook disabled or deleted, mark as failed
            sqlx::query(
                "UPDATE cms_webhook_deliveries SET status = 'failed', error_message = 'Webhook disabled' WHERE id = $1",
            )
            .bind(delivery_id)
            .execute(pool)
            .await?;
            continue;
        };

        // Deliver webhook
        let result = deliver_webhook(client, &config, &payload).await;

        // Update delivery record
        let new_status = if result.success {
            "delivered"
        } else if attempts + 1 >= config.retry_count {
            "failed"
        } else {
            "retrying"
        };

        // Calculate next retry time with exponential backoff
        let next_retry = if new_status == "retrying" {
            let delay_seconds = 2_i64.pow((attempts + 1) as u32).min(3600);
            Some(Utc::now() + Duration::seconds(delay_seconds))
        } else {
            None
        };

        sqlx::query(
            r"
            UPDATE cms_webhook_deliveries
            SET status = $1,
                attempts = attempts + 1,
                response_status = $2,
                response_body = $3,
                response_time_ms = $4,
                error_message = $5,
                delivered_at = CASE WHEN $1 = 'delivered' THEN NOW() ELSE NULL END,
                next_retry_at = $6
            WHERE id = $7
            ",
        )
        .bind(new_status)
        .bind(result.status_code)
        .bind(&result.response_body)
        .bind(result.response_time_ms as i32)
        .bind(&result.error)
        .bind(next_retry)
        .bind(delivery_id)
        .execute(pool)
        .await?;

        // Update webhook stats
        if result.success {
            sqlx::query(
                "UPDATE cms_webhooks SET success_count = success_count + 1, last_triggered_at = NOW() WHERE id = $1",
            )
            .bind(webhook_id)
            .execute(pool)
            .await?;
        } else {
            sqlx::query("UPDATE cms_webhooks SET failure_count = failure_count + 1 WHERE id = $1")
                .bind(webhook_id)
                .execute(pool)
                .await?;
        }

        processed += 1;
    }

    Ok(processed)
}

/// Queue a webhook delivery for an event
pub async fn queue_webhook_event(
    pool: &PgPool,
    event_type: &str,
    content_id: Option<Uuid>,
    data: JsonValue,
) -> Result<i32> {
    let payload = create_webhook_payload(event_type, content_id, data);

    // Use database function to queue webhooks
    let count: i32 = sqlx::query_scalar("SELECT cms_queue_webhook_delivery($1, $2, $3)")
        .bind(event_type)
        .bind(content_id)
        .bind(&payload)
        .fetch_one(pool)
        .await?;

    Ok(count)
}

/// Get delivery history for a webhook
pub async fn get_webhook_deliveries(
    pool: &PgPool,
    webhook_id: Uuid,
    limit: i64,
    offset: i64,
) -> Result<Vec<WebhookDelivery>> {
    let deliveries: Vec<WebhookDelivery> = sqlx::query_as!(
        WebhookDelivery,
        r#"
        SELECT id, webhook_id, event_type, content_id, payload,
               status::text as "status!", attempts, response_status, response_body,
               response_time_ms, error_message, created_at,
               delivered_at, next_retry_at
        FROM cms_webhook_deliveries
        WHERE webhook_id = $1
        ORDER BY created_at DESC
        LIMIT $2 OFFSET $3
        "#,
        webhook_id,
        limit,
        offset
    )
    .fetch_all(pool)
    .await?;

    Ok(deliveries)
}

/// Retry a failed delivery
pub async fn retry_delivery(pool: &PgPool, delivery_id: Uuid) -> Result<()> {
    sqlx::query(
        r"
        UPDATE cms_webhook_deliveries
        SET status = 'pending',
            next_retry_at = NULL,
            error_message = NULL
        WHERE id = $1 AND status IN ('failed', 'retrying')
        ",
    )
    .bind(delivery_id)
    .execute(pool)
    .await?;

    Ok(())
}

// ─────────────────────────────────────────────────────────────────────────────
// P2-E (FULL_REPO_AUDIT_2026-05-17): SSRF egress guard — in-source unit tests.
//
// Pure-IP logic is exercised exhaustively here with NO network and NO DB.
// `validate_webhook_url` cases that need DNS (a real public name resolving
// safely) are intentionally NOT asserted on a live resolver — that would be
// flaky and is out of scope for a deterministic unit test; the scheme check,
// the pre-DNS literal-IP path (both forbidden and allowed), and the parse
// failure path are fully covered without touching the network.
// ─────────────────────────────────────────────────────────────────────────────
#[cfg(test)]
mod tests {
    use super::*;
    use std::net::{Ipv4Addr, Ipv6Addr};

    fn v4(s: &str) -> IpAddr {
        IpAddr::V4(s.parse::<Ipv4Addr>().unwrap())
    }
    fn v6(s: &str) -> IpAddr {
        IpAddr::V6(s.parse::<Ipv6Addr>().unwrap())
    }

    // ── The pure predicate: forbidden ranges are rejected ──

    #[test]
    fn cloud_metadata_169_254_169_254_is_forbidden() {
        // The whole reason this fix exists: AWS/GCP/Azure/DO IMDS.
        assert!(is_forbidden_egress_ip(v4("169.254.169.254")));
        // Rest of the link-local /16 too.
        assert!(is_forbidden_egress_ip(v4("169.254.0.1")));
    }

    #[test]
    fn ipv4_loopback_is_forbidden() {
        assert!(is_forbidden_egress_ip(v4("127.0.0.1")));
        assert!(is_forbidden_egress_ip(v4("127.255.255.255")));
    }

    #[test]
    fn ipv6_loopback_is_forbidden() {
        assert!(is_forbidden_egress_ip(v6("::1")));
    }

    #[test]
    fn rfc1918_ranges_are_forbidden() {
        assert!(is_forbidden_egress_ip(v4("10.0.0.1"))); // 10/8
        assert!(is_forbidden_egress_ip(v4("10.255.255.255")));
        assert!(is_forbidden_egress_ip(v4("172.16.0.1"))); // 172.16/12
        assert!(is_forbidden_egress_ip(v4("172.31.255.255")));
        assert!(is_forbidden_egress_ip(v4("192.168.0.1"))); // 192.168/16
        assert!(is_forbidden_egress_ip(v4("192.168.255.255")));
    }

    #[test]
    fn ipv6_unique_local_fc00_is_forbidden() {
        assert!(is_forbidden_egress_ip(v6("fc00::1")));
        assert!(is_forbidden_egress_ip(v6("fd00::1"))); // fc00::/7 covers fd00::
    }

    #[test]
    fn unspecified_addresses_are_forbidden() {
        assert!(is_forbidden_egress_ip(v4("0.0.0.0")));
        assert!(is_forbidden_egress_ip(v6("::")));
    }

    #[test]
    fn link_local_v6_is_forbidden() {
        assert!(is_forbidden_egress_ip(v6("fe80::1")));
    }

    #[test]
    fn ipv4_mapped_loopback_is_forbidden() {
        // ::ffff:127.0.0.1 — the dual-stack representation of IPv4 loopback.
        // IpCidr::contains() unwraps the mapping, so the 127/8 entry catches it.
        assert!(is_forbidden_egress_ip(v6("::ffff:127.0.0.1")));
        // ::ffff:169.254.169.254 — mapped cloud metadata.
        assert!(is_forbidden_egress_ip(v6("::ffff:169.254.169.254")));
    }

    #[test]
    fn normal_public_ip_is_allowed() {
        // 1.1.1.1 (Cloudflare), 8.8.8.8 (Google), 203.0.113.7 (TEST-NET-3),
        // and a public IPv6 — none of these are in a forbidden range.
        assert!(!is_forbidden_egress_ip(v4("1.1.1.1")));
        assert!(!is_forbidden_egress_ip(v4("8.8.8.8")));
        assert!(!is_forbidden_egress_ip(v4("203.0.113.7")));
        assert!(!is_forbidden_egress_ip(v6("2606:4700:4700::1111")));
    }

    // ── The URL guard: scheme + pre-DNS literal-IP path (no network) ──

    #[test]
    fn http_scheme_is_rejected_https_required() {
        let err = validate_webhook_url("http://example.com/hook").unwrap_err();
        assert!(matches!(err, WebhookEgressError::SchemeNotHttps(s) if s == "http"));
    }

    #[test]
    fn non_http_schemes_are_rejected() {
        assert!(matches!(
            validate_webhook_url("file:///etc/passwd").unwrap_err(),
            WebhookEgressError::SchemeNotHttps(_)
        ));
        assert!(matches!(
            validate_webhook_url("gopher://127.0.0.1/").unwrap_err(),
            WebhookEgressError::SchemeNotHttps(_)
        ));
    }

    #[test]
    fn https_literal_loopback_is_rejected_pre_dns() {
        // No resolver touched — the host parses as an IP literal.
        let err = validate_webhook_url("https://127.0.0.1/hook").unwrap_err();
        assert_eq!(err, WebhookEgressError::ForbiddenTarget(v4("127.0.0.1")));
    }

    #[test]
    fn https_literal_cloud_metadata_is_rejected_pre_dns() {
        let err = validate_webhook_url("https://169.254.169.254/latest/meta-data/").unwrap_err();
        assert_eq!(
            err,
            WebhookEgressError::ForbiddenTarget(v4("169.254.169.254"))
        );
    }

    #[test]
    fn https_bracketed_ipv6_loopback_is_rejected_pre_dns() {
        let err = validate_webhook_url("https://[::1]/hook").unwrap_err();
        assert_eq!(err, WebhookEgressError::ForbiddenTarget(v6("::1")));
    }

    #[test]
    fn https_public_literal_ip_is_allowed_no_dns() {
        // Public literal → allowed, and resolution is skipped entirely.
        let url = validate_webhook_url("https://1.1.1.1/hook").unwrap();
        assert_eq!(url.scheme(), "https");
        assert_eq!(url.host_str(), Some("1.1.1.1"));
    }

    #[test]
    fn garbage_url_is_unparseable() {
        assert_eq!(
            validate_webhook_url("not a url").unwrap_err(),
            WebhookEgressError::Unparseable
        );
        // Scheme-relative / no host also fails parse as absolute URL.
        assert_eq!(
            validate_webhook_url("https://").unwrap_err(),
            WebhookEgressError::Unparseable
        );
    }
}
