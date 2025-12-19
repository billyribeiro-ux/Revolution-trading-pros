//! Utility functions - December 2025 ICT11+ Principal Engineer Grade
//!
//! Authentication utilities:
//! - Password hashing with Argon2 (new users)
//! - Password verification supporting both bcrypt (Laravel) and Argon2
//! - JWT token creation and verification
//! - Refresh token generation
//! - Session ID generation

use anyhow::Result;
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// JWT claims for access tokens
#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: i64,       // Subject (user ID)
    pub exp: i64,       // Expiration time
    pub iat: i64,       // Issued at
    #[serde(default)]
    pub token_type: String,  // "access" or "refresh"
}

/// Hash a password using Argon2 (for new users)
pub fn hash_password(password: &str) -> Result<String> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let hash = argon2
        .hash_password(password.as_bytes(), &salt)
        .map_err(|e| anyhow::anyhow!("Password hashing failed: {}", e))?;
    Ok(hash.to_string())
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
                Err(anyhow::anyhow!("Password verification failed: {}", e))
            }
        }
    } else if hash.starts_with("$argon2") {
        // Argon2 hash (new Rust API users)
        let parsed_hash = PasswordHash::new(hash)
            .map_err(|e| anyhow::anyhow!("Invalid Argon2 hash: {}", e))?;
        Ok(Argon2::default()
            .verify_password(password.as_bytes(), &parsed_hash)
            .is_ok())
    } else {
        // Unknown hash format
        tracing::error!("Unknown password hash format: {}", &hash[..hash.len().min(10)]);
        Err(anyhow::anyhow!("Unknown password hash format"))
    }
}

/// Create a JWT access token
pub fn create_jwt(user_id: i64, secret: &str, expires_in_hours: i64) -> Result<String> {
    let now = Utc::now();
    let claims = Claims {
        sub: user_id,
        iat: now.timestamp(),
        exp: (now + Duration::hours(expires_in_hours)).timestamp(),
        token_type: "access".to_string(),
    };

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )?;

    Ok(token)
}

/// Create a JWT refresh token (longer expiry - 7 days)
pub fn create_refresh_token(user_id: i64, secret: &str) -> Result<String> {
    let now = Utc::now();
    let claims = Claims {
        sub: user_id,
        iat: now.timestamp(),
        exp: (now + Duration::days(7)).timestamp(),
        token_type: "refresh".to_string(),
    };

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )?;

    Ok(token)
}

/// Verify and decode a JWT token
pub fn verify_jwt(token: &str, secret: &str) -> Result<Claims> {
    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::default(),
    )?;

    Ok(token_data.claims)
}

/// Generate a unique session ID
pub fn generate_session_id() -> String {
    Uuid::new_v4().to_string()
}

/// Generate a random token (for password reset, etc.)
pub fn generate_token() -> String {
    use rand::Rng;
    let mut rng = rand::thread_rng();
    (0..32)
        .map(|_| rng.sample(rand::distributions::Alphanumeric) as char)
        .collect()
}

/// Generate a secure password reset token (URL-safe, 64 chars)
pub fn generate_password_reset_token() -> String {
    use rand::Rng;
    let mut rng = rand::thread_rng();
    (0..64)
        .map(|_| {
            let idx = rng.gen_range(0..62);
            match idx {
                0..=9 => (b'0' + idx) as char,
                10..=35 => (b'a' + idx - 10) as char,
                36..=61 => (b'A' + idx - 36) as char,
                _ => unreachable!(),
            }
        })
        .collect()
}
