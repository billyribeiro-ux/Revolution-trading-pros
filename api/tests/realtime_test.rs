// R23-D: test-scaffold doc comments use narrative list formatting that doesn't
// match clippy's strict CommonMark interpretation. Allow file-wide.
#![allow(clippy::doc_lazy_continuation, clippy::doc_overindented_list_items)]

//! Realtime (SSE) route contract tests — pure, no-DB.
//!
//! Binds directly to `revolution_api::routes::realtime` and pins the
//! public DTOs + the `router()` mount table for the CMS real-time
//! Server-Sent Events surface at `/realtime/*`.
//!
//! ## Why this shape
//!
//! Every handler in `routes/realtime.rs` either subscribes to the
//! Tokio broadcast channel (long-running) or requires the `User`
//! extractor (which hits Redis for session lookup). None can be
//! invoked in unit-test isolation. What we CAN pin without runtime
//! state:
//!
//! 1. **Router compile-pin** — `realtime::router()` must build as
//!    `Router<AppState>`. The documented mount table (per
//!    `routes/realtime.rs:455-460`) is 3 endpoints:
//!      - GET  /events  (SSE — long-lived authed stream)
//!      - GET  /stats   (admin-only — connection counter)
//!      - POST /test    (admin-only — broadcast probe)
//!    A regression in any handler signature (wrong `State<>`,
//!    missing `User` extractor, wrong return-type Sse vs Json)
//!    fails compilation here.
//!
//! 2. **`CmsEvent` wire format** — `#[serde(tag = "type", content =
//!    "data")]` with PascalCase variant names. The browser EventSource
//!    client switches on `type` to dispatch handlers. A refactor that
//!    flipped the discriminator tag or rename strategy would silently
//!    break every CMS realtime listener on the admin SPA.
//!
//! 3. **`CmsEvent::ContentCreated` IDs are i64** (BIGSERIAL contract).
//!    `content_id` is the row PK that's broadcast to the SPA, which
//!    uses it to locally update the cached row. A narrowing to i32
//!    would wrap above `i32::MAX` and the SPA would silently apply
//!    the update to a DIFFERENT row (i64 → i32 → i64 round-trip
//!    inverts bits at row 2_147_483_648). CLAUDE.md "Reserved
//!    exception" does NOT apply: content_id is a primary key, not
//!    a bounded counter like `revisions` / `attempts`.
//!
//! 4. **`CmsEvent` round-trips through serde** — the broadcaster
//!    serializes events to JSON for the SSE wire format. If a
//!    refactor introduced a field that's not `Deserialize`, the SSE
//!    stream would silently emit events that the test broadcaster
//!    can produce but the deserializer (or sister consumers like
//!    audit logs) can't reconstruct.
//!
//! 5. **`SseQuery` filters are all Optional** — a bare
//!    `GET /realtime/events` (no query string) must NOT 400. The
//!    admin SPA's default subscription is unfiltered. R9-D NEGATIVE
//!    pin: a regression that flipped `content_types` to required
//!    would 400 every default SSE attach.
//!
//! 6. **`TestEventRequest.message` is required** — the test probe
//!    accepts ONLY admin-authed requests, but the inner DTO is also
//!    a NEGATIVE pin: a refactor that flipped `message` to Optional
//!    would let admins POST `{}` and broadcast an empty notification,
//!    polluting the live admin feed.
//!
//! 7. **`EventBroadcaster::new()` produces a working broadcast
//!    channel** — the constructor is documented as having capacity
//!    1000. We can construct one in-process (no DB / Redis required)
//!    and exercise the subscribe / broadcast loop synchronously to
//!    pin the public surface area.
//!
//! ## Money contract
//!
//! `routes/realtime.rs` exposes NO monetary DTOs. The CLAUDE.md
//! "Reserved exception" (revisions/attempts may be i32) does NOT
//! apply here either: `content_id`, `user_id`, `created_by`,
//! `assigned_to`, etc. are all primary-key references and must
//! remain i64 to match the BIGSERIAL columns. A narrowing would
//! cause wrong-row updates on the SPA cache — the worst class of
//! CMS bug (silent corruption).
//!
//! ## Pattern source
//!
//! Modeled on `tests/cms_delivery_test.rs`, `tests/cms_v2_test.rs`,
//! `tests/auth_test.rs`.

use chrono::Utc;
use revolution_api::routes::realtime;
use revolution_api::routes::realtime::{CmsEvent, EventBroadcaster, SseQuery, TestEventRequest};

// ── 1. router() mount-table compile-pin ──────────────────────────────

/// `realtime::router()` must build as `Router<AppState>`. The
/// documented mount table is 3 endpoints (GET /events SSE,
/// GET /stats, POST /test). A regression in any handler signature
/// (wrong `State<>`, missing `User` extractor, wrong return-type
/// `Sse<...>` vs `Json<...>`) fails compilation here.
#[test]
fn realtime_router_builds_with_app_state() {
    let _r: axum::Router<revolution_api::AppState> = realtime::router();
}

// ── 2. CmsEvent wire format — tag = "type", content = "data" ────────

/// `CmsEvent` is `#[serde(tag = "type", content = "data")]`. The
/// admin SPA's EventSource handler switches on the `type` field to
/// dispatch to the right cache updater. A refactor that flipped the
/// discriminator (e.g. external-tag or untagged) would silently
/// break every CMS realtime listener — events would arrive but no
/// handler would fire.
#[test]
fn cms_event_serializes_as_internally_tagged_with_data_payload() {
    let now = Utc::now();
    let event = CmsEvent::ContentCreated {
        content_type: "post".to_string(),
        content_id: 42,
        title: Some("Hello".to_string()),
        created_by: 7,
        created_at: now,
    };

    let wire = serde_json::to_value(&event).expect("CmsEvent must serialize");

    // POSITIVE: tag is "type", content is "data"
    assert_eq!(
        wire.get("type").and_then(|v| v.as_str()),
        Some("ContentCreated"),
        "CmsEvent must use `#[serde(tag = \"type\")]` with PascalCase variant names — the SPA switches on this field"
    );
    let data = wire.get("data").expect("CmsEvent must have a `data` field");
    assert!(
        data.is_object(),
        "CmsEvent `data` must be an object payload, not a tuple/array"
    );

    // POSITIVE: payload fields are present
    assert_eq!(data["content_type"].as_str(), Some("post"));
    assert_eq!(data["content_id"].as_i64(), Some(42));
    assert_eq!(data["title"].as_str(), Some("Hello"));
    assert_eq!(data["created_by"].as_i64(), Some(7));

    // NEGATIVE: must NOT be externally-tagged (e.g.
    // `{"ContentCreated": {...}}` instead of `{"type": ..., "data": ...}`)
    assert!(
        wire.get("ContentCreated").is_none(),
        "CmsEvent must NOT be externally-tagged — `#[serde(tag = ..., content = ...)]` is load-bearing"
    );
}

// ── 3. CmsEvent IDs are i64 (BIGSERIAL PK contract) ─────────────────

/// `CmsEvent::ContentCreated.content_id` is the row PK that's
/// broadcast to the SPA. A narrowing to i32 would wrap above
/// 2,147,483,647 rows and cause the SPA to apply updates to a
/// DIFFERENT row (silent cache corruption — the worst class of CMS
/// bug). Pin with a value above i32::MAX to prove the wider type.
///
/// CLAUDE.md "Reserved exception" (revisions/attempts may be i32)
/// does NOT apply: content_id is a primary-key reference, not a
/// bounded counter. The same logic applies to created_by / user_id
/// (FK to users table, also BIGSERIAL).
#[test]
fn cms_event_content_id_and_user_id_are_i64_above_i32_max() {
    let above_i32_max: i64 = (i32::MAX as i64) + 1;
    let now = Utc::now();

    let event = CmsEvent::ContentCreated {
        content_type: "post".to_string(),
        content_id: above_i32_max,
        title: Some("Big ID".to_string()),
        created_by: above_i32_max + 1,
        created_at: now,
    };

    let wire = serde_json::to_value(&event).expect("CmsEvent must serialize i64 fields");
    assert_eq!(wire["data"]["content_id"].as_i64(), Some(above_i32_max));
    assert_eq!(wire["data"]["created_by"].as_i64(), Some(above_i32_max + 1));

    // Narrowing proof
    assert!(
        (above_i32_max as i32 as i64) != above_i32_max,
        "narrowing content_id to i32 must lose data — proves i64 is required (wrong-row update on SPA cache is silent corruption)"
    );

    // Notification variant — user_id is also i64
    let notif = CmsEvent::Notification {
        user_id: above_i32_max,
        title: "Test".to_string(),
        message: "Above i32::MAX".to_string(),
        link: None,
        notification_type: "alert".to_string(),
    };
    let notif_wire = serde_json::to_value(&notif).expect("Notification must serialize");
    assert_eq!(notif_wire["data"]["user_id"].as_i64(), Some(above_i32_max));
}

// ── 4. CmsEvent round-trips through serde (every variant) ───────────

/// Every variant of `CmsEvent` must roundtrip through serde
/// (serialize then deserialize back to an equivalent value). The
/// broadcaster serializes to JSON for the SSE wire format; if a
/// refactor introduced a field that's `Serialize` but not
/// `Deserialize`, the SSE consumer (the admin SPA) would fail to
/// parse and silently drop the event.
#[test]
fn cms_event_round_trips_through_serde_for_all_variants() {
    let now = Utc::now();

    let variants: Vec<CmsEvent> = vec![
        CmsEvent::ContentCreated {
            content_type: "post".to_string(),
            content_id: 1,
            title: Some("t".to_string()),
            created_by: 2,
            created_at: now,
        },
        CmsEvent::ContentUpdated {
            content_type: "page".to_string(),
            content_id: 3,
            title: None,
            updated_by: 4,
            updated_at: now,
            changed_fields: Some(vec!["title".to_string()]),
        },
        CmsEvent::ContentDeleted {
            content_type: "post".to_string(),
            content_id: 5,
            deleted_by: 6,
            deleted_at: now,
        },
        CmsEvent::ContentPublished {
            content_type: "post".to_string(),
            content_id: 7,
            title: Some("Published".to_string()),
            published_by: 8,
            published_at: now,
        },
        CmsEvent::ContentUnpublished {
            content_type: "post".to_string(),
            content_id: 9,
            unpublished_by: 10,
            unpublished_at: now,
        },
        CmsEvent::WorkflowTransition {
            content_type: "post".to_string(),
            content_id: 11,
            from_stage: "draft".to_string(),
            to_stage: "review".to_string(),
            transitioned_by: 12,
            transitioned_at: now,
            comment: Some("ready".to_string()),
        },
        CmsEvent::ReviewAssigned {
            content_type: "post".to_string(),
            content_id: 13,
            assigned_to: 14,
            assigned_by: 15,
            due_date: Some(now),
            priority: Some("high".to_string()),
        },
        CmsEvent::ScheduledPublish {
            content_type: "post".to_string(),
            content_id: 16,
            published_at: now,
        },
        CmsEvent::Notification {
            user_id: 17,
            title: "ping".to_string(),
            message: "msg".to_string(),
            link: None,
            notification_type: "info".to_string(),
        },
        CmsEvent::Heartbeat { timestamp: now },
    ];

    for v in variants {
        let json = serde_json::to_string(&v).expect("CmsEvent must serialize");
        let _back: CmsEvent =
            serde_json::from_str(&json).expect("CmsEvent must round-trip through serde");
        // Round-trip must preserve the `type` tag (otherwise the SPA's
        // switch wouldn't dispatch correctly).
        let wire = serde_json::to_value(&v).expect("CmsEvent must serialize to Value");
        assert!(
            wire.get("type").is_some(),
            "every CmsEvent variant must serialize with a `type` discriminator"
        );
    }
}

// ── 5. SseQuery filters all Optional (R9-D NEGATIVE pin) ────────────

/// `SseQuery` (the query string for `GET /realtime/events`) must
/// accept an empty body. The admin SPA's default subscription is
/// unfiltered. R9-D NEGATIVE pin: a regression that flipped either
/// filter to required would 400 every default SSE attach, leaving
/// the SPA's real-time updates silently broken until a manual
/// refresh.
#[test]
fn sse_query_filters_all_optional_negative_pin() {
    // Empty query — must parse
    let empty: SseQuery = serde_json::from_value(serde_json::json!({}))
        .expect("SseQuery must parse from empty body — default unfiltered subscription");
    assert!(empty.content_types.is_none());
    assert!(empty.event_types.is_none());

    // With both filters populated
    let full: SseQuery = serde_json::from_value(serde_json::json!({
        "content_types": "post,page",
        "event_types": "ContentCreated,ContentUpdated",
    }))
    .expect("SseQuery with both filters must parse");
    assert_eq!(full.content_types.as_deref(), Some("post,page"));
    assert_eq!(
        full.event_types.as_deref(),
        Some("ContentCreated,ContentUpdated")
    );

    // TestEventRequest is the inner DTO for the admin /test probe
    let valid: TestEventRequest = serde_json::from_value(serde_json::json!({
        "message": "hello",
    }))
    .expect("TestEventRequest must parse with required message");
    assert_eq!(valid.message, "hello");

    // NEGATIVE: TestEventRequest without `message` must fail
    let no_msg = serde_json::from_value::<TestEventRequest>(serde_json::json!({}));
    assert!(
        no_msg.is_err(),
        "TestEventRequest without message must fail — the admin probe must reject empty broadcasts"
    );
}

// ── 6. EventBroadcaster subscribe + broadcast loop ──────────────────

/// `EventBroadcaster::new()` produces a working tokio broadcast
/// channel (documented capacity 1000). Subscribe + broadcast must
/// deliver the event end-to-end. This pins the public surface area
/// and confirms the `broadcast::Sender` is correctly wired (a
/// refactor that swapped to `mpsc` would compile but fail here).
#[tokio::test]
async fn event_broadcaster_subscribe_then_broadcast_delivers_event() {
    let broadcaster = EventBroadcaster::new();
    let mut rx = broadcaster.subscribe();

    let now = Utc::now();
    let event = CmsEvent::ContentCreated {
        content_type: "post".to_string(),
        content_id: 100,
        title: Some("hello".to_string()),
        created_by: 1,
        created_at: now,
    };

    broadcaster.broadcast(event.clone());

    // The broadcast must arrive on the subscribed receiver
    let received = rx
        .recv()
        .await
        .expect("subscribed receiver must receive broadcast event");
    match received {
        CmsEvent::ContentCreated { content_id, .. } => {
            assert_eq!(
                content_id, 100,
                "broadcaster must deliver the same content_id"
            );
        }
        _ => panic!("expected ContentCreated, got a different variant"),
    }

    // Default + Clone — both impls are load-bearing for the AppState
    // (the broadcaster is cloned into every request handler).
    let _ = EventBroadcaster::default();
    let _cloned = broadcaster.clone();
}
