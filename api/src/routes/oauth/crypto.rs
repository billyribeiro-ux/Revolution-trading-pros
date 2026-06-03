//! OAuth cryptographic helpers — PKCE + random state/nonce generation.
//!
//! Kept tiny and dependency-light so the auth flows can call these without
//! pulling in unrelated logic. All three fns are `pub(super)` — visible to
//! sibling oauth submodules only.

use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine};
use rand::RngExt;
use sha2::{Digest, Sha256};

/// Generate cryptographically secure random string
pub(super) fn generate_random_string(length: usize) -> String {
    let mut rng = rand::rng();
    let bytes: Vec<u8> = (0..length).map(|_| rng.random()).collect();
    URL_SAFE_NO_PAD.encode(&bytes)
}

/// Generate PKCE code verifier (43-128 chars)
pub(super) fn generate_code_verifier() -> String {
    generate_random_string(32) // Results in ~43 chars base64
}

/// Generate PKCE code challenge from verifier
pub(super) fn generate_code_challenge(verifier: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(verifier.as_bytes());
    let hash = hasher.finalize();
    URL_SAFE_NO_PAD.encode(hash)
}
