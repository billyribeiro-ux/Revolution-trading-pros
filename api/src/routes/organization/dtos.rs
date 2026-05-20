//! Organization route DTOs — Revolution Trading Pros
//!
//! R27-B3 (2026-05-20): the original 1,233-LOC `organization.rs` was
//! split into this `organization/` directory as a PURE STRUCTURAL MOVE.
//! Every type below is re-exported from `routes::organization` so
//! `tests/organization_test.rs` and any downstream consumers continue
//! to resolve them at the same path. `id` / `parent_id` remain `i64`
//! (BIGSERIAL contract) and `member_count` stays `Option<i32>` (the
//! legitimate row-count exception per CLAUDE.md "Money / cents" rule).
//!
//! Public types (re-exported by `organization/mod.rs`):
//!   - `OrganizationProfile`, `UpdateOrganizationRequest`
//!   - `Team`, `CreateTeamRequest`, `UpdateTeamRequest`
//!   - `Department`, `CreateDepartmentRequest`, `UpdateDepartmentRequest`
//!   - `ListQuery`

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

// ═══════════════════════════════════════════════════════════════════════════
// TYPES - ORGANIZATION PROFILE
// ═══════════════════════════════════════════════════════════════════════════

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct OrganizationProfile {
    pub id: i64,
    pub name: String,
    pub slug: String,
    pub description: Option<String>,
    pub logo_url: Option<String>,
    pub favicon_url: Option<String>,
    pub primary_color: Option<String>,
    pub secondary_color: Option<String>,
    pub contact_email: Option<String>,
    pub contact_phone: Option<String>,
    pub address: Option<String>,
    pub city: Option<String>,
    pub state: Option<String>,
    pub country: Option<String>,
    pub postal_code: Option<String>,
    pub website_url: Option<String>,
    pub social_links: Option<serde_json::Value>,
    pub business_hours: Option<serde_json::Value>,
    pub timezone: Option<String>,
    pub currency: Option<String>,
    pub tax_id: Option<String>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateOrganizationRequest {
    pub name: Option<String>,
    pub description: Option<String>,
    pub logo_url: Option<String>,
    pub favicon_url: Option<String>,
    pub primary_color: Option<String>,
    pub secondary_color: Option<String>,
    pub contact_email: Option<String>,
    pub contact_phone: Option<String>,
    pub address: Option<String>,
    pub city: Option<String>,
    pub state: Option<String>,
    pub country: Option<String>,
    pub postal_code: Option<String>,
    pub website_url: Option<String>,
    pub social_links: Option<serde_json::Value>,
    pub business_hours: Option<serde_json::Value>,
    pub timezone: Option<String>,
    pub currency: Option<String>,
    pub tax_id: Option<String>,
}

// ═══════════════════════════════════════════════════════════════════════════
// TYPES - TEAMS
// ═══════════════════════════════════════════════════════════════════════════

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Team {
    pub id: i64,
    pub name: String,
    pub slug: String,
    pub description: Option<String>,
    pub color: Option<String>,
    pub icon: Option<String>,
    pub is_active: bool,
    pub member_count: Option<i32>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Deserialize)]
pub struct CreateTeamRequest {
    pub name: String,
    pub description: Option<String>,
    pub color: Option<String>,
    pub icon: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateTeamRequest {
    pub name: Option<String>,
    pub description: Option<String>,
    pub color: Option<String>,
    pub icon: Option<String>,
    pub is_active: Option<bool>,
}

// ═══════════════════════════════════════════════════════════════════════════
// TYPES - DEPARTMENTS
// ═══════════════════════════════════════════════════════════════════════════

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Department {
    pub id: i64,
    pub name: String,
    pub slug: String,
    pub description: Option<String>,
    pub color: Option<String>,
    pub icon: Option<String>,
    pub parent_id: Option<i64>,
    pub is_active: bool,
    pub member_count: Option<i32>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Deserialize)]
pub struct CreateDepartmentRequest {
    pub name: String,
    pub description: Option<String>,
    pub color: Option<String>,
    pub icon: Option<String>,
    pub parent_id: Option<i64>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateDepartmentRequest {
    pub name: Option<String>,
    pub description: Option<String>,
    pub color: Option<String>,
    pub icon: Option<String>,
    pub parent_id: Option<i64>,
    pub is_active: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct ListQuery {
    pub search: Option<String>,
    pub active_only: Option<bool>,
}
