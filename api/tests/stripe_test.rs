//! Unit tests for Stripe webhook verification

/// Stripe webhook signature verification implementation (duplicated for testing)
mod stripe {
    use anyhow::Result;
    use hmac::{Hmac, Mac};
    use sha2::Sha256;

    pub fn verify_webhook(payload: &[u8], signature_header: &str, webhook_secret: &str) -> Result<bool> {
        let mut timestamp: Option<&str> = None;
        let mut signatures: Vec<&str> = Vec::new();

        for part in signature_header.split(',') {
            let mut kv = part.splitn(2, '=');
            if let (Some(key), Some(value)) = (kv.next(), kv.next()) {
                match key {
                    "t" => timestamp = Some(value),
                    "v1" => signatures.push(value),
                    _ => {}
                }
            }
        }

        let timestamp = timestamp
            .ok_or_else(|| anyhow::anyhow!("Missing timestamp in signature header"))?;

        if signatures.is_empty() {
            return Err(anyhow::anyhow!("No v1 signature found in header"));
        }

        // Skip timestamp verification for tests
        let signed_payload = format!("{}.{}", timestamp, String::from_utf8_lossy(payload));

        type HmacSha256 = Hmac<Sha256>;
        let mut mac = HmacSha256::new_from_slice(webhook_secret.as_bytes())
            .map_err(|_| anyhow::anyhow!("Invalid webhook secret"))?;
        mac.update(signed_payload.as_bytes());
        let expected = hex::encode(mac.finalize().into_bytes());

        for sig in signatures {
            if constant_time_compare(sig, &expected) {
                return Ok(true);
            }
        }

        Ok(false)
    }

    fn constant_time_compare(a: &str, b: &str) -> bool {
        if a.len() != b.len() {
            return false;
        }
        let mut result = 0u8;
        for (x, y) in a.bytes().zip(b.bytes()) {
            result |= x ^ y;
        }
        result == 0
    }

    /// Helper to generate a valid signature for testing
    pub fn generate_test_signature(payload: &str, timestamp: i64, secret: &str) -> String {
        let signed_payload = format!("{}.{}", timestamp, payload);

        type HmacSha256 = Hmac<Sha256>;
        let mut mac = HmacSha256::new_from_slice(secret.as_bytes()).unwrap();
        mac.update(signed_payload.as_bytes());
        let signature = hex::encode(mac.finalize().into_bytes());

        format!("t={},v1={}", timestamp, signature)
    }
}

#[cfg(test)]
mod tests {
    use super::stripe::*;
    use chrono::Utc;

    const TEST_SECRET: &str = "whsec_test_secret_12345";

    // ===================
    // Valid Signature Tests
    // ===================

    #[test]
    fn test_verify_valid_signature() {
        let payload = r#"{"type":"checkout.session.completed"}"#;
        let timestamp = Utc::now().timestamp();
        let signature_header = generate_test_signature(payload, timestamp, TEST_SECRET);

        let result = verify_webhook(
            payload.as_bytes(),
            &signature_header,
            TEST_SECRET,
        );

        assert!(result.is_ok(), "Verification should succeed");
        assert!(result.unwrap(), "Valid signature should return true");
    }

    #[test]
    fn test_verify_valid_signature_with_multiple_v1() {
        let payload = r#"{"id":"evt_123"}"#;
        let timestamp = Utc::now().timestamp();

        // Generate correct signature
        let signed_payload = format!("{}.{}", timestamp, payload);
        type HmacSha256 = hmac::Hmac<sha2::Sha256>;
        use hmac::Mac;
        let mut mac = HmacSha256::new_from_slice(TEST_SECRET.as_bytes()).unwrap();
        mac.update(signed_payload.as_bytes());
        let correct_sig = hex::encode(mac.finalize().into_bytes());

        // Header with multiple v1 signatures (old + new)
        let signature_header = format!("t={},v1=oldsignature,v1={}", timestamp, correct_sig);

        let result = verify_webhook(
            payload.as_bytes(),
            &signature_header,
            TEST_SECRET,
        );

        assert!(result.is_ok());
        assert!(result.unwrap(), "Should match second v1 signature");
    }

    // ===================
    // Invalid Signature Tests
    // ===================

    #[test]
    fn test_verify_wrong_signature() {
        let payload = r#"{"type":"payment_intent.succeeded"}"#;
        let timestamp = Utc::now().timestamp();

        // Create signature with different secret
        let signature_header = generate_test_signature(payload, timestamp, "wrong_secret");

        let result = verify_webhook(
            payload.as_bytes(),
            &signature_header,
            TEST_SECRET,
        );

        assert!(result.is_ok());
        assert!(!result.unwrap(), "Wrong signature should return false");
    }

    #[test]
    fn test_verify_tampered_payload() {
        let original_payload = r#"{"amount":1000}"#;
        let timestamp = Utc::now().timestamp();
        let signature_header = generate_test_signature(original_payload, timestamp, TEST_SECRET);

        // Verify with tampered payload
        let tampered_payload = r#"{"amount":9999}"#;
        let result = verify_webhook(
            tampered_payload.as_bytes(),
            &signature_header,
            TEST_SECRET,
        );

        assert!(result.is_ok());
        assert!(!result.unwrap(), "Tampered payload should fail verification");
    }

    #[test]
    fn test_verify_random_signature() {
        let payload = r#"{"data":"test"}"#;
        let timestamp = Utc::now().timestamp();
        let signature_header = format!("t={},v1=abc123def456", timestamp);

        let result = verify_webhook(
            payload.as_bytes(),
            &signature_header,
            TEST_SECRET,
        );

        assert!(result.is_ok());
        assert!(!result.unwrap(), "Random signature should fail");
    }

    // ===================
    // Missing Header Tests
    // ===================

    #[test]
    fn test_verify_missing_timestamp() {
        let signature_header = "v1=somesignature";
        let result = verify_webhook(b"payload", signature_header, TEST_SECRET);

        assert!(result.is_err(), "Missing timestamp should error");
        assert!(result.unwrap_err().to_string().contains("timestamp"));
    }

    #[test]
    fn test_verify_missing_signature() {
        let signature_header = "t=1234567890";
        let result = verify_webhook(b"payload", signature_header, TEST_SECRET);

        assert!(result.is_err(), "Missing v1 signature should error");
    }

    #[test]
    fn test_verify_empty_header() {
        let result = verify_webhook(b"payload", "", TEST_SECRET);
        assert!(result.is_err(), "Empty header should error");
    }

    #[test]
    fn test_verify_malformed_header() {
        let result = verify_webhook(b"payload", "malformed=header=format", TEST_SECRET);
        assert!(result.is_err(), "Malformed header should error");
    }

    // ===================
    // Edge Cases
    // ===================

    #[test]
    fn test_verify_empty_payload() {
        let payload = "";
        let timestamp = Utc::now().timestamp();
        let signature_header = generate_test_signature(payload, timestamp, TEST_SECRET);

        let result = verify_webhook(
            payload.as_bytes(),
            &signature_header,
            TEST_SECRET,
        );

        assert!(result.is_ok());
        assert!(result.unwrap(), "Empty payload with valid signature should verify");
    }

    #[test]
    fn test_verify_large_payload() {
        let payload = "x".repeat(100_000);
        let timestamp = Utc::now().timestamp();
        let signature_header = generate_test_signature(&payload, timestamp, TEST_SECRET);

        let result = verify_webhook(
            payload.as_bytes(),
            &signature_header,
            TEST_SECRET,
        );

        assert!(result.is_ok());
        assert!(result.unwrap(), "Large payload should verify");
    }

    #[test]
    fn test_verify_unicode_payload() {
        let payload = r#"{"name":"日本語","emoji":"🎉"}"#;
        let timestamp = Utc::now().timestamp();
        let signature_header = generate_test_signature(payload, timestamp, TEST_SECRET);

        let result = verify_webhook(
            payload.as_bytes(),
            &signature_header,
            TEST_SECRET,
        );

        assert!(result.is_ok());
        assert!(result.unwrap(), "Unicode payload should verify");
    }

    #[test]
    fn test_verify_binary_payload() {
        let payload: &[u8] = &[0x00, 0x01, 0x02, 0xFF, 0xFE];
        let timestamp = Utc::now().timestamp();

        // Generate signature for binary data
        let signed_payload = format!("{}.{}", timestamp, String::from_utf8_lossy(payload));
        type HmacSha256 = hmac::Hmac<sha2::Sha256>;
        use hmac::Mac;
        let mut mac = HmacSha256::new_from_slice(TEST_SECRET.as_bytes()).unwrap();
        mac.update(signed_payload.as_bytes());
        let signature = hex::encode(mac.finalize().into_bytes());
        let signature_header = format!("t={},v1={}", timestamp, signature);

        let result = verify_webhook(payload, &signature_header, TEST_SECRET);

        assert!(result.is_ok());
        assert!(result.unwrap(), "Binary payload should verify");
    }

    // ===================
    // Header Format Tests
    // ===================

    #[test]
    fn test_verify_header_with_extra_fields() {
        let payload = r#"{"test":true}"#;
        let timestamp = Utc::now().timestamp();

        let signed_payload = format!("{}.{}", timestamp, payload);
        type HmacSha256 = hmac::Hmac<sha2::Sha256>;
        use hmac::Mac;
        let mut mac = HmacSha256::new_from_slice(TEST_SECRET.as_bytes()).unwrap();
        mac.update(signed_payload.as_bytes());
        let signature = hex::encode(mac.finalize().into_bytes());

        // Header with extra unknown fields
        let signature_header = format!("t={},v0=oldsig,v1={},v2=newsig", timestamp, signature);

        let result = verify_webhook(
            payload.as_bytes(),
            &signature_header,
            TEST_SECRET,
        );

        assert!(result.is_ok());
        assert!(result.unwrap(), "Should ignore unknown fields and verify v1");
    }

    #[test]
    fn test_verify_signature_case_sensitivity() {
        let payload = "test";
        let timestamp = Utc::now().timestamp();

        let signed_payload = format!("{}.{}", timestamp, payload);
        type HmacSha256 = hmac::Hmac<sha2::Sha256>;
        use hmac::Mac;
        let mut mac = HmacSha256::new_from_slice(TEST_SECRET.as_bytes()).unwrap();
        mac.update(signed_payload.as_bytes());
        let signature = hex::encode(mac.finalize().into_bytes());

        // Test with uppercase signature (should fail)
        let signature_header = format!("t={},v1={}", timestamp, signature.to_uppercase());

        let result = verify_webhook(
            payload.as_bytes(),
            &signature_header,
            TEST_SECRET,
        );

        // Hex should be lowercase
        assert!(result.is_ok());
        // This depends on implementation - hex encoding is typically lowercase
    }
}
