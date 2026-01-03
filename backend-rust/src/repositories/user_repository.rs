//! User Repository
use sqlx::PgPool;
use uuid::Uuid;

use crate::{errors::AppError, models::User};

pub struct UserRepository<'a> {
    pool: &'a PgPool,
}

impl<'a> UserRepository<'a> {
    pub fn new(pool: &'a PgPool) -> Self {
        Self { pool }
    }

    pub async fn find(&self, id: Uuid) -> Result<Option<User>, AppError> {
        let user = sqlx::query_as::<_, User>(
            "SELECT * FROM users WHERE id = $1 AND deleted_at IS NULL"
        )
        .bind(id)
        .fetch_optional(self.pool)
        .await?;
        Ok(user)
    }

    pub async fn find_by_email(&self, email: &str) -> Result<Option<User>, AppError> {
        let user = sqlx::query_as::<_, User>(
            "SELECT * FROM users WHERE email = $1 AND deleted_at IS NULL"
        )
        .bind(email.to_lowercase())
        .fetch_optional(self.pool)
        .await?;
        Ok(user)
    }

    pub async fn email_exists(&self, email: &str, exclude_id: Option<Uuid>) -> Result<bool, AppError> {
        let exists: (bool,) = match exclude_id {
            Some(id) => {
                sqlx::query_as(
                    "SELECT EXISTS(SELECT 1 FROM users WHERE email = $1 AND id != $2 AND deleted_at IS NULL)"
                )
                .bind(email.to_lowercase())
                .bind(id)
                .fetch_one(self.pool)
                .await?
            }
            None => {
                sqlx::query_as(
                    "SELECT EXISTS(SELECT 1 FROM users WHERE email = $1 AND deleted_at IS NULL)"
                )
                .bind(email.to_lowercase())
                .fetch_one(self.pool)
                .await?
            }
        };
        Ok(exists.0)
    }
}
