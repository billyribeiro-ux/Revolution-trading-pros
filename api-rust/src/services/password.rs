//! Password hashing service
//!
//! WASM-compatible password hashing using PBKDF2-SHA256
//! Also supports legacy bcrypt hashes from Laravel for backward compatibility

use sha2::Sha256;
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
    /// Hash a password using PBKDF2-SHA256 (for new passwords)
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

    /// Check if a hash is a bcrypt hash (from Laravel)
    fn is_bcrypt_hash(hash: &str) -> bool {
        hash.starts_with("$2y$") || hash.starts_with("$2a$") || hash.starts_with("$2b$")
    }

    /// Verify a password against a hash (supports both PBKDF2 and bcrypt)
    pub fn verify(password: &str, stored_hash: &str) -> Result<bool, ApiError> {
        // Check if this is a legacy bcrypt hash from Laravel
        if Self::is_bcrypt_hash(stored_hash) {
            return Self::verify_bcrypt(password, stored_hash);
        }
        
        // Otherwise, use PBKDF2 verification
        Self::verify_pbkdf2(password, stored_hash)
    }

    /// Verify against bcrypt hash (legacy Laravel passwords)
    fn verify_bcrypt(password: &str, stored_hash: &str) -> Result<bool, ApiError> {
        worker::console_log!("[PASSWORD] Starting bcrypt verification");
        
        // Laravel uses $2y$ prefix, bcrypt crate handles $2a$, $2b$, $2y$
        match bcrypt::verify(password, stored_hash) {
            Ok(valid) => {
                worker::console_log!("[PASSWORD] bcrypt verify completed: {}", valid);
                Ok(valid)
            },
            Err(e) => {
                worker::console_error!("[PASSWORD] bcrypt verification error: {:?}", e);
                Err(ApiError::Internal(format!("Bcrypt verification failed: {:?}", e)))
            }
        }
    }

    /// Verify against PBKDF2 hash (new passwords)
    fn verify_pbkdf2(password: &str, stored_hash: &str) -> Result<bool, ApiError> {
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
    
    /// Check if a stored hash needs migration to PBKDF2
    pub fn needs_rehash(stored_hash: &str) -> bool {
        Self::is_bcrypt_hash(stored_hash)
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
