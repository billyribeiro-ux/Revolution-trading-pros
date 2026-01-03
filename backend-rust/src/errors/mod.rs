//! Application Error Handling
//!
//! ICT 11+ Principal Engineer Grade
//! Comprehensive error types with proper HTTP response mapping

use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;
use thiserror::Error;
use validator::ValidationErrors;

/// Main application error type
#[derive(Error, Debug)]
pub enum AppError {
    // =========================================================================
    // Authentication Errors (4xx)
    // =========================================================================
    #[error("Unauthorized: {0}")]
    Unauthorized(String),

    #[error("Forbidden: {0}")]
    Forbidden(String),

    #[error("Invalid credentials")]
    InvalidCredentials,

    #[error("Token expired")]
    TokenExpired,

    #[error("Invalid token")]
    InvalidToken,

    // =========================================================================
    // Resource Errors (4xx)
    // =========================================================================
    #[error("Not found: {0}")]
    NotFound(String),

    #[error("Conflict: {0}")]
    Conflict(String),

    #[error("Bad request: {0}")]
    BadRequest(String),

    // =========================================================================
    // Validation Errors (422)
    // =========================================================================
    #[error("Validation failed")]
    Validation(#[from] ValidationErrors),

    #[error("Validation error: {0}")]
    ValidationMessage(String),

    // =========================================================================
    // Database Errors (5xx)
    // =========================================================================
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),

    // =========================================================================
    // External Service Errors (5xx)
    // =========================================================================
    #[error("External service error: {0}")]
    ExternalService(String),

    #[error("Stripe error: {0}")]
    Stripe(String),

    // =========================================================================
    // Internal Errors (5xx)
    // =========================================================================
    #[error("Internal server error: {0}")]
    Internal(String),

    #[error(transparent)]
    Anyhow(#[from] anyhow::Error),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, error_message, details) = match &self {
            // Authentication errors
            AppError::Unauthorized(msg) => (
                StatusCode::UNAUTHORIZED,
                msg.clone(),
                None,
            ),
            AppError::Forbidden(msg) => (
                StatusCode::FORBIDDEN,
                msg.clone(),
                None,
            ),
            AppError::InvalidCredentials => (
                StatusCode::UNAUTHORIZED,
                "Invalid email or password".to_string(),
                None,
            ),
            AppError::TokenExpired => (
                StatusCode::UNAUTHORIZED,
                "Token has expired".to_string(),
                None,
            ),
            AppError::InvalidToken => (
                StatusCode::UNAUTHORIZED,
                "Invalid token".to_string(),
                None,
            ),

            // Resource errors
            AppError::NotFound(msg) => (
                StatusCode::NOT_FOUND,
                msg.clone(),
                None,
            ),
            AppError::Conflict(msg) => (
                StatusCode::CONFLICT,
                msg.clone(),
                None,
            ),
            AppError::BadRequest(msg) => (
                StatusCode::BAD_REQUEST,
                msg.clone(),
                None,
            ),

            // Validation errors
            AppError::Validation(errors) => (
                StatusCode::UNPROCESSABLE_ENTITY,
                "Validation failed".to_string(),
                Some(format_validation_errors(errors)),
            ),
            AppError::ValidationMessage(msg) => (
                StatusCode::UNPROCESSABLE_ENTITY,
                msg.clone(),
                None,
            ),

            // Database errors - log but don't expose internals
            AppError::Database(e) => {
                tracing::error!("Database error: {:?}", e);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "A database error occurred".to_string(),
                    None,
                )
            }

            // External service errors
            AppError::ExternalService(msg) => {
                tracing::error!("External service error: {}", msg);
                (
                    StatusCode::BAD_GATEWAY,
                    "External service unavailable".to_string(),
                    None,
                )
            }
            AppError::Stripe(msg) => {
                tracing::error!("Stripe error: {}", msg);
                (
                    StatusCode::BAD_REQUEST,
                    format!("Payment error: {}", msg),
                    None,
                )
            }

            // Internal errors - log but don't expose internals
            AppError::Internal(msg) => {
                tracing::error!("Internal error: {}", msg);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "An internal error occurred".to_string(),
                    None,
                )
            }
            AppError::Anyhow(e) => {
                tracing::error!("Unexpected error: {:?}", e);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "An unexpected error occurred".to_string(),
                    None,
                )
            }
        };

        let body = match details {
            Some(d) => json!({
                "success": false,
                "message": error_message,
                "errors": d,
            }),
            None => json!({
                "success": false,
                "message": error_message,
            }),
        };

        (status, Json(body)).into_response()
    }
}

/// Format validation errors to match Laravel's format for frontend compatibility
fn format_validation_errors(errors: &ValidationErrors) -> serde_json::Value {
    let mut error_map = serde_json::Map::new();

    for (field, field_errors) in errors.field_errors() {
        let messages: Vec<String> = field_errors
            .iter()
            .map(|e| {
                e.message
                    .clone()
                    .map(|m| m.to_string())
                    .unwrap_or_else(|| format!("Invalid value for {}", field))
            })
            .collect();

        error_map.insert(field.to_string(), json!(messages));
    }

    serde_json::Value::Object(error_map)
}

/// Result type alias for handlers
pub type HandlerResult<T> = Result<T, AppError>;
