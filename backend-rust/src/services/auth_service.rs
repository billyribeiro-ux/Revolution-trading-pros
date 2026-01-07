//! Authentication Service
use argon2::{password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString}, Argon2};
use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;

use crate::{config::JwtSettings, errors::AppError, models::User};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub email: String,
    pub role: String,
    pub exp: i64,
    pub iat: i64,
}

#[derive(Debug)]
pub struct TokenPair {
    pub access_token: String,
    pub refresh_token: String,
    pub expires_in: i64,
}

pub struct AuthService<'a> {
    db: &'a PgPool,
    jwt_config: &'a JwtSettings,
}

impl<'a> AuthService<'a> {
    pub fn new(db: &'a PgPool, jwt_config: &'a JwtSettings) -> Self {
        Self { db, jwt_config }
    }

    pub async fn register(&self, name: &str, email: &str, password: &str) -> Result<(User, TokenPair), AppError> {
        // Check if email exists
        let exists: (bool,) = sqlx::query_as("SELECT EXISTS(SELECT 1 FROM users WHERE email = $1)")
            .bind(email.to_lowercase())
            .fetch_one(self.db)
            .await?;

        if exists.0 {
            return Err(AppError::Conflict("Email already registered".to_string()));
        }

        // Hash password
        let password_hash = self.hash_password(password)?;

        // Create user
        let id = Uuid::now_v7();
        let now = Utc::now();

        let user = sqlx::query_as::<_, User>(
            &format!(r#"
            INSERT INTO users (id, name, email, password, role, created_at, updated_at)
            VALUES ($1, $2, $3, $4, 'user', $5, $5)
            RETURNING {}
            "#, User::SELECT_COLUMNS)
        )
        .bind(id)
        .bind(name)
        .bind(email.to_lowercase())
        .bind(password_hash)
        .bind(now)
        .fetch_one(self.db)
        .await?;

        let tokens = self.generate_tokens(&user)?;
        Ok((user, tokens))
    }

    pub async fn login(&self, email: &str, password: &str) -> Result<(User, TokenPair), AppError> {
        let user = sqlx::query_as::<_, User>(
            &format!("SELECT {} FROM users WHERE email = $1", User::SELECT_COLUMNS)
        )
        .bind(email.to_lowercase())
        .fetch_optional(self.db)
        .await?
        .ok_or(AppError::InvalidCredentials)?;

        self.verify_password(password, &user.password)?;

        let tokens = self.generate_tokens(&user)?;
        Ok((user, tokens))
    }

    pub async fn refresh_token(&self, refresh_token: &str) -> Result<(User, TokenPair), AppError> {
        let claims = self.decode_token(refresh_token)?;
        let user_id = Uuid::parse_str(&claims.sub).map_err(|_| AppError::InvalidToken)?;

        let user = sqlx::query_as::<_, User>(&format!("SELECT {} FROM users WHERE id = $1", User::SELECT_COLUMNS))
            .bind(user_id)
            .fetch_optional(self.db)
            .await?
            .ok_or(AppError::InvalidToken)?;

        let tokens = self.generate_tokens(&user)?;
        Ok((user, tokens))
    }

    pub async fn validate_token(&self, token: &str) -> Result<User, AppError> {
        let claims = self.decode_token(token)?;
        let user_id = Uuid::parse_str(&claims.sub).map_err(|_| AppError::InvalidToken)?;

        sqlx::query_as::<_, User>(&format!("SELECT {} FROM users WHERE id = $1", User::SELECT_COLUMNS))
            .bind(user_id)
            .fetch_optional(self.db)
            .await?
            .ok_or(AppError::InvalidToken)
    }

    pub async fn send_password_reset(&self, _email: &str) -> Result<(), AppError> {
        // TODO: Implement password reset email
        Ok(())
    }

    pub async fn reset_password(&self, _token: &str, _email: &str, _password: &str) -> Result<(), AppError> {
        // TODO: Implement password reset
        Err(AppError::Internal("Not implemented".to_string()))
    }

    pub async fn verify_email(&self, _token: &str) -> Result<(), AppError> {
        // TODO: Implement email verification
        Err(AppError::Internal("Not implemented".to_string()))
    }

    fn hash_password(&self, password: &str) -> Result<String, AppError> {
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();
        argon2.hash_password(password.as_bytes(), &salt)
            .map(|h| h.to_string())
            .map_err(|e| AppError::Internal(format!("Password hashing error: {}", e)))
    }

    fn verify_password(&self, password: &str, hash: &str) -> Result<(), AppError> {
        let parsed_hash = PasswordHash::new(hash)
            .map_err(|_| AppError::InvalidCredentials)?;
        Argon2::default()
            .verify_password(password.as_bytes(), &parsed_hash)
            .map_err(|_| AppError::InvalidCredentials)
    }

    fn generate_tokens(&self, user: &User) -> Result<TokenPair, AppError> {
        let now = Utc::now();
        let expires_in = 15 * 60; // 15 minutes

        let access_claims = Claims {
            sub: user.id.to_string(),
            email: user.email.clone(),
            role: user.role.clone(),
            exp: (now + Duration::seconds(expires_in)).timestamp(),
            iat: now.timestamp(),
        };

        let refresh_claims = Claims {
            sub: user.id.to_string(),
            email: user.email.clone(),
            role: user.role.clone(),
            exp: (now + Duration::days(7)).timestamp(),
            iat: now.timestamp(),
        };

        let access_token = encode(
            &Header::default(),
            &access_claims,
            &EncodingKey::from_secret(self.jwt_config.secret.as_bytes()),
        ).map_err(|e| AppError::Internal(format!("JWT encoding error: {}", e)))?;

        let refresh_token = encode(
            &Header::default(),
            &refresh_claims,
            &EncodingKey::from_secret(self.jwt_config.secret.as_bytes()),
        ).map_err(|e| AppError::Internal(format!("JWT encoding error: {}", e)))?;

        Ok(TokenPair { access_token, refresh_token, expires_in })
    }

    fn decode_token(&self, token: &str) -> Result<Claims, AppError> {
        decode::<Claims>(
            token,
            &DecodingKey::from_secret(self.jwt_config.secret.as_bytes()),
            &Validation::default(),
        )
        .map(|data| data.claims)
        .map_err(|e| match e.kind() {
            jsonwebtoken::errors::ErrorKind::ExpiredSignature => AppError::TokenExpired,
            _ => AppError::InvalidToken,
        })
    }
}
