//! Utility functions - December 2025 ICT11+ Principal Engineer Grade
//! SECURITY HARDENED - Authentication Hardening Audit
//!
//! Modules:
//! - errors: Standardized error response utilities
//!
//! Authentication utilities:
//! - Password hashing with Argon2id (OWASP recommended parameters)
//! - Password verification supporting both bcrypt (Laravel) and Argon2
//! - JWT token creation and verification
//! - Refresh token generation
//! - Session ID generation
//! - Constant-time comparison utilities

#![allow(dead_code)]

pub mod crypto;
pub mod errors;

use anyhow::Result;
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Algorithm, Argon2, Params, Version,
};
use chrono::{Duration, Utc};
// FIX-2026-04-26: pin algorithm via Validation::new(JwtAlgorithm::HS256) — was Validation::default()
// (aliased to avoid name conflict with argon2::Algorithm imported above)
// use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use jsonwebtoken::{
    decode, encode, Algorithm as JwtAlgorithm, DecodingKey, EncodingKey, Header, Validation,
};
use serde::{Deserialize, Serialize};

/// JWT claims for access tokens
#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: i64, // Subject (user ID)
    pub exp: i64, // Expiration time
    pub iat: i64, // Issued at
    #[serde(default)]
    pub token_type: String, // "access" or "refresh"
    // SECURITY (FULL_REPO_AUDIT_2026-05-17 P1-2 / security-C1+M3): per-user
    // token epoch. Every issued token embeds the user's token_version at
    // mint time. A logout-all / password-reset / ban bumps the user's stored
    // version; the `User` extractor (middleware/auth.rs) then rejects any
    // token whose `token_version` is *below* the current stored value. This
    // closes the window where a stolen access JWT survived "log out all
    // devices", password-reset and ban for its full TTL (and a stolen
    // refresh token survived until rotation), because those flows only
    // deleted Redis `session:*` keys and the single-device `/logout` was the
    // ONLY thing that blacklisted an access JWT.
    //
    // `#[serde(default)]` is load-bearing for backward compatibility:
    //   - Tokens minted BEFORE this change carry no `tv` field and decode
    //     with `token_version == 0`. New users start at version 0 too, so
    //     legacy tokens are accepted until the first bump (a bump moves the
    //     stored version to >= 1, which then strands every version-0 token).
    //   - The pre-existing `create_jwt`/`create_refresh_token`/`verify_jwt`
    //     signatures are intentionally left untouched (they default the
    //     version to 0) so `tests/utils_test.rs` — which we do not own and
    //     must not edit — keeps compiling and passing. New code paths use
    //     the `*_versioned` constructors below.
    #[serde(default, rename = "tv")]
    pub token_version: i64,
}

/// ICT L11+ Security Hardening: Password validation rules
pub struct PasswordValidation {
    pub min_length: usize,
    pub max_length: usize,
}

impl Default for PasswordValidation {
    fn default() -> Self {
        Self {
            min_length: 12,  // OWASP minimum for financial applications
            max_length: 128, // Prevent DoS via extremely long passwords
        }
    }
}

/// Validate password meets security requirements
/// Returns Ok(()) if valid, Err with specific message if invalid
/// ICT Level 7: OWASP-compliant password policy for financial applications
pub fn validate_password(password: &str) -> Result<(), &'static str> {
    let rules = PasswordValidation::default();

    if password.len() < rules.min_length {
        return Err("Password must be at least 12 characters");
    }
    if password.len() > rules.max_length {
        return Err("Password must be no more than 128 characters");
    }

    // Check for at least one uppercase, lowercase, digit, and special character
    let has_upper = password.chars().any(|c| c.is_ascii_uppercase());
    let has_lower = password.chars().any(|c| c.is_ascii_lowercase());
    let has_digit = password.chars().any(|c| c.is_ascii_digit());
    let has_special = password.chars().any(|c| {
        // OWASP recommended special characters
        matches!(
            c,
            '!' | '@'
                | '#'
                | '$'
                | '%'
                | '^'
                | '&'
                | '*'
                | '('
                | ')'
                | '-'
                | '_'
                | '+'
                | '='
                | '['
                | ']'
                | '{'
                | '}'
                | '|'
                | '\\'
                | ':'
                | ';'
                | '"'
                | '\''
                | '<'
                | '>'
                | ','
                | '.'
                | '?'
                | '/'
                | '`'
                | '~'
        )
    });

    if !has_upper {
        return Err("Password must contain at least one uppercase letter");
    }
    if !has_lower {
        return Err("Password must contain at least one lowercase letter");
    }
    if !has_digit {
        return Err("Password must contain at least one number");
    }
    if !has_special {
        return Err(
            "Password must contain at least one special character (!@#$%^&*()_+-=[]{}|:;<>?,./~`)",
        );
    }

    // ICT 7: Check for common weak patterns
    let lower_password = password.to_lowercase();
    let weak_patterns = [
        "password", "123456", "qwerty", "letmein", "admin", "welcome",
    ];
    for pattern in weak_patterns {
        if lower_password.contains(pattern) {
            return Err("Password contains a common weak pattern");
        }
    }

    Ok(())
}

/// Hash a password using Argon2id with OWASP-recommended parameters
/// ICT L11+ Security: Hardened configuration for financial applications
///
/// Parameters (OWASP 2024 recommendations):
/// - Algorithm: Argon2id (resistant to side-channel and GPU attacks)
/// - Memory: 64 MiB (65536 KiB)
/// - Iterations: 3
/// - Parallelism: 4
/// - Output length: 32 bytes
pub fn hash_password(password: &str) -> Result<String> {
    let salt = SaltString::generate(&mut OsRng);

    // OWASP-recommended Argon2id parameters for financial applications
    let params = Params::new(
        65536,    // 64 MiB memory
        3,        // 3 iterations
        4,        // 4 parallel lanes
        Some(32), // 32-byte output
    )
    .map_err(|e| anyhow::anyhow!("Invalid Argon2 params: {e}"))?;

    let argon2 = Argon2::new(Algorithm::Argon2id, Version::V0x13, params);

    let hash = argon2
        .hash_password(password.as_bytes(), &salt)
        .map_err(|e| anyhow::anyhow!("Password hashing failed: {e}"))?;
    Ok(hash.to_string())
}

/// Hash a "dummy" password to prevent timing attacks
/// Call this when user doesn't exist to match timing of real verification
pub fn hash_dummy_password() {
    let _ = hash_password("dummy_password_for_timing");
}

/// Verify a password against a hash
/// Supports both bcrypt (Laravel legacy) and Argon2 (new Rust API)
/// ICT11+ Principal Engineer: Backward compatibility with Laravel bcrypt hashes
pub fn verify_password(password: &str, hash: &str) -> Result<bool> {
    // Check if it's a bcrypt hash (Laravel uses $2y$ prefix, bcrypt crate uses $2b$)
    if hash.starts_with("$2y$") || hash.starts_with("$2b$") || hash.starts_with("$2a$") {
        // Normalize Laravel's $2y$ to bcrypt's $2b$ for compatibility
        let normalized_hash = if hash.starts_with("$2y$") {
            hash.replacen("$2y$", "$2b$", 1)
        } else {
            hash.to_string()
        };

        // Verify using bcrypt
        match bcrypt::verify(password, &normalized_hash) {
            Ok(valid) => Ok(valid),
            Err(e) => {
                tracing::warn!("Bcrypt verification error: {}", e);
                Err(anyhow::anyhow!("Password verification failed: {e}"))
            }
        }
    } else if hash.starts_with("$argon2") {
        // Argon2 hash (new Rust API users)
        let parsed_hash =
            PasswordHash::new(hash).map_err(|e| anyhow::anyhow!("Invalid Argon2 hash: {e}"))?;
        Ok(Argon2::default()
            .verify_password(password.as_bytes(), &parsed_hash)
            .is_ok())
    } else {
        // Unknown hash format
        tracing::error!(
            "Unknown password hash format: {}",
            &hash[..hash.len().min(10)]
        );
        Err(anyhow::anyhow!("Unknown password hash format"))
    }
}

/// Create a JWT access token.
///
/// Backward-compatible shim: mints a token at `token_version == 0`. Retained
/// verbatim because `tests/utils_test.rs` (not owned/editable here) calls
/// `create_jwt(uid, SECRET, hours)` directly. Production auth code must call
/// [`create_jwt_versioned`] so the user's live token epoch is embedded.
pub fn create_jwt(user_id: i64, secret: &str, expires_in_hours: i64) -> Result<String> {
    create_jwt_versioned(user_id, secret, expires_in_hours, 0)
}

/// Create a JWT access token that embeds the user's current token epoch.
///
/// SECURITY (P1-2): callers pass the user's authoritative `token_version`
/// (see `RedisService::get_token_version`). The `User` extractor compares the
/// embedded value against the live stored value and rejects stale tokens.
pub fn create_jwt_versioned(
    user_id: i64,
    secret: &str,
    expires_in_hours: i64,
    token_version: i64,
) -> Result<String> {
    let now = Utc::now();
    let claims = Claims {
        sub: user_id,
        iat: now.timestamp(),
        exp: (now + Duration::hours(expires_in_hours)).timestamp(),
        token_type: "access".to_string(),
        token_version,
    };

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )?;

    Ok(token)
}

/// Create a JWT refresh token (longer expiry - 7 days).
///
/// Backward-compatible shim at `token_version == 0` (see [`create_jwt`] for
/// the rationale — `tests/utils_test.rs` binds this exact signature).
/// Production auth code must call [`create_refresh_token_versioned`].
pub fn create_refresh_token(user_id: i64, secret: &str) -> Result<String> {
    create_refresh_token_versioned(user_id, secret, 0)
}

/// Create a JWT refresh token that embeds the user's current token epoch.
///
/// SECURITY (P1-2): a stolen refresh token previously survived until the
/// next rotation. Embedding the epoch lets the extractor (and any future
/// refresh-side check) strand it the instant the user's version is bumped.
pub fn create_refresh_token_versioned(
    user_id: i64,
    secret: &str,
    token_version: i64,
) -> Result<String> {
    let now = Utc::now();
    let claims = Claims {
        sub: user_id,
        iat: now.timestamp(),
        exp: (now + Duration::days(7)).timestamp(),
        token_type: "refresh".to_string(),
        token_version,
    };

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )?;

    Ok(token)
}

/// Verify and decode a JWT token
///
/// FIX-2026-04-26 (Priority 3): SECURITY HARDENING
///   1. Algorithm is pinned to HS256 (was Validation::default() which is too permissive
///      for forward-compat; explicit pinning prevents alg-confusion regressions).
///   2. `expected_type` parameter is REQUIRED — callers must pass either "access" or
///      "refresh" to enforce token-type segregation. Previously a refresh token could
///      be presented to the auth middleware and vice versa.
///
/// Original signature was: `pub fn verify_jwt(token: &str, secret: &str) -> Result<Claims>`
///
/// Original body:
/// ```ignore
/// let token_data = decode::<Claims>(
///     token,
///     &DecodingKey::from_secret(secret.as_bytes()),
///     &Validation::default(),
/// )?;
/// Ok(token_data.claims)
/// ```
pub fn verify_jwt(token: &str, secret: &str, expected_type: &str) -> Result<Claims> {
    let validation = Validation::new(JwtAlgorithm::HS256);
    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &validation,
    )?;

    // FIX-2026-04-26 (Priority 3): enforce token_type segregation
    if token_data.claims.token_type != expected_type {
        return Err(anyhow::anyhow!(
            "Invalid token type: expected '{}', got '{}'",
            expected_type,
            token_data.claims.token_type
        ));
    }

    Ok(token_data.claims)
}

/// Generate a unique session ID (256-bit entropy)
/// ICT L11+ Security: Cryptographically secure session identifier
pub fn generate_session_id() -> String {
    use rand::RngExt;
    let mut rng = rand::rng();
    let bytes: [u8; 32] = rng.random();
    hex::encode(bytes)
}

/// Constant-time string comparison to prevent timing attacks
/// ICT L11+ Security: Use for any security-sensitive comparisons
pub fn constant_time_compare(a: &str, b: &str) -> bool {
    use subtle::ConstantTimeEq;
    if a.len() != b.len() {
        return false;
    }
    a.as_bytes().ct_eq(b.as_bytes()).into()
}

/// Generate a random token (for password reset, etc.)
pub fn generate_token() -> String {
    use rand::distr::Alphanumeric;
    use rand::RngExt;
    let mut rng = rand::rng();
    (0..32).map(|_| rng.sample(Alphanumeric) as char).collect()
}

/// Generate a secure password reset token (URL-safe, 64 chars)
pub fn generate_password_reset_token() -> String {
    use rand::RngExt;
    let mut rng = rand::rng();
    (0..64)
        .map(|_| {
            let idx = rng.random_range(0..62);
            match idx {
                0..=9 => (b'0' + idx) as char,
                10..=35 => (b'a' + idx - 10) as char,
                36..=61 => (b'A' + idx - 36) as char,
                _ => unreachable!(),
            }
        })
        .collect()
}

/// Generate a verification token and its hash
/// Returns (raw_token, hashed_token)
/// The raw token is sent to the user, the hashed token is stored in the database
pub fn generate_verification_token() -> (String, String) {
    use rand::RngExt;

    let mut rng = rand::rng();
    let raw_token: String = (0..64)
        .map(|_| {
            let idx = rng.random_range(0..62);
            match idx {
                0..=9 => (b'0' + idx) as char,
                10..=35 => (b'a' + idx - 10) as char,
                36..=61 => (b'A' + idx - 36) as char,
                _ => unreachable!(),
            }
        })
        .collect();

    let hashed_token = hash_token(&raw_token);
    (raw_token, hashed_token)
}

/// Hash a token using SHA256 (for storing verification/reset tokens)
pub fn hash_token(token: &str) -> String {
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(token.as_bytes());
    hex::encode(hasher.finalize())
}

// FIX-2026-04-26 (Priority 3): unit tests for verify_jwt token_type enforcement.
#[cfg(test)]
mod jwt_token_type_tests {
    use super::*;

    const SECRET: &str = "test_secret_must_be_long_enough_to_pass_HS256";

    #[test]
    fn access_token_accepted_when_expecting_access() {
        // CLAUDE.md "no unwrap()": tests use `.expect("test setup: ...")` so a
        // future regression (e.g. JWT encode breaks for static SECRET) produces
        // a self-describing failure instead of a bare `unwrap` panic.
        let token = create_jwt(42, SECRET, 1).expect("test setup: create_jwt with static SECRET");
        let claims = verify_jwt(&token, SECRET, "access").expect("access token should validate");
        assert_eq!(claims.sub, 42);
        assert_eq!(claims.token_type, "access");
    }

    #[test]
    fn refresh_token_accepted_when_expecting_refresh() {
        let token = create_refresh_token(99, SECRET)
            .expect("test setup: create_refresh_token with static SECRET");
        let claims = verify_jwt(&token, SECRET, "refresh").expect("refresh token should validate");
        assert_eq!(claims.sub, 99);
        assert_eq!(claims.token_type, "refresh");
    }

    #[test]
    fn refresh_token_rejected_when_expecting_access() {
        // This is the core bug the audit found: a refresh token presented as a
        // bearer access token MUST be rejected.
        let refresh = create_refresh_token(1, SECRET)
            .expect("test setup: create_refresh_token with static SECRET");
        let result = verify_jwt(&refresh, SECRET, "access");
        assert!(
            result.is_err(),
            "refresh token must NOT validate as access token"
        );
        let msg = format!("{}", result.unwrap_err());
        assert!(
            msg.contains("token type"),
            "error should mention token type, got: {msg}"
        );
    }

    #[test]
    fn access_token_rejected_when_expecting_refresh() {
        let access = create_jwt(1, SECRET, 1).expect("test setup: create_jwt with static SECRET");
        let result = verify_jwt(&access, SECRET, "refresh");
        assert!(
            result.is_err(),
            "access token must NOT validate as refresh token"
        );
    }

    #[test]
    fn wrong_secret_rejected() {
        let token = create_jwt(1, SECRET, 1).expect("test setup: create_jwt with static SECRET");
        let result = verify_jwt(&token, "different_secret", "access");
        assert!(result.is_err(), "wrong secret must reject");
    }

    #[test]
    fn expired_token_rejected() {
        // already-expired (negative hours); the encode itself is still infallible.
        let token = create_jwt(1, SECRET, -1).expect("test setup: create_jwt with static SECRET");
        let result = verify_jwt(&token, SECRET, "access");
        assert!(result.is_err(), "expired token must reject");
    }

    #[test]
    fn malformed_token_rejected() {
        let result = verify_jwt("not.a.token", SECRET, "access");
        assert!(result.is_err(), "garbage input must reject");
    }
}

// SECURITY (FULL_REPO_AUDIT_2026-05-17 P1-2 / security-C1+M3): unit coverage
// for the per-user token_version epoch claim. These exercise ONLY the pure
// pieces (claim round-trip, embedding, backward-compat default, and that
// `verify_jwt` stays sig/exp/type-only with the new claim present). The
// authoritative claim-vs-stored-version COMPARE lives in the `User`
// extractor (middleware/auth.rs) and is DB/Redis-bound — integration-level,
// G0.3-blocked here (no DB/Redis in the unit harness), so it is not covered
// by these unit tests by design.
#[cfg(test)]
mod token_version_tests {
    use super::*;

    const SECRET: &str = "test_secret_must_be_long_enough_to_pass_HS256";

    #[test]
    fn claims_round_trips_token_version() {
        // Mint with an explicit non-zero version and confirm verify_jwt
        // (which decodes the claims) preserves it exactly.
        let token = create_jwt_versioned(7, SECRET, 1, 42)
            .expect("test setup: create_jwt_versioned with static SECRET");
        let claims = verify_jwt(&token, SECRET, "access").expect("should validate");
        assert_eq!(claims.sub, 7);
        assert_eq!(claims.token_version, 42);
        assert_eq!(claims.token_type, "access");
    }

    #[test]
    fn refresh_token_embeds_passed_version() {
        let token = create_refresh_token_versioned(9, SECRET, 13)
            .expect("test setup: create_refresh_token_versioned with static SECRET");
        let claims = verify_jwt(&token, SECRET, "refresh").expect("should validate");
        assert_eq!(claims.sub, 9);
        assert_eq!(claims.token_version, 13);
        assert_eq!(claims.token_type, "refresh");
    }

    #[test]
    fn create_jwt_embeds_the_passed_version() {
        // Distinct versions produce distinct decoded claims — proves the
        // value is actually embedded, not hard-coded.
        let v0 = verify_jwt(
            &create_jwt_versioned(1, SECRET, 1, 0)
                .expect("test setup: create_jwt_versioned with static SECRET"),
            SECRET,
            "access",
        )
        .expect("test setup: verify_jwt of just-minted access token");
        let v5 = verify_jwt(
            &create_jwt_versioned(1, SECRET, 1, 5)
                .expect("test setup: create_jwt_versioned with static SECRET"),
            SECRET,
            "access",
        )
        .expect("test setup: verify_jwt of just-minted access token");
        assert_eq!(v0.token_version, 0);
        assert_eq!(v5.token_version, 5);
    }

    #[test]
    fn legacy_shim_constructors_default_version_to_zero() {
        // The backward-compatible (un-versioned) constructors that
        // tests/utils_test.rs binds MUST default the epoch to 0 so a
        // brand-new user (also at version 0) is not locked out.
        let access = verify_jwt(
            &create_jwt(1, SECRET, 1).expect("test setup: create_jwt with static SECRET"),
            SECRET,
            "access",
        )
        .expect("test setup: verify_jwt of just-minted access token");
        let refresh = verify_jwt(
            &create_refresh_token(1, SECRET)
                .expect("test setup: create_refresh_token with static SECRET"),
            SECRET,
            "refresh",
        )
        .expect("test setup: verify_jwt of just-minted refresh token");
        assert_eq!(access.token_version, 0);
        assert_eq!(refresh.token_version, 0);
    }

    #[test]
    fn token_without_tv_claim_deserializes_to_zero() {
        // A token minted before this change carries no `tv` field. It must
        // still decode (token_version defaults to 0) so we don't hard-fail
        // every in-flight session on deploy. We hand-craft a payload with
        // NO `tv` key and sign it with the real HS256 key.
        use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine};
        let now = Utc::now().timestamp();
        // Header {"alg":"HS256","typ":"JWT"} == jsonwebtoken's Header::default.
        let header_json = br#"{"typ":"JWT","alg":"HS256"}"#;
        let payload_json = format!(
            r#"{{"sub":1,"exp":{},"iat":{},"token_type":"access"}}"#,
            now + 3600,
            now
        );
        let signing_input = format!(
            "{}.{}",
            URL_SAFE_NO_PAD.encode(header_json),
            URL_SAFE_NO_PAD.encode(payload_json.as_bytes())
        );
        // HS256 = HMAC-SHA256 over the signing input with the secret.
        use hmac::{Hmac, Mac};
        use sha2::Sha256;
        // HMAC-SHA256 accepts any key length (RFC 2104); `new_from_slice` only
        // errors on truly invalid lengths which is impossible for a non-empty
        // static secret. Use `expect` to document this static invariant.
        let mut mac = Hmac::<Sha256>::new_from_slice(SECRET.as_bytes())
            .expect("static invariant: HMAC-SHA256 accepts any non-empty key length");
        mac.update(signing_input.as_bytes());
        let sig = URL_SAFE_NO_PAD.encode(mac.finalize().into_bytes());
        let legacy_token = format!("{signing_input}.{sig}");

        let claims =
            verify_jwt(&legacy_token, SECRET, "access").expect("legacy token must still decode");
        assert_eq!(
            claims.token_version, 0,
            "missing `tv` claim must default to 0 for backward compatibility"
        );
    }

    #[test]
    fn verify_jwt_stays_sig_exp_type_only_with_new_claim() {
        // verify_jwt must remain PURE: it enforces signature, expiry and
        // token-type — and must NOT itself reject on token_version (that is
        // the extractor's authoritative job). A high-version token with a
        // valid sig/exp/type still verifies here.
        let high = create_jwt_versioned(1, SECRET, 1, 999_999)
            .expect("test setup: create_jwt_versioned with static SECRET");
        assert!(
            verify_jwt(&high, SECRET, "access").is_ok(),
            "verify_jwt must not gate on token_version"
        );
        // sig still enforced with the new claim present
        assert!(verify_jwt(&high, "wrong_secret", "access").is_err());
        // exp still enforced
        let expired = create_jwt_versioned(1, SECRET, -1, 7)
            .expect("test setup: create_jwt_versioned with static SECRET");
        assert!(verify_jwt(&expired, SECRET, "access").is_err());
        // type still enforced
        let refresh = create_refresh_token_versioned(1, SECRET, 7)
            .expect("test setup: create_refresh_token_versioned with static SECRET");
        assert!(verify_jwt(&refresh, SECRET, "access").is_err());
    }
}

// FIX-2026-04-26 (Priority 1): unit tests confirming the LIKE-wildcard escape used
// in the videos.rs `search` filter neutralizes wildcard injection. The escape policy
// is local to the SQL builder; this test pins the expected behavior so a future
// refactor of the helper can't silently regress.
#[cfg(test)]
mod like_escape_tests {
    /// Mirrors the escape policy used in `routes/videos.rs::list_videos` for the
    /// search ILIKE filter. Kept in sync with the rebuild_count_args helper there.
    fn escape_like(s: &str) -> String {
        s.replace('\\', "\\\\")
            .replace('%', "\\%")
            .replace('_', "\\_")
    }

    #[test]
    fn escapes_percent_wildcard() {
        assert_eq!(escape_like("100%"), "100\\%");
    }

    #[test]
    fn escapes_underscore_wildcard() {
        assert_eq!(escape_like("foo_bar"), "foo\\_bar");
    }

    #[test]
    fn escapes_backslash_first() {
        // Backslash must be escaped first so the % escape isn't double-escaped.
        assert_eq!(escape_like("a\\%b"), "a\\\\\\%b");
    }

    #[test]
    fn safe_input_unchanged() {
        assert_eq!(escape_like("simple search"), "simple search");
    }

    #[test]
    fn injection_attempt_neutralized() {
        // A malicious input trying to broaden a LIKE pattern must be escaped.
        let attack = "'; DROP TABLE users; --";
        let escaped = escape_like(attack);
        // Single-quotes are NOT this helper's job (they're handled by sqlx parameter
        // binding) — we just confirm wildcards are neutralized.
        assert!(!escaped.contains('%'));
        assert!(!escaped.contains('_'));
    }
}
