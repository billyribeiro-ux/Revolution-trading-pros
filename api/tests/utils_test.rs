//! Auth/crypto utilities — exercises the REAL `revolution_api::utils`.
//!
//! These tests previously ran against a hand-duplicated `mod utils`
//! whose `verify_jwt` used `Validation::default()` and a `Claims`
//! struct with no `token_type`. The shipped verifier pins HS256 and
//! enforces access/refresh segregation, so a regression that weakened
//! either (e.g. reverting to `Validation::default()`) would have
//! passed the old tests. They now bind to the production functions and
//! assert the real security invariants.

use revolution_api::utils;

// ── Password hashing (Argon2id) ─────────────────────────────────────

#[test]
fn hash_is_argon2_format_and_salted() {
    let h1 = utils::hash_password("SecurePassword123!").unwrap();
    let h2 = utils::hash_password("SecurePassword123!").unwrap();
    assert!(h1.starts_with("$argon2"), "expected Argon2 PHC string");
    assert_ne!(h1, h2, "per-hash salt must make digests unique");
}

#[test]
fn verify_password_accepts_correct_and_rejects_wrong() {
    let hash = utils::hash_password("CorrectHorse1!").unwrap();
    assert!(utils::verify_password("CorrectHorse1!", &hash).unwrap());
    assert!(!utils::verify_password("WrongHorse1!", &hash).unwrap());
}

#[test]
fn verify_password_errors_on_malformed_hash() {
    assert!(utils::verify_password("whatever", "not-a-phc-string").is_err());
}

#[test]
fn unicode_password_roundtrips() {
    let pw = "密码🔐パスワード-9!";
    let hash = utils::hash_password(pw).unwrap();
    assert!(utils::verify_password(pw, &hash).unwrap());
}

// ── Password policy ─────────────────────────────────────────────────

#[test]
fn password_policy_enforces_owasp_rules() {
    // Too short (<12).
    assert!(utils::validate_password("Short1!").is_err());
    // Missing a special character.
    assert!(utils::validate_password("NoSpecialChar123abc").is_err());
    // Contains a known weak pattern even though it is otherwise complex.
    assert!(utils::validate_password("MyPassword123!").is_err());
    // Satisfies length + upper + lower + digit + special, no weak word.
    assert!(utils::validate_password("SecureTrading#2026").is_ok());
}

// ── JWT: type segregation + algorithm pinning ───────────────────────

const SECRET: &str = "test_secret_key_minimum_32_chars_xxxxx";

#[test]
fn access_token_roundtrips_and_preserves_i64_subject() {
    // 2^53 + 1 — would lose precision if ever coerced through f64.
    let uid: i64 = 9_007_199_254_740_993;
    let token = utils::create_jwt(uid, SECRET, 24).unwrap();
    assert_eq!(token.split('.').count(), 3, "JWS compact form");
    let claims = utils::verify_jwt(&token, SECRET, "access").unwrap();
    assert_eq!(claims.sub, uid);
    assert_eq!(claims.token_type, "access");
}

#[test]
fn refresh_token_roundtrips_as_refresh() {
    let token = utils::create_refresh_token(42, SECRET).unwrap();
    let claims = utils::verify_jwt(&token, SECRET, "refresh").unwrap();
    assert_eq!(claims.sub, 42);
    assert_eq!(claims.token_type, "refresh");
}

/// Security invariant: an access token MUST NOT satisfy a refresh-token
/// check and vice versa. The old duplicated verifier had no concept of
/// token type, so this whole class was untested.
#[test]
fn token_type_segregation_is_enforced() {
    let access = utils::create_jwt(7, SECRET, 1).unwrap();
    let refresh = utils::create_refresh_token(7, SECRET).unwrap();
    assert!(
        utils::verify_jwt(&access, SECRET, "refresh").is_err(),
        "access token accepted where refresh required"
    );
    assert!(
        utils::verify_jwt(&refresh, SECRET, "access").is_err(),
        "refresh token accepted where access required"
    );
}

/// Security invariant: HS256 is pinned. A forged `alg:none` token (the
/// classic algorithm-confusion attack) must be rejected. This is the
/// 10-year regression guard against anyone reverting to
/// `Validation::default()`.
#[test]
fn alg_none_forgery_is_rejected() {
    use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine};

    let now = chrono::Utc::now().timestamp();
    let header = URL_SAFE_NO_PAD.encode(br#"{"alg":"none","typ":"JWT"}"#);
    let claims = URL_SAFE_NO_PAD.encode(
        format!(
            r#"{{"sub":1,"exp":{},"iat":{},"token_type":"access"}}"#,
            now + 3600,
            now
        )
        .as_bytes(),
    );
    // alg:none → empty signature segment.
    let forged = format!("{header}.{claims}.");
    assert!(
        utils::verify_jwt(&forged, SECRET, "access").is_err(),
        "alg:none token must never verify under HS256 pinning"
    );
}

#[test]
fn wrong_secret_fails_verification() {
    let token = utils::create_jwt(1, "correct-secret-correct-secret-xx", 24).unwrap();
    assert!(utils::verify_jwt(&token, "another-secret-another-secret-xx", "access").is_err());
}

#[test]
fn expired_token_fails_verification() {
    let token = utils::create_jwt(1, SECRET, -1).unwrap();
    assert!(utils::verify_jwt(&token, SECRET, "access").is_err());
}

#[test]
fn tampered_or_malformed_token_fails_verification() {
    assert!(utils::verify_jwt("not.a.jwt", SECRET, "access").is_err());
    let mut token = utils::create_jwt(1, SECRET, 1).unwrap();
    token.push('x'); // corrupt the signature segment
    assert!(utils::verify_jwt(&token, SECRET, "access").is_err());
}

// ── Token / session generators ──────────────────────────────────────

#[test]
fn generate_token_is_32_alphanumeric_and_unique() {
    let a = utils::generate_token();
    let b = utils::generate_token();
    assert_eq!(a.len(), 32);
    assert!(a.chars().all(|c| c.is_ascii_alphanumeric()));
    assert_ne!(a, b);
}

#[test]
fn session_id_is_256_bit_hex() {
    let s = utils::generate_session_id();
    assert_eq!(s.len(), 64, "32 bytes hex-encoded");
    assert!(s.chars().all(|c| c.is_ascii_hexdigit()));
}

#[test]
fn password_reset_token_is_64_chars_unique() {
    let a = utils::generate_password_reset_token();
    assert_eq!(a.len(), 64);
    assert_ne!(a, utils::generate_password_reset_token());
}

#[test]
fn generators_are_collision_free_over_a_batch() {
    let n = 200;
    let set: std::collections::HashSet<String> = (0..n).map(|_| utils::generate_token()).collect();
    assert_eq!(set.len(), n, "no collisions across {n} tokens");
}

// ── Constant-time compare + token hashing ───────────────────────────

#[test]
fn constant_time_compare_matches_only_identical_strings() {
    assert!(utils::constant_time_compare("abc123", "abc123"));
    assert!(!utils::constant_time_compare("abc123", "abc124"));
    assert!(!utils::constant_time_compare("abc", "abcd")); // length mismatch
}

#[test]
fn hash_token_is_deterministic_and_not_identity() {
    let h1 = utils::hash_token("opaque-token");
    let h2 = utils::hash_token("opaque-token");
    assert_eq!(h1, h2, "same input → same digest (DB lookup key)");
    assert_ne!(h1, "opaque-token", "must store a hash, not the token");
    assert_ne!(h1, utils::hash_token("different-token"));
}
