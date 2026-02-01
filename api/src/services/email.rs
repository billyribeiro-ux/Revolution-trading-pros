//! Email service - Postmark Integration
//! ICT 11+ Principal Engineer Grade
//!
//! Provides email sending functionality via Postmark API:
//! - Email verification emails
//! - Welcome emails
//! - Password reset emails
//! - Newsletter and transactional emails

use anyhow::Result;
use reqwest::Client;
use serde::{Deserialize, Serialize};

/// Email service for sending transactional emails via Postmark
#[derive(Clone)]
pub struct EmailService {
    client: Client,
    token: String,
    from_email: String,
    from_name: String,
    app_url: String,
}

#[derive(Serialize)]
struct PostmarkEmail {
    #[serde(rename = "From")]
    from: String,
    #[serde(rename = "To")]
    to: String,
    #[serde(rename = "Subject")]
    subject: String,
    #[serde(rename = "HtmlBody")]
    html_body: String,
    #[serde(rename = "TextBody")]
    text_body: Option<String>,
    #[serde(rename = "MessageStream")]
    message_stream: String,
}

#[derive(Deserialize)]
#[allow(dead_code)]
struct PostmarkResponse {
    #[serde(rename = "MessageID")]
    message_id: Option<String>,
    #[serde(rename = "ErrorCode")]
    error_code: Option<i32>,
    #[serde(rename = "Message")]
    message: Option<String>,
}

impl EmailService {
    /// Create a new EmailService instance
    pub fn new(token: &str, from_email: &str, app_url: &str) -> Self {
        Self {
            client: Client::new(),
            token: token.to_string(),
            from_email: from_email.to_string(),
            from_name: "Revolution Trading Pros".to_string(),
            app_url: app_url.trim_end_matches('/').to_string(),
        }
    }

    /// Send an email via Postmark
    pub async fn send(
        &self,
        to: &str,
        subject: &str,
        html_body: &str,
        text_body: Option<&str>,
    ) -> Result<()> {
        let from = format!("{} <{}>", self.from_name, self.from_email);

        let email = PostmarkEmail {
            from,
            to: to.to_string(),
            subject: subject.to_string(),
            html_body: html_body.to_string(),
            text_body: text_body.map(|s| s.to_string()),
            message_stream: "outbound".to_string(),
        };

        let response = self
            .client
            .post("https://api.postmarkapp.com/email")
            .header("Accept", "application/json")
            .header("Content-Type", "application/json")
            .header("X-Postmark-Server-Token", &self.token)
            .json(&email)
            .send()
            .await?;

        if !response.status().is_success() {
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());
            tracing::error!("Postmark API error: {}", error_text);
            return Err(anyhow::anyhow!("Failed to send email: {}", error_text));
        }

        tracing::info!("Email sent successfully to {}", to);
        Ok(())
    }

    /// Send email verification email
    pub async fn send_verification_email(&self, to: &str, name: &str, token: &str) -> Result<()> {
        let verification_url = format!("{}/verify-email?token={}", self.app_url, token);

        let subject = "Verify Your Email - Revolution Trading Pros";

        let html = Self::email_template(
            "Verify Your Email",
            &format!(
                r#"
                <p style="font-size: 16px; color: #374151; margin-bottom: 20px;">
                    Hi <strong>{}</strong>,
                </p>
                <p style="font-size: 16px; color: #374151; margin-bottom: 20px;">
                    Thank you for registering with Revolution Trading Pros! To complete your registration 
                    and access all features, please verify your email address by clicking the button below:
                </p>
                <div style="text-align: center; margin: 32px 0;">
                    <a href="{}" 
                       style="display: inline-block; background: linear-gradient(135deg, #10b981, #059669); 
                              color: white; text-decoration: none; padding: 16px 32px; 
                              border-radius: 12px; font-weight: 700; font-size: 16px;
                              box-shadow: 0 4px 14px rgba(16, 185, 129, 0.4);">
                        Verify Email Address
                    </a>
                </div>
                <p style="font-size: 14px; color: #6b7280; margin-top: 24px;">
                    Or copy and paste this link into your browser:
                </p>
                <p style="font-size: 14px; color: #10b981; word-break: break-all;">
                    {}
                </p>
                <p style="font-size: 14px; color: #6b7280; margin-top: 24px;">
                    This link will expire in <strong>24 hours</strong>.
                </p>
                <p style="font-size: 14px; color: #6b7280; margin-top: 16px;">
                    If you didn't create an account with Revolution Trading Pros, you can safely ignore this email.
                </p>
                "#,
                html_escape(name),
                verification_url,
                verification_url
            ),
        );

        let text = format!(
            "Hi {},\n\n\
            Thank you for registering with Revolution Trading Pros!\n\n\
            Please verify your email by visiting this link:\n{}\n\n\
            This link will expire in 24 hours.\n\n\
            If you didn't create an account, you can safely ignore this email.\n\n\
            - The Revolution Trading Pros Team",
            name, verification_url
        );

        self.send(to, subject, &html, Some(&text)).await
    }

    /// Send welcome email (after verification)
    pub async fn send_welcome_email(&self, to: &str, name: &str) -> Result<()> {
        let subject = "Welcome to Revolution Trading Pros! 🎉";
        let dashboard_url = format!("{}/dashboard", self.app_url);

        let html = Self::email_template(
            "Welcome to the Revolution!",
            &format!(
                r#"
                <p style="font-size: 16px; color: #374151; margin-bottom: 20px;">
                    Hi <strong>{}</strong>,
                </p>
                <p style="font-size: 16px; color: #374151; margin-bottom: 20px;">
                    Your email has been verified and your account is now fully activated! 
                    Welcome to the Revolution Trading Pros community.
                </p>
                <div style="background: linear-gradient(135deg, #f0fdf4, #dcfce7); 
                            border-radius: 16px; padding: 24px; margin: 24px 0;">
                    <h3 style="color: #166534; margin: 0 0 16px 0; font-size: 18px;">
                        🚀 Get Started
                    </h3>
                    <ul style="color: #15803d; margin: 0; padding-left: 20px; line-height: 2;">
                        <li>Access live trading rooms</li>
                        <li>Explore professional trading courses</li>
                        <li>Set up real-time trading alerts</li>
                        <li>Join our Discord community</li>
                    </ul>
                </div>
                <div style="text-align: center; margin: 32px 0;">
                    <a href="{}" 
                       style="display: inline-block; background: linear-gradient(135deg, #10b981, #059669); 
                              color: white; text-decoration: none; padding: 16px 32px; 
                              border-radius: 12px; font-weight: 700; font-size: 16px;
                              box-shadow: 0 4px 14px rgba(16, 185, 129, 0.4);">
                        Go to Dashboard
                    </a>
                </div>
                <p style="font-size: 14px; color: #6b7280; margin-top: 24px;">
                    If you have any questions, our support team is here to help.
                </p>
                "#,
                html_escape(name),
                dashboard_url
            ),
        );

        let text = format!(
            "Hi {},\n\n\
            Your email has been verified and your account is now fully activated!\n\n\
            Welcome to the Revolution Trading Pros community.\n\n\
            Get started:\n\
            - Access live trading rooms\n\
            - Explore professional trading courses\n\
            - Set up real-time trading alerts\n\
            - Join our Discord community\n\n\
            Visit your dashboard: {}\n\n\
            - The Revolution Trading Pros Team",
            name, dashboard_url
        );

        self.send(to, subject, &html, Some(&text)).await
    }

    /// Send password reset email
    pub async fn send_password_reset(&self, to: &str, name: &str, token: &str) -> Result<()> {
        let reset_url = format!(
            "{}/reset-password?token={}&email={}",
            self.app_url,
            token,
            urlencoding::encode(to)
        );

        let subject = "Reset Your Password - Revolution Trading Pros";

        let html = Self::email_template(
            "Reset Your Password",
            &format!(
                r#"
                <p style="font-size: 16px; color: #374151; margin-bottom: 20px;">
                    Hi <strong>{}</strong>,
                </p>
                <p style="font-size: 16px; color: #374151; margin-bottom: 20px;">
                    We received a request to reset your password for your Revolution Trading Pros account. 
                    Click the button below to create a new password:
                </p>
                <div style="text-align: center; margin: 32px 0;">
                    <a href="{}" 
                       style="display: inline-block; background: linear-gradient(135deg, #f59e0b, #d97706); 
                              color: white; text-decoration: none; padding: 16px 32px; 
                              border-radius: 12px; font-weight: 700; font-size: 16px;
                              box-shadow: 0 4px 14px rgba(245, 158, 11, 0.4);">
                        Reset Password
                    </a>
                </div>
                <p style="font-size: 14px; color: #6b7280; margin-top: 24px;">
                    Or copy and paste this link into your browser:
                </p>
                <p style="font-size: 14px; color: #f59e0b; word-break: break-all;">
                    {}
                </p>
                <p style="font-size: 14px; color: #6b7280; margin-top: 24px;">
                    This link will expire in <strong>1 hour</strong>.
                </p>
                <div style="background: #fef3c7; border-left: 4px solid #f59e0b; 
                            padding: 16px; margin-top: 24px; border-radius: 0 8px 8px 0;">
                    <p style="color: #92400e; margin: 0; font-size: 14px;">
                        <strong>⚠️ Security Notice:</strong> If you didn't request this password reset, 
                        please ignore this email. Your password will remain unchanged.
                    </p>
                </div>
                "#,
                html_escape(name),
                reset_url,
                reset_url
            ),
        );

        let text = format!(
            "Hi {},\n\n\
            We received a request to reset your password.\n\n\
            Click this link to reset your password:\n{}\n\n\
            This link will expire in 1 hour.\n\n\
            If you didn't request this, please ignore this email.\n\n\
            - The Revolution Trading Pros Team",
            name, reset_url
        );

        self.send(to, subject, &html, Some(&text)).await
    }

    /// Send payment failed email
    /// ICT 7 Fix: Added missing payment failed notification
    pub async fn send_payment_failed_email(
        &self,
        to: &str,
        name: &str,
        subscription_name: &str,
        retry_url: &str,
    ) -> Result<()> {
        let subject = "Payment Failed - Action Required";
        let billing_url = format!("{}/dashboard/account/billing", self.app_url);

        let html = Self::email_template(
            "Payment Failed",
            &format!(
                r#"
                <div style="text-align: center; margin-bottom: 24px;">
                    <div style="font-size: 64px; color: #ef4444;">&#9888;</div>
                </div>
                <p style="font-size: 16px; color: #374151; margin-bottom: 20px;">
                    Hi <strong>{}</strong>,
                </p>
                <p style="font-size: 16px; color: #374151; margin-bottom: 20px;">
                    We were unable to process the payment for your <strong>{}</strong> subscription.
                </p>
                <div style="background: #fef2f2; border: 1px solid #fecaca; border-radius: 12px;
                            padding: 20px; margin: 24px 0;">
                    <p style="color: #991b1b; margin: 0; font-size: 14px;">
                        <strong>What this means:</strong> Your subscription access may be limited
                        until we can successfully process your payment.
                    </p>
                </div>
                <div style="background: #f3f4f6; border-radius: 12px; padding: 20px; margin: 24px 0;">
                    <p style="color: #374151; margin: 0 0 12px 0; font-size: 14px;">
                        <strong>Common reasons for payment failure:</strong>
                    </p>
                    <ul style="color: #6b7280; margin: 0; padding-left: 20px; font-size: 14px;">
                        <li>Insufficient funds</li>
                        <li>Card expired or cancelled</li>
                        <li>Incorrect billing information</li>
                        <li>Card issuer declined the transaction</li>
                    </ul>
                </div>
                <div style="text-align: center; margin: 32px 0;">
                    <a href="{}"
                       style="display: inline-block; background: linear-gradient(135deg, #ef4444, #dc2626);
                              color: white; text-decoration: none; padding: 16px 32px;
                              border-radius: 12px; font-weight: 700; font-size: 16px;
                              box-shadow: 0 4px 14px rgba(239, 68, 68, 0.4);">
                        Update Payment Method
                    </a>
                </div>
                <p style="font-size: 14px; color: #6b7280; margin-top: 24px; text-align: center;">
                    Need help? Contact our support team at support@revolutiontradingpros.com
                </p>
                "#,
                html_escape(name),
                html_escape(subscription_name),
                billing_url
            ),
        );

        let text = format!(
            "Hi {},\n\n\
            We were unable to process the payment for your {} subscription.\n\n\
            What this means: Your subscription access may be limited until we can successfully process your payment.\n\n\
            Common reasons for payment failure:\n\
            - Insufficient funds\n\
            - Card expired or cancelled\n\
            - Incorrect billing information\n\
            - Card issuer declined the transaction\n\n\
            Please update your payment method: {}\n\n\
            Need help? Contact support@revolutiontradingpros.com\n\n\
            - The Revolution Trading Pros Team",
            name, subscription_name, billing_url
        );

        self.send(to, subject, &html, Some(&text)).await
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // SUBSCRIPTION EMAIL NOTIFICATIONS - ICT 7+ Enterprise
    // ═══════════════════════════════════════════════════════════════════════════

    /// Send subscription confirmation email
    pub async fn send_subscription_confirmation(
        &self,
        to: &str,
        name: &str,
        plan_name: &str,
        price: f64,
        billing_cycle: &str,
        has_trial: bool,
        trial_days: i32,
    ) -> Result<()> {
        let subject = format!("Subscription Confirmed - {}", plan_name);
        let dashboard_url = format!("{}/my/subscriptions", self.app_url);

        let trial_notice = if has_trial {
            format!(
                r#"<div style="background: #dbeafe; border-left: 4px solid #3b82f6;
                            padding: 16px; margin: 24px 0; border-radius: 0 8px 8px 0;">
                    <p style="color: #1e40af; margin: 0; font-size: 14px;">
                        <strong>Trial Period:</strong> You have a {}-day free trial.
                        Your card won't be charged until the trial ends.
                    </p>
                </div>"#,
                trial_days
            )
        } else {
            String::new()
        };

        let html = Self::email_template(
            "Subscription Confirmed!",
            &format!(
                r#"
                <div style="text-align: center; margin-bottom: 24px;">
                    <div style="font-size: 64px;">🎉</div>
                </div>
                <p style="font-size: 16px; color: #374151; margin-bottom: 20px;">
                    Hi <strong>{}</strong>,
                </p>
                <p style="font-size: 16px; color: #374151; margin-bottom: 20px;">
                    Welcome! Your subscription to <strong>{}</strong> has been confirmed.
                </p>
                {}
                <div style="background: #f0fdf4; border: 1px solid #86efac; border-radius: 12px;
                            padding: 20px; margin: 24px 0;">
                    <h3 style="color: #166534; margin: 0 0 12px 0; font-size: 16px;">Subscription Details</h3>
                    <table style="width: 100%; font-size: 14px; color: #374151;">
                        <tr>
                            <td style="padding: 8px 0;">Plan:</td>
                            <td style="padding: 8px 0; text-align: right; font-weight: 600;">{}</td>
                        </tr>
                        <tr>
                            <td style="padding: 8px 0;">Price:</td>
                            <td style="padding: 8px 0; text-align: right; font-weight: 600;">${:.2}/{}</td>
                        </tr>
                    </table>
                </div>
                <div style="text-align: center; margin: 32px 0;">
                    <a href="{}"
                       style="display: inline-block; background: linear-gradient(135deg, #10b981, #059669);
                              color: white; text-decoration: none; padding: 16px 32px;
                              border-radius: 12px; font-weight: 700; font-size: 16px;
                              box-shadow: 0 4px 14px rgba(16, 185, 129, 0.4);">
                        Manage Subscription
                    </a>
                </div>
                "#,
                html_escape(name),
                html_escape(plan_name),
                trial_notice,
                html_escape(plan_name),
                price,
                billing_cycle,
                dashboard_url
            ),
        );

        let text = format!(
            "Hi {},\n\nYour subscription to {} has been confirmed!\n\nPlan: {}\nPrice: ${:.2}/{}\n\n{}\nManage: {}\n\n- The Revolution Trading Pros Team",
            name, plan_name, plan_name, price, billing_cycle,
            if has_trial { format!("You have a {}-day free trial.", trial_days) } else { String::new() },
            dashboard_url
        );

        self.send(to, &subject, &html, Some(&text)).await
    }

    /// Send subscription cancellation email
    pub async fn send_subscription_cancelled(
        &self,
        to: &str,
        name: &str,
        plan_name: &str,
        end_date: &str,
        cancel_immediately: bool,
    ) -> Result<()> {
        let subject = "Subscription Cancelled - Revolution Trading Pros";
        let resubscribe_url = format!("{}/pricing", self.app_url);

        let message = if cancel_immediately {
            "Your subscription has been cancelled immediately."
        } else {
            "Your subscription will remain active until the end of your billing period."
        };

        let html = Self::email_template(
            "Subscription Cancelled",
            &format!(
                r#"
                <p style="font-size: 16px; color: #374151; margin-bottom: 20px;">
                    Hi <strong>{}</strong>,
                </p>
                <p style="font-size: 16px; color: #374151; margin-bottom: 20px;">
                    Your <strong>{}</strong> subscription has been cancelled.
                </p>
                <div style="background: #fef3c7; border-left: 4px solid #f59e0b;
                            padding: 16px; margin: 24px 0; border-radius: 0 8px 8px 0;">
                    <p style="color: #92400e; margin: 0; font-size: 14px;">
                        {} {}
                    </p>
                </div>
                <div style="text-align: center; margin: 32px 0;">
                    <a href="{}"
                       style="display: inline-block; background: linear-gradient(135deg, #6366f1, #8b5cf6);
                              color: white; text-decoration: none; padding: 16px 32px;
                              border-radius: 12px; font-weight: 700; font-size: 16px;">
                        Resubscribe
                    </a>
                </div>
                "#,
                html_escape(name),
                html_escape(plan_name),
                message,
                if !cancel_immediately { format!("<br><strong>Access until:</strong> {}", end_date) } else { String::new() },
                resubscribe_url
            ),
        );

        let text = format!(
            "Hi {},\n\nYour {} subscription has been cancelled.\n\n{}\n\n- The Revolution Trading Pros Team",
            name, plan_name, message
        );

        self.send(to, subject, &html, Some(&text)).await
    }

    /// Send payment failed with grace period email
    pub async fn send_payment_failed_with_grace(
        &self,
        to: &str,
        name: &str,
        plan_name: &str,
        amount: f64,
        grace_period_end: &str,
        retry_count: i32,
    ) -> Result<()> {
        let subject = "Action Required: Payment Failed";
        let update_url = format!("{}/my/subscriptions", self.app_url);

        let urgency = if retry_count >= 3 {
            "FINAL NOTICE: Your subscription will be cancelled if payment is not received."
        } else {
            "Please update your payment method to avoid losing access."
        };

        let html = Self::email_template(
            "Payment Failed",
            &format!(
                r#"
                <div style="text-align: center; margin-bottom: 24px;">
                    <div style="font-size: 64px;">⚠️</div>
                </div>
                <p style="font-size: 16px; color: #374151; margin-bottom: 20px;">
                    Hi <strong>{}</strong>,
                </p>
                <p style="font-size: 16px; color: #374151; margin-bottom: 20px;">
                    We were unable to process your payment of <strong>${:.2}</strong> for <strong>{}</strong>.
                </p>
                <div style="background: #fef2f2; border: 1px solid #fecaca; border-radius: 12px;
                            padding: 20px; margin: 24px 0;">
                    <p style="color: #991b1b; margin: 0; font-size: 14px;">
                        <strong>{}</strong><br><br>
                        Grace period ends: <strong>{}</strong>
                    </p>
                </div>
                <div style="text-align: center; margin: 32px 0;">
                    <a href="{}"
                       style="display: inline-block; background: linear-gradient(135deg, #ef4444, #dc2626);
                              color: white; text-decoration: none; padding: 16px 32px;
                              border-radius: 12px; font-weight: 700; font-size: 16px;">
                        Update Payment Method
                    </a>
                </div>
                "#,
                html_escape(name),
                amount,
                html_escape(plan_name),
                urgency,
                grace_period_end,
                update_url
            ),
        );

        let text = format!(
            "Hi {},\n\nPayment of ${:.2} for {} failed.\n\n{}\nGrace period ends: {}\n\nUpdate payment: {}\n\n- The Revolution Trading Pros Team",
            name, amount, plan_name, urgency, grace_period_end, update_url
        );

        self.send(to, subject, &html, Some(&text)).await
    }

    /// Send renewal reminder email
    pub async fn send_renewal_reminder(
        &self,
        to: &str,
        name: &str,
        plan_name: &str,
        amount: f64,
        renewal_date: &str,
    ) -> Result<()> {
        let subject = format!("Subscription Renewal Reminder - {}", plan_name);
        let manage_url = format!("{}/my/subscriptions", self.app_url);

        let html = Self::email_template(
            "Renewal Reminder",
            &format!(
                r#"
                <p style="font-size: 16px; color: #374151; margin-bottom: 20px;">
                    Hi <strong>{}</strong>,
                </p>
                <p style="font-size: 16px; color: #374151; margin-bottom: 20px;">
                    Your <strong>{}</strong> subscription will renew on <strong>{}</strong> for <strong>${:.2}</strong>.
                </p>
                <div style="background: #f0f9ff; border: 1px solid #bae6fd; border-radius: 12px;
                            padding: 20px; margin: 24px 0;">
                    <p style="color: #0369a1; margin: 0; font-size: 14px;">
                        No action is required. Your subscription will automatically renew.
                    </p>
                </div>
                <div style="text-align: center; margin: 32px 0;">
                    <a href="{}"
                       style="display: inline-block; background: linear-gradient(135deg, #10b981, #059669);
                              color: white; text-decoration: none; padding: 16px 32px;
                              border-radius: 12px; font-weight: 700; font-size: 16px;">
                        Manage Subscription
                    </a>
                </div>
                "#,
                html_escape(name),
                html_escape(plan_name),
                renewal_date,
                amount,
                manage_url
            ),
        );

        let text = format!(
            "Hi {},\n\nYour {} subscription will renew on {} for ${:.2}.\n\nManage: {}\n\n- The Revolution Trading Pros Team",
            name, plan_name, renewal_date, amount, manage_url
        );

        self.send(to, &subject, &html, Some(&text)).await
    }

    /// Send trial ending soon notification
    pub async fn send_trial_ending_soon(
        &self,
        to: &str,
        name: &str,
        plan_name: &str,
        trial_end_date: &str,
        price: f64,
        billing_cycle: &str,
    ) -> Result<()> {
        let subject = "Your Trial Ends Soon - Revolution Trading Pros";
        let manage_url = format!("{}/my/subscriptions", self.app_url);

        let html = Self::email_template(
            "Trial Ending Soon",
            &format!(
                r#"
                <p style="font-size: 16px; color: #374151; margin-bottom: 20px;">
                    Hi <strong>{}</strong>,
                </p>
                <p style="font-size: 16px; color: #374151; margin-bottom: 20px;">
                    Your free trial for <strong>{}</strong> ends on <strong>{}</strong>.
                </p>
                <div style="background: #fef3c7; border-left: 4px solid #f59e0b;
                            padding: 16px; margin: 24px 0; border-radius: 0 8px 8px 0;">
                    <p style="color: #92400e; margin: 0; font-size: 14px;">
                        After your trial ends, you'll be charged <strong>${:.2}/{}</strong>.
                    </p>
                </div>
                <div style="text-align: center; margin: 32px 0;">
                    <a href="{}"
                       style="display: inline-block; background: linear-gradient(135deg, #10b981, #059669);
                              color: white; text-decoration: none; padding: 16px 32px;
                              border-radius: 12px; font-weight: 700; font-size: 16px;">
                        Manage Subscription
                    </a>
                </div>
                "#,
                html_escape(name),
                html_escape(plan_name),
                trial_end_date,
                price,
                billing_cycle,
                manage_url
            ),
        );

        let text = format!(
            "Hi {},\n\nYour trial for {} ends on {}. You'll be charged ${:.2}/{}.\n\nManage: {}\n\n- The Revolution Trading Pros Team",
            name, plan_name, trial_end_date, price, billing_cycle, manage_url
        );

        self.send(to, subject, &html, Some(&text)).await
    }

    /// Send subscription upgraded email
    pub async fn send_subscription_upgraded(
        &self,
        to: &str,
        name: &str,
        old_plan: &str,
        new_plan: &str,
        new_price: f64,
        billing_cycle: &str,
    ) -> Result<()> {
        let subject = format!("Subscription Upgraded to {}", new_plan);
        let manage_url = format!("{}/my/subscriptions", self.app_url);

        let html = Self::email_template(
            "Subscription Upgraded!",
            &format!(
                r#"
                <div style="text-align: center; margin-bottom: 24px;">
                    <div style="font-size: 64px;">🚀</div>
                </div>
                <p style="font-size: 16px; color: #374151; margin-bottom: 20px;">
                    Hi <strong>{}</strong>,
                </p>
                <p style="font-size: 16px; color: #374151; margin-bottom: 20px;">
                    Your subscription has been upgraded from <strong>{}</strong> to <strong>{}</strong>.
                </p>
                <div style="background: #f0fdf4; border: 1px solid #86efac; border-radius: 12px;
                            padding: 20px; margin: 24px 0;">
                    <p style="color: #166534; margin: 0; font-size: 14px;">
                        New price: <strong>${:.2}/{}</strong>
                    </p>
                </div>
                <div style="text-align: center; margin: 32px 0;">
                    <a href="{}"
                       style="display: inline-block; background: linear-gradient(135deg, #10b981, #059669);
                              color: white; text-decoration: none; padding: 16px 32px;
                              border-radius: 12px; font-weight: 700; font-size: 16px;">
                        View Subscription
                    </a>
                </div>
                "#,
                html_escape(name),
                html_escape(old_plan),
                html_escape(new_plan),
                new_price,
                billing_cycle,
                manage_url
            ),
        );

        let text = format!(
            "Hi {},\n\nUpgraded from {} to {}. New price: ${:.2}/{}\n\nView: {}\n\n- The Revolution Trading Pros Team",
            name, old_plan, new_plan, new_price, billing_cycle, manage_url
        );

        self.send(to, &subject, &html, Some(&text)).await
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // NEWSLETTER EMAIL FUNCTIONS - ICT 7+ Double Opt-In & GDPR Compliance
    // ═══════════════════════════════════════════════════════════════════════════

    /// Send newsletter confirmation email (double opt-in)
    pub async fn send_newsletter_confirmation(
        &self,
        to: &str,
        name: &str,
        confirm_token: &str,
    ) -> Result<()> {
        let confirm_url = format!("{}/newsletter/confirm?token={}", self.app_url, confirm_token);

        let subject = "Confirm Your Newsletter Subscription";

        let html = Self::email_template(
            "Confirm Your Subscription",
            &format!(
                r#"
                <p style="font-size: 16px; color: #374151; margin-bottom: 20px;">
                    Hi{},
                </p>
                <p style="font-size: 16px; color: #374151; margin-bottom: 20px;">
                    Thank you for subscribing to the Revolution Trading Pros newsletter!
                    Please confirm your subscription by clicking the button below:
                </p>
                <div style="text-align: center; margin: 32px 0;">
                    <a href="{}"
                       style="display: inline-block; background: linear-gradient(135deg, #10b981, #059669);
                              color: white; text-decoration: none; padding: 16px 32px;
                              border-radius: 12px; font-weight: 700; font-size: 16px;
                              box-shadow: 0 4px 14px rgba(16, 185, 129, 0.4);">
                        Confirm Subscription
                    </a>
                </div>
                <p style="font-size: 14px; color: #6b7280; margin-top: 24px;">
                    Or copy and paste this link into your browser:
                </p>
                <p style="font-size: 14px; color: #10b981; word-break: break-all;">
                    {}
                </p>
                <div style="background: #f3f4f6; border-radius: 12px; padding: 20px; margin-top: 24px;">
                    <p style="color: #6b7280; margin: 0; font-size: 14px;">
                        <strong>Why confirm?</strong> We use double opt-in to ensure you actually
                        requested this newsletter and to comply with privacy regulations (GDPR).
                    </p>
                </div>
                <p style="font-size: 14px; color: #6b7280; margin-top: 24px;">
                    If you didn't subscribe to our newsletter, you can safely ignore this email.
                </p>
                "#,
                if name.is_empty() { String::new() } else { format!(" <strong>{}</strong>", html_escape(name)) },
                confirm_url,
                confirm_url
            ),
        );

        let text = format!(
            "Hi{},\n\n\
            Thank you for subscribing to the Revolution Trading Pros newsletter!\n\n\
            Please confirm your subscription by visiting this link:\n{}\n\n\
            If you didn't subscribe, you can safely ignore this email.\n\n\
            - The Revolution Trading Pros Team",
            if name.is_empty() { String::new() } else { format!(" {}", name) },
            confirm_url
        );

        self.send(to, subject, &html, Some(&text)).await
    }

    /// Send newsletter with unsubscribe link (bulk email)
    pub async fn send_newsletter(
        &self,
        to: &str,
        name: &str,
        subject: &str,
        html_content: &str,
        unsubscribe_token: &str,
    ) -> Result<()> {
        let unsubscribe_url = format!(
            "{}/newsletter/unsubscribe?token={}",
            self.app_url, unsubscribe_token
        );

        // Add unsubscribe footer to content
        let html_with_footer = format!(
            r#"{}
            <div style="border-top: 1px solid #e5e7eb; margin-top: 32px; padding-top: 24px; text-align: center;">
                <p style="font-size: 12px; color: #9ca3af; margin: 0 0 8px 0;">
                    You're receiving this email because you subscribed to Revolution Trading Pros newsletter.
                </p>
                <p style="font-size: 12px; color: #9ca3af; margin: 0;">
                    <a href="{}" style="color: #6b7280; text-decoration: underline;">Unsubscribe</a>
                    &nbsp;|&nbsp;
                    <a href="{}/privacy" style="color: #6b7280; text-decoration: underline;">Privacy Policy</a>
                </p>
            </div>"#,
            html_content, unsubscribe_url, self.app_url
        );

        let full_html = Self::email_template(
            subject,
            &format!(
                r#"
                <p style="font-size: 16px; color: #374151; margin-bottom: 20px;">
                    Hi{},
                </p>
                {}
                "#,
                if name.is_empty() { String::new() } else { format!(" <strong>{}</strong>", html_escape(name)) },
                html_with_footer
            ),
        );

        let text = format!(
            "Hi{},\n\n{}\n\n---\nUnsubscribe: {}\n\n- The Revolution Trading Pros Team",
            if name.is_empty() { String::new() } else { format!(" {}", name) },
            "View this email in your browser for the full content.",
            unsubscribe_url
        );

        self.send(to, subject, &full_html, Some(&text)).await
    }

    /// Send order confirmation email
    pub async fn send_order_confirmation(&self, to: &str, order_number: &str) -> Result<()> {
        let subject = format!("Order Confirmed - #{}", order_number);
        let orders_url = format!("{}/dashboard/account/orders", self.app_url);

        let html = Self::email_template(
            "Order Confirmed!",
            &format!(
                r#"
                <div style="text-align: center; margin-bottom: 24px;">
                    <div style="font-size: 64px;">✅</div>
                </div>
                <p style="font-size: 16px; color: #374151; margin-bottom: 20px; text-align: center;">
                    Thank you for your purchase! Your order <strong>#{}</strong> has been confirmed.
                </p>
                <div style="background: #f0fdf4; border: 1px solid #86efac; border-radius: 12px;
                            padding: 20px; margin: 24px 0; text-align: center;">
                    <p style="color: #166534; margin: 0; font-size: 14px;">
                        Your order is now being processed. You will receive access to your purchases shortly.
                    </p>
                </div>
                <div style="text-align: center; margin: 32px 0;">
                    <a href="{}"
                       style="display: inline-block; background: linear-gradient(135deg, #10b981, #059669);
                              color: white; text-decoration: none; padding: 16px 32px;
                              border-radius: 12px; font-weight: 700; font-size: 16px;
                              box-shadow: 0 4px 14px rgba(16, 185, 129, 0.4);">
                        View Your Orders
                    </a>
                </div>
                <p style="font-size: 14px; color: #6b7280; margin-top: 24px; text-align: center;">
                    If you have any questions about your order, please contact our support team.
                </p>
                "#,
                order_number, orders_url
            ),
        );

        let text = format!(
            "Order Confirmed!\n\n\
            Thank you for your purchase! Your order #{} has been confirmed.\n\n\
            Your order is now being processed. You will receive access to your purchases shortly.\n\n\
            View your orders: {}\n\n\
            - The Revolution Trading Pros Team",
            order_number, orders_url
        );

        self.send(to, &subject, &html, Some(&text)).await
    }

    /// Generate a beautiful HTML email template wrapper
    fn email_template(title: &str, content: &str) -> String {
        format!(
            r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>{}</title>
</head>
<body style="margin: 0; padding: 0; background-color: #f3f4f6; font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, 'Helvetica Neue', Arial, sans-serif;">
    <div style="max-width: 600px; margin: 0 auto; padding: 40px 20px;">
        <!-- Header -->
        <div style="text-align: center; margin-bottom: 32px;">
            <div style="display: inline-block; background: linear-gradient(135deg, #064e3b, #065f46); 
                        padding: 16px 32px; border-radius: 16px;">
                <h1 style="margin: 0; color: #6ee7b7; font-size: 24px; font-weight: 800; letter-spacing: -0.5px;">
                    Revolution Trading Pros
                </h1>
            </div>
        </div>
        
        <!-- Main Card -->
        <div style="background: white; border-radius: 24px; padding: 40px; 
                    box-shadow: 0 4px 6px rgba(0, 0, 0, 0.05), 0 10px 15px rgba(0, 0, 0, 0.1);">
            <h2 style="color: #111827; font-size: 28px; margin: 0 0 24px 0; font-weight: 700;">
                {}
            </h2>
            {}
        </div>
        
        <!-- Footer -->
        <div style="text-align: center; margin-top: 32px; padding: 0 20px;">
            <p style="color: #9ca3af; font-size: 14px; margin: 0 0 8px 0;">
                © 2025 Revolution Trading Pros. All rights reserved.
            </p>
            <p style="color: #9ca3af; font-size: 12px; margin: 0;">
                You're receiving this email because you have an account with Revolution Trading Pros.
            </p>
        </div>
    </div>
</body>
</html>"#,
            title, title, content
        )
    }
}

/// Escape HTML special characters
fn html_escape(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#39;")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_html_escape() {
        assert_eq!(html_escape("<script>"), "&lt;script&gt;");
        assert_eq!(html_escape("John & Jane"), "John &amp; Jane");
        assert_eq!(html_escape("Hello \"World\""), "Hello &quot;World&quot;");
    }

    #[test]
    fn test_email_template_generation() {
        let template = EmailService::email_template("Test Title", "<p>Test content</p>");
        assert!(template.contains("Test Title"));
        assert!(template.contains("<p>Test content</p>"));
        assert!(template.contains("Revolution Trading Pros"));
    }
}
