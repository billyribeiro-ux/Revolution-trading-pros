//! ValidatedJson extractor — wraps `axum::Json<T>` and invokes
//! `validator::Validate::validate()` after deserialization.
//!
//! FIX-2026-04-26 (Priority 2): Created as part of the validator wire-up audit.
//! 26 `#[validate(...)]` attributes existed across `models/*.rs` but `.validate()`
//! was invoked ZERO times across 216 handlers. This extractor closes that gap.
//!
//! ## Migration plan
//!
//! Handlers using `Json<T>` should incrementally migrate to `ValidatedJson<T>`:
//! ```rust,ignore
//! // Before
//! async fn handler(Json(input): Json<MyRequest>) { ... }
//! // After
//! use crate::middleware::validated_json::ValidatedJson;
//! async fn handler(ValidatedJson(input): ValidatedJson<MyRequest>) { ... }
//! ```
//!
//! The `T` type must derive both `serde::Deserialize` AND `validator::Validate`.
//! On validation failure the extractor returns HTTP 422 with a structured error body:
//!
//! ```json
//! { "message": "Validation failed", "errors": { "field": ["error msg", ...] } }
//! ```
//!
//! ## Hot-path proof point (Priority 2)
//!
//! Applied to:
//! - `routes/auth.rs::login`
//! - `routes/auth.rs::register`
//! - `routes/auth.rs::reset_password`
//!
//! Remaining 213 handlers should be migrated incrementally in follow-up sweeps.

use axum::{
    extract::{rejection::JsonRejection, FromRequest, Request},
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::de::DeserializeOwned;
use serde_json::json;
use std::collections::HashMap;
use validator::Validate;

/// Newtype wrapper that pairs `Json<T>` deserialization with
/// `validator::Validate::validate()` execution.
#[derive(Debug, Clone, Copy, Default)]
pub struct ValidatedJson<T>(pub T);

impl<S, T> FromRequest<S> for ValidatedJson<T>
where
    S: Send + Sync,
    T: DeserializeOwned + Validate + 'static,
    Json<T>: FromRequest<S, Rejection = JsonRejection>,
{
    type Rejection = ValidatedJsonRejection;

    #[allow(clippy::manual_async_fn)]
    fn from_request(
        req: Request,
        state: &S,
    ) -> impl std::future::Future<Output = Result<Self, Self::Rejection>> + Send {
        async move {
            // Step 1: standard JSON deserialization (delegates to axum::Json).
            let Json(value) = Json::<T>::from_request(req, state)
                .await
                .map_err(ValidatedJsonRejection::Json)?;

            // Step 2: invoke validator-derived validate().
            value
                .validate()
                .map_err(ValidatedJsonRejection::Validation)?;

            Ok(ValidatedJson(value))
        }
    }
}

/// Rejection type for `ValidatedJson` — separates JSON parse failure (400)
/// from validation failure (422) and serializes a structured error body.
#[derive(Debug)]
pub enum ValidatedJsonRejection {
    Json(JsonRejection),
    Validation(validator::ValidationErrors),
}

impl IntoResponse for ValidatedJsonRejection {
    fn into_response(self) -> Response {
        match self {
            ValidatedJsonRejection::Json(rej) => {
                let body = json!({
                    "message": "Invalid JSON request body",
                    "error": rej.body_text(),
                });
                (rej.status(), Json(body)).into_response()
            }
            ValidatedJsonRejection::Validation(errs) => {
                // Convert ValidationErrors → { field: [msg, ...] } map.
                let mut field_errors: HashMap<String, Vec<String>> = HashMap::new();
                for (field, errs) in errs.field_errors() {
                    let msgs: Vec<String> = errs
                        .iter()
                        .map(|e| {
                            e.message
                                .as_ref()
                                .map(|m| m.to_string())
                                .unwrap_or_else(|| e.code.to_string())
                        })
                        .collect();
                    field_errors.insert(field.to_string(), msgs);
                }

                let body = json!({
                    "message": "Validation failed",
                    "errors": field_errors,
                });
                (StatusCode::UNPROCESSABLE_ENTITY, Json(body)).into_response()
            }
        }
    }
}
