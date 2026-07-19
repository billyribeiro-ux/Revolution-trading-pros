//! Authenticated encryption for secrets at rest (AES-256-GCM).
//!
//! RUST_DEEP_AUDIT_2026-06-07 (P1-4): third-party service credentials
//! (including live Stripe keys) were stored base64-encoded — i.e. effectively
//! plaintext. This module provides real AES-256-GCM encryption keyed by
//! `CREDENTIALS_ENCRYPTION_KEY`, with transparent read-back of the legacy
//! base64 format so existing rows keep working and are upgraded on next write.
//!
//! Wire format: `"v1:" + base64(nonce(12) ‖ ciphertext‖tag)`. The `v1:` prefix
//! distinguishes an AEAD blob from a legacy base64(JSON) value; anything
//! without the prefix is treated as legacy.

use std::collections::HashMap;

use aes_gcm::{
    aead::{Aead, KeyInit},
    Aes256Gcm, Nonce,
};
use base64::{engine::general_purpose::STANDARD, Engine};
use rand::RngExt;
use sha2::{Digest, Sha256};

/// Version tag for the AES-256-GCM wire format.
const AEAD_PREFIX: &str = "v1:";

/// Derive a 32-byte AES-256 key from an arbitrary-length secret so the env var
/// can be any sufficiently-long string (mirrors how the other secrets are used)
/// rather than requiring an exact 32-byte hex blob.
fn derive_key(secret: &str) -> [u8; 32] {
    let mut hasher = Sha256::new();
    hasher.update(secret.as_bytes());
    hasher.finalize().into()
}

/// Encrypt a string→string map. Returns the legacy-distinguishable `v1:` blob,
/// or an empty string on the (practically impossible) AEAD failure.
pub fn encrypt_map(secret: &str, map: &HashMap<String, String>) -> String {
    let plaintext = serde_json::to_vec(map).unwrap_or_default();
    let cipher = Aes256Gcm::new_from_slice(&derive_key(secret))
        .expect("SHA-256 derivation always yields a 32-byte key");

    let mut rng = rand::rng();
    let nonce_bytes: [u8; 12] = std::array::from_fn(|_| rng.random());
    let nonce = Nonce::from(nonce_bytes);

    match cipher.encrypt(&nonce, plaintext.as_ref()) {
        Ok(ciphertext) => {
            let mut blob = Vec::with_capacity(nonce_bytes.len() + ciphertext.len());
            blob.extend_from_slice(&nonce_bytes);
            blob.extend_from_slice(&ciphertext);
            format!("{AEAD_PREFIX}{}", STANDARD.encode(blob))
        }
        Err(_) => String::new(),
    }
}

/// Decrypt a blob written by [`encrypt_map`], transparently falling back to the
/// legacy base64(JSON) format for rows written before AES-GCM. Returns an empty
/// map on any failure (same lenient contract as the previous implementation).
pub fn decrypt_map(secret: &str, blob: &str) -> HashMap<String, String> {
    let Some(b64) = blob.strip_prefix(AEAD_PREFIX) else {
        // Legacy: base64(JSON) written before this migration.
        return match STANDARD.decode(blob) {
            Ok(bytes) => serde_json::from_slice(&bytes).unwrap_or_default(),
            Err(_) => HashMap::new(),
        };
    };

    let raw = match STANDARD.decode(b64) {
        Ok(r) if r.len() > 12 => r,
        _ => return HashMap::new(),
    };
    let (nonce_bytes, ciphertext) = raw.split_at(12);
    let nonce_arr: [u8; 12] = match nonce_bytes.try_into() {
        Ok(a) => a,
        Err(_) => return HashMap::new(),
    };
    let cipher = Aes256Gcm::new_from_slice(&derive_key(secret))
        .expect("SHA-256 derivation always yields a 32-byte key");

    match cipher.decrypt(&Nonce::from(nonce_arr), ciphertext) {
        Ok(plaintext) => serde_json::from_slice(&plaintext).unwrap_or_default(),
        Err(_) => HashMap::new(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample() -> HashMap<String, String> {
        HashMap::from([
            ("secret_key".to_string(), "sk_live_abc123".to_string()),
            ("webhook".to_string(), "whsec_xyz".to_string()),
        ])
    }

    #[test]
    fn round_trips() {
        let blob = encrypt_map("a-strong-secret-at-least-32-chars-long!!", &sample());
        assert!(blob.starts_with("v1:"));
        let back = decrypt_map("a-strong-secret-at-least-32-chars-long!!", &blob);
        assert_eq!(back, sample());
    }

    #[test]
    fn wrong_key_fails_closed() {
        let blob = encrypt_map("a-strong-secret-at-least-32-chars-long!!", &sample());
        assert!(decrypt_map("a-different-secret-of-the-right-length!!", &blob).is_empty());
    }

    #[test]
    fn reads_legacy_base64() {
        // Legacy rows: raw base64(JSON), no "v1:" prefix.
        let legacy = STANDARD.encode(serde_json::to_vec(&sample()).unwrap());
        assert_eq!(decrypt_map("any-key", &legacy), sample());
    }

    #[test]
    fn nonce_is_randomized() {
        let a = encrypt_map("a-strong-secret-at-least-32-chars-long!!", &sample());
        let b = encrypt_map("a-strong-secret-at-least-32-chars-long!!", &sample());
        assert_ne!(a, b, "each encryption must use a fresh nonce");
    }
}
