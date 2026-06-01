// R23-D: test-scaffold doc comments use narrative list formatting that doesn't
// match clippy's strict CommonMark interpretation. Allow file-wide.
#![allow(clippy::doc_lazy_continuation, clippy::doc_overindented_list_items)]

//! WebSocket route contract tests — pure, no-DB.
//!
//! Binds directly to `revolution_api::routes::websocket` and pins the
//! public DTOs + the `router()` mount table for the real-time
//! trading-alerts WebSocket surface at `/ws/*`.
//!
//! ## Why this shape
//!
//! Every handler in `routes/websocket.rs` either upgrades to a
//! long-lived WebSocket (`ws_handler`) or requires the `AdminUser`
//! extractor (which hits Redis for session lookup), and the
//! `WsConnectionManager` lives inside `AppState`. None of the
//! handlers can be invoked in unit-test isolation. What we CAN pin
//! without a live socket:
//!
//! 1. **Router compile-pin** — `websocket::router()` must build as
//!    `Router<AppState>`. The documented mount table (per
//!    `routes/websocket.rs:667-672`) is 3 endpoints:
//!      - GET  /ws    (WebSocketUpgrade — long-lived alert stream)
//!      - GET  /stats (admin — connection counter)
//!      - POST /test  (admin — broadcast probe)
//!    A regression in any handler signature (wrong `State<>`,
//!    missing `AdminUser`, wrong return-type) fails compilation here.
//!
//! 2. **`WsMessage` wire format** — `#[serde(tag = "type", content =
//!    "payload")]` with PascalCase variant names. The browser
//!    WebSocket client (`src/lib/services/wsClient.ts` on the
//!    frontend) switches on `type` to dispatch to the alert / trade
//!    / video cache updaters. A refactor that flipped the
//!    discriminator would silently break every realtime trade-room
//!    listener — events arrive, no handler fires, screen stays
//!    stale, users miss alerts.
//!
//! 3. **`AlertPayload.id` and `TradePayload.id` are i64** (BIGSERIAL
//!    contract). These IDs flow through to the SPA cache for local
//!    update. A narrowing to i32 would wrap above i32::MAX and the
//!    SPA would apply the alert update to a DIFFERENT row — a
//!    "wrong-alert-shown" bug. CLAUDE.md "Reserved exception" does
//!    NOT apply: alert/trade IDs are primary-key references.
//!
//! 4. **`WsMessage` round-trips through serde** — every variant must
//!    serialize AND deserialize, because the WS protocol is
//!    bidirectional. If a refactor introduced a field that's
//!    `Serialize` but not `Deserialize`, the client would fail to
//!    decode and silently drop the message.
//!
//! 5. **`ClientMessage` accepts subscribe/unsubscribe/ping/pong** —
//!    the client → server protocol. R9-D NEGATIVE pin: a refactor
//!    that dropped any variant would silently break the heartbeat
//!    (`Pong` → server times out client at 60s) or the dynamic-room
//!    subscription flow.
//!
//! 6. **`WsParams.rooms` and `token` are both Optional** — a bare
//!    `GET /ws` (no query string) must NOT 400 at the type-parse
//!    layer. The handler internally rejects anonymous room-requests,
//!    but the DTO itself accepts both none. R9-D NEGATIVE pin.
//!
//! 7. **`TestBroadcastRequest` requires room + message_type +
//!    payload** — the admin debugging endpoint. A refactor that
//!    flipped `room` to Optional would let admins broadcast to ALL
//!    rooms by accident.
//!
//! ## Money contract
//!
//! `routes/websocket.rs` exposes NO i64 *_cents fields, but multiple
//! `f64` price fields (`entry_price`, `exit_price`, `pnl_percent`)
//! — these are PRESENTATION values for the trade-room UI, NOT
//! storage values. They flow from `trades` table (where the source
//! of truth IS i64 cents) through a `f64` projection for display.
//! The CLAUDE.md "i64 only" rule applies to the storage layer; this
//! wire DTO is downstream of that and uses f64 because the SPA
//! plots them on a chart. The pin here is that these stay `f64`,
//! NOT widened to a struct (which would break the chart consumer).
//!
//! ## Pattern source
//!
//! Modeled on `tests/cms_v2_test.rs`, `tests/auth_test.rs`,
//! `tests/cms_delivery_test.rs`.

use chrono::Utc;
use revolution_api::routes::websocket;
use revolution_api::routes::websocket::{
    AlertPayload, ClientMessage, StatsPayload, TestBroadcastRequest, TradePayload,
    TradePlanPayload, VideoPayload, WsMessage, WsParams,
};

// ── 1. router() mount-table compile-pin ──────────────────────────────

/// `websocket::router()` must build as `Router<AppState>`. The
/// documented mount table is 3 endpoints (GET /ws WebSocket upgrade,
/// GET /stats admin-only, POST /test admin-only). A regression in
/// any handler signature fails compilation here.
#[test]
fn websocket_router_builds_with_app_state() {
    let _r: axum::Router<revolution_api::AppState> = websocket::router();
}

// ── 2. WsMessage wire format — tag = "type", content = "payload" ────

/// `WsMessage` is `#[serde(tag = "type", content = "payload")]`. The
/// browser WS client switches on `type`. A refactor that flipped the
/// discriminator would silently break every realtime listener on the
/// trade rooms.
#[test]
fn ws_message_serializes_as_internally_tagged_with_payload() {
    let now = Utc::now();
    let alert = AlertPayload {
        id: 1,
        room_slug: "explosive-swings".to_string(),
        alert_type: "watchlist".to_string(),
        ticker: "AAPL".to_string(),
        title: Some("Strong setup".to_string()),
        message: "Watch this".to_string(),
        notes: None,
        tos_string: None,
        is_new: true,
        is_pinned: false,
        published_at: now,
        created_at: now,
    };
    let message = WsMessage::AlertCreated { alert };

    let wire = serde_json::to_value(&message).expect("WsMessage must serialize");

    // POSITIVE: tag is "type", content is "payload"
    assert_eq!(
        wire.get("type").and_then(|v| v.as_str()),
        Some("AlertCreated"),
        "WsMessage must use `#[serde(tag = \"type\", content = \"payload\")]` — the SPA switches on type"
    );
    let payload = wire
        .get("payload")
        .expect("WsMessage must have a `payload` field");
    assert!(
        payload.is_object(),
        "payload must be an object (the variant struct)"
    );
    // The variant struct holds an `alert` field
    assert_eq!(payload["alert"]["ticker"].as_str(), Some("AAPL"));

    // NEGATIVE: must NOT be externally-tagged
    assert!(
        wire.get("AlertCreated").is_none(),
        "WsMessage must NOT be externally-tagged — `#[serde(tag = ..., content = ...)]` is load-bearing"
    );

    // Heartbeat must serialize with the timestamp inside payload
    let hb = WsMessage::Heartbeat {
        timestamp: 1_700_000_000,
    };
    let hb_wire = serde_json::to_value(&hb).expect("Heartbeat must serialize");
    assert_eq!(hb_wire["type"].as_str(), Some("Heartbeat"));
    assert_eq!(
        hb_wire["payload"]["timestamp"].as_i64(),
        Some(1_700_000_000)
    );
}

// ── 3. AlertPayload + TradePayload IDs are i64 (BIGSERIAL contract) ─

/// `AlertPayload.id`, `TradePayload.id`, `TradePlanPayload.id`, and
/// `VideoPayload.id` are all `i64` matching the BIGSERIAL PKs in
/// `alerts`, `trades`, `trade_plan_entries`, `videos`. A narrowing
/// to i32 would mean alerts with id > 2_147_483_647 wrap and the
/// SPA cache applies the update to a DIFFERENT row — wrong alert
/// shown to traders, who then trade on bad info.
///
/// CLAUDE.md "Reserved exception" (revisions/attempts) does NOT
/// apply: alert/trade IDs are primary-key references, not bounded
/// counters. Trading rooms produce many alerts per day; while
/// reaching i32::MAX is far off, the wire contract must match the
/// storage type.
#[test]
fn alert_trade_payload_ids_are_i64_above_i32_max() {
    let above_i32_max: i64 = (i32::MAX as i64) + 1;
    let now = Utc::now();

    // AlertPayload.id
    let alert = AlertPayload {
        id: above_i32_max,
        room_slug: "explosive-swings".to_string(),
        alert_type: "watchlist".to_string(),
        ticker: "TSLA".to_string(),
        title: None,
        message: "msg".to_string(),
        notes: None,
        tos_string: None,
        is_new: true,
        is_pinned: false,
        published_at: now,
        created_at: now,
    };
    let _: i64 = alert.id; // compile-pin
    let alert_wire = serde_json::to_value(&alert).expect("AlertPayload must serialize");
    assert_eq!(alert_wire["id"].as_i64(), Some(above_i32_max));

    // TradePayload.id
    let trade = TradePayload {
        id: above_i32_max,
        room_slug: "explosive-swings".to_string(),
        ticker: "AAPL".to_string(),
        direction: "long".to_string(),
        status: "open".to_string(),
        entry_price: 150.25,
        exit_price: None,
        pnl_percent: None,
        result: None,
        invalidation_reason: None,
        was_updated: None,
        entry_date: "2026-04-25".to_string(),
        exit_date: None,
    };
    let _: i64 = trade.id; // compile-pin
    let trade_wire = serde_json::to_value(&trade).expect("TradePayload must serialize");
    assert_eq!(trade_wire["id"].as_i64(), Some(above_i32_max));

    // TradePlanPayload.id
    let plan = TradePlanPayload {
        id: above_i32_max,
        room_slug: "weekly-watchlist".to_string(),
        ticker: "MSFT".to_string(),
        bias: "bullish".to_string(),
        entry: None,
        target1: None,
        target2: None,
        target3: None,
        runner: None,
        stop: None,
        options_strike: None,
        options_exp: None,
        notes: None,
    };
    let _: i64 = plan.id;

    // VideoPayload.id
    let video = VideoPayload {
        id: above_i32_max,
        room_slug: "weekly-watchlist".to_string(),
        week_title: "Week 17".to_string(),
        video_title: "v".to_string(),
        video_url: "https://example.com/v".to_string(),
        thumbnail_url: None,
        duration: None,
        published_at: now,
    };
    let _: i64 = video.id;

    // Narrowing proof
    assert!(
        (above_i32_max as i32 as i64) != above_i32_max,
        "narrowing alert/trade/plan/video id to i32 must lose data — wrong-row update on SPA cache is silent corruption"
    );

    // WsMessage::AlertDeleted.alert_id is also i64 — the SPA uses
    // this to remove the row from cache. Wrong-row deletion is bad.
    let deleted = WsMessage::AlertDeleted {
        alert_id: above_i32_max,
    };
    let dw = serde_json::to_value(&deleted).expect("AlertDeleted must serialize");
    assert_eq!(dw["payload"]["alert_id"].as_i64(), Some(above_i32_max));
}

// ── 4. WsMessage round-trips through serde ──────────────────────────

/// `WsMessage` is bidirectional: the server emits it AND the client
/// echoes structured messages (pongs etc). Every variant must
/// roundtrip through serde so the client can decode it. If a
/// refactor introduced a field that's `Serialize` but not
/// `Deserialize`, the client would fail to parse and silently drop
/// the message, which on a heartbeat would cascade to a forced
/// disconnect at 60s timeout.
#[test]
fn ws_message_round_trips_through_serde() {
    let now = Utc::now();
    let variants: Vec<WsMessage> = vec![
        WsMessage::AlertDeleted { alert_id: 1 },
        WsMessage::Heartbeat {
            timestamp: now.timestamp(),
        },
        WsMessage::Connected {
            connection_id: "abc-123".to_string(),
            rooms: vec!["explosive-swings".to_string()],
            timestamp: now.timestamp(),
        },
        WsMessage::Error {
            code: "AUTH_REQUIRED".to_string(),
            message: "missing token".to_string(),
        },
        WsMessage::Subscribed {
            room: "weekly-watchlist".to_string(),
        },
        WsMessage::Unsubscribed {
            room: "explosive-swings".to_string(),
        },
        WsMessage::StatsUpdated {
            stats: StatsPayload {
                room_slug: "explosive-swings".to_string(),
                win_rate: Some(0.62),
                weekly_profit: Some("+$12,345".to_string()),
                active_trades: Some(3),
                closed_this_week: Some(7),
                total_trades: Some(120),
                current_streak: Some(5),
            },
        },
        WsMessage::TradePlanDeleted { entry_id: 9 },
    ];

    for v in variants {
        let json = serde_json::to_string(&v).expect("WsMessage must serialize");
        let _back: WsMessage =
            serde_json::from_str(&json).expect("WsMessage must round-trip through serde");
        // Tag must always be present
        let val: serde_json::Value =
            serde_json::from_str(&json).expect("WsMessage must be valid JSON");
        assert!(
            val.get("type").is_some(),
            "every WsMessage variant must serialize with a `type` discriminator"
        );
    }
}

// ── 5. ClientMessage accepts subscribe/unsubscribe/ping/pong ────────

/// `ClientMessage` is the client → server protocol. It uses
/// `#[serde(tag = "action", content = "data")]`. The frontend WS
/// client sends `{"action": "Subscribe", "data": {"room": "..."}}`
/// to join a room mid-session. A refactor that dropped any variant
/// would silently break the dynamic room-subscription flow.
///
/// R9-D NEGATIVE pin: assert all four variants deserialize from
/// the documented wire format.
#[test]
fn client_message_accepts_all_four_actions() {
    let sub: ClientMessage = serde_json::from_value(serde_json::json!({
        "action": "Subscribe",
        "data": { "room": "explosive-swings" },
    }))
    .expect("ClientMessage::Subscribe must deserialize");
    match sub {
        ClientMessage::Subscribe { room } => assert_eq!(room, "explosive-swings"),
        _ => panic!("expected Subscribe"),
    }

    let unsub: ClientMessage = serde_json::from_value(serde_json::json!({
        "action": "Unsubscribe",
        "data": { "room": "weekly-watchlist" },
    }))
    .expect("ClientMessage::Unsubscribe must deserialize");
    match unsub {
        ClientMessage::Unsubscribe { room } => assert_eq!(room, "weekly-watchlist"),
        _ => panic!("expected Unsubscribe"),
    }

    let ping: ClientMessage = serde_json::from_value(serde_json::json!({
        "action": "Ping",
        "data": { "timestamp": 1_700_000_000_i64 },
    }))
    .expect("ClientMessage::Ping must deserialize");
    match ping {
        ClientMessage::Ping { timestamp } => assert_eq!(timestamp, 1_700_000_000),
        _ => panic!("expected Ping"),
    }

    let pong: ClientMessage = serde_json::from_value(serde_json::json!({
        "action": "Pong",
        "data": { "timestamp": 1_700_000_000_i64 },
    }))
    .expect("ClientMessage::Pong must deserialize");
    match pong {
        ClientMessage::Pong { timestamp } => assert_eq!(timestamp, 1_700_000_000),
        _ => panic!("expected Pong"),
    }

    // NEGATIVE: unknown action must fail
    let unknown = serde_json::from_value::<ClientMessage>(serde_json::json!({
        "action": "RubberDuck",
        "data": {},
    }));
    assert!(
        unknown.is_err(),
        "ClientMessage with unknown action must fail — protocol must be strict"
    );
}

// ── 6. WsParams + TestBroadcastRequest contract ─────────────────────

/// `WsParams.rooms` and `WsParams.token` are both Optional — a bare
/// `GET /ws` (no query string) must NOT 400 at the type-parse layer.
/// The handler internally rejects anonymous room-requests, but the
/// DTO itself must accept both none.
///
/// `TestBroadcastRequest` is the admin debug-broadcast DTO and
/// requires `room` + `message_type` + `payload`. A refactor that
/// flipped `room` to Optional would let admins broadcast to ALL
/// rooms by accident, polluting every active trader's feed.
#[test]
fn ws_params_optional_and_test_broadcast_request_required_negative_pin() {
    // WsParams — empty query OK
    let empty: WsParams = serde_json::from_value(serde_json::json!({}))
        .expect("WsParams must accept empty query — public/global broadcasts have no rooms");
    assert!(empty.rooms.is_none());
    assert!(empty.token.is_none());

    let full: WsParams = serde_json::from_value(serde_json::json!({
        "rooms": "explosive-swings,weekly-watchlist",
        "token": "eyJ.fake.jwt",
    }))
    .expect("WsParams with both fields must parse");
    assert_eq!(
        full.rooms.as_deref(),
        Some("explosive-swings,weekly-watchlist")
    );
    assert_eq!(full.token.as_deref(), Some("eyJ.fake.jwt"));

    // TestBroadcastRequest — all three required
    let valid: TestBroadcastRequest = serde_json::from_value(serde_json::json!({
        "room": "explosive-swings",
        "message_type": "heartbeat",
        "payload": {},
    }))
    .expect("TestBroadcastRequest with all fields must parse");
    assert_eq!(valid.room, "explosive-swings");
    assert_eq!(valid.message_type, "heartbeat");

    // NEGATIVE: every required field
    for missing in &["room", "message_type", "payload"] {
        let mut obj = serde_json::json!({
            "room": "explosive-swings",
            "message_type": "heartbeat",
            "payload": {},
        });
        obj.as_object_mut().unwrap().remove(*missing);
        let r = serde_json::from_value::<TestBroadcastRequest>(obj);
        assert!(
            r.is_err(),
            "TestBroadcastRequest without {missing} must fail — admin probe must require explicit scope"
        );
    }
}
