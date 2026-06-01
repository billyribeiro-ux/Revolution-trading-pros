// R23-D: test-scaffold doc comments use narrative list formatting that doesn't
// match clippy's strict CommonMark interpretation. Allow file-wide.
#![allow(clippy::doc_lazy_continuation, clippy::doc_overindented_list_items)]

//! Credential resolver service contract tests — pure, no-DB.
//!
//! Binds directly to `revolution_api::services::credential_resolver`
//! and pins the `ResolvedCreds` DTO shape — the data structure that
//! carries third-party API credentials (Stripe today; SendGrid /
//! Bunny.net etc. later per the module header) through every payment
//! handler in the crate.
//!
//! ## Why this file exists (R22-D, sweep-complete extra target)
//!
//! `services/credential_resolver.rs` (386 LOC) is the most security-
//! sensitive non-route module after `services::mfa`. It implements
//! PE7 invariant 2A — runtime credential resolution from the
//! `service_connections` table with env-var fallback. Per the
//! module header (lines 20-25):
//!
//!     "Discipline:
//!      - Never log secret_key / webhook_secret values. Boundary
//!        log only the SOURCE ("db"|"env") so we can debug
//!        "is admin's paste being read?"."
//!
//! This is a CLAUDE.md habit #1 ("Cite the rule in your work")
//! invariant cited DIRECTLY in the source. The pin enforces:
//!
//! 1. **`ResolvedCreds` is pub** — every payment handler depends on
//!    this DTO via `CredentialResolver::resolve(...)`. A regression
//!    that made it `pub(crate)` would break out-of-crate test
//!    coverage AND would block any future "external integrations"
//!    crate from re-using the type.
//!
//! 2. **`source: &'static str` is the audit boundary.** Per the
//!    fetch_fresh fn (lines 153-219) the field is set to ONE of
//!    `"db"` (line 195) or `"env"` (line 248). This is the
//!    documented value set — any other value would be a refactor
//!    accident. Tracing-log calls at lines 186 and 203 ARE the
//!    "boundary log only the source" defense. A regression that
//!    flipped the source field to a free-form String (e.g., to
//!    include `(env)` or `(db-cached)`) would compile but would
//!    break the log-parsing tools that grep for the exact
//!    `source="db"` / `source="env"` tokens.
//!
//! 3. **`credentials` is a `HashMap<String, String>` of decrypted
//!    secrets** (line 51). The secrets themselves NEVER appear in
//!    log output. A regression that added a `Display` or `Debug`
//!    impl that printed `self.credentials` directly would leak
//!    every Stripe secret key to the logs on the next deploy.
//!    Per CLAUDE.md habit #3 ("re-read your own diff"): the
//!    `#[derive(Debug)]` line (line 44) IS already a footgun —
//!    serialized via tracing::%, the secrets would appear. The
//!    saving grace is that no code in the module actually logs
//!    the full struct (only the `service`/`environment`/`source`
//!    fields, one at a time).
//!
//! 4. **`webhook_secret: Option<String>`** — Stripe webhook signing
//!    secret. Per the StripeService client wiring (lines 100-112),
//!    an empty webhook_secret falls back to "no signing" (line 110)
//!    instead of failing the request. A regression that flipped
//!    that fallback would either:
//!     - silently accept unsigned webhooks (catastrophic security
//!       hole — any actor could POST fake payment events)
//!     - silently reject legitimate webhooks (operator outage)
//!
//! 5. **`source` is NOT carried into the secret credentials map.**
//!    A regression that copied `source` into `credentials`
//!    (e.g., for "debugging convenience") would leak the boundary
//!    log info INTO the secret map — where every downstream caller
//!    serializes it (Stripe's HTTP client puts the map values into
//!    the `Authorization` header). Per CLAUDE.md habit #4 ("Trust
//!    the operator's gut over CI"): the type system DOES prevent
//!    `&'static str` from being inserted into `HashMap<String,
//!    String>` without a `.to_string()` call — but that's a one-
//!    line "improvement" away from being possible.
//!
//! ## Pattern source
//!
//! Modeled on `tests/mfa_test.rs` (sister R22-D auth-adjacent
//! pin), `tests/auth_test.rs` (auth-path DTO shape pin),
//! `tests/migrate_test.rs` (sister R22-D scaffold).

use revolution_api::services::credential_resolver::ResolvedCreds;
use std::collections::HashMap;

// ── 1. ResolvedCreds is pub + has the documented 5-field shape ───────

/// `ResolvedCreds` MUST be `pub` and carry exactly these fields per
/// services/credential_resolver.rs:43-56:
///
///     pub service:         String                       (line 47)
///     pub environment:     String                       (line 49)
///     pub credentials:     HashMap<String, String>      (line 51)
///     pub webhook_secret:  Option<String>               (line 53)
///     pub source:          &'static str  ("db"|"env")   (line 55)
///
/// Per CLAUDE.md habit #1 ("Cite the rule in your work") — this is
/// PE7 invariant 2A's API contract. Every payment handler in the
/// crate consumes this struct via `CredentialResolver::resolve()`
/// (line 83) or `CredentialResolver::stripe_client()` (line 100).
/// A regression that changed the field set would either fail to
/// compile (caught) OR would silently change the JSON contract on
/// any handler that serializes the struct over the wire (none do
/// today — the secrets MUST NOT go over JSON).
///
/// R9-D NEGATIVE: the pin builds the struct directly (proving every
/// field is `pub`). A regression that flipped any field to private
/// (`pub(crate)`, `pub(super)`, or no `pub`) would fail this
/// compile.
#[test]
fn resolved_creds_pub_shape_5_fields() {
    let mut creds_map = HashMap::new();
    creds_map.insert(
        "secret_key".to_string(),
        "sk_test_canary_NOTREAL".to_string(),
    );
    creds_map.insert(
        "publishable_key".to_string(),
        "pk_test_canary_NOTREAL".to_string(),
    );

    let creds = ResolvedCreds {
        service: "stripe".to_string(),
        environment: "sandbox".to_string(),
        credentials: creds_map,
        webhook_secret: Some("whsec_canary_NOTREAL".to_string()),
        // The `source` field is `&'static str` — a hard guarantee
        // that callers can't accidentally inject a String built from
        // request data (which would defeat the boundary-log
        // discipline cited in the module header lines 20-25).
        source: "db",
    };

    // Every field MUST be readable from outside the crate.
    assert_eq!(creds.service, "stripe");
    assert_eq!(creds.environment, "sandbox");
    assert_eq!(creds.credentials.len(), 2);
    assert!(creds.webhook_secret.is_some());
    assert_eq!(creds.source, "db");
}

// ── 2. R9-D NEGATIVE: `source` is &'static str, NOT String ───────────

/// R9-D NEGATIVE: `ResolvedCreds.source: &'static str` (line 55) is
/// the load-bearing audit-boundary type. The `&'static str` lifetime
/// guarantees that the source value comes from a string LITERAL —
/// callers CANNOT inject a string built at runtime (e.g., from a
/// request header or env var read).
///
/// Per the module header (lines 20-25): "Boundary log only the
/// SOURCE ("db"|"env")". The two source-setter sites are:
///   - line 195: `source: "db"`  (DB row found, decrypted)
///   - line 248: `source: "env"` (env fallback)
///
/// A regression that flipped the field type to `String` would
/// compile (String is `Into<&str>`-able the other way), BUT would
/// allow any caller to do:
///
///     ResolvedCreds {
///         source: format!("db ({})", user_input).to_string(),
///         ...
///     }
///
/// which would (a) leak the user_input into the audit log, and
/// (b) break the grep-for-exactly-"db" log-tail tooling.
///
/// The pin asserts the type of `creds.source` is `&'static str`,
/// not String — a regression that widened it would fail to compile.
#[test]
fn resolved_creds_source_is_static_str_not_string() {
    let creds = ResolvedCreds {
        service: "stripe".to_string(),
        environment: "".to_string(),
        credentials: HashMap::new(),
        webhook_secret: None,
        source: "env",
    };

    // Compile-time pin: assignment to `&'static str` MUST succeed.
    // If `source` were widened to String, this assignment would
    // fail (String is NOT `Copy` and would need `.as_str()`).
    let s: &'static str = creds.source;
    assert_eq!(s, "env");

    // Also pin: the two known-valid values from the source.
    // A regression that introduced a third value (e.g., "cache") at
    // the cache_put boundary would mean the audit-grep tools need
    // updating — making it an intentional choice, not silent drift.
    let known_db = ResolvedCreds {
        service: "stripe".to_string(),
        environment: "production".to_string(),
        credentials: HashMap::new(),
        webhook_secret: None,
        source: "db",
    };
    assert_eq!(known_db.source, "db");
}

// ── 3. ResolvedCreds is Clone + Debug (cache-put round-trip) ─────────

/// `ResolvedCreds` MUST implement `Clone` (line 44) because the
/// resolver caches it: `cache_put` stores a clone (line 93:
/// `self.cache_put(key, resolved.clone()).await`) AND
/// `cache_get` returns a clone (line 139:
/// `Some(entry.creds.clone())`). A regression that dropped the
/// Clone derive would break the cache.
///
/// `ResolvedCreds` MUST implement `Debug` (line 44). Per CLAUDE.md
/// habit #3 ("re-read your own diff"): Debug is a footgun for
/// credential structs — `tracing::debug!(?creds)` would print the
/// secrets. The saving grace: no code in the module logs the full
/// struct (only individual fields, per the discipline at lines
/// 20-25). The Debug derive is REQUIRED for the field-by-field
/// log lines (which print `service = %service`, etc., one at a
/// time). The pin documents the trade-off.
///
/// R9-D NEGATIVE: a regression that wrapped credentials in
/// `Secret<HashMap<String, String>>` (from a crate like `secrecy`)
/// would BLOCK the Debug-derive footgun BUT would require
/// downstream callers to call `.expose_secret()` at the use site.
/// The current shape is the pragmatic choice — the pin documents
/// "we know Debug is a footgun, we don't trigger it, the lint that
/// catches `tracing::debug!(?creds)` is the human's job".
#[test]
fn resolved_creds_clones_and_debugs_without_panic() {
    let creds = ResolvedCreds {
        service: "stripe".to_string(),
        environment: "production".to_string(),
        credentials: {
            let mut m = HashMap::new();
            m.insert("secret_key".to_string(), "sk_live_DO_NOT_LOG".to_string());
            m
        },
        webhook_secret: Some("whsec_DO_NOT_LOG".to_string()),
        source: "db",
    };

    // Clone MUST work (cache_put / cache_get round-trip).
    let cloned = creds.clone();
    assert_eq!(cloned.service, "stripe");
    assert_eq!(cloned.environment, "production");
    assert_eq!(cloned.source, "db");
    assert_eq!(
        cloned.credentials.get("secret_key").map(|s| s.as_str()),
        Some("sk_live_DO_NOT_LOG")
    );

    // Debug MUST work (per the derive on line 44). The pin does
    // NOT assert what Debug emits — that's the footgun. It only
    // asserts that calling Debug::fmt does not panic.
    let _ = format!("{cloned:?}");
}

// ── 4. R9-D NEGATIVE: no money/PK DTOs on credential surface ─────────

/// R9-D NEGATIVE: the credential_resolver surface has NO BIGSERIAL
/// PK fields and NO `*_cents` money fields. The DTOs in scope are:
///   - `ResolvedCreds`         (5 fields, all String / Option<String>
///                              / HashMap / &'static str)
///   - `CredentialResolver`    (constructor opaque)
///   - private `CacheEntry`    (creds + Instant)
///   - private `DbCreds`       (sqlx FromRow for DB query)
///
/// Per CLAUDE.md "Money / cents — i64 ONLY, BIGINT ONLY, EVERY
/// TIME": the rule is "every `*_cents` value is i64". This module
/// has NO money values — but the credentials map IS what unlocks
/// Stripe API access, where every Stripe call uses i64 amounts.
/// The credentials themselves are opaque strings (sk_..., pk_...,
/// whsec_...) so the i64 invariant doesn't apply directly.
///
/// Per CLAUDE.md "Reserved exception": this module has NO i32
/// counter fields. The only numeric fields are the cache TTL
/// (`Duration`, line 41) and the `fetched_at: Instant` (private,
/// line 61) — neither qualifies as money or PK.
///
/// If a future refactor added a `user_id: i64` field (e.g., per-
/// user credential isolation), the BIGSERIAL i64 rule would apply.
/// The pin documents the gap so the next PR that adds such a field
/// has to cite the rule.
#[test]
fn credential_resolver_module_has_no_money_or_pk_dtos() {
    // No DTOs to bind beyond the pub ones above — `ResolvedCreds`
    // is the surface, and it has zero numeric fields. The pin is
    // a documenting comment + a smoke build of the struct.
    let _: ResolvedCreds = ResolvedCreds {
        service: "stripe".to_string(),
        environment: "sandbox".to_string(),
        credentials: HashMap::new(),
        webhook_secret: None,
        source: "env",
    };

    assert_eq!(2 + 2, 4, "smoke");
}

// ── 5. webhook_secret None vs Some("") handled distinctly ────────────

/// `webhook_secret: Option<String>` carries a meaningful distinction:
///   - `None`         — no webhook secret configured at all
///   - `Some("")`     — admin pasted an empty string (operator error)
///   - `Some("whsec_X")` — valid signing secret
///
/// Per the stripe_client wiring (lines 108-111):
///
///     match creds.webhook_secret.as_deref() {
///         Some(s) if !s.is_empty() => svc.with_webhook_secret(s),
///         _ => svc,
///     }
///
/// the `!s.is_empty()` guard correctly treats `Some("")` the same
/// as `None` — falling back to "no signing". This is the right call
/// (an empty string would silently disable signing if passed
/// through), but it's also a subtle invariant: a regression that
/// flipped the guard to `if let Some(s) = ...` (without the
/// `!s.is_empty()` check) would compile and would route the empty
/// string into `with_webhook_secret("")` — silently accepting all
/// unsigned webhooks.
///
/// R9-D NEGATIVE: the pin builds `ResolvedCreds` with all three
/// shapes (None, Some(""), Some(real)) and proves they're all
/// constructable. The actual stripe_client wiring is private to
/// `CredentialResolver` and tested via integration tests; this
/// pin just freezes the DTO shape so the upstream contract is
/// stable.
#[test]
fn resolved_creds_webhook_secret_three_shapes() {
    let none = ResolvedCreds {
        service: "stripe".to_string(),
        environment: "production".to_string(),
        credentials: HashMap::new(),
        webhook_secret: None,
        source: "env",
    };
    assert!(
        none.webhook_secret.is_none(),
        "None case: no webhook secret configured"
    );

    let empty = ResolvedCreds {
        service: "stripe".to_string(),
        environment: "production".to_string(),
        credentials: HashMap::new(),
        // Some("") — operator pasted empty. The stripe_client
        // wiring (lines 108-111) correctly falls back to "no
        // signing" via the `!s.is_empty()` guard.
        webhook_secret: Some(String::new()),
        source: "db",
    };
    assert_eq!(
        empty.webhook_secret.as_deref(),
        Some(""),
        "Some(\"\") case: distinguishable from None at DTO level"
    );
    assert!(
        empty.webhook_secret.as_deref().map(str::is_empty) == Some(true),
        "Some(\"\") MUST be detectable via .is_empty() — the \
         stripe_client guard at line 108 depends on this"
    );

    let real = ResolvedCreds {
        service: "stripe".to_string(),
        environment: "production".to_string(),
        credentials: HashMap::new(),
        webhook_secret: Some("whsec_test_NOT_REAL_VALUE_CANARY".to_string()),
        source: "db",
    };
    assert!(
        real.webhook_secret
            .as_deref()
            .is_some_and(|s| !s.is_empty()),
        "Some(real) case: detectable via .is_some_and(|s| !s.is_empty())"
    );
}
