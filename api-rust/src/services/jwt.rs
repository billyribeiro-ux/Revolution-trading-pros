//! JWT authentication service
//! 
//! WASM-compatible JWT implementation using HMAC-SHA256

use chrono::{Duration, Utc};
use hmac::{Hmac, Mac};
use sha2::Sha256;
use base64::{Engine as _, engine::general_purpose::URL_SAFE_NO_PAD};
use serde::{Deserialize, Serialize};
use crate::error::ApiError;
use crate::models::user::{Claims, UserRole};

type HmacSha256 = Hmac<Sha256>;

/// JWT Header
#[derive(Serialize)]
struct JwtHeader {
    alg: &'static str,
    typ: &'static str,
}

impl Default for JwtHeader {
    fn default() -> Self {
        Self {
            alg: "HS256",
            typ: "JWT",
        }
    }
}

/// JWT service for token generation and validation
#[derive(Clone)]
pub struct JwtService {
    secret: String,
    issuer: String,
    audience: String,
    access_token_duration: Duration,
    refresh_token_duration: Duration,
}

impl JwtService {
    pub fn new(secret: &str, issuer: &str, audience: &str) -> Self {
        Self {
            secret: secret.to_string(),
            issuer: issuer.to_string(),
            audience: audience.to_string(),
            access_token_duration: Duration::hours(1),
            refresh_token_duration: Duration::days(30),
        }
    }

    /// Encode a JWT token
    fn encode_jwt<T: Serialize>(&self, claims: &T) -> Result<String, ApiError> {
        let header = JwtHeader::default();
        
        let header_json = serde_json::to_string(&header)
            .map_err(|e| ApiError::Internal(format!("Failed to serialize header: {}", e)))?;
        let claims_json = serde_json::to_string(claims)
            .map_err(|e| ApiError::Internal(format!("Failed to serialize claims: {}", e)))?;
        
        let header_b64 = URL_SAFE_NO_PAD.encode(header_json.as_bytes());
        let claims_b64 = URL_SAFE_NO_PAD.encode(claims_json.as_bytes());
        
        let message = format!("{}.{}", header_b64, claims_b64);
        
        let mut mac = HmacSha256::new_from_slice(self.secret.as_bytes())
            .map_err(|e| ApiError::Internal(format!("HMAC error: {}", e)))?;
        mac.update(message.as_bytes());
        let signature = mac.finalize().into_bytes();
        let signature_b64 = URL_SAFE_NO_PAD.encode(&signature);
        
        Ok(format!("{}.{}", message, signature_b64))
    }

    /// Decode and verify a JWT token
    fn decode_jwt<T: for<'de> Deserialize<'de>>(&self, token: &str) -> Result<T, ApiError> {
        let parts: Vec<&str> = token.split('.').collect();
        if parts.len() != 3 {
            return Err(ApiError::Unauthorized("Invalid token format".to_string()));
        }
        
        let message = format!("{}.{}", parts[0], parts[1]);
        let signature = URL_SAFE_NO_PAD.decode(parts[2])
            .map_err(|_| ApiError::Unauthorized("Invalid token signature encoding".to_string()))?;
        
        // Verify signature
        let mut mac = HmacSha256::new_from_slice(self.secret.as_bytes())
            .map_err(|e| ApiError::Internal(format!("HMAC error: {}", e)))?;
        mac.update(message.as_bytes());
        mac.verify_slice(&signature)
            .map_err(|_| ApiError::Unauthorized("Invalid token signature".to_string()))?;
        
        // Decode claims
        let claims_json = URL_SAFE_NO_PAD.decode(parts[1])
            .map_err(|_| ApiError::Unauthorized("Invalid token claims encoding".to_string()))?;
        let claims: T = serde_json::from_slice(&claims_json)
            .map_err(|e| ApiError::Unauthorized(format!("Invalid token claims: {}", e)))?;
        
        Ok(claims)
    }

    /// Generate an access token for a user
    pub fn generate_access_token(
        &self,
        user_id: &str,
        email: &str,
        role: UserRole,
    ) -> Result<(String, i64), ApiError> {
        let now = crate::utils::now();
        let exp = now + self.access_token_duration;

        let claims = Claims {
            sub: user_id.to_string(),
            email: email.to_string(),
            role,
            iat: now.timestamp(),
            exp: exp.timestamp(),
            iss: self.issuer.clone(),
            aud: self.audience.clone(),
        };

        let token = self.encode_jwt(&claims)?;
        Ok((token, exp.timestamp()))
    }

    /// Generate a refresh token
    pub fn generate_refresh_token(&self, user_id: &str) -> Result<(String, i64), ApiError> {
        let now = crate::utils::now();
        let exp = now + self.refresh_token_duration;

        #[derive(Serialize)]
        struct RefreshClaims {
            sub: String,
            #[serde(rename = "type")]
            token_type: String,
            iat: i64,
            exp: i64,
            iss: String,
        }

        let claims = RefreshClaims {
            sub: user_id.to_string(),
            token_type: "refresh".to_string(),
            iat: now.timestamp(),
            exp: exp.timestamp(),
            iss: self.issuer.clone(),
        };

        let token = self.encode_jwt(&claims)?;
        Ok((token, exp.timestamp()))
    }

    /// Validate and decode an access token
    pub fn validate_access_token(&self, token: &str) -> Result<Claims, ApiError> {
        let claims: Claims = self.decode_jwt(token)?;
        
        // Check expiration
        let now = crate::utils::now_timestamp();
        if claims.exp < now {
            return Err(ApiError::Unauthorized("Token expired".to_string()));
        }
        
        // Check issuer
        if claims.iss != self.issuer {
            return Err(ApiError::Unauthorized("Invalid token issuer".to_string()));
        }
        
        // Check audience
        if claims.aud != self.audience {
            return Err(ApiError::Unauthorized("Invalid token audience".to_string()));
        }
        
        Ok(claims)
    }

    /// Validate a refresh token and return the user_id
    pub fn validate_refresh_token(&self, token: &str) -> Result<String, ApiError> {
        #[derive(Deserialize)]
        struct RefreshClaims {
            sub: String,
            #[serde(rename = "type")]
            token_type: String,
            exp: i64,
            iss: String,
        }

        let claims: RefreshClaims = self.decode_jwt(token)?;
        
        // Check expiration
        let now = crate::utils::now_timestamp();
        if claims.exp < now {
            return Err(ApiError::Unauthorized("Refresh token expired".to_string()));
        }
        
        // Check issuer
        if claims.iss != self.issuer {
            return Err(ApiError::Unauthorized("Invalid token issuer".to_string()));
        }
        
        // Check token type
        if claims.token_type != "refresh" {
            return Err(ApiError::Unauthorized("Invalid token type".to_string()));
        }

        Ok(claims.sub)
    }

    /// Extract token from Authorization header
    pub fn extract_token(auth_header: &str) -> Option<&str> {
        if auth_header.starts_with("Bearer ") {
            Some(&auth_header[7..])
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_token_generation_and_validation() {
        let service = JwtService::new("test-secret-key-12345", "test-issuer", "test-audience");
        
        let (token, _exp) = service
            .generate_access_token("user-123", "test@example.com", UserRole::User)
            .unwrap();
        
        let claims = service.validate_access_token(&token).unwrap();
        
        assert_eq!(claims.sub, "user-123");
        assert_eq!(claims.email, "test@example.com");
        assert_eq!(claims.role, UserRole::User);
    }

    #[test]
    fn test_refresh_token() {
        let service = JwtService::new("test-secret-key-12345", "test-issuer", "test-audience");
        
        let (token, _exp) = service.generate_refresh_token("user-123").unwrap();
        let user_id = service.validate_refresh_token(&token).unwrap();
        
        assert_eq!(user_id, "user-123");
    }

    #[test]
    fn test_extract_token() {
        assert_eq!(
            JwtService::extract_token("Bearer abc123"),
            Some("abc123")
        );
        assert_eq!(JwtService::extract_token("abc123"), None);
    }
}
