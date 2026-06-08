//! Credential encryption / masking helpers (split from `connections.rs`
//! lines 108-152, R20-B maintainability pass, 2026-05-20). All
//! semantics preserved verbatim; only the file boundary changed.

use std::collections::HashMap;

/// Encrypt credentials for storage with AES-256-GCM (RUST_DEEP_AUDIT P1-4).
/// `key` is `Config::credentials_encryption_key`.
pub(super) fn encrypt_credentials(key: &str, credentials: &HashMap<String, String>) -> String {
    crate::utils::crypto::encrypt_map(key, credentials)
}

/// Decrypt credentials from storage. Transparently reads both the new
/// AES-256-GCM format and the legacy base64(JSON) format.
pub(super) fn decrypt_credentials(key: &str, encrypted: &str) -> HashMap<String, String> {
    crate::utils::crypto::decrypt_map(key, encrypted)
}

/// Mask sensitive credential values for display
pub(super) fn mask_credentials(credentials: &HashMap<String, String>) -> HashMap<String, String> {
    credentials
        .iter()
        .map(|(k, v)| {
            // Char-safe (P2-4 class): byte slicing would panic on multibyte tails.
            let chars: Vec<char> = v.chars().collect();
            let masked = if chars.len() <= 8 {
                "*".repeat(chars.len())
            } else {
                let last4: String = chars[chars.len() - 4..].iter().collect();
                format!("{}...{}", "*".repeat(8), last4)
            };
            (k.clone(), masked)
        })
        .collect()
}

/// Generate a secure webhook secret
pub(super) fn generate_webhook_secret() -> String {
    use rand::RngExt;
    let mut rng = rand::rng();
    let bytes: Vec<u8> = (0..32).map(|_| rng.random()).collect();
    hex::encode(bytes)
}
