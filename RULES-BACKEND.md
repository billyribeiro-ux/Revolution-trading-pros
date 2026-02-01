# RULES-BACKEND.md
## Rust/Axum Backend Patterns

## ✅ VERIFICATION
```bash
cargo check                    # 0 errors
cargo clippy -- -D warnings    # 0 warnings
cargo build --release          # success
cargo test                     # all pass
```

## 🛣️ HANDLER PATTERN
```rust
use axum::{extract::{Path, State, Json}, http::StatusCode, response::IntoResponse};
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Deserialize, Validate)]
pub struct CreateItemRequest {
    #[validate(length(min = 1, max = 100))]
    name: String,
    #[validate(range(min = 0.01))]
    price: f64,
}

#[derive(Serialize)]
pub struct ItemResponse {
    id: uuid::Uuid,
    name: String,
    price: f64,
}

pub async fn create_item(
    State(state): State<AppState>,
    Json(payload): Json<CreateItemRequest>,
) -> Result<impl IntoResponse, AppError> {
    payload.validate()?;
    let item = state.db.create_item(&payload).await?;
    Ok((StatusCode::CREATED, Json(ItemResponse::from(item))))
}
```

## ⚠️ ERROR HANDLING
```rust
#[derive(Debug)]
pub enum AppError {
    NotFound,
    Unauthorized,
    BadRequest(String),
    Internal(anyhow::Error),
    Database(sqlx::Error),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, msg) = match self {
            AppError::NotFound => (StatusCode::NOT_FOUND, "Not found"),
            AppError::Unauthorized => (StatusCode::UNAUTHORIZED, "Unauthorized"),
            AppError::BadRequest(m) => (StatusCode::BAD_REQUEST, m.as_str()),
            AppError::Internal(e) => {
                tracing::error!("{:?}", e);
                (StatusCode::INTERNAL_SERVER_ERROR, "Internal error")
            }
            AppError::Database(e) => {
                tracing::error!("{:?}", e);
                (StatusCode::INTERNAL_SERVER_ERROR, "Database error")
            }
        };
        (status, Json(json!({ "error": msg }))).into_response()
    }
}

impl From<sqlx::Error> for AppError {
    fn from(e: sqlx::Error) -> Self { AppError::Database(e) }
}
```

## 🗄️ DATABASE - SQLx
```rust
pub async fn get_by_id(&self, id: Uuid) -> Result<Option<Item>, sqlx::Error> {
    sqlx::query_as!(
        Item,
        r#"SELECT id, name, price, created_at FROM items WHERE id = $1"#,
        id
    )
    .fetch_optional(&self.pool)
    .await
}

pub async fn list(&self, page: i64, limit: i64) -> Result<Vec<Item>, sqlx::Error> {
    sqlx::query_as!(
        Item,
        r#"SELECT * FROM items ORDER BY created_at DESC LIMIT $1 OFFSET $2"#,
        limit,
        (page - 1) * limit
    )
    .fetch_all(&self.pool)
    .await
}
```

## 🔐 AUTH MIDDLEWARE
```rust
pub struct AuthUser { pub id: Uuid, pub email: String }

#[async_trait]
impl<S: Send + Sync> FromRequestParts<S> for AuthUser {
    type Rejection = (StatusCode, &'static str);
    
    async fn from_request_parts(parts: &mut Parts, _: &S) -> Result<Self, Self::Rejection> {
        let token = parts.headers
            .get("Authorization")
            .and_then(|h| h.to_str().ok())
            .and_then(|h| h.strip_prefix("Bearer "))
            .ok_or((StatusCode::UNAUTHORIZED, "Missing token"))?;
            
        let claims = verify_jwt(token)
            .map_err(|_| (StatusCode::UNAUTHORIZED, "Invalid token"))?;
            
        Ok(AuthUser { id: claims.sub, email: claims.email })
    }
}
```

## 🧪 TESTING
```rust
#[tokio::test]
async fn test_create_item() {
    let server = TestServer::new(create_app().await).unwrap();
    
    let res = server.post("/api/items")
        .json(&json!({ "name": "Test", "price": 29.99 }))
        .await;
    
    res.assert_status_created();
    let body: ItemResponse = res.json();
    assert_eq!(body.name, "Test");
}
```

## 📋 CHECKLIST
```
□ cargo check passes
□ cargo clippy -D warnings passes
□ All handlers return Result<_, AppError>
□ All inputs validated
□ Proper error types
□ Auth on protected routes
```
