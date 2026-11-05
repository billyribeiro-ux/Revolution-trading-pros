//! Login handler — POST /api/auth/login.
//!
//! Extracted from `auth.rs` during R12-B as a pure structural move.

use crate::services::{account_authority, account_mfa};
use axum::{
    Json,
    extract::State,
    http::{HeaderMap, StatusCode},
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use validator::Validate;

use super::helpers::{ClientAddr, client_ip, enforce_ip_rate_limit_strict};
use crate::{
    AppState,
    // FIX-2026-04-26 (Priority 2): pull in ValidatedJson extractor for hot-path handlers.
    middleware::validated_json::ValidatedJson,
    models::{AuthResponse, LoginUser},
    utils::{
        create_jwt_authenticated, create_refresh_token_authenticated, generate_session_id,
        hash_dummy_password, hash_password_async, verify_password_async,
    },
};

/// Login user
/// POST /api/auth/login
/// ICT L11+ Security: Rate limiting, timing attack prevention, session management
// FIX-2026-04-26 (Priority 2): migrated Json<LoginUser> -> ValidatedJson<LoginUser>.
// FIX-2026-04-26 (Priority 4): added per-IP rate limit (10 / 15min) ALONGSIDE existing
// per-email rate limit. Per-email alone is bypassable by an attacker rotating emails;
// per-IP catches that pattern. Both rate gates fail closed when Redis is unavailable.
// Original signature: async fn login(State(state): State<AppState>, Json(input): Json<LoginUser>)
#[derive(Serialize)]
#[serde(untagged)]
pub(super) enum LoginOutcome {
    Authenticated(Box<AuthResponse>),
    MfaRequired {
        mfa_required: bool,
        message: &'static str,
    },
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub(super) struct MfaLoginInput {
    email: String,
    password: String,
    mfa_code: Option<String>,
    backup_code: Option<String>,
    remember: Option<bool>,
    device_name: Option<String>,
    device_fingerprint: Option<String>,
}
enum OwnedProof {
    Totp(String),
    Backup(String),
}
pub(super) async fn login(
    state: State<AppState>,
    peer: ClientAddr,
    headers: HeaderMap,
    input: ValidatedJson<LoginUser>,
) -> Result<Json<LoginOutcome>, (StatusCode, Json<serde_json::Value>)> {
    run(state, peer, headers, input, None).await
}
pub(super) async fn login_mfa(
    state: State<AppState>,
    peer: ClientAddr,
    headers: HeaderMap,
    Json(input): Json<MfaLoginInput>,
) -> Result<Json<LoginOutcome>, (StatusCode, Json<serde_json::Value>)> {
    if peer.0.is_none() {
        return Err((
            StatusCode::SERVICE_UNAVAILABLE,
            Json(json!({"error":"The client address is unavailable for this security operation"})),
        ));
    }
    if input.device_name.as_ref().is_some_and(|v| v.len() > 200)
        || input
            .device_fingerprint
            .as_ref()
            .is_some_and(|v| v.len() > 128)
    {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(json!({"error":"Device information exceeds its limit"})),
        ));
    }
    let proof = match (input.mfa_code, input.backup_code) {
        (Some(code), None) if code.len() == 6 && code.bytes().all(|b| b.is_ascii_digit()) => {
            OwnedProof::Totp(code)
        }
        (None, Some(code)) if code.len() <= 80 && !code.trim().is_empty() => {
            OwnedProof::Backup(code)
        }
        _ => {
            return Err((
                StatusCode::BAD_REQUEST,
                Json(json!({"error":"Provide exactly one bounded authenticator or recovery code"})),
            ));
        }
    };
    let login = LoginUser {
        email: input.email,
        password: input.password,
        remember: input.remember,
        device_name: input.device_name,
        device_fingerprint: input.device_fingerprint,
    };
    login.validate().map_err(|_| {
        (
            StatusCode::BAD_REQUEST,
            Json(json!({"error":"Provide valid bounded login credentials"})),
        )
    })?;
    run(state, peer, headers, ValidatedJson(login), Some(proof)).await
}

async fn run(
    State(state): State<AppState>,
    peer: ClientAddr,
    headers: HeaderMap,
    ValidatedJson(input): ValidatedJson<LoginUser>,
    proof: Option<OwnedProof>,
) -> Result<Json<LoginOutcome>, (StatusCode, Json<serde_json::Value>)> {
    // Security audit logging
    tracing::info!(
        target: "security",
        event = "login_attempt",
        "Login attempt initiated"
    );

    // FIX-2026-04-27 (H-2): Both per-IP and per-email checks now FAIL CLOSED.
    // A Redis outage must not become a free pass for brute-force attacks.
    // P1-3 (2026-05-17): IP is now spoof-resistant (trusted-proxy aware), so
    // the per-IP bucket can no longer be sidestepped by rotating XFF.
    let ip = client_ip(peer.0, &headers, &state.config.trusted_proxy_cidrs);
    enforce_ip_rate_limit_strict(&state, &ip, "login_ip", 10, 900).await?;

    let Some(ref redis) = state.services.redis else {
        tracing::error!(
            target: "security",
            event = "login_rate_limit_redis_unavailable",
            "Redis unavailable for login rate limit check - rejecting request (fail-closed)"
        );
        return Err((
            StatusCode::SERVICE_UNAVAILABLE,
            Json(json!({"error": "Service temporarily unavailable, please try again"})),
        ));
    };

    let email_limit = tokio::time::timeout(
        std::time::Duration::from_secs(3),
        redis.check_login_rate_limit(&input.email),
    )
    .await
    .map_err(|_| {
        (
            StatusCode::SERVICE_UNAVAILABLE,
            Json(json!({"error":"Service temporarily unavailable, please try again"})),
        )
    })?;
    match email_limit {
        Ok(rate_limit) if !rate_limit.allowed => {
            let error_msg = if rate_limit.locked {
                "Account temporarily locked due to too many failed attempts"
            } else {
                "Too many login attempts. Please wait before trying again"
            };
            return Err((
                StatusCode::TOO_MANY_REQUESTS,
                Json(json!({
                    "error": error_msg,
                    "retry_after": rate_limit.retry_after,
                    "locked": rate_limit.locked
                })),
            ));
        }
        Ok(_) => {}
        Err(e) => {
            tracing::error!(
                target: "security",
                event = "login_email_rate_limit_failed_closed",
                error = %e,
                "Per-email rate limit Redis check failed - rejecting request (fail-closed)"
            );
            return Err((
                StatusCode::SERVICE_UNAVAILABLE,
                Json(json!({"error": "Service temporarily unavailable, please try again"})),
            ));
        }
    }

    // Find user
    let user_result: Option<serde_json::Value> = sqlx::query_scalar(
        "SELECT to_jsonb(account) FROM (SELECT id, email, password_hash, name, role, is_active, email_verified_at, avatar_url, mfa_enabled, created_at, updated_at FROM users WHERE email = $1 AND deleted_at IS NULL AND COALESCE(LOWER(role),'')<>'deleted') account"
    )
        .bind(&input.email)
        .fetch_optional(&state.db.pool)
        .await
        .map_err(|e| {
            tracing::error!("Database error during login: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": "Database error"})),
            )
        })?;

    // ICT L11+ Security: Timing attack prevention
    // Always perform password hashing even if user doesn't exist
    // This prevents timing-based user enumeration
    let mut user = match user_result {
        Some(value) => crate::services::account_authority::checked_user(value)
            .map_err(|(status, message)| (status, Json(json!({"error":message}))))?,
        None => {
            // Hash dummy password to match timing of real verification
            hash_dummy_password().await;
            tracing::info!(
                target: "security",
                event = "login_failed",
                reason = "user_not_found",
                "Login failed - user not found (timing protected)"
            );
            return Err((
                StatusCode::UNAUTHORIZED,
                Json(json!({"error": "Invalid credentials"})),
            ));
        }
    };

    // ICT 7 SECURITY: Password verification - NO password data logged
    // Apple Principal Engineer Grade: Never log credentials, hashes, or password metadata

    // Verify password (this happens regardless of user existence due to above)
    let password_valid = verify_password_async(&input.password, &user.password_hash)
        .await
        .map_err(|e| {
            tracing::error!("Password verification error: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": "Authentication error"})),
            )
        })?;

    if !password_valid {
        // Record failed attempt for rate limiting
        if let Some(redis) = &state.services.redis
            && let Err(e) = redis.record_failed_login(&input.email).await
        {
            // Brute-force counter silently not incrementing = rate limit off.
            tracing::error!(target: "security", error = %e, "Failed to record failed login attempt");
        }

        tracing::info!(
            target: "security",
            event = "login_failed",
            reason = "invalid_password",
            user_id = %user.id,
            "Login failed - invalid password"
        );
        return Err((
            StatusCode::UNAUTHORIZED,
            Json(json!({"error": "Invalid credentials"})),
        ));
    }

    // Clear failed login attempts on successful authentication
    if let Some(redis) = &state.services.redis
        && let Err(e) = redis.clear_login_attempts(&input.email).await
    {
        tracing::warn!(target: "security", error = %e, "Failed to clear login attempts after successful login");
    }

    // FIX-M-4 (2026-04-29): silently re-hash bcrypt (Laravel legacy) into
    // argon2id on every successful login. Bcrypt at default cost is still
    // acceptable, but argon2id is the project standard (utils/mod.rs:151)
    // and we should not maintain two hash schemes indefinitely.
    //
    // verify_password() (utils/mod.rs:180) recognizes hashes by prefix:
    //   $2y$ / $2b$ / $2a$  -> bcrypt
    //   $argon2             -> argon2id
    // If the stored hash is bcrypt, we already verified it — re-hash the
    // plaintext we have in `input.password` and overwrite the column.
    // Best-effort: a failure here logs but does not fail the login; the
    // user keeps a bcrypt hash and we'll try again on their next login.
    if user.password_hash.starts_with("$2") {
        let new_hash = hash_password_async(&input.password).await.map_err(|_| {
            (
                StatusCode::SERVICE_UNAVAILABLE,
                Json(json!({"error":"Password migration is unavailable"})),
            )
        })?;
        let saved=sqlx::query("UPDATE users SET password_hash=$1,updated_at=NOW() AT TIME ZONE 'UTC' WHERE id=$2 AND password_hash=$3 AND deleted_at IS NULL")
            .bind(&new_hash).bind(user.id).bind(&user.password_hash).execute(&state.db.pool).await
            .map_err(|_|(StatusCode::SERVICE_UNAVAILABLE,Json(json!({"error":"Password migration is unavailable"}))))?.rows_affected();
        if saved != 1 {
            return Err((
                StatusCode::UNAUTHORIZED,
                Json(json!({"error":"Account credentials changed; authenticate again"})),
            ));
        }
        user.password_hash = new_hash;
    }

    // FIX-2026-04-27 (C-1): Block banned/deactivated accounts at login.
    // is_active == Some(false) means an admin explicitly banned this account.
    // NULL (legacy rows) and Some(true) are both treated as active.
    //
    // SECURITY (FULL_REPO_AUDIT_2026-05-17 P1-2) — COMPANION NOW IMPLEMENTED.
    // The ban *state transition* lives in `routes/admin.rs::ban_user`; the
    // required `redis.bump_token_version(id)` call has been added there,
    // immediately next to the existing `redis.invalidate_all_user_sessions(id)`
    // call, using the same best-effort/log-on-Err pattern as `logout_all`
    // and `reset_password` below. That bump strands every prior
    // access/refresh JWT for a just-banned user the instant the extractor
    // re-reads the epoch — the C-1 `is_active` guard below + the extractor
    // only catch the *next DB-backed* fetch, not a token served from the
    // Redis user-cache hot path. This guard itself is a login gate (no live
    // token exists at this point), so no bump belongs here.
    if user.is_active == Some(false) {
        tracing::warn!(
            target: "security_audit",
            event = "login_blocked_banned",
            user_id = %user.id,
            "Login blocked - account is banned/deactivated"
        );
        return Err((
            StatusCode::UNAUTHORIZED,
            Json(json!({"error": "Invalid credentials"})),
        ));
    }

    // ICT 11+ ENTERPRISE SECURITY: Email Verification Enforcement
    // Apple Principal Engineer Grade - Zero Trust Security Model
    //
    // P2-H / security-M1 (FULL_REPO_AUDIT_2026-05-17): close the
    // email-verification bypass AND the email-list-as-identity elevation
    // hole. ROOT CAUSE: the previous code OR'd `is_developer_email` /
    // `is_superadmin_email` (a configured env email-list string) straight
    // into `bypass_verification`. That meant ANYONE who controlled (or could
    // get their address onto) the configured `DEVELOPER_EMAILS` /
    // `SUPERADMIN_EMAILS` list could log in WITHOUT ever verifying that email
    // and be treated as a privileged role — an env list string was being
    // accepted as proof of identity.
    //
    // FIX: email-list membership is at most a *secondary hint* layered on
    // top of a verified, DB-role'd account. The privileged bypass now
    // requires BOTH (a) the user row has `email_verified_at` set, AND (b)
    // the user carries a real DB privileged role (developer / super_admin /
    // super-admin / admin) — the env list can only *confirm* an elevation
    // the database already grants, never *create* one or skip verification.
    // `is_*_email_strict` enforces (verified ∧ DB-role ∧ on-list); the
    // explicit DB-role checks below cover a verified DB-role'd account that
    // is NOT on any env list (legitimate verified-admin login preserved).
    let email_verified = user.email_verified_at.is_some();
    let db_role = user.role.as_deref();
    let has_db_developer_role = db_role.is_some_and(|r| r.eq_ignore_ascii_case("developer"));
    let has_db_superadmin_role = crate::middleware::admin::is_superadmin_role(db_role);

    let is_developer = email_verified
        && (has_db_developer_role
            || state
                .config
                .is_developer_email_strict(&user.email, db_role, email_verified));

    let is_superadmin = email_verified
        && (has_db_superadmin_role
            || state
                .config
                .is_superadmin_email_strict(&user.email, db_role, email_verified));

    // ICT 11+ SECURITY: Strict verification enforcement.
    // A privileged bypass now requires a VERIFIED account with a real DB
    // privileged role — controlling a configured email is never sufficient
    // on its own. All other users MUST verify email.
    let bypass_verification = is_developer || is_superadmin;

    // P2-H / security-M1 (FULL_REPO_AUDIT_2026-05-17): the old code logged an
    // "email_verification_bypassed" warning here for `bypass_verification &&
    // email_verified_at.is_none()`. Under the hardened derivation above,
    // `bypass_verification` IMPLIES `email_verified` (a privileged bypass now
    // requires a verified, DB-role'd account), so an unverified privileged
    // bypass is no longer reachable — that warning would be dead code that
    // misrepresents the (now-closed) invariant. We instead audit the real,
    // legitimate event: a verified privileged account authenticating.
    if bypass_verification {
        let role_type = if is_developer {
            "developer"
        } else {
            "superadmin"
        };
        tracing::info!(
            target: "security_audit",
            event = "privileged_login",
            user_id = %user.id,
            role = %role_type,
            email_verified = true,
            "ICT 11+ AUDIT: Verified privileged account login (email-list is a secondary hint only, never a standalone gate)"
        );
    }

    // ICT 11+ SECURITY GATE: Block unverified users
    if user.email_verified_at.is_none() && !bypass_verification {
        tracing::warn!(
            target: "security",
            event = "login_blocked_unverified",
            user_id = %user.id,
            attempt_timestamp = %chrono::Utc::now().to_rfc3339(),
            "ICT 11+ SECURITY: Login blocked - email not verified"
        );
        return Err((
            StatusCode::FORBIDDEN,
            Json(json!({
                "error": "Please verify your email before logging in. Check your inbox for the verification link.",
                "code": "EMAIL_NOT_VERIFIED",
                "email": user.email,
                "help": "Didn't receive the email? Contact support or check your spam folder.",
                "security_level": "ICT_11_ENFORCED"
            })),
        ));
    }

    // P2-H / security-M1 (FULL_REPO_AUDIT_2026-05-17): the second
    // "privileged_verification_bypass" log (previously here, also gated on
    // `bypass_verification && email_verified_at.is_none()`) is likewise
    // unreachable under the hardened invariant and has been removed. The
    // single `privileged_login` audit above is the accurate, reachable event.

    // Mint against the current PostgreSQL account epoch, never an optional Redis snapshot.
    let mut account = crate::services::account_authority::current(&state.db.pool, user.id)
        .await
        .map_err(|(status, message)| (status, Json(json!({"error":message}))))?;
    if account.user.password_hash != user.password_hash || account.user.email_verified_at.is_none()
    {
        return Err((
            StatusCode::UNAUTHORIZED,
            Json(json!({"error":"Account credentials changed; authenticate again"})),
        ));
    }
    let mfa_required = account_authority::require_mfa_state(&account)
        .map_err(|(status, message)| (status, Json(json!({"error":message}))))?;
    let mfa_assured = if mfa_required {
        let Some(proof) = proof else {
            return Ok(Json(LoginOutcome::MfaRequired {
                mfa_required: true,
                message: "An authenticator or recovery code is required; no access tokens were issued",
            }));
        };
        let proof = match &proof {
            OwnedProof::Totp(code) => account_mfa::Proof::Totp(code),
            OwnedProof::Backup(code) => account_mfa::Proof::Backup(code),
        };
        let observed = ip.parse().map_err(|_| {
            (
                StatusCode::SERVICE_UNAVAILABLE,
                Json(json!({"error":"The client address is unavailable"})),
            )
        })?;
        account = account_mfa::verify_login(
            &state,
            account.user.id,
            account.version,
            proof,
            Some(observed),
        )
        .await
        .map_err(|error| {
            (
                StatusCode::from_bytes(error.status.as_bytes())
                    .unwrap_or(StatusCode::INTERNAL_SERVER_ERROR),
                Json(json!({"error":error.message})),
            )
        })?;
        true
    } else {
        if proof.is_some() {
            return Err((
                StatusCode::BAD_REQUEST,
                Json(json!({"error":"MFA is not enabled; use normal password sign-in"})),
            ));
        }
        false
    };
    let token_version = account.version;
    let user = account.user;

    // Create access token
    let token = create_jwt_authenticated(
        user.id,
        &state.config.jwt_secret,
        state.config.jwt_expires_in,
        token_version,
        mfa_assured,
    )
    .map_err(|e| {
        tracing::error!("JWT creation error: {}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": "Token creation failed"})),
        )
    })?;

    // Create refresh token
    let refresh_token = create_refresh_token_authenticated(
        user.id,
        &state.config.jwt_secret,
        token_version,
        mfa_assured,
    )
    .map_err(|e| {
        tracing::error!("Refresh token creation error: {}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": "Refresh token creation failed"})),
        )
    })?;

    // Generate session ID and store in Redis
    let session_id = generate_session_id();

    // ICT L11+ Security: Create server-side session in Redis
    if let Some(redis) = &state.services.redis
        && let Err(e) = redis
            .create_session(&session_id, user.id, &user.email, None, None)
            .await
    {
        tracing::warn!("Failed to create session in Redis: {}", e);
    }

    // Security audit: successful login
    tracing::info!(
        target: "security",
        event = "login_success",
        user_id = %user.id,
        session_id = %session_id,
        "Login successful"
    );

    Ok(Json(LoginOutcome::Authenticated(Box::new(AuthResponse {
        token: token.clone(),
        access_token: token, // Frontend prefers access_token
        refresh_token,
        session_id,
        user: crate::models::UserResponse::from_authenticated(user, &state.config),
        expires_in: state.config.jwt_expires_in * 3600, // Convert hours to seconds
    }))))
}
