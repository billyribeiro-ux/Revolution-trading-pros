//! Webhook routes for external services

use worker::{Request, Response, RouteContext};
use crate::AppState;
use crate::error::ApiError;

/// POST /api/webhooks/stripe - Handle Stripe webhooks
pub async fn stripe(mut req: Request, ctx: RouteContext<AppState>) -> worker::Result<Response> {
    // Get signature header
    let signature = req.headers().get("Stripe-Signature")
        .map_err(|_| ApiError::BadRequest("Missing Stripe signature".to_string()))?
        .ok_or_else(|| ApiError::BadRequest("Missing Stripe signature".to_string()))?;

    // Get raw body
    let body = req.text().await
        .map_err(|e| ApiError::BadRequest(format!("Failed to read body: {}", e)))?;

    // Verify webhook signature
    let stripe = &ctx.data.services.stripe;
    let event = stripe.verify_webhook(&body, &signature)?;

    let now = crate::utils::now();

    // Handle different event types
    match event.event_type.as_str() {
        "checkout.session.completed" => {
            // Payment successful - fulfill order
            if let Some(session_id) = event.data.object.get("id").and_then(|v| v.as_str()) {
                handle_checkout_completed(&ctx, session_id).await?;
            }
        }
        
        "invoice.paid" => {
            // Subscription payment successful
            if let Some(subscription_id) = event.data.object.get("subscription").and_then(|v| v.as_str()) {
                ctx.data.db.execute(
                    r#"
                    UPDATE user_subscriptions 
                    SET status = 'active', updated_at = $1
                    WHERE stripe_subscription_id = $2
                    "#,
                    vec![serde_json::json!(now.to_rfc3339()), serde_json::json!(subscription_id)]
                ).await?;
            }
        }
        
        "invoice.payment_failed" => {
            // Subscription payment failed
            if let Some(subscription_id) = event.data.object.get("subscription").and_then(|v| v.as_str()) {
                ctx.data.db.execute(
                    r#"
                    UPDATE user_subscriptions 
                    SET status = 'past_due', updated_at = $1
                    WHERE stripe_subscription_id = $2
                    "#,
                    vec![serde_json::json!(now.to_rfc3339()), serde_json::json!(subscription_id)]
                ).await?;
            }
        }
        
        "customer.subscription.updated" => {
            // Subscription updated (plan change, etc.)
            if let Some(subscription_id) = event.data.object.get("id").and_then(|v| v.as_str()) {
                let status = event.data.object.get("status").and_then(|v| v.as_str()).unwrap_or("active");
                let cancel_at_period_end = event.data.object.get("cancel_at_period_end").and_then(|v| v.as_bool()).unwrap_or(false);
                
                ctx.data.db.execute(
                    r#"
                    UPDATE user_subscriptions 
                    SET status = $1, cancel_at_period_end = $2, updated_at = $3
                    WHERE stripe_subscription_id = $4
                    "#,
                    vec![
                        serde_json::json!(status),
                        serde_json::json!(cancel_at_period_end),
                        serde_json::json!(now.to_rfc3339()),
                        serde_json::json!(subscription_id),
                    ]
                ).await?;
            }
        }
        
        "customer.subscription.deleted" => {
            // Subscription cancelled/ended
            if let Some(subscription_id) = event.data.object.get("id").and_then(|v| v.as_str()) {
                ctx.data.db.execute(
                    r#"
                    UPDATE user_subscriptions 
                    SET status = 'cancelled', ended_at = $1, updated_at = $1
                    WHERE stripe_subscription_id = $2
                    "#,
                    vec![serde_json::json!(now.to_rfc3339()), serde_json::json!(subscription_id)]
                ).await?;

                // Revoke product access
                let sub: Option<SubscriptionInfo> = ctx.data.db.query_one(
                    "SELECT user_id, product_id FROM user_subscriptions WHERE stripe_subscription_id = $1",
                    vec![serde_json::json!(subscription_id)]
                ).await?;

                if let Some(s) = sub {
                    ctx.data.db.execute(
                        "UPDATE product_user SET status = 'expired', updated_at = $1 WHERE user_id = $2 AND product_id = $3",
                        vec![
                            serde_json::json!(now.to_rfc3339()),
                            serde_json::json!(s.user_id.to_string()),
                            serde_json::json!(s.product_id.to_string()),
                        ]
                    ).await?;
                }
            }
        }
        
        _ => {
            // Log unhandled event types
            worker::console_log!("Unhandled Stripe event: {}", event.event_type);
        }
    }

    // Log webhook event
    ctx.data.db.execute(
        r#"
        INSERT INTO webhook_logs (id, provider, event_type, payload, created_at)
        VALUES ($1, 'stripe', $2, $3, $4)
        "#,
        vec![
            serde_json::json!(uuid::Uuid::new_v4().to_string()),
            serde_json::json!(event.event_type),
            serde_json::json!(body),
            serde_json::json!(now.to_rfc3339()),
        ]
    ).await?;

    Response::from_json(&serde_json::json!({"received": true}))
}

/// POST /api/webhooks/postmark/inbound - Handle Postmark inbound emails
pub async fn postmark_inbound(mut req: Request, ctx: RouteContext<AppState>) -> worker::Result<Response> {
    #[derive(serde::Deserialize)]
    struct InboundEmail {
        #[serde(rename = "From")]
        from: String,
        #[serde(rename = "FromName")]
        from_name: Option<String>,
        #[serde(rename = "To")]
        to: String,
        #[serde(rename = "Subject")]
        subject: String,
        #[serde(rename = "TextBody")]
        text_body: Option<String>,
        #[serde(rename = "HtmlBody")]
        html_body: Option<String>,
        #[serde(rename = "MessageID")]
        message_id: Option<String>,
        #[serde(rename = "Attachments")]
        attachments: Option<Vec<serde_json::Value>>,
    }

    let email: InboundEmail = req.json().await
        .map_err(|e| ApiError::BadRequest(format!("Invalid inbound email: {}", e)))?;

    let now = crate::utils::now();
    let conversation_id = uuid::Uuid::new_v4();
    let message_id = uuid::Uuid::new_v4();

    // Check if this is a reply to an existing conversation
    let existing_conversation: Option<ConversationInfo> = if let Some(ref msg_id) = email.message_id {
        ctx.data.db.query_one(
            r#"
            SELECT c.id, c.user_id FROM email_conversations c
            JOIN email_messages m ON m.conversation_id = c.id
            WHERE m.message_id = $1
            "#,
            vec![serde_json::json!(msg_id)]
        ).await?
    } else {
        None
    };

    let conv_id = if let Some(existing) = existing_conversation {
        // Add to existing conversation
        existing.id
    } else {
        // Create new conversation
        ctx.data.db.execute(
            r#"
            INSERT INTO email_conversations (
                id, subject, from_email, from_name, status, priority, 
                message_count, last_message_at, created_at, updated_at
            ) VALUES ($1, $2, $3, $4, 'open', 'normal', 1, $5, $5, $5)
            "#,
            vec![
                serde_json::json!(conversation_id.to_string()),
                serde_json::json!(email.subject),
                serde_json::json!(email.from),
                serde_json::json!(email.from_name),
                serde_json::json!(now.to_rfc3339()),
            ]
        ).await?;
        conversation_id
    };

    // Create message
    ctx.data.db.execute(
        r#"
        INSERT INTO email_messages (
            id, conversation_id, direction, from_email, from_name, to_email,
            subject, html_body, text_body, message_id, created_at
        ) VALUES ($1, $2, 'inbound', $3, $4, $5, $6, $7, $8, $9, $10)
        "#,
        vec![
            serde_json::json!(message_id.to_string()),
            serde_json::json!(conv_id.to_string()),
            serde_json::json!(email.from),
            serde_json::json!(email.from_name),
            serde_json::json!(email.to),
            serde_json::json!(email.subject),
            serde_json::json!(email.html_body),
            serde_json::json!(email.text_body),
            serde_json::json!(email.message_id),
            serde_json::json!(now.to_rfc3339()),
        ]
    ).await?;

    // Update conversation message count
    ctx.data.db.execute(
        r#"
        UPDATE email_conversations 
        SET message_count = message_count + 1, last_message_at = $1, updated_at = $1
        WHERE id = $2
        "#,
        vec![serde_json::json!(now.to_rfc3339()), serde_json::json!(conv_id.to_string())]
    ).await?;

    Response::from_json(&serde_json::json!({"received": true}))
}

/// Handle successful checkout
async fn handle_checkout_completed(ctx: &RouteContext<AppState>, session_id: &str) -> Result<(), ApiError> {
    let now = crate::utils::now();

    // Find order by payment intent
    let order: Option<OrderInfo> = ctx.data.db.query_one(
        "SELECT id, user_id FROM orders WHERE payment_intent_id LIKE $1",
        vec![serde_json::json!(format!("%{}%", session_id))]
    ).await?;

    if let Some(order) = order {
        // Update order status
        ctx.data.db.execute(
            "UPDATE orders SET status = 'completed', completed_at = $1, updated_at = $1 WHERE id = $2",
            vec![serde_json::json!(now.to_rfc3339()), serde_json::json!(order.id.to_string())]
        ).await?;

        // Get order items and grant product access
        let items: Vec<OrderItemInfo> = ctx.data.db.query(
            "SELECT product_id FROM order_items WHERE order_id = $1",
            vec![serde_json::json!(order.id.to_string())]
        ).await?;

        for item in items {
            ctx.data.db.execute(
                r#"
                INSERT INTO product_user (id, user_id, product_id, status, starts_at, created_at)
                VALUES ($1, $2, $3, 'active', $4, $4)
                ON CONFLICT (user_id, product_id) DO UPDATE SET status = 'active', updated_at = $4
                "#,
                vec![
                    serde_json::json!(uuid::Uuid::new_v4().to_string()),
                    serde_json::json!(order.user_id.to_string()),
                    serde_json::json!(item.product_id.to_string()),
                    serde_json::json!(now.to_rfc3339()),
                ]
            ).await?;
        }
    }

    Ok(())
}

#[derive(serde::Deserialize)]
struct SubscriptionInfo {
    user_id: uuid::Uuid,
    product_id: uuid::Uuid,
}

#[derive(serde::Deserialize)]
struct ConversationInfo {
    id: uuid::Uuid,
    user_id: Option<uuid::Uuid>,
}

#[derive(serde::Deserialize)]
struct OrderInfo {
    id: uuid::Uuid,
    user_id: uuid::Uuid,
}

#[derive(serde::Deserialize)]
struct OrderItemInfo {
    product_id: uuid::Uuid,
}
