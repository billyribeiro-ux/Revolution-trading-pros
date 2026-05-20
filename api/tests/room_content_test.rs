//! Room content route contract tests — pure, no-DB.
//!
//! Binds directly to `revolution_api::routes::room_content` and exercises
//! the public DTO surface + both mount-table routers (public + admin).
//! Trade plans / alerts / weekly videos are the paid product — a regression
//! that flips a price-like field's type or removes a route from the admin
//! router silently breaks customer-facing content. These contracts pin the
//! shape the trading-room UI deserializes.

use chrono::NaiveDate;
use revolution_api::routes::room_content::{
    CloseTradeRequest, CreateAlertRequest, CreateTradePlanRequest, CreateTradeRequest,
    CreateWeeklyVideoRequest, ReorderRequest, RoomTrade, UpdateAlertRequest,
    UpdateTradePlanRequest,
};

// ── Request DTO deserialization (frontend → backend contract) ────────

#[test]
fn create_trade_plan_request_accepts_full_payload() {
    let body = serde_json::json!({
        "room_slug": "swings",
        "week_of": "2026-05-18",
        "ticker": "AAPL",
        "bias": "bullish",
        "entry": "215",
        "target1": "220",
        "target2": "225",
        "target3": "230",
        "runner": "240",
        "runner_stop": "212",
        "stop": "210",
        "options_strike": "215C",
        "options_exp": "2026-06-19",
        "notes": "Earnings play",
        "sort_order": 1
    });
    let req: CreateTradePlanRequest = serde_json::from_value(body).expect("trade plan payload");
    assert_eq!(req.room_slug, "swings");
    assert_eq!(req.ticker, "AAPL");
    assert_eq!(req.bias, "bullish");
    assert_eq!(req.entry.as_deref(), Some("215"));
    assert_eq!(req.target1.as_deref(), Some("220"));
    assert_eq!(req.notes.as_deref(), Some("Earnings play"));
    assert_eq!(req.sort_order, Some(1));
}

#[test]
fn create_alert_request_carries_full_tos_fields() {
    // Alerts carry the ThinkOrSwim-format option fields the trade tracker
    // emits. Any change to the field set is a wire-format break.
    let body = serde_json::json!({
        "room_slug": "swings",
        "alert_type": "entry",
        "ticker": "TSLA",
        "title": "Long TSLA Calls",
        "message": "Buy 5 contracts",
        "notes": "Earnings setup",
        "trade_type": "options",
        "action": "BUY",
        "quantity": 5,
        "option_type": "CALL",
        "strike": 250.0,
        "expiration": "2026-06-19",
        "contract_type": "Weeklys",
        "order_type": "LMT",
        "limit_price": 3.50,
        "tos_string": "+5 TSLA 19 JUN 26 250 CALL @3.50 LMT",
        "is_published": true,
        "is_pinned": false
    });
    let req: CreateAlertRequest = serde_json::from_value(body).expect("alert payload");
    assert_eq!(req.ticker, "TSLA");
    assert_eq!(req.alert_type, "entry");
    assert_eq!(req.action.as_deref(), Some("BUY"));
    assert_eq!(req.option_type.as_deref(), Some("CALL"));
    assert_eq!(req.strike, Some(250.0));
    assert_eq!(req.limit_price, Some(3.50));
    assert_eq!(req.is_published, Some(true));
}

#[test]
fn update_alert_request_is_fully_optional() {
    // The PATCH-style update DTO must let every field be omitted — an
    // empty body has to parse to "no changes".
    let empty: UpdateAlertRequest = serde_json::from_str("{}").expect("empty update");
    assert!(empty.ticker.is_none());
    assert!(empty.message.is_none());
    assert!(empty.is_pinned.is_none());

    // Partial update — only pin status.
    let partial: UpdateAlertRequest =
        serde_json::from_str(r#"{"is_pinned": true}"#).expect("partial update");
    assert_eq!(partial.is_pinned, Some(true));
    assert!(partial.ticker.is_none());
}

#[test]
fn create_trade_request_and_close_trade_request_carry_decimal_pnl_inputs() {
    let body = serde_json::json!({
        "room_slug": "swings",
        "ticker": "MSFT",
        "trade_type": "options",
        "direction": "long",
        "quantity": 10,
        "option_type": "CALL",
        "strike": 410.0,
        "expiration": "2026-06-12",
        "contract_type": "Weeklys",
        "entry_price": 4.25,
        "entry_date": "2026-05-19",
        "setup": "post-earnings continuation"
    });
    let open: CreateTradeRequest = serde_json::from_value(body).expect("create trade payload");
    assert_eq!(open.ticker, "MSFT");
    assert_eq!(open.quantity, 10);
    assert_eq!(open.entry_price, 4.25);

    let close_body = serde_json::json!({
        "exit_price": 6.50,
        "exit_date": "2026-05-21",
        "notes": "trimmed at 1st target"
    });
    let close: CloseTradeRequest = serde_json::from_value(close_body).expect("close trade payload");
    assert_eq!(close.exit_price, 6.50);
    assert_eq!(close.exit_date.as_deref(), Some("2026-05-21"));
}

#[test]
fn create_weekly_video_request_requires_room_and_video_fields() {
    let body = serde_json::json!({
        "room_slug": "swings",
        "week_of": "2026-05-18",
        "week_title": "Week of May 18",
        "video_title": "Weekly Market Outlook",
        "video_url": "https://video.bunnycdn.com/play/abc/xyz",
        "video_platform": "bunny",
        "duration": "00:32:15",
        "description": "Macro setup + tickers in focus"
    });
    let req: CreateWeeklyVideoRequest = serde_json::from_value(body).expect("weekly video payload");
    assert_eq!(req.room_slug, "swings");
    assert_eq!(req.week_title, "Week of May 18");
    assert_eq!(req.video_url, "https://video.bunnycdn.com/play/abc/xyz");
    assert_eq!(req.video_platform.as_deref(), Some("bunny"));
}

// ── ReorderRequest: bulk reorder is the shape drag-and-drop emits ────

#[test]
fn reorder_request_carries_id_and_sort_order_pairs() {
    let body = serde_json::json!({
        "items": [
            {"id": 1, "sort_order": 0},
            {"id": 2, "sort_order": 1},
            {"id": 3, "sort_order": 2}
        ]
    });
    let req: ReorderRequest = serde_json::from_value(body).expect("reorder payload");
    assert_eq!(req.items.len(), 3);
    assert_eq!(req.items[0].id, 1);
    assert_eq!(req.items[0].sort_order, 0);
    assert_eq!(req.items[2].sort_order, 2);
}

// ── UpdateTradePlanRequest is fully optional (PATCH semantics) ───────

#[test]
fn update_trade_plan_request_is_fully_optional() {
    let empty: UpdateTradePlanRequest = serde_json::from_str("{}").expect("empty patch");
    assert!(empty.ticker.is_none());
    assert!(empty.is_active.is_none());

    let toggle: UpdateTradePlanRequest =
        serde_json::from_str(r#"{"is_active": false}"#).expect("toggle patch");
    assert_eq!(toggle.is_active, Some(false));
}

// ── RoomTrade row: numeric P&L fields are f64 (DB DOUBLE PRECISION) ──

/// Pin the P&L wire shape. Trades store `pnl` and `pnl_percent` as f64
/// (sqlx DOUBLE PRECISION). The trade tracker UI charts these directly;
/// a regression to Decimal would break JSON parsing on every closed trade.
#[test]
fn room_trade_pnl_fields_serialize_as_f64() {
    let trade = RoomTrade {
        id: 1,
        room_id: 1,
        room_slug: "swings".to_string(),
        ticker: "SPY".to_string(),
        trade_type: "options".to_string(),
        direction: "long".to_string(),
        quantity: 1,
        option_type: Some("CALL".to_string()),
        strike: Some(530.0),
        expiration: Some(NaiveDate::from_ymd_opt(2026, 6, 12).unwrap()),
        contract_type: Some("Weeklys".to_string()),
        entry_alert_id: None,
        entry_price: 2.50,
        entry_date: NaiveDate::from_ymd_opt(2026, 5, 19).unwrap(),
        entry_tos_string: None,
        exit_alert_id: None,
        exit_price: Some(4.00),
        exit_date: Some(NaiveDate::from_ymd_opt(2026, 5, 21).unwrap()),
        exit_tos_string: None,
        setup: None,
        status: "closed".to_string(),
        result: Some("win".to_string()),
        pnl: Some(150.0),
        pnl_percent: Some(60.0),
        holding_days: Some(2),
        notes: None,
        was_updated: Some(false),
        invalidation_reason: None,
        created_at: chrono::Utc::now(),
        updated_at: chrono::Utc::now(),
    };
    let wire = serde_json::to_value(&trade).expect("serialize trade");
    assert_eq!(wire["pnl"].as_f64(), Some(150.0));
    assert_eq!(wire["pnl_percent"].as_f64(), Some(60.0));
    assert_eq!(wire["result"], "win");
    assert_eq!(wire["status"], "closed");
}

// ── Router mount tables (public + admin) ─────────────────────────────

/// Mount-table test: BOTH the public and admin routers must build. The
/// public router serves paid content gated by `ensure_room_access`; the
/// admin router exposes CRUD. Removing either is a customer-facing break.
#[test]
fn room_content_public_and_admin_routers_build() {
    let _pub: axum::Router<revolution_api::AppState> =
        revolution_api::routes::room_content::public_router();
    let _adm: axum::Router<revolution_api::AppState> =
        revolution_api::routes::room_content::admin_router();
}
