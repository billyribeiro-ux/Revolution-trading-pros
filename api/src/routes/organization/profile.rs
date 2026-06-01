//! Organization profile handlers — GET/PUT /admin/organization/profile.
//!
//! R27-B3 (2026-05-20): extracted verbatim from the original
//! `organization.rs`. SQL (the COALESCE-driven UPSERT into the
//! singleton row `id = 1`), audit-log call sites, tracing fields,
//! status codes, and JSON error envelopes are preserved byte-for-byte.

use axum::{extract::State, http::StatusCode, Json};
use serde_json::json;

use super::dtos::{OrganizationProfile, UpdateOrganizationRequest};
use super::helpers::{generate_slug, log_organization_audit};
use crate::{middleware::admin::AdminUser, AppState};

/// GET /admin/organization/profile - Get organization profile
#[tracing::instrument(skip(state, admin))]
pub(super) async fn get_organization_profile(
    State(state): State<AppState>,
    admin: AdminUser,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    tracing::info!(
        admin_id = admin.0.id,
        admin_email = %admin.0.email,
        "Admin fetching organization profile"
    );

    let profile: Option<OrganizationProfile> = sqlx::query_as(
        r"
        SELECT id, name, slug, description, logo_url, favicon_url,
               primary_color, secondary_color, contact_email, contact_phone,
               address, city, state, country, postal_code, website_url,
               social_links, business_hours, timezone, currency, tax_id,
               created_at, updated_at
        FROM organization_profile
        LIMIT 1
        ",
    )
    .fetch_optional(state.db.pool())
    .await
    .map_err(|e| {
        tracing::error!("Database error: {}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": "Database error", "details": e.to_string()})),
        )
    })?;

    match profile {
        Some(p) => Ok(Json(json!({
            "success": true,
            "data": p
        }))),
        None => {
            // Return default profile if none exists
            Ok(Json(json!({
                "success": true,
                "data": {
                    "id": 0,
                    "name": "Revolution Trading Pros",
                    "slug": "revolution-trading-pros",
                    "description": null,
                    "timezone": "America/New_York",
                    "currency": "USD"
                }
            })))
        }
    }
}

/// PUT /admin/organization/profile - Update organization profile
#[tracing::instrument(skip(state, admin, input))]
pub(super) async fn update_organization_profile(
    State(state): State<AppState>,
    admin: AdminUser,
    Json(input): Json<UpdateOrganizationRequest>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    tracing::info!(
        admin_id = admin.0.id,
        admin_email = %admin.0.email,
        "Admin updating organization profile"
    );

    // Get old profile for audit
    let old_profile: Option<OrganizationProfile> =
        sqlx::query_as("SELECT * FROM organization_profile LIMIT 1")
            .fetch_optional(state.db.pool())
            .await
            .ok()
            .flatten();

    let old_value = old_profile.as_ref().map(|p| {
        json!({
            "name": p.name,
            "contact_email": p.contact_email
        })
    });

    // Generate slug if name is provided
    let slug = input.name.as_ref().map(|n| generate_slug(n));

    let profile: OrganizationProfile = sqlx::query_as(
        r"
        INSERT INTO organization_profile (
            id, name, slug, description, logo_url, favicon_url,
            primary_color, secondary_color, contact_email, contact_phone,
            address, city, state, country, postal_code, website_url,
            social_links, business_hours, timezone, currency, tax_id,
            created_at, updated_at
        ) VALUES (
            1,
            COALESCE($1, 'Revolution Trading Pros'),
            COALESCE($2, 'revolution-trading-pros'),
            $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18, $19, $20,
            COALESCE((SELECT created_at FROM organization_profile WHERE id = 1), NOW()),
            NOW()
        )
        ON CONFLICT (id) DO UPDATE SET
            name = COALESCE($1, organization_profile.name),
            slug = COALESCE($2, organization_profile.slug),
            description = COALESCE($3, organization_profile.description),
            logo_url = COALESCE($4, organization_profile.logo_url),
            favicon_url = COALESCE($5, organization_profile.favicon_url),
            primary_color = COALESCE($6, organization_profile.primary_color),
            secondary_color = COALESCE($7, organization_profile.secondary_color),
            contact_email = COALESCE($8, organization_profile.contact_email),
            contact_phone = COALESCE($9, organization_profile.contact_phone),
            address = COALESCE($10, organization_profile.address),
            city = COALESCE($11, organization_profile.city),
            state = COALESCE($12, organization_profile.state),
            country = COALESCE($13, organization_profile.country),
            postal_code = COALESCE($14, organization_profile.postal_code),
            website_url = COALESCE($15, organization_profile.website_url),
            social_links = COALESCE($16, organization_profile.social_links),
            business_hours = COALESCE($17, organization_profile.business_hours),
            timezone = COALESCE($18, organization_profile.timezone),
            currency = COALESCE($19, organization_profile.currency),
            tax_id = COALESCE($20, organization_profile.tax_id),
            updated_at = NOW()
        RETURNING id, name, slug, description, logo_url, favicon_url,
                  primary_color, secondary_color, contact_email, contact_phone,
                  address, city, state, country, postal_code, website_url,
                  social_links, business_hours, timezone, currency, tax_id,
                  created_at, updated_at
        ",
    )
    .bind(&input.name)
    .bind(&slug)
    .bind(&input.description)
    .bind(&input.logo_url)
    .bind(&input.favicon_url)
    .bind(&input.primary_color)
    .bind(&input.secondary_color)
    .bind(&input.contact_email)
    .bind(&input.contact_phone)
    .bind(&input.address)
    .bind(&input.city)
    .bind(&input.state)
    .bind(&input.country)
    .bind(&input.postal_code)
    .bind(&input.website_url)
    .bind(&input.social_links)
    .bind(&input.business_hours)
    .bind(&input.timezone)
    .bind(&input.currency)
    .bind(&input.tax_id)
    .fetch_one(state.db.pool())
    .await
    .map_err(|e| {
        tracing::error!("Failed to update organization profile: {}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": "Failed to update profile", "details": e.to_string()})),
        )
    })?;

    // Audit log
    log_organization_audit(
        state.db.pool(),
        admin.0.id,
        &admin.0.email,
        "organization.profile.updated",
        "organization_profile",
        Some(profile.id),
        old_value,
        Some(json!({"name": profile.name, "contact_email": profile.contact_email})),
        None,
    )
    .await;

    tracing::info!(admin_id = admin.0.id, "Organization profile updated");

    Ok(Json(json!({
        "success": true,
        "message": "Organization profile updated",
        "data": profile
    })))
}
