//! Canonical Stripe webhook-signature simulator — TEST ONLY.
//!
//! Production never signs webhooks (Stripe does); this mirrors Stripe's
//! `v1` scheme so tests can exercise the *real*
//! `revolution_api::services::stripe::StripeService::verify_webhook`
//! against authentic input instead of a hand-duplicated verifier.
//!
//! Scheme: `Stripe-Signature: t=<unix_ts>,v1=<hex(HMAC_SHA256(secret,
//! "<unix_ts>.<raw_payload>"))>`. The signed bytes are exactly
//! `{timestamp}.{payload}`, identical to the production verifier's
//! reconstruction for any UTF-8 payload.
//!
//! Single source of truth — reused by the Stage-3 webhook integration
//! tests. Not all consumers use every helper; `dead_code` is expected
//! for a shared test-support module.
#![allow(dead_code)]

use hmac::{Hmac, KeyInit, Mac};
use sha2::Sha256;

type HmacSha256 = Hmac<Sha256>;

/// Build a `Stripe-Signature` header value for `payload` signed with
/// `secret` at `timestamp` (unix seconds). Pass an off-tolerance
/// `timestamp` to exercise the verifier's replay-window guard.
pub fn sign(payload: &[u8], secret: &str, timestamp: i64) -> String {
    // HMAC-SHA256 accepts a key of any length (RFC 2104 §2); construction
    // is infallible for the `new_from_slice` API.
    let mut mac = HmacSha256::new_from_slice(secret.as_bytes())
        .expect("HMAC-SHA256 accepts keys of any length (RFC 2104)");
    mac.update(timestamp.to_string().as_bytes());
    mac.update(b".");
    mac.update(payload);
    let v1 = hex::encode(mac.finalize().into_bytes());
    format!("t={timestamp},v1={v1}")
}

/// Sign at the current time — inside the verifier's ±300 s tolerance.
pub fn sign_now(payload: &[u8], secret: &str) -> String {
    sign(payload, secret, chrono::Utc::now().timestamp())
}
