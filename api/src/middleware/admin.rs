//! Admin authorization middleware
//! ICT 11+ Principal Engineer: Role-based access control

use axum::{
    extract::FromRequestParts,
    http::{request::Parts, StatusCode},
};

use crate::{models::User, AppState};

/// Admin user extractor - requires admin, super_admin, or developer role
pub struct AdminUser(pub User);

#[axum::async_trait]
impl FromRequestParts<AppState> for AdminUser {
    type Rejection = (StatusCode, &'static str);

    async fn from_request_parts(
        parts: &mut Parts,
        state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        // First, authenticate the user
        let user = User::from_request_parts(parts, state).await?;

        // Check if user has admin role (includes developer)
        let user_role = user.role.as_deref().unwrap_or("none");
        let is_admin = user_role == "admin"
            || user_role == "super_admin"
            || user_role == "super-admin"
            || user_role == "developer";

        // Also check if email is in superadmin list
        let is_superadmin = state.config.is_superadmin_email(&user.email);

        // ICT 11+ DEBUG: Log role check for troubleshooting 403 issues
        tracing::info!(
            target: "auth_debug",
            user_id = user.id,
            email = %user.email,
            role = %user_role,
            is_admin = is_admin,
            is_superadmin_email = is_superadmin,
            "Admin auth check"
        );

        if is_admin || is_superadmin {
            Ok(AdminUser(user))
        } else {
            tracing::warn!(
                target: "security",
                event = "unauthorized_admin_access",
                user_id = %user.id,
                email = %user.email,
                "Non-admin user attempted to access admin endpoint"
            );
            Err((StatusCode::FORBIDDEN, "Admin access required"))
        }
    }
}

/// Super admin user extractor - requires super_admin role
pub struct SuperAdminUser(pub User);

#[axum::async_trait]
impl FromRequestParts<AppState> for SuperAdminUser {
    type Rejection = (StatusCode, &'static str);

    async fn from_request_parts(
        parts: &mut Parts,
        state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        // First, authenticate the user
        let user = User::from_request_parts(parts, state).await?;

        // Check if user has super_admin role
        let is_super_admin = user.role.as_deref() == Some("super_admin")
            || user.role.as_deref() == Some("super-admin");

        // Also check if email is in superadmin list
        let is_superadmin_email = state.config.is_superadmin_email(&user.email);

        if is_super_admin || is_superadmin_email {
            Ok(SuperAdminUser(user))
        } else {
            tracing::warn!(
                target: "security",
                event = "unauthorized_superadmin_access",
                user_id = %user.id,
                email = %user.email,
                "Non-superadmin user attempted to access superadmin endpoint"
            );
            Err((StatusCode::FORBIDDEN, "Super admin access required"))
        }
    }
}
