//! JWT authentication service

use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use crate::error::ApiError;
use crate::models::user::{Claims, UserRole};

/// JWT service for token generation and validation
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

    /// Generate an access token for a user
    pub fn generate_access_token(
        &self,
        user_id: &str,
        email: &str,
        role: UserRole,
    ) -> Result<(String, i64), ApiError> {
        let now = Utc::now();
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

        let token = encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(self.secret.as_bytes()),
        )
        .map_err(|e| ApiError::Internal(format!("Failed to generate token: {}", e)))?;

        Ok((token, exp.timestamp()))
    }

    /// Generate a refresh token
    pub fn generate_refresh_token(&self, user_id: &str) -> Result<(String, i64), ApiError> {
        let now = Utc::now();
        let exp = now + self.refresh_token_duration;

        // Refresh token has minimal claims
        let claims = serde_json::json!({
            "sub": user_id,
            "type": "refresh",
            "iat": now.timestamp(),
            "exp": exp.timestamp(),
            "iss": self.issuer,
        });

        let token = encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(self.secret.as_bytes()),
        )
        .map_err(|e| ApiError::Internal(format!("Failed to generate refresh token: {}", e)))?;

        Ok((token, exp.timestamp()))
    }

    /// Validate and decode an access token
    pub fn validate_access_token(&self, token: &str) -> Result<Claims, ApiError> {
        let mut validation = Validation::default();
        validation.set_issuer(&[&self.issuer]);
        validation.set_audience(&[&self.audience]);

        let token_data = decode::<Claims>(
            token,
            &DecodingKey::from_secret(self.secret.as_bytes()),
            &validation,
        )
        .map_err(|e| match e.kind() {
            jsonwebtoken::errors::ErrorKind::ExpiredSignature => {
                ApiError::Unauthorized("Token expired".to_string())
            }
            jsonwebtoken::errors::ErrorKind::InvalidToken => {
                ApiError::Unauthorized("Invalid token".to_string())
            }
            _ => ApiError::Unauthorized(format!("Token validation failed: {}", e)),
        })?;

        Ok(token_data.claims)
    }

    /// Validate a refresh token and return the user_id
    pub fn validate_refresh_token(&self, token: &str) -> Result<String, ApiError> {
        let mut validation = Validation::default();
        validation.set_issuer(&[&self.issuer]);
        // Refresh tokens don't have audience

        #[derive(serde::Deserialize)]
        struct RefreshClaims {
            sub: String,
            #[serde(rename = "type")]
            token_type: String,
        }

        let token_data = decode::<RefreshClaims>(
            token,
            &DecodingKey::from_secret(self.secret.as_bytes()),
            &validation,
        )
        .map_err(|e| ApiError::Unauthorized(format!("Invalid refresh token: {}", e)))?;

        if token_data.claims.token_type != "refresh" {
            return Err(ApiError::Unauthorized("Invalid token type".to_string()));
        }

        Ok(token_data.claims.sub)
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
