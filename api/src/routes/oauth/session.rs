//! OAuth callback Response builder — sets httpOnly cookies + redirects.
//!
//! FIX-C-1 (2026-04-29) extracted: builds a Response that sets the access
//! token, refresh token, and session ID as httpOnly cookies, and redirects
//! to the frontend `/auth/callback` page WITHOUT putting credentials in the
//! URL.
//!
//! Why: previously the OAuth callbacks redirected to
//!   /auth/callback?provider=google&token=<JWT>&refresh_token=<JWT>&session_id=...
//! which leaked full JWTs into Cloudflare access logs, browser history,
//! and any third-party Referer headers. Cookies set on the redirect
//! Response are sent with the immediately-following GET to /auth/callback
//! because they are scoped to the same site, so the frontend has full
//! access to the session via cookie + the existing `GET /me` flow.
//!
//! Cookie attributes:
//!   - HttpOnly: not readable from JavaScript (XSS isolation).
//!   - Secure: only sent over HTTPS in production. We omit Secure in
//!     non-production so local-dev OAuth (http://localhost:5173) keeps
//!     working.
//!   - SameSite=Lax: required for the cookie to be sent on the top-level
//!     redirect from the OAuth provider back to our callback.
//!   - Path=/: visible to every route on the frontend origin.
//!   - Max-Age: matches token TTL.
//!
//! Cookie names (`rtp_access_token`, `rtp_refresh_token`, `rtp_session_id`)
//! match what `frontend/src/hooks.server.ts` reads. Renaming here without
//! the matching frontend change silently breaks the post-OAuth session.

use axum::{
    http::HeaderValue,
    response::{IntoResponse, Redirect, Response},
};

use crate::{models::AuthResponse, AppState};

/// Build a redirect Response with the OAuth session set in httpOnly cookies.
pub(super) fn oauth_callback_response_with_cookies(
    state: &AppState,
    auth: &AuthResponse,
    provider: &str,
) -> Response {
    let app_url = state.config.app_url.trim_end_matches('/');
    // No tokens in the URL — only the provider name (UX, so the callback
    // page can show "Signed in with Google" if it wants).
    let redirect_to = format!("{app_url}/auth/callback?provider={provider}");

    // Cookies use the SAME names hooks.server.ts already reads
    // (rtp_access_token, rtp_refresh_token, rtp_session_id) so this
    // change is transparent to the frontend.
    let secure_attr = if state.config.is_production() {
        "; Secure"
    } else {
        ""
    };
    let access_max_age = state.config.jwt_expires_in * 3600; // hours -> seconds
    let refresh_max_age: i64 = 7 * 24 * 3600; // 7 days

    let access_cookie = format!(
        "rtp_access_token={}; HttpOnly; SameSite=Lax; Path=/; Max-Age={}{}",
        auth.access_token, access_max_age, secure_attr
    );
    let refresh_cookie = format!(
        "rtp_refresh_token={}; HttpOnly; SameSite=Lax; Path=/; Max-Age={}{}",
        auth.refresh_token, refresh_max_age, secure_attr
    );
    let session_cookie = format!(
        "rtp_session_id={}; HttpOnly; SameSite=Lax; Path=/; Max-Age={}{}",
        auth.session_id, refresh_max_age, secure_attr
    );

    let mut response = Redirect::to(&redirect_to).into_response();
    let headers = response.headers_mut();
    if let Ok(v) = HeaderValue::from_str(&access_cookie) {
        headers.append(axum::http::header::SET_COOKIE, v);
    }
    if let Ok(v) = HeaderValue::from_str(&refresh_cookie) {
        headers.append(axum::http::header::SET_COOKIE, v);
    }
    if let Ok(v) = HeaderValue::from_str(&session_cookie) {
        headers.append(axum::http::header::SET_COOKIE, v);
    }
    response
}
