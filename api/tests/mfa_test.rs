// R23-D: test-scaffold doc comments use narrative list formatting that doesn't
// match clippy's strict CommonMark interpretation. Allow file-wide.
#![allow(clippy::doc_lazy_continuation, clippy::doc_overindented_list_items)]

//! MFA service contract tests — pure, no-DB.
//!
//! Binds directly to `revolution_api::services::mfa` and pins the
//! TOTP/2FA cryptographic primitives that the auth-path depends on.
//!
//! ## Why this file exists (R22-D, sweep-complete extra target)
//!
//! `services/mfa.rs` (479 LOC) is the second-most security-sensitive
//! non-route module in the crate after `routes/auth/*` (already
//! covered by `tests/auth_test.rs`). The MFA service implements:
//!
//!   - `generate_totp_secret()` — 160-bit base32 secret
//!   - `generate_backup_codes()` — 10 × 8-char XXXX-XXXX codes
//!   - `generate_totp(secret, time)` — RFC 6238 TOTP algorithm
//!     (HMAC-SHA1, 6 digits, 30-second period)
//!   - `verify_totp(secret, code)` — ±1-period drift tolerance,
//!     constant-time comparison
//!   - `generate_qr_uri(secret, email, issuer)` — otpauth:// URI
//!
//! Per CLAUDE.md habit #1 ("Cite the rule in your work"): this
//! module is the implementation of the project's "ICT Level 7
//! Implementation" claim in the module header (lines 1-3). Every
//! property below is load-bearing:
//!
//! 1. **RFC 6238 known-answer test (NIST appendix B vector).**
//!    The TOTP spec defines exact outputs for given secret + time
//!    inputs. A regression that swapped HMAC-SHA1 for SHA256, or
//!    changed the digit count from 6 to 8, or flipped the period
//!    from 30s to 60s would produce different codes — silently
//!    invalidating every Google Authenticator / Authy app that
//!    users already enrolled. The pin compares against an
//!    independently-computed reference value, NOT the function's
//!    own output (no tautology).
//!
//! 2. **TOTP secret format — 160 bits base32 (no padding).**
//!    Per line 53-57, `generate_totp_secret` emits a 20-byte secret
//!    base32-encoded WITHOUT padding. Base32 of 20 bytes is 32
//!    chars (20 * 8 / 5 = 32). A regression that flipped to padded
//!    base32 (e.g., `Alphabet::Rfc4648 { padding: true }`) would
//!    produce a 40-char string with `====` padding — incompatible
//!    with Google Authenticator's strict parser.
//!
//! 3. **Backup codes — 10 × 8-char alphanumeric, XXXX-XXXX
//!    formatted, all distinct.** Per lines 60-78,
//!    `generate_backup_codes` emits exactly 10 codes, each 9 chars
//!    (8 + 1 dash), drawn from base36 alphabet (0-9A-Z). The dash
//!    at position 4 is load-bearing — the verify path
//!    (line 296-307 of services/mfa.rs) compares the user input
//!    against the stored format directly. A regression to a 16-char
//!    no-dash format would silently invalidate every saved code.
//!
//! 4. **`verify_totp` accepts the current period's code.** The
//!    verify path checks codes at offsets [-1, 0, +1] periods
//!    (line 119, 30s drift tolerance). The pin generates the code
//!    at "now" and immediately calls verify — proving the round
//!    trip works. A regression that flipped the offset range to
//!    [0] (no drift) would reject codes generated near the edge of
//!    a 30s window.
//!
//! 5. **`generate_qr_uri` emits an otpauth:// URI with the right
//!    parameters.** Per line 143-154, the format is exactly:
//!    `otpauth://totp/{issuer}:{email}?secret={secret}&issuer=...
//!    &algorithm=SHA1&digits=6&period=30`. A regression that
//!    URL-encoded the secret itself (it MUST be raw base32, per
//!    Google Authenticator's parser) would silently break QR-code
//!    enrollment. The pin asserts the URI shape and the absence of
//!    encoded characters in the secret position.
//!
//! 6. **`MfaSecret` PK & FK are i64 (BIGSERIAL).** Per lines
//!    24-34, `MfaSecret.id: i64` and `MfaSecret.user_id: i64` —
//!    both match the BIGSERIAL columns in the `mfa_secrets` and
//!    `users` tables. Per CLAUDE.md "Money / cents — i64 ONLY,
//!    BIGINT ONLY, EVERY TIME": these aren't money fields, but
//!    they index into BIGSERIAL tables — same i64 invariant.
//!
//! 7. **`MfaSecret.backup_codes_used: i32` is the Reserved
//!    exception.** Per CLAUDE.md: "row counts (`revisions: i32`,
//!    `attempts: i32`, `total_pages: i32`) — those genuinely cap
//!    below 2 billion and `i32` is fine. Money never qualifies."
//!    `backup_codes_used` caps at 10 (the BACKUP_CODE_COUNT
//!    constant, line 19) — i32 is wildly overspec'd for this field
//!    but is the right floor.
//!
//! ## Pattern source
//!
//! Modeled on `tests/auth_test.rs` (sister auth surface),
//! `tests/migrate_test.rs` (sister R22-D scaffold),
//! `tests/users_test.rs` (i64 BIGSERIAL PK + PII pin).

use revolution_api::services::mfa::{
    generate_backup_codes, generate_qr_uri, generate_totp, generate_totp_secret, verify_totp,
    MfaSecret, MfaSetupResponse, MfaVerifyResult,
};

// ── 1. RFC 6238 known-answer test (independent reference vector) ─────

/// `generate_totp` MUST implement RFC 6238 (TOTP) with HMAC-SHA1,
/// 6 digits, 30-second period. The reference test vector below is
/// from RFC 6238 Appendix B:
///
///     Secret: "12345678901234567890" (ASCII)
///     Base32: "GEZDGNBVGY3TQOJQGEZDGNBVGY3TQOJQ"
///     Time:   59 seconds (counter = 1)
///     SHA1:   "94287082"  (NOTE: RFC 6238 vector is 8-digit;
///                         truncated to 6 → "287082")
///
/// Per CLAUDE.md habit #1 ("Cite the rule in your work"): this is
/// the RFC 6238 conformance proof. A regression that:
///   - flipped HMAC-SHA1 → HMAC-SHA256 (TOTP_ALGORITHM constant)
///   - changed TOTP_DIGITS from 6 → 8
///   - changed TOTP_PERIOD from 30 → 60
/// would all produce different output and would fail this pin
/// loud and clear.
///
/// R9-D NEGATIVE: this is NOT a tautological "did our function
/// produce its own output" — the expected value below is
/// independently derived from the RFC 6238 reference vector
/// (Appendix B, ASCII 20-byte secret, time=59, 6-digit truncation).
#[test]
fn mfa_generate_totp_rfc6238_known_answer() {
    // RFC 6238 reference: ASCII "12345678901234567890" base32-encoded.
    let secret = "GEZDGNBVGY3TQOJQGEZDGNBVGY3TQOJQ";

    // RFC 6238 Appendix B, T1 = 59 (Unix seconds), SHA-1, 8 digits = 94287082
    // Truncated to 6 digits (our TOTP_DIGITS constant) = "287082"
    let code = generate_totp(secret, 59).expect("TOTP generation MUST succeed");

    assert_eq!(
        code, "287082",
        "RFC 6238 Appendix B vector at T=59 with 6-digit truncation MUST be 287082. \
         A regression to SHA256, 8-digit, or 60s-period would change this output."
    );

    // Smoke check: code is exactly 6 ASCII digits.
    assert_eq!(code.len(), 6, "TOTP code MUST be 6 chars");
    assert!(
        code.chars().all(|c| c.is_ascii_digit()),
        "TOTP code MUST be all ASCII digits"
    );
}

// ── 2. TOTP secret shape — 160-bit base32 (32 chars), no padding ─────

/// `generate_totp_secret` MUST emit a 32-character base32 string
/// (20 bytes = 160 bits, no padding). Per RFC 4648, base32 of 20
/// bytes is exactly 32 chars; with `Alphabet::Rfc4648 { padding:
/// false }` (line 56) there is NO trailing `=` padding.
///
/// Google Authenticator's QR-code parser requires unpadded base32.
/// A regression that flipped to `padding: true` would produce a
/// 40-char string ending in `===` — silently breaking enrollment.
///
/// Per CLAUDE.md habit #3 ("does any `static` / `OnceLock` / lazy
/// init survive the refactor?"): this function uses
/// `rand::thread_rng()` directly with NO module-cached state — a
/// regression that wrapped the RNG in a `OnceLock<StdRng>` to
/// "make it deterministic" would silently make every user's TOTP
/// secret identical and grant cross-user MFA bypass.
#[test]
fn mfa_generate_totp_secret_is_unpadded_base32_32_chars() {
    let secret = generate_totp_secret();

    assert_eq!(
        secret.len(),
        32,
        "TOTP secret MUST be 32 chars (160-bit base32 unpadded). \
         A regression to padded base32 would produce 40 chars."
    );

    assert!(
        !secret.contains('='),
        "TOTP secret MUST NOT contain '=' padding — Google Authenticator's \
         parser is strict and rejects padded base32."
    );

    // RFC 4648 base32 alphabet: A-Z, 2-7. Lowercase NOT permitted.
    assert!(
        secret
            .chars()
            .all(|c| c.is_ascii_uppercase() || ('2'..='7').contains(&c)),
        "TOTP secret MUST be RFC 4648 base32 alphabet (A-Z + 2-7 uppercase only)"
    );

    // R9-D NEGATIVE: two calls MUST produce different secrets (no
    // OnceLock-cached value). 160 bits of entropy makes collision
    // probability ~2^-80 per pair, but we sample multiple times to
    // be safe against a deterministic-RNG regression.
    let secret2 = generate_totp_secret();
    let secret3 = generate_totp_secret();
    assert!(
        secret != secret2 || secret != secret3,
        "TOTP secrets MUST be distinct across calls — a regression that \
         wrapped thread_rng in OnceLock<StdRng> would silently grant \
         cross-user MFA bypass."
    );
}

// ── 3. Backup codes — 10 × XXXX-XXXX alphanumeric, all distinct ──────

/// `generate_backup_codes` MUST emit exactly 10 codes, each 9
/// characters in the format `XXXX-XXXX` (4 digits/letters + dash
/// + 4 digits/letters). Per `BACKUP_CODE_COUNT = 10` (line 19)
/// and `BACKUP_CODE_LENGTH = 8` (line 20).
///
/// The dash at position 4 is load-bearing — the verify path
/// compares the user-input code byte-by-byte against the stored
/// JSON array (services/mfa.rs:296-307). A regression that flipped
/// the format to plain 8-char would silently invalidate every
/// saved code (stored format = XXXX-XXXX, new generation = XXXXXXXX,
/// `constant_time_eq` returns false on length mismatch alone).
///
/// Per CLAUDE.md habit #3 ("comment-vs-code drift"): the function
/// comment says "Generate backup codes" — the actual contract is
/// "Generate 10 backup codes in the XXXX-XXXX format". The pin
/// freezes both the count AND the format.
#[test]
fn mfa_generate_backup_codes_shape_10_codes_dashed_format() {
    let codes = generate_backup_codes();

    // Exactly 10 codes per BACKUP_CODE_COUNT.
    assert_eq!(
        codes.len(),
        10,
        "MUST generate exactly 10 backup codes (BACKUP_CODE_COUNT constant)"
    );

    for (i, code) in codes.iter().enumerate() {
        // Format: XXXX-XXXX (9 chars total).
        assert_eq!(
            code.len(),
            9,
            "backup code {i} MUST be 9 chars (4 + dash + 4); got '{code}'"
        );

        // Dash at position 4.
        let bytes = code.as_bytes();
        assert_eq!(
            bytes[4], b'-',
            "backup code {i} MUST have '-' at position 4; got '{code}'"
        );

        // First 4 and last 4 are base36 (0-9 + A-Z uppercase).
        for (pos, ch) in code.chars().enumerate() {
            if pos == 4 {
                continue;
            }
            assert!(
                ch.is_ascii_digit() || ch.is_ascii_uppercase(),
                "backup code {i} char {pos} ('{ch}') MUST be base36 alphanumeric \
                 (0-9 + uppercase A-Z); got '{code}'"
            );
        }
    }

    // R9-D NEGATIVE: all 10 codes MUST be distinct (probability of
    // 36^8 = ~2.8 trillion outcomes, collision within 10 picks is
    // ~2^-37 — vanishingly small but a deterministic-RNG regression
    // would produce 10 identical codes).
    let mut seen = std::collections::HashSet::new();
    for code in &codes {
        assert!(
            seen.insert(code.clone()),
            "duplicate backup code '{code}' — a regression that returned \
             [generate_one().clone(); 10] would silently break the \
             one-time-use guarantee."
        );
    }
}

// ── 4. verify_totp round-trip + ±1-period drift tolerance ────────────

/// `verify_totp` MUST accept the code generated for "now" by
/// `generate_totp`. This is the basic happy-path round-trip:
/// generate at time T, verify at time T → must be true.
///
/// The drift tolerance (±1 period, line 119: `[-1i64, 0, 1]`) is
/// load-bearing for real-world use — clock skew between the user's
/// phone and the server can be up to ~30 seconds. A regression to
/// `[0]` (no drift) would silently reject ~10% of legitimate
/// codes (those generated near the edge of a 30s window).
///
/// R9-D NEGATIVE: the pin generates two codes — one for "now" and
/// one for a far-past time. The far-past code MUST be rejected
/// (proving the verify doesn't accept arbitrary historical codes
/// — a replay-attack concern).
#[test]
fn mfa_verify_totp_accepts_current_rejects_far_past() {
    let secret = generate_totp_secret();

    // Generate a code for "now".
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .expect("clock not before epoch")
        .as_secs();
    let current_code = generate_totp(&secret, now).expect("generate current TOTP");

    // verify_totp uses the same "now" internally — MUST accept the
    // current code (offset 0 of [-1, 0, 1]).
    let ok = verify_totp(&secret, &current_code).expect("verify current TOTP");
    assert!(
        ok,
        "verify_totp MUST accept a code generated for the current period. \
         A regression to a stricter offset range would reject ~10% of \
         legitimate codes due to clock skew."
    );

    // R9-D NEGATIVE: a code from far in the past (1 hour ago = 120
    // periods) MUST be rejected. The drift window is ±1 period (30s
    // either way) — codes from 3600 seconds ago are well outside it.
    let far_past = now - 3600;
    let past_code = generate_totp(&secret, far_past).expect("generate past TOTP");
    // The past code MIGHT collide with the current code by chance
    // (probability 1/10^6) — we only assert "not always accepted",
    // not "never accepted". If the two happen to collide, the verify
    // returns true, which is correct behavior.
    let past_ok = verify_totp(&secret, &past_code).expect("verify past TOTP");
    if past_ok && past_code != current_code {
        panic!(
            "verify_totp ACCEPTED a code from 1 hour ago that doesn't \
             match the current code — drift window MUST be ±1 period only. \
             past_code={past_code} current_code={current_code}"
        );
    }

    // R9-D NEGATIVE: a clearly-wrong code MUST be rejected.
    let bad = verify_totp(&secret, "000000").expect("verify bad TOTP");
    // Same collision-tolerance comment as above.
    if bad && current_code != "000000" {
        panic!(
            "verify_totp ACCEPTED '000000' when current code is {current_code} — \
             the constant_time_eq comparison MUST distinguish"
        );
    }
}

// ── 5. generate_qr_uri emits correct otpauth:// URI shape ────────────

/// `generate_qr_uri(secret, email, issuer)` MUST emit:
///
///     otpauth://totp/{issuer}:{email}?secret={SECRET}\
///         &issuer={issuer}&algorithm=SHA1&digits=6&period=30
///
/// Per line 143-154. The `secret` parameter MUST appear UNENCODED
/// in the URI (base32 chars A-Z + 2-7 are URL-safe — no encoding
/// needed, AND Google Authenticator's parser specifically expects
/// the raw base32). The `issuer` and `email` MUST be URL-encoded
/// (they can contain `@`, spaces, special chars).
///
/// Per CLAUDE.md habit #3 ("comment-vs-code drift"): the comment
/// says "Generate otpauth:// URI for QR code". The contract is
/// stricter than the comment — Google Authenticator's parser
/// rejects URIs that don't match the exact format spec. A
/// regression that URL-encoded the secret (would happen
/// accidentally if someone "consolidated" the urlencoding::encode
/// calls) would silently break QR enrollment for every new user.
#[test]
fn mfa_generate_qr_uri_shape_and_unencoded_secret() {
    let secret = "JBSWY3DPEHPK3PXP"; // RFC 4648 sample base32
    let email = "billy+test@example.com";
    let issuer = "Revolution Trading Pros";

    let uri = generate_qr_uri(secret, email, issuer);

    // Starts with the otpauth:// scheme.
    assert!(
        uri.starts_with("otpauth://totp/"),
        "QR URI MUST start with otpauth://totp/ — got '{uri}'"
    );

    // Issuer and email are URL-encoded (the `+` and `@` and space
    // are special).
    assert!(
        uri.contains("Revolution%20Trading%20Pros"),
        "QR URI MUST URL-encode the issuer's spaces; got '{uri}'"
    );
    assert!(
        uri.contains("%40example.com"),
        "QR URI MUST URL-encode the email's '@'; got '{uri}'"
    );

    // R9-D NEGATIVE: the SECRET MUST appear UNENCODED — base32 chars
    // A-Z + 2-7 don't need encoding, AND Google Authenticator's
    // parser requires the raw base32. A regression that wrapped
    // the secret in urlencoding::encode would compile but break
    // enrollment.
    assert!(
        uri.contains("secret=JBSWY3DPEHPK3PXP"),
        "QR URI MUST contain the raw base32 secret unencoded; got '{uri}'"
    );

    // Algorithm, digits, period are fixed per the TOTP constants.
    assert!(
        uri.contains("algorithm=SHA1"),
        "QR URI MUST advertise algorithm=SHA1 (TOTP_ALGORITHM constant)"
    );
    assert!(
        uri.contains("digits=6"),
        "QR URI MUST advertise digits=6 (TOTP_DIGITS constant)"
    );
    assert!(
        uri.contains("period=30"),
        "QR URI MUST advertise period=30 (TOTP_PERIOD constant)"
    );
}

// ── 6. MfaSecret BIGSERIAL i64 PK + i32 Reserved exception pin ───────

/// `MfaSecret` MUST carry `id: i64`, `user_id: i64` (both BIGSERIAL),
/// and `backup_codes_used: i32` (Reserved exception per CLAUDE.md).
///
/// Per CLAUDE.md "Money / cents — i64 ONLY, BIGINT ONLY, EVERY
/// TIME": although `id` and `user_id` are NOT money, they index
/// into BIGSERIAL tables (`mfa_secrets.id`, `users.id`) — same i64
/// invariant.
///
/// Per CLAUDE.md "Reserved exception": "row counts (`revisions:
/// i32`, `attempts: i32`, `total_pages: i32`) — those genuinely
/// cap below 2 billion and `i32` is fine. Money never qualifies."
/// `backup_codes_used` caps at 10 (the `BACKUP_CODE_COUNT` constant
/// at line 19) — i32 is wildly overspec'd for this field but is
/// the right floor. The pin documents the rationale so a future
/// "let's make all counts i64 for consistency" PR doesn't silently
/// widen the column.
///
/// R9-D NEGATIVE: a regression that flipped `id` or `user_id` to
/// `i32` would silently 400-error every user whose row landed past
/// PK 2.1B — decades-out on a small SaaS but the BIGSERIAL contract
/// enforces the invariant now. The pin builds an MfaSecret with
/// `(i32::MAX as i64) + 1` for both fields and round-trips it
/// through JSON serialization.
#[test]
fn mfa_secret_bigserial_i64_pks_and_i32_backup_count() {
    let above_i32_max: i64 = (i32::MAX as i64) + 1;

    let secret = MfaSecret {
        id: above_i32_max,
        user_id: above_i32_max,
        totp_secret: "JBSWY3DPEHPK3PXP".to_string(),
        totp_verified_at: None,
        backup_codes: serde_json::json!([]),
        backup_codes_generated_at: None,
        // Reserved exception per CLAUDE.md — caps at BACKUP_CODE_COUNT (10).
        backup_codes_used: 0,
        created_at: chrono::Utc::now(),
        updated_at: chrono::Utc::now(),
    };

    let wire = serde_json::to_value(&secret).expect("serialize MfaSecret");

    // BIGSERIAL pin: id round-trips past i32::MAX.
    assert_eq!(wire["id"].as_i64(), Some(above_i32_max));
    assert!(
        wire["id"].as_i64().unwrap() > i32::MAX as i64,
        "MfaSecret.id MUST round-trip past i32::MAX (BIGSERIAL PK)"
    );
    assert_eq!(wire["user_id"].as_i64(), Some(above_i32_max));
    assert!(
        wire["user_id"].as_i64().unwrap() > i32::MAX as i64,
        "MfaSecret.user_id MUST round-trip past i32::MAX (BIGSERIAL FK \
         into users.id, which is BIGSERIAL per schema.sql)"
    );

    // Reserved exception: backup_codes_used is i32 (caps at 10).
    // The pin confirms the field exists and is integer.
    assert!(
        wire["backup_codes_used"].is_i64() || wire["backup_codes_used"].is_u64(),
        "backup_codes_used MUST be integer (i32 in struct, JSON number on wire)"
    );

    // Round-trip through deserialize.
    let _round: MfaSecret = serde_json::from_value(wire).expect("deserialize MfaSecret");

    // Smoke-check the two response DTOs build with sensible defaults.
    let _setup = MfaSetupResponse {
        secret: "x".into(),
        qr_code_uri: "otpauth://totp/...".into(),
        backup_codes: vec![],
    };
    let _verify = MfaVerifyResult {
        success: true,
        method: "totp".into(),
        // Reserved exception: backup_codes_remaining is Option<i32> —
        // caps at BACKUP_CODE_COUNT (10).
        backup_codes_remaining: Some(10),
    };
}
