//! Shared auth helpers — client-IP resolution, per-IP rate limiting, and
//! token-epoch / token-hash plumbing. Extracted from the monolithic
//! `auth.rs` during R12-B as a pure structural move; every byte of logic
//! (including comments, log targets, and event names) is preserved.

use std::net::{IpAddr, SocketAddr};

use axum::{
    http::{HeaderMap, StatusCode},
    Json,
};
use serde_json::json;

use crate::{config::IpCidr, AppState};

/// P1-3 (FULL_REPO_AUDIT_2026-05-17): spoof-resistant client-IP resolver.
///
/// Replaces the old `client_ip(headers)` helper, which blindly trusted the
/// first hop of client-supplied `X-Forwarded-For` (then `X-Real-IP`). Because
/// every per-IP rate-limit bucket is keyed by the returned string, an attacker
/// could mint a brand-new bucket on every request just by rotating the XFF
/// header — defeating the register (5/15m), per-IP login (10/15m),
/// forgot-password (3/h) and reset-password (5/h) limiters.
///
/// Root cause: there was no notion of *which* proxy is allowed to assert a
/// forwarded client IP. The fix introduces an explicit trusted-proxy CIDR
/// allowlist (`Config::trusted_proxy_cidrs`, env `TRUSTED_PROXY_CIDRS`) and the
/// standard, well-understood algorithm:
///
/// 1. If the immediate TCP peer is **not** within a trusted-proxy CIDR, the
///    request did not arrive through our infrastructure: `X-Forwarded-For` /
///    `X-Real-IP` are attacker-controlled noise and are **ignored entirely**.
///    The real TCP peer is the client.
/// 2. If the peer **is** trusted, walk `X-Forwarded-For` **right-to-left**
///    (proxies *append* the address they saw, so the rightmost entries are the
///    ones closest to us and the most trustworthy) and return the rightmost
///    address that is **not** itself a trusted proxy. That is the first hop we
///    cannot vouch for — i.e. the real external client. This correctly handles
///    a chain of N trusted proxies.
/// 3. If every XFF entry is a trusted proxy (or XFF is absent/empty/malformed),
///    fall back to the trusted peer's own socket address.
///
/// With an empty allowlist (the default — `TRUSTED_PROXY_CIDRS` unset) NO peer
/// is ever trusted, so forwarded headers are always ignored and the socket
/// peer is authoritative. The legitimate single-proxy and direct-connection
/// cases are byte-for-byte unchanged; only header spoofing is closed.
pub fn resolve_client_ip(peer: IpAddr, headers: &HeaderMap, trusted: &[IpCidr]) -> IpAddr {
    // Step 1: is the immediate peer a proxy we explicitly trust?
    let peer_trusted = trusted.iter().any(|cidr| cidr.contains(peer));
    if !peer_trusted {
        // Untrusted (or no allowlist configured): forwarded headers are
        // adversary-controlled. Ignore them completely.
        return peer;
    }

    // Step 2: the peer is a trusted proxy. Reconstruct the forwarded chain
    // and find the rightmost non-trusted (i.e. real client) hop.
    //
    // RFC 7239 / de-facto XFF semantics: a proxy appends the address it
    // received the connection from. So the list reads, left→right,
    // [original client, proxy1, proxy2, ...]. Walking right→left and
    // skipping trusted proxies yields the genuine client even with several
    // chained load balancers.
    if let Some(xff) = headers.get("x-forwarded-for").and_then(|v| v.to_str().ok()) {
        for hop in xff.rsplit(',') {
            let hop = hop.trim();
            if hop.is_empty() {
                continue;
            }
            // XFF entries can carry a `:port` (and IPv6 may be bracketed).
            let Some(parsed) = parse_forwarded_hop(hop) else {
                // Malformed entry — cannot trust or attribute it. Skip and
                // keep walking; do NOT abort to the peer, since a later
                // (more leftward) entry might still be a valid client and a
                // garbage rightmost hop would otherwise mask it.
                continue;
            };
            if trusted.iter().any(|cidr| cidr.contains(parsed)) {
                // Another trusted proxy in the chain — keep walking left.
                continue;
            }
            // First non-trusted hop from the right: the real client.
            return parsed;
        }
    }

    // Step 3: XFF absent / empty / all-trusted / all-malformed. Fall back
    // to the trusted proxy's socket address. (We intentionally do NOT
    // consult `X-Real-IP` here: a trusted proxy that wants to assert the
    // client IP must do so via XFF, which is what every reverse proxy in
    // this stack emits. Honoring a second, independently-spoofable header
    // would re-open a parallel bypass.)
    peer
}

/// Parse one `X-Forwarded-For` element into an [`IpAddr`].
///
/// Tolerates the common real-world shapes:
///
/// - bare IPv4 / IPv6 (`203.0.113.7` / `2001:db8::1`)
/// - IPv4 with port (`203.0.113.7:54321`)
/// - bracketed IPv6, optionally with port (`[2001:db8::1]` / `[2001:db8::1]:443`)
///
/// Anything else returns `None` (the caller skips it).
fn parse_forwarded_hop(hop: &str) -> Option<IpAddr> {
    // Fast path: already a valid bare IP (covers unbracketed IPv6).
    if let Ok(ip) = hop.parse::<IpAddr>() {
        return Some(ip);
    }
    // Bracketed IPv6, optionally followed by `:port`.
    if let Some(rest) = hop.strip_prefix('[') {
        if let Some(end) = rest.find(']') {
            return rest[..end].parse::<IpAddr>().ok();
        }
        return None;
    }
    // `host:port` — only meaningful for IPv4 (an unbracketed IPv6 with a
    // port is ambiguous and non-conformant; reject it).
    if let Some((host, _port)) = hop.rsplit_once(':') {
        if let Ok(v4) = host.parse::<std::net::Ipv4Addr>() {
            return Some(IpAddr::V4(v4));
        }
    }
    None
}

/// Resolve the client IP for rate-limiting and return it as the `String`
/// bucket key the limiter expects. Thin adapter over [`resolve_client_ip`]
/// so the handler call sites stay terse.
///
/// `peer` is `Option` because the `ConnectInfo` extension is only present
/// when the server is launched via `into_make_service_with_connect_info`
/// (production — see `main.rs`). Tower's `oneshot`-driven integration test
/// harness does not install it; in that case we fall back to the
/// unspecified address `0.0.0.0`, which is **never** inside any
/// trusted-proxy CIDR — so the spoof-resistant branch still ignores
/// forwarded headers. Fail-safe: a missing peer can only make the limiter
/// *more* conservative (single shared bucket), never bypassable.
pub(super) fn client_ip(
    peer: Option<SocketAddr>,
    headers: &HeaderMap,
    trusted: &[IpCidr],
) -> String {
    let peer_ip = peer
        .map(|p| p.ip())
        .unwrap_or(IpAddr::V4(std::net::Ipv4Addr::UNSPECIFIED));
    resolve_client_ip(peer_ip, headers, trusted).to_string()
}

/// FIX-2026-04-26 (Priority 4): per-IP rate limit gate.
/// FAIL-CLOSED on Redis outage for register/forgot/reset (sensitive endpoints).
/// Returns Err if rate limit exceeded OR if Redis is unreachable.
pub(super) async fn enforce_ip_rate_limit_strict(
    state: &AppState,
    ip: &str,
    bucket_key: &str,
    max_requests: i64,
    window_seconds: u64,
) -> Result<(), (StatusCode, Json<serde_json::Value>)> {
    let Some(redis) = state.services.redis.as_ref() else {
        // No Redis configured at all — fail closed for these sensitive endpoints.
        tracing::error!(
            target: "security",
            event = "rate_limit_redis_unavailable_fail_closed",
            bucket = %bucket_key,
            ip = %ip,
            "Redis service not configured; rejecting sensitive request"
        );
        return Err((
            StatusCode::SERVICE_UNAVAILABLE,
            Json(json!({"error": "Service temporarily unavailable, please try again"})),
        ));
    };

    // Scope key by bucket + IP so register/forgot/reset are independently throttled.
    let key = format!("{bucket_key}:{ip}");
    match redis
        .check_ip_rate_limit(&key, max_requests, window_seconds)
        .await
    {
        Ok(result) => {
            if !result.allowed {
                tracing::warn!(
                    target: "security",
                    event = "ip_rate_limit_exceeded",
                    bucket = %bucket_key,
                    ip = %ip,
                    "IP rate limit exceeded for sensitive endpoint"
                );
                return Err((
                    StatusCode::TOO_MANY_REQUESTS,
                    Json(json!({
                        "error": "Too many requests, please try again later",
                        "retry_after": result.retry_after,
                    })),
                ));
            }
            Ok(())
        }
        Err(e) => {
            // FAIL-CLOSED: Redis transient failure must not become a free pass for abuse.
            tracing::error!(
                target: "security",
                event = "rate_limit_check_failed_fail_closed",
                bucket = %bucket_key,
                ip = %ip,
                error = %e,
                "Rate limit Redis check failed; rejecting request (fail-closed)"
            );
            Err((
                StatusCode::SERVICE_UNAVAILABLE,
                Json(json!({"error": "Service temporarily unavailable, please try again"})),
            ))
        }
    }
}

/// Hash a JWT token for blacklist storage
pub(super) fn hash_token_for_blacklist(token: &str) -> String {
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(token.as_bytes());
    hex::encode(hasher.finalize())
}

/// SECURITY (FULL_REPO_AUDIT_2026-05-17 P1-2): resolve the user's current
/// token epoch for embedding into a freshly minted access/refresh token.
///
/// Fail-closed contract (mirrors the `User` extractor and the existing
/// fail-closed Redis paths in this file): if Redis is configured but the
/// lookup errors we return 503 rather than minting a token at an unknown
/// epoch — a token minted at the wrong (stale) version would either be
/// instantly rejected by the extractor (annoying) or, worse, a guessed-too-
/// low value could mask a real revocation. If Redis is NOT configured the
/// epoch is 0, which is exactly what the extractor's "Redis absent -> skip
/// the epoch check" branch expects, so the two stay consistent.
pub(super) async fn current_token_version(
    state: &AppState,
    user_id: i64,
) -> Result<i64, (StatusCode, Json<serde_json::Value>)> {
    match state.services.redis.as_ref() {
        Some(redis) => redis.get_token_version(user_id).await.map_err(|e| {
            tracing::error!(
                target: "security_audit",
                event = "token_version_read_failed_fail_closed",
                user_id = %user_id,
                error = %e,
                "Could not read token epoch; refusing to mint token (fail-closed)"
            );
            (
                StatusCode::SERVICE_UNAVAILABLE,
                Json(json!({"error": "Service temporarily unavailable, please try again"})),
            )
        }),
        None => Ok(0),
    }
}
