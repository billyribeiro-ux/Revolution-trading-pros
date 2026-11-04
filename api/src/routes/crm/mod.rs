//! CRM Admin Routes - Revolution Trading Pros
//! Apple Principal Engineer ICT 7 Grade - January 2026
//!
//! Comprehensive CRM backend matching FluentCRM Pro functionality:
//! - Contacts, Deals, Pipelines
//! - Email Sequences, Recurring Campaigns
//! - Smart Links, Automation Funnels
//! - Lists, Tags, Segments, Companies
//! - Templates, Webhooks, Import/Export
//! - Manager Permissions, Settings, System Logs
//!
//! Split from a single 3467-LOC `crm.rs` file (R3-C maintainability pass,
//! 2026-05-20). Public API unchanged: external callers still see
//! `routes::crm::router()` returning `Router<AppState>`.

use axum::{Json, Router, extract::State, http::StatusCode, routing::get};
use serde::Deserialize;
use serde_json::json;

use crate::{AppState, middleware::admin::AdminUser};

pub mod automations;
mod campaign_audience;
mod campaign_delivery;
mod campaigns;
pub mod companies;
pub mod contacts;
mod deal_activity;
mod deal_mutations;
pub mod deals_pipelines;
pub mod import_export;
pub mod leads;
mod list_members;
pub mod lists_tags;
mod logs;
mod segment_rules;
pub mod segments;
pub mod sequences_campaigns;
pub mod settings;
pub mod smart_links;
pub mod templates;
pub mod webhooks;

// ═══════════════════════════════════════════════════════════════════════════
// SHARED TYPES (used by multiple sub-modules)
// ═══════════════════════════════════════════════════════════════════════════

/// Common query-string filter used by most list endpoints (lists, tags,
/// segments, sequences, campaigns, automations, smart-links, templates,
/// webhooks, companies, system-logs, recurring-campaigns).
#[derive(Debug, Deserialize)]
pub struct ListFilters {
    pub search: Option<String>,
    pub is_public: Option<bool>,
    pub per_page: Option<i64>,
}

// ═══════════════════════════════════════════════════════════════════════════
// HANDLER - Dashboard Stats (cross-domain; lives at the mod root)
// ═══════════════════════════════════════════════════════════════════════════

/// GET /admin/crm/stats - CRM dashboard statistics
async fn get_crm_stats(
    State(state): State<AppState>,
    _admin: AdminUser,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let (stats,):(serde_json::Value,)=sqlx::query_as(
        "SELECT json_build_object('total_contacts',(SELECT COUNT(*) FROM contacts),'active_contacts',(SELECT COUNT(*) FROM contacts WHERE status NOT IN ('churned','unqualified')),'new_this_month',(SELECT COUNT(*) FROM contacts WHERE created_at>=date_trunc('month',CURRENT_DATE)),'total_leads',(SELECT COUNT(*) FROM contacts WHERE status IN ('new','contacted','qualified','proposal','negotiation','won','lost','lead')),'high_score_contacts',(SELECT COUNT(*) FROM contacts WHERE score>=75),'total_deals',(SELECT COUNT(*) FROM crm_deals),'active_deals',(SELECT COUNT(*) FROM crm_deals WHERE status='open'),'open_deals',(SELECT COUNT(*) FROM crm_deals WHERE status='open'),'won_deals',(SELECT COUNT(*) FROM crm_deals WHERE status='won'),'lost_deals',(SELECT COUNT(*) FROM crm_deals WHERE status='lost'),'deals_value_cents',(SELECT ROUND(COALESCE(SUM(amount),0)*100)::bigint FROM crm_deals WHERE status='open' AND currency='USD'),'deal_value',(SELECT COALESCE(SUM(amount),0) FROM crm_deals WHERE status='open' AND currency='USD'),'email_sent',(SELECT COALESCE(SUM(emails_sent),0) FROM crm_campaigns),'email_opened',(SELECT COALESCE(SUM(opens),0) FROM crm_campaigns),'email_clicked',(SELECT COALESCE(SUM(clicks),0) FROM crm_campaigns))"
    ).fetch_one(&state.db.pool).await.map_err(|error| {
        tracing::error!(%error,"CRM statistics query failed");
        (StatusCode::INTERNAL_SERVER_ERROR,Json(json!({"error":"Unable to load CRM statistics"})))
    })?;
    Ok(Json(stats))
}

// ═══════════════════════════════════════════════════════════════════════════
// MASTER ROUTER (preserves the pre-split public API exactly)
// ═══════════════════════════════════════════════════════════════════════════

pub fn router() -> Router<AppState> {
    Router::new()
        // Dashboard Stats
        .route("/stats", get(get_crm_stats))
        // Sub-domain routers (each owns its own paths)
        .merge(leads::router())
        .merge(deals_pipelines::router())
        .merge(deal_activity::router())
        .merge(contacts::router())
        .merge(lists_tags::router())
        .merge(list_members::router())
        .merge(segments::router())
        .merge(sequences_campaigns::router())
        .merge(campaigns::router())
        .merge(automations::router())
        .merge(smart_links::router())
        .merge(templates::router())
        .merge(webhooks::router())
        .merge(companies::router())
        .merge(import_export::router())
        .merge(settings::router())
        .merge(logs::router())
}
