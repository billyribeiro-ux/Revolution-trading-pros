//! Error types for the API

use serde::Serialize;
use worker::Response;

#[derive(Debug, thiserror::Error)]
pub enum ApiError {
    #[error("Not found: {0}")]
    NotFound(String),

    #[error("Unauthorized: {0}")]
    Unauthorized(String),

    #[error("Forbidden: {0}")]
    Forbidden(String),

    #[error("Bad request: {0}")]
    BadRequest(String),

    #[error("Validation error: {0}")]
    Validation(String),

    #[error("Conflict: {0}")]
    Conflict(String),

    #[error("Rate limited")]
    RateLimited,

    #[error("Internal error: {0}")]
    Internal(String),

    #[error("Database error: {0}")]
    Database(String),

    #[error("External service error: {0}")]
    ExternalService(String),
}

#[derive(Serialize)]
struct ErrorResponse {
    error: String,
    message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    details: Option<serde_json::Value>,
}

impl ApiError {
    pub fn to_response(&self) -> Response {
        let (status, error_type) = match self {
            ApiError::NotFound(_) => (404, "not_found"),
            ApiError::Unauthorized(_) => (401, "unauthorized"),
            ApiError::Forbidden(_) => (403, "forbidden"),
            ApiError::BadRequest(_) => (400, "bad_request"),
            ApiError::Validation(_) => (422, "validation_error"),
            ApiError::Conflict(_) => (409, "conflict"),
            ApiError::RateLimited => (429, "rate_limited"),
            ApiError::Internal(_) => (500, "internal_error"),
            ApiError::Database(_) => (500, "database_error"),
            ApiError::ExternalService(_) => (502, "external_service_error"),
        };

        let body = ErrorResponse {
            error: error_type.to_string(),
            message: self.to_string(),
            details: None,
        };

        Response::from_json(&body)
            .unwrap_or_else(|_| Response::error("Internal error", 500).unwrap())
            .with_status(status)
    }
}

impl From<ApiError> for Response {
    fn from(err: ApiError) -> Self {
        err.to_response()
    }
}

impl From<worker::Error> for ApiError {
    fn from(err: worker::Error) -> Self {
        ApiError::Internal(err.to_string())
    }
}

impl From<serde_json::Error> for ApiError {
    fn from(err: serde_json::Error) -> Self {
        ApiError::BadRequest(format!("JSON error: {}", err))
    }
}

impl From<jsonwebtoken::errors::Error> for ApiError {
    fn from(err: jsonwebtoken::errors::Error) -> Self {
        ApiError::Unauthorized(format!("Token error: {}", err))
    }
}
