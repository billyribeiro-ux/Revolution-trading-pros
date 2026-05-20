//! Type definitions for CMS Datasources.
//!
//! Database models, request/response DTOs, query structs, and a couple of
//! crate-local result aliases. Kept free of HTTP/handler logic so this
//! module can be re-exported as the public API surface.

use axum::Json;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use sqlx::FromRow;
use utoipa::{IntoParams, ToSchema};
use uuid::Uuid;

use crate::utils::errors::ApiError;

// ============================================================================
// TYPE ALIASES
// ============================================================================

pub(super) type ApiResult<T> = Result<Json<T>, ApiError>;
pub(super) type ApiResultEmpty = Result<Json<JsonValue>, ApiError>;

// ============================================================================
// DATABASE MODELS
// ============================================================================

/// Datasource container for reusable option lists
#[derive(Debug, Clone, Serialize, Deserialize, FromRow, ToSchema)]
pub struct CmsDatasource {
    pub id: Uuid,
    pub name: String,
    pub slug: String,
    pub description: Option<String>,
    pub icon: Option<String>,
    pub color: Option<String>,
    pub entry_count: i32,
    pub is_system: bool,
    pub is_locked: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub created_by: Option<Uuid>,
    pub updated_by: Option<Uuid>,
    pub deleted_at: Option<DateTime<Utc>>,
}

/// Lightweight datasource summary for listings
#[derive(Debug, Clone, Serialize, Deserialize, FromRow, ToSchema)]
pub struct CmsDatasourceSummary {
    pub id: Uuid,
    pub name: String,
    pub slug: String,
    pub description: Option<String>,
    pub icon: Option<String>,
    pub color: Option<String>,
    pub entry_count: i32,
    pub is_system: bool,
    pub is_locked: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Datasource entry (key-value pair)
#[derive(Debug, Clone, Serialize, Deserialize, FromRow, ToSchema)]
pub struct CmsDatasourceEntry {
    pub id: Uuid,
    pub datasource_id: Uuid,
    pub name: String,
    pub value: String,
    pub dimension: String,
    pub sort_order: i32,
    pub metadata: Option<JsonValue>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Lightweight entry for listings
#[derive(Debug, Clone, Serialize, Deserialize, FromRow, ToSchema)]
pub struct CmsDatasourceEntrySummary {
    pub id: Uuid,
    pub name: String,
    pub value: String,
    pub dimension: String,
    pub sort_order: i32,
    pub metadata: Option<JsonValue>,
}

// ============================================================================
// REQUEST/RESPONSE MODELS
// ============================================================================

/// Request to create a new datasource
#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateDatasourceRequest {
    pub name: String,
    pub slug: Option<String>,
    pub description: Option<String>,
    pub icon: Option<String>,
    pub color: Option<String>,
}

/// Request to update a datasource
#[derive(Debug, Deserialize, ToSchema)]
pub struct UpdateDatasourceRequest {
    pub name: Option<String>,
    pub slug: Option<String>,
    pub description: Option<String>,
    pub icon: Option<String>,
    pub color: Option<String>,
    pub is_locked: Option<bool>,
}

/// Request to create a new entry
#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateEntryRequest {
    pub name: String,
    pub value: String,
    pub dimension: Option<String>,
    pub metadata: Option<JsonValue>,
}

/// Request to update an entry
#[derive(Debug, Deserialize, ToSchema)]
pub struct UpdateEntryRequest {
    pub name: Option<String>,
    pub value: Option<String>,
    pub dimension: Option<String>,
    pub sort_order: Option<i32>,
    pub metadata: Option<JsonValue>,
}

/// Request to bulk create entries
#[derive(Debug, Deserialize, ToSchema)]
pub struct BulkCreateEntriesRequest {
    pub entries: Vec<CreateEntryRequest>,
}

/// Request to reorder entries
#[derive(Debug, Deserialize, ToSchema)]
pub struct ReorderEntriesRequest {
    pub entry_ids: Vec<Uuid>,
}

/// Query parameters for listing datasources
#[derive(Debug, Deserialize, IntoParams, ToSchema)]
pub struct ListDatasourcesQuery {
    pub search: Option<String>,
    pub is_system: Option<bool>,
    pub sort_by: Option<String>,
    pub sort_order: Option<String>,
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

/// Query parameters for listing entries
#[derive(Debug, Deserialize, IntoParams, ToSchema)]
pub struct ListEntriesQuery {
    pub dimension: Option<String>,
    pub search: Option<String>,
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

/// Paginated response wrapper for datasources
#[derive(Debug, Serialize, ToSchema)]
pub struct PaginatedDatasourcesResponse {
    pub data: Vec<CmsDatasourceSummary>,
    pub meta: PaginationMeta,
}

/// Paginated response wrapper for entries
#[derive(Debug, Serialize, ToSchema)]
pub struct PaginatedEntriesResponse {
    pub data: Vec<CmsDatasourceEntrySummary>,
    pub meta: PaginationMeta,
    pub dimensions: Vec<String>,
}

/// Pagination metadata
#[derive(Debug, Serialize, ToSchema)]
pub struct PaginationMeta {
    pub total: i64,
    pub limit: i64,
    pub offset: i64,
    pub has_more: bool,
}

/// CSV export row
#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct CsvEntryRow {
    pub name: String,
    pub value: String,
    pub dimension: String,
}

/// Response for public datasource access
#[derive(Debug, Serialize, ToSchema)]
pub struct PublicDatasourceResponse {
    pub slug: String,
    pub name: String,
    pub entries: Vec<PublicEntryResponse>,
}

/// Public entry response (minimal)
#[derive(Debug, Serialize, FromRow, ToSchema)]
pub struct PublicEntryResponse {
    pub name: String,
    pub value: String,
}
