//! OAuth public DTOs.
//!
//! These types form the public API of the `oauth` module — `oauth_test.rs`
//! pins their wire shapes (`OAuthProvider` Display + serde rename,
//! `OAuthInitResponse` JSON, `GoogleCallbackQuery` / `AppleCallbackBody`
//! field names). Do not rename or restructure without updating that test.

use serde::{Deserialize, Serialize};

// =============================================================================
// Configuration & Constants
// =============================================================================

/// OAuth provider types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum OAuthProvider {
    Google,
    Apple,
}

impl std::fmt::Display for OAuthProvider {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OAuthProvider::Google => write!(f, "google"),
            OAuthProvider::Apple => write!(f, "apple"),
        }
    }
}

// =============================================================================
// Request/Response Types
// =============================================================================

/// OAuth initiation response
#[derive(Debug, Serialize)]
pub struct OAuthInitResponse {
    pub authorization_url: String,
    pub state: String,
}

/// Google OAuth callback query params
#[derive(Debug, Deserialize)]
pub struct GoogleCallbackQuery {
    pub code: Option<String>,
    pub state: Option<String>,
    pub error: Option<String>,
    pub error_description: Option<String>,
}

/// Apple OAuth callback (POST body from form_post)
#[derive(Debug, Deserialize)]
pub struct AppleCallbackBody {
    pub code: Option<String>,
    pub id_token: Option<String>,
    pub state: Option<String>,
    pub user: Option<String>, // JSON string with user info (first auth only)
    pub error: Option<String>,
}
