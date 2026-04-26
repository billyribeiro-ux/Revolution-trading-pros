//! Middleware
//! ICT 11+ Principal Engineer: Request processing middleware

pub mod admin;
pub mod auth;
pub mod content_type;
pub mod validation;
// FIX-2026-04-26 (Priority 2): ValidatedJson<T> extractor wired into module tree.
pub mod validated_json;

#[allow(unused_imports)]
pub use auth::*;
pub use content_type::*;
#[allow(unused_imports)]
pub use validated_json::ValidatedJson;
