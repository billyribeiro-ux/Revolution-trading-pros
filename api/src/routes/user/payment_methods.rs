//! Stripe payment-methods management — list, add, delete.
//!
//! R26-B split: handlers moved verbatim from `routes/user.rs`. The
//! Stripe customer-ID lookup → attach/detach flow, ownership
//! verification, default-PM guard, and `payments` audit target are
//! preserved.

use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use serde_json::json;

use crate::{models::User, AppState};

use super::dtos::{AddPaymentMethodRequest, PaymentMethodResponse};

/// Get user payment methods (Stripe)
/// GET /api/user/payment-methods
/// ICT 7 Fix: Complete Stripe payment methods listing implementation
pub(super) async fn get_payment_methods(
    State(state): State<AppState>,
    user: User,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    // Get user's Stripe customer ID from memberships
    let customer_id: Option<String> = sqlx::query_scalar(
        "SELECT stripe_customer_id FROM user_memberships WHERE user_id = $1 AND stripe_customer_id IS NOT NULL LIMIT 1"
    )
    .bind(user.id)
    .fetch_optional(&state.db.pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to get customer ID: {}", e);
        (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()})))
    })?
    .flatten();

    // If no customer ID, return empty array
    let customer_id = match customer_id {
        Some(id) => id,
        None => {
            return Ok(Json(json!({
                "success": true,
                "payment_methods": [],
                "paymentMethods": [] // Camel case for frontend
            })));
        }
    };

    // List payment methods from Stripe
    let stripe_methods = state
        .services
        .stripe
        .list_payment_methods(&customer_id)
        .await
        .map_err(|e| {
            tracing::error!("Failed to list payment methods: {}", e);
            (
                StatusCode::BAD_GATEWAY,
                Json(json!({"error": "Failed to retrieve payment methods"})),
            )
        })?;

    // Get default payment method for this customer
    let default_pm = state
        .services
        .stripe
        .get_customer_default_payment_method(&customer_id)
        .await
        .ok()
        .flatten();

    // Get subscription IDs for each payment method
    #[derive(sqlx::FromRow)]
    struct SubscriptionRow {
        #[allow(dead_code)]
        stripe_subscription_id: Option<String>,
        plan_name: Option<String>,
    }

    let subscriptions: Vec<SubscriptionRow> = sqlx::query_as(
        r"SELECT um.stripe_subscription_id, mp.name as plan_name
           FROM user_memberships um
           LEFT JOIN membership_plans mp ON um.plan_id = mp.id
           WHERE um.user_id = $1 AND um.status IN ('active', 'trialing', 'past_due')",
    )
    .bind(user.id)
    .fetch_all(&state.db.pool)
    .await
    .unwrap_or_default();

    // Build response
    let payment_methods: Vec<PaymentMethodResponse> = stripe_methods
        .into_iter()
        .map(|pm| {
            let is_default = default_pm.as_ref() == Some(&pm.id);

            // Find subscriptions using this payment method
            // Note: Stripe subscriptions don't directly link to payment methods easily,
            // so we show all subscriptions for the default payment method
            let subscription_names: Vec<String> = if is_default {
                subscriptions
                    .iter()
                    .filter_map(|s| s.plan_name.clone())
                    .collect()
            } else {
                Vec::new()
            };

            PaymentMethodResponse {
                id: pm.id,
                method_type: pm.method_type,
                brand: pm.card.as_ref().map(|c| c.brand.clone()),
                last4: pm.card.as_ref().map(|c| c.last4.clone()),
                expiry_month: pm.card.as_ref().map(|c| c.exp_month),
                expiry_year: pm.card.as_ref().map(|c| c.exp_year),
                is_default,
                subscriptions: subscription_names,
            }
        })
        .collect();

    tracing::info!(
        target: "payments",
        event = "payment_methods_listed",
        user_id = %user.id,
        count = %payment_methods.len(),
        "Listed user payment methods"
    );

    Ok(Json(json!({
        "success": true,
        "payment_methods": payment_methods,
        "paymentMethods": payment_methods // Camel case for frontend compatibility
    })))
}

/// Add payment method
/// POST /api/user/payment-methods
/// ICT 7 Fix: Complete Stripe payment method addition implementation
pub(super) async fn add_payment_method(
    State(state): State<AppState>,
    user: User,
    Json(input): Json<AddPaymentMethodRequest>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    // Get or create Stripe customer
    let customer_id = get_or_create_stripe_customer(&state, &user).await?;

    // Attach payment method to customer
    let payment_method = state
        .services
        .stripe
        .attach_payment_method(&input.payment_method_id, &customer_id)
        .await
        .map_err(|e| {
            tracing::error!("Failed to attach payment method: {}", e);
            (
                StatusCode::BAD_GATEWAY,
                Json(json!({"error": format!("Failed to add payment method: {}", e)})),
            )
        })?;

    // Set as default if requested
    if input.set_as_default {
        state
            .services
            .stripe
            .update_default_payment_method(&customer_id, &input.payment_method_id)
            .await
            .map_err(|e| {
                tracing::warn!("Failed to set default payment method: {}", e);
                // Don't fail the request, just log warning
            })
            .ok();
    }

    tracing::info!(
        target: "payments",
        event = "payment_method_added",
        user_id = %user.id,
        payment_method_id = %payment_method.id,
        set_as_default = %input.set_as_default,
        "Payment method added successfully"
    );

    Ok(Json(json!({
        "success": true,
        "message": "Payment method added successfully",
        "payment_method": {
            "id": payment_method.id,
            "type": payment_method.method_type,
            "brand": payment_method.card.as_ref().map(|c| &c.brand),
            "last4": payment_method.card.as_ref().map(|c| &c.last4),
            "expiryMonth": payment_method.card.as_ref().map(|c| c.exp_month),
            "expiryYear": payment_method.card.as_ref().map(|c| c.exp_year),
            "isDefault": input.set_as_default
        }
    })))
}

/// Delete payment method
/// DELETE /api/user/payment-methods/:id
/// ICT 7 Fix: Complete Stripe payment method deletion implementation
pub(super) async fn delete_payment_method(
    State(state): State<AppState>,
    user: User,
    Path(payment_method_id): Path<String>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    // Verify the payment method belongs to this user
    let customer_id: Option<String> = sqlx::query_scalar(
        "SELECT stripe_customer_id FROM user_memberships WHERE user_id = $1 AND stripe_customer_id IS NOT NULL LIMIT 1"
    )
    .bind(user.id)
    .fetch_optional(&state.db.pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()}))))?
    .flatten();

    let customer_id = customer_id.ok_or_else(|| {
        (
            StatusCode::NOT_FOUND,
            Json(json!({"error": "No payment methods on file"})),
        )
    })?;

    // Get the payment method to verify it belongs to this customer
    let payment_method = state
        .services
        .stripe
        .get_payment_method(&payment_method_id)
        .await
        .map_err(|e| {
            tracing::error!("Failed to get payment method: {}", e);
            (
                StatusCode::NOT_FOUND,
                Json(json!({"error": "Payment method not found"})),
            )
        })?;

    // Verify ownership
    if payment_method.customer.as_ref() != Some(&customer_id) {
        return Err((
            StatusCode::FORBIDDEN,
            Json(json!({"error": "Payment method does not belong to this account"})),
        ));
    }

    // Check if this is the default payment method
    let default_pm = state
        .services
        .stripe
        .get_customer_default_payment_method(&customer_id)
        .await
        .ok()
        .flatten();

    if default_pm.as_ref() == Some(&payment_method_id) {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(json!({
                "error": "Cannot delete default payment method",
                "message": "Please set another payment method as default first, or this payment method is linked to an active subscription."
            })),
        ));
    }

    // Detach the payment method
    state
        .services
        .stripe
        .detach_payment_method(&payment_method_id)
        .await
        .map_err(|e| {
            tracing::error!("Failed to detach payment method: {}", e);
            (
                StatusCode::BAD_GATEWAY,
                Json(json!({"error": format!("Failed to delete payment method: {}", e)})),
            )
        })?;

    tracing::info!(
        target: "payments",
        event = "payment_method_deleted",
        user_id = %user.id,
        payment_method_id = %payment_method_id,
        "Payment method deleted successfully"
    );

    Ok(Json(json!({
        "success": true,
        "message": "Payment method deleted successfully"
    })))
}

/// Helper to get or create Stripe customer for user
async fn get_or_create_stripe_customer(
    state: &AppState,
    user: &User,
) -> Result<String, (StatusCode, Json<serde_json::Value>)> {
    // First check if user already has a customer ID
    let existing_customer_id: Option<String> = sqlx::query_scalar(
        "SELECT stripe_customer_id FROM user_memberships WHERE user_id = $1 AND stripe_customer_id IS NOT NULL LIMIT 1"
    )
    .bind(user.id)
    .fetch_optional(&state.db.pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()}))))?
    .flatten();

    if let Some(customer_id) = existing_customer_id {
        return Ok(customer_id);
    }

    // Create a new Stripe customer
    let customer = state
        .services
        .stripe
        .get_or_create_customer(&user.email, Some(user.name.as_str()), None)
        .await
        .map_err(|e| {
            tracing::error!("Failed to create Stripe customer: {}", e);
            (
                StatusCode::BAD_GATEWAY,
                Json(json!({"error": "Failed to create payment profile"})),
            )
        })?;

    // Store the customer ID in a new user_memberships record (or user_payment_profiles table if exists)
    // For now, we'll store it when they actually subscribe, but return it here
    Ok(customer.id)
}
