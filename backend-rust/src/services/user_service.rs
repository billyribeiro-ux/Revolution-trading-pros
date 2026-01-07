//! User Service
use sqlx::PgPool;
use uuid::Uuid;

use crate::{errors::AppError, models::{User, UserMembership, UserProduct}};

pub struct UserService<'a> {
    db: &'a PgPool,
}

impl<'a> UserService<'a> {
    pub fn new(db: &'a PgPool) -> Self {
        Self { db }
    }

    pub async fn find_by_id(&self, id: Uuid) -> Result<Option<User>, AppError> {
        let user = sqlx::query_as::<_, User>(&format!("SELECT {} FROM users WHERE id = $1", User::SELECT_COLUMNS))
            .bind(id)
            .fetch_optional(self.db)
            .await?;
        Ok(user)
    }

    pub async fn update_profile(
        &self,
        id: Uuid,
        name: Option<&str>,
        first_name: Option<&str>,
        last_name: Option<&str>,
        avatar_url: Option<&str>,
    ) -> Result<User, AppError> {
        let now = chrono::Utc::now();
        let user = sqlx::query_as::<_, User>(
            &format!(r#"
            UPDATE users SET
                name = COALESCE($2, name),
                first_name = COALESCE($3, first_name),
                last_name = COALESCE($4, last_name),
                avatar_url = COALESCE($5, avatar_url),
                updated_at = $6
            WHERE id = $1
            RETURNING {}
            "#, User::SELECT_COLUMNS)
        )
        .bind(id)
        .bind(name)
        .bind(first_name)
        .bind(last_name)
        .bind(avatar_url)
        .bind(now)
        .fetch_optional(self.db)
        .await?
        .ok_or(AppError::NotFound("User not found".to_string()))?;

        Ok(user)
    }

    pub async fn get_memberships(&self, user_id: i64) -> Result<Vec<UserMembership>, AppError> {
        // TODO: Implement actual membership query
        let _ = user_id;
        Ok(vec![])
    }

    pub async fn get_products(&self, user_id: i64) -> Result<Vec<UserProduct>, AppError> {
        // TODO: Implement actual products query
        let _ = user_id;
        Ok(vec![])
    }

    pub async fn get_indicators(&self, user_id: i64) -> Result<Vec<UserProduct>, AppError> {
        // TODO: Implement actual indicators query
        let _ = user_id;
        Ok(vec![])
    }
}
