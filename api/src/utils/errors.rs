//! Standardized Error Response Utilities
//! ICT 11+ Principal Engineer: Consistent error handling across all routes
//!
//! This module provides a unified error response format that:
//! - Ensures consistent JSON structure for all errors
//! - Includes request tracking for debugging
//! - Supports validation errors with field-level details
//! - Provides security-conscious error messages

#![allow(dead_code)]

use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::Serialize;
use std::collections::HashMap;

/// Standardized API error response
/// All errors across the API use this consistent format
#[derive(Debug, Serialize)]
pub struct ApiError {
    /// Human-readable error message
    pub message: String,

    /// HTTP status code as string (e.g., "404", "500")
    pub status: String,

    /// Error code for client-side handling (e.g., "USER_NOT_FOUND", "VALIDATION_ERROR")
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,

    /// Field-level validation errors
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errors: Option<HashMap<String, Vec<String>>>,

    /// Request ID for debugging/support
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,

    /// Timestamp of the error
    pub timestamp: String,
}

impl ApiError {
    /// Create a new API error
    pub fn new(status: StatusCode, message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
            status: status.as_u16().to_string(),
            code: None,
            errors: None,
            request_id: None,
            timestamp: chrono::Utc::now().to_rfc3339(),
        }
    }

    /// Add an error code
    pub fn with_code(mut self, code: impl Into<String>) -> Self {
        self.code = Some(code.into());
        self
    }

    /// Add validation errors
    pub fn with_errors(mut self, errors: HashMap<String, Vec<String>>) -> Self {
        self.errors = Some(errors);
        self
    }

    /// Add a single field error
    pub fn with_field_error(mut self, field: impl Into<String>, error: impl Into<String>) -> Self {
        let mut errors = self.errors.unwrap_or_default();
        errors.entry(field.into()).or_default().push(error.into());
        self.errors = Some(errors);
        self
    }

    /// Add request ID for tracking
    pub fn with_request_id(mut self, request_id: impl Into<String>) -> Self {
        self.request_id = Some(request_id.into());
        self
    }

    /// Create a database error
    pub fn database_error(message: &str) -> Self {
        Self::new(StatusCode::INTERNAL_SERVER_ERROR, message)
    }

    /// Create a not found error
    pub fn not_found(message: &str) -> Self {
        Self::new(StatusCode::NOT_FOUND, message)
    }

    /// Create a validation error
    pub fn validation_error(message: &str) -> Self {
        Self::new(StatusCode::BAD_REQUEST, message)
    }

    /// Create an internal server error
    pub fn internal_error(message: &str) -> Self {
        Self::new(StatusCode::INTERNAL_SERVER_ERROR, message)
    }

    /// Convert to Axum response tuple
    pub fn into_response_tuple(self) -> (StatusCode, Json<Self>) {
        let status = StatusCode::from_u16(self.status.parse().unwrap_or(500))
            .unwrap_or(StatusCode::INTERNAL_SERVER_ERROR);
        (status, Json(self))
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let status = StatusCode::from_u16(self.status.parse().unwrap_or(500))
            .unwrap_or(StatusCode::INTERNAL_SERVER_ERROR);
        (status, Json(self)).into_response()
    }
}

/// Result type alias for API handlers
pub type ApiResult<T> = Result<T, ApiError>;

// ═══════════════════════════════════════════════════════════════════════════
// Convenience constructors for common errors
// ═══════════════════════════════════════════════════════════════════════════

/// 400 Bad Request
pub fn bad_request(message: impl Into<String>) -> ApiError {
    ApiError::new(StatusCode::BAD_REQUEST, message).with_code("BAD_REQUEST")
}

/// 401 Unauthorized
pub fn unauthorized(message: impl Into<String>) -> ApiError {
    ApiError::new(StatusCode::UNAUTHORIZED, message).with_code("UNAUTHORIZED")
}

/// 403 Forbidden
pub fn forbidden(message: impl Into<String>) -> ApiError {
    ApiError::new(StatusCode::FORBIDDEN, message).with_code("FORBIDDEN")
}

/// 404 Not Found
pub fn not_found(resource: impl Into<String>) -> ApiError {
    ApiError::new(
        StatusCode::NOT_FOUND,
        format!("{} not found", resource.into()),
    )
    .with_code("NOT_FOUND")
}

/// 409 Conflict
pub fn conflict(message: impl Into<String>) -> ApiError {
    ApiError::new(StatusCode::CONFLICT, message).with_code("CONFLICT")
}

/// 422 Validation Error
pub fn validation_error(message: impl Into<String>) -> ApiError {
    ApiError::new(StatusCode::UNPROCESSABLE_ENTITY, message).with_code("VALIDATION_ERROR")
}

/// 429 Too Many Requests
pub fn rate_limited(retry_after: Option<i64>) -> ApiError {
    let mut error = ApiError::new(
        StatusCode::TOO_MANY_REQUESTS,
        "Too many requests. Please try again later.",
    )
    .with_code("RATE_LIMITED");

    if let Some(seconds) = retry_after {
        error.message = format!(
            "Too many requests. Please try again in {} seconds.",
            seconds
        );
    }

    error
}

/// 500 Internal Server Error
pub fn internal_error(message: impl Into<String>) -> ApiError {
    // ICT 11+ Security: Don't expose internal details in production
    let msg = message.into();
    let safe_message = if std::env::var("ENVIRONMENT").unwrap_or_default() == "production" {
        "An internal error occurred. Please try again later.".to_string()
    } else {
        msg.clone()
    };

    // Always log the full error
    tracing::error!("Internal error: {}", msg);

    ApiError::new(StatusCode::INTERNAL_SERVER_ERROR, safe_message).with_code("INTERNAL_ERROR")
}

/// 503 Service Unavailable
pub fn service_unavailable(message: impl Into<String>) -> ApiError {
    ApiError::new(StatusCode::SERVICE_UNAVAILABLE, message).with_code("SERVICE_UNAVAILABLE")
}

// ═══════════════════════════════════════════════════════════════════════════
// Conversion implementations
// ═══════════════════════════════════════════════════════════════════════════

impl From<sqlx::Error> for ApiError {
    fn from(err: sqlx::Error) -> Self {
        tracing::error!("Database error: {:?}", err);

        match &err {
            sqlx::Error::RowNotFound => not_found("Resource"),
            sqlx::Error::Database(db_err) => {
                // Check for unique constraint violations
                if db_err.code().map(|c| c == "23505").unwrap_or(false) {
                    conflict("A record with this value already exists")
                } else {
                    internal_error("Database error")
                }
            }
            _ => internal_error("Database error"),
        }
    }
}

impl From<anyhow::Error> for ApiError {
    fn from(err: anyhow::Error) -> Self {
        tracing::error!("Error: {:?}", err);
        internal_error(err.to_string())
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// Validation helpers
// ═══════════════════════════════════════════════════════════════════════════

/// Validate that a field is not empty
pub fn validate_not_empty(field: &str, value: &str) -> Result<(), (String, String)> {
    if value.trim().is_empty() {
        Err((field.to_string(), format!("{} is required", field)))
    } else {
        Ok(())
    }
}

/// Validate email format
pub fn validate_email(email: &str) -> Result<(), (String, String)> {
    if !email.contains('@') || !email.contains('.') {
        Err(("email".to_string(), "Invalid email format".to_string()))
    } else {
        Ok(())
    }
}

/// Validate minimum length
pub fn validate_min_length(field: &str, value: &str, min: usize) -> Result<(), (String, String)> {
    if value.len() < min {
        Err((
            field.to_string(),
            format!("{} must be at least {} characters", field, min),
        ))
    } else {
        Ok(())
    }
}

/// Validate maximum length
pub fn validate_max_length(field: &str, value: &str, max: usize) -> Result<(), (String, String)> {
    if value.len() > max {
        Err((
            field.to_string(),
            format!("{} must be no more than {} characters", field, max),
        ))
    } else {
        Ok(())
    }
}

/// Collect validation errors and return ApiError if any
#[allow(clippy::result_large_err)]
pub fn collect_validation_errors(
    validations: Vec<Result<(), (String, String)>>,
) -> Result<(), ApiError> {
    let errors: Vec<(String, String)> = validations.into_iter().filter_map(|r| r.err()).collect();

    if errors.is_empty() {
        Ok(())
    } else {
        let mut error = validation_error("Validation failed");
        for (field, message) in errors {
            error = error.with_field_error(field, message);
        }
        Err(error)
    }
}
