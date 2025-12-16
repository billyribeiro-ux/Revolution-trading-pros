//! Utility functions for WASM compatibility
//!
//! Provides time functions that work in Cloudflare Workers WASM environment

use chrono::{DateTime, Utc};

/// Get current UTC timestamp (WASM compatible using js_sys)
pub fn now() -> DateTime<Utc> {
    let ms = js_sys::Date::now() as i64;
    let secs = ms / 1000;
    let nanos = ((ms % 1000) * 1_000_000) as u32;
    DateTime::from_timestamp(secs, nanos)
        .unwrap_or_else(|| DateTime::from_timestamp(0, 0).unwrap())
}

/// Get current timestamp as ISO 8601 string
pub fn now_iso() -> String {
    now().to_rfc3339()
}

/// Get current Unix timestamp in seconds
pub fn now_timestamp() -> i64 {
    (js_sys::Date::now() / 1000.0) as i64
}

/// Get current Unix timestamp in milliseconds
pub fn now_millis() -> i64 {
    js_sys::Date::now() as i64
}
