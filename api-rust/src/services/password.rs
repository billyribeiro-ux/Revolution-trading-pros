//! Password hashing service using Argon2

use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use crate::error::ApiError;

/// Password service for hashing and verification
pub struct PasswordService;

impl PasswordService {
    /// Hash a password using Argon2id
    pub fn hash(password: &str) -> Result<String, ApiError> {
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();
        
        let hash = argon2
            .hash_password(password.as_bytes(), &salt)
            .map_err(|e| ApiError::Internal(format!("Password hashing failed: {}", e)))?;
        
        Ok(hash.to_string())
    }

    /// Verify a password against a hash
    pub fn verify(password: &str, hash: &str) -> Result<bool, ApiError> {
        let parsed_hash = PasswordHash::new(hash)
            .map_err(|e| ApiError::Internal(format!("Invalid password hash: {}", e)))?;
        
        let argon2 = Argon2::default();
        
        Ok(argon2.verify_password(password.as_bytes(), &parsed_hash).is_ok())
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
        use rand::Rng;
        let mut rng = rand::thread_rng();
        let bytes: [u8; 32] = rng.gen();
        base64::encode_config(&bytes, base64::URL_SAFE_NO_PAD)
    }

    /// Hash a token for storage (using SHA-256)
    pub fn hash_token(token: &str) -> String {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        
        let mut hasher = DefaultHasher::new();
        token.hash(&mut hasher);
        format!("{:x}", hasher.finish())
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
