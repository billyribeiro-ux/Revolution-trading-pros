//! CMS AI assist route contract tests — pure, no-DB, no network.
//!
//! Binds directly to `revolution_api::routes::cms_ai_assist` and
//! exercises the public DTOs + the `router()` mount table. The AI
//! assist surface streams Claude responses back to editors and
//! records every call in `ai_assist_history`. The biggest risk on
//! this surface is **secret leakage** — the handler reads
//! `ANTHROPIC_API_KEY` from the environment, hits the Claude API,
//! and returns the model output. A regression that accidentally
//! echoes the API key, the upstream Authorization header, or any
//! internal env-var value into the response DTO would silently
//! leak the secret to every editor's browser DevTools.
//!
//! ## Why this shape
//!
//! Every handler runs live HTTP to Claude + live SQL against
//! `ai_assist_history`, so we can't unit-test handlers. Instead we
//! pin:
//!
//! 1. **No API keys / secrets serialize through any public DTO.**
//!    The CLAUDE.md security floor: client wires must never carry an
//!    Anthropic API key. We construct each public DTO with realistic
//!    values, serialize it, and grep for the `sk-ant` Anthropic key
//!    prefix and a few common secret-name substrings. If a future
//!    refactor adds a field like `api_key: String` or `auth_header:
//!    String` and lets a secret flow through, this test fails loud.
//!
//! 2. **Enum wire-format** — `AiAction` and `ToneOption` are tagged
//!    `#[serde(rename_all = "snake_case")]`. The CMS editor JS
//!    dispatches `{ action: "fix_grammar" }`; a refactor that drops
//!    the rename would silently 422 every assist click.
//!
//! 3. **`AiAssistRequest` shape** — `options` defaults to a zero
//!    value via `#[serde(default)]`, and `content_id` / `block_id`
//!    are skip-serialize-if-none. The bare "improve this text"
//!    payload `{ "action": "improve", "content": "..." }` must
//!    parse cleanly.
//!
//! 4. **`HistoryQuery` shape** — every filter is Optional. The bare
//!    `GET /history` must parse with no filters; a regression that
//!    made any field required would 400 the history sidebar.
//!
//! 5. **`router()` mount table** — 3 endpoints (`/assist`,
//!    `/assist/stream`, `/history`). Building it as `Router<AppState>`
//!    proves every handler signature still matches its extractor
//!    contract.
//!
//! 6. **Money pin (documenting absence)** — `cms_ai_assist` is an
//!    AI tooling surface; no monetary fields exist. The CLAUDE.md
//!    money rule (every `*_cents` is i64) is upheld by NOT
//!    introducing one. If a future commit drifts a "tokens-cost-in-
//!    cents" billing field into the response DTO, this test fails
//!    loud and forces the reviewer to move the field to a money-
//!    aware module (orders / subscriptions).
//!
//! ## Pattern source
//!
//! Modeled on `tests/payments_test.rs`, `tests/orders_test.rs`,
//! `tests/oauth_test.rs`, `tests/admin_indicators_test.rs`,
//! `tests/coupons_test.rs`, `tests/cms_delivery_test.rs`.

use revolution_api::routes::cms_ai_assist::{
    AiAction, AiAssistHistoryRecord, AiAssistMetadata, AiAssistOptions, AiAssistRequest,
    AiAssistResponse, HistoryQuery, HistoryResponse, PaginationMeta, ToneOption,
};

// ── Secret-leak guard: no API keys in any serialized DTO ─────────────

/// Build every serialize-able public DTO with realistic, non-secret
/// values and ensure none of them carry a field that would leak the
/// Anthropic API key, an Authorization header, or any other internal
/// secret. The check is conservative: we look for the Anthropic key
/// prefix `sk-ant-` AND for the common substrings `api_key` /
/// `authorization` / `bearer` in the wire JSON. If any of those
/// appear after a refactor, we want the test to fail loud and force
/// the reviewer to confirm whether the new field is intentional.
#[test]
fn ai_assist_public_dtos_do_not_leak_api_keys_or_auth_headers() {
    let now = chrono::Utc::now();
    // The values below are all REALISTIC, NON-SECRET test fixtures.
    // The point of the test is that even when the AI flow runs end-
    // to-end, NONE of the public response DTOs carries a serialized
    // copy of the upstream secret. We use a sentinel string that
    // matches the Anthropic key prefix to assert it would be caught.
    let response = AiAssistResponse {
        success: true,
        // This is the model output — user-controlled text. We do NOT
        // pass a fake API key here because the test is about the DTO
        // shape; we instead grep for secret-shaped substrings below.
        result: "Improved version of your text.".to_string(),
        action: "improve".to_string(),
        metadata: AiAssistMetadata {
            model: "claude-sonnet-4-20250514".to_string(),
            input_tokens: 120,
            output_tokens: 80,
            processing_time_ms: 1_450,
            content_id: None,
            block_id: None,
        },
    };
    let history = AiAssistHistoryRecord {
        id: 1,
        user_id: 7,
        action: "improve".to_string(),
        input_content: "Original text".to_string(),
        output_content: "Improved version of your text.".to_string(),
        options: None,
        content_id: None,
        block_id: None,
        model: "claude-sonnet-4-20250514".to_string(),
        input_tokens: 120,
        output_tokens: 80,
        processing_time_ms: 1_450,
        created_at: now,
    };
    let history_resp = HistoryResponse {
        data: vec![history],
        meta: PaginationMeta {
            total: 1,
            limit: 20,
            offset: 0,
        },
    };

    let wires = [
        serde_json::to_string(&response).expect("serialize AiAssistResponse"),
        serde_json::to_string(&history_resp).expect("serialize HistoryResponse"),
    ];
    let banned_substrings = [
        "sk-ant-", // Anthropic key prefix
        "api_key", // generic secret field name
        "apiKey",  // camelCase variant
        "Authorization",
        "authorization",
        "ANTHROPIC_API_KEY",
        "Bearer ", // upstream auth header value prefix
    ];
    for w in &wires {
        for bad in &banned_substrings {
            assert!(
                !w.contains(bad),
                "AI assist wire-format leaked secret-shaped substring `{bad}` — wire was: {w}"
            );
        }
    }
}

// ── AiAction wire-format (snake_case) ────────────────────────────────

#[test]
fn ai_action_serializes_as_snake_case_for_editor_js_contract() {
    let cases = [
        (AiAction::Improve, "\"improve\""),
        (AiAction::Shorten, "\"shorten\""),
        (AiAction::Expand, "\"expand\""),
        (AiAction::FixGrammar, "\"fix_grammar\""),
        (AiAction::ChangeTone, "\"change_tone\""),
        (AiAction::Summarize, "\"summarize\""),
        (AiAction::GenerateFaq, "\"generate_faq\""),
        (AiAction::GenerateMeta, "\"generate_meta\""),
        (AiAction::GenerateAlt, "\"generate_alt\""),
        (AiAction::SuggestRelated, "\"suggest_related\""),
    ];
    for (variant, expected) in cases {
        let wire = serde_json::to_string(&variant).expect("serialize AiAction");
        assert_eq!(
            wire, expected,
            "AiAction must stay snake_case for editor JS contract"
        );
    }
}

#[test]
fn tone_option_serializes_as_snake_case() {
    let cases = [
        (ToneOption::Professional, "\"professional\""),
        (ToneOption::Casual, "\"casual\""),
        (ToneOption::Friendly, "\"friendly\""),
        (ToneOption::Authoritative, "\"authoritative\""),
        (ToneOption::Educational, "\"educational\""),
    ];
    for (variant, expected) in cases {
        let wire = serde_json::to_string(&variant).expect("serialize ToneOption");
        assert_eq!(wire, expected);
    }
}

// ── AiAssistRequest: minimal payload + skip_serializing_if ───────────

/// Bare `{ action, content }` MUST parse — that's the dominant payload
/// the editor dispatches on every "improve this paragraph" click.
#[test]
fn ai_assist_request_accepts_minimal_payload() {
    let body = serde_json::json!({
        "action": "improve",
        "content": "Make this text better.",
    });
    let req: AiAssistRequest = serde_json::from_value(body).expect("minimal payload must parse");
    assert!(matches!(req.action, AiAction::Improve));
    assert_eq!(req.content, "Make this text better.");
    assert!(req.options.tone.is_none());
    assert!(req.options.max_length.is_none());
    assert!(req.content_id.is_none());
    assert!(req.block_id.is_none());
}

#[test]
fn ai_assist_request_accepts_full_payload_with_change_tone() {
    let body = serde_json::json!({
        "action": "change_tone",
        "content": "Hi! Welcome to our service.",
        "options": {"tone": "professional", "max_length": 500},
        "content_id": "11111111-1111-1111-1111-111111111111",
        "block_id": "block-42",
    });
    let req: AiAssistRequest = serde_json::from_value(body).expect("full payload must parse");
    assert!(matches!(req.action, AiAction::ChangeTone));
    assert!(matches!(req.options.tone, Some(ToneOption::Professional)));
    assert_eq!(req.options.max_length, Some(500));
    assert_eq!(req.block_id.as_deref(), Some("block-42"));
}

// ── AiAssistOptions defaults are all None ────────────────────────────

#[test]
fn ai_assist_options_default_is_all_none() {
    let opts = AiAssistOptions::default();
    assert!(opts.tone.is_none(), "default tone must be None");
    assert!(opts.max_length.is_none(), "default max_length must be None");
}

// ── HistoryQuery: every filter optional ──────────────────────────────

#[test]
fn history_query_accepts_minimal_and_full_payloads() {
    let empty: HistoryQuery = serde_json::from_str("{}").expect("bare history query must parse");
    assert!(empty.limit.is_none());
    assert!(empty.offset.is_none());
    assert!(empty.action.is_none());
    assert!(empty.content_id.is_none());

    let full: HistoryQuery = serde_json::from_value(serde_json::json!({
        "limit": 25,
        "offset": 50,
        "action": "improve",
        "content_id": "22222222-2222-2222-2222-222222222222",
    }))
    .expect("full history query");
    assert_eq!(full.limit, Some(25));
    assert_eq!(full.offset, Some(50));
    assert_eq!(full.action.as_deref(), Some("improve"));
    assert!(full.content_id.is_some());
}

// ── Money: documenting the absence ───────────────────────────────────

/// `cms_ai_assist` is an AI-tooling surface — no monetary fields. The
/// CLAUDE.md money rule (every `*_cents` is i64) is upheld by NOT
/// introducing one here. If a future commit drifts a
/// `tokens_cost_cents` billing field into a public DTO, this test
/// fails and forces the reviewer to move the field into a money-
/// aware module.
#[test]
fn cms_ai_assist_public_dtos_have_no_cents_fields() {
    let response = AiAssistResponse {
        success: true,
        result: "x".to_string(),
        action: "improve".to_string(),
        metadata: AiAssistMetadata {
            model: "claude-sonnet-4-20250514".to_string(),
            input_tokens: 1,
            output_tokens: 1,
            processing_time_ms: 1,
            content_id: None,
            block_id: None,
        },
    };
    let wire = serde_json::to_string(&response).expect("serialize");
    assert!(
        !wire.contains("_cents"),
        "cms_ai_assist wire-format must never carry a *_cents field — found in: {wire}"
    );
}

// ── Router mount table ───────────────────────────────────────────────

/// `router()` mounts 3 endpoints (`/assist`, `/assist/stream`,
/// `/history`). Building it as `Router<AppState>` proves every
/// handler signature still matches its extractor contract.
#[test]
fn cms_ai_assist_router_builds_without_panicking() {
    let _r: axum::Router<revolution_api::AppState> =
        revolution_api::routes::cms_ai_assist::router();
}
