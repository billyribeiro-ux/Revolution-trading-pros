//! TOTP (Time-based One-Time Password) Service
//!
//! WASM-compatible TOTP implementation following RFC 6238
//! Uses HMAC-SHA1 as per Google Authenticator standard
//!
//! @version 1.0.0
//! @author Revolution Trading Pros
//! @level ICT11+ Principal Engineer

use hmac::{Hmac, Mac};
use sha1::Sha1;
use base64::{Engine as _, engine::general_purpose::STANDARD as BASE64};
use crate::error::ApiError;

type HmacSha1 = Hmac<Sha1>;

/// TOTP time step in seconds (standard is 30)
const TIME_STEP: u64 = 30;

/// Number of digits in TOTP code (standard is 6)
const CODE_DIGITS: u32 = 6;

/// Window for time drift tolerance (allows codes from adjacent time steps)
const TIME_WINDOW: i64 = 1;

/// TOTP Service for MFA verification
pub struct TotpService;

impl TotpService {
    /// Verify a TOTP code against a secret
    ///
    /// # Arguments
    /// * `secret` - Base32 encoded secret key
    /// * `code` - 6-digit TOTP code to verify
    ///
    /// # Returns
    /// * `Ok(true)` if code is valid
    /// * `Ok(false)` if code is invalid
    /// * `Err` if there's a processing error
    pub fn verify(secret: &str, code: &str) -> Result<bool, ApiError> {
        // Validate code format
        if code.len() != 6 || !code.chars().all(|c| c.is_ascii_digit()) {
            return Ok(false);
        }

        let code_num: u32 = code.parse()
            .map_err(|_| ApiError::BadRequest("Invalid code format".to_string()))?;

        // Decode secret from base32
        let secret_bytes = Self::decode_base32(secret)
            .map_err(|e| ApiError::Internal(format!("Invalid secret encoding: {}", e)))?;

        // Get current time step
        let current_time = Self::get_current_time();
        let time_step = current_time / TIME_STEP;

        // Check current time step and adjacent ones (for clock drift)
        for offset in -TIME_WINDOW..=TIME_WINDOW {
            let check_step = (time_step as i64 + offset) as u64;
            let expected_code = Self::generate_totp(&secret_bytes, check_step)?;
            
            if expected_code == code_num {
                return Ok(true);
            }
        }

        Ok(false)
    }

    /// Generate a TOTP code for a given time step
    fn generate_totp(secret: &[u8], time_step: u64) -> Result<u32, ApiError> {
        // Convert time step to big-endian bytes
        let time_bytes = time_step.to_be_bytes();

        // Create HMAC-SHA1
        let mut mac = HmacSha1::new_from_slice(secret)
            .map_err(|e| ApiError::Internal(format!("HMAC error: {}", e)))?;
        mac.update(&time_bytes);
        let result = mac.finalize().into_bytes();

        // Dynamic truncation (RFC 4226)
        let offset = (result[19] & 0x0f) as usize;
        let binary = ((result[offset] & 0x7f) as u32) << 24
            | (result[offset + 1] as u32) << 16
            | (result[offset + 2] as u32) << 8
            | (result[offset + 3] as u32);

        // Generate 6-digit code
        let code = binary % 10u32.pow(CODE_DIGITS);

        Ok(code)
    }

    /// Decode base32 string to bytes
    /// Handles both padded and unpadded base32
    fn decode_base32(input: &str) -> Result<Vec<u8>, String> {
        const ALPHABET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ234567";

        // Remove padding and convert to uppercase
        let input = input.trim().to_uppercase().replace('=', "");
        
        if input.is_empty() {
            return Err("Empty input".to_string());
        }

        let mut bits = 0u64;
        let mut bit_count = 0;
        let mut output = Vec::new();

        for c in input.chars() {
            let value = ALPHABET.iter().position(|&x| x == c as u8)
                .ok_or_else(|| format!("Invalid base32 character: {}", c))?;

            bits = (bits << 5) | (value as u64);
            bit_count += 5;

            if bit_count >= 8 {
                bit_count -= 8;
                output.push((bits >> bit_count) as u8);
                bits &= (1 << bit_count) - 1;
            }
        }

        Ok(output)
    }

    /// Get current Unix timestamp in seconds
    /// Uses js_sys for WASM compatibility
    fn get_current_time() -> u64 {
        #[cfg(target_arch = "wasm32")]
        {
            (js_sys::Date::now() / 1000.0) as u64
        }
        #[cfg(not(target_arch = "wasm32"))]
        {
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs()
        }
    }

    /// Generate a new random secret (base32 encoded)
    /// Returns a 160-bit (20 byte) secret as recommended for TOTP
    pub fn generate_secret() -> String {
        const ALPHABET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ234567";
        
        let mut secret = String::with_capacity(32);
        
        // Generate 32 base32 characters (160 bits)
        for _ in 0..32 {
            let idx = Self::random_byte() as usize % 32;
            secret.push(ALPHABET[idx] as char);
        }
        
        secret
    }

    /// Generate a random byte using crypto-safe random
    fn random_byte() -> u8 {
        let mut buf = [0u8; 1];
        getrandom::getrandom(&mut buf).expect("Failed to generate random bytes");
        buf[0]
    }

    /// Verify a backup code against a list of hashed codes
    /// Returns the index of the matched code, or None if not found
    pub fn verify_backup_code(code: &str, hashed_codes: &[String]) -> Option<usize> {
        let code_normalized = code.trim().to_uppercase().replace(['-', ' '], "");
        
        for (idx, hashed) in hashed_codes.iter().enumerate() {
            // Use constant-time comparison to prevent timing attacks
            if Self::verify_backup_hash(&code_normalized, hashed) {
                return Some(idx);
            }
        }
        
        None
    }

    /// Verify a backup code against its hash
    fn verify_backup_hash(code: &str, hash: &str) -> bool {
        use sha2::{Sha256, Digest};
        
        let mut hasher = Sha256::new();
        hasher.update(code.as_bytes());
        let result = hasher.finalize();
        let computed_hash = hex::encode(result);
        
        // Constant-time comparison
        computed_hash.len() == hash.len() && 
            computed_hash.bytes().zip(hash.bytes()).fold(0u8, |acc, (a, b)| acc | (a ^ b)) == 0
    }

    /// Generate backup codes with their hashes
    /// Returns (plain_codes, hashed_codes)
    pub fn generate_backup_codes(count: usize) -> (Vec<String>, Vec<String>) {
        use sha2::{Sha256, Digest};
        
        let mut plain_codes = Vec::with_capacity(count);
        let mut hashed_codes = Vec::with_capacity(count);
        
        for _ in 0..count {
            // Generate 8-character alphanumeric code
            let code: String = (0..8)
                .map(|_| {
                    const CHARS: &[u8] = b"ABCDEFGHJKLMNPQRSTUVWXYZ23456789";
                    let idx = Self::random_byte() as usize % CHARS.len();
                    CHARS[idx] as char
                })
                .collect();
            
            // Format as XXXX-XXXX for readability
            let formatted = format!("{}-{}", &code[0..4], &code[4..8]);
            
            // Hash the normalized code
            let mut hasher = Sha256::new();
            hasher.update(code.as_bytes());
            let hash = hex::encode(hasher.finalize());
            
            plain_codes.push(formatted);
            hashed_codes.push(hash);
        }
        
        (plain_codes, hashed_codes)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_base32_decode() {
        // Test vector from RFC 4648
        let decoded = TotpService::decode_base32("JBSWY3DPEHPK3PXP").unwrap();
        assert_eq!(decoded, b"Hello!".to_vec());
    }

    #[test]
    fn test_generate_secret() {
        let secret = TotpService::generate_secret();
        assert_eq!(secret.len(), 32);
        assert!(secret.chars().all(|c| "ABCDEFGHIJKLMNOPQRSTUVWXYZ234567".contains(c)));
    }

    #[test]
    fn test_backup_codes() {
        let (plain, hashed) = TotpService::generate_backup_codes(10);
        assert_eq!(plain.len(), 10);
        assert_eq!(hashed.len(), 10);
        
        // Verify first code
        let code = plain[0].replace("-", "");
        let idx = TotpService::verify_backup_code(&code, &hashed);
        assert_eq!(idx, Some(0));
    }
}
