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

use axum::{Json, Router, extract::State, routing::get};
use serde::Deserialize;

use crate::{
    AppState, routes::crm::manager_permissions::CrmUser as AdminUser, utils::errors::ApiError,
};

pub mod automations;
mod campaign_audience;
mod campaign_delivery;
mod campaigns;
pub(crate) use campaign_audience::{ResolvedAudience, resolve as resolve_campaign_audience};
pub(crate) use campaign_delivery::freeze_on;
pub(crate) use campaigns::row_on as campaign_definition_on;
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
mod manager_permissions;
mod manager_sources;
mod segment_rules;
pub mod segments;
mod sequence_delivery;
mod sequence_enrollment;
pub use sequence_delivery::{run as run_sequences, stage_one as stage_sequence_delivery};
mod sequence_model;
mod sequence_triggers;
pub use sequence_triggers::process_one as process_sequence_trigger;
mod sequences;
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
    admin: AdminUser,
) -> Result<Json<serde_json::Value>, ApiError> {
    let mut tx = manager_sources::read_transaction(&state.db.pool).await?;
    manager_sources::campaign_all_on(&mut tx, &admin).await?;
    let (stats,):(serde_json::Value,)=sqlx::query_as(
        "SELECT json_build_object('total_contacts',(SELECT COUNT(*) FROM contacts),'active_contacts',(SELECT COUNT(*) FROM contacts WHERE status NOT IN ('churned','unqualified')),'new_this_month',(SELECT COUNT(*) FROM contacts WHERE created_at>=date_trunc('month',CURRENT_DATE)),'total_leads',(SELECT COUNT(*) FROM contacts WHERE status IN ('new','contacted','qualified','proposal','negotiation','won','lost','lead')),'high_score_contacts',(SELECT COUNT(*) FROM contacts WHERE score>=75 AND score_assessment->>'status'='calculated' AND score_assessed_at IS NOT NULL AND score_assessment->'policy'=(SELECT value->'scoring_settings' FROM settings WHERE key='crm.settings') AND score_assessment->'score'=to_jsonb(score)),'measured_score_contacts',(SELECT COUNT(*) FROM contacts WHERE score_assessment->>'status'='calculated' AND score_assessed_at IS NOT NULL AND score_assessment->'policy'=(SELECT value->'scoring_settings' FROM settings WHERE key='crm.settings') AND score_assessment->'score'=to_jsonb(score)),'unknown_score_contacts',(SELECT COUNT(*) FROM contacts WHERE NOT COALESCE((score_assessment->>'status'='calculated' AND score_assessed_at IS NOT NULL AND score_assessment->'policy'=(SELECT value->'scoring_settings' FROM settings WHERE key='crm.settings') AND score_assessment->'score'=to_jsonb(score)),false)),'total_deals',(SELECT COUNT(*) FROM crm_deals),'active_deals',(SELECT COUNT(*) FROM crm_deals WHERE status='open'),'open_deals',(SELECT COUNT(*) FROM crm_deals WHERE status='open'),'won_deals',(SELECT COUNT(*) FROM crm_deals WHERE status='won'),'lost_deals',(SELECT COUNT(*) FROM crm_deals WHERE status='lost'),'deals_value_cents',(SELECT ROUND(COALESCE(SUM(amount),0)*100)::bigint FROM crm_deals WHERE status='open' AND currency='USD'),'deal_value',(SELECT COALESCE(SUM(amount),0) FROM crm_deals WHERE status='open' AND currency='USD'),'email_sent',(SELECT COALESCE(SUM(emails_sent),0) FROM crm_campaigns WHERE native_email_campaign_id IS NULL AND NOT EXISTS(SELECT 1 FROM crm_sequence_engines e WHERE e.campaign_id=crm_campaigns.id)),'email_opened',(SELECT COALESCE(SUM(opens),0) FROM crm_campaigns WHERE native_email_campaign_id IS NULL AND NOT EXISTS(SELECT 1 FROM crm_sequence_engines e WHERE e.campaign_id=crm_campaigns.id)),'email_clicked',(SELECT COALESCE(SUM(clicks),0) FROM crm_campaigns WHERE native_email_campaign_id IS NULL AND NOT EXISTS(SELECT 1 FROM crm_sequence_engines e WHERE e.campaign_id=crm_campaigns.id)))"
    ).fetch_one(&mut *tx).await.map_err(|error| {
        tracing::error!(%error,"CRM statistics query failed");
        ApiError::database_error("Unable to load CRM statistics")
    })?;
    tx.commit()
        .await
        .map_err(|_| ApiError::database_error("Unable to finish CRM statistics"))?;
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
        .merge(sequences::router())
        .merge(sequence_enrollment::router())
        .merge(sequence_triggers::router())
        .merge(campaigns::router())
        .merge(automations::router())
        .merge(smart_links::router())
        .merge(templates::router())
        .merge(webhooks::router())
        .merge(companies::router())
        .merge(import_export::router())
        .merge(settings::router())
        .merge(manager_permissions::router())
        .merge(logs::router())
}
