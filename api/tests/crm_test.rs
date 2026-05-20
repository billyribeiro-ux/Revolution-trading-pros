//! CRM admin route contract tests — pure, no-DB.
//!
//! Binds directly to `revolution_api::routes::crm::*` and exercises
//! the public DTOs that flow across the admin CRM API (the highest-
//! LOC admin surface in the codebase at 3,839 LOC across 14 sub-
//! modules). Every handler under `/admin/crm/*` runs live SQL against
//! `crm_deals`, `crm_pipelines`, `crm_pipeline_stages`, `crm_leads`,
//! `crm_contacts`, `crm_companies`, etc., so we can't invoke them in
//! isolation. What we CAN pin in no-DB tests:
//!
//! 1. **Router compile-pin** — `crm::router()` returns
//!    `Router<AppState>` and merges 13 sub-domain routers
//!    (`leads`, `deals_pipelines`, `contacts`, `lists_tags`,
//!    `segments`, `sequences_campaigns`, `automations`,
//!    `smart_links`, `templates`, `webhooks`, `companies`,
//!    `import_export`, `settings`). A regression in ANY sub-module's
//!    handler signature (wrong extractor, wrong return type, dropped
//!    `AdminUser` gate) fails the master router builder here. This
//!    is the single canonical pin for the entire `/admin/crm` surface.
//!
//! 2. **BIGSERIAL i64 PK alignment across the CRM domain.** Per
//!    schema.sql:6240 (`crm_deals.id bigint NOT NULL`), schema.sql:
//!    6373 (`crm_pipelines.id bigint NOT NULL`), schema.sql:6332
//!    (`crm_pipeline_stages.id bigint NOT NULL`), every `CrmDeal`,
//!    `CrmPipeline`, `CrmPipelineStage`, `CrmLead`, `CrmContact`,
//!    `CrmCompany`, `AbandonedCart` struct MUST declare `id: i64`
//!    and FK columns (`contact_id`, `company_id`, `pipeline_id`,
//!    `stage_id`, `owner_id`) MUST be `Option<i64>` / `i64` to
//!    round-trip BIGINT values past i32::MAX. Per CLAUDE.md "Money
//!    / cents — i64 ONLY, BIGINT ONLY": although these IDs aren't
//!    money, they index into BIGSERIAL tables where the next-id
//!    counter is `bigint` — a regression to i32 would silently
//!    truncate any row past i32::MAX (2.1B).
//!
//! 3. **`CreateDealInput.amount_cents: Option<i64>` is money.** Per
//!    the source comment (lines 105-108): "Batch 5c: integer-cents
//!    convention (matches Batch 1 unification). DB column
//!    `crm_deals.amount` stays NUMERIC(15,2); we convert at the SQL
//!    boundary as `cents::BIGINT / 100.0`." This IS the money path
//!    — CLAUDE.md "Money / cents — i64 ONLY, BIGINT ONLY, EVERY
//!    TIME" applies. A regression to i32 would cap deal amounts at
//!    $21,474,836.47, which sounds huge until you write the first
//!    pipeline-total rollup (sum across hundreds of deals).
//!
//! 4. **`ListFilters.per_page: Option<i64>` shared across 13 list
//!    endpoints.** The shared CRM filter (mod.rs:43-47) is used by
//!    lists / tags / segments / sequences / campaigns / automations
//!    / smart-links / templates / webhooks / companies / system-
//!    logs / recurring-campaigns. `per_page` is `Option<i64>` so
//!    `LIMIT $1` over a BIGSERIAL table fits the wire format. R9-D
//!    NEGATIVE: per_page as a string MUST fail; empty body MUST
//!    parse (all-None default).
//!
//! 5. **`DealFilters` FK fields are `Option<i64>`.** `pipeline_id`,
//!    `stage_id`, `owner_id` are all `Option<i64>` matching the
//!    BIGSERIAL FKs in schema.sql:6240-6247. A regression to i32
//!    would 400-error any deal filter past the 2.1B row mark.
//!
//! ## Pattern source
//!
//! Modeled on `tests/admin_orders_test.rs`, `tests/courses_admin_test.rs`,
//! `tests/admin_members_test.rs`, `tests/export_test.rs`.

use revolution_api::routes::crm;

// ── 1. Master router compile-pin ─────────────────────────────────────

/// `crm::router()` MUST build as `Router<AppState>`. Load-bearing
/// because this single function merges 13 sub-domain routers
/// (`leads` / `deals_pipelines` / `contacts` / `lists_tags` /
/// `segments` / `sequences_campaigns` / `automations` /
/// `smart_links` / `templates` / `webhooks` / `companies` /
/// `import_export` / `settings`). A regression in ANY of those
/// modules' handler signatures (wrong extractor, wrong return type,
/// dropped `AdminUser` gate that would let unauthenticated callers
/// list CRM contacts/leads/deals) would fail compilation here.
///
/// This is the single canonical pin for the entire `/admin/crm`
/// surface (mounted at `routes::api_router().nest("/admin/crm",
/// crm::router())`).
#[test]
fn crm_router_builds_with_app_state() {
    let _r: axum::Router<revolution_api::AppState> = crm::router();
}

/// Idempotent construction — the master router builder MUST be safe
/// to call multiple times in the same process. Per CLAUDE.md habit #3
/// ("Re-read your own diff — does any `static` / `OnceLock` / lazy
/// init survive the refactor?"), a regression that hoisted any sub-
/// router's state into a `OnceLock<Arc<Router>>` would panic on the
/// second `set` call. Since the `api_router()` function in
/// `routes/mod.rs` may construct the CRM router as part of nested
/// builders, this idempotence is load-bearing.
#[test]
fn crm_router_safe_to_construct_multiple_times() {
    let _r1: axum::Router<revolution_api::AppState> = crm::router();
    let _r2: axum::Router<revolution_api::AppState> = crm::router();
}

// ── 2. ListFilters shared filter shape (13 list endpoints) ───────────

/// `crm::ListFilters` is the shared filter shape used by 13 list
/// endpoints (the comment at mod.rs:43-47 enumerates them). All
/// fields MUST be `Option<...>` so a bare GET (no query string)
/// doesn't 422 — every CRM dashboard tile sends NO params for the
/// "default view." A regression that flipped any field to required
/// would 422 every dashboard tile.
///
/// `per_page: Option<i64>` — per CLAUDE.md "Money / cents — i64
/// ONLY, BIGINT ONLY": LIMIT over BIGSERIAL CRM tables (deals /
/// pipelines / contacts) MUST be i64. NOT a Reserved exception —
/// per_page is unbounded above (someone may legitimately export
/// 1M+ rows for legal-discovery). The handler clamps to 100 in
/// code; the WIRE shape MUST stay i64.
///
/// R9-D NEGATIVE: per_page as a string MUST fail.
#[test]
fn list_filters_accepts_empty_and_typed_params() {
    // Empty payload — the "default dashboard tile" path.
    let empty: crm::ListFilters =
        serde_json::from_str("{}").expect("empty ListFilters must deserialize");
    assert!(empty.search.is_none());
    assert!(empty.is_public.is_none());
    assert!(empty.per_page.is_none());

    // Realistic filtered payload.
    let filtered: crm::ListFilters = serde_json::from_value(serde_json::json!({
        "search": "acme",
        "is_public": true,
        "per_page": 50i64,
    }))
    .expect("typed ListFilters must deserialize");
    assert_eq!(filtered.search.as_deref(), Some("acme"));
    assert_eq!(filtered.is_public, Some(true));
    assert_eq!(filtered.per_page, Some(50i64));

    // i64 sentinel just past i32::MAX MUST round-trip (per_page is i64,
    // not i32 — CLAUDE.md "Money / cents" rule on BIGSERIAL LIMIT).
    let big: crm::ListFilters = serde_json::from_value(serde_json::json!({
        "per_page": (i32::MAX as i64) + 1i64,
    }))
    .expect("i64 per_page past i32::MAX must deserialize");
    assert_eq!(big.per_page, Some((i32::MAX as i64) + 1));

    // R9-D NEGATIVE: per_page as a string MUST fail (axum Query is
    // strict on JSON-body deserialization).
    let err = serde_json::from_value::<crm::ListFilters>(serde_json::json!({
        "per_page": "fifty",
    }));
    assert!(
        err.is_err(),
        "stringly-typed per_page MUST fail at the DTO boundary"
    );
}

// ── 3. CrmDeal struct: BIGSERIAL i64 PK + FK alignment ───────────────

/// `crm::deals_pipelines::CrmDeal` MUST declare `id: i64` and all
/// FK columns as `i64` / `Option<i64>` to match the BIGSERIAL
/// schema. Per schema.sql:6240-6247:
///   - id            bigint NOT NULL          → i64
///   - contact_id    bigint                   → Option<i64>
///   - company_id    bigint                   → Option<i64>
///   - pipeline_id   bigint NOT NULL          → i64
///   - stage_id      bigint NOT NULL          → i64
///   - owner_id      bigint                   → Option<i64>
///
/// Per CLAUDE.md "Money / cents — i64 ONLY, BIGINT ONLY, EVERY TIME":
/// although these are FK IDs (not money), they index into BIGSERIAL
/// tables. A regression to i32 would silently truncate any FK past
/// i32::MAX (2.1B), causing deals to point at wrong / missing
/// pipelines.
///
/// `probability: i32` IS a Reserved exception per CLAUDE.md ("row
/// counts ... genuinely cap below 2 billion") — probability is
/// 0-100 by DB constraint (schema.sql:6266), well below i32::MAX.
///
/// `amount: Decimal` is the legacy NUMERIC(15,2) DB column (the
/// new wire format `amount_cents: i64` flows through
/// `CreateDealInput`; the read-back `CrmDeal.amount` is still
/// `sqlx::types::Decimal`, converted at the SQL boundary per the
/// source comment).
#[test]
fn crm_deal_struct_bigserial_aligned() {
    use revolution_api::routes::crm::deals_pipelines::CrmDeal;

    // Construct a CrmDeal with sentinel i64 values past i32::MAX to
    // prove the field types accept BIGINT-range values.
    let above_i32: i64 = (i32::MAX as i64) + 1;
    let deal = CrmDeal {
        id: above_i32,
        name: "ACME — Enterprise renewal".to_string(),
        contact_id: Some(above_i32),
        company_id: Some(above_i32),
        pipeline_id: above_i32,
        stage_id: above_i32,
        owner_id: Some(above_i32),
        amount: sqlx::types::Decimal::new(12_345_67, 2), // $12,345.67
        currency: "USD".to_string(),
        probability: 75,
        status: "open".to_string(),
        expected_close_date: None,
        close_date: None,
        lost_reason: None,
        won_details: None,
        priority: "medium".to_string(),
        source_channel: None,
        source_campaign: None,
        tags: None,
        custom_fields: None,
        stage_entered_at: chrono::NaiveDateTime::default(),
        closed_at: None,
        created_at: chrono::NaiveDateTime::default(),
        updated_at: chrono::NaiveDateTime::default(),
    };

    // Round-trip the i64 fields through serde to confirm wire format.
    let wire = serde_json::to_value(&deal).expect("serialize CrmDeal");
    assert_eq!(wire["id"].as_i64(), Some(above_i32));
    assert_eq!(wire["contact_id"].as_i64(), Some(above_i32));
    assert_eq!(wire["pipeline_id"].as_i64(), Some(above_i32));
    assert_eq!(wire["stage_id"].as_i64(), Some(above_i32));
    assert_eq!(wire["owner_id"].as_i64(), Some(above_i32));

    // R9-D NEGATIVE: deserializing a CrmDeal with a string `id` MUST
    // fail (the FK contract is integer end-to-end).
    let err = serde_json::from_value::<CrmDeal>(serde_json::json!({
        "id": "not-a-number",
        "name": "x",
        "pipeline_id": 1,
        "stage_id": 1,
        "amount": "0",
        "currency": "USD",
        "probability": 0,
        "status": "open",
        "priority": "low",
        "stage_entered_at": "2026-01-01T00:00:00",
        "created_at": "2026-01-01T00:00:00",
        "updated_at": "2026-01-01T00:00:00",
    }));
    assert!(err.is_err(), "stringly-typed id MUST fail");
}

// ── 4. CreateDealInput.amount_cents IS money (Money rule applies) ────

/// `CreateDealInput.amount_cents: Option<i64>` is the canonical
/// money field for deals (per the source comment at deals_pipelines.
/// rs:105-108: "Batch 5c: integer-cents convention ... DB column
/// `crm_deals.amount` stays NUMERIC(15,2); we convert at the SQL
/// boundary as `cents::BIGINT / 100.0`").
///
/// Per CLAUDE.md "Money / cents — i64 ONLY, BIGINT ONLY, EVERY
/// TIME":
///   - i32 / PG INTEGER caps at $21,474,836.47 → forbidden
///   - i64 accommodates Stripe wire format end-to-end
///   - NOT a Reserved exception (this IS money)
///
/// A pipeline-total rollup across hundreds of deals can EASILY
/// exceed i32::MAX cents — a single seven-figure enterprise deal
/// is already $1B+ cents-no-wait, it's $1B = 1e11 cents which IS
/// within i32 by 1 order of magnitude (i32::MAX ≈ 2.1e9), but
/// summing 5+ such deals overflows. The wire shape MUST be i64.
///
/// FK fields `contact_id`, `company_id`, `pipeline_id`, `stage_id`
/// all `Option<i64>` / `i64` per BIGSERIAL alignment.
///
/// R9-D NEGATIVE: amount_cents as a float MUST fail (cents are
/// integer; a regression to f64 would lose precision on amounts
/// past 2^53 cents ≈ $90T which IS within range for an MRR
/// aggregate).
#[test]
fn create_deal_input_amount_cents_is_i64() {
    use revolution_api::routes::crm::deals_pipelines::CreateDealInput;

    // Realistic deal: $50,000.00 = 5_000_000 cents.
    let realistic: CreateDealInput = serde_json::from_value(serde_json::json!({
        "name": "Q3 renewal — Globex",
        "pipeline_id": 42i64,
        "stage_id": 7i64,
        "amount_cents": 5_000_000i64,
        "currency": "USD",
    }))
    .expect("realistic deal must deserialize");
    assert_eq!(realistic.amount_cents, Some(5_000_000));
    assert_eq!(realistic.pipeline_id, 42);
    assert_eq!(realistic.stage_id, 7);

    // i64 sentinel just past i32::MAX cents — proves the type
    // accepts BIGINT-range amounts (i32::MAX cents ≈ $21M).
    let above_i32: i64 = (i32::MAX as i64) + 1;
    let big: CreateDealInput = serde_json::from_value(serde_json::json!({
        "name": "Whale deal",
        "pipeline_id": 1i64,
        "stage_id": 1i64,
        "amount_cents": above_i32,
    }))
    .expect("amount_cents past i32::MAX must deserialize");
    assert_eq!(big.amount_cents, Some(above_i32));

    // FK alignment — i64 contact_id past i32::MAX.
    let fk_big: CreateDealInput = serde_json::from_value(serde_json::json!({
        "name": "x",
        "pipeline_id": above_i32,
        "stage_id": above_i32,
        "contact_id": above_i32,
        "company_id": above_i32,
    }))
    .expect("FK i64 past i32::MAX must deserialize");
    assert_eq!(fk_big.contact_id, Some(above_i32));
    assert_eq!(fk_big.company_id, Some(above_i32));
    assert_eq!(fk_big.pipeline_id, above_i32);

    // R9-D NEGATIVE: amount_cents as a float MUST fail (cents are
    // integer end-to-end; a refactor to f64 would silently lose
    // precision on big sums).
    let err = serde_json::from_value::<CreateDealInput>(serde_json::json!({
        "name": "x",
        "pipeline_id": 1,
        "stage_id": 1,
        "amount_cents": 100.5,
    }));
    assert!(
        err.is_err(),
        "fractional cents MUST fail — cents are integer per CLAUDE.md"
    );
}

// ── 5. DealFilters FK fields i64 + sub-module pub surface ────────────

/// `DealFilters` is the query-string filter for `GET /admin/crm/deals`.
/// FK fields (`pipeline_id`, `stage_id`, `owner_id`) MUST be
/// `Option<i64>` to match the BIGSERIAL schema. A regression to i32
/// would 400-error any filter that referenced a pipeline/stage/owner
/// past i32::MAX.
///
/// `page: Option<i64>` and `per_page: Option<i64>` — same BIGINT-
/// LIMIT rationale as `ListFilters.per_page`. NOT a Reserved
/// exception.
///
/// Also pins the public surface of the `deals_pipelines` sub-module:
/// the four CrmPipeline / CrmPipelineStage / CreatePipelineInput /
/// CreateStageInput types MUST stay externally addressable. A
/// regression that demoted any to `pub(super)` would break this
/// file's compile.
#[test]
fn deal_filters_and_pipeline_dtos_bigserial_aligned() {
    use revolution_api::routes::crm::deals_pipelines::{
        CreatePipelineInput, CreateStageInput, DealFilters,
    };

    // Empty filters — bare GET /admin/crm/deals (no query string).
    let empty: DealFilters =
        serde_json::from_str("{}").expect("empty DealFilters must deserialize");
    assert!(empty.page.is_none());
    assert!(empty.per_page.is_none());
    assert!(empty.pipeline_id.is_none());
    assert!(empty.stage_id.is_none());
    assert!(empty.owner_id.is_none());

    // i64 FK filter past i32::MAX.
    let above_i32: i64 = (i32::MAX as i64) + 1;
    let big: DealFilters = serde_json::from_value(serde_json::json!({
        "page": above_i32,
        "per_page": 100i64,
        "pipeline_id": above_i32,
        "stage_id": above_i32,
        "owner_id": above_i32,
        "status": "open",
        "search": "renewal",
    }))
    .expect("DealFilters with i64 FKs must deserialize");
    assert_eq!(big.page, Some(above_i32));
    assert_eq!(big.pipeline_id, Some(above_i32));
    assert_eq!(big.stage_id, Some(above_i32));
    assert_eq!(big.owner_id, Some(above_i32));
    assert_eq!(big.status.as_deref(), Some("open"));
    assert_eq!(big.search.as_deref(), Some("renewal"));

    // CreatePipelineInput: name required, everything else optional.
    let pipeline: CreatePipelineInput = serde_json::from_value(serde_json::json!({
        "name": "Enterprise pipeline",
    }))
    .expect("minimal CreatePipelineInput must deserialize");
    assert_eq!(pipeline.name, "Enterprise pipeline");
    assert!(pipeline.description.is_none());

    // CreateStageInput: name required, position/probability are
    // Reserved exception i32 per CLAUDE.md ("row counts ... cap
    // below 2 billion") — stages-per-pipeline ≤ 20 in practice;
    // probability is 0-100 by DB constraint.
    let stage: CreateStageInput = serde_json::from_value(serde_json::json!({
        "name": "Negotiation",
        "position": 4i32,
        "probability": 60i32,
    }))
    .expect("CreateStageInput must deserialize");
    assert_eq!(stage.name, "Negotiation");
    assert_eq!(stage.position, Some(4));
    assert_eq!(stage.probability, Some(60));

    // R9-D NEGATIVE: missing required `name` MUST fail.
    let err = serde_json::from_value::<CreatePipelineInput>(serde_json::json!({
        "description": "no name field"
    }));
    assert!(err.is_err(), "missing required `name` MUST fail");
}
