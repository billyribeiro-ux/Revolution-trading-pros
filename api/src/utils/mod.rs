//! Utility functions - December 2025 ICT11+ Principal Engineer Grade
//! SECURITY HARDENED - Authentication Hardening Audit
//!
//! Modules:
//! - errors: Standardized error response utilities
//!
//! Authentication utilities:
//! - Password hashing with Argon2id (OWASP recommended parameters)
//! - Password verification supporting both bcrypt (Laravel) and Argon2
//! - JWT token creation and verification
//! - Refresh token generation
//! - Session ID generation
//! - Constant-time comparison utilities

pub mod errors;
pub use errors::*;

use anyhow::Result;
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2, Algorithm, Params, Version,
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

/// ICT L11+ Security Hardening: Password validation rules
pub struct PasswordValidation {
    pub min_length: usize,
    pub max_length: usize,
}

impl Default for PasswordValidation {
    fn default() -> Self {
        Self {
            min_length: 12,  // OWASP minimum for financial applications
            max_length: 128, // Prevent DoS via extremely long passwords
        }
    }
}

/// Validate password meets security requirements
/// Returns Ok(()) if valid, Err with specific message if invalid
pub fn validate_password(password: &str) -> Result<(), &'static str> {
    let rules = PasswordValidation::default();
    
    if password.len() < rules.min_length {
        return Err("Password must be at least 12 characters");
    }
    if password.len() > rules.max_length {
        return Err("Password must be no more than 128 characters");
    }
    
    // Check for at least one uppercase, lowercase, digit
    let has_upper = password.chars().any(|c| c.is_ascii_uppercase());
    let has_lower = password.chars().any(|c| c.is_ascii_lowercase());
    let has_digit = password.chars().any(|c| c.is_ascii_digit());
    
    if !has_upper || !has_lower || !has_digit {
        return Err("Password must contain uppercase, lowercase, and a number");
    }
    
    Ok(())
}

/// Hash a password using Argon2id with OWASP-recommended parameters
/// ICT L11+ Security: Hardened configuration for financial applications
/// 
/// Parameters (OWASP 2024 recommendations):
/// - Algorithm: Argon2id (resistant to side-channel and GPU attacks)
/// - Memory: 64 MiB (65536 KiB)
/// - Iterations: 3
/// - Parallelism: 4
/// - Output length: 32 bytes
pub fn hash_password(password: &str) -> Result<String> {
    let salt = SaltString::generate(&mut OsRng);
    
    // OWASP-recommended Argon2id parameters for financial applications
    let params = Params::new(
        65536,  // 64 MiB memory
        3,      // 3 iterations
        4,      // 4 parallel lanes
        Some(32) // 32-byte output
    ).map_err(|e| anyhow::anyhow!("Invalid Argon2 params: {}", e))?;
    
    let argon2 = Argon2::new(Algorithm::Argon2id, Version::V0x13, params);
    
    let hash = argon2
        .hash_password(password.as_bytes(), &salt)
        .map_err(|e| anyhow::anyhow!("Password hashing failed: {}", e))?;
    Ok(hash.to_string())
}

/// Hash a "dummy" password to prevent timing attacks
/// Call this when user doesn't exist to match timing of real verification
pub fn hash_dummy_password() {
    let _ = hash_password("dummy_password_for_timing");
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

/// Generate a unique session ID (256-bit entropy)
/// ICT L11+ Security: Cryptographically secure session identifier
pub fn generate_session_id() -> String {
    use rand::Rng;
    let mut rng = rand::thread_rng();
    let bytes: [u8; 32] = rng.gen();
    hex::encode(bytes)
}

/// Constant-time string comparison to prevent timing attacks
/// ICT L11+ Security: Use for any security-sensitive comparisons
pub fn constant_time_compare(a: &str, b: &str) -> bool {
    use subtle::ConstantTimeEq;
    if a.len() != b.len() {
        return false;
    }
    a.as_bytes().ct_eq(b.as_bytes()).into()
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

/// Generate a verification token and its hash
/// Returns (raw_token, hashed_token)
/// The raw token is sent to the user, the hashed token is stored in the database
pub fn generate_verification_token() -> (String, String) {
    use rand::Rng;
    use sha2::{Digest, Sha256};
    
    let mut rng = rand::thread_rng();
    let raw_token: String = (0..64)
        .map(|_| {
            let idx = rng.gen_range(0..62);
            match idx {
                0..=9 => (b'0' + idx) as char,
                10..=35 => (b'a' + idx - 10) as char,
                36..=61 => (b'A' + idx - 36) as char,
                _ => unreachable!(),
            }
        })
        .collect();
    
    let hashed_token = hash_token(&raw_token);
    (raw_token, hashed_token)
}

/// Hash a token using SHA256 (for storing verification/reset tokens)
pub fn hash_token(token: &str) -> String {
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(token.as_bytes());
    hex::encode(hasher.finalize())
}
