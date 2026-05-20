//! Apple ID token cryptographic validation.
//!
//! Fetches Apple's JWKS, finds the matching key by `kid`, and validates the
//! ID token signature, issuer, audience, and nonce. The nonce comparison
//! hashes the stored nonce with SHA-256 + URL-safe base64 (no pad) — the
//! same encoding the init handler used when sending it to Apple — and
//! refuses the token on mismatch.
//!
//! Security audit logs (`apple_token_validation_failed`,
//! `apple_nonce_mismatch`, `apple_token_validated`) are emitted verbatim
//! against `target: "security_audit"` and MUST NOT be reworded.

use axum::{http::StatusCode, Json};
use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine};
use jsonwebtoken::{decode, decode_header, Algorithm, DecodingKey, Validation};
use serde::Deserialize;
use sha2::{Digest, Sha256};

/// Apple ID token claims
#[derive(Debug, Deserialize)]
pub(super) struct AppleIdTokenClaims {
    pub(super) iss: String, // https://appleid.apple.com
    pub(super) sub: String, // Apple user ID
    pub(super) aud: String, // Client ID
    pub(super) exp: i64,
    pub(super) iat: i64,
    pub(super) nonce: Option<String>,
    pub(super) nonce_supported: Option<bool>,
    pub(super) email: Option<String>,
    pub(super) email_verified: Option<String>, // "true" or "false" as string
    pub(super) is_private_email: Option<String>,
    pub(super) real_user_status: Option<i32>, // 0=unsupported, 1=unknown, 2=likely_real
}

/// Apple public key from JWKS
#[derive(Debug, Deserialize, Clone)]
struct AppleJwk {
    kty: String,
    kid: String,
    #[serde(rename = "use")]
    use_: String,
    alg: String,
    n: String, // RSA modulus
    e: String, // RSA exponent
}

/// Apple JWKS response
#[derive(Debug, Deserialize)]
struct AppleJwks {
    keys: Vec<AppleJwk>,
}

/// Fetch Apple's public keys from JWKS endpoint
/// ICT 7 SECURITY: Proper cryptographic validation of Apple ID tokens
async fn fetch_apple_public_keys() -> Result<AppleJwks, (StatusCode, Json<serde_json::Value>)> {
    let response = reqwest::Client::new()
        .get("https://appleid.apple.com/auth/keys")
        .timeout(std::time::Duration::from_secs(10))
        .send()
        .await
        .map_err(|e| {
            tracing::error!("Failed to fetch Apple public keys: {}", e);
            (
                StatusCode::BAD_GATEWAY,
                Json(serde_json::json!({"error": "Failed to fetch Apple public keys"})),
            )
        })?;

    response.json::<AppleJwks>().await.map_err(|e| {
        tracing::error!("Failed to parse Apple JWKS: {}", e);
        (
            StatusCode::BAD_GATEWAY,
            Json(serde_json::json!({"error": "Invalid Apple JWKS response"})),
        )
    })
}

/// Validate Apple ID token with proper signature verification
/// ICT 7 SECURITY: Full cryptographic validation using Apple's public keys
pub(super) async fn validate_apple_id_token(
    id_token: &str,
    expected_audience: &str,
    stored_nonce: &str,
) -> Result<AppleIdTokenClaims, (StatusCode, Json<serde_json::Value>)> {
    // Decode header to get the key ID
    let header = decode_header(id_token).map_err(|e| {
        tracing::error!("Failed to decode Apple ID token header: {}", e);
        (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({"error": "Invalid ID token header"})),
        )
    })?;

    let kid = header.kid.ok_or_else(|| {
        tracing::error!("Apple ID token missing kid in header");
        (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({"error": "Invalid ID token: missing key ID"})),
        )
    })?;

    // Fetch Apple's public keys
    let jwks = fetch_apple_public_keys().await?;

    // Find the matching key
    let apple_key = jwks.keys.iter().find(|k| k.kid == kid).ok_or_else(|| {
        tracing::error!("No matching Apple public key found for kid: {}", kid);
        (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({"error": "Invalid ID token: unknown signing key"})),
        )
    })?;

    // Create decoding key from RSA components
    let decoding_key =
        DecodingKey::from_rsa_components(&apple_key.n, &apple_key.e).map_err(|e| {
            tracing::error!("Failed to create decoding key from Apple JWK: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({"error": "Failed to process signing key"})),
            )
        })?;

    // Set up validation
    let mut validation = Validation::new(Algorithm::RS256);
    validation.set_issuer(&["https://appleid.apple.com"]);
    validation.set_audience(&[expected_audience]);

    // Decode and validate the token
    let token_data =
        decode::<AppleIdTokenClaims>(id_token, &decoding_key, &validation).map_err(|e| {
            tracing::error!(
                target: "security_audit",
                event = "apple_token_validation_failed",
                error = %e,
                "Apple ID token validation failed"
            );
            (
                StatusCode::UNAUTHORIZED,
                Json(serde_json::json!({"error": "Invalid or expired ID token"})),
            )
        })?;

    let claims = token_data.claims;

    // ICT 7 SECURITY: Validate nonce to prevent replay attacks
    if let Some(ref token_nonce) = claims.nonce {
        // Hash the stored nonce the same way we did when sending it
        let expected_nonce_hash = {
            let mut hasher = Sha256::new();
            hasher.update(stored_nonce.as_bytes());
            URL_SAFE_NO_PAD.encode(hasher.finalize())
        };

        if token_nonce != &expected_nonce_hash {
            tracing::error!(
                target: "security_audit",
                event = "apple_nonce_mismatch",
                "Apple ID token nonce validation failed"
            );
            return Err((
                StatusCode::UNAUTHORIZED,
                Json(serde_json::json!({"error": "Invalid token: nonce mismatch"})),
            ));
        }
    }

    tracing::info!(
        target: "security_audit",
        event = "apple_token_validated",
        sub = %claims.sub,
        "Apple ID token cryptographically validated"
    );

    Ok(claims)
}
