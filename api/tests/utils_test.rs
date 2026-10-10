//! Auth/crypto utilities — exercises the REAL `revolution_api::utils`.
//!
//! These tests previously ran against a hand-duplicated `mod utils`
//! whose `verify_jwt` used `Validation::default()` and a `Claims`
//! struct with no `token_type`. The shipped verifier pins HS256 and
//! enforces access/refresh segregation, so a regression that weakened
//! either (e.g. reverting to `Validation::default()`) would have
//! passed the old tests. They now bind to the production functions and
//! assert the real security invariants.

use revolution_api::utils;

// ── Password hashing (Argon2id) ─────────────────────────────────────

#[test]
fn hash_is_argon2_format_and_salted() {
    let h1 = utils::hash_password("SecurePassword123!").unwrap();
    let h2 = utils::hash_password("SecurePassword123!").unwrap();
    assert!(h1.starts_with("$argon2"), "expected Argon2 PHC string");
    assert_ne!(h1, h2, "per-hash salt must make digests unique");
}

#[test]
fn verify_password_accepts_correct_and_rejects_wrong() {
    let hash = utils::hash_password("CorrectHorse1!").unwrap();
    assert!(utils::verify_password("CorrectHorse1!", &hash).unwrap());
    assert!(!utils::verify_password("WrongHorse1!", &hash).unwrap());
}

#[test]
fn verify_password_errors_on_malformed_hash() {
    assert!(utils::verify_password("whatever", "not-a-phc-string").is_err());
}

#[test]
fn unicode_password_roundtrips() {
    let pw = "密码🔐パスワード-9!";
    let hash = utils::hash_password(pw).unwrap();
    assert!(utils::verify_password(pw, &hash).unwrap());
}

// ── Password policy ─────────────────────────────────────────────────

#[test]
fn password_policy_enforces_owasp_rules() {
    // Too short (<12).
    assert!(utils::validate_password("Short1!").is_err());
    // Missing a special character.
    assert!(utils::validate_password("NoSpecialChar123abc").is_err());
    // Contains a known weak pattern even though it is otherwise complex.
    assert!(utils::validate_password("MyPassword123!").is_err());
    // Satisfies length + upper + lower + digit + special, no weak word.
    assert!(utils::validate_password("SecureTrading#2026").is_ok());
}

// ── JWT: type segregation + algorithm pinning ───────────────────────

const SECRET: &str = "test_secret_key_minimum_32_chars_xxxxx";

#[test]
fn access_token_roundtrips_and_preserves_i64_subject() {
    // 2^53 + 1 — would lose precision if ever coerced through f64.
    let uid: i64 = 9_007_199_254_740_993;
    let token = utils::create_jwt(uid, SECRET, 24).unwrap();
    assert_eq!(token.split('.').count(), 3, "JWS compact form");
    let claims = utils::verify_jwt(&token, SECRET, "access").unwrap();
    assert_eq!(claims.sub, uid);
    assert_eq!(claims.token_type, "access");
}

#[test]
fn refresh_token_roundtrips_as_refresh() {
    let token = utils::create_refresh_token(42, SECRET).unwrap();
    let claims = utils::verify_jwt(&token, SECRET, "refresh").unwrap();
    assert_eq!(claims.sub, 42);
    assert_eq!(claims.token_type, "refresh");
}

/// Security invariant: an access token MUST NOT satisfy a refresh-token
/// check and vice versa. The old duplicated verifier had no concept of
/// token type, so this whole class was untested.
#[test]
fn token_type_segregation_is_enforced() {
    let access = utils::create_jwt(7, SECRET, 1).unwrap();
    let refresh = utils::create_refresh_token(7, SECRET).unwrap();
    assert!(
        utils::verify_jwt(&access, SECRET, "refresh").is_err(),
        "access token accepted where refresh required"
    );
    assert!(
        utils::verify_jwt(&refresh, SECRET, "access").is_err(),
        "refresh token accepted where access required"
    );
}

/// Security invariant: HS256 is pinned. A forged `alg:none` token (the
/// classic algorithm-confusion attack) must be rejected. This is the
/// 10-year regression guard against anyone reverting to
/// `Validation::default()`.
#[test]
fn alg_none_forgery_is_rejected() {
    use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine};

    let now = chrono::Utc::now().timestamp();
    let header = URL_SAFE_NO_PAD.encode(br#"{"alg":"none","typ":"JWT"}"#);
    let claims = URL_SAFE_NO_PAD.encode(
        format!(
            r#"{{"sub":1,"exp":{},"iat":{},"token_type":"access"}}"#,
            now + 3600,
            now
        )
        .as_bytes(),
    );
    // alg:none → empty signature segment.
    let forged = format!("{header}.{claims}.");
    assert!(
        utils::verify_jwt(&forged, SECRET, "access").is_err(),
        "alg:none token must never verify under HS256 pinning"
    );
}

#[test]
fn wrong_secret_fails_verification() {
    let token = utils::create_jwt(1, "correct-secret-correct-secret-xx", 24).unwrap();
    assert!(utils::verify_jwt(&token, "another-secret-another-secret-xx", "access").is_err());
}

#[test]
fn expired_token_fails_verification() {
    let token = utils::create_jwt(1, SECRET, -1).unwrap();
    assert!(utils::verify_jwt(&token, SECRET, "access").is_err());
}

#[test]
fn tampered_or_malformed_token_fails_verification() {
    assert!(utils::verify_jwt("not.a.jwt", SECRET, "access").is_err());
    let mut token = utils::create_jwt(1, SECRET, 1).unwrap();
    token.push('x'); // corrupt the signature segment
    assert!(utils::verify_jwt(&token, SECRET, "access").is_err());
}

// ── Token / session generators ──────────────────────────────────────

#[test]
fn generate_token_is_32_alphanumeric_and_unique() {
    let a = utils::generate_token();
    let b = utils::generate_token();
    assert_eq!(a.len(), 32);
    assert!(a.chars().all(|c| c.is_ascii_alphanumeric()));
    assert_ne!(a, b);
}

#[test]
fn session_id_is_256_bit_hex() {
    let s = utils::generate_session_id();
    assert_eq!(s.len(), 64, "32 bytes hex-encoded");
    assert!(s.chars().all(|c| c.is_ascii_hexdigit()));
}

#[test]
fn password_reset_token_is_64_chars_unique() {
    let a = utils::generate_password_reset_token();
    assert_eq!(a.len(), 64);
    assert_ne!(a, utils::generate_password_reset_token());
}

#[test]
fn generators_are_collision_free_over_a_batch() {
    let n = 200;
    let set: std::collections::HashSet<String> = (0..n).map(|_| utils::generate_token()).collect();
    assert_eq!(set.len(), n, "no collisions across {n} tokens");
}

// ── Constant-time compare + token hashing ───────────────────────────

#[test]
fn constant_time_compare_matches_only_identical_strings() {
    assert!(utils::constant_time_compare("abc123", "abc123"));
    assert!(!utils::constant_time_compare("abc123", "abc124"));
    assert!(!utils::constant_time_compare("abc", "abcd")); // length mismatch
}

#[test]
fn hash_token_is_deterministic_and_not_identity() {
    let h1 = utils::hash_token("opaque-token");
    let h2 = utils::hash_token("opaque-token");
    assert_eq!(h1, h2, "same input → same digest (DB lookup key)");
    assert_ne!(h1, "opaque-token", "must store a hash, not the token");
    assert_ne!(h1, utils::hash_token("different-token"));
}

/// Regression guard for argon2 crate upgrades.
///
/// This PHC string was produced by argon2 0.5.3 (the version in use before the
/// 2026-08-30 bump to 0.6) at `hash_password`'s real parameters
/// (Argon2id, m=65536, t=3, p=4, 32-byte output), so it mirrors a hash
/// actually sitting in the users table.
/// Every stored user password predates that upgrade, so if a future argon2
/// major stops verifying this string, the whole user base is locked out — this
/// test fails first instead.
#[test]
fn verifies_password_hashed_by_argon2_0_5() {
    const LEGACY_ARGON2_0_5_HASH: &str =
        "$argon2id$v=19$m=65536,t=3,p=4$CEoQaV7jsF2LZbHp3C/UBw$Exi2MqlY5D2/sLl+inv7tgWdMTgJH8FMuyzt1JsXzqY";

    assert!(
        utils::verify_password("correct horse battery staple", LEGACY_ARGON2_0_5_HASH).unwrap(),
        "argon2 upgrade broke verification of pre-existing password hashes"
    );
    assert!(
        !utils::verify_password("wrong password", LEGACY_ARGON2_0_5_HASH).unwrap(),
        "verification must still reject an incorrect password"
    );
}

// ── HTML sanitization (utils::html) ─────────────────────────────────

mod html_sanitization {
    use revolution_api::utils::html::{
        is_safe_url, sanitize_content_blocks, sanitize_form_fields, sanitize_html,
    };
    use serde_json::json;

    #[test]
    fn keeps_formatting_and_hardens_links() {
        assert_eq!(
            sanitize_html(
                r#"<p>Q&amp;A <strong>bold</strong> <a href="https://x.com/?a=1&b=2">x</a></p>"#
            ),
            r#"<p>Q&amp;A <strong>bold</strong> <a href="https://x.com/?a=1&amp;b=2" rel="noopener noreferrer">x</a></p>"#
        );
    }

    #[test]
    fn removes_scripts_handlers_and_foreign_content() {
        let out = sanitize_html(
            r#"<script>alert(1)</script><style>p{}</style><svg onload=alert(1)><a>s</a></svg><math><mi>m</mi></math><noscript>n</noscript><select><option>o</option></select><object data="x">obj</object><img src="/i.png" onerror="alert(1)"><div onclick="x" class="c">d</div>"#,
        );
        assert_eq!(out, r#"<img src="/i.png"><div class="c">d</div>"#);
    }

    #[test]
    fn drops_script_capable_urls_however_spelled() {
        for href in [
            "javascript:alert(1)",
            "java&#x09;script:alert(1)",
            " javascript:alert(1)",
            "JaVaScRiPt:alert(1)",
            "&#106;avascript:alert(1)",
            "vbscript:x",
            "data:text/html,x",
        ] {
            assert_eq!(
                sanitize_html(&format!(r#"<a href="{href}">x</a>"#)),
                r#"<a rel="noopener noreferrer">x</a>"#,
                "href {href:?} must not survive"
            );
        }
    }

    #[test]
    fn keeps_relative_and_allowed_scheme_urls() {
        for href in [
            "/p",
            "#top",
            "?q=1",
            "mailto:a@b.c",
            "tel:+1",
            "https://example.com/",
        ] {
            assert_eq!(
                sanitize_html(&format!(r#"<a href="{href}">x</a>"#)),
                format!(r#"<a href="{href}" rel="noopener noreferrer">x</a>"#)
            );
        }
    }

    #[test]
    fn allows_data_urls_only_as_media_sources() {
        assert_eq!(
            sanitize_html(
                r#"<img src="data:image/png;base64,iVBORw0KGgo=" alt="px"><img src="data:text/html,x"><a href="data:image/png;base64,AA">a</a>"#
            ),
            r#"<img src="data:image/png;base64,iVBORw0KGgo=" alt="px"><img><a rel="noopener noreferrer">a</a>"#
        );
    }

    #[test]
    fn embeds_only_trusted_https_players() {
        // An iframe's children are raw text to the HTML parser (they can never
        // contain `</iframe>`), so the fallback text is inert and kept as-is.
        assert_eq!(
            sanitize_html(
                r#"<iframe src="https://www.youtube.com/embed/abc" allowfullscreen="">fallback</iframe><iframe src="//player.vimeo.com/video/1"></iframe>"#
            ),
            r#"<iframe src="https://www.youtube.com/embed/abc" allowfullscreen="">fallback</iframe><iframe src="//player.vimeo.com/video/1"></iframe>"#
        );
        for src in [
            "https://evil.tld/?youtube.com",
            "https://youtube.com.evil.tld/",
            "http://www.youtube.com/embed/a",
            "/embed",
            "javascript:alert(1)",
        ] {
            assert_eq!(
                sanitize_html(&format!(r#"<iframe src="{src}"></iframe>"#)),
                "<iframe></iframe>",
                "iframe src {src:?} must not survive"
            );
        }
    }

    #[test]
    fn mutation_xss_payloads_come_out_inert() {
        for payload in [
            "<math><mtext><table><mglyph><style><img src=x onerror=alert(1)>",
            r#"<noscript><p title="</noscript><img src=x onerror=alert(1)>"></p></noscript>"#,
            r#"<svg></p><style><a id="</style><img src=1 onerror=alert(1)>">"#,
            r#"<form><math><mtext></form><form><mglyph><style></math><img src onerror=alert(1)>"#,
            "<!--><img src=x onerror=alert(1)>-->",
        ] {
            let out = sanitize_html(payload);
            assert!(!out.contains("onerror"), "{payload:?} -> {out:?}");
            assert!(!out.contains("<script"), "{payload:?} -> {out:?}");
            assert!(!out.contains("<style"), "{payload:?} -> {out:?}");
        }
    }

    #[test]
    fn classifies_bare_urls() {
        for safe in [
            "",
            "/a",
            "#x",
            "?q",
            "https://a.b/",
            "HTTP://A.B/",
            "mailto:a@b.c",
            "tel:1",
        ] {
            assert!(is_safe_url(safe), "{safe:?} should be safe");
        }
        for unsafe_url in [
            "javascript:alert(1)",
            "java\tscript:alert(1)",
            " JAVASCRIPT:alert(1)",
            "vbscript:x",
            "data:text/html,x",
            "file:///etc/passwd",
        ] {
            assert!(
                !is_safe_url(unsafe_url),
                "{unsafe_url:?} should be rejected"
            );
        }
    }

    #[test]
    fn sanitizes_html_block_fields_and_leaves_plain_text_alone() {
        let mut blocks = json!([
            {"type": "paragraph", "data": {"text": "a<img src=x onerror=alert(1)> &amp; b"}},
            {"type": "heading", "content": {"text": "<b>T</b><script>x</script>", "level": 2}},
            {"type": "code", "data": {"code": "<div>keep</div>", "text": "if (a < b && c) {}"}},
            {"type": "button", "data": {"text": "Q&A", "url": "javascript:alert(1)"}},
            {"type": "video", "data": {"url": "https://www.youtube.com/embed/x", "caption": "a & b"}},
            {"type": "list", "data": {"items": ["<i>one</i><script>x</script>", {"content": "<u>two</u><img src=x onerror=y>", "children": ["<b>c</b><script>z</script>"]}]}},
            {"type": "checklist", "data": {"checklistItems": [{"text": "<s>done</s><svg onload=x>", "checked": true}]}},
            {"type": "callout", "data": {"title": "Note & tip", "content": "<p onclick=x>c</p>"}},
            {"type": "html", "content": {"html": "<iframe src=\"https://evil.tld/\"></iframe><p>ok</p>"}, "settings": {"linkUrl": "vbscript:x", "backgroundImage": "/bg.png"}},
            {"type": "columns", "content": {"columns": [{"blocks": [{"type": "quote", "data": {"text": "<q>q</q><script>x</script>"}}]}]}}
        ]);
        sanitize_content_blocks(&mut blocks);
        assert_eq!(
            blocks,
            json!([
                {"type": "paragraph", "data": {"text": "a<img src=\"x\"> &amp; b"}},
                {"type": "heading", "content": {"text": "<b>T</b>", "level": 2}},
                {"type": "code", "data": {"code": "<div>keep</div>", "text": "if (a < b && c) {}"}},
                {"type": "button", "data": {"text": "Q&A"}},
                {"type": "video", "data": {"url": "https://www.youtube.com/embed/x", "caption": "a & b"}},
                {"type": "list", "data": {"items": ["<i>one</i>", {"content": "<u>two</u><img src=\"x\">", "children": ["<b>c</b>"]}]}},
                {"type": "checklist", "data": {"checklistItems": [{"text": "<s>done</s>", "checked": true}]}},
                {"type": "callout", "data": {"title": "Note & tip", "content": "<p>c</p>"}},
                {"type": "html", "content": {"html": "<iframe></iframe><p>ok</p>"}, "settings": {"backgroundImage": "/bg.png"}},
                {"type": "columns", "content": {"columns": [{"blocks": [{"type": "quote", "data": {"text": "q"}}]}]}}
            ])
        );
    }

    #[test]
    fn sanitizes_form_markup_and_policy_links_only() {
        let mut fields = json!([
            {"field_type": "html", "placeholder": "<p>Intro</p><script>x</script>"},
            {"field_type": "text", "placeholder": "Q&A <name>", "label": "<b>Name</b>"},
            {"field_type": "gdpr", "attributes": {
                "consent_text": "I agree <a href=\"javascript:x\">terms</a>",
                "privacy_url": "javascript:alert(1)",
                "terms_url": "https://example.com/terms"
            }}
        ]);
        sanitize_form_fields(&mut fields);
        assert_eq!(
            fields,
            json!([
                {"field_type": "html", "placeholder": "<p>Intro</p>"},
                {"field_type": "text", "placeholder": "Q&A <name>", "label": "<b>Name</b>"},
                {"field_type": "gdpr", "attributes": {
                    "consent_text": "I agree <a rel=\"noopener noreferrer\">terms</a>",
                    "terms_url": "https://example.com/terms"
                }}
            ])
        );
    }
}
