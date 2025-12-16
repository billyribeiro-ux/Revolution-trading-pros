//! Newsletter routes

use worker::{Request, Response, RouteContext};
use crate::AppState;
use crate::error::ApiError;
use crate::models::email::{NewsletterSubscribeRequest, SubscriberStatus};
use crate::services::PasswordService;

/// POST /api/newsletter/subscribe - Subscribe to newsletter
pub async fn subscribe(mut req: Request, ctx: RouteContext<AppState>) -> worker::Result<Response> {
    let body: NewsletterSubscribeRequest = req.json().await
        .map_err(|e| ApiError::BadRequest(format!("Invalid request body: {}", e)))?;

    let email = body.email.to_lowercase();
    let now = chrono::Utc::now();

    // Check if already subscribed
    let existing: Option<crate::models::email::EmailSubscriber> = ctx.data.db.query_one(
        "SELECT * FROM email_subscribers WHERE email = $1",
        vec![serde_json::json!(&email)]
    ).await?;

    if let Some(subscriber) = existing {
        match subscriber.status {
            SubscriberStatus::Subscribed => {
                return Response::from_json(&serde_json::json!({
                    "message": "You are already subscribed to our newsletter"
                }));
            }
            SubscriberStatus::Unsubscribed | SubscriberStatus::Bounced => {
                // Re-subscribe
                ctx.data.db.execute(
                    r#"
                    UPDATE email_subscribers 
                    SET status = 'pending', unsubscribed_at = NULL, bounced_at = NULL, updated_at = $1
                    WHERE email = $2
                    "#,
                    vec![serde_json::json!(now.to_rfc3339()), serde_json::json!(&email)]
                ).await?;
            }
            _ => {}
        }
    } else {
        // Create new subscriber
        let subscriber_id = uuid::Uuid::new_v4();
        
        ctx.data.db.execute(
            r#"
            INSERT INTO email_subscribers (id, email, name, status, source, subscribed_at, created_at, updated_at)
            VALUES ($1, $2, $3, 'pending', $4, $5, $5, $5)
            "#,
            vec![
                serde_json::json!(subscriber_id.to_string()),
                serde_json::json!(&email),
                serde_json::json!(body.name),
                serde_json::json!(body.source.unwrap_or_else(|| "website".to_string())),
                serde_json::json!(now.to_rfc3339()),
            ]
        ).await?;
    }

    // Generate confirmation token
    let token = PasswordService::generate_token();
    let token_hash = PasswordService::hash_token(&token);
    let expires_at = now + chrono::Duration::hours(24);

    // Store confirmation token
    ctx.data.db.execute(
        r#"
        INSERT INTO email_confirmations (id, email, token_hash, expires_at, created_at)
        VALUES ($1, $2, $3, $4, $5)
        ON CONFLICT (email) DO UPDATE SET token_hash = $3, expires_at = $4
        "#,
        vec![
            serde_json::json!(uuid::Uuid::new_v4().to_string()),
            serde_json::json!(&email),
            serde_json::json!(token_hash),
            serde_json::json!(expires_at.to_rfc3339()),
            serde_json::json!(now.to_rfc3339()),
        ]
    ).await?;

    // Send confirmation email
    let postmark = &ctx.data.services.postmark;
    let confirm_url = format!(
        "{}/api/newsletter/confirm?token={}&email={}",
        ctx.data.config.site_url,
        token,
        urlencoding::encode(&email)
    );

    let _ = postmark.send_template_email(
        &email,
        "newsletter-confirm",
        serde_json::json!({
            "confirm_url": confirm_url,
            "name": body.name.unwrap_or_else(|| "Subscriber".to_string())
        }),
    ).await;

    Response::from_json(&serde_json::json!({
        "message": "Please check your email to confirm your subscription"
    }))
}

/// GET /api/newsletter/confirm - Confirm newsletter subscription
pub async fn confirm(req: Request, ctx: RouteContext<AppState>) -> worker::Result<Response> {
    let url = req.url()?;
    let params: std::collections::HashMap<String, String> = url.query_pairs()
        .map(|(k, v)| (k.to_string(), v.to_string()))
        .collect();

    let token = params.get("token")
        .ok_or_else(|| ApiError::BadRequest("Missing token".to_string()))?;
    let email = params.get("email")
        .ok_or_else(|| ApiError::BadRequest("Missing email".to_string()))?;

    let token_hash = PasswordService::hash_token(token);
    let now = chrono::Utc::now();

    // Verify token
    let confirmation: Option<EmailConfirmation> = ctx.data.db.query_one(
        r#"
        SELECT * FROM email_confirmations 
        WHERE email = $1 AND token_hash = $2 AND expires_at > $3
        "#,
        vec![
            serde_json::json!(email.to_lowercase()),
            serde_json::json!(token_hash),
            serde_json::json!(now.to_rfc3339()),
        ]
    ).await?;

    if confirmation.is_none() {
        return Err(ApiError::BadRequest("Invalid or expired confirmation link".to_string()).into());
    }

    // Confirm subscription
    ctx.data.db.execute(
        r#"
        UPDATE email_subscribers 
        SET status = 'subscribed', confirmed_at = $1, updated_at = $1
        WHERE email = $2
        "#,
        vec![
            serde_json::json!(now.to_rfc3339()),
            serde_json::json!(email.to_lowercase()),
        ]
    ).await?;

    // Delete confirmation token
    ctx.data.db.execute(
        "DELETE FROM email_confirmations WHERE email = $1",
        vec![serde_json::json!(email.to_lowercase())]
    ).await?;

    // Redirect to success page
    Response::redirect_with_status(
        worker::Url::parse(&format!("{}/newsletter/confirmed", ctx.data.config.site_url))?,
        302
    )
}

/// GET /api/newsletter/unsubscribe - Unsubscribe from newsletter
pub async fn unsubscribe(req: Request, ctx: RouteContext<AppState>) -> worker::Result<Response> {
    let url = req.url()?;
    let params: std::collections::HashMap<String, String> = url.query_pairs()
        .map(|(k, v)| (k.to_string(), v.to_string()))
        .collect();

    let token = params.get("token")
        .ok_or_else(|| ApiError::BadRequest("Missing token".to_string()))?;
    let email = params.get("email")
        .ok_or_else(|| ApiError::BadRequest("Missing email".to_string()))?;

    // Verify unsubscribe token (simplified - in production use signed tokens)
    let expected_token = PasswordService::hash_token(&format!("unsub:{}", email.to_lowercase()));
    if token != &expected_token {
        return Err(ApiError::BadRequest("Invalid unsubscribe link".to_string()).into());
    }

    let now = chrono::Utc::now();

    // Unsubscribe
    ctx.data.db.execute(
        r#"
        UPDATE email_subscribers 
        SET status = 'unsubscribed', unsubscribed_at = $1, updated_at = $1
        WHERE email = $2
        "#,
        vec![
            serde_json::json!(now.to_rfc3339()),
            serde_json::json!(email.to_lowercase()),
        ]
    ).await?;

    // Redirect to unsubscribe confirmation page
    Response::redirect_with_status(
        worker::Url::parse(&format!("{}/newsletter/unsubscribed", ctx.data.config.site_url))?,
        302
    )
}

#[derive(serde::Deserialize)]
struct EmailConfirmation {
    id: uuid::Uuid,
    email: String,
    token_hash: String,
    expires_at: chrono::DateTime<chrono::Utc>,
}
