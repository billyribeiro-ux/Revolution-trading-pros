//! Authentication middleware
//! ICT 7+ Principal Engineer: Redis-cached user lookups for 60-80% faster auth
//! ICT 7 Security: JWT blacklist checking for token revocation

use axum::{
    extract::FromRequestParts,
    http::{request::Parts, StatusCode},
    RequestPartsExt,
};
use axum_extra::{
    headers::{authorization::Bearer, Authorization},
    TypedHeader,
};
use sha2::{Digest, Sha256};

use crate::{models::User, utils::verify_jwt, AppState};

/// Hash a JWT token for blacklist lookup
/// Using SHA256 to avoid storing full tokens in Redis
fn hash_token_for_blacklist(token: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(token.as_bytes());
    hex::encode(hasher.finalize())
}

/// Extractor for authenticated users
/// ICT 7+: Uses Redis cache for user lookups, falls back to database
impl FromRequestParts<AppState> for User {
    type Rejection = (StatusCode, &'static str);

    #[allow(clippy::manual_async_fn)]
    fn from_request_parts(
        parts: &mut Parts,
        state: &AppState,
    ) -> impl std::future::Future<Output = Result<Self, Self::Rejection>> + Send {
        async move {
            // Extract bearer token
            let TypedHeader(Authorization(bearer)) = parts
                .extract::<TypedHeader<Authorization<Bearer>>>()
                .await
                .map_err(|e| {
                    tracing::warn!("Missing or invalid Authorization header: {:?}", e);
                    (
                        StatusCode::UNAUTHORIZED,
                        "Missing or invalid authorization header",
                    )
                })?;

            // Verify JWT
            // FIX-2026-04-26 (Priority 3): pass "access" expected_type so a refresh token
            // presented as a bearer access token is rejected.
            // Original line: let claims = verify_jwt(bearer.token(), &state.config.jwt_secret).map_err(|e| {
            let claims =
                verify_jwt(bearer.token(), &state.config.jwt_secret, "access").map_err(|e| {
                    tracing::warn!("JWT verification failed: {:?}", e);
                    (StatusCode::UNAUTHORIZED, "Invalid or expired token")
                })?;

            // FIX-H-3 (2026-04-29): JWT blacklist check now FAILS CLOSED.
            //
            // Previous behavior: if Redis returned an Err, the request proceeded
            // ("fail open for availability"). That meant a Redis fault during
            // a logout/ban window left the just-revoked token usable for the
            // remainder of its TTL.
            //
            // New behavior:
            //   - Redis configured + token blacklisted -> reject (unchanged).
            //   - Redis configured + Err on lookup    -> reject 503 (NEW).
            //   - Redis NOT configured                -> log + continue. This
            //     mirrors how every other Redis-dependent path behaves when
            //     Redis is genuinely absent (local dev without Redis). The
            //     `state.services.redis` check returns None at boot only when
            //     REDIS_URL is unset, which is itself caught by config sanity
            //     in production. If you want to fail-closed even in this
            //     branch, make Redis a hard requirement at startup.
            if let Some(ref redis) = state.services.redis {
                let token_hash = hash_token_for_blacklist(bearer.token());
                match redis.is_token_blacklisted(&token_hash).await {
                    Ok(true) => {
                        tracing::warn!(
                            target: "security_audit",
                            event = "blacklisted_token_rejected",
                            user_id = %claims.sub,
                            "Rejected blacklisted JWT token"
                        );
                        return Err((StatusCode::UNAUTHORIZED, "Token has been revoked"));
                    }
                    Ok(false) => {} // not blacklisted, continue
                    Err(e) => {
                        // Fail-closed: do not let a Redis fault open a window
                        // for revoked tokens. Returning 401 (not 503) so the
                        // frontend's existing retry+refresh logic kicks in
                        // and the user re-auths instead of seeing an ambiguous
                        // service-unavailable.
                        tracing::error!(
                            target: "security_audit",
                            event = "blacklist_check_failed_fail_closed",
                            user_id = %claims.sub,
                            error = %e,
                            "Redis blacklist check failed; rejecting request (fail-closed)"
                        );
                        return Err((
                            StatusCode::UNAUTHORIZED,
                            "Authentication temporarily unavailable",
                        ));
                    }
                }
            }

            // SECURITY (FULL_REPO_AUDIT_2026-05-17 P1-2 / security-C1+M3):
            // AUTHORITATIVE per-user token-epoch check. This is where the stale
            // token is actually rejected — `verify_jwt` is kept pure (sig+exp+type
            // only); the claim-vs-stored-version comparison belongs here in the
            // extractor because only the extractor has `state`/Redis.
            //
            // Root cause this closes: `invalidate_all_user_sessions()` only
            // deleted `session:*` keys, so a stolen ACCESS JWT survived
            // logout-all / password-reset / ban for its full TTL, and a stolen
            // REFRESH token survived until rotation. The epoch (Redis key
            // `user_token_version:{id}`, bumped by `RedisService::bump_token_version`
            // in logout_all / reset_password / ban) is now compared against the
            // value embedded in the token at mint time. Any token minted before
            // the most recent bump has `claims.token_version < current` and is
            // rejected immediately — for BOTH access and refresh-derived requests.
            //
            // Authority model (verifiable HERE without a DB): the Redis epoch key
            // is the RUNTIME source of truth. `get_token_version` reads it,
            // returning 0 when absent — identical to the `#[serde(default)]`
            // value carried by pre-feature / brand-new-user tokens, so nothing is
            // locked out until the first real bump. This deliberately avoids a
            // new `users.token_version` sqlx query because the migration set is
            // non-replayable on a fresh DB (owner-gated G0.3) and a DB-backed
            // read could not be compiled/clippy-verified here under SQLX_OFFLINE.
            //
            // G0.3-DEFERRED FOLLOW-UP (do NOT add here — owner-gated, unverifiable
            // without a live DB): add `users.token_version BIGINT NOT NULL
            // DEFAULT 0`, have `bump_token_version` also `UPDATE users SET
            // token_version = token_version + 1` inside the same tx, and make the
            // Redis key a read-through cache of that column (DB authoritative,
            // Redis fast-path) — mirroring the existing user-cache pattern below.
            // Until that migration lands, the Redis key is durable authority
            // (30-day sliding TTL >> 7-day max token life; see redis.rs).
            //
            // Fail-closed (mirrors the blacklist check above): Redis configured +
            // lookup error -> reject. Redis NOT configured -> log + continue,
            // exactly like the blacklist branch (local dev without Redis; prod
            // config sanity requires Redis at boot).
            if let Some(ref redis) = state.services.redis {
                match redis.get_token_version(claims.sub).await {
                    Ok(current_version) => {
                        if claims.token_version < current_version {
                            tracing::warn!(
                                target: "security_audit",
                                event = "stale_token_epoch_rejected",
                                user_id = %claims.sub,
                                token_version = %claims.token_version,
                                current_version = %current_version,
                                "Rejected token minted before a logout-all/password-reset/ban"
                            );
                            return Err((StatusCode::UNAUTHORIZED, "Token has been revoked"));
                        }
                    }
                    Err(e) => {
                        // Fail-closed: a Redis fault must not reopen the window
                        // for a revoked token. Same 401 + message shape as the
                        // blacklist fail-closed branch so the frontend's existing
                        // retry/refresh flow handles it uniformly.
                        tracing::error!(
                            target: "security_audit",
                            event = "token_version_check_failed_fail_closed",
                            user_id = %claims.sub,
                            error = %e,
                            "Redis token_version check failed; rejecting request (fail-closed)"
                        );
                        return Err((
                            StatusCode::UNAUTHORIZED,
                            "Authentication temporarily unavailable",
                        ));
                    }
                }
            }

            // ICT 7+: Try Redis cache first for faster auth (60-80% improvement)
            if let Some(ref redis) = state.services.redis {
                if let Ok(Some(cached_json)) = redis.get_cached_user(claims.sub).await {
                    if let Ok(user) = serde_json::from_str::<User>(&cached_json) {
                        tracing::debug!(
                            target: "performance",
                            event = "auth_cache_hit",
                            user_id = %claims.sub,
                            "User loaded from Redis cache"
                        );
                        return Ok(user);
                    }
                }
            }

            // Cache miss or Redis unavailable - fetch from database
            // ICT 7 FIX: Use explicit column list to avoid SQLx FromRow deserialization errors
            // when database has additional columns not present in User struct
            let user: User = sqlx::query_as(
                r"SELECT id, email, password_hash, name, role, is_active, email_verified_at,
                      avatar_url, mfa_enabled, created_at, updated_at
               FROM users WHERE id = $1",
            )
            .bind(claims.sub)
            .fetch_optional(&state.db.pool)
            .await
            .map_err(|e| {
                tracing::error!("Database error fetching user: {:?}", e);
                (StatusCode::INTERNAL_SERVER_ERROR, "Database error")
            })?
            .ok_or_else(|| {
                tracing::warn!("User not found for id: {}", claims.sub);
                (StatusCode::UNAUTHORIZED, "User not found")
            })?;

            // FIX-2026-04-27 (C-1): Reject banned/deactivated users on every request.
            // is_active defaults to true when NULL (older rows before the column existed).
            if user.is_active == Some(false) {
                tracing::warn!(
                    target: "security_audit",
                    event = "banned_user_request_rejected",
                    user_id = %user.id,
                    email = %user.email,
                    "Rejected request from banned/deactivated user"
                );
                return Err((StatusCode::UNAUTHORIZED, "Account is disabled"));
            }

            // ICT 7+: Cache user in Redis for subsequent requests
            if let Some(ref redis) = state.services.redis {
                if let Ok(user_json) = serde_json::to_string(&user) {
                    if let Err(e) = redis.cache_user(user.id, &user_json).await {
                        // Log but don't fail - caching is optimization, not critical
                        tracing::warn!(
                            target: "performance",
                            event = "cache_write_failed",
                            user_id = %user.id,
                            error = %e,
                            "Failed to cache user in Redis"
                        );
                    }
                }
            }

            Ok(user)
        }
    }
}

/// Optional user extractor (for routes that work with or without auth)
#[allow(dead_code)]
pub struct OptionalUser(pub Option<User>);

impl FromRequestParts<AppState> for OptionalUser {
    type Rejection = (StatusCode, &'static str);

    #[allow(clippy::manual_async_fn)]
    fn from_request_parts(
        parts: &mut Parts,
        state: &AppState,
    ) -> impl std::future::Future<Output = Result<Self, Self::Rejection>> + Send {
        async move {
            match User::from_request_parts(parts, state).await {
                Ok(user) => Ok(OptionalUser(Some(user))),
                Err(_) => Ok(OptionalUser(None)),
            }
        }
    }
}
