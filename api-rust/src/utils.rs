//! Utility functions for WASM-compatible operations

use chrono::{DateTime, NaiveDateTime, Utc};
use serde::{Deserialize, Deserializer};

/// Get current time as DateTime<Utc> (WASM-compatible)
pub fn now() -> DateTime<Utc> {
    let timestamp_millis = js_sys::Date::now() as i64;
    let naive = NaiveDateTime::from_timestamp_millis(timestamp_millis).unwrap_or_default();
    DateTime::from_naive_utc_and_offset(naive, Utc)
}

/// Get current time as ISO 8601 string (WASM-compatible)
pub fn now_iso() -> String {
    let date = js_sys::Date::new_0();
    date.to_iso_string().as_string().unwrap_or_default()
}

/// Get current Unix timestamp in seconds (WASM-compatible)
pub fn now_timestamp() -> i64 {
    (js_sys::Date::now() / 1000.0) as i64
}

/// Get current Unix timestamp in milliseconds (WASM-compatible)
pub fn now_millis() -> i64 {
    js_sys::Date::now() as i64
}

/// Deserialize DateTime from Postgres timestamp format
pub fn deserialize_datetime<'de, D>(deserializer: D) -> Result<DateTime<Utc>, D::Error>
where
    D: Deserializer<'de>,
{
    use serde::de::Error;
    let s: String = String::deserialize(deserializer)?;
    
    // Try parsing various formats
    if let Ok(dt) = DateTime::parse_from_rfc3339(&s) {
        return Ok(dt.with_timezone(&Utc));
    }
    
    // Postgres format with milliseconds: "2025-12-17 11:07:48.179+00"
    if let Ok(dt) = DateTime::parse_from_str(&s, "%Y-%m-%d %H:%M:%S%.f%#z") {
        return Ok(dt.with_timezone(&Utc));
    }
    
    // Postgres format without milliseconds: "2025-12-06 13:37:36+00"
    if let Ok(dt) = DateTime::parse_from_str(&s, "%Y-%m-%d %H:%M:%S%#z") {
        return Ok(dt.with_timezone(&Utc));
    }
    
    // Without timezone but with milliseconds
    if let Ok(naive) = NaiveDateTime::parse_from_str(&s, "%Y-%m-%d %H:%M:%S%.f") {
        return Ok(DateTime::from_naive_utc_and_offset(naive, Utc));
    }
    
    // Without timezone
    if let Ok(naive) = NaiveDateTime::parse_from_str(&s, "%Y-%m-%d %H:%M:%S") {
        return Ok(DateTime::from_naive_utc_and_offset(naive, Utc));
    }
    
    Err(Error::custom(format!("Cannot parse datetime: {}", s)))
}

/// Deserialize optional DateTime from Postgres timestamp format
pub fn deserialize_option_datetime<'de, D>(deserializer: D) -> Result<Option<DateTime<Utc>>, D::Error>
where
    D: Deserializer<'de>,
{
    use serde::de::Error;
    
    let opt: Option<String> = Option::deserialize(deserializer)?;
    match opt {
        None => Ok(None),
        Some(s) if s.is_empty() => Ok(None),
        Some(s) => {
            if let Ok(dt) = DateTime::parse_from_rfc3339(&s) {
                return Ok(Some(dt.with_timezone(&Utc)));
            }
            if let Ok(dt) = DateTime::parse_from_str(&s, "%Y-%m-%d %H:%M:%S%.f%#z") {
                return Ok(Some(dt.with_timezone(&Utc)));
            }
            if let Ok(dt) = DateTime::parse_from_str(&s, "%Y-%m-%d %H:%M:%S%#z") {
                return Ok(Some(dt.with_timezone(&Utc)));
            }
            if let Ok(naive) = NaiveDateTime::parse_from_str(&s, "%Y-%m-%d %H:%M:%S%.f") {
                return Ok(Some(DateTime::from_naive_utc_and_offset(naive, Utc)));
            }
            if let Ok(naive) = NaiveDateTime::parse_from_str(&s, "%Y-%m-%d %H:%M:%S") {
                return Ok(Some(DateTime::from_naive_utc_and_offset(naive, Utc)));
            }
            Err(Error::custom(format!("Cannot parse datetime: {}", s)))
        }
    }
}

/// Deserialize i64 from either number or string (Neon returns BIGINT as string)
pub fn deserialize_i64_from_string<'de, D>(deserializer: D) -> Result<i64, D::Error>
where
    D: Deserializer<'de>,
{
    use serde::de::Error;
    
    #[derive(Deserialize)]
    #[serde(untagged)]
    enum StringOrInt {
        String(String),
        Int(i64),
    }
    
    match StringOrInt::deserialize(deserializer)? {
        StringOrInt::String(s) => s.parse::<i64>().map_err(Error::custom),
        StringOrInt::Int(i) => Ok(i),
    }
}

/// Deserialize Option<i64> from either number or string
pub fn deserialize_option_i64_from_string<'de, D>(deserializer: D) -> Result<Option<i64>, D::Error>
where
    D: Deserializer<'de>,
{
    use serde::de::Error;
    
    #[derive(Deserialize)]
    #[serde(untagged)]
    enum StringOrIntOrNull {
        Null,
        String(String),
        Int(i64),
    }
    
    match Option::<StringOrIntOrNull>::deserialize(deserializer)? {
        None => Ok(None),
        Some(StringOrIntOrNull::Null) => Ok(None),
        Some(StringOrIntOrNull::String(s)) if s.is_empty() => Ok(None),
        Some(StringOrIntOrNull::String(s)) => s.parse::<i64>().map(Some).map_err(Error::custom),
        Some(StringOrIntOrNull::Int(i)) => Ok(Some(i)),
    }
}
