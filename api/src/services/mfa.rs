//! RFC TOTP primitives and retained wire types.
//! Durable MFA workflows and one-use credential storage live in account_mfa.

use base32::Alphabet;
use chrono::{DateTime, Utc};
use rand::RngExt;
use serde::{Deserialize, Serialize};

use crate::utils::errors::ApiError;

/// TOTP configuration
const TOTP_DIGITS: u32 = 6;
const TOTP_PERIOD: u64 = 30;
const TOTP_ALGORITHM: &str = "SHA1";
const BACKUP_CODE_COUNT: usize = 10;
const BACKUP_CODE_LENGTH: usize = 8;

/// MFA secret stored in database
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct MfaSecret {
    pub id: i64,
    pub user_id: i64,
    pub totp_secret: String,
    pub totp_verified_at: Option<DateTime<Utc>>,
    pub backup_codes: serde_json::Value,
    pub backup_codes_generated_at: Option<DateTime<Utc>>,
    pub backup_codes_used: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// MFA setup response
#[derive(Debug, Serialize)]
pub struct MfaSetupResponse {
    pub secret: String,
    pub qr_code_uri: String,
    pub backup_codes: Vec<String>,
}

/// MFA verification result
#[derive(Debug, Serialize)]
pub struct MfaVerifyResult {
    pub success: bool,
    pub method: String, // "totp" or "backup"
    pub backup_codes_remaining: Option<i32>,
}

/// Generate a random TOTP secret (20 bytes = 160 bits)
pub fn generate_totp_secret() -> String {
    let mut rng = rand::rng();
    let secret: Vec<u8> = (0..20).map(|_| rng.random()).collect();
    base32::encode(Alphabet::Rfc4648 { padding: false }, &secret)
}

/// Generate backup codes
pub fn generate_backup_codes() -> Vec<String> {
    let mut rng = rand::rng();
    (0..BACKUP_CODE_COUNT)
        .map(|_| {
            let code: String = (0..BACKUP_CODE_LENGTH)
                .map(|_| {
                    let idx = rng.random_range(0..36);
                    if idx < 10 {
                        (b'0' + idx) as char
                    } else {
                        (b'A' + idx - 10) as char
                    }
                })
                .collect();
            // Format as XXXX-XXXX for readability
            format!("{}-{}", &code[0..4], &code[4..8])
        })
        .collect()
}

/// Generate TOTP code for a given secret and time
pub fn generate_totp(secret: &str, time: u64) -> Result<String, ApiError> {
    let secret_bytes = base32::decode(Alphabet::Rfc4648 { padding: false }, secret)
        .ok_or_else(|| ApiError::internal_error("Invalid TOTP secret"))?;

    let counter = time / TOTP_PERIOD;
    let counter_bytes = counter.to_be_bytes();

    // HMAC-SHA1
    use hmac::{Hmac, KeyInit, Mac};
    use sha1::Sha1;

    type HmacSha1 = Hmac<Sha1>;
    let mut mac = HmacSha1::new_from_slice(&secret_bytes)
        .map_err(|_| ApiError::internal_error("HMAC error"))?;
    mac.update(&counter_bytes);
    let result = mac.finalize().into_bytes();

    // Dynamic truncation
    let offset = (result[19] & 0x0f) as usize;
    let code = ((result[offset] & 0x7f) as u32) << 24
        | (result[offset + 1] as u32) << 16
        | (result[offset + 2] as u32) << 8
        | (result[offset + 3] as u32);

    let otp = code % 10u32.pow(TOTP_DIGITS);
    Ok(format!("{:0width$}", otp, width = TOTP_DIGITS as usize))
}

/// Verify TOTP code (allows 1 period drift)
pub fn verify_totp(secret: &str, code: &str) -> Result<bool, ApiError> {
    Ok(verify_totp_timestep(secret, code)?.is_some())
}

/// Like [`verify_totp`], but on a match returns the accepted TOTP time-step
/// counter (Unix time / `TOTP_PERIOD`) so the caller can enforce single-use
/// replay protection (RUST_DEEP_AUDIT P2-2 / RFC 6238 §5.2). Returns `None`
/// when the code matches no period in the ±1 drift window.
pub fn verify_totp_timestep(secret: &str, code: &str) -> Result<Option<i64>, ApiError> {
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map_err(|_| ApiError::internal_error("Time error"))?
        .as_secs();

    // Check current period and one before/after (30 second drift tolerance)
    for offset in [-1i64, 0, 1] {
        let check_time = (now as i64 + offset * TOTP_PERIOD as i64) as u64;
        let expected = generate_totp(secret, check_time)?;
        if constant_time_eq(code.as_bytes(), expected.as_bytes()) {
            return Ok(Some((check_time / TOTP_PERIOD) as i64));
        }
    }

    Ok(None)
}

/// Constant-time string comparison to prevent timing attacks
fn constant_time_eq(a: &[u8], b: &[u8]) -> bool {
    if a.len() != b.len() {
        return false;
    }
    let mut result = 0u8;
    for (x, y) in a.iter().zip(b.iter()) {
        result |= x ^ y;
    }
    result == 0
}

/// Generate otpauth:// URI for QR code
pub fn generate_qr_uri(secret: &str, email: &str, issuer: &str) -> String {
    format!(
        "otpauth://totp/{}:{}?secret={}&issuer={}&algorithm={}&digits={}&period={}",
        urlencoding::encode(issuer),
        urlencoding::encode(email),
        secret,
        urlencoding::encode(issuer),
        TOTP_ALGORITHM,
        TOTP_DIGITS,
        TOTP_PERIOD
    )
}
