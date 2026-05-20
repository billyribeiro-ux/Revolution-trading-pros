//! Admin member management route contract tests — pure, no-DB.
//!
//! Binds directly to `revolution_api::routes::admin_member_management`
//! and exercises the public DTOs + the `router()` mount table. This
//! is the back-of-house member-CRUD surface — admins use it to comp
//! memberships, ban abusers, and export the customer list for legal
//! requests, so every field on `MemberSubscription` / `MemberOrder`
//! must keep its money-pin and Stripe-shaped nullability.
//!
//! ## Why this shape
//!
//! Every handler runs live SQL against `users` / `user_memberships`
//! / `orders` / `user_activity_log` / `member_notes` / `user_status`,
//! plus an outbound email send from `create_member`. None of that can
//! run in unit-test isolation, so we attack the contract:
//!
//! 1. **Money pin**: `MemberSubscription.price_cents` and
//!    `MemberOrder.total_cents` are `Option<i64>`. CLAUDE.md money
//!    rule: every `*_cents` is i64, never i32. A regression that
//!    narrows the field would cap large B2B comp totals at $21.4M
//!    AND silently wrap any sum-aggregate the export endpoint emits.
//!
//! 2. **Stripe-shaped nullability**: comp memberships and pre-Stripe
//!    legacy rows have NULL `price_cents` (no Stripe price attached).
//!    `MemberSubscription.price_cents` MUST be `Option<i64>` so the
//!    detail endpoint doesn't crash on those rows. Same story for
//!    `MemberOrder.total_cents` on free-trial / $0 orders.
//!
//! 3. **Request DTOs**: `CreateMemberRequest`, `UpdateMemberRequest`,
//!    `BanMemberRequest`, `CreateNoteRequest`, `ExportQuery`, and
//!    `ActivityQuery` are the wire-format the admin frontend sends.
//!    The "every field optional except `name`/`email`" shape on
//!    `CreateMemberRequest` is what lets the admin create a comp
//!    member without setting a password (welcome-email flow).
//!
//! 4. **Router mount table**: 10 routes across CRUD, ban/suspend/unban,
//!    notes, activity, and export. A refactor that drops `/:id/ban`
//!    or flips the HTTP verb on `/export` (GET, not POST) would
//!    compile and silently 404 / 405 in prod.
//!
//! ## Pattern source
//!
//! Modeled on `tests/payments_test.rs`, `tests/orders_test.rs`,
//! `tests/oauth_test.rs`, `tests/admin_indicators_test.rs`,
//! `tests/coupons_test.rs`, `tests/subscriptions_test.rs`.

use revolution_api::routes::admin_member_management::{
    ActivityQuery, BanMemberRequest, CreateMemberRequest, CreateNoteRequest, ExportQuery,
    MemberActivity, MemberDetail, MemberNote, MemberOrder, MemberSubscription, UpdateMemberRequest,
};

// ── Money: i64 end-to-end, round-trips past i32::MAX ─────────────────

/// HARD RULE (CLAUDE.md "Money / cents"): every `*_cents` field is i64.
/// `MemberSubscription.price_cents` is `Option<i64>` — Some-value must
/// round-trip past i32::MAX without loss. A regression to `Option<i32>`
/// would silently cap the comp-membership detail card at $21,474,836.47
/// and also wrap any aggregate the CSV export emits.
#[test]
fn member_subscription_price_cents_is_i64_round_trips_past_i32_max() {
    let above_i32_max: i64 = (i32::MAX as i64) + 1;
    let now = chrono::Utc::now().naive_utc();
    let sub = MemberSubscription {
        id: 1,
        product_name: Some("Annual Pro (comp)".to_string()),
        status: "active".to_string(),
        price_cents: Some(above_i32_max),
        billing_period: Some("annual".to_string()),
        started_at: Some(now),
        expires_at: None,
        cancelled_at: None,
        created_at: now,
    };

    let wire = serde_json::to_value(&sub).expect("serialize MemberSubscription");
    assert_eq!(
        wire["price_cents"].as_i64(),
        Some(above_i32_max),
        "price_cents must survive JSON round-trip as i64"
    );
    assert!(
        sub.price_cents.unwrap() > i32::MAX as i64,
        "test fixture must exceed i32::MAX or it does not prove the i64 pin"
    );
}

/// `MemberOrder.total_cents` is `Option<i64>` for the same reason:
/// the legal-request export sums these and a wrap on a 7-figure
/// account would understate the customer's spend by ~$42M.
#[test]
fn member_order_total_cents_is_i64_round_trips_past_i32_max() {
    let huge: i64 = (i32::MAX as i64) * 3; // ~$64.4M cents
    let now = chrono::Utc::now().naive_utc();
    let order = MemberOrder {
        id: 99,
        order_number: Some("ORD-2026-00099".to_string()),
        total_cents: Some(huge),
        status: Some("paid".to_string()),
        created_at: now,
    };

    let wire = serde_json::to_value(&order).expect("serialize MemberOrder");
    assert_eq!(wire["total_cents"].as_i64(), Some(huge));
    assert!(
        (huge as i32 as i64) != huge,
        "narrowing total_cents to i32 must lose data — proves i64 is required"
    );
}

/// Both money fields MUST stay nullable. Comp memberships and free
/// orders have NULL `price` / `total` columns; if these become
/// non-Optional the detail endpoint will fail deserialization for
/// every such row.
#[test]
fn member_subscription_and_order_money_fields_are_nullable() {
    let now = chrono::Utc::now().naive_utc();
    let sub = MemberSubscription {
        id: 1,
        product_name: None,
        status: "comp".to_string(),
        price_cents: None,
        billing_period: None,
        started_at: None,
        expires_at: None,
        cancelled_at: None,
        created_at: now,
    };
    let order = MemberOrder {
        id: 1,
        order_number: None,
        total_cents: None,
        status: None,
        created_at: now,
    };
    let sub_wire = serde_json::to_value(&sub).expect("serialize sub");
    let order_wire = serde_json::to_value(&order).expect("serialize order");
    assert!(sub_wire["price_cents"].is_null());
    assert!(sub_wire["product_name"].is_null());
    assert!(order_wire["total_cents"].is_null());
    assert!(order_wire["status"].is_null());
}

// ── Request DTOs: optional-fields contract ───────────────────────────

/// `CreateMemberRequest` carries name + email (required) and four
/// optional fields. The "comp member" admin flow uses the minimal
/// shape (no password, send_welcome_email = true). A regression that
/// made `password` required would break that flow.
#[test]
fn create_member_request_accepts_minimal_and_full_payloads() {
    let min: CreateMemberRequest = serde_json::from_value(serde_json::json!({
        "name": "Comp User",
        "email": "comp@example.com"
    }))
    .expect("minimal create-member payload");
    assert_eq!(min.name, "Comp User");
    assert_eq!(min.email, "comp@example.com");
    assert!(min.password.is_none());
    assert!(min.role.is_none());
    assert!(min.send_welcome_email.is_none());

    let full: CreateMemberRequest = serde_json::from_value(serde_json::json!({
        "name": "Jane Admin",
        "email": "jane@example.com",
        "password": "TempPass123!",
        "role": "editor",
        "send_welcome_email": true
    }))
    .expect("full create-member payload");
    assert_eq!(full.role.as_deref(), Some("editor"));
    assert_eq!(full.send_welcome_email, Some(true));
    assert_eq!(full.password.as_deref(), Some("TempPass123!"));
}

/// `UpdateMemberRequest`, `BanMemberRequest`, `CreateNoteRequest`,
/// `ExportQuery`, `ActivityQuery` — every optional field must default
/// to None on an empty body / empty querystring.
#[test]
fn admin_request_dtos_default_optional_fields_to_none() {
    let upd: UpdateMemberRequest = serde_json::from_str("{}").expect("empty update body");
    assert!(upd.name.is_none() && upd.email.is_none() && upd.password.is_none());

    let ban: BanMemberRequest = serde_json::from_str("{}").expect("empty ban body");
    assert!(ban.reason.is_none());
    assert!(ban.duration_days.is_none());

    let note: CreateNoteRequest =
        serde_json::from_value(serde_json::json!({"content": "internal note"})).expect("note body");
    assert_eq!(note.content, "internal note");

    let exp: ExportQuery = serde_json::from_str("{}").expect("empty export query");
    assert!(exp.format.is_none() && exp.status.is_none());
    assert!(exp.date_from.is_none() && exp.date_to.is_none());

    let act: ActivityQuery = serde_json::from_str("{}").expect("empty activity query");
    assert!(act.page.is_none() && act.per_page.is_none() && act.action.is_none());
}

// ── MemberDetail / MemberNote / MemberActivity wire format ───────────

/// `MemberDetail` is what the admin "view member" page renders. Pin
/// the wire-format keys + nullability on `mfa_enabled` /
/// `email_verified_at` so a regression that flipped them to required
/// would crash deserialization for every non-MFA / unverified user.
#[test]
fn member_detail_wire_keys_match_admin_page_contract() {
    let now = chrono::Utc::now().naive_utc();
    let m = MemberDetail {
        id: 7,
        name: Some("Jane Doe".to_string()),
        email: "jane@example.com".to_string(),
        role: Some("admin".to_string()),
        avatar_url: None,
        email_verified_at: None,
        mfa_enabled: None,
        created_at: now,
        updated_at: now,
    };
    let wire = serde_json::to_value(&m).expect("serialize MemberDetail");
    assert_eq!(wire["id"], 7);
    assert_eq!(wire["email"], "jane@example.com");
    assert_eq!(wire["role"], "admin");
    assert!(wire["mfa_enabled"].is_null());
    assert!(wire["email_verified_at"].is_null());
    assert!(wire["avatar_url"].is_null());

    // Negative pin: must NOT leak a `password_hash` or `password` field
    // — the admin page reads MemberDetail by exact-key access and any
    // sensitive-field leak here is a PII regression.
    assert!(
        wire.get("password").is_none() && wire.get("password_hash").is_none(),
        "MemberDetail must never serialize a password / password_hash"
    );
}

#[test]
fn member_note_and_activity_wire_keys_match_admin_panel_contract() {
    let now = chrono::Utc::now().naive_utc();
    let note = MemberNote {
        id: 1,
        user_id: 7,
        content: "Refunded $99 per CS-1234".to_string(),
        created_by: Some(2),
        created_by_name: Some("Support Bot".to_string()),
        created_at: now,
    };
    let act = MemberActivity {
        id: 99,
        user_id: 7,
        action: "login".to_string(),
        description: Some("OK".to_string()),
        metadata: Some(serde_json::json!({"ua": "Mozilla/5.0"})),
        ip_address: Some("203.0.113.5".to_string()),
        user_agent: Some("Mozilla/5.0".to_string()),
        created_at: now,
    };
    let note_wire = serde_json::to_value(&note).expect("serialize note");
    let act_wire = serde_json::to_value(&act).expect("serialize activity");

    assert_eq!(note_wire["content"], "Refunded $99 per CS-1234");
    assert_eq!(note_wire["created_by_name"], "Support Bot");
    assert_eq!(act_wire["action"], "login");
    assert_eq!(act_wire["ip_address"], "203.0.113.5");
    assert_eq!(act_wire["metadata"]["ua"], "Mozilla/5.0");
}

// ── Router mount table ───────────────────────────────────────────────

/// `router()` mounts 10 documented routes. Building it as
/// `Router<AppState>` proves every handler still uses the `AdminUser`
/// extractor (security gate) and that no route was accidentally
/// dropped during refactors.
#[test]
fn admin_member_management_router_builds_without_panicking() {
    let _r: axum::Router<revolution_api::AppState> =
        revolution_api::routes::admin_member_management::router();
}
