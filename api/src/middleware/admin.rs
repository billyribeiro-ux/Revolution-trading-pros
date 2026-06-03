//! Admin authorization middleware
//! ICT 11+ Principal Engineer: Role-based access control

use axum::{
    extract::FromRequestParts,
    http::{request::Parts, StatusCode},
};

use crate::{models::User, AppState};

/// Admin user extractor - requires admin, super_admin, or developer role
pub struct AdminUser(pub User);

impl FromRequestParts<AppState> for AdminUser {
    type Rejection = (StatusCode, &'static str);

    #[allow(clippy::manual_async_fn)]
    fn from_request_parts(
        parts: &mut Parts,
        state: &AppState,
    ) -> impl std::future::Future<Output = Result<Self, Self::Rejection>> + Send {
        async move {
            // First, authenticate the user
            let user = User::from_request_parts(parts, state).await?;

            // Check if user has admin role (includes developer)
            let user_role = user.role.as_deref().unwrap_or("none");
            let is_admin = user_role == "admin"
                || user_role == "super_admin"
                || user_role == "super-admin"
                || user_role == "developer";

            // security-M1: email-list is only honored on a verified, DB-role'd
            // account — never as a standalone identity claim.
            let is_superadmin = state.config.is_superadmin_email_strict(
                &user.email,
                user.role.as_deref(),
                user.email_verified_at.is_some(),
            );

            // ICT 11+ DEBUG: Log role check for troubleshooting 403 issues
            tracing::info!(
                target: "auth_debug",
                user_id = user.id,
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
                    "Non-admin user attempted to access admin endpoint"
                );
                Err((StatusCode::FORBIDDEN, "Admin access required"))
            }
        }
    }
}

/// Super admin user extractor - requires super_admin role
pub struct SuperAdminUser(pub User);

impl FromRequestParts<AppState> for SuperAdminUser {
    type Rejection = (StatusCode, &'static str);

    #[allow(clippy::manual_async_fn)]
    fn from_request_parts(
        parts: &mut Parts,
        state: &AppState,
    ) -> impl std::future::Future<Output = Result<Self, Self::Rejection>> + Send {
        async move {
            // First, authenticate the user
            let user = User::from_request_parts(parts, state).await?;

            // Check if user has super_admin role
            let is_super_admin = user.role.as_deref() == Some("super_admin")
                || user.role.as_deref() == Some("super-admin");

            // security-M1: email-list is only honored on a verified, DB-role'd
            // account — never as a standalone identity claim.
            let is_superadmin_email = state.config.is_superadmin_email_strict(
                &user.email,
                user.role.as_deref(),
                user.email_verified_at.is_some(),
            );

            if is_super_admin || is_superadmin_email {
                Ok(SuperAdminUser(user))
            } else {
                tracing::warn!(
                    target: "security",
                    event = "unauthorized_superadmin_access",
                    user_id = %user.id,
                    "Non-superadmin user attempted to access superadmin endpoint"
                );
                Err((StatusCode::FORBIDDEN, "Super admin access required"))
            }
        }
    }
}
