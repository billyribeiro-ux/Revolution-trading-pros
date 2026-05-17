//! Stripe webhook signature verification — exercises the REAL
//! `revolution_api::services::stripe::StripeService::verify_webhook`.
//!
//! These tests previously ran against a hand-duplicated copy of the
//! verifier that *skipped the timestamp check*, so a regression in the
//! shipped code (e.g. dropping the replay-window guard or the strict
//! UTF-8 requirement) would not have been caught. They now bind to the
//! production type via the public builder
//! (`StripeService::new(..).with_webhook_secret(..)`) and assert the
//! real security properties.

use revolution_api::services::stripe::StripeService;

#[path = "common/stripe_sig.rs"]
mod stripe_sig;

const SECRET: &str = "whsec_test_secret_12345";

/// Service whose webhook secret matches the signer.
fn svc() -> StripeService {
    StripeService::new("sk_test_dummy").with_webhook_secret(SECRET)
}

// ── Valid signatures ────────────────────────────────────────────────

#[test]
fn valid_signature_verifies() {
    let payload = br#"{"type":"checkout.session.completed"}"#;
    let header = stripe_sig::sign_now(payload, SECRET);
    assert!(svc().verify_webhook(payload, &header).unwrap());
}

#[test]
fn matches_when_one_of_several_v1_is_correct() {
    let payload = br#"{"id":"evt_123"}"#;
    let good = stripe_sig::sign_now(payload, SECRET);
    // good == "t=<ts>,v1=<sig>" — splice a bogus v1 before the real one.
    let (t, v1) = good.split_once(',').unwrap();
    let header = format!("{t},v1=deadbeef,{v1}");
    assert!(svc().verify_webhook(payload, &header).unwrap());
}

#[test]
fn ignores_unknown_header_fields() {
    let payload = br#"{"test":true}"#;
    let good = stripe_sig::sign_now(payload, SECRET);
    let (t, v1) = good.split_once(',').unwrap();
    let header = format!("{t},v0=old,{v1},v2=new");
    assert!(svc().verify_webhook(payload, &header).unwrap());
}

#[test]
fn empty_payload_with_valid_signature_verifies() {
    let header = stripe_sig::sign_now(b"", SECRET);
    assert!(svc().verify_webhook(b"", &header).unwrap());
}

#[test]
fn unicode_payload_verifies() {
    let payload = r#"{"name":"日本語","emoji":"🎉"}"#.as_bytes();
    let header = stripe_sig::sign_now(payload, SECRET);
    assert!(svc().verify_webhook(payload, &header).unwrap());
}

#[test]
fn large_payload_verifies() {
    let payload = "x".repeat(100_000);
    let header = stripe_sig::sign_now(payload.as_bytes(), SECRET);
    assert!(svc().verify_webhook(payload.as_bytes(), &header).unwrap());
}

// ── Rejected signatures (Ok(false)) ─────────────────────────────────

#[test]
fn wrong_secret_does_not_verify() {
    let payload = br#"{"type":"payment_intent.succeeded"}"#;
    let header = stripe_sig::sign_now(payload, "the_wrong_secret");
    assert!(!svc().verify_webhook(payload, &header).unwrap());
}

#[test]
fn tampered_payload_does_not_verify() {
    let header = stripe_sig::sign_now(br#"{"amount":1000}"#, SECRET);
    assert!(!svc()
        .verify_webhook(br#"{"amount":9999}"#, &header)
        .unwrap());
}

#[test]
fn garbage_signature_does_not_verify() {
    let now = chrono::Utc::now().timestamp();
    let header = format!("t={now},v1=abc123def456");
    assert!(!svc().verify_webhook(b"{}", &header).unwrap());
}

#[test]
fn signature_comparison_is_case_sensitive() {
    // Production hex-encodes lowercase and compares constant-time over
    // bytes; an upper-cased signature must not verify.
    let payload = br#"{"x":1}"#;
    let good = stripe_sig::sign_now(payload, SECRET);
    let header = good
        .to_uppercase()
        .replace("T=", "t=")
        .replace("V1=", "v1=");
    assert!(!svc().verify_webhook(payload, &header).unwrap());
}

// ── Hard errors (Result::Err) ───────────────────────────────────────

#[test]
fn missing_timestamp_errors() {
    let err = svc().verify_webhook(b"{}", "v1=somesignature").unwrap_err();
    assert!(err.to_string().contains("timestamp"));
}

#[test]
fn missing_v1_signature_errors() {
    let now = chrono::Utc::now().timestamp();
    assert!(svc().verify_webhook(b"{}", &format!("t={now}")).is_err());
}

#[test]
fn empty_header_errors() {
    assert!(svc().verify_webhook(b"{}", "").is_err());
}

#[test]
fn malformed_header_errors() {
    assert!(svc()
        .verify_webhook(b"{}", "malformed=header=format")
        .is_err());
}

#[test]
fn non_numeric_timestamp_errors() {
    assert!(svc()
        .verify_webhook(b"{}", "t=not-a-number,v1=deadbeef")
        .is_err());
}

/// Security property (previously UNTESTED — the duplicated verifier
/// skipped it): a stale timestamp must be rejected even with an
/// otherwise-valid signature, defeating webhook replay.
#[test]
fn stale_timestamp_is_rejected() {
    let payload = br#"{"type":"checkout.session.completed"}"#;
    let stale = chrono::Utc::now().timestamp() - 3600; // 1h old, >300s
    let header = stripe_sig::sign(payload, SECRET, stale);
    let err = svc().verify_webhook(payload, &header).unwrap_err();
    assert!(err.to_string().to_lowercase().contains("tolerance"));
}

/// The tolerance window is symmetric — a far-future timestamp is also
/// rejected (covers the `.abs()` branch).
#[test]
fn far_future_timestamp_is_rejected() {
    let payload = br#"{"type":"checkout.session.completed"}"#;
    let future = chrono::Utc::now().timestamp() + 3600;
    let header = stripe_sig::sign(payload, SECRET, future);
    assert!(svc().verify_webhook(payload, &header).is_err());
}

/// Security property (previously UNTESTED): a non-UTF-8 payload is
/// rejected outright — production must not lossily coerce bytes before
/// HMAC (that would let an attacker mutate non-UTF-8 bytes freely).
#[test]
fn non_utf8_payload_is_rejected() {
    let payload: &[u8] = &[0x00, 0x01, 0x02, 0xFF, 0xFE];
    let now = chrono::Utc::now().timestamp();
    let header = format!("t={now},v1=00");
    let err = svc().verify_webhook(payload, &header).unwrap_err();
    assert!(err.to_string().to_lowercase().contains("utf-8"));
}

/// A service without a configured webhook secret must fail closed.
#[test]
fn unconfigured_webhook_secret_errors() {
    let no_secret = StripeService::new("sk_test_dummy");
    let now = chrono::Utc::now().timestamp();
    assert!(no_secret
        .verify_webhook(b"{}", &format!("t={now},v1=00"))
        .is_err());
}
