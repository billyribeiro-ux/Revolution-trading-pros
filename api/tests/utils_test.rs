//! Unit tests for utility functions

// Import the utils module functions
// Note: These tests assume the functions are exported from the crate

mod utils {
    use anyhow::Result;
    use argon2::{
        password_hash::{
            rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString,
        },
        Argon2,
    };
    use chrono::{Duration, Utc};
    use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
    use serde::{Deserialize, Serialize};
    use uuid::Uuid;

    #[derive(Debug, Serialize, Deserialize)]
    pub struct Claims {
        pub sub: Uuid,
        pub exp: i64,
        pub iat: i64,
    }

    pub fn hash_password(password: &str) -> Result<String> {
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();
        let hash = argon2
            .hash_password(password.as_bytes(), &salt)
            .map_err(|e| anyhow::anyhow!("Password hashing failed: {}", e))?;
        Ok(hash.to_string())
    }

    pub fn verify_password(password: &str, hash: &str) -> Result<bool> {
        let parsed_hash =
            PasswordHash::new(hash).map_err(|e| anyhow::anyhow!("Invalid password hash: {}", e))?;
        Ok(Argon2::default()
            .verify_password(password.as_bytes(), &parsed_hash)
            .is_ok())
    }

    pub fn create_jwt(user_id: &Uuid, secret: &str, expires_in_hours: i64) -> Result<String> {
        let now = Utc::now();
        let claims = Claims {
            sub: *user_id,
            iat: now.timestamp(),
            exp: (now + Duration::hours(expires_in_hours)).timestamp(),
        };

        let token = encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(secret.as_bytes()),
        )?;

        Ok(token)
    }

    pub fn verify_jwt(token: &str, secret: &str) -> Result<Claims> {
        let token_data = decode::<Claims>(
            token,
            &DecodingKey::from_secret(secret.as_bytes()),
            &Validation::default(),
        )?;

        Ok(token_data.claims)
    }

    pub fn generate_token() -> String {
        use rand::Rng;
        let mut rng = rand::thread_rng();
        (0..32)
            .map(|_| rng.sample(rand::distributions::Alphanumeric) as char)
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::utils::*;
    use uuid::Uuid;

    // ===================
    // Password Hashing Tests
    // ===================

    #[test]
    fn test_hash_password_creates_valid_hash() {
        let password = "SecurePassword123!";
        let hash = hash_password(password).expect("Hashing should succeed");

        // Argon2 hashes start with $argon2
        assert!(
            hash.starts_with("$argon2"),
            "Hash should be in Argon2 format"
        );
    }

    #[test]
    fn test_hash_password_creates_unique_hashes() {
        let password = "SamePassword";
        let hash1 = hash_password(password).expect("Hashing should succeed");
        let hash2 = hash_password(password).expect("Hashing should succeed");

        // Same password should produce different hashes (due to salt)
        assert_ne!(hash1, hash2, "Each hash should be unique due to salt");
    }

    #[test]
    fn test_verify_password_correct_password() {
        let password = "CorrectPassword123";
        let hash = hash_password(password).expect("Hashing should succeed");

        let result = verify_password(password, &hash).expect("Verification should succeed");
        assert!(result, "Correct password should verify");
    }

    #[test]
    fn test_verify_password_incorrect_password() {
        let password = "CorrectPassword123";
        let hash = hash_password(password).expect("Hashing should succeed");

        let result = verify_password("WrongPassword", &hash).expect("Verification should succeed");
        assert!(!result, "Wrong password should not verify");
    }

    #[test]
    fn test_verify_password_invalid_hash() {
        let result = verify_password("password", "not-a-valid-hash");
        assert!(result.is_err(), "Invalid hash should return error");
    }

    #[test]
    fn test_hash_password_empty_password() {
        let hash = hash_password("").expect("Empty password should still hash");
        let result = verify_password("", &hash).expect("Verification should succeed");
        assert!(result, "Empty password should verify against its hash");
    }

    #[test]
    fn test_hash_password_unicode() {
        let password = "密码🔐パスワード";
        let hash = hash_password(password).expect("Unicode password should hash");
        let result = verify_password(password, &hash).expect("Verification should succeed");
        assert!(result, "Unicode password should verify");
    }

    // ===================
    // JWT Tests
    // ===================

    #[test]
    fn test_create_jwt_returns_valid_token() {
        let user_id = Uuid::new_v4();
        let secret = "test_secret_key_12345";
        let expires_in = 24;

        let token = create_jwt(&user_id, secret, expires_in).expect("JWT creation should succeed");

        // JWT has three parts separated by dots
        let parts: Vec<&str> = token.split('.').collect();
        assert_eq!(parts.len(), 3, "JWT should have 3 parts");
    }

    #[test]
    fn test_verify_jwt_valid_token() {
        let user_id = Uuid::new_v4();
        let secret = "test_secret_key_12345";

        let token = create_jwt(&user_id, secret, 24).expect("JWT creation should succeed");
        let claims = verify_jwt(&token, secret).expect("JWT verification should succeed");

        assert_eq!(claims.sub, user_id, "User ID should match");
    }

    #[test]
    fn test_verify_jwt_wrong_secret() {
        let user_id = Uuid::new_v4();
        let token =
            create_jwt(&user_id, "correct_secret", 24).expect("JWT creation should succeed");

        let result = verify_jwt(&token, "wrong_secret");
        assert!(result.is_err(), "Wrong secret should fail verification");
    }

    #[test]
    fn test_verify_jwt_expired_token() {
        let user_id = Uuid::new_v4();
        let secret = "test_secret";

        // Create a token that's already expired (negative hours)
        let token = create_jwt(&user_id, secret, -1).expect("JWT creation should succeed");

        let result = verify_jwt(&token, secret);
        assert!(result.is_err(), "Expired token should fail verification");
    }

    #[test]
    fn test_verify_jwt_invalid_format() {
        let result = verify_jwt("not.a.valid.jwt.token", "secret");
        assert!(result.is_err(), "Invalid JWT format should fail");
    }

    #[test]
    fn test_jwt_roundtrip_preserves_user_id() {
        let original_id = Uuid::parse_str("550e8400-e29b-41d4-a716-446655440000").unwrap();
        let secret = "roundtrip_test_secret";

        let token = create_jwt(&original_id, secret, 1).expect("JWT creation should succeed");
        let claims = verify_jwt(&token, secret).expect("JWT verification should succeed");

        assert_eq!(claims.sub, original_id, "User ID should survive roundtrip");
    }

    // ===================
    // Token Generation Tests
    // ===================

    #[test]
    fn test_generate_token_length() {
        let token = generate_token();
        assert_eq!(token.len(), 32, "Token should be 32 characters");
    }

    #[test]
    fn test_generate_token_is_alphanumeric() {
        let token = generate_token();
        assert!(
            token.chars().all(|c| c.is_alphanumeric()),
            "Token should only contain alphanumeric characters"
        );
    }

    #[test]
    fn test_generate_token_uniqueness() {
        let token1 = generate_token();
        let token2 = generate_token();
        assert_ne!(token1, token2, "Generated tokens should be unique");
    }

    #[test]
    fn test_generate_token_multiple_uniqueness() {
        let tokens: Vec<String> = (0..100).map(|_| generate_token()).collect();
        let unique_count = tokens
            .iter()
            .collect::<std::collections::HashSet<_>>()
            .len();
        assert_eq!(unique_count, 100, "All 100 tokens should be unique");
    }
}
