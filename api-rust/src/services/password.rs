//! Password hashing service
//!
//! WASM-compatible password hashing using PBKDF2-SHA256

use sha2::Sha256;
use hmac::Hmac;
use pbkdf2::pbkdf2_hmac;
use base64::{Engine as _, engine::general_purpose::URL_SAFE_NO_PAD};
use crate::error::ApiError;

/// Password service for hashing and verification
#[derive(Clone)]
pub struct PasswordService;

const PBKDF2_ITERATIONS: u32 = 100_000;
const SALT_LENGTH: usize = 16;
const HASH_LENGTH: usize = 32;

impl PasswordService {
    /// Hash a password using PBKDF2-SHA256
    pub fn hash(password: &str) -> Result<String, ApiError> {
        // Generate random salt
        let mut salt = [0u8; SALT_LENGTH];
        getrandom::getrandom(&mut salt)
            .map_err(|e| ApiError::Internal(format!("Failed to generate salt: {}", e)))?;
        
        // Hash password
        let mut hash = [0u8; HASH_LENGTH];
        pbkdf2_hmac::<Sha256>(password.as_bytes(), &salt, PBKDF2_ITERATIONS, &mut hash);
        
        // Encode as: iterations$salt$hash (all base64)
        let encoded = format!(
            "{}${}${}",
            PBKDF2_ITERATIONS,
            URL_SAFE_NO_PAD.encode(&salt),
            URL_SAFE_NO_PAD.encode(&hash)
        );
        
        Ok(encoded)
    }

    /// Verify a password against a hash
    pub fn verify(password: &str, stored_hash: &str) -> Result<bool, ApiError> {
        let parts: Vec<&str> = stored_hash.split('$').collect();
        if parts.len() != 3 {
            return Err(ApiError::Internal("Invalid hash format".to_string()));
        }
        
        let iterations: u32 = parts[0].parse()
            .map_err(|_| ApiError::Internal("Invalid iterations".to_string()))?;
        let salt = URL_SAFE_NO_PAD.decode(parts[1])
            .map_err(|_| ApiError::Internal("Invalid salt encoding".to_string()))?;
        let stored = URL_SAFE_NO_PAD.decode(parts[2])
            .map_err(|_| ApiError::Internal("Invalid hash encoding".to_string()))?;
        
        // Hash the provided password with the same salt
        let mut computed = vec![0u8; stored.len()];
        pbkdf2_hmac::<Sha256>(password.as_bytes(), &salt, iterations, &mut computed);
        
        // Constant-time comparison
        Ok(Self::secure_compare(&computed, &stored))
    }

    /// Constant-time comparison to prevent timing attacks
    fn secure_compare(a: &[u8], b: &[u8]) -> bool {
        if a.len() != b.len() {
            return false;
        }
        a.iter().zip(b.iter()).fold(0, |acc, (x, y)| acc | (x ^ y)) == 0
    }

    /// Check if a password meets minimum requirements
    pub fn validate_strength(password: &str) -> Result<(), ApiError> {
        if password.len() < 8 {
            return Err(ApiError::Validation("Password must be at least 8 characters".to_string()));
        }
        
        let has_uppercase = password.chars().any(|c| c.is_uppercase());
        let has_lowercase = password.chars().any(|c| c.is_lowercase());
        let has_digit = password.chars().any(|c| c.is_ascii_digit());
        
        if !has_uppercase || !has_lowercase || !has_digit {
            return Err(ApiError::Validation(
                "Password must contain uppercase, lowercase, and a number".to_string()
            ));
        }
        
        Ok(())
    }

    /// Generate a random token (for password resets, email verification, etc.)
    pub fn generate_token() -> String {
        let mut bytes = [0u8; 32];
        getrandom::getrandom(&mut bytes).expect("Failed to generate random bytes");
        URL_SAFE_NO_PAD.encode(&bytes)
    }

    /// Hash a token for storage (using SHA-256)
    pub fn hash_token(token: &str) -> String {
        use sha2::{Sha256, Digest};
        let mut hasher = Sha256::new();
        hasher.update(token.as_bytes());
        hex::encode(hasher.finalize())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_password_hash_and_verify() {
        let password = "SecurePass123!";
        let hash = PasswordService::hash(password).unwrap();
        
        assert!(PasswordService::verify(password, &hash).unwrap());
        assert!(!PasswordService::verify("WrongPassword", &hash).unwrap());
    }

    #[test]
    fn test_password_validation() {
        assert!(PasswordService::validate_strength("Secure123").is_ok());
        assert!(PasswordService::validate_strength("short").is_err());
        assert!(PasswordService::validate_strength("nouppercase123").is_err());
        assert!(PasswordService::validate_strength("NOLOWERCASE123").is_err());
        assert!(PasswordService::validate_strength("NoDigitsHere").is_err());
    }

    #[test]
    fn test_token_generation() {
        let token1 = PasswordService::generate_token();
        let token2 = PasswordService::generate_token();
        
        assert_ne!(token1, token2);
        assert!(token1.len() > 20);
    }
}
