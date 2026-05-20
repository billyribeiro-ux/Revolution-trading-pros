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

use axum::{extract::State, http::StatusCode, routing::get, Json, Router};
use serde::Deserialize;
use serde_json::json;

use crate::{middleware::admin::AdminUser, AppState};

pub mod automations;
pub mod companies;
pub mod contacts;
pub mod deals_pipelines;
pub mod import_export;
pub mod leads;
pub mod lists_tags;
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
    // Get total contacts count
    let total_contacts: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM contacts")
        .fetch_one(&state.db.pool)
        .await
        .unwrap_or((0,));

    // Get active contacts (not churned/unqualified)
    let active_contacts: (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM contacts WHERE status NOT IN ('churned', 'unqualified')",
    )
    .fetch_one(&state.db.pool)
    .await
    .unwrap_or((0,));

    // Get new contacts this month
    let new_this_month: (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM contacts WHERE created_at >= date_trunc('month', CURRENT_DATE)",
    )
    .fetch_one(&state.db.pool)
    .await
    .unwrap_or((0,));

    // Get leads stats (leads are contacts with status 'lead')
    let total_leads: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM contacts WHERE status = 'lead'")
        .fetch_one(&state.db.pool)
        .await
        .unwrap_or((0,));

    // Get campaigns stats
    let campaigns_sent: (i64,) =
        sqlx::query_as("SELECT COALESCE(SUM(emails_sent), 0) FROM crm_campaigns")
            .fetch_one(&state.db.pool)
            .await
            .unwrap_or((0,));

    let campaigns_opened: (i64,) =
        sqlx::query_as("SELECT COALESCE(SUM(opens), 0) FROM crm_campaigns")
            .fetch_one(&state.db.pool)
            .await
            .unwrap_or((0,));

    let campaigns_clicked: (i64,) =
        sqlx::query_as("SELECT COALESCE(SUM(clicks), 0) FROM crm_campaigns")
            .fetch_one(&state.db.pool)
            .await
            .unwrap_or((0,));

    Ok(Json(json!({
        "total_contacts": total_contacts.0,
        "active_contacts": active_contacts.0,
        "new_this_month": new_this_month.0,
        "total_leads": total_leads.0,
        "total_deals": 0,
        "deals_value": 0.0,
        "open_deals": 0,
        "won_deals": 0,
        "lost_deals": 0,
        "email_sent": campaigns_sent.0,
        "email_opened": campaigns_opened.0,
        "email_clicked": campaigns_clicked.0
    })))
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
        .merge(contacts::router())
        .merge(lists_tags::router())
        .merge(segments::router())
        .merge(sequences_campaigns::router())
        .merge(automations::router())
        .merge(smart_links::router())
        .merge(templates::router())
        .merge(webhooks::router())
        .merge(companies::router())
        .merge(import_export::router())
        .merge(settings::router())
}
