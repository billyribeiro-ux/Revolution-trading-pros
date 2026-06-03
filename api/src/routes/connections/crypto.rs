//! Credential encryption / masking helpers (split from `connections.rs`
//! lines 108-152, R20-B maintainability pass, 2026-05-20). All
//! semantics preserved verbatim; only the file boundary changed.

use std::collections::HashMap;

/// Encrypt credentials for storage (uses base64 encoding as placeholder - in production use AES-256)
pub(super) fn encrypt_credentials(credentials: &HashMap<String, String>) -> String {
    use base64::{engine::general_purpose::STANDARD, Engine};
    let json = serde_json::to_string(credentials).unwrap_or_default();
    // In production, use proper AES-256-GCM encryption with a key from env
    // For now, we use base64 encoding as a placeholder
    STANDARD.encode(json.as_bytes())
}

/// Decrypt credentials from storage
pub(super) fn decrypt_credentials(encrypted: &str) -> HashMap<String, String> {
    use base64::{engine::general_purpose::STANDARD, Engine};
    // In production, use proper AES-256-GCM decryption
    match STANDARD.decode(encrypted) {
        Ok(bytes) => serde_json::from_slice(&bytes).unwrap_or_default(),
        Err(_) => HashMap::new(),
    }
}

/// Mask sensitive credential values for display
pub(super) fn mask_credentials(credentials: &HashMap<String, String>) -> HashMap<String, String> {
    credentials
        .iter()
        .map(|(k, v)| {
            let masked = if v.len() <= 8 {
                "*".repeat(v.len())
            } else {
                format!("{}...{}", "*".repeat(8), &v[v.len().saturating_sub(4)..])
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
