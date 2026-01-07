//! Authentication Service
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
// ICT 11+: Production DB uses INT8 for user IDs, not UUID

use crate::{
    config::{DeveloperSettings, JwtSettings},
    errors::AppError,
    models::User,
};

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
    developer_config: Option<&'a DeveloperSettings>,
}

impl<'a> AuthService<'a> {
    pub fn new(db: &'a PgPool, jwt_config: &'a JwtSettings) -> Self {
        Self {
            db,
            jwt_config,
            developer_config: None,
        }
    }

    /// ICT 11+: Create auth service with developer mode support
    pub fn with_developer_config(
        db: &'a PgPool,
        jwt_config: &'a JwtSettings,
        developer_config: &'a DeveloperSettings,
    ) -> Self {
        Self {
            db,
            jwt_config,
            developer_config: Some(developer_config),
        }
    }

    /// Check if email is a developer email with bypass enabled
    fn is_developer_email(&self, email: &str) -> bool {
        if let Some(dev_config) = &self.developer_config {
            if dev_config.enabled {
                return dev_config.emails.contains(&email.to_lowercase());
            }
        }
        false
    }

    /// Get developer password for bypass
    fn get_developer_password(&self) -> Option<&str> {
        self.developer_config
            .filter(|c| c.enabled)
            .map(|c| c.password.as_str())
    }

    pub async fn register(
        &self,
        name: &str,
        email: &str,
        password: &str,
    ) -> Result<(User, TokenPair), AppError> {
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

        // Create user - production DB uses auto-increment INT8 for id
        // Use NaiveDateTime for TIMESTAMP columns (no timezone)
        let now = chrono::Utc::now().naive_utc();

        let user = sqlx::query_as::<_, User>(&format!(
            r#"
            INSERT INTO users (name, email, password, role, created_at, updated_at)
            VALUES ($1, $2, $3, 'user', $4, $4)
            RETURNING {}
            "#,
            User::SELECT_COLUMNS
        ))
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
        // ICT 11+: Developer mode bypass - create super-admin user if doesn't exist
        if self.is_developer_email(email) {
            if let Some(dev_password) = self.get_developer_password() {
                if password == dev_password {
                    return self.developer_login(email).await;
                }
            }
        }

        let user = sqlx::query_as::<_, User>(&format!(
            "SELECT {} FROM users WHERE email = $1",
            User::SELECT_COLUMNS
        ))
        .bind(email.to_lowercase())
        .fetch_optional(self.db)
        .await?
        .ok_or(AppError::InvalidCredentials)?;

        self.verify_password(password, &user.password)?;

        let tokens = self.generate_tokens(&user)?;
        Ok((user, tokens))
    }

    /// ICT 11+: Developer login - creates super-admin user if doesn't exist
    /// Developer email ALWAYS bypasses email verification
    async fn developer_login(&self, email: &str) -> Result<(User, TokenPair), AppError> {
        let email_lower = email.to_lowercase();
        let now = chrono::Utc::now().naive_utc();

        // Check if developer user exists
        let existing_user = sqlx::query_as::<_, User>(&format!(
            "SELECT {} FROM users WHERE email = $1",
            User::SELECT_COLUMNS
        ))
        .bind(&email_lower)
        .fetch_optional(self.db)
        .await?;

        let user = match existing_user {
            Some(user) => {
                // ICT 11+: Developer ALWAYS gets super-admin + email verified
                // Update role AND set email_verified_at if needed
                let needs_update = user.role != "super-admin" || user.email_verified_at.is_none();

                if needs_update {
                    sqlx::query(
                        "UPDATE users SET role = 'super-admin', email_verified_at = $2, updated_at = $2 WHERE id = $1"
                    )
                    .bind(user.id)
                    .bind(now)
                    .execute(self.db)
                    .await?;

                    // Refetch with updated data
                    sqlx::query_as::<_, User>(&format!(
                        "SELECT {} FROM users WHERE id = $1",
                        User::SELECT_COLUMNS
                    ))
                    .bind(user.id)
                    .fetch_one(self.db)
                    .await?
                } else {
                    user
                }
            }
            None => {
                // Create developer user with super-admin role AND verified email
                let password_hash = self.hash_password("developer_bypass")?;

                sqlx::query_as::<_, User>(
                    &format!(r#"
                    INSERT INTO users (name, email, password, role, email_verified_at, created_at, updated_at)
                    VALUES ($1, $2, $3, 'super-admin', $4, $4, $4)
                    RETURNING {}
                    "#, User::SELECT_COLUMNS)
                )
                .bind("Developer")
                .bind(&email_lower)
                .bind(password_hash)
                .bind(now)
                .fetch_one(self.db)
                .await?
            }
        };

        tracing::info!(
            email = %email_lower,
            user_id = user.id,
            role = %user.role,
            verified = user.email_verified_at.is_some(),
            "Developer mode login successful - email verification bypassed"
        );
        let tokens = self.generate_tokens(&user)?;
        Ok((user, tokens))
    }

    pub async fn refresh_token(&self, refresh_token: &str) -> Result<(User, TokenPair), AppError> {
        let claims = self.decode_token(refresh_token)?;
        let user_id: i64 = claims.sub.parse().map_err(|_| AppError::InvalidToken)?;

        let user = sqlx::query_as::<_, User>(&format!(
            "SELECT {} FROM users WHERE id = $1",
            User::SELECT_COLUMNS
        ))
        .bind(user_id)
        .fetch_optional(self.db)
        .await?
        .ok_or(AppError::InvalidToken)?;

        let tokens = self.generate_tokens(&user)?;
        Ok((user, tokens))
    }

    pub async fn validate_token(&self, token: &str) -> Result<User, AppError> {
        let claims = self.decode_token(token)?;
        let user_id: i64 = claims.sub.parse().map_err(|_| AppError::InvalidToken)?;

        sqlx::query_as::<_, User>(&format!(
            "SELECT {} FROM users WHERE id = $1",
            User::SELECT_COLUMNS
        ))
        .bind(user_id)
        .fetch_optional(self.db)
        .await?
        .ok_or(AppError::InvalidToken)
    }

    pub async fn send_password_reset(&self, _email: &str) -> Result<(), AppError> {
        // TODO: Implement password reset email
        Ok(())
    }

    pub async fn reset_password(
        &self,
        _token: &str,
        _email: &str,
        _password: &str,
    ) -> Result<(), AppError> {
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
        argon2
            .hash_password(password.as_bytes(), &salt)
            .map(|h| h.to_string())
            .map_err(|e| AppError::Internal(format!("Password hashing error: {}", e)))
    }

    fn verify_password(&self, password: &str, hash: &str) -> Result<(), AppError> {
        let parsed_hash = PasswordHash::new(hash).map_err(|_| AppError::InvalidCredentials)?;
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
        )
        .map_err(|e| AppError::Internal(format!("JWT encoding error: {}", e)))?;

        let refresh_token = encode(
            &Header::default(),
            &refresh_claims,
            &EncodingKey::from_secret(self.jwt_config.secret.as_bytes()),
        )
        .map_err(|e| AppError::Internal(format!("JWT encoding error: {}", e)))?;

        Ok(TokenPair {
            access_token,
            refresh_token,
            expires_in,
        })
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
